import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/main.dart';
import 'package:get/get.dart';

class RegisterPage extends StatefulWidget {
  const RegisterPage({Key? key}) : super(key: key);

  @override
  State<RegisterPage> createState() => _RegisterPageState();
}

class _RegisterPageState extends State<RegisterPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
            onPressed: () {
              Get.back();
            },
            icon: const Icon(Icons.arrow_back_ios_new_outlined)),
        title: const Text("注册"),
        backgroundColor: Colors.white10,
        centerTitle: true,
        elevation: 0,
      ),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: SingleChildScrollView(
          child: Column(
            children: [
              const SizedBox(
                height: 10,
              ),
              const RegisterForm(),
              const SizedBox(
                height: 10,
              ),
              Row(
                mainAxisAlignment: MainAxisAlignment.start,
                children: [
                  TextButton.icon(
                      onPressed: () {
                        Get.toNamed(ScreenDef.login);
                      },
                      icon: const Icon(Icons.phone_android_outlined),
                      label: const Text("使用手机号注册")),
                ],
              )
            ],
          ),
        ),
      ),
    );
  }
}

class RegisterForm extends StatefulWidget {
  const RegisterForm({
    Key? key,
  }) : super(key: key);

  @override
  State<RegisterForm> createState() => _RegisterFormState();
}

class _RegisterFormState extends State<RegisterForm> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _password2Controller = TextEditingController();
  final GlobalKey _formKey = GlobalKey<FormState>();

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
    _usernameController.dispose();
    _passwordController.dispose();
    _password2Controller.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Form(
        key: _formKey,
        autovalidateMode: AutovalidateMode.onUserInteraction,
        child: Column(
          children: [
            TextFormField(
              autofocus: true,
              controller: _usernameController,
              decoration: InputDecoration(
                filled: true,
                fillColor: Colors.grey.shade200,
                // labelText: "用户名",
                hintText: "用户名或邮箱",
                prefixIcon: const Icon(Icons.person_outline_outlined),
                border: const OutlineInputBorder(),
                contentPadding: const EdgeInsets.all(0),
              ),
              validator: (v) {
                return v!.trim().isNotEmpty ? null : "用户名不能为空";
              },
            ),
            const SizedBox(
              height: 10,
            ),
            TextFormField(
              controller: _passwordController,
              decoration: InputDecoration(
                // labelText: "密码",
                hintText: "密码",

                filled: true,
                fillColor: Colors.grey.shade200,
                prefixIcon: const Icon(Icons.lock),
                border: const OutlineInputBorder(),
                contentPadding: const EdgeInsets.all(0),
              ),
              obscureText: true,
              //校验密码
              validator: (v) {
                return v!.trim().length > 5 ? null : "密码不能少于 6 位";
              },
            ),
            const SizedBox(
              height: 10,
            ),
            TextFormField(
              controller: _password2Controller,
              decoration: InputDecoration(
                // labelText: "重复密码",
                hintText: "重复密码",
                filled: true,
                fillColor: Colors.grey.shade200,
                prefixIcon: const Icon(Icons.lock),
                border: const OutlineInputBorder(),
                contentPadding: const EdgeInsets.all(0),
              ),
              obscureText: true,
              //校验密码
              validator: (v) {
                return v!.trim() == _passwordController.value.text
                    ? null
                    : "密码不一致";
              },
            ),
            const SizedBox(
              height: 10,
            ),
            Row(
              children: <Widget>[
                Expanded(
                  child: OutlinedButton.icon(
                    icon: const Icon(Icons.app_registration_rounded),
                    label: const Text("注册"),
                    onPressed: () {
                      // 通过_formKey.currentState 获取 FormState 后，
                      // 调用 validate() 方法校验用户名密码是否合法，校验
                      // 通过后再提交数据。
                      if ((_formKey.currentState as FormState).validate()) {
                        //验证通过提交数据
                        // Req.dio().get("path", queryParameters: {});
                      }
                      Get.back();
                    },
                  ),
                ),
              ],
            ),
          ],
        ));
  }
}
