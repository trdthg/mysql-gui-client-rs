import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/utils/logger.dart';
import 'package:get/get.dart';

class SettingPage extends StatefulWidget {
  const SettingPage({Key? key}) : super(key: key);

  @override
  State<SettingPage> createState() => _SettingPageState();
}

class _SettingPageState extends State<SettingPage> {
  var _isDark = false;
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("设置"),
      ),
      body: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SettingGroup(title: "设置", children: const [
              ListTile(
                title: Text("设置"),
                subtitle: Text("一些说明"),
                leading: Icon(Icons.sunny),
              ),
            ]),
            SettingGroup(title: "设置", children: [
              ListTile(
                title: const Text("设置"),
                leading: const Icon(Icons.sunny),
                onTap: () {},
              ),
              ListTile(
                leading: const Icon(Icons.wb_sunny_outlined),
                title: const Text("夜间模式"),
                trailing: Switch(
                    value: _isDark,
                    onChanged: (v) {
                      setState(() {
                        _isDark = v;
                        Log.i("$v");
                        v
                            ? Get.changeThemeMode(ThemeMode.dark)
                            : Get.changeThemeMode(ThemeMode.light);
                      });
                    }),
              )
            ])
          ],
        ),
      ),
    );
  }
}

class SettingGroup extends StatelessWidget {
  SettingGroup({Key? key, required this.title, required this.children})
      : super(key: key);

  List<Widget> children;
  String title;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ...ListTile.divideTiles(color: Colors.grey, context: context, tiles: [
          ListTile(
            dense: true,
            title: Text(title, style: const TextStyle(color: Colors.blue)),
          ),
          ...children
        ]).toList(),
        Container(
          height: 6,
          color: Colors.grey,
        ),
      ],
    );
  }
}
