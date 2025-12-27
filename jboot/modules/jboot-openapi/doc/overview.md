# OpenAPI Module Overview

> **📝 Important**: This overview links to working examples and tests.

## WHAT: OpenAPI Documentation Generator

The `jboot-openapi` module provides utilities for generating OpenAPI 3.0 specifications from Java code, enabling automatic API documentation.

Key capabilities:
- **Spec Building** - Fluent API for OpenAPI specification
- **Schema Generation** - Automatic schema from Java classes
- **Multiple Formats** - Output as JSON or YAML
- **Validation** - Validate specifications

## WHY: API Documentation Automation

**Problems Solved**:
1. **Documentation Drift** - Code-first documentation stays in sync
2. **Boilerplate** - Automatic schema generation from classes
3. **Discoverability** - Standard OpenAPI format for tooling

**When to Use**: 
- REST API documentation
- API-first development
- Client SDK generation

**When NOT to Use**: 
- GraphQL APIs (use GraphQL schema)
- Internal-only services without documentation needs

## HOW: Usage Guide

### Basic Specification

```java
import com.jboot.openapi.OpenApi;

var spec = OpenApi.spec()
    .title("User API")
    .version("1.0.0")
    .description("API for managing users")
    .build();

spec.path("/users")
    .get(Operation.builder()
        .summary("List all users")
        .tag("users")
        .response(200, Schema.arrayOf(User.class))
        .build())
    .post(Operation.builder()
        .summary("Create a user")
        .tag("users")
        .requestBody(Schema.of(CreateUserRequest.class))
        .response(201, Schema.of(User.class))
        .build());

// Generate JSON
String json = spec.toJson();
```

**Available**:
- `spec()` - Create specification builder
- `schemaOf()` - Generate schema from class
- `arrayOf()` - Create array schema
- `mapOf()` - Create map schema

**Planned**:
- Annotation-based generation
- Spring integration
- Security scheme builders

### Schema Generation

```java
// From a record/class
record User(Long id, String name, String email) {}

var schema = OpenApi.schemaOf(User.class);
// Generates: { "type": "object", "properties": { "id": {...}, "name": {...}, "email": {...} } }

// Array of type
var arraySchema = OpenApi.arrayOf(User.class);

// Map with value type
var mapSchema = OpenApi.mapOf(String.class);
```

### Path Operations

```java
spec.path("/users/{id}")
    .parameter(Parameter.path("id", Schema.integer()))
    .get(Operation.builder()
        .summary("Get user by ID")
        .response(200, Schema.of(User.class))
        .response(404, "User not found")
        .build())
    .delete(Operation.builder()
        .summary("Delete user")
        .response(204, "User deleted")
        .build());
```

## Relationship to Other Modules

| Module | Purpose | Relationship |
|--------|---------|--------------|
| jboot-web | Web framework | Generate from routes |
| jboot-validation | Validation | Document constraints |
| jboot-http | HTTP client | Generate client |

## Examples and Tests

### Examples

**Location**: [`examples/`](../../examples/) directory

**Current examples**:
- See `OpenApiExample.java` in examples README

### Tests

**Location**: [`src/test/java/com/jboot/openapi/`](../src/test/java/com/jboot/openapi/)

**Current tests**:
- `OpenApiTest.java` - Spec creation, schema generation tests

### Testing Guidance

```bash
mvn test -pl modules/jboot-openapi
```

---

**Status**: Beta  
**OpenAPI Version**: 3.0.3
