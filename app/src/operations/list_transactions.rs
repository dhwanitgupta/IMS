use crate::State;
use aws_smithy_http_server::Extension;
use std::collections::{HashMap, HashSet};

use chrono::TimeZone;
use liwimean_ims_core_server_sdk::error::ListTransactionsError;
use liwimean_ims_core_server_sdk::input::ListTransactionsInput;
use liwimean_ims_core_server_sdk::model::TransactionCategory::{
    Investment, Leisure, Living, Medical, Salary,
};
use liwimean_ims_core_server_sdk::model::TransactionSubCategory::{
    Amazon, CreditCard, Crypto, Divident, Equity, Flat, Food, Grocery, HealthCheckUp, Internet,
    MutualFunds, Operation, Rd, Travel, Zerodha,
};
use liwimean_ims_core_server_sdk::model::TransactionType::{Credit, Debit};
use liwimean_ims_core_server_sdk::model::{
    BreakageByPurpose, TransactionAggregation, TransactionCategory, TransactionPurpose,
    TransactionSubCategory, TransactionSummary, TransactionType,
};
use liwimean_ims_core_server_sdk::output::ListTransactionsOutput;
use liwimean_ims_core_server_sdk::types::DateTime;
use log::info;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use serde::Deserialize;

pub async fn do_list_transactions(
    input: ListTransactionsInput,
    state: Extension<Arc<State>>,
) -> Result<ListTransactionsOutput, ListTransactionsError> {
    let result = state.resources_map.get(input.tlr()).unwrap();
    let path = &result.transaction_path;

    Ok(get_transaction_output(path))
}

pub fn get_transaction_output(path: &PathBuf) -> ListTransactionsOutput {
    let files = path.read_dir().unwrap();

    let mut transaction_records = vec![];
    let mut count = 0;

    for file in files {
        let file_path = file.unwrap().path().display().to_string();

        info!("Started Loading File : {}", file_path);

        let cvs_string = fs::read_to_string(file_path.clone())
            .unwrap_or_else(|_| panic!("Not able to read file with path {}", file_path.clone()));

        let mut rdr = csv::Reader::from_reader(cvs_string.as_bytes());
        for result in rdr.deserialize() {
            let record: TransactionRecord = result.expect("Not able to read as TR");
            transaction_records.push(record);
        }

        info!(
            "Finished Loading File : {}, successfully loaded {} records",
            file_path.clone(),
            transaction_records.len() - count
        );

        count = transaction_records.len();
    }

    let trasactions_summaries = map_to_summaries(transaction_records);

    let aggregate = get_aggregate_view(trasactions_summaries.as_ref());

    ListTransactionsOutput {
        transactions: trasactions_summaries,
        aggregate,
    }
}

fn get_total_by_type(
    summaries: &Vec<TransactionSummary>,
    transaction_type: TransactionType,
) -> f64 {
    return summaries
        .iter()
        .cloned()
        .filter(|summary| summary.transaction_type().unwrap().clone() == transaction_type)
        .map(|summary| summary.amount().unwrap())
        .sum();
}

fn get_total_by_purpose(
    summaries: &Vec<TransactionSummary>,
    category: TransactionCategory,
    sub_category: TransactionSubCategory,
) -> f64 {
    summaries
        .iter()
        .cloned()
        .filter(|summary| {
            summary
                .transaction_purpose()
                .unwrap()
                .category()
                .unwrap()
                .clone()
                == category
                && summary
                    .transaction_purpose()
                    .unwrap()
                    .sub_category()
                    .unwrap()
                    .clone()
                    == sub_category
        })
        .map(|summary| summary.amount().unwrap())
        .sum()
}

fn get_list_of_breakage_by_purpose(
    summaries: &Vec<TransactionSummary>,
    transaction_type: TransactionType,
) -> Vec<BreakageByPurpose> {
    let filttered_summaries: Vec<TransactionSummary> = summaries
        .iter()
        .cloned()
        .filter(|summary| summary.transaction_type().unwrap().clone() == transaction_type)
        .collect();

    let unique_purposes = filttered_summaries
        .iter()
        .map(|summary| {
            (
                summary.transaction_purpose().unwrap().category().unwrap(),
                summary
                    .transaction_purpose()
                    .unwrap()
                    .sub_category()
                    .unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    let mut breakage_list = vec![];

    for tup in unique_purposes {
        breakage_list.push(BreakageByPurpose {
            category: Option::from(tup.0.clone()),
            sub_category: Option::from(tup.1.clone()),
            amount: Option::from(get_total_by_purpose(
                filttered_summaries.as_ref(),
                tup.0.clone(),
                tup.1.clone(),
            )),
        })
    }

    breakage_list
}
fn get_aggregate_view(summaries: &Vec<TransactionSummary>) -> TransactionAggregation {
    get_list_of_breakage_by_purpose(summaries, Credit);

    TransactionAggregation {
        total_credit: Option::from(get_total_by_type(summaries, Credit)),
        total_debit: Option::from(get_total_by_type(summaries, Debit)),
        credit_breakage_by_purpose: Option::from(get_list_of_breakage_by_purpose(
            summaries, Credit,
        )),
        debit_breakage_by_purpose: Option::from(get_list_of_breakage_by_purpose(summaries, Debit)),
    }
}

fn map_to_summaries(records: Vec<TransactionRecord>) -> Vec<TransactionSummary> {
    let mut summaries = vec![];

    for record in records {
        if !record.is_complete() {
            continue;
        }

        summaries.push(TransactionSummary {
            transaction_id: Option::from(record.seq.unwrap().to_string()),
            transaction_type: get_transaction_type(&record),
            amount: get_transaction_amount(&record),
            transaction_date: get_transaction_date(&record),
            transaction_purpose: get_transaction_purpose(&record),
            transaction_date_string: Option::from(record.transaction_date.unwrap().to_string()),
        });
    }

    summaries
    /*.iter()
    .cloned()
    .filter(|summary| {
        summary
            .transaction_purpose()
            .unwrap()
            .category()
            .unwrap()
            .clone()
            == TransactionCategory::Others
            && summary
                .transaction_purpose()
                .unwrap()
                .sub_category()
                .unwrap()
                .clone()
                == TransactionSubCategory::Others
    })
    .collect()*/
}

fn get_category_from_subcategory(sub_category: TransactionSubCategory) -> TransactionCategory {
    let sub_category_to_category = HashMap::from([
        (
            vec![Crypto, Divident, Equity, MutualFunds, Rd, Zerodha],
            Investment,
        ),
        (vec![Operation, HealthCheckUp], Medical),
        (vec![Food, Travel, CreditCard], Leisure),
        (vec![Amazon], Salary),
        (vec![Internet, Flat, Grocery], Living),
    ]);

    for (sub_categories, category) in &sub_category_to_category {
        if sub_categories.contains(&sub_category) {
            return category.clone();
        }
    }
    TransactionCategory::Others
}

fn is_present_in_vec(vec_of_str: Vec<&str>, search_str: &str) -> bool {
    for s in vec_of_str {
        if search_str.contains(s) {
            return true;
        }
    }
    false
}

fn get_sub_category_from_remark(remark: &str) -> TransactionSubCategory {
    let binding = remark.to_lowercase();
    let str_to_match = binding.as_str();

    if str_to_match.contains("to rd") || str_to_match.contains("ppf account") {
        return Rd;
    }

    if is_present_in_vec(vec!["bhupendra", "mygate"], str_to_match) {
        return Flat;
    }

    if is_present_in_vec(vec!["bbdaily", "pay to shoppy m"], str_to_match) {
        return Grocery;
    }

    if is_present_in_vec(vec!["airteli"], str_to_match) {
        return Internet;
    }

    if is_present_in_vec(vec!["zomato", "swiggy"], str_to_match) {
        return Food;
    }

    if is_present_in_vec(vec!["dg04", "vssl"], str_to_match)
        || str_to_match.starts_with("eba/nse")
        || str_to_match.starts_with("eba/prepaid")
    {
        return Equity;
    }

    if str_to_match.contains("sal") && str_to_match.starts_with("ach/") {
        return Amazon;
    }

    if str_to_match.starts_with("eba/mf") {
        return MutualFunds;
    }

    if is_present_in_vec(vec!["smallcase", "zerodha"], str_to_match) {
        return Zerodha;
    }

    if is_present_in_vec(
        vec![
            "spice tree",
            "makemytrip",
            "uberrides",
            "olacabs",
            "olamoney",
            "irctc",
            "insane tra",
            "uber",
            "cleartrip",
        ],
        str_to_match,
    ) {
        return Travel;
    }

    if str_to_match.contains("narayananethral") {
        return Operation;
    }

    if str_to_match.contains("orangehealth") {
        return HealthCheckUp;
    }

    if str_to_match.contains("atd/auto debit") {
        return CreditCard;
    }

    if str_to_match.starts_with("ach") || str_to_match.starts_with("cms/") {
        return Divident;
    }

    TransactionSubCategory::Others
}

fn get_transaction_purpose(record: &TransactionRecord) -> Option<TransactionPurpose> {
    let remark = record.remark.as_ref().unwrap();
    let sub_category = get_sub_category_from_remark(remark.as_str());
    let category = get_category_from_subcategory(sub_category.clone());

    Option::from(TransactionPurpose {
        category: Option::from(category),
        sub_category: Option::from(sub_category),
        remark: Option::from(remark.clone()),
    })
}

const FORMAT: &str = "%d/%m/%y %H:%M:%S %z";
const OFFSET: &str = "10:00:00 +05:30";

fn get_transaction_date(record: &TransactionRecord) -> Option<DateTime> {
    let offset_date_time = &format!("{} {}", record.transaction_date.as_ref().unwrap(), OFFSET);

    let date_time =
        chrono::DateTime::parse_from_str(offset_date_time, FORMAT).unwrap_or_else(|error| {
            panic!(
                "Not able to format {:?} due to {:?}",
                offset_date_time.clone(),
                error
            )
        });

    Option::from(DateTime::from_millis(date_time.timestamp_millis()))
}

fn get_transaction_amount(record: &TransactionRecord) -> Option<f64> {
    if get_transaction_type(record).unwrap() == TransactionType::Credit {
        return Option::from(record.deposit.unwrap());
    }

    Option::from(record.withdrawal.unwrap())
}

fn get_transaction_type(record: &TransactionRecord) -> Option<TransactionType> {
    if record.deposit.unwrap() == 0.0 {
        return Option::from(Debit);
    }

    Option::from(Credit)
}

#[derive(Debug, Deserialize)]
struct TransactionRecord {
    #[serde(rename = "S No.")]
    #[serde(deserialize_with = "csv::invalid_option")]
    seq: Option<i32>,
    #[serde(rename = "Value Date")]
    #[serde(deserialize_with = "csv::invalid_option")]
    value_date: Option<String>,
    #[serde(rename = "Transaction Date")]
    #[serde(deserialize_with = "csv::invalid_option")]
    transaction_date: Option<String>,
    #[serde(rename = "Cheque Number")]
    #[serde(default)]
    #[serde(deserialize_with = "csv::invalid_option")]
    cheque_number: Option<String>,
    #[serde(rename = "Transaction Remarks")]
    #[serde(default)]
    #[serde(deserialize_with = "csv::invalid_option")]
    remark: Option<String>,
    #[serde(rename = "Withdrawal Amount (INR )")]
    #[serde(deserialize_with = "csv::invalid_option")]
    withdrawal: Option<f64>,
    #[serde(rename = "Deposit Amount (INR )")]
    #[serde(deserialize_with = "csv::invalid_option")]
    deposit: Option<f64>,
    #[serde(rename = "Balance (INR )")]
    #[serde(deserialize_with = "csv::invalid_option")]
    balance: Option<f64>,
}

impl TransactionRecord {
    pub fn is_complete(&self) -> bool {
        if self.seq == None || self.transaction_date == None || self.balance == None {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::list_transactions::FORMAT;
    use chrono::TimeZone;
    use liwimean_ims_core_server_sdk::types::DateTime;

    #[test]
    fn validate_s1() {
        let date_time =
            chrono::DateTime::parse_from_str("10/10/22 22:10:57 +05:30", FORMAT).unwrap();

        let _x = DateTime::from_millis(date_time.timestamp_millis());

        println!("{:?}", DateTime::from_millis(date_time.timestamp_millis()));
    }
}
