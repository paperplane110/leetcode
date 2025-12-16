fn main() {
    println!("Hello, world!");
}

fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        return false;
    }
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut has_vowel = false;
    let mut has_consonant = false;
    let invalid_char = vec!['@', '$', '#'];
    for c in word.to_lowercase().chars() {
        if invalid_char.contains(&c) {
            return false;
        }
        if c.is_digit(10) {
            continue;
        }
        if vowels.contains(&c) {
            has_vowel = true;
        } else {
            has_consonant = true;
        }
    }
    return has_vowel && has_consonant;
}