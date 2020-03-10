package dev.luncj.cache.lru;

import dev.luncj.cache.Cache;

import java.util.HashMap;
import java.util.Map;

public class LRUCache<K, V> implements Cache<K, V> {

    private final int capacity;
    private final Map<K, Entry<K, V>> map;
    private final Entry<K, V> head;
    private final Entry<K, V> tail;

    public LRUCache(final int capacity) {
        this.capacity = capacity;
        this.map = new HashMap<K, Entry<K, V>>(capacity + 1);
        this.head = new Entry<K, V>(null, null);
        this.tail = new Entry<K, V>(null, null);
        head.next = tail;
        tail.previous = head;
    }

    @Override
    public V get(K key) {
        final var entry = map.get(key);

        if (entry == null) {
            return null;
        }

        entry.detach();
        push(entry);

        return entry.value;
    }

    @Override
    public void put(K key, V value) {
        var entry = map.get(key);
        if (entry == null) {

            if (map.size() == capacity) {
                map.remove(pop().key);
            }

            entry = new Entry<>(key, value);
            map.put(key, entry);
        } else {
            entry.value = value;
            entry.detach();
        }
        push(entry);
    }

    @Override
    public int capacity() {
        return capacity;
    }

    private Entry<K, V> pop() {
        if (head.next == tail) {
            return null;
        }
        final var n = head.next;
        n.detach();
        return n;
    }

    private void push(final Entry<K, V> entry) {
        final var p = tail.previous;
        p.next = entry;
        entry.previous = p;

        entry.next = tail;
        tail.previous = entry;
    }

    private static class Entry<K, V> {
        private final K key;
        private V value;
        private Entry<K, V> previous;
        private Entry<K, V> next;

        public Entry(final K key, final V value) {
            this.key = key;
            this.value = value;
        }

        public void detach() {
            if (previous == null || next == null) {
                return;
            }

            previous.next = next;
            next.previous = previous;

            previous = null;
            next = null;
        }
    }
}
