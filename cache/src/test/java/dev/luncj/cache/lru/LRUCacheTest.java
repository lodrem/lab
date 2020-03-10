package dev.luncj.cache.lru;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;

class LRUCacheTest {
    @Test
    public void test() {
        assertDoesNotThrow(() -> {
            final var cache = new LRUCache(2);

            cache.put(1, 1);
            cache.put(2, 2);
            assertEquals(1, cache.get(1));
            cache.put(3, 3);
            assertEquals(null, cache.get(2));
            cache.put(4, 4);
            assertEquals(null, cache.get(1));
            assertEquals(3, cache.get(3));
            assertEquals(4, cache.get(4));
        });
        assertDoesNotThrow(() -> {
            final var cache = new LRUCache(2);

            assertEquals(null, cache.get(2));
            cache.put(2, 6);
            assertEquals(null, cache.get(1));
            cache.put(1, 5);
            cache.put(1, 2);
            assertEquals(2, cache.get(1));
            assertEquals(6, cache.get(2));
        });
        assertDoesNotThrow(() -> {
            final var cache = new LRUCache(2);

            cache.put(2, 1);
            cache.put(1, 1);
            cache.put(2, 3);
            cache.put(4, 1);
            assertEquals(null, cache.get(1));
            assertEquals(3, cache.get(2));
        });
    }
}