import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/generated/bridge_generated.dart';
import 'package:flutter_rust_bridge_example/views/tabPage/chat/detail.dart';
import 'package:get/get.dart';

class Tab3Page extends StatefulWidget {
  const Tab3Page({Key? key}) : super(key: key);

  @override
  State<Tab3Page> createState() => _Tab3PageState();
}

class _Tab3PageState extends State<Tab3Page> {
  List<User> data = [];

  late ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    data.addAll(Iterable<int>.generate(100)
        .map((e) => User(id: e, name: "User $e", readed: false)));
  }

  @override
  void dispose() {
    super.dispose();
    _scrollController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
        child: Scrollbar(
            child: RefreshIndicator(
      onRefresh: () async {
        Future.delayed(const Duration(seconds: 3), () {
          print("refresh");
        });
      },
      child: ListView(
        shrinkWrap: true,
        controller: _scrollController,
        children: data.map<Widget>((e) {
          var card = UserCard(e);
          return GestureDetector(
            onTap: () {
              Get.to(const ChatDetailPage());
            },
            onLongPress: () {
              showDialog(
                  context: context,
                  builder: (builder) {
                    return AlertDialog(
                      title: const Text("WARN"),
                      content: ListView(
                          children: Iterable<int>.generate(100)
                              .map((e) => const Text("aa"))
                              .toList()),
                      actions: [
                        TextButton(
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                            child: const Text("按钮 1")),
                        TextButton(
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                            child: const Text("按钮 2")),
                        TextButton(
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                            child: const Text("按钮 3")),
                      ],
                    );
                  });
            },
            child: card,
          );
        }).toList(),
      ),
    )));
  }

  Card UserCard(User e) {
    String url =
        e.avatorUrl ?? "https://randomuser.me/api/portraits/men/${e.id}.jpg";
    return Card(
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 16),
        child: Row(
          children: [
            CircleAvatar(
              backgroundImage: NetworkImage(url),
              maxRadius: 30,
            ),
            const SizedBox(width: 20),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(e.name),
                  const SizedBox(
                    height: 12,
                  ),
                  Text(
                    e.latestMsg ?? "无最新消息",
                    style: TextStyle(
                        fontSize: 10,
                        color: Colors.grey.shade600,
                        fontWeight:
                            e.readed ? FontWeight.bold : FontWeight.normal),
                  ),
                ],
              ),
            ),
            Column(
              children: [
                Text(
                  "18:34",
                  style: TextStyle(
                      fontSize: 12,
                      fontWeight:
                          e.readed ? FontWeight.bold : FontWeight.normal),
                )
              ],
            )
          ],
        ),
      ),
    );
  }
}
