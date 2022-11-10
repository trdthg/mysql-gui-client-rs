import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_example/utils/logger.dart';

class Req {
  static late Dio _dio;

  Req() {
    Dio dio = Dio();
    dio.options = BaseOptions(
      baseUrl: "",
      connectTimeout: 5000,
      sendTimeout: 5000,
      receiveTimeout: 5000,
      headers: {
        "token": "token:/aeifuahwod209uq",
      },
      contentType: ContentType.json.value,
      responseType: ResponseType.json,
    );
    dio.interceptors.add(InterceptorsWrapper(
      onRequest: (options, handler) {
        Log.i(options.uri);
      },
      onResponse: (options, handler) {
        Log.i(options.statusCode.toString() + options.toString());
      },
      onError: (e, handler) {
        Log.e(e.error);
        //
      },
    ));
    _dio = dio;
  }

  static Dio dio() {
    return _dio;
  }
}
