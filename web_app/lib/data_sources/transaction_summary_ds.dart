import 'dart:async';
import 'package:ims_dart_client/api.dart';

import '../models/transaction_summary.dart' as TS;

class TransactionSummaryDataSource {
  static const String TRANSACTION_SUMMARY_API_URL =
      "http://127.0.0.1:12446";

  TS.TransactionSummary fetch()  {

    final apiInstance = DefaultApi(ApiClient(basePath: TRANSACTION_SUMMARY_API_URL));
    TS.TransactionSummary result = const TS.TransactionSummary(totalCredit: 1600, totalDebit: 32111.41);


    try {
      var call = apiInstance.listTransactions("dhwanit");

      apiInstance.listTransactions("dhwanit").then((value) {
        result =
            TS.TransactionSummary(totalDebit: value!.aggregate.totalDebit ?? 0,
                totalCredit: value!.aggregate.totalCredit ?? 0);
            print(result.totalDebit);
        }
      );

      print(result.totalDebit);

      return result;

    } catch (e) {
      rethrow;
    }
  }
}
