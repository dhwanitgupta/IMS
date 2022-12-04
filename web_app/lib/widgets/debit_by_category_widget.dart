import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:syncfusion_flutter_charts/charts.dart';
import 'package:web_app/models/debit_by_category_model.dart';

class AggregationByCategoryWidget extends StatefulWidget {
  const AggregationByCategoryWidget({super.key, required this.aggregationByCategoryModel, required this.title});

  final AggregationByCategoryModel aggregationByCategoryModel;
  final String title;

  @override
  State<StatefulWidget> createState() => _AggregationByCategoryWidget();
}

class _AggregationByCategoryWidget extends State<AggregationByCategoryWidget> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    var aggregationByCategoryModel = Provider.of<AggregationByCategoryModel>(context);
    var limits = aggregationByCategoryModel.getMinMax();

    return SfCartesianChart(
      plotAreaBorderWidth: 1,
      zoomPanBehavior:
      ZoomPanBehavior(enableMouseWheelZooming: true),
      title: ChartTitle(
          text: widget.title,
          borderWidth: 2,
          borderColor: Colors.grey,
          backgroundColor: Colors.greenAccent),
      legend: Legend(isVisible: true),
      primaryXAxis: CategoryAxis(
          majorGridLines: const MajorGridLines(width: 1),
          labelPlacement: LabelPlacement.onTicks),
      primaryYAxis: NumericAxis(
          minimum: limits.minVal * 0.8,
          maximum: limits.maxVal * 1.2,
          axisLine: const AxisLine(width: 1),
          edgeLabelPlacement: EdgeLabelPlacement.shift,
          labelFormat: '{value}',
          majorTickLines: const MajorTickLines(size: 10)),
      series: _getSeries(),
      tooltipBehavior: TooltipBehavior(enable: true),
    );
  }

  List<SplineSeries<Stat, String>> _getSeries() {
    List<CategoryToStatsData> debitByCategoryList =
    widget.aggregationByCategoryModel.getDebitByCategoryLists();

    List<SplineSeries<Stat, String>> series = [];

    for (var debitByCategory in debitByCategoryList) {
      print(debitByCategory.stats);
      series.add(SplineSeries<Stat, String>(
          dataSource: debitByCategory.stats,
          xValueMapper: (Stat data, _) => data.name,
          yValueMapper: (Stat data, _) => data.value,
          markerSettings: const MarkerSettings(isVisible: true),
          name: debitByCategory.category
      ));
    }

    return series;
  }
}
