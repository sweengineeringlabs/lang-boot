# Testing Conventions by Language

Idiomatic test organization patterns for each language in the Lang-Boot ecosystem.

---

## 🦀 Rust

### Unit Tests (Co-located, Private Access)

```rust
// src/handler.rs
pub fn public_func() -> i32 { private_func() }
fn private_func() -> i32 { 42 }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_private() {
        assert_eq!(private_func(), 42);  // ✅ Private access
    }
}
```

### Separate Test File (Still Co-located)

```
src/
├── handler.rs
└── handler_tests.rs   ← Separate but same directory
```

```rust
// handler.rs
#[cfg(test)]
#[path = "handler_tests.rs"]
mod tests;
```

### Integration Tests (Public API Only)

```
tests/
└── integration.rs     ← Separate crate, no private access
```

| Location | Type | Private Access | Convention |
|----------|------|----------------|------------|
| `#[cfg(test)]` in source | Unit | ✅ Yes | **Idiomatic** |
| `tests/*.rs` | Integration | ❌ No | External API testing |

---

## 🦫 Go

### Co-located Tests (Always)

```
mypackage/
├── handler.go         ← package mypackage
└── handler_test.go    ← Test file (same directory)
```

### White-box (Same Package)

```go
// handler_test.go
package mypackage  // Same package = private access

func TestPrivate(t *testing.T) {
    result := privateFunc()  // ✅ Can test private
}
```

### Black-box (Test Package)

```go
// handler_test.go
package mypackage_test  // _test suffix = public API only

import "mypackage"

func TestPublic(t *testing.T) {
    result := mypackage.PublicFunc()  // Public API only
}
```

| Package Declaration | Private Access | Convention |
|---------------------|----------------|------------|
| `package foo` | ✅ Yes | White-box unit tests |
| `package foo_test` | ❌ No | Black-box API tests |

**Go Convention**: Tests are ALWAYS co-located with `_test.go` suffix.

---

## ☕ Java

> **📝 Convention Note**: The traditional Java convention uses a separate `src/test/java/` tree. 
> **Lang-Boot adopts co-located tests** because we believe domain aspects must co-locate — 
> tests are part of the domain knowledge and should live alongside the code they verify.
> This improves discoverability, refactoring, and code review.

### Co-located Tests (Lang-Boot Convention)

```
src/main/java/com/example/
├── UserService.java
├── UserServiceTest.java      ← Unit tests (co-located)
├── UserServiceIT.java        ← Integration tests (co-located)
└── UserServiceE2E.java       ← End-to-end tests (co-located)
```

### Test Type Naming Conventions

| Suffix | Type | Runs With | Purpose |
|--------|------|-----------|---------|
| `*Test.java` | Unit | `mvn test` | Fast, isolated tests |
| `*IT.java` | Integration | `mvn verify` | Tests with dependencies |
| `*E2E.java` | End-to-end | `mvn verify -Pe2e` | Full system tests |

### Maven Configuration (pom.xml)

```xml
<build>
    <!-- Tests live in main source directory -->
    <testSourceDirectory>${project.basedir}/src/main/java</testSourceDirectory>
    
    <plugins>
        <!-- Exclude test files from production JAR -->
        <plugin>
            <groupId>org.apache.maven.plugins</groupId>
            <artifactId>maven-jar-plugin</artifactId>
            <configuration>
                <excludes>
                    <exclude>**/*Test.class</exclude>
                    <exclude>**/*Test.java</exclude>
                    <exclude>**/*IT.class</exclude>
                    <exclude>**/*IT.java</exclude>
                    <exclude>**/*E2E.class</exclude>
                    <exclude>**/*E2E.java</exclude>
                </excludes>
            </configuration>
        </plugin>
        
        <!-- Unit tests (*Test.java) -->
        <plugin>
            <groupId>org.apache.maven.plugins</groupId>
            <artifactId>maven-surefire-plugin</artifactId>
            <configuration>
                <includes>
                    <include>**/*Test.java</include>
                </includes>
                <excludes>
                    <exclude>**/*IT.java</exclude>
                    <exclude>**/*E2E.java</exclude>
                </excludes>
            </configuration>
        </plugin>
        
        <!-- Integration tests (*IT.java) -->
        <plugin>
            <groupId>org.apache.maven.plugins</groupId>
            <artifactId>maven-failsafe-plugin</artifactId>
            <configuration>
                <includes>
                    <include>**/*IT.java</include>
                </includes>
            </configuration>
            <executions>
                <execution>
                    <goals>
                        <goal>integration-test</goal>
                        <goal>verify</goal>
                    </goals>
                </execution>
            </executions>
        </plugin>
    </plugins>
</build>

<!-- E2E tests profile -->
<profiles>
    <profile>
        <id>e2e</id>
        <build>
            <plugins>
                <plugin>
                    <groupId>org.apache.maven.plugins</groupId>
                    <artifactId>maven-failsafe-plugin</artifactId>
                    <configuration>
                        <includes>
                            <include>**/*E2E.java</include>
                        </includes>
                    </configuration>
                </plugin>
            </plugins>
        </build>
    </profile>
</profiles>
```

### Gradle Configuration (build.gradle.kts)

```kotlin
sourceSets {
    test {
        java {
            srcDir("src/main/java")
        }
    }
}

tasks.jar {
    exclude("**/*Test.class", "**/*IT.class", "**/*E2E.class")
    exclude("**/*Test.java", "**/*IT.java", "**/*E2E.java")
}

tasks.test {
    useJUnitPlatform()
    filter {
        includeTestsMatching("*Test")
        excludeTestsMatching("*IT")
        excludeTestsMatching("*E2E")
    }
}

tasks.register<Test>("integrationTest") {
    useJUnitPlatform()
    filter {
        includeTestsMatching("*IT")
    }
}

tasks.register<Test>("e2eTest") {
    useJUnitPlatform()
    filter {
        includeTestsMatching("*E2E")
    }
}
```

### Usage

```bash
# Unit tests only
mvn test                    # Maven
./gradlew test              # Gradle

# Unit + Integration tests
mvn verify                  # Maven
./gradlew integrationTest   # Gradle

# End-to-end tests
mvn verify -Pe2e            # Maven
./gradlew e2eTest           # Gradle
```

### Benefits of Co-location

| Benefit | Description |
|---------|-------------|
| **Package-private access** | Tests can access package-private members |
| **Discoverability** | Tests are next to the code they test |
| **Refactoring** | Move class = tests move with it |
| **Code review** | See tests in same PR diff |

---

## 🐍 Python

### Separate tests/ Directory

```
mypackage/
├── __init__.py
├── handler.py
└── tests/
    ├── __init__.py
    └── test_handler.py
```

### Co-located Alternative

```
mypackage/
├── __init__.py
├── handler.py
└── test_handler.py    ← Same directory
```

Both work with pytest. Co-located requires:
```bash
pytest mypackage/
```

| Location | Convention |
|----------|------------|
| `tests/` directory | **Traditional** |
| Co-located `test_*.py` | Supported, less common |

---

## Summary

| Language | Unit Tests | Integration Tests | E2E Tests |
|----------|------------|-------------------|-----------|
| **Rust** | `#[cfg(test)]` co-located | `tests/` directory | `tests/` |
| **Go** | `*_test.go` co-located | `*_test.go` (package _test) | Same |
| **Java** | `*Test.java` co-located | `*IT.java` co-located | `*E2E.java` co-located |
| **Python** | `test_*.py` or `tests/` | `tests/` | `tests/` |

### Idiomatic Recommendations

- **Rust**: Co-located `#[cfg(test)]` for unit, `tests/` for integration
- **Go**: Always co-located `*_test.go`, use `_test` package suffix for black-box
- **Java**: Co-located with naming: `*Test.java`, `*IT.java`, `*E2E.java`
- **Python**: `tests/` directory (pytest standard) or co-located `test_*.py`
