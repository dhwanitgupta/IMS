import 'dart:math';
import 'package:flutter/cupertino.dart';
import 'package:ims_dart_client/api.dart';
import 'package:web_app/models/credit_debit_model.dart';

class AggregationByCategoryModel extends ChangeNotifier {
  late AggregationByCategory aggregationByCategory;

  AggregationByCategoryModel() {
    aggregationByCategory = AggregationByCategory();
  }

  void update(AggregationByCategory aggregationByCategory) {
    this.aggregationByCategory = aggregationByCategory;
    notifyListeners();
  }

  MinMax getMinMax() {

    if (aggregationByCategory.categoryToStatsMap.isEmpty) {
      return MinMax(minVal: 0, maxVal: 1);
    }

    var smin = getDebitByCategoryLists().first.stats.first.value;
    var smax = smin;

    for (var statData in getDebitByCategoryLists()){
       var tmin = statData.stats.reduce((e1, e2) => e1.value > e2.value? e2:e1).value;
       var tmax = statData.stats.reduce((e1, e2) => e1.value < e2.value? e2:e1).value;

       smin = min(tmin, smin);
       smax = max(tmax, smax);
    }

    return MinMax(minVal: smin, maxVal: smax);
  }

  List<CategoryToStatsData> getDebitByCategoryLists() {

    List<CategoryToStatsData> categoryToStats = [];

    if (aggregationByCategory.categoryToStatsMap.isNotEmpty) {
      aggregationByCategory.categoryToStatsMap.forEach((category, stats) {
        categoryToStats.add(
            CategoryToStatsData(category: category,
              stats: stats.map((e) => Stat(name: e.windowName!,
                  value: e.windowValue!)).toList()
            )
        );
      });
    }

    return categoryToStats;
  }
}

class CategoryToStatsData {
  final String category;
  final List<Stat> stats;

  CategoryToStatsData({required this.category, required this.stats});
}

class Stat {
  final String name;
  final double value;
  Stat({required this.name, required this.value});

  @override
  String toString() {
    return '{name: ${name}, value: ${value}}';
  }
}