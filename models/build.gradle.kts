buildscript {
    val smithyVersion: String by project
    dependencies {
        classpath("software.amazon.smithy:smithy-openapi:$smithyVersion")
    }
}

repositories {
    mavenCentral()
}

val smithyVersion: String by project
dependencies {
    implementation("software.amazon.smithy:smithy-model:$smithyVersion")
    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
}

plugins {
    id("software.amazon.smithy")
}

sourceSets {
    getByName("main") {
        java.srcDir("model")
    }
}