plugins {
    id("base")
}

val startServer = tasks.create<Exec>("startServer") {
    executable(project(":app").projectDir.resolve("target").resolve("debug").resolve("core"))
    environment("RUST_LOG", "debug")
}