import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/generated/bridge_generated.dart';

class TablePage extends StatefulWidget {
  const TablePage({Key? key}) : super(key: key);

  @override
  State<TablePage> createState() => _TablePageState();
}

class _TablePageState extends State<TablePage> {
  List<User> data = [];

  @override
  void initState() {
    super.initState();
    data.addAll(Iterable<int>.generate(20)
        .map((e) => User(id: e, name: "User $e", readed: false)));
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(1),
      child: InteractiveViewer(
          constrained: false,
          child: Scrollbar(
            child: DataTable(
              border: TableBorder.all(color: Colors.grey),
              columns: const [
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
                DataColumn(label: Text("a")),
              ],
              rows: data.map((e) {
                return DataRow(cells: [
                  DataCell(Text("${e.id}")),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                  DataCell(Text(e.name)),
                ]);
              }).toList(),
            ),
          )),
    );
  }
}
