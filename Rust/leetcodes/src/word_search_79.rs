#![allow(dead_code)]

struct Solution;

impl Solution {
    const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        if m == 0 {
            return false;
        }
        let n = board[0].len();
        let mut visited = vec![vec![false; n]; m];

        let word: Vec<char> = word.chars().collect();
        for i in 0..m {
            for j in 0..n {
                if Self::search((i, j), 0, &mut visited, &board, &word) {
                    // println!("{} {}", i, j);
                    return true;
                }
            }
        }
        false
    }

    fn search(
        pos: (usize, usize),
        i: usize,
        visited: &mut Vec<Vec<bool>>,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
    ) -> bool {
        if visited[pos.0][pos.1] {
            return false;
        }

        if word[i] != board[pos.0][pos.1] {
            return false;
        }

        if i == word.len() - 1 {
            return true;
        }

        visited[pos.0][pos.1] = true;

        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for dire in Self::DIRECTIONS.iter() {
            let x = pos.0 as i32 + dire[0];
            let y = pos.1 as i32 + dire[1];
            if x < 0 || x >= m || y < 0 || y >= n {
                continue;
            }
            let next = (x as usize, y as usize);
            if Self::search(next, i + 1, visited, board, word) {
                return true;
            }
        }

        visited[pos.0][pos.1] = false;
        return false;
    }
}

#[test]
fn word_search_test1() {
    let board: Vec<Vec<char>> = vec![
        "ABCE".chars().collect(),
        "SFCS".chars().collect(),
        "ADEE".chars().collect(),
    ];
    let word = String::from("SEE");
    assert!(Solution::exist(board, word));
}

#[test]
fn word_search_test2() {
    let board: Vec<Vec<char>> = vec![
        "ABCE".chars().collect(),
        "SFCS".chars().collect(),
        "ADEE".chars().collect(),
    ];
    let word = String::from("ABCCED");
    assert!(Solution::exist(board, word));
}

#[test]
fn word_search_test3() {
    let board: Vec<Vec<char>> = vec![
        "ABCE".chars().collect(),
        "SFCS".chars().collect(),
        "ADEE".chars().collect(),
    ];
    let word = String::from("ABCB");
    assert!(!Solution::exist(board, word));
}

#[test]
fn word_search_test4() {
    let board: Vec<Vec<char>> = vec!["a".chars().collect()];
    let word = String::from("a");
    assert!(Solution::exist(board, word));
}
