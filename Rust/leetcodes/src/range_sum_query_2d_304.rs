#![allow(dead_code)]

struct NumMatrix {
    sum_matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let zeros = vec![0; matrix[0].len() + 1];
        let mut sum_matrix: Vec<Vec<i32>> = vec![zeros.clone(); matrix.len() + 1];
        for (i, row) in matrix.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                sum_matrix[i + 1][j + 1] =
                    sum_matrix[i][j + 1] + sum_matrix[i + 1][j] - sum_matrix[i][j] + x;
            }
        }
        NumMatrix { sum_matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        return self.sum_matrix[row2 + 1][col2 + 1]
            - self.sum_matrix[row1][col2 + 1]
            - self.sum_matrix[row2 + 1][col1]
            + self.sum_matrix[row1][col1];
    }
}

#[test]
fn range_sum_query_2d_304_test() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];

    let obj = NumMatrix::new(matrix);
    assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
    assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
    assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
}
