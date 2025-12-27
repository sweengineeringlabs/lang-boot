package com.jboot.core.api;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import static org.assertj.core.api.Assertions.*;

class OptionTest {

    @Test
    @DisplayName("Some isSome returns true")
    void someIsSome() {
        Option<String> option = Option.some("value");

        assertThat(option.isSome()).isTrue();
        assertThat(option.isNone()).isFalse();
    }

    @Test
    @DisplayName("None isNone returns true")
    void noneIsNone() {
        Option<String> option = Option.none();

        assertThat(option.isSome()).isFalse();
        assertThat(option.isNone()).isTrue();
    }

    @Test
    @DisplayName("of creates Some for non-null value")
    void ofCreatesForNonNull() {
        Option<String> option = Option.of("value");

        assertThat(option.isSome()).isTrue();
        assertThat(option.unwrap()).isEqualTo("value");
    }

    @Test
    @DisplayName("of creates None for null value")
    void ofCreatesNoneForNull() {
        Option<String> option = Option.of(null);

        assertThat(option.isNone()).isTrue();
    }

    @Test
    @DisplayName("unwrap returns value for Some")
    void unwrapReturnsValueForSome() {
        Option<Integer> option = Option.some(42);

        assertThat(option.unwrap()).isEqualTo(42);
    }

    @Test
    @DisplayName("unwrap throws for None")
    void unwrapThrowsForNone() {
        Option<Integer> option = Option.none();

        assertThatThrownBy(option::unwrap)
                .isInstanceOf(java.util.NoSuchElementException.class);
    }

    @Test
    @DisplayName("unwrapOr returns value for Some")
    void unwrapOrReturnsValueForSome() {
        Option<Integer> option = Option.some(42);

        assertThat(option.unwrapOr(0)).isEqualTo(42);
    }

    @Test
    @DisplayName("unwrapOr returns default for None")
    void unwrapOrReturnsDefaultForNone() {
        Option<Integer> option = Option.none();

        assertThat(option.unwrapOr(0)).isEqualTo(0);
    }

    @Test
    @DisplayName("map transforms Some value")
    void mapTransformsSomeValue() {
        Option<Integer> option = Option.some(5);

        Option<Integer> mapped = option.map(x -> x * 2);

        assertThat(mapped.unwrap()).isEqualTo(10);
    }

    @Test
    @DisplayName("map returns None for None")
    void mapReturnsNoneForNone() {
        Option<Integer> option = Option.none();

        Option<Integer> mapped = option.map(x -> x * 2);

        assertThat(mapped.isNone()).isTrue();
    }

    @Test
    @DisplayName("flatMap chains Some values")
    void flatMapChainsSomeValues() {
        Option<Integer> option = Option.some(5);

        Option<Integer> chained = option.flatMap(x -> Option.some(x * 2));

        assertThat(chained.unwrap()).isEqualTo(10);
    }

    @Test
    @DisplayName("flatMap short-circuits on None")
    void flatMapShortCircuitsOnNone() {
        Option<Integer> option = Option.none();

        Option<Integer> chained = option.flatMap(x -> Option.some(x * 2));

        assertThat(chained.isNone()).isTrue();
    }

    @Test
    @DisplayName("filter keeps matching Some")
    void filterKeepsMatchingSome() {
        Option<Integer> option = Option.some(10);

        Option<Integer> filtered = option.filter(x -> x > 5);

        assertThat(filtered.isSome()).isTrue();
        assertThat(filtered.unwrap()).isEqualTo(10);
    }

    @Test
    @DisplayName("filter removes non-matching Some")
    void filterRemovesNonMatchingSome() {
        Option<Integer> option = Option.some(3);

        Option<Integer> filtered = option.filter(x -> x > 5);

        assertThat(filtered.isNone()).isTrue();
    }

    @Test
    @DisplayName("match works for Some")
    void matchWorksForSome() {
        Option<Integer> option = Option.some(42);

        String result = option.match(
                v -> "value: " + v,
                () -> "none");

        assertThat(result).isEqualTo("value: 42");
    }

    @Test
    @DisplayName("match works for None")
    void matchWorksForNone() {
        Option<Integer> option = Option.none();

        String result = option.match(
                v -> "value: " + v,
                () -> "none");

        assertThat(result).isEqualTo("none");
    }

    @Test
    @DisplayName("toOptional converts Some")
    void toOptionalConvertsSome() {
        Option<String> option = Option.some("test");

        var optional = option.toOptional();

        assertThat(optional).isPresent();
        assertThat(optional.get()).isEqualTo("test");
    }

    @Test
    @DisplayName("toOptional converts None")
    void toOptionalConvertsNone() {
        Option<String> option = Option.none();

        var optional = option.toOptional();

        assertThat(optional).isEmpty();
    }

    @Test
    @DisplayName("stream returns single element for Some")
    void streamReturnsSingleElementForSome() {
        Option<String> option = Option.some("test");

        var list = option.stream().toList();

        assertThat(list).containsExactly("test");
    }

    @Test
    @DisplayName("stream returns empty for None")
    void streamReturnsEmptyForNone() {
        Option<String> option = Option.none();

        var list = option.stream().toList();

        assertThat(list).isEmpty();
    }

    @Test
    @DisplayName("ifSome executes action for Some")
    void ifSomeExecutesActionForSome() {
        var holder = new int[] { 0 };
        Option<Integer> option = Option.some(42);

        option.ifSome(v -> holder[0] = v);

        assertThat(holder[0]).isEqualTo(42);
    }

    @Test
    @DisplayName("ifNone executes action for None")
    void ifNoneExecutesActionForNone() {
        var holder = new boolean[] { false };
        Option<Integer> option = Option.none();

        option.ifNone(() -> holder[0] = true);

        assertThat(holder[0]).isTrue();
    }
}
