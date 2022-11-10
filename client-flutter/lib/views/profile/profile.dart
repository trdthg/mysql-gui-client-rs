import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/utils/iter_ext.dart';
import 'package:flutter_rust_bridge_example/utils/tuple.dart';
import 'package:get/get.dart';

import '../../generated/bridge_generated.dart';
import 'setting/setting.dart';

class ProfilePage extends StatefulWidget {
  const ProfilePage({
    Key? key,
  }) : super(key: key);

  @override
  State<ProfilePage> createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage>
    with SingleTickerProviderStateMixin {
  late User me;

  List<Tuple3<IconData, String, Widget>> sections = [
    Tuple3(Icons.group_outlined, "新群组 New Group", const SettingPage()),
    Tuple3(Icons.person_outline, "联系人 Friends", const SettingPage()),
    Tuple3(Icons.phone_outlined, "电话 Phone Call", const SettingPage()),
    Tuple3(Icons.videocam_outlined, "视频 Video Call", const SettingPage()),
    Tuple3(Icons.bookmark_border, "已保存 Saved Messages", const SettingPage()),
    Tuple3(Icons.settings_outlined, "设置 Setting", const SettingPage()),
  ];

  @override
  void initState() {
    me = User(id: 1, name: "小垃圾", readed: true);

    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  final _icon_color = Get.textTheme.bodyMedium!.color;
  final _icon_size = 24.0;
  final _title_color = Get.textTheme.bodyMedium!.color;
  final _title_size = 14.0;

  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Profile
          Container(
            width: double.infinity,
            color: Colors.black12,
            padding: const EdgeInsets.only(top: 100, left: 20, bottom: 20),
            child: Row(children: [
              CircleAvatar(
                backgroundImage: NetworkImage(me.avatorUrl ??
                    "https://randomuser.me/api/portraits/men/99.jpg"),
                radius: 30,
              ),
              const SizedBox(width: 10),
              Column(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    "Trdthg",
                    style: TextStyle(
                        fontSize:
                            Theme.of(context).textTheme.titleLarge!.fontSize),
                  ),
                  const SizedBox(
                    height: 6,
                  ),
                  Text(
                    "加入时长：114514 天",
                    style: TextStyle(
                        // color: Colors.black54,
                        fontSize: Get.textTheme.subtitle2!.fontSize),
                  ),
                ],
              )
            ]),
          ),
          // sections
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                children: sections
                    .sublist(0, sections.length - 1)
                    .mapIndexed((e, index) {
                  return DecoratedBox(
                      decoration: BoxDecoration(
                        border: Border(
                          bottom: Divider.createBorderSide(
                            context,
                          ),
                        ),
                      ),
                      child: ListTile(
                        dense: true,
                        contentPadding: const EdgeInsets.only(left: 20),
                        leading: Icon(
                          e.field0,
                          size: _icon_size,
                          // color: _icon_color,
                        ),
                        minLeadingWidth: 12,
                        title: Text(
                          e.field1,
                          style: TextStyle(
                            fontSize: _title_size,
                            fontWeight: FontWeight.w400,
                            // color: _title_color
                          ),
                        ),
                        onTap: () {
                          Navigator.push(context,
                              MaterialPageRoute(builder: (context) {
                            return e.field2;
                          }));
                        },
                      ));
                }).toList(),
              ),
            ),
          ),
          const Divider(),
          Row(
            children: [
              Expanded(
                child: InkWell(
                  onTap: () {
                    Navigator.push(context,
                        MaterialPageRoute(builder: (context) {
                      return sections.last.field2;
                    }));
                  },
                  child: Row(
                    children: [
                      const SizedBox(
                        width: 20,
                      ),
                      Icon(
                        sections.last.field0,
                        size: _icon_size,
                        // color: _icon_color,
                      ),
                      const SizedBox(
                        width: 12,
                      ),
                      Text(
                        sections.last.field1,
                        style: TextStyle(
                          fontSize: _title_size,
                          fontWeight: FontWeight.normal,
                          // color: _title_color
                        ),
                      )
                    ],
                  ),
                ),
              ),
              IconButton(
                  onPressed: () {
                    if (Get.isDarkMode) {
                      Get.changeThemeMode(ThemeMode.light);
                    } else {
                      Get.changeThemeMode(ThemeMode.dark);
                    }
                  },
                  icon: const Icon(Icons.mode_night_outlined)),
              const SizedBox(
                width: 12,
              ),
            ],
          ),
        ],
      ),
    );
  }
}
