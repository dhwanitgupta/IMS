apply<CargoPlugin>()

tasks.getByName("cargoBuild").dependsOn(":sdks:rust-servers:build")
