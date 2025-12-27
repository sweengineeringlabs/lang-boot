package com.jboot.openapi;

import com.jboot.openapi.api.*;
import com.jboot.openapi.core.*;

/**
 * OpenAPI/Swagger documentation generation utilities.
 * 
 * <p>
 * This module enables automatic API documentation generation from Java
 * handlers.
 * 
 * <h2>Example Usage:</h2>
 * 
 * <pre>{@code
 * var spec = OpenApiSpec.builder()
 *         .title("My API")
 *         .version("1.0.0")
 *         .description("API for managing users")
 *         .build();
 * 
 * spec.path("/users")
 *         .get(Operation.builder()
 *                 .summary("List all users")
 *                 .tag("users")
 *                 .response(200, Schema.arrayOf(User.class))
 *                 .build())
 *         .post(Operation.builder()
 *                 .summary("Create a user")
 *                 .tag("users")
 *                 .requestBody(Schema.of(CreateUserRequest.class))
 *                 .response(201, Schema.of(User.class))
 *                 .build());
 * 
 * // Generate JSON
 * String json = spec.toJson();
 * 
 * // Generate YAML
 * String yaml = spec.toYaml();
 * }</pre>
 */
public final class OpenApi {

    private OpenApi() {
    }

    /**
     * Creates a new OpenAPI specification builder.
     */
    public static OpenApiSpec.Builder spec() {
        return OpenApiSpec.builder();
    }

    /**
     * Creates a schema from a Java class using reflection.
     */
    public static Schema schemaOf(Class<?> clazz) {
        return SchemaGenerator.generate(clazz);
    }

    /**
     * Creates an array schema.
     */
    public static Schema arrayOf(Class<?> itemClass) {
        return Schema.builder()
                .type("array")
                .items(schemaOf(itemClass))
                .build();
    }

    /**
     * Creates a map schema.
     */
    public static Schema mapOf(Class<?> valueClass) {
        return Schema.builder()
                .type("object")
                .additionalProperties(schemaOf(valueClass))
                .build();
    }
}
