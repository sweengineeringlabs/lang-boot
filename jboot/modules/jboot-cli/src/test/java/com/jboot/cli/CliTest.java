package com.jboot.cli;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class CliTest {

    @Test
    void app_shouldCreateWithName() {
        var app = Cli.app("testapp");

        assertNotNull(app);
        assertEquals("testapp", app.getName());
    }

    @Test
    void app_shouldAcceptConfiguration() {
        var app = Cli.app("testapp", config -> {
            config.setVersion("1.0.0");
            config.setDescription("Test application");
        });

        assertEquals("1.0.0", app.getVersion());
        assertEquals("Test application", app.getDescription());
    }

    @Test
    void progressBar_shouldCreate() {
        var progressBar = Cli.progressBar("Processing", 100);

        assertNotNull(progressBar);
    }

    @Test
    void spinner_shouldCreate() {
        var spinner = Cli.spinner("Loading...");

        assertNotNull(spinner);
    }
}
