#!/usr/bin/env bash
# Java Build Tools Benchmark Runner
# Compares Maven, Gradle, Buck2, and GraalVM native-image

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
SAMPLE_PROJECT="$SCRIPT_DIR/sample-project"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[BENCH]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Time a command and return milliseconds
time_cmd() {
    local start=$(date +%s%3N)
    "$@" > /dev/null 2>&1
    local end=$(date +%s%3N)
    echo $((end - start))
}

# Ensure results directory exists
mkdir -p "$RESULTS_DIR"

# =============================================================================
# BENCHMARK: Maven
# =============================================================================
benchmark_maven() {
    log "Benchmarking Maven..."
    
    cd "$SAMPLE_PROJECT/maven"
    
    # Clean build
    rm -rf target
    local clean_time=$(time_cmd mvn clean compile -q)
    log "Maven clean build: ${clean_time}ms"
    
    # Incremental build (touch a file)
    touch src/main/java/com/example/App.java
    local incr_time=$(time_cmd mvn compile -q)
    log "Maven incremental: ${incr_time}ms"
    
    # Dependency resolution
    rm -rf ~/.m2/repository/com/google
    local deps_time=$(time_cmd mvn dependency:resolve -q)
    log "Maven deps: ${deps_time}ms"
    
    echo "{\"tool\":\"maven\",\"clean\":$clean_time,\"incremental\":$incr_time,\"deps\":$deps_time}"
}

# =============================================================================
# BENCHMARK: Gradle
# =============================================================================
benchmark_gradle() {
    log "Benchmarking Gradle..."
    
    cd "$SAMPLE_PROJECT/gradle"
    
    # Clean build (no cache)
    rm -rf build ~/.gradle/caches/build-cache-*
    local clean_time=$(time_cmd ./gradlew clean compileJava --no-build-cache -q)
    log "Gradle clean (no cache): ${clean_time}ms"
    
    # Clean build (with cache)
    rm -rf build
    local cached_time=$(time_cmd ./gradlew clean compileJava --build-cache -q)
    log "Gradle clean (cached): ${cached_time}ms"
    
    # Incremental build
    touch src/main/java/com/example/App.java
    local incr_time=$(time_cmd ./gradlew compileJava -q)
    log "Gradle incremental: ${incr_time}ms"
    
    # Dependency resolution
    rm -rf ~/.gradle/caches/modules-*
    local deps_time=$(time_cmd ./gradlew dependencies -q)
    log "Gradle deps: ${deps_time}ms"
    
    echo "{\"tool\":\"gradle\",\"clean\":$clean_time,\"cached\":$cached_time,\"incremental\":$incr_time,\"deps\":$deps_time}"
}

# =============================================================================
# BENCHMARK: Buck2
# =============================================================================
benchmark_buck2() {
    log "Benchmarking Buck2..."
    
    if ! command -v buck2 &> /dev/null; then
        warn "Buck2 not installed, skipping..."
        echo "{\"tool\":\"buck2\",\"error\":\"not installed\"}"
        return
    fi
    
    cd "$SAMPLE_PROJECT/buck2"
    
    # Clean build
    buck2 clean
    local clean_time=$(time_cmd buck2 build //...)
    log "Buck2 clean build: ${clean_time}ms"
    
    # Incremental build
    touch src/com/example/App.java
    local incr_time=$(time_cmd buck2 build //...)
    log "Buck2 incremental: ${incr_time}ms"
    
    echo "{\"tool\":\"buck2\",\"clean\":$clean_time,\"incremental\":$incr_time}"
}

# =============================================================================
# BENCHMARK: GraalVM Native Image
# =============================================================================
benchmark_graalvm() {
    log "Benchmarking GraalVM native-image..."
    
    if ! command -v native-image &> /dev/null; then
        warn "GraalVM native-image not installed, skipping..."
        echo "{\"tool\":\"graalvm\",\"error\":\"not installed\"}"
        return
    fi
    
    cd "$SAMPLE_PROJECT/maven"
    
    # Build JAR first
    mvn clean package -q -DskipTests
    
    # Native image compilation
    local native_time=$(time_cmd native-image -jar target/app.jar -o target/app-native)
    log "GraalVM compile: ${native_time}ms"
    
    # Startup time comparison
    log "Measuring startup times..."
    
    # JVM startup
    local jvm_start=$(time_cmd java -jar target/app.jar --version)
    log "JVM startup: ${jvm_start}ms"
    
    # Native startup
    local native_start=$(time_cmd ./target/app-native --version)
    log "Native startup: ${native_start}ms"
    
    echo "{\"tool\":\"graalvm\",\"compile_time\":$native_time,\"jvm_startup\":$jvm_start,\"native_startup\":$native_start}"
}

# =============================================================================
# MAIN
# =============================================================================
main() {
    local tool="${1:-all}"
    local results=()
    
    log "Java Build Benchmark Runner"
    log "Timestamp: $TIMESTAMP"
    
    case "$tool" in
        maven)    results+=("$(benchmark_maven)") ;;
        gradle)   results+=("$(benchmark_gradle)") ;;
        buck2)    results+=("$(benchmark_buck2)") ;;
        graalvm)  results+=("$(benchmark_graalvm)") ;;
        all)
            results+=("$(benchmark_maven)")
            results+=("$(benchmark_gradle)")
            results+=("$(benchmark_buck2)")
            results+=("$(benchmark_graalvm)")
            ;;
        *)
            error "Unknown tool: $tool. Use: maven, gradle, buck2, graalvm, all"
            ;;
    esac
    
    # Write results
    local output_file="$RESULTS_DIR/benchmark-$TIMESTAMP.json"
    echo "[" > "$output_file"
    local first=true
    for result in "${results[@]}"; do
        if [ "$first" = true ]; then
            first=false
        else
            echo "," >> "$output_file"
        fi
        echo "  $result" >> "$output_file"
    done
    echo "]" >> "$output_file"
    
    log "Results written to: $output_file"
    cat "$output_file"
}

main "$@"
