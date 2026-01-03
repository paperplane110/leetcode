use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn set_zero(matrix: &mut Vec<Vec<i32>>) {
    let row_num = matrix.len();
    let col_num = matrix[0].len();
    let mut zero_row_set: HashSet<usize> = HashSet::new();
    let mut zero_col_set: HashSet<usize> = HashSet::new();
    for row in 0..row_num {
        for col in 0..col_num {
            if matrix[row][col] == 0 {
                zero_row_set.insert(row);
                zero_col_set.insert(col);
            }
        }
    }
    for row in 0..row_num {
        for col in 0..col_num {
            if zero_row_set.contains(&row) || zero_col_set.contains(&col) {
                matrix[row][col] = 0;
            }
        }
    }
}
