package dev.luncj.cache.lfu;

import dev.luncj.cache.Cache;

import java.util.HashMap;
import java.util.Map;

public class LFUCache<K, V> implements Cache<K, V> {
    private final int capacity;
    private final Map<K, Entry<K, V>> map;
    private final Map<Integer, Frequency<K, V>> frequencies;
    private int minFrequency;

    public LFUCache(final int capacity) {
        this.capacity = capacity;
        this.map = new HashMap<>(capacity + 1);
        this.frequencies = new HashMap<>();
        this.minFrequency = 0;
    }

    private void updateFrequency(Entry<K, V> entry) {
        entry.detach();
        entry.usedCount++;

        var freq = frequencies.get(entry.usedCount);
        if (freq == null) {
            freq = new Frequency<>();
            frequencies.put(entry.usedCount, freq);
        }

        freq.push(entry);
        if (frequencies.get(minFrequency).isEmpty()) {
            minFrequency++;
        }
    }

    @Override
    public V get(K key) {
        final var entry = map.get(key);
        if (entry == null) {
            return null;
        }
        updateFrequency(entry);

        return entry.value;
    }

    @Override
    public void put(K key, V value) {
        var entry = map.get(key);
        if (entry == null) {
            evict();
            entry = new Entry<>(key, value);
            map.put(key, entry);

            minFrequency = 0;
            var freq = frequencies.get(minFrequency);
            if (freq == null) {
                freq = new Frequency<>();
                frequencies.put(minFrequency, freq);
            }
            freq.push(entry);
        } else {
            entry.value = value;
            updateFrequency(entry);
        }
    }

    @Override
    public int capacity() {
        return capacity;
    }

    public Entry<K, V> evict() {
        if (map.size() < capacity) {
            return null;
        }

        final var freq = frequencies.get(minFrequency);
        final var entry = freq.pop();
        map.remove(entry.key);
        return entry;
    }

    private static class Frequency<K, V> {
        private final Entry<K, V> head;
        private final Entry<K, V> tail;

        public Frequency() {
            head = new Entry<>(null, null);
            tail = new Entry<>(null, null);
            head.next = tail;
            tail.previous = head;
        }

        public Entry<K, V> pop() {
            if (isEmpty()) {
                return null;
            }

            final var n = head.next;
            n.detach();
            return n;
        }

        public void push(final Entry<K, V> entry) {
            entry.previous = tail.previous;
            entry.next = tail;
            entry.previous.next = entry;
            entry.next.previous = entry;
        }

        public boolean isEmpty() {
            return head.next == tail;
        }
    }

    private static class Entry<K, V> {
        private final K key;
        private V value;
        private Entry<K, V> previous;
        private Entry<K, V> next;
        private int usedCount;

        public Entry(final K key, final V value) {
            this.key = key;
            this.value = value;
            this.usedCount = 0;
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
