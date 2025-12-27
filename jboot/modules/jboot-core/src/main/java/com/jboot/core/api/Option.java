package com.jboot.core.api;

import java.util.NoSuchElementException;
import java.util.Objects;
import java.util.Optional;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.function.Supplier;
import java.util.stream.Stream;

/**
 * An Option type representing an optional value.
 * <p>
 * Similar to {@link Optional} but with a more functional API.
 *
 * @param <T> The type of the contained value
 */
public sealed interface Option<T> permits Option.Some, Option.None {

    /**
     * Creates an Option with a value.
     */
    static <T> Option<T> some(T value) {
        return new Some<>(value);
    }

    /**
     * Creates an empty Option.
     */
    @SuppressWarnings("unchecked")
    static <T> Option<T> none() {
        return (Option<T>) None.INSTANCE;
    }

    /**
     * Creates an Option from a nullable value.
     */
    static <T> Option<T> of(T value) {
        return value == null ? none() : some(value);
    }

    /**
     * Creates an Option from an Optional.
     */
    static <T> Option<T> from(Optional<T> optional) {
        return optional.map(Option::some).orElseGet(Option::none);
    }

    /**
     * Returns true if this contains a value.
     */
    boolean isSome();

    /**
     * Returns true if this is empty.
     */
    boolean isNone();

    /**
     * Gets the value, throwing if empty.
     */
    T unwrap();

    /**
     * Gets the value or a default.
     */
    T unwrapOr(T defaultValue);

    /**
     * Gets the value or computes a default.
     */
    T unwrapOrElse(Supplier<T> supplier);

    /**
     * Maps the value.
     */
    <U> Option<U> map(Function<T, U> mapper);

    /**
     * Flat maps the value.
     */
    <U> Option<U> flatMap(Function<T, Option<U>> mapper);

    /**
     * Filters the value.
     */
    Option<T> filter(Predicate<T> predicate);

    /**
     * Executes an action if present.
     */
    Option<T> ifSome(Consumer<T> action);

    /**
     * Executes an action if empty.
     */
    Option<T> ifNone(Runnable action);

    /**
     * Converts to Optional.
     */
    Optional<T> toOptional();

    /**
     * Converts to Stream.
     */
    Stream<T> stream();

    /**
     * Matches on the option.
     */
    <U> U match(Function<T, U> onSome, Supplier<U> onNone);

    /**
     * Some case - contains a value.
     */
    record Some<T>(T value) implements Option<T> {
        public Some {
            Objects.requireNonNull(value, "value cannot be null");
        }

        @Override
        public boolean isSome() {
            return true;
        }

        @Override
        public boolean isNone() {
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
        public <U> Option<U> map(Function<T, U> mapper) {
            return Option.some(mapper.apply(value));
        }

        @Override
        public <U> Option<U> flatMap(Function<T, Option<U>> mapper) {
            return mapper.apply(value);
        }

        @Override
        public Option<T> filter(Predicate<T> predicate) {
            return predicate.test(value) ? this : Option.none();
        }

        @Override
        public Option<T> ifSome(Consumer<T> action) {
            action.accept(value);
            return this;
        }

        @Override
        public Option<T> ifNone(Runnable action) {
            return this;
        }

        @Override
        public Optional<T> toOptional() {
            return Optional.of(value);
        }

        @Override
        public Stream<T> stream() {
            return Stream.of(value);
        }

        @Override
        public <U> U match(Function<T, U> onSome, Supplier<U> onNone) {
            return onSome.apply(value);
        }
    }

    /**
     * None case - no value.
     */
    record None<T>() implements Option<T> {
        static final None<?> INSTANCE = new None<>();

        @Override
        public boolean isSome() {
            return false;
        }

        @Override
        public boolean isNone() {
            return true;
        }

        @Override
        public T unwrap() {
            throw new NoSuchElementException("Called unwrap on None");
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
        @SuppressWarnings("unchecked")
        public <U> Option<U> map(Function<T, U> mapper) {
            return (Option<U>) this;
        }

        @Override
        @SuppressWarnings("unchecked")
        public <U> Option<U> flatMap(Function<T, Option<U>> mapper) {
            return (Option<U>) this;
        }

        @Override
        public Option<T> filter(Predicate<T> predicate) {
            return this;
        }

        @Override
        public Option<T> ifSome(Consumer<T> action) {
            return this;
        }

        @Override
        public Option<T> ifNone(Runnable action) {
            action.run();
            return this;
        }

        @Override
        public Optional<T> toOptional() {
            return Optional.empty();
        }

        @Override
        public Stream<T> stream() {
            return Stream.empty();
        }

        @Override
        public <U> U match(Function<T, U> onSome, Supplier<U> onNone) {
            return onNone.get();
        }
    }
}
