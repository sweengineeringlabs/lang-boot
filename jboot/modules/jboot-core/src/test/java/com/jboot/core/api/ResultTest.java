package com.jboot.core.api;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import static org.assertj.core.api.Assertions.*;

class ResultTest {

    @Test
    @DisplayName("Ok result isOk returns true")
    void okResultIsOk() {
        Result<String, String> result = Result.ok("success");

        assertThat(result.isOk()).isTrue();
        assertThat(result.isErr()).isFalse();
    }

    @Test
    @DisplayName("Err result isErr returns true")
    void errResultIsErr() {
        Result<String, String> result = Result.err("error");

        assertThat(result.isOk()).isFalse();
        assertThat(result.isErr()).isTrue();
    }

    @Test
    @DisplayName("unwrap returns value for Ok")
    void unwrapReturnsValueForOk() {
        Result<Integer, String> result = Result.ok(42);

        assertThat(result.unwrap()).isEqualTo(42);
    }

    @Test
    @DisplayName("unwrap throws for Err")
    void unwrapThrowsForErr() {
        Result<Integer, String> result = Result.err("error");

        assertThatThrownBy(result::unwrap)
                .isInstanceOf(IllegalStateException.class)
                .hasMessageContaining("error");
    }

    @Test
    @DisplayName("unwrapOr returns value for Ok")
    void unwrapOrReturnsValueForOk() {
        Result<Integer, String> result = Result.ok(42);

        assertThat(result.unwrapOr(0)).isEqualTo(42);
    }

    @Test
    @DisplayName("unwrapOr returns default for Err")
    void unwrapOrReturnsDefaultForErr() {
        Result<Integer, String> result = Result.err("error");

        assertThat(result.unwrapOr(0)).isEqualTo(0);
    }

    @Test
    @DisplayName("map transforms Ok value")
    void mapTransformsOkValue() {
        Result<Integer, String> result = Result.ok(5);

        Result<Integer, String> mapped = result.map(x -> x * 2);

        assertThat(mapped.unwrap()).isEqualTo(10);
    }

    @Test
    @DisplayName("map does not affect Err")
    void mapDoesNotAffectErr() {
        Result<Integer, String> result = Result.err("error");

        Result<Integer, String> mapped = result.map(x -> x * 2);

        assertThat(mapped.isErr()).isTrue();
        assertThat(mapped.unwrapErr()).isEqualTo("error");
    }

    @Test
    @DisplayName("flatMap chains Ok results")
    void flatMapChainsOkResults() {
        Result<Integer, String> result = Result.ok(5);

        Result<Integer, String> chained = result.flatMap(x -> Result.ok(x * 2));

        assertThat(chained.unwrap()).isEqualTo(10);
    }

    @Test
    @DisplayName("flatMap short-circuits on Err")
    void flatMapShortCircuitsOnErr() {
        Result<Integer, String> result = Result.err("error");

        Result<Integer, String> chained = result.flatMap(x -> Result.ok(x * 2));

        assertThat(chained.isErr()).isTrue();
    }

    @Test
    @DisplayName("match works for Ok")
    void matchWorksForOk() {
        Result<Integer, String> result = Result.ok(42);

        String matched = result.match(
                v -> "value: " + v,
                e -> "error: " + e);

        assertThat(matched).isEqualTo("value: 42");
    }

    @Test
    @DisplayName("match works for Err")
    void matchWorksForErr() {
        Result<Integer, String> result = Result.err("oops");

        String matched = result.match(
                v -> "value: " + v,
                e -> "error: " + e);

        assertThat(matched).isEqualTo("error: oops");
    }

    @Test
    @DisplayName("of captures success")
    void ofCapturesSuccess() {
        Result<Integer, Exception> result = Result.of(() -> 42);

        assertThat(result.isOk()).isTrue();
        assertThat(result.unwrap()).isEqualTo(42);
    }

    @Test
    @DisplayName("of captures exception")
    void ofCapturesException() {
        Result<Integer, Exception> result = Result.of(() -> {
            throw new RuntimeException("boom");
        });

        assertThat(result.isErr()).isTrue();
        assertThat(result.unwrapErr()).hasMessage("boom");
    }

    @Test
    @DisplayName("ifOk executes action for Ok")
    void ifOkExecutesActionForOk() {
        var holder = new int[] { 0 };
        Result<Integer, String> result = Result.ok(42);

        result.ifOk(v -> holder[0] = v);

        assertThat(holder[0]).isEqualTo(42);
    }

    @Test
    @DisplayName("ifErr executes action for Err")
    void ifErrExecutesActionForErr() {
        var holder = new String[] { "" };
        Result<Integer, String> result = Result.err("error");

        result.ifErr(e -> holder[0] = e);

        assertThat(holder[0]).isEqualTo("error");
    }
}
