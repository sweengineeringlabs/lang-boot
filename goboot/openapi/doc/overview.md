# OpenAPI Module Overview

> **📝 Important**: This overview links to working examples and tests.

## WHAT: OpenAPI Documentation Generator

The `openapi` package provides utilities for generating OpenAPI 3.0 specifications from Go code, enabling automatic API documentation.

Key capabilities:
- **Spec Building** - Build OpenAPI specs programmatically
- **Schema Generation** - Generate schemas from Go structs
- **Multiple Formats** - Output as JSON or YAML
- **Full OpenAPI 3.0** - Complete specification support

## WHY: API Documentation Automation

**Problems Solved**:
1. **Documentation Drift** - Code-first keeps docs in sync
2. **Boilerplate** - Automatic schema from structs
3. **Tooling Support** - Standard OpenAPI format

**When to Use**: 
- REST API documentation
- API-first development
- Client SDK generation

**When NOT to Use**: 
- GraphQL APIs
- Internal services without docs needs

## HOW: Usage Guide

### Basic Specification

```go
import "dev.engineeringlabs/goboot/openapi"

spec := openapi.NewSpec("User API", "1.0.0")
spec.Info.Description = "API for managing users"

spec.Paths["/users"] = openapi.Path{
    Get: &openapi.Operation{
        Summary: "List all users",
        Tags:    []string{"users"},
        Responses: map[string]openapi.Response{
            "200": {
                Description: "Success",
                Content: map[string]openapi.MediaType{
                    "application/json": {
                        Schema: openapi.ArrayOf(User{}),
                    },
                },
            },
        },
    },
}
```

**Available**:
- `NewSpec()` - Create specification
- `SchemaFrom()` - Generate schema from struct
- `ArrayOf()` - Create array schema
- Full OpenAPI 3.0 type support

**Planned**:
- Struct tag-based generation
- Chi/Gin router integration
- Validation from openapi specs

### Schema Generation

```go
type User struct {
    ID    int64  `json:"id"`
    Name  string `json:"name"`
    Email string `json:"email"`
}

// Generate schema
schema := openapi.SchemaFrom(User{})

// Array of type
arraySchema := openapi.ArrayOf(User{})
```

### Path Operations

```go
spec.Paths["/users/{id}"] = openapi.Path{
    Parameters: []openapi.Parameter{
        {
            Name:     "id",
            In:       "path",
            Required: true,
            Schema:   &openapi.Schema{Type: "integer"},
        },
    },
    Get: &openapi.Operation{
        Summary: "Get user by ID",
        Responses: map[string]openapi.Response{
            "200": {Description: "Success"},
            "404": {Description: "Not found"},
        },
    },
}
```

### Output as JSON

```go
import "encoding/json"

output, _ := json.MarshalIndent(spec, "", "  ")
fmt.Println(string(output))
```

## Relationship to Other Modules

| Module | Purpose | Relationship |
|--------|---------|--------------|
| web | Web framework | Generate from routes |
| validation | Validation | Document constraints |
| http | HTTP client | Generate client |

## Examples and Tests

### Examples

**Location**: [`examples/`](../examples/)

**Current examples**:
- [`openapi_example.go`](../examples/openapi_example.go) - Complete spec generation

### Tests

**Location**: [`openapi_test.go`](openapi_test.go)

**Current tests**:
- `TestNewSpec` - Spec creation
- `TestArrayOf` - Array schema
- `TestSchemaFrom` - Struct to schema

### Testing Guidance

```bash
go test ./openapi/...
```

---

**Status**: Beta  
**OpenAPI Version**: 3.0.3
