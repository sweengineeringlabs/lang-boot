plugins {
    id("java")
    id("application")
}

group = "com.example"
version = "1.0.0"

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

repositories {
    mavenCentral()
}

dependencies {
    // Same dependencies as Maven for fair comparison
    implementation("com.google.guava:guava:32.1.3-jre")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.16.0")
    implementation("org.slf4j:slf4j-api:2.0.9")
}

application {
    mainClass.set("com.example.App")
}
