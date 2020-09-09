import java.util.HashMap;
import java.util.Map;

public class LRUCache {

    static class LRUNode {

        final int key;
        int val;
        LRUNode prev;
        LRUNode next;

        LRUNode(int key, int val) {
            this.key = key;
            this.val = val;
        }
    }

    final int capacity;
    final Map<Integer, LRUNode> cache;

    // use dummy head and tail to avoid countless null checks
    final LRUNode dummyHead;
    final LRUNode dummyTail;

    int size;

    public LRUCache(int capacity) {
        if (capacity <= 0) {
            throw new IllegalArgumentException("capacity");
        }
        this.capacity = capacity;
        cache = new HashMap<>();
        dummyHead = new LRUNode(-1, -1);
        dummyTail = new LRUNode(-1, -1);
        dummyHead.next = dummyTail;
        dummyTail.prev = dummyHead;
        size = 0;
    }

    public int get(int key) {
        LRUNode node = cache.get(key);
        if (node == null) {
            return -1;
        }
        unlinkNode(node);
        insertNode(node);
        return node.val;
    }

    void unlinkNode(LRUNode node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    void insertNode(LRUNode node) {
        node.next = dummyHead.next;
        node.prev = dummyHead;
        dummyHead.next.prev = node;
        dummyHead.next = node;
    }

    public void put(int key, int value) {
        LRUNode node = cache.get(key);
        if (node != null) {
            node.val = value;
            // "put" is also recently used
            unlinkNode(node);
            insertNode(node);
            return;
        }
        if (size == capacity) {
            LRUNode lastNode = dummyTail.prev;
            cache.remove(lastNode.key);
            unlinkNode(lastNode);
            size -= 1;
        }
        node = new LRUNode(key, value);
        cache.put(key, node);
        insertNode(node);
        size += 1;
    }
}
