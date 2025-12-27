package com.jboot.openapi;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class OpenApiTest {

    @Test
    void spec_shouldCreateBuilder() {
        var builder = OpenApi.spec();

        assertNotNull(builder);
    }

    @Test
    void schemaOf_shouldCreateSchemaFromClass() {
        record User(Long id, String name, String email) {
        }

        var schema = OpenApi.schemaOf(User.class);

        assertNotNull(schema);
    }

    @Test
    void arrayOf_shouldCreateArraySchema() {
        record User(Long id, String name) {
        }

        var schema = OpenApi.arrayOf(User.class);

        assertNotNull(schema);
        assertEquals("array", schema.getType());
        assertNotNull(schema.getItems());
    }

    @Test
    void mapOf_shouldCreateMapSchema() {
        var schema = OpenApi.mapOf(String.class);

        assertNotNull(schema);
        assertEquals("object", schema.getType());
        assertNotNull(schema.getAdditionalProperties());
    }
}
