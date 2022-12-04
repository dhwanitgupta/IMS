import 'dart:math';
import 'package:flutter/cupertino.dart';
import 'package:ims_dart_client/api.dart';

class CreditDebitModel extends ChangeNotifier {
  late AggregationByType aggregationByType;

  CreditDebitModel() {
    aggregationByType = AggregationByType();
  }

  void updateAggregationByType(AggregationByType aggregationByType) {
    this.aggregationByType = aggregationByType;
    notifyListeners();
  }

  List<CreditDebitData> getCreditDebitDataList() {
    List<CreditDebitData> creditDebitList = [];

    if (aggregationByType.credit.isEmpty) return creditDebitList;

    for (var element in aggregationByType.credit) {
      var debit =
          _getValueFromWindowName(element.windowName!, aggregationByType.debit);

      creditDebitList.add(CreditDebitData(
          windowName: element.windowName!,
          credit: element.windowValue!,
          debit: debit));
    }

    return creditDebitList;
  }

  double _getValueFromWindowName(
      String windowName, List<AggregationStat> typeList) {
    for (var element in typeList) {
      if (element.windowName == windowName) {
        return element.windowValue!;
      }
    }
    return 0;
  }

  MinMax getMinMaxAmount() {
    if (aggregationByType.credit.isEmpty) return MinMax(minVal: 0, maxVal: 10);

    var creditMinMax = _getMinMaxFromAggType(aggregationByType.credit);
    var debitMinMax = _getMinMaxFromAggType(aggregationByType.debit);

    return MinMax(
        minVal: min(creditMinMax.minVal, debitMinMax.minVal),
        maxVal: max(creditMinMax.maxVal, debitMinMax.maxVal));
  }

  MinMax _getMinMaxFromAggType(List<AggregationStat> typeList) {
    double minAmount = typeList.first.windowValue!;
    double maxAmount = minAmount;

    for (var element in aggregationByType.credit) {
      if (element.windowValue! > maxAmount) {
        maxAmount = element.windowValue!;
      }

      if (element.windowValue! < minAmount) {
        minAmount = element.windowValue!;
      }
    }

    return MinMax(minVal: minAmount, maxVal: maxAmount);
  }
}

class CreditDebitData {
  CreditDebitData(
      {required this.windowName, required this.credit, required this.debit});

  final String windowName;
  final double credit;
  final double debit;
}

class MinMax {
  MinMax({required this.minVal, required this.maxVal});

  final double minVal;
  final double maxVal;
}
