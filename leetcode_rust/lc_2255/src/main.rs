fn main() {
    println!("Hello, world!");
    let words = vec!["a".to_string(), "b".to_string(), "c".to_string(), "ab".to_string(), "bc".to_string(), "abc".to_string()];
    let s = "abc".to_string();
    let result = count_prefixes(words, s);
    println!("{}", result);
}

fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    let mut count = 0;
    for word in words {
        if s.starts_with(&word) {
            count += 1;
        }
    }
    count
}