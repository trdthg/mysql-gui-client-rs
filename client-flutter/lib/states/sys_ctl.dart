import 'package:flutter/material.dart';
import 'package:get/get.dart';

class SysController extends GetxController {
  var themeMode = ThemeMode.system.obs;
  toDark() => themeMode = ThemeMode.dark as Rx<ThemeMode>;
  toLight() => themeMode = ThemeMode.light as Rx<ThemeMode>;
  toSystem() => themeMode = ThemeMode.system as Rx<ThemeMode>;
}
