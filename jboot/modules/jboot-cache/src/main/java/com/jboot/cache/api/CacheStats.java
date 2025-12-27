package com.jboot.cache.api;

/**
 * Statistics for cache operations.
 */
public record CacheStats(
        long hits,
        long misses,
        long evictions,
        long size) {
    /**
     * Calculates the hit rate (0.0 to 1.0).
     */
    public double hitRate() {
        long total = hits + misses;
        return total == 0 ? 0.0 : (double) hits / total;
    }

    /**
     * Creates empty stats.
     */
    public static CacheStats empty() {
        return new CacheStats(0, 0, 0, 0);
    }
}
