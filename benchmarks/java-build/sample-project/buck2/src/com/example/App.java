package com.example;

import java.util.List;
import java.util.stream.IntStream;

/**
 * Sample application for benchmarking build tools.
 * Contains enough code to make compilation measurable.
 */
public class App {
    private final String name;
    private final List<String> features;

    public App(String name) {
        this.name = name;
        this.features = List.of(
                "feature-1", "feature-2", "feature-3",
                "feature-4", "feature-5", "feature-6");
    }

    public String getName() {
        return name;
    }

    public List<String> getFeatures() {
        return features;
    }

    public int computeSum(int n) {
        return IntStream.rangeClosed(1, n).sum();
    }

    public String greet(String who) {
        return String.format("Hello, %s! Welcome to %s.", who, name);
    }

    public static void main(String[] args) {
        if (args.length > 0 && "--version".equals(args[0])) {
            System.out.println("1.0.0");
            return;
        }

        var app = new App("BenchmarkApp");
        System.out.println(app.greet("World"));
        System.out.println("Sum 1-100: " + app.computeSum(100));
        System.out.println("Features: " + app.getFeatures());
    }
}
