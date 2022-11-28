import 'dart:math';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:syncfusion_flutter_charts/charts.dart';

import '../models/transaction_summary.dart';

class CreditDebitWidget extends StatefulWidget {
  const CreditDebitWidget({super.key, required this.transactionSummary});

  final TransactionSummary transactionSummary;

  @override
  State<StatefulWidget> createState() => _CreditDebitWidget();
}

class _CreditDebitWidget extends State<CreditDebitWidget> {

  late List<CreditDebitData> creditDebitList;

  @override
  void initState() {
    creditDebitList = <CreditDebitData>[
      CreditDebitData(timeRange: "Jan", credit: 100, debit: 50),
      CreditDebitData(timeRange: "Feb", credit: 200, debit: 150),
      CreditDebitData(timeRange: "Mar", credit: 50, debit: 270),
      CreditDebitData(timeRange: "Apr",
        credit: widget.transactionSummary.totalCredit,
        debit: widget.transactionSummary.totalDebit)
    ];
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return SfCartesianChart(
      plotAreaBorderWidth: 10,
      title: ChartTitle(
        text: "Monthly Credit and Debit",
        borderWidth: 2,
        borderColor: Colors.grey,
        backgroundColor: Colors.greenAccent
      ),
      legend: Legend(isVisible: true),
      primaryXAxis: CategoryAxis(
        majorGridLines: const MajorGridLines(width: 1),
        labelPlacement: LabelPlacement.onTicks
      ),
      primaryYAxis: NumericAxis(
        minimum: 10,
        maximum: max(widget.transactionSummary.totalCredit, widget.transactionSummary.totalDebit),
        axisLine: const AxisLine(width: 1),
        edgeLabelPlacement: EdgeLabelPlacement.shift,
        labelFormat: '{value}',
        majorTickLines: const MajorTickLines(size: 10)
      ),
      series: _getSeries(),
      tooltipBehavior: TooltipBehavior(enable: true),
    );
  }

  List<SplineSeries<CreditDebitData, String>> _getSeries() {
    return <SplineSeries<CreditDebitData, String>>[
      SplineSeries<CreditDebitData, String>(
        dataSource: creditDebitList,
        xValueMapper: (CreditDebitData data, _) => data.timeRange,
        yValueMapper: (CreditDebitData data, _) => data.credit,
        markerSettings: const MarkerSettings(isVisible: true),
        name: "Credit"
      ),
      SplineSeries<CreditDebitData, String>(
        dataSource: creditDebitList,
        xValueMapper: (CreditDebitData data, _) => data.timeRange,
        yValueMapper: (CreditDebitData data, _) => data.debit,
        markerSettings: const MarkerSettings(isVisible: true),
        name: "Debit"
      )
    ];
  }
}

class CreditDebitData {
  CreditDebitData({
    required this.timeRange,
    required this.credit,
    required this.debit
  });

  final String timeRange;
  final double credit;
  final double debit;
}