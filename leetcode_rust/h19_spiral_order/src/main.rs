fn main() {
    println!("Hello, world!");
    let test = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
    let ans = spiral_order(test);
    println!("{:?}", ans);
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let row_num = matrix.len();
    let col_num = matrix[0].len();
    let total_num = row_num * col_num;
    let mut ans: Vec<i32> = Vec::new();
    //  [left_top, right_top, right_bottom, left_bottom]
    let mut visit_cnt = [1, 0, 0, 0];
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut direction = "right";
    while ans.len() < total_num {
        if direction == "right" {
            while col < col_num - visit_cnt[1] {
                ans.push(matrix[row][col]);
                col += 1;
            }
            col -= 1;
            row += 1;
            visit_cnt[1] += 1;
            direction = "down";
        } else if direction == "down" {
            while row < row_num - visit_cnt[2] {
                ans.push(matrix[row][col]);
                row += 1;
            }
            row -= 1;
            col -= 1;
            visit_cnt[2] += 1;
            direction = "left";
        } else if direction == "left" {
            while col > visit_cnt[3] {
                ans.push(matrix[row][col]);
                col -= 1;
            }
            ans.push(matrix[row][col]);
            row -= 1;
            visit_cnt[3] += 1;
            direction = "up"
        } else {
            while row > visit_cnt[0] {
                ans.push(matrix[row][col]);
                row -= 1;
            }
            ans.push(matrix[row][col]);
            col += 1;
            visit_cnt[0] += 1;
            direction = "right";
        }
    }
    ans
}
