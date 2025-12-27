package com.jboot.core.api;

import java.util.Objects;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Supplier;

/**
 * A Result type representing either success (Ok) or failure (Err).
 * <p>
 * Provides functional error handling without exceptions.
 *
 * @param <T> The type of the success value
 * @param <E> The type of the error value
 */
public sealed interface Result<T, E> permits Result.Ok, Result.Err {

    /**
     * Creates a successful result.
     */
    static <T, E> Result<T, E> ok(T value) {
        return new Ok<>(value);
    }

    /**
     * Creates a failed result.
     */
    static <T, E> Result<T, E> err(E error) {
        return new Err<>(error);
    }

    /**
     * Creates a result from a supplier that might throw.
     */
    static <T> Result<T, Exception> of(Supplier<T> supplier) {
        try {
            return ok(supplier.get());
        } catch (Exception e) {
            return err(e);
        }
    }

    /**
     * Returns true if this is a success.
     */
    boolean isOk();

    /**
     * Returns true if this is a failure.
     */
    boolean isErr();

    /**
     * Gets the success value, throwing if this is an error.
     */
    T unwrap();

    /**
     * Gets the success value or a default.
     */
    T unwrapOr(T defaultValue);

    /**
     * Gets the success value or computes a default.
     */
    T unwrapOrElse(Supplier<T> supplier);

    /**
     * Gets the error value, throwing if this is a success.
     */
    E unwrapErr();

    /**
     * Maps the success value.
     */
    <U> Result<U, E> map(Function<T, U> mapper);

    /**
     * Maps the error value.
     */
    <F> Result<T, F> mapErr(Function<E, F> mapper);

    /**
     * Flat maps the success value.
     */
    <U> Result<U, E> flatMap(Function<T, Result<U, E>> mapper);

    /**
     * Executes an action if this is a success.
     */
    Result<T, E> ifOk(Consumer<T> action);

    /**
     * Executes an action if this is a failure.
     */
    Result<T, E> ifErr(Consumer<E> action);

    /**
     * Matches on the result, returning a value.
     */
    <U> U match(Function<T, U> onOk, Function<E, U> onErr);

    /**
     * Success case.
     */
    record Ok<T, E>(T value) implements Result<T, E> {
        public Ok {
            Objects.requireNonNull(value, "value cannot be null");
        }

        @Override
        public boolean isOk() {
            return true;
        }

        @Override
        public boolean isErr() {
            return false;
        }

        @Override
        public T unwrap() {
            return value;
        }

        @Override
        public T unwrapOr(T defaultValue) {
            return value;
        }

        @Override
        public T unwrapOrElse(Supplier<T> supplier) {
            return value;
        }

        @Override
        public E unwrapErr() {
            throw new IllegalStateException("Called unwrapErr on Ok");
        }

        @Override
        public <U> Result<U, E> map(Function<T, U> mapper) {
            return Result.ok(mapper.apply(value));
        }

        @Override
        public <F> Result<T, F> mapErr(Function<E, F> mapper) {
            return Result.ok(value);
        }

        @Override
        public <U> Result<U, E> flatMap(Function<T, Result<U, E>> mapper) {
            return mapper.apply(value);
        }

        @Override
        public Result<T, E> ifOk(Consumer<T> action) {
            action.accept(value);
            return this;
        }

        @Override
        public Result<T, E> ifErr(Consumer<E> action) {
            return this;
        }

        @Override
        public <U> U match(Function<T, U> onOk, Function<E, U> onErr) {
            return onOk.apply(value);
        }
    }

    /**
     * Failure case.
     */
    record Err<T, E>(E error) implements Result<T, E> {
        public Err {
            Objects.requireNonNull(error, "error cannot be null");
        }

        @Override
        public boolean isOk() {
            return false;
        }

        @Override
        public boolean isErr() {
            return true;
        }

        @Override
        public T unwrap() {
            throw new IllegalStateException("Called unwrap on Err: " + error);
        }

        @Override
        public T unwrapOr(T defaultValue) {
            return defaultValue;
        }

        @Override
        public T unwrapOrElse(Supplier<T> supplier) {
            return supplier.get();
        }

        @Override
        public E unwrapErr() {
            return error;
        }

        @Override
        @SuppressWarnings("unchecked")
        public <U> Result<U, E> map(Function<T, U> mapper) {
            return (Result<U, E>) this;
        }

        @Override
        public <F> Result<T, F> mapErr(Function<E, F> mapper) {
            return Result.err(mapper.apply(error));
        }

        @Override
        @SuppressWarnings("unchecked")
        public <U> Result<U, E> flatMap(Function<T, Result<U, E>> mapper) {
            return (Result<U, E>) this;
        }

        @Override
        public Result<T, E> ifOk(Consumer<T> action) {
            return this;
        }

        @Override
        public Result<T, E> ifErr(Consumer<E> action) {
            action.accept(error);
            return this;
        }

        @Override
        public <U> U match(Function<T, U> onOk, Function<E, U> onErr) {
            return onErr.apply(error);
        }
    }
}
