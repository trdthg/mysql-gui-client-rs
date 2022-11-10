import 'package:flutter/material.dart';
import 'package:get/get.dart';
import '../states/count_controller.dart';

class PageAPage extends StatelessWidget {
  const PageAPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final CountController c = Get.put(CountController());
    return Scaffold(
      appBar: AppBar(
        title: const Text("第二页"),
      ),
      body: Center(
        child: Column(
          children: [
            Obx(() => Text("A Clicks: ${c.count}")),
            Text("${c.count}"),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
          child: const Icon(Icons.add),
          onPressed: () {
            c.increment();
          }),
    );
  }
}
