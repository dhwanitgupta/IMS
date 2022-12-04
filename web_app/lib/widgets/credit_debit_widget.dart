import 'package:provider/provider.dart';
import 'package:flutter/material.dart';
import 'package:syncfusion_flutter_charts/charts.dart';
import '../models/credit_debit_model.dart';

class CreditDebitWidget extends StatefulWidget {
  const CreditDebitWidget({super.key, required this.creditDebitModel});

  final CreditDebitModel creditDebitModel;

  @override
  State<StatefulWidget> createState() => _CreditDebitWidget();
}

class _CreditDebitWidget extends State<CreditDebitWidget> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    var creditDebitModel = Provider.of<CreditDebitModel>(context);
    var limits = creditDebitModel.getMinMaxAmount();
    return SfCartesianChart(
      plotAreaBorderWidth: 1,
      zoomPanBehavior:
          ZoomPanBehavior(enableMouseWheelZooming: true, enablePinching: true),
      title: ChartTitle(
          text: "Credit and Debit",
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

  List<SplineSeries<CreditDebitData, String>> _getSeries() {
    List<CreditDebitData> creditDebitList =
        widget.creditDebitModel.getCreditDebitDataList();

    return <SplineSeries<CreditDebitData, String>>[
      SplineSeries<CreditDebitData, String>(
          dataSource: creditDebitList,
          xValueMapper: (CreditDebitData data, _) => data.windowName,
          yValueMapper: (CreditDebitData data, _) => data.credit,
          markerSettings: const MarkerSettings(isVisible: true),
          name: "Credit"),
      SplineSeries<CreditDebitData, String>(
          dataSource: creditDebitList,
          xValueMapper: (CreditDebitData data, _) => data.windowName,
          yValueMapper: (CreditDebitData data, _) => data.debit,
          markerSettings: const MarkerSettings(isVisible: true),
          name: "Debit")
    ];
  }
}
