fn main() {
    println!("Hello, world!");
    assert!(divisor_substrings(430043, 2) == 2)
}

fn divisor_substrings(num: i32, k: i32) -> i32 {
    let num_str = num.to_string();
    let mut count = 0;
    for i in 0..(num_str.len() -k as usize + 1) {
        let sub_int: i32 = num_str[i..i+k as usize]
            .parse()
            .map_err(|e| format!("Parse error at position {}: {}", i, e))
            .expect("invalid number");
        if sub_int != 0 && num % sub_int == 0 {
            count += 1;
        }
    }
    count
}