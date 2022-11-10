import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class ChatDetailPage extends StatefulWidget {
  const ChatDetailPage({Key? key}) : super(key: key);

  @override
  State<ChatDetailPage> createState() => _ChatDetailPageState();
}

class ChatMessage {
  String messageContent;
    String messageType;
    ChatMessage({
     required this.messageContent,required this.messageType
    });
}

class _ChatDetailPageState extends State<ChatDetailPage> {
  List<ChatMessage> messages = [];

  TextEditingController messageController = TextEditingController();

  final ScrollController _messageListController = ScrollController();

  @override
  void initState() {
    super.initState();
    messages.addAll([
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(
          messageContent: "How have you been?", messageType: "receiver"),
      ChatMessage(
          messageContent: "Hey Kriss, I am doing fine dude. wbu?",
          messageType: "sender"),
      ChatMessage(messageContent: "ehhhh, doing OK.", messageType: "receiver"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(
          messageContent: "Is there any thing wrong?", messageType: "sender"),
      ChatMessage(messageContent: "Hello, Will", messageType: "receiver"),
    ]);

    SchedulerBinding.instance.addPostFrameCallback((_) {
      _messageListController
          .jumpTo(_messageListController.position.maxScrollExtent);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      resizeToAvoidBottomInset: true,
      appBar: AppBar(
        // title: const Text("于 xxx 的聊天"),
        elevation: 0,
        automaticallyImplyLeading: false,
        backgroundColor: Colors.white,
        flexibleSpace: SafeArea(
          child: Container(
            padding: const EdgeInsets.only(right: 16),
            child: Row(
              children: [
                IconButton(
                  onPressed: () {
                    Navigator.pop(context);
                  },
                  icon: const Icon(Icons.arrow_back),
                ),
                const SizedBox(
                  width: 2,
                ),
                const CircleAvatar(
                  backgroundImage: NetworkImage(
                      "https://randomuser.me/api/portraits/men/5.jpg"),
                  maxRadius: 20,
                ),
                const SizedBox(
                  width: 12,
                ),
                Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const Text(
                          "Kriss Benwat",
                          style:
                          TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                        ),
                        const SizedBox(
                          height: 6,
                        ),
                        Text(
                          "Online",
                          style:
                          TextStyle(color: Colors.grey.shade600, fontSize: 13),
                        ),
                      ],
                    )),
                const Icon(
                  Icons.settings,
                  color: Colors.black54,
                ),
              ],
            ),
          ),
        ),
      ),
      body: Column(children: [
        Expanded(
          child: ListView.builder(
              itemCount: messages.length,
              shrinkWrap: true,
              controller: _messageListController,
              padding: const EdgeInsets.only(top: 10, bottom: 10),
              itemBuilder: (context, index) {
                return Container(
                    padding: const EdgeInsets.only(
                        left: 14, right: 14, top: 10, bottom: 10),
                    child: Align(
                      alignment: (messages[index].messageType == "receiver"
                          ? Alignment.topLeft
                          : Alignment.topRight),
                      child: Container(
                        padding: const EdgeInsets.all(16),
                        decoration: BoxDecoration(
                            borderRadius: BorderRadius.circular(20),
                            color: (messages[index].messageType == "receiver"
                                ? Colors.grey.shade200
                                : Colors.blue.shade200)),
                        child: Text(
                          messages[index].messageContent,
                          style: const TextStyle(fontSize: 15),
                        ),
                      ),
                    ));
              }),
        ),
        Container(
          padding: const EdgeInsets.only(left: 10, bottom: 10, top: 10),
          width: double.infinity,
          height: 60,
          color: Colors.white,
          child: Row(
            children: [
              GestureDetector(
                onTap: () {
                  onTapMore();
                },
                child: Container(
                  height: 30,
                  width: 30,
                  decoration: BoxDecoration(
                    color: Colors.lightBlue,
                    borderRadius: BorderRadius.circular(30),
                  ),
                  child: const Icon(
                    Icons.add,
                    color: Colors.white,
                  ),
                ),
              ),
              const SizedBox(
                width: 15,
              ),
              Expanded(
                  child: TextField(
                    controller: messageController,
                    onTap: () {
                      onTapInput();
                    },
                    decoration: const InputDecoration(
                        hintText: "White something...",
                        hintStyle: TextStyle(color: Colors.black54),
                        border: InputBorder.none),
                  )),
              const SizedBox(
                width: 15,
              ),
              FloatingActionButton(
                tooltip: "发送",
                onPressed: () {
                  onSend();
                },
                child: const Icon(
                  Icons.send,
                  color: Colors.white,
                  size: 18,
                ),
                backgroundColor: Colors.blue,
                elevation: 0,
              )
            ],
          ),
        )
      ]),
    );
  }

  void jumpToBottom() {
    SchedulerBinding.instance.addPostFrameCallback((_) {
      final position = _messageListController.position.maxScrollExtent;
      _messageListController.jumpTo(position);
    });
  }

  void onSend() {
    setState(() {
      if (messageController.text.isNotEmpty) {
        messages.add(ChatMessage(
            messageContent: messageController.text, messageType: "sender"));
        messageController.clear();
      }
    });
    jumpToBottom();
  }

  void onTapInput() {
    jumpToBottom();
  }

  void onTapMore() {}
}
