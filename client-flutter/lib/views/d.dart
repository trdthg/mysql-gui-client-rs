import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/states/count_controller.dart';
import 'package:get/get.dart';

class PageBPage extends StatelessWidget {
  const PageBPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final CountController c = Get.find();
    return Scaffold(
      appBar: AppBar(
        title: const Text("第四页"),
      ),
      body: Center(
        child: Column(
          children: [
            Obx(() => Text("B Clicks: ${c.count}")),
            Text("${c.count}"),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        child: const Icon(Icons.add),
        onPressed: () {
          c.increment();
        },
      ),
    );
  }
}
