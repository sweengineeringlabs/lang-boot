package com.jboot.cache.core;

import com.jboot.cache.api.Cache;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

import java.time.Duration;

import static org.assertj.core.api.Assertions.*;

class InMemoryCacheTest {

    private Cache<String, String> cache;

    @BeforeEach
    void setUp() {
        cache = new InMemoryCache<>(Duration.ofHours(1));
    }

    @Nested
    @DisplayName("Basic operations")
    class BasicOperationsTests {
        @Test
        void putAndGet() {
            cache.put("key", "value");

            var result = cache.get("key");

            assertThat(result).isPresent().contains("value");
        }

        @Test
        void getReturnsEmptyForMissingKey() {
            var result = cache.get("nonexistent");

            assertThat(result).isEmpty();
        }

        @Test
        void containsKeyReturnsTrueForExistingKey() {
            cache.put("key", "value");

            assertThat(cache.containsKey("key")).isTrue();
        }

        @Test
        void containsKeyReturnsFalseForMissingKey() {
            assertThat(cache.containsKey("nonexistent")).isFalse();
        }

        @Test
        void removeReturnsValue() {
            cache.put("key", "value");

            var result = cache.remove("key");

            assertThat(result).isPresent().contains("value");
            assertThat(cache.containsKey("key")).isFalse();
        }

        @Test
        void removeReturnsEmptyForMissingKey() {
            var result = cache.remove("nonexistent");

            assertThat(result).isEmpty();
        }

        @Test
        void clearRemovesAllEntries() {
            cache.put("key1", "value1");
            cache.put("key2", "value2");

            cache.clear();

            assertThat(cache.size()).isEqualTo(0);
        }

        @Test
        void sizeReturnsCorrectCount() {
            cache.put("key1", "value1");
            cache.put("key2", "value2");
            cache.put("key3", "value3");

            assertThat(cache.size()).isEqualTo(3);
        }
    }

    @Nested
    @DisplayName("TTL behavior")
    class TtlTests {
        @Test
        void entryExpiresAfterTtl() throws InterruptedException {
            cache.put("key", "value", Duration.ofMillis(50));

            assertThat(cache.get("key")).isPresent();

            Thread.sleep(100);

            assertThat(cache.get("key")).isEmpty();
        }

        @Test
        void entryDoesNotExpireBeforeTtl() throws InterruptedException {
            cache.put("key", "value", Duration.ofMillis(500));

            Thread.sleep(100);

            assertThat(cache.get("key")).isPresent();
        }

        @Test
        void containsKeyReturnsFalseForExpiredEntry() throws InterruptedException {
            cache.put("key", "value", Duration.ofMillis(50));

            Thread.sleep(100);

            assertThat(cache.containsKey("key")).isFalse();
        }

        @Test
        void zeroTtlMeansNoExpiration() throws InterruptedException {
            cache.put("key", "value", Duration.ZERO);

            Thread.sleep(100);

            assertThat(cache.get("key")).isPresent();
        }
    }

    @Nested
    @DisplayName("Statistics")
    class StatsTests {
        @Test
        void tracksHits() {
            cache.put("key", "value");

            cache.get("key");
            cache.get("key");
            cache.get("key");

            var stats = cache.getStats();
            assertThat(stats.hits()).isEqualTo(3);
        }

        @Test
        void tracksMisses() {
            cache.get("nonexistent1");
            cache.get("nonexistent2");

            var stats = cache.getStats();
            assertThat(stats.misses()).isEqualTo(2);
        }

        @Test
        void tracksEvictions() throws InterruptedException {
            cache.put("key", "value", Duration.ofMillis(50));

            Thread.sleep(100);

            cache.get("key"); // This triggers eviction

            var stats = cache.getStats();
            assertThat(stats.evictions()).isEqualTo(1);
        }

        @Test
        void calculatesHitRate() {
            cache.put("key", "value");

            cache.get("key"); // hit
            cache.get("key"); // hit
            cache.get("miss"); // miss

            var stats = cache.getStats();
            assertThat(stats.hitRate()).isEqualTo(2.0 / 3.0);
        }

        @Test
        void emptyStatsHasZeroHitRate() {
            var stats = cache.getStats();
            assertThat(stats.hitRate()).isEqualTo(0.0);
        }
    }

    @Nested
    @DisplayName("Overwrite behavior")
    class OverwriteTests {
        @Test
        void putOverwritesExistingValue() {
            cache.put("key", "value1");
            cache.put("key", "value2");

            assertThat(cache.get("key")).contains("value2");
        }

        @Test
        void putWithNewTtlResetsExpiration() throws InterruptedException {
            cache.put("key", "value1", Duration.ofMillis(50));

            Thread.sleep(30);

            cache.put("key", "value2", Duration.ofMillis(500));

            Thread.sleep(50);

            assertThat(cache.get("key")).isPresent().contains("value2");
        }
    }

    @Nested
    @DisplayName("Different value types")
    class TypeTests {
        @Test
        void worksWithIntegerKeys() {
            var intCache = new InMemoryCache<Integer, String>();

            intCache.put(1, "one");
            intCache.put(2, "two");

            assertThat(intCache.get(1)).contains("one");
            assertThat(intCache.get(2)).contains("two");
        }

        @Test
        void worksWithComplexValues() {
            record User(String name, int age) {
            }
            var userCache = new InMemoryCache<String, User>();

            userCache.put("user1", new User("John", 30));

            var result = userCache.get("user1");
            assertThat(result).isPresent();
            assertThat(result.get().name()).isEqualTo("John");
        }
    }
}
