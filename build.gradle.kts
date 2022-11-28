plugins {
    id("base")
}

System.setProperty("VERSION", version as String)

val startServer = tasks.create<Exec>("startServer") {
    executable(project(":app").projectDir.resolve("target").resolve("debug").resolve("core"))
    environment("RUST_LOG", "debug")
}

val startApp = tasks.create<Exec>("startApp") {
    workingDir(project(":web_app").projectDir)
    executable("flutter")
    args("run")
}
