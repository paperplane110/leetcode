fn main() {
    let word = String::from("aab");
    println!("{}", possible_string_count(word));
    println!("{}", possible_string_count(String::from("aaaa")));
    println!("{}", possible_string_count(String::from("abbcccc")));
}

fn possible_string_count(word: String) -> i32 {
    let mut res = 0;
    let mut r_cnt = 0;
    let mut f_cnt = 0;
    let word_iter = word.chars();
    let mut current_char = word.chars().next().unwrap();
    for c in word_iter {
        if c == current_char {
            // same letter
            if r_cnt == 1 {
                // already typed, means this time it is an error
                f_cnt += 1;
            }
            r_cnt += 1;
        } else {
            // new letter
            if r_cnt > 1 {
                res += r_cnt;
            }
            // reset r_cnt and current char
            r_cnt = 1;
            current_char = c;
        }
    }
    if r_cnt > 1 {
        res += r_cnt;
    }
    res + 1 - f_cnt
}
