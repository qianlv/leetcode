#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut counter = 0;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                counter = 0;
                if i >= 1 {
                    counter += board[i - 1][j] % 2;
                    if j >= 1 {
                        counter += board[i - 1][j - 1] % 2;
                    }

                    if j + 1 < board[i].len() {
                        counter += board[i - 1][j + 1] % 2;
                    }
                }
                if i + 1 < board.len() {
                    counter += board[i + 1][j] % 2;
                    if j >= 1 {
                        counter += board[i + 1][j - 1] % 2;
                    }

                    if j + 1 < board[i].len() {
                        counter += board[i + 1][j + 1] % 2;
                    }
                }

                if j >= 1 {
                    counter += board[i][j - 1] % 2;
                }
                if j + 1 < board[i].len() {
                    counter += board[i][j + 1] % 2;
                }
                if board[i][j] == 1 {
                    if counter == 2 || counter == 3 {
                        board[i][j] += 2;
                    }
                } else {
                    if counter == 3 {
                        board[i][j] += 2;
                    }
                }
            }
        }
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                board[i][j] /= 2;
            }
        }
    }
}
