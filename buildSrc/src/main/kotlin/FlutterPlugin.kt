import org.gradle.api.Project
import org.gradle.api.plugins.BasePlugin
import org.gradle.api.tasks.Delete
import org.gradle.api.tasks.Exec
import org.gradle.kotlin.dsl.register

class FlutterPlugin : BasePlugin() {
    override fun apply(project: Project) {
        super.apply(project)
        with(project) {
            with(tasks) {
                named("clean", Delete::class.java) {
                    delete.add(file("build"))
                }

                var flutterBuild = register<Exec>("flutterBuild") {
                    group = "build"
                    executable("flutter")
                    args("build", "web")
                }

                val dartFormat = register<Exec>("dartFormat") {
                    group = "verification"
                    executable("dart")
                    args("format", ".")
                    dependsOn(flutterBuild)
                }

                named("check") {
                    dependsOn(dartFormat)
                }
            }
        }
    }
}