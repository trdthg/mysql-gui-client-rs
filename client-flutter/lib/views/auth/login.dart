import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/main.dart';
import 'package:flutter_rust_bridge_example/views/auth/phone.dart';
import 'package:flutter_rust_bridge_example/views/auth/register.dart';
import 'package:get/get.dart';

import 'find_passwd.dart';

class Login extends StatefulWidget {
  const Login({Key? key}) : super(key: key);

  @override
  State<Login> createState() => _LoginState();
}

class _LoginState extends State<Login> with SingleTickerProviderStateMixin {
  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
            onPressed: () {
              Get.back();
            },
            icon: const Icon(Icons.arrow_back_ios_new_outlined)),
        title: const Text("登录"),
        backgroundColor: Colors.white10,
        centerTitle: true,
        elevation: 0,
        actions: [
          TextButton(
              onPressed: () {
                Get.to(const RegisterPage());
              },
              child: const Text("现在注册")),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: SingleChildScrollView(
          child: Padding(
            padding: const EdgeInsets.all(16.0),
            child: Column(
              children: [
                // const Padding(
                //   padding: EdgeInsets.all(36.0),
                //   child: Image(
                //       image: NetworkImage(
                //     "https://images.squarespace-cdn.com/content/v1/617f6f16b877c06711e87373/c3f23723-37f4-44d7-9c5d-6e2a53064ae7/Asset+10.png?format=1500w",
                //   )),
                // ),
                const SizedBox(
                  height: 10,
                ),
                const LoginForm(),
                const SizedBox(
                  height: 10,
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  children: [
                    TextButton.icon(
                        onPressed: () {
                          Get.to(const PhoneLogin());
                        },
                        icon: const Icon(Icons.phone_android_outlined),
                        label: const Text("使用手机验证码登录")),
                  ],
                )
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class LoginForm extends StatefulWidget {
  const LoginForm({
    Key? key,
  }) : super(key: key);

  @override
  State<LoginForm> createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final GlobalKey _formKey = GlobalKey<FormState>();

  @override
  void dispose() {
    super.dispose();
    _usernameController.dispose();
    _passwordController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Form(
        key: _formKey,
        autovalidateMode: AutovalidateMode.onUserInteraction,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          children: [
            // 用户名
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
                  contentPadding: const EdgeInsets.all(0)),
              validator: (v) {
                return v!.trim().isNotEmpty ? null : "用户名不能为空";
              },
            ),
            const SizedBox(
              height: 10,
            ),
            // 密码
            TextFormField(
              controller: _passwordController,
              decoration: InputDecoration(
                filled: true,
                fillColor: Colors.grey.shade200,
                // labelText: "密码",
                hintText: "密码",
                border: const OutlineInputBorder(),
                contentPadding: const EdgeInsets.all(0),
                prefixIcon: const Icon(Icons.lock),
              ),
              obscureText: true,
              //校验密码
              validator: (v) {
                return v!.trim().length > 5 ? null : "密码不能少于 6 位";
              },
            ),
            // 忘记密码
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: TextButton.icon(
                    icon: const Icon(Icons.info_outlined),
                    label: const Text("忘记密码？"),
                    onPressed: () {
                      Get.to(const FindPassword());
                    },
                  ),
                ),
              ],
            ),
            // 登录
            Row(
              children: <Widget>[
                Expanded(
                  child: OutlinedButton.icon(
                    icon: const Icon(Icons.login_outlined),
                    label: const Padding(
                      padding: EdgeInsets.all(16.0),
                      child: Text("登录"),
                    ),
                    onPressed: () {
                      // 通过_formKey.currentState 获取 FormState 后，
                      // 调用 validate() 方法校验用户名密码是否合法，校验
                      // 通过后再提交数据。
                      if ((_formKey.currentState as FormState).validate()) {
                        //验证通过提交数据
                        // Req.dio().get("path", queryParameters: {});
                      }
                      Get.offAndToNamed(ScreenDef.home);
                    },
                  ),
                ),
              ],
            ),
          ],
        ));
  }
}
