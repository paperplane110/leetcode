fn main() {
    println!("Hello, world!");
}

fn rotate(matrix: &mut Vec<Vec<i32>>) {
    if matrix.len() != matrix[0].len() {
        return;
    }

    let n = matrix.len();
    for row in 0..n {
        for col in 0..row {
            let tmp = matrix[row][col];
            matrix[row][col] = matrix[col][row];
            matrix[col][row] = tmp;
        }
    }

    for row in matrix.iter_mut() {
        row.reverse();
    }
}