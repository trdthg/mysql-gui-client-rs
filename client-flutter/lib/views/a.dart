import 'package:flutter/material.dart';
import 'tabPage/chat/tab2.dart';
import 'tabPage/tab1.dart';

class Page1 extends StatefulWidget {
  Page1({Key? key}) : super(key: key);

  final List<Widget> _widgets = [const Tab1Page(), const Tab3Page()];

  @override
  State<Page1> createState() => _Page1State();
}

class _Page1State extends State<Page1>
    with SingleTickerProviderStateMixin {
  List<String> tabNames = ["页面", "观察"];

  final int _index = 0;
  late TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(
        initialIndex: _index, length: tabNames.length, vsync: this);
  }

  @override
  void dispose() {
    super.dispose();
    _tabController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          leading: IconButton(onPressed: () {}, icon: const Icon(Icons.home)),
          centerTitle: true,
          title: const Text("测试"),
          actions: [IconButton(onPressed: () {}, icon: const Icon(Icons.add))],
          bottom: PreferredSize(
              child: TabBar(
                  controller: _tabController,
                  tabs: tabNames.map<Widget>((e) {
                    return Text(e);
                  }).toList()),
              preferredSize: const Size.fromHeight(20)),
          elevation: 4,
        ),
        body: TabBarView(
          children: widget._widgets,
          controller: _tabController,
        ));
  }
}
