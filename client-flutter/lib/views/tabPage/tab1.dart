// ignore_for_file: file_names

import 'dart:typed_data';

import 'package:flutter_rust_bridge_example/generated/bridge_generated.dart';

import 'package:flutter/cupertino.dart';
import '../../main.dart';
import 'off_topic_code.dart';

class Tab1Page extends StatefulWidget {
  const Tab1Page({Key? key}) : super(key: key);

  @override
  State<Tab1Page> createState() => _Tab1PageState();
}

class _Tab1PageState extends State<Tab1Page> {
  Uint8List? exampleImage;
  String? exampleText;

  @override
  void initState() {
    super.initState();
    User(id: 1, name: "name", readed: false);
    api.add(a: 1, b: 2);
    runPeriodically(_callExampleFfiOne);
    _callExampleFfiTwo();
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: buildPageUi(
        exampleImage,
        exampleText,
      ),
    );
  }

  Future<void> _callExampleFfiOne() async {
    final receivedImage = await api.drawMandelbrot(
        imageSize: Size(width: 50, height: 50),
        zoomPoint: examplePoint,
        scale: generateScale(),
        numThreads: 4);
    if (mounted) setState(() => exampleImage = receivedImage);
  }

  Future<void> _callExampleFfiTwo() async {
    final receivedText =
        await api.passingComplexStructs(root: createExampleTree());
    if (mounted) setState(() => exampleText = receivedText);
  }
}
