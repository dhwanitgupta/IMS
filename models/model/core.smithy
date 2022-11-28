$version: "2"

namespace ims.core

use aws.protocols#restJson1
use aws.api#service

@service(sdkId: "ImsCore")
@restJson1
@title("Investment Management Service")
@documentation("""
This exposes transaction resource and also expose APIs to support transaction views
""")
service ImsCore {
    version: "2022-05-19",
    resources: [Transaction, Session]
}

@documentation("""
Session is exposed to maintain the state of the development environment, it has settings (configuration) and deployment
(action) as child resources. Session captures the configuration/state and actions of development environment
""")
resource Session {
    identifiers: {
        sessionId: String
    },
    create: StartSession
}

@documentation("""
This is a POST API, which creates the session against a transaction directory which is present in local machine.
Currently it is performing two functions 1) creating session, and 2) loading transactions.
""")
@http(uri: "/api/v1/session", method: "POST")
operation StartSession {
    input := {
             @required tlr: String
             @required transactionsDirectoryPath: String
         },
    output := {
             @required sessionId: String,
    },
    errors: [InvalidTransactionDirectory]
}

@error("server")
@documentation("This represents the internal error while fetching the transactions in server")
structure InvalidTransactionDirectory {}

@documentation("""
Resource Hold Transaction
""")
resource Transaction {
    identifiers: {
        transactionId: String
    },
    list: ListTransactions
}

@readonly
@http(uri: "/api/v1/core/{tlr}/transactions", method: "GET")
@documentation("""
GET API which is used by users to query transactions
""")
operation ListTransactions {
    input :=  { @required @httpLabel tlr: String },
    output :=  { @required transactions: TransactionsList, @required aggregate: TransactionAggregation },
    errors: [ErrorLoadingTransactions]
}

@error("server")
@documentation("This represents the internal error while fetching the transactions in server")
structure ErrorLoadingTransactions {}

@documentation("This contains list of Transactions")
list TransactionsList {
    member: TransactionSummary
}

@documentation("This contains the transaction summary")
structure TransactionSummary {
    transactionId: String,
    transactionType: TransactionType,
    amount: Double,
    transactionDate: Timestamp,
    transactionPurpose: TransactionPurpose,
}

structure TransactionPurpose {
    category: TransactionCategory,
    subCategory: TransactionSubCategory,
    remark: String
}

structure TransactionAggregation {
    total_credit: Double,
    total_debit: Double,
    credit_breakage_by_purpose: BreakageByPurposeList,
    debit_breakage_by_purpose: BreakageByPurposeList
}

list BreakageByPurposeList {
    member: BreakageByPurpose
}

structure BreakageByPurpose {
    category: TransactionCategory,
    subCategory: TransactionSubCategory,
    amount: Double
}

enum TransactionType {
    DEBIT,
    CREDIT
}

enum TransactionCategory {
    INVESTMENT,
    LEISURE,
    SALARY,
    MEDICAL,
    LIVING,
    OTHERS
}

enum TransactionSubCategory {
    MUTUAL_FUNDS,
    EQUITY,
    RD,
    DIVIDENT,
    ZERODHA,
    OTHERS,
    HEALTH_CHECK_UP,
    OPERATION,
    CRYPTO,
    TRAVEL,
    FOOD,
    CREDIT_CARD,
    AMAZON,
    FLAT,
    INTERNET,
    GROCERY
}
