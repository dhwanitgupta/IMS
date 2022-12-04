import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:web_app/data_sources/transaction_summary_ds.dart';
import 'package:web_app/models/credit_debit_model.dart';
import 'package:web_app/models/debit_by_category_model.dart';
import 'package:web_app/widgets/credit_debit_widget.dart';
import 'package:web_app/widgets/debit_by_category_widget.dart';

void main() {
  runApp(const ImsApp());
}

class ImsApp extends StatelessWidget {
  const ImsApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'IMS',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const HomePage(title: 'IMS Home'),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key, required this.title});

  final String title;

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late CreditDebitModel creditDebitModel;
  late AggregationByCategoryModel debitByCategoryModel, creditByCategoryModel;

  @override
  void initState() {
    super.initState();
    creditDebitModel = CreditDebitModel();
    debitByCategoryModel = AggregationByCategoryModel();
    creditByCategoryModel = AggregationByCategoryModel();

    TransactionSummaryDataSource(creditDebitModel: creditDebitModel,
        debitByCategoryModel: debitByCategoryModel, creditByCategoryModel: creditByCategoryModel).fetch();
  }

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
        providers: [
          ChangeNotifierProvider<CreditDebitModel>(create: (_) => creditDebitModel),
          ChangeNotifierProvider<AggregationByCategoryModel>(create: (_) => debitByCategoryModel)
        ],
        child: Scaffold(
          appBar: AppBar(
            title: Text(widget.title),
          ),
          body: Center(
            child: ListView(children: <Widget>[
              CreditDebitWidget(creditDebitModel: creditDebitModel),
              AggregationByCategoryWidget(aggregationByCategoryModel: debitByCategoryModel, title: "Debit By Category"),
              AggregationByCategoryWidget(aggregationByCategoryModel: creditByCategoryModel, title: "Credit By Category")
            ]),
          ),
        ));
  }
}
