/*
 * @lc app=leetcode id=460 lang=cpp
 *
 * [460] LFU Cache
 */

// @lc code=start
#include <climits>
#include <cstdio>
#include <list>
#include <unordered_map>
class LFUCache {
    using Iter = std::list<int>::iterator;
    struct Node {
        int key;
        int value;
        int freq;
        Iter iter;
        Node() {}
        Node(int k, int v, int f) : key(k), value(v), freq(f) {}
    };

    using KeyMapIter = std::unordered_map<int, Node>::iterator;
    using FreqMapIter = std::unordered_map<int, std::list<int>>::iterator;

  private:
    int capacity;
    std::unordered_map<int, Node> keyMap;
    std::unordered_map<int, std::list<int>> freqMap;
    int minFreq = INT_MAX;

    void removeNode(Node &node) {
        freqMap[node.freq].erase(node.iter);
        if (freqMap[node.freq].empty()) {
            if (node.freq == minFreq) {
                minFreq = INT_MAX;
            }
            freqMap.erase(node.freq);
        }
    }

    void addNode(Node &node) {
        freqMap[node.freq].push_back(node.key);
        minFreq = std::min(minFreq, node.freq);
        node.iter = --freqMap[node.freq].end();
    }

  public:
    LFUCache(int capacity) : capacity(capacity) { keyMap.reserve(capacity); }

    int get(int key) {
        // printf("get key %d\n", key);
        KeyMapIter it = keyMap.find(key);
        if (it == keyMap.end()) {
            return -1;
        }
        Node &node = it->second;
        removeNode(node);
        node.freq++;
        addNode(node);
        return node.value;
    }

    void put(int key, int value) {
        // printf("put key %d, value %d\n", key, value);
        KeyMapIter it = keyMap.find(key);
        if (it == keyMap.end()) {
            if (keyMap.size() >= (size_t)capacity) {
                Iter it = freqMap[minFreq].begin();
                int key = *it;
                // printf("evict key %d, count %d\n", key, minFreq);
                Node &node = keyMap[key];
                removeNode(node);
                keyMap.erase(key);
            }
            keyMap.emplace(key, Node(key, value, 1));
            addNode(keyMap[key]);
        } else {
            Node &node = it->second;
            removeNode(node);
            node.value = value;
            node.freq++;
            addNode(node);
        }
    }
};

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache* obj = new LFUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// @lc code=end

int main(int argc, const char **argv) {
    LFUCache cache(2);
    cache.put(1, 1);
    cache.put(2, 2);
    printf("%d\n", cache.get(1)); // returns 1
    cache.put(3, 3);
    printf("%d\n", cache.get(2)); // returns -1 (not found)
    printf("%d\n", cache.get(3)); // returns 3.
    cache.put(4, 4);              // evicts key 1.
    printf("%d\n", cache.get(1)); // returns -1 (not found)
    printf("%d\n", cache.get(3)); // returns 3
    printf("%d\n", cache.get(4)); // returns 4
}