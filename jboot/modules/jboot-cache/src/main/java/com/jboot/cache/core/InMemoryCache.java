package com.jboot.cache.core;

import com.jboot.cache.api.Cache;
import com.jboot.cache.api.CacheStats;

import java.time.Duration;
import java.time.Instant;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;

/**
 * In-memory cache implementation with TTL support.
 *
 * @param <K> The key type
 * @param <V> The value type
 */
public class InMemoryCache<K, V> implements Cache<K, V> {

    private final Map<K, CacheEntry<V>> cache = new ConcurrentHashMap<>();
    private final Duration defaultTtl;

    private final AtomicLong hits = new AtomicLong(0);
    private final AtomicLong misses = new AtomicLong(0);
    private final AtomicLong evictions = new AtomicLong(0);

    /**
     * Creates a cache with the specified default TTL.
     */
    public InMemoryCache(Duration defaultTtl) {
        this.defaultTtl = defaultTtl;
    }

    /**
     * Creates a cache with 1-hour default TTL.
     */
    public InMemoryCache() {
        this(Duration.ofHours(1));
    }

    @Override
    public Optional<V> get(K key) {
        var entry = cache.get(key);
        if (entry == null) {
            misses.incrementAndGet();
            return Optional.empty();
        }

        if (entry.isExpired()) {
            cache.remove(key);
            evictions.incrementAndGet();
            misses.incrementAndGet();
            return Optional.empty();
        }

        hits.incrementAndGet();
        return Optional.of(entry.value());
    }

    @Override
    public void put(K key, V value) {
        put(key, value, defaultTtl);
    }

    @Override
    public void put(K key, V value, Duration ttl) {
        var expiresAt = ttl == null || ttl.isZero()
                ? null
                : Instant.now().plus(ttl);
        cache.put(key, new CacheEntry<>(value, expiresAt));
    }

    @Override
    public Optional<V> remove(K key) {
        var entry = cache.remove(key);
        if (entry == null) {
            return Optional.empty();
        }
        return Optional.of(entry.value());
    }

    @Override
    public boolean containsKey(K key) {
        var entry = cache.get(key);
        if (entry == null) {
            return false;
        }
        if (entry.isExpired()) {
            cache.remove(key);
            evictions.incrementAndGet();
            return false;
        }
        return true;
    }

    @Override
    public void clear() {
        cache.clear();
    }

    @Override
    public long size() {
        // Clean up expired entries first
        cache.entrySet().removeIf(e -> e.getValue().isExpired());
        return cache.size();
    }

    @Override
    public CacheStats getStats() {
        return new CacheStats(
                hits.get(),
                misses.get(),
                evictions.get(),
                cache.size());
    }

    private record CacheEntry<V>(V value, Instant expiresAt) {
        boolean isExpired() {
            return expiresAt != null && Instant.now().isAfter(expiresAt);
        }
    }
}
