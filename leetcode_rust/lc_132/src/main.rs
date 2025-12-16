fn main() {
    println!("Hello, world!");
}

fn min_cut(s: String) -> i32 {
    
}

fn is_palindrome(s: String) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        if s.chars().nth(left) != s.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
