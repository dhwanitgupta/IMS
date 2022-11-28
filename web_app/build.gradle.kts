apply<FlutterPlugin>()


tasks.getByName("flutterBuild").dependsOn(":sdks:dart-clients:build")
