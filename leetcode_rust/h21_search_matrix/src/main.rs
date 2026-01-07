use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut row: usize = 0;
    let mut col = n - 1;
    while row < m && col < n {
        match matrix[row][col].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => row += 1,
            _ => col -= 1
        }
    }
    false
}