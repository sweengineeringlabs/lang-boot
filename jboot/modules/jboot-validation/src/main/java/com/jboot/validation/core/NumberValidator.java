package com.jboot.validation.core;

import com.jboot.validation.api.*;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

/**
 * Fluent builder for number validation.
 *
 * @param <T> The number type
 */
public class NumberValidator<T extends Number & Comparable<T>> implements Validator<T> {

    private final String fieldName;
    private final List<Rule<T>> rules = new ArrayList<>();

    private NumberValidator(String fieldName) {
        this.fieldName = fieldName;
    }

    /**
     * Creates a new number validator builder.
     */
    public static <T extends Number & Comparable<T>> NumberValidator<T> forField(String fieldName) {
        return new NumberValidator<>(fieldName);
    }

    /**
     * Requires the number to be non-null.
     */
    public NumberValidator<T> notNull() {
        rules.add(new Rule<>(
                n -> n != null,
                "must not be null",
                "NOT_NULL"));
        return this;
    }

    /**
     * Requires minimum value (inclusive).
     */
    public NumberValidator<T> min(T min) {
        rules.add(new Rule<>(
                n -> n != null && n.compareTo(min) >= 0,
                "must be at least " + min,
                "MIN"));
        return this;
    }

    /**
     * Requires maximum value (inclusive).
     */
    public NumberValidator<T> max(T max) {
        rules.add(new Rule<>(
                n -> n != null && n.compareTo(max) <= 0,
                "must be at most " + max,
                "MAX"));
        return this;
    }

    /**
     * Requires value to be within range (inclusive).
     */
    public NumberValidator<T> range(T min, T max) {
        return min(min).max(max);
    }

    /**
     * Requires positive value (> 0).
     */
    public NumberValidator<T> positive() {
        rules.add(new Rule<>(
                n -> n != null && n.doubleValue() > 0,
                "must be positive",
                "POSITIVE"));
        return this;
    }

    /**
     * Requires non-negative value (>= 0).
     */
    public NumberValidator<T> nonNegative() {
        rules.add(new Rule<>(
                n -> n != null && n.doubleValue() >= 0,
                "must be non-negative",
                "NON_NEGATIVE"));
        return this;
    }

    /**
     * Requires negative value (< 0).
     */
    public NumberValidator<T> negative() {
        rules.add(new Rule<>(
                n -> n != null && n.doubleValue() < 0,
                "must be negative",
                "NEGATIVE"));
        return this;
    }

    /**
     * Adds a custom validation rule.
     */
    public NumberValidator<T> custom(Predicate<T> predicate, String message) {
        rules.add(new Rule<>(predicate, message, "CUSTOM"));
        return this;
    }

    @Override
    public ValidationResult validate(T value) {
        var errors = new ArrayList<ValidationError>();
        for (var rule : rules) {
            if (!rule.predicate.test(value)) {
                errors.add(ValidationError.of(fieldName, rule.message, rule.code, value));
            }
        }
        return errors.isEmpty() ? ValidationResult.success() : ValidationResult.failure(errors);
    }

    private record Rule<T>(Predicate<T> predicate, String message, String code) {
    }
}
