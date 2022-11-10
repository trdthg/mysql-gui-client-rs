import 'dart:ffi' hide Size;
import 'dart:io';

import 'package:flutter/material.dart' hide Size;
import 'package:flutter_rust_bridge_example/generated/bridge_generated.dart'
    hide Size;
import 'package:flutter_rust_bridge_example/views/home.dart';
import 'package:flutter_rust_bridge_example/views/auth/login.dart';
import 'package:get/get.dart';

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

const base = 'flutter_rust_bridge_example';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
late final api = FlutterRustBridgeExampleImpl(dylib);

void main() => runApp(GetMaterialApp(
      debugShowCheckedModeBanner: false,
      initialRoute: ScreenDef.init,
      themeMode: ThemeMode.light,
      theme: ThemeConfig.lightTheme,
      darkTheme: ThemeConfig.darkTheme,
      getPages: [
        GetPage(name: ScreenDef.init, page: () => const Login()),
        GetPage(name: ScreenDef.login, page: () => const Login()),
        GetPage(name: ScreenDef.home, page: () => const Home()),
      ],
    ));

class ScreenDef {
  static String init = "/";
  static String login = "/login";
  static String home = "/home";
}

class ThemeConfig {
  static Color lightPrimary = Colors.white;
  static Color darkPrimary = const Color(0xff1f1f1f);
  static Color lightAccent = const Color(0xff2ca8e2);
  static Color darkAccent = const Color(0xff2ca8e2);
  static Color lightBG = Colors.white;
  static Color darkBG = const Color(0xff121212);
  static PageTransitionsTheme pageTransitionsTheme =
      const PageTransitionsTheme(builders: {
    TargetPlatform.android: CupertinoPageTransitionsBuilder(),
    TargetPlatform.iOS: CupertinoPageTransitionsBuilder(),
    TargetPlatform.linux: ZoomPageTransitionsBuilder(),
  });

  static ThemeData lightTheme = ThemeData(
    backgroundColor: lightBG,
    primaryColor: lightPrimary,
    colorScheme: const ColorScheme.light(),
    scaffoldBackgroundColor: lightBG,
    pageTransitionsTheme: pageTransitionsTheme,
    appBarTheme: AppBarTheme(
      color: lightPrimary,
      elevation: 0.0,
      titleTextStyle: const TextStyle(
        color: Colors.black,
        fontSize: 20,
        fontWeight: FontWeight.w800,
      ),
      iconTheme: const IconThemeData(
        color: Colors.black,
      ),
    ),
  );

  static ThemeData darkTheme = ThemeData(
    brightness: Brightness.dark,
    backgroundColor: darkBG,
    primaryColor: darkPrimary,
    colorScheme: const ColorScheme.dark(),
    scaffoldBackgroundColor: darkBG,
    pageTransitionsTheme: pageTransitionsTheme,
    appBarTheme: AppBarTheme(
      color: darkPrimary,
      elevation: 0.0,
      titleTextStyle: const TextStyle(
        color: Colors.white,
        fontSize: 20,
        fontWeight: FontWeight.w800,
      ),
      iconTheme: const IconThemeData(
        color: Colors.white,
      ),
    ),
  );
}
