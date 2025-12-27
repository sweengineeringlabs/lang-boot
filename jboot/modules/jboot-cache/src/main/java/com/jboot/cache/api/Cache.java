package com.jboot.cache.api;

import java.time.Duration;
import java.util.Optional;

/**
 * Cache abstraction for storing and retrieving values.
 *
 * @param <K> The key type
 * @param <V> The value type
 */
public interface Cache<K, V> {

    /**
     * Gets a value from the cache.
     *
     * @param key The key
     * @return The value if present
     */
    Optional<V> get(K key);

    /**
     * Puts a value in the cache with default TTL.
     *
     * @param key   The key
     * @param value The value
     */
    void put(K key, V value);

    /**
     * Puts a value in the cache with specified TTL.
     *
     * @param key   The key
     * @param value The value
     * @param ttl   Time-to-live
     */
    void put(K key, V value, Duration ttl);

    /**
     * Removes a value from the cache.
     *
     * @param key The key
     * @return The removed value if present
     */
    Optional<V> remove(K key);

    /**
     * Checks if a key exists in the cache.
     *
     * @param key The key
     * @return true if the key exists
     */
    boolean containsKey(K key);

    /**
     * Clears all entries from the cache.
     */
    void clear();

    /**
     * Returns the number of entries in the cache.
     */
    long size();

    /**
     * Gets cache statistics.
     */
    CacheStats getStats();
}
