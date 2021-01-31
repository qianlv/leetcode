/*
 * 先把 word 中的单词，于处理插入到 trie 树中，
 * 然后 dfs 搜索 board，递归的同时，要有延着 trie 树遍历。
 */
#![allow(dead_code)]

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<WordDictionaryNode>>;
type NodeType = Option<Node>;

struct WordDictionaryNode {
    word: String,
    childs: RefCell<Vec<NodeType>>,
}

impl WordDictionaryNode {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WordDictionaryNode {
            word: String::from(""),
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
            root: Some(WordDictionaryNode::new()),
        }
    }

    fn add_word(&self, word: String) {
        let mut curr = Rc::clone(&(self.root.as_ref().unwrap()));
        for ch in word.bytes() {
            let index = (ch - b'a') as usize;
            let next = curr.borrow().childs.borrow_mut()[index]
                .get_or_insert(WordDictionaryNode::new())
                .clone();
            curr = next;
        }
        curr.borrow_mut().word = word;
    }

    fn get_root(&self) -> Node {
        return Rc::clone(&(self.root.as_ref().unwrap()));
    }
}

impl Solution {
    const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let tries = WordDictionary::new();
        for word in words {
            tries.add_word(word);
        }
        let m = board.len();
        let n = board[0].len();

        let mut visited = vec![vec![false; n]; m];
        let mut results: Vec<String> = vec![];
        let root = tries.get_root();
        for i in 0..m {
            for j in 0..n {
                visited[i][j] = true;
                let index = (board[i][j] as u32 - 'a' as u32) as usize;
                let next = root.borrow().childs.borrow()[index].as_ref().cloned();
                if let Some(next) = next {
                    Self::find_words_helper(&board, &mut results, &mut visited, i, j, next);
                }
                visited[i][j] = false;
            }
        }
        results
    }

    fn find_words_helper(
        board: &Vec<Vec<char>>,
        results: &mut Vec<String>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        root: Node,
    ) {
        if !root.borrow().word.is_empty() {
            results.push(root.borrow().word.clone());
            root.borrow_mut().word.clear();
        }

        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for dire in Self::DIRECTIONS.iter() {
            let x = i as i32 + dire[0];
            let y = j as i32 + dire[1];

            if x < 0 || x >= m || y < 0 || y >= n {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;

            let ch = board[x][y];
            let index = (ch as u32 - 'a' as u32) as usize;

            let next = root.borrow().childs.borrow()[index].as_ref().cloned();
            if let Some(next) = next {
                Self::find_words_helper(board, results, visited, x, y, next);
            }

            visited[x][y] = false;
        }
    }
}

#[test]
fn find_words_test1() {
    let board: Vec<Vec<char>> = vec![
        "oaan".chars().collect(),
        "etae".chars().collect(),
        "ihkr".chars().collect(),
        "iflv".chars().collect(),
    ];

    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];

    let mut results = Solution::find_words(board, words);
    results.sort();
    assert_eq!(results, vec!["eat".to_string(), "oath".to_string()]);
    // assert_eq!(results, vec!["oath".to_string(), "eat".to_string()]);
}

#[test]
fn find_words_test2() {
    let board: Vec<Vec<char>> = vec!["a".chars().collect()];

    let words = vec!["a".to_string()];

    let mut results = Solution::find_words(board, words);
    results.sort();
    assert_eq!(results, vec!["a".to_string()]);
    // assert_eq!(results, vec!["oath".to_string(), "eat".to_string()]);
}
