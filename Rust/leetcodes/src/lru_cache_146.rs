#![allow(dead_code)]

use std::collections::HashMap;

struct LRUCache {
    capacity: i32,
    list: List,
    map: HashMap<i32, LinkPtr>,
}

type Link = Option<Box<Node>>;
type LinkPtr = *mut Node;

struct Node {
    key: i32,
    value: i32,
    next: Link,
    prev: LinkPtr,
}

impl Node {
    fn new(key: i32, value: i32) -> Node {
        Node {
            key,
            value,
            next: None,
            prev: std::ptr::null_mut(),
        }
    }
}

struct List {
    head: Link,
    tail: LinkPtr,
    size: i32,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    fn push_front(&mut self, key: i32, value: i32) {
        let mut new_head = Box::new(Node::new(key, value));
        let raw_head: LinkPtr = &mut *new_head;

        match self.head.take() {
            Some(mut old_head) => {
                old_head.as_mut().prev = raw_head;
                new_head.as_mut().next = Some(old_head);
            }
            None => {
                self.tail = raw_head;
            }
        }

        self.head = Some(new_head);
        self.size += 1;
    }

    fn pop_back(&mut self) {
        if !self.tail.is_null() {
            let prev: LinkPtr = unsafe { (*self.tail).prev };
            if prev.is_null() {
                self.head.take();
            } else {
                unsafe {
                    (*prev).next.take();
                }
            }
            self.size -= 1;
        }
    }

    fn erase(&mut self, node: LinkPtr) {
        if node.is_null() {
            return;
        }

        let prev: LinkPtr = unsafe { (*node).prev };
        let next = unsafe { (*node).next.take() };
        match (next, prev.is_null()) {
            (Some(next), true) => {
                self.head = unsafe {
                    (*node).prev = std::ptr::null_mut();
                    Some(next)
                };
            }
            (Some(mut next), false) => unsafe {
                next.as_mut().prev = prev;
                (*prev).next = Some(next);
            },
            (None, true) => {
                self.head = None;
                self.tail = std::ptr::null_mut();
            }
            (None, false) => {
                unsafe {
                    (*prev).next = None;
                }
                self.tail = prev;
            }
        }
        self.size -= 1;
    }

    fn head_ptr(&mut self) -> LinkPtr {
        self.head
            .as_mut()
            .map_or(std::ptr::null_mut(), |head| &mut **head)
    }

    fn back(&mut self) -> Option<(i32, i32)> {
        if self.tail.is_null() {
            return None;
        }

        unsafe { Some(((*self.tail).key, (*self.tail).value)) }
    }

    fn size(&self) -> i32 {
        self.size
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            list: List::new(),
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.list.size() <= 0 {
            return -1;
        }
        let ret = self.erase(key);
        if ret != -1 {
            self.insert(key, ret);
        }
        ret
    }

    fn erase(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(node) => {
                let ret = unsafe { (**node).value };
                self.list.erase(*node);
                self.map.remove(&key);
                ret
            }
            None => -1,
        }
    }

    fn insert(&mut self, key: i32, value: i32) {
        self.list.push_front(key, value);
        self.map.insert(key, self.list.head_ptr());
    }

    fn evict(&mut self) {
        if self.list.size() >= self.capacity {
            let kv = self.list.back();
            if let Some((k, _)) = kv {
                if let Some(node) = self.map.get(&k) {
                    self.list.erase(*node);
                    self.map.remove(&k);
                }
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.erase(key);
        self.evict();
        self.insert(key, value);
    }
}

//
// Your LRUCache object will be instantiated and called as such:
// let obj = LRUCache::new(capacity);
// let ret_1: i32 = obj.get(key);
// obj.put(key, value);
//

#[test]
fn lru_cache_test1() {
    let mut obj = LRUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    assert_eq!(obj.get(1), 1);
    obj.put(3, 3);
    assert_eq!(obj.get(2), -1);
    obj.put(4, 4);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), 4);
}

#[test]
fn lru_cache_test2() {
    let mut obj = LRUCache::new(2);
    assert_eq!(obj.get(2), -1);
    obj.put(2, 6);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(2), 6);
    obj.put(1, 5);
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 6);
}
