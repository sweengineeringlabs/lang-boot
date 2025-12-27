package com.jboot.validation.core;

import com.jboot.validation.api.*;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;
import java.util.regex.Pattern;

/**
 * Fluent builder for string validation.
 */
public class StringValidator implements Validator<String> {

    private final String fieldName;
    private final List<Rule> rules = new ArrayList<>();

    private StringValidator(String fieldName) {
        this.fieldName = fieldName;
    }

    /**
     * Creates a new string validator builder.
     */
    public static StringValidator forField(String fieldName) {
        return new StringValidator(fieldName);
    }

    /**
     * Requires the string to be non-null and non-empty.
     */
    public StringValidator notEmpty() {
        rules.add(new Rule(
                s -> s != null && !s.isEmpty(),
                "must not be empty",
                "NOT_EMPTY"));
        return this;
    }

    /**
     * Requires the string to be non-null and non-blank.
     */
    public StringValidator notBlank() {
        rules.add(new Rule(
                s -> s != null && !s.isBlank(),
                "must not be blank",
                "NOT_BLANK"));
        return this;
    }

    /**
     * Requires minimum length.
     */
    public StringValidator minLength(int min) {
        rules.add(new Rule(
                s -> s != null && s.length() >= min,
                "must be at least " + min + " characters",
                "MIN_LENGTH"));
        return this;
    }

    /**
     * Requires maximum length.
     */
    public StringValidator maxLength(int max) {
        rules.add(new Rule(
                s -> s == null || s.length() <= max,
                "must be at most " + max + " characters",
                "MAX_LENGTH"));
        return this;
    }

    /**
     * Requires length to be within range.
     */
    public StringValidator length(int min, int max) {
        return minLength(min).maxLength(max);
    }

    /**
     * Requires the string to match a pattern.
     */
    public StringValidator pattern(String regex) {
        var pattern = Pattern.compile(regex);
        rules.add(new Rule(
                s -> s != null && pattern.matcher(s).matches(),
                "must match pattern: " + regex,
                "PATTERN"));
        return this;
    }

    /**
     * Requires valid email format.
     */
    public StringValidator email() {
        var emailPattern = Pattern.compile("^[A-Za-z0-9+_.-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$");
        rules.add(new Rule(
                s -> s != null && emailPattern.matcher(s).matches(),
                "must be a valid email address",
                "EMAIL"));
        return this;
    }

    /**
     * Requires alphanumeric characters only.
     */
    public StringValidator alphanumeric() {
        rules.add(new Rule(
                s -> s != null && s.matches("^[a-zA-Z0-9]+$"),
                "must contain only alphanumeric characters",
                "ALPHANUMERIC"));
        return this;
    }

    /**
     * Requires numeric characters only.
     */
    public StringValidator numeric() {
        rules.add(new Rule(
                s -> s != null && s.matches("^[0-9]+$"),
                "must contain only numeric characters",
                "NUMERIC"));
        return this;
    }

    /**
     * Requires the value to be one of the allowed values.
     */
    public StringValidator oneOf(String... allowedValues) {
        var allowed = List.of(allowedValues);
        rules.add(new Rule(
                s -> s != null && allowed.contains(s),
                "must be one of: " + String.join(", ", allowedValues),
                "ONE_OF"));
        return this;
    }

    /**
     * Adds a custom validation rule.
     */
    public StringValidator custom(Predicate<String> predicate, String message) {
        rules.add(new Rule(predicate, message, "CUSTOM"));
        return this;
    }

    @Override
    public ValidationResult validate(String value) {
        var errors = new ArrayList<ValidationError>();
        for (var rule : rules) {
            if (!rule.predicate.test(value)) {
                errors.add(ValidationError.of(fieldName, rule.message, rule.code, value));
            }
        }
        return errors.isEmpty() ? ValidationResult.success() : ValidationResult.failure(errors);
    }

    private record Rule(Predicate<String> predicate, String message, String code) {
    }
}
