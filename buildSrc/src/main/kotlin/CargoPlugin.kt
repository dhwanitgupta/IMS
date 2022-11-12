import org.gradle.api.Project
import org.gradle.api.plugins.BasePlugin
import org.gradle.api.tasks.Delete
import org.gradle.api.tasks.Exec
import org.gradle.internal.os.OperatingSystem
import org.gradle.kotlin.dsl.register

class CargoPlugin : BasePlugin() {
    override fun apply(project: Project) {
        super.apply(project)
        with(project) {
            with(tasks) {
                named("clean", Delete::class.java) {
                    delete.add(file("target"))
                    if (file("Cargo.toml.hbs").exists()) {
                        delete.add(file("Cargo.toml"))
                    }
                }

                val cargoBuild = register<Exec>("cargoBuild") {
                    group = "build"
                    executable("cargo")
                    args("build")
                }

                val cargoTest = register<Exec>("cargoTest") {
                    group = "verification"
                    executable("cargo")
                    args("test")
                    dependsOn(cargoBuild)
                }

                val cargoClippy = register<Exec>("cargoClippy") {
                    group = "verification"
                    executable("cargo")
                    args("clippy")
                    if (!providers.environmentVariable("CI").isPresent) {
                        args("--fix", "--allow-dirty", "--allow-staged", "--allow-no-vcs")
                    }
                    args("--", "-D", "warnings")
                    dependsOn(cargoBuild)
                }

                val cargoFormat = register<Exec>("cargoFormat") {
                    group = "verification"
                    executable("cargo")
                    args("fmt", "--all")
                    if (providers.environmentVariable("CI").isPresent) {
                        args("--", "--check")
                    }
                    dependsOn(cargoBuild)
                }

                register<Exec>("cargoUpdate") {
                    group = "build"
                    executable("cargo")
                    args("update")
                }

                named("check") {
                    dependsOn(cargoTest)
                    dependsOn(cargoFormat)
                    dependsOn(cargoClippy)
                }
            }
        }
    }
}