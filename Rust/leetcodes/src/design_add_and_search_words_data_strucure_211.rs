#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<WordDictionaryNode>>;
type NodeType = Option<Node>;

struct WordDictionaryNode {
    is_word: bool,
    childs: RefCell<Vec<NodeType>>,
}

impl WordDictionaryNode {
    fn new(is_word: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WordDictionaryNode {
            is_word,
            childs: RefCell::new(vec![None; 26]),
        }))
    }
}

struct WordDictionary {
    root: NodeType,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        WordDictionary {
            root: Some(WordDictionaryNode::new(false)),
        }
    }

    fn add_word(&self, word: String) {
        let mut curr = Rc::clone(&(self.root.as_ref().unwrap()));
        for ch in word.bytes() {
            let index = (ch - b'a') as usize;
            let next = curr.borrow().childs.borrow_mut()[index]
                .get_or_insert(WordDictionaryNode::new(false))
                .clone();
            curr = next;
        }
        curr.borrow_mut().is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let curr = Rc::clone(&(self.root.as_ref().unwrap()));
        return Self::search_helper(curr, word.as_bytes());
    }

    fn search_helper(mut root: Node, word: &[u8]) -> bool {
        for (i, ch) in word.iter().enumerate() {
            if ch == &b'.' {
                for index in 0..26usize {
                    let next = root.borrow().childs.borrow()[index]
                        .as_ref()
                        .map(|x| x.clone());
                    if let Some(next) = next {
                        if Self::search_helper(next, &word[i + 1..]) {
                            return true;
                        }
                    } else {
                        continue;
                    }
                }
                return false;
            } else {
                let index = (ch - b'a') as usize;
                let next = root.borrow().childs.borrow()[index]
                    .as_ref()
                    .map(|x| x.clone());
                if let Some(next) = next {
                    root = next;
                } else {
                    return false;
                }
            }
        }
        return root.borrow().is_word;
    }
}

//
// Your WordDictionary object will be instantiated and called as such:
// let obj = WordDictionary::new();
// obj.add_word(word);
// let ret_2: bool = obj.search(word);
//

#[test]
fn word_dictionary_test1() {
    let obj = WordDictionary::new();
    obj.add_word("abc".to_string());
    assert_eq!(obj.search("abc".to_string()), true);
    assert_eq!(obj.search("ab".to_string()), false);
    assert_eq!(obj.search("b".to_string()), false);
    assert_eq!(obj.search("abcef".to_string()), false);
}

#[test]
fn word_dictionary_test2() {
    let obj = WordDictionary::new();
    obj.add_word("abc".to_string());
    assert_eq!(obj.search(".bc".to_string()), true);
    assert_eq!(obj.search("..c".to_string()), true);
    assert_eq!(obj.search("...".to_string()), true);
    assert_eq!(obj.search("..".to_string()), false);
}

#[test]
fn word_dictionary_test3() {
    let obj = WordDictionary::new();
    obj.add_word("bad".to_string());
    obj.add_word("dad".to_string());
    obj.add_word("mad".to_string());
    assert_eq!(obj.search("pad".to_string()), false);
    assert_eq!(obj.search("bad".to_string()), true);
    assert_eq!(obj.search(".ad".to_string()), true);
    assert_eq!(obj.search("b..".to_string()), true);
}

#[test]
fn word_dictionary_test4() {
    let obj = WordDictionary::new();
    obj.add_word("ab".to_string());
    obj.add_word("a".to_string());

    assert_eq!(obj.search("ab".to_string()), true);
    assert_eq!(obj.search("a".to_string()), true);
}
