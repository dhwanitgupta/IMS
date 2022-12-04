use crate::operations::list_transactions::get_transaction_output;
use crate::State;
use aws_smithy_http_server::Extension;
use chrono::{Datelike, Month, NaiveDateTime};
use liwimean_ims_core_server_sdk::error::GetAggregatedTransactionAnalysisByFiltersError;
use liwimean_ims_core_server_sdk::input::GetAggregatedTransactionAnalysisByFiltersInput;
use liwimean_ims_core_server_sdk::model::TransactionType::{Credit, Debit};
use liwimean_ims_core_server_sdk::model::{
    AggregationByCategory, AggregationByType, AggregationConfig, AggregationStat,
    AggregationWindow, AnalysisResponse, TransactionCategory, TransactionSummary, TransactionType,
};
use liwimean_ims_core_server_sdk::output::GetAggregatedTransactionAnalysisByFiltersOutput;
use polars::df;
use polars::export::num::{FromPrimitive, ToPrimitive};
use polars::frame::DataFrame;
use polars::prelude::{col, IntoLazy, NamedFrom};
use polars::prelude::{Series, SortOptions};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_aggregated_transaction_analysis_by_filters(
    input: GetAggregatedTransactionAnalysisByFiltersInput,
    state: Extension<Arc<State>>,
) -> Result<
    GetAggregatedTransactionAnalysisByFiltersOutput,
    GetAggregatedTransactionAnalysisByFiltersError,
> {
    let result = state.resources_map.get(input.tlr()).unwrap();
    let path = &result.transaction_path;
    let output = get_transaction_output(path);
    let transactions = output.transactions();

    let df = df! [
      "id" => transactions.iter().clone().map(|t| t.transaction_id().unwrap().to_string()).collect::<Vec<String>>(),
      "type" => transactions.iter().clone().map(|t| t.transaction_type().unwrap().clone().as_str().to_string()).collect::<Vec<String>>(),
      "amount" => transactions.iter().clone().map(|t| t.amount().unwrap()).collect::<Vec<f64>>(),
      "category" => transactions.iter().clone().map(|t| t.transaction_purpose().unwrap().category().unwrap().as_str().to_string()).collect::<Vec<String>>(),
      "date" => transactions.iter().clone().map(|t| t.transaction_date().unwrap().to_millis().unwrap()).collect::<Vec<i64>>(),
      "window" => transactions.iter().clone().map(|t| get_window(t, input.clone().aggregation_config())).collect::<Vec<i64>>(),
      "displayable_window" => transactions.iter().clone().map(|t| get_displayable_window(t, input.clone().aggregation_config())).collect::<Vec<String>>(),
    ].unwrap();

    let aggr_by_type = df
        .clone()
        .lazy()
        .groupby(["window", "type"])
        .agg([
            col("amount").sum(),
            col("displayable_window").unique(),
            col("date").unique(),
        ])
        .sort("window", SortOptions::default())
        .collect()
        .unwrap();

    let aggregation_by_type = get_aggregation_by_type(aggr_by_type);

    Ok(GetAggregatedTransactionAnalysisByFiltersOutput {
        response: AnalysisResponse {
            aggregation_by_type: Some(aggregation_by_type),
            debit_by_category: Some(get_type_by_category(df.clone(), Debit)),
            credit_by_cateegory: Some(get_type_by_category(df, Credit)),
        },
    })
}

fn get_type_by_category(df: DataFrame, transaction_type: TransactionType) -> AggregationByCategory {
    let predicate = col("type").str().contains(transaction_type.as_str());

    let type_by_category = df
        .lazy()
        .filter(predicate)
        .groupby(["window", "category"])
        .agg([
            col("amount").sum(),
            col("displayable_window").unique(),
            col("date").unique(),
            col("type").unique(),
        ])
        .sort("window", SortOptions::default())
        .collect()
        .unwrap();

    println!("{:?}", type_by_category);

    get_aggregation_by_category(type_by_category)
}

fn get_aggregation_by_category(debit_by_category: DataFrame) -> AggregationByCategory {
    let mut category_to_states_map: HashMap<TransactionCategory, Vec<AggregationStat>> =
        HashMap::new();
    let mut iters = debit_by_category
        .iter()
        .map(|s| s.iter())
        .collect::<Vec<_>>();

    for _row in 0..debit_by_category.height() {
        let mut col = 0;
        let mut category = TransactionCategory::Others;
        let mut window_name: Option<String> = None;
        let mut window_value: Option<f64> = None;

        for iter in &mut iters {
            let value = iter.next().unwrap();
            match col {
                1 => {
                    let category_str: String =
                        serde_json::from_str(value.clone().to_string().as_str()).unwrap();

                    category = TransactionCategory::from_str(category_str.as_str()).unwrap();
                }
                2 => {
                    window_value = Some(value.clone().to_string().parse::<f64>().unwrap());
                }
                3 => {
                    let val: Vec<String> =
                        serde_json::from_str(value.clone().to_string().as_str()).unwrap();
                    window_name = Some(val.first().unwrap().to_string());
                }
                _ => {}
            }
            col += 1;
        }

        if !category_to_states_map.contains_key(&category) {
            category_to_states_map.insert(category.clone(), Vec::new());
        }

        category_to_states_map
            .get_mut(&category)
            .unwrap()
            .push(AggregationStat {
                window_name,
                window_value,
            });
    }

    AggregationByCategory {
        category_to_stats_map: Option::from(category_to_states_map),
    }
}

fn get_aggregation_by_type(aggr_by_type: DataFrame) -> AggregationByType {
    let mut credit_type = vec![];
    let mut debit_type = vec![];

    let mut iters = aggr_by_type.iter().map(|s| s.iter()).collect::<Vec<_>>();

    for _row in 0..aggr_by_type.height() {
        let mut col = 0;
        let mut transaction_type = Credit;
        let mut window_name: Option<String> = None;
        let mut window_value: Option<f64> = None;

        for iter in &mut iters {
            let value = iter.next().unwrap();
            if col == 1 && value.to_string().contains(Debit.as_str()) {
                transaction_type = Debit;
            }

            if col == 2 {
                window_value = Some(value.clone().to_string().parse::<f64>().unwrap());
            }

            if col == 3 {
                let val: Vec<String> =
                    serde_json::from_str(value.clone().to_string().as_str()).unwrap();
                window_name = Some(val.first().unwrap().to_string());
            }

            col += 1;
        }

        if transaction_type == Credit {
            credit_type.push(AggregationStat {
                window_name,
                window_value,
            });
        } else {
            debit_type.push(AggregationStat {
                window_name,
                window_value,
            });
        }
    }

    AggregationByType {
        credit: Some(credit_type),
        debit: Some(debit_type),
    }
}

fn get_window(transaction: &TransactionSummary, config: &AggregationConfig) -> i64 {
    let naive_date = get_naive_date(transaction);

    match config.window().unwrap().clone() {
        AggregationWindow::Day => naive_date.timestamp_millis(),
        AggregationWindow::Month => naive_date.month().to_i64().unwrap(),
        AggregationWindow::Week => naive_date.iso_week().week().to_i64().unwrap(),
        AggregationWindow::Year => naive_date.year().to_i64().unwrap(),
    }
}

fn get_displayable_window(transaction: &TransactionSummary, config: &AggregationConfig) -> String {
    let naive_date = get_naive_date(transaction);

    match config.window().unwrap().clone() {
        AggregationWindow::Day => transaction.transaction_date_string().unwrap().to_string(),
        AggregationWindow::Month => format!(
            "{} {}",
            Month::from_u32(naive_date.month()).unwrap().name(),
            naive_date.year()
        ),
        AggregationWindow::Week => {
            format!("week{} {}", naive_date.iso_week().week(), naive_date.year())
        }
        AggregationWindow::Year => naive_date.year().to_string(),
    }
}

fn get_naive_date(transaction: &TransactionSummary) -> NaiveDateTime {
    let transaction_date = transaction
        .clone()
        .transaction_date()
        .unwrap()
        .to_millis()
        .unwrap();

    chrono::NaiveDateTime::from_timestamp_millis(transaction_date).unwrap()
}
