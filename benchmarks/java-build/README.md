# Java Build Tool Benchmarks

Comparing Java build performance across different tools and configurations.

## Tools Compared

| Tool | Type | Written In | Focus |
|------|------|-----------|-------|
| **Maven** | Traditional | Java | Baseline |
| **Gradle** | Incremental | Groovy/Kotlin | Build cache |
| **Buck2** | Hermetic | Rust | Speed + reproducibility |
| **GraalVM** | Native compile | Java/C++ | Startup time |

---

## Quick Start

```bash
# Run all benchmarks
./run-benchmarks.sh

# Run specific benchmark
./run-benchmarks.sh maven
./run-benchmarks.sh gradle
./run-benchmarks.sh buck2
./run-benchmarks.sh graalvm
```

---

## Benchmark Categories

### 1. Clean Build (Cold Start)

Build from scratch, no caches.

| Metric | Description |
|--------|-------------|
| Total time | Wall clock time |
| Memory used | Peak heap |
| CPU utilization | Average % |

### 2. Incremental Build (Hot Start)

Single file change, caches warm.

| Metric | Description |
|--------|-------------|
| Time to rebuild | Wall clock |
| Files recompiled | Count |

### 3. Dependency Resolution

Time to resolve and download dependencies.

### 4. Startup Time (GraalVM only)

Time from `java -jar` to first response.

---

## Results

See [results/](./results/) for benchmark outputs.

| Tool | Clean Build | Incremental | Deps Resolution |
|------|-------------|-------------|-----------------|
| Maven | TBD | TBD | TBD |
| Gradle | TBD | TBD | TBD |
| Gradle (cached) | TBD | TBD | TBD |
| Buck2 | TBD | TBD | TBD |
| GraalVM | TBD | N/A | N/A |

---

## Setup Requirements

### Maven

```bash
# Already installed via SDKMAN or apt
mvn --version
```

### Gradle

```bash
sdk install gradle
gradle --version
```

### Buck2

```bash
# Install from pre-built binary
curl -LO https://github.com/facebook/buck2/releases/latest/download/buck2-x86_64-unknown-linux-gnu.zst
zstd -d buck2-x86_64-unknown-linux-gnu.zst -o buck2
chmod +x buck2
sudo mv buck2 /usr/local/bin/
```

### GraalVM

```bash
sdk install java 21.0.1-graalce
gu install native-image
```

---

## Running

```bash
cd benchmarks/java-build
./run-benchmarks.sh all
```

Results will be written to `results/benchmark-{timestamp}.json`.
