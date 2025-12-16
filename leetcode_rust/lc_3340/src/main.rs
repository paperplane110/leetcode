fn main() {
    println!("Hello, world!");
    println!("{}", is_balanced("213076"));
    println!("{}", is_balanced("111111"));
}

fn is_balanced(s: &str) -> bool {
    let n: Vec<u32> = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let n_odd_num: u32 = n
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 1)
        .map(|(_, &x)| x)
        .sum();
    let n_even_sum: u32 = n
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, &x)| x)
        .sum();
    n_odd_num == n_even_sum
}
