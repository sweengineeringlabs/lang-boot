package com.jboot.validation.api;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

/**
 * Default implementation of ValidationResult.
 */
public record DefaultValidationResult(List<ValidationError> errors) implements ValidationResult {

    public DefaultValidationResult {
        errors = List.copyOf(errors);
    }

    @Override
    public boolean isValid() {
        return errors.isEmpty();
    }

    @Override
    public List<ValidationError> getErrors() {
        return errors;
    }

    @Override
    public List<ValidationError> getErrorsForField(String fieldName) {
        return errors.stream()
                .filter(e -> e.field().equals(fieldName))
                .collect(Collectors.toList());
    }

    @Override
    public void throwIfInvalid() throws ValidationException {
        if (!isValid()) {
            throw new ValidationException(errors);
        }
    }

    @Override
    public ValidationResult merge(ValidationResult other) {
        if (this.isValid() && other.isValid()) {
            return this;
        }
        var merged = new ArrayList<ValidationError>();
        merged.addAll(this.errors);
        merged.addAll(other.getErrors());
        return new DefaultValidationResult(merged);
    }
}
