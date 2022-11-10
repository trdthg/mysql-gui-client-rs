import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/views/c.dart';

import 'a.dart';
import 'b.dart';
import 'd.dart';
import 'profile/profile.dart';
import 'e.dart';

class Home extends StatefulWidget {
  const Home({Key? key}) : super(key: key);

  @override
  _HomeState createState() => _HomeState();
}

class _HomeState extends State<Home> {
  int _index = 0;

  List<Widget> widgets = [
    Page1(),
    const PageAPage(),
    const PreviewPage(),
    const PageBPage(),
    const TablePage()
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: Drawer(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              color: Colors.blue,
              child: const Padding(
                padding: EdgeInsets.only(top: 150),
                child: SizedBox(),
              ),
            ),
            const Text("功能 1"),
            const Text("功能 1"),
            const Text("功能 1"),
            const Text("功能 1"),
          ],
        ),
      ),
      endDrawer: const ProfilePage(),
      body: widgets[_index],

      // floatingActionButton: FloatingActionButton(
      //   onPressed: () {},
      //   child: const Icon(Icons.widgets_rounded),
      // ),
      // floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,

      bottomNavigationBar: BottomAppBar(
        //     showSelectedLabels: true,
        //     showUnselectedLabels: true,
        //     selectedItemColor: Colors.black,
        //     unselectedItemColor: Colors.grey.shade700,
        //     backgroundColor: Colors.red,
        // backgroundColor: Colors.amber,
        shape: const CircularNotchedRectangle(),
        child: Builder(builder: (BuildContext context) {
          return Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              IconButton(
                  onPressed: () {
                    setState(() {
                      _index = 0;
                    });
                  },
                  icon: const Icon(Icons.home_outlined)),
              IconButton(
                  onPressed: () {
                    setState(() {
                      _index = 1;
                    });
                  },
                  icon: const Icon(Icons.amp_stories_outlined)),
              // 占位符
              // Container(
              //   color: Colors.blue,
              //   child: const SizedBox(),
              // ),
              IconButton(
                  onPressed: () {
                    setState(() {
                      _index = 2;
                    });
                  },
                  icon: const Icon(Icons.widgets_rounded)),
              IconButton(
                  onPressed: () {
                    setState(() {
                      _index = 3;
                    });
                  },
                  icon: const Icon(Icons.subscriptions_outlined)),
              IconButton(
                  onPressed: () {
                    setState(() {
                      _index = 4;
                    });
                  },
                  icon: const Icon(Icons.space_dashboard_outlined)),
            ],
          );
        }),
      ),
    );
  }
}
