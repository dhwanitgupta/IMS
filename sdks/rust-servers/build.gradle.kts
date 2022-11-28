buildscript {
    repositories {
        mavenLocal()
    }

    dependencies {
        classpath("software.amazon.smithy.rust.codegen.server.smithy:codegen-server:0.1.0")
    }
}

repositories {
    mavenCentral()
}

plugins {
    id("software.amazon.smithy")
}

val smithyVersion: String by project
dependencies {
    implementation(project(":models"))
    implementation("software.amazon.smithy:smithy-model:$smithyVersion")
}

System.setProperty("RUST_RUNTIME_VERSION", project.property("rust.smithyRuntimeVersion").toString())

smithy {
    outputDirectory = buildDir.resolve("output")
}

