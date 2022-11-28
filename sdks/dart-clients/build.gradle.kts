plugins {
    id("org.openapi.generator").version("6.2.1")
}

buildscript {
    repositories {
        mavenLocal()
        maven(url = "https://repo1.maven.org/maven2")
        maven(url = "https://plugins.gradle.org/m2/")
        maven(url = "https://oss.sonatype.org/content/repositories/releases/")
        maven(url = "https://oss.sonatype.org/content/repositories/snapshots/")
    }
    dependencies {
        classpath("org.openapitools:openapi-generator-gradle-plugin:6.2.1")
    }
}

apply(plugin = "org.openapi.generator")

openApiValidate {
    inputSpec.set(projectDir.resolve("ImsCore.openapi.json").toPath().toString())
    recommend.set(true)
}
openApiGenerate {
    generatorName.set("dart")
    inputSpec.set(projectDir.resolve("ImsCore.openapi.json").toPath().toString())
    outputDir.set("output/dart")
    logToStderr.set(true)
    verbose.set(true)
}


val build = tasks.create<Exec>("build") {
    workingDir(project(":sdks:dart-clients").projectDir)
    executable("openapi-generator")
    args("generate", "-i" ,
        "${project(":models").projectDir}/build/smithyprojections/models/openapi-deploy/openapi/ImsCore.openapi.json",
        "-g", "dart", "-o", "build", "-p=pubName=ims_dart_client", "-p=pubVersion=1.0.0", "-p=pubDescription=IMS Dart Client")
    dependsOn(":models:build")
}

val clean = tasks.create<Exec>("clean") {
    workingDir(project(":sdks:dart-clients").projectDir)
    executable("rm")
    args("-rf", "build")
}
