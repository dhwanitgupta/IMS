import 'package:ims_dart_client/api.dart';
import 'package:web_app/data_sources/ims_post_client.dart';
import '../models/credit_debit_model.dart';
import '../models/debit_by_category_model.dart';

class TransactionSummaryDataSource {
  static const String TRANSACTION_SUMMARY_API_URL = "http://127.0.0.1:12446";
  CreditDebitModel creditDebitModel;
  AggregationByCategoryModel debitByCategoryModel, creditByCategoryModel;

  TransactionSummaryDataSource({required this.creditDebitModel, required this.debitByCategoryModel, required  this.creditByCategoryModel});

  void fetch() {
    final apiClient = ApiClient(basePath: TRANSACTION_SUMMARY_API_URL);
    apiClient.client = IMSPostClient();
    final apiInstance = DefaultApi(apiClient);

    try {
      GetAggregatedTransactionAnalysisByFiltersRequestContent request =
          GetAggregatedTransactionAnalysisByFiltersRequestContent(
              aggregationConfig:
                  AggregationConfig(window: AggregationWindow.MONTH));

      apiInstance
          .getAggregatedTransactionAnalysisByFilters("dhwanit", request)
          .then((value) {
        creditDebitModel
            .updateAggregationByType(value!.response.aggregationByType!);
        debitByCategoryModel.update(value!.response.debitByCategory!);
        creditByCategoryModel.update(value!.response.creditByCateegory!);
      });
    } catch (e) {
      creditDebitModel.updateAggregationByType(AggregationByType());
      debitByCategoryModel.update(AggregationByCategory());
    }
  }
}
