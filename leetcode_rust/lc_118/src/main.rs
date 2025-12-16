fn main() {
    println!("Hello, world!");
}

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::<Vec<i32>>::new();
    for row in 0..num_rows {
        if row == 0 {
            ans.push(vec![1]);
        } else if row == 1 {
            ans.push(vec![1,1]);
        } else {
            let mut tmp = Vec::<i32>::new();
            for i in 0..=row {
                if i == 0 || i == row {
                    tmp.push(1);
                } else {
                    tmp.push(ans[(row - 1) as usize][(i - 1) as usize] + ans[(row - 1) as usize][i as usize]);
                }
            }
            ans.push(tmp);
        }
    }
    ans
}

fn generate2(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();

    for row in 0..num_rows {
        let row_usize = row as usize;

        match row {
            0 => ans.push(vec![1]),
            _ => {
                let mut current_row = Vec::with_capacity(row_usize + 1);
                current_row.push(1);

                for i in 1..row_usize {
                    let prev_row = &ans[row_usize - 1];
                    current_row.push(prev_row[i-1] + prev_row[i]);
                }

                current_row.push(1);
                ans.push(current_row);
            }
        }
    }
    ans
}