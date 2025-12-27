package com.jboot.cli;

import com.jboot.cli.api.*;
import com.jboot.cli.core.*;
import java.util.function.Consumer;

/**
 * Command-line interface building utilities.
 * 
 * <p>
 * This module enables building type-safe CLI applications with:
 * <ul>
 * <li>Argument parsing</li>
 * <li>Subcommands</li>
 * <li>Help generation</li>
 * <li>Environment variable binding</li>
 * </ul>
 * 
 * <h2>Example Usage:</h2>
 * 
 * <pre>{@code
 * var app = Cli.app("myapp")
 *         .version("1.0.0")
 *         .description("My CLI application");
 * 
 * app.command("greet")
 *         .description("Greet someone")
 *         .option("--name", String.class, "Name to greet")
 *         .flag("--loud", "Use uppercase")
 *         .action(ctx -> {
 *             String name = ctx.get("name", String.class);
 *             boolean loud = ctx.flag("loud");
 *             String greeting = "Hello, " + name + "!";
 *             System.out.println(loud ? greeting.toUpperCase() : greeting);
 *         });
 * 
 * app.command("version")
 *         .description("Show version")
 *         .action(ctx -> System.out.println(app.version()));
 * 
 * app.run(args);
 * }</pre>
 */
public final class Cli {

    private Cli() {
    }

    /**
     * Creates a new CLI application.
     */
    public static App app(String name) {
        return new DefaultApp(name);
    }

    /**
     * Creates a new CLI application with configuration.
     */
    public static App app(String name, Consumer<AppConfig> config) {
        var appConfig = new AppConfig();
        config.accept(appConfig);
        return new DefaultApp(name, appConfig);
    }

    /**
     * Parses command-line arguments.
     */
    public static ParsedArgs parse(String[] args) {
        return ArgumentParser.parse(args);
    }

    /**
     * Creates a progress bar for long-running operations.
     */
    public static ProgressBar progressBar(String label, long total) {
        return new ConsoleProgressBar(label, total);
    }

    /**
     * Creates a spinner for indeterminate operations.
     */
    public static Spinner spinner(String message) {
        return new ConsoleSpinner(message);
    }

    /**
     * Prompts for user input.
     */
    public static String prompt(String message) {
        return ConsoleInput.prompt(message);
    }

    /**
     * Prompts for password input (hidden).
     */
    public static String promptPassword(String message) {
        return ConsoleInput.promptPassword(message);
    }

    /**
     * Prompts for confirmation.
     */
    public static boolean confirm(String message) {
        return ConsoleInput.confirm(message);
    }
}
