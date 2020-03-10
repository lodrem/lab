package dev.luncj.cache.lfu;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;

class LFUCacheTest {
    @Test
    public void test() {
        assertDoesNotThrow(() -> {
            final var cache = new LFUCache(2);

            cache.put(1, 1);
            cache.put(2, 2);
            assertEquals(1, cache.get(1));
            cache.put(3, 3);
            assertEquals(null, cache.get(2));
            assertEquals(3, cache.get(3));
            cache.put(4, 4);
            assertEquals(null, cache.get(1));
            assertEquals(3, cache.get(3));
            assertEquals(4, cache.get(4));
        });
        assertDoesNotThrow(() -> {
            final var cache = new LFUCache(3);

            cache.put(2, 2);
            cache.put(1, 1);
            assertEquals(2, cache.get(2));
            assertEquals(1, cache.get(1));
            assertEquals(2, cache.get(2));
            cache.put(3, 3);
            cache.put(4, 4);
            assertEquals(null, cache.get(3));
            assertEquals(2, cache.get(2));
            assertEquals(1, cache.get(1));
            assertEquals(4, cache.get(4));
        });
        assertDoesNotThrow(() -> {
            final var cache = new LFUCache(2);

            cache.put(2, 1);
            cache.put(1, 1);
            cache.put(2, 3);
            cache.put(4, 1);
            assertEquals(null, cache.get(1));
            assertEquals(3, cache.get(2));
        });
    }
}