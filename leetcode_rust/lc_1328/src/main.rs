fn main() {
    println!("Hello, world!");
    println!("{}", break_palindorme("abccba".to_string()));
    println!("{}", break_palindorme("a".to_string()));
    println!("{}", break_palindorme("aba".to_string()));
}

fn break_palindorme(palindrome: String) -> String {
    let mut palindrome = palindrome.chars().collect::<Vec<char>>();
    let len = palindrome.len();
    if len == 1 {
        return "".to_string();
    }

    let mut mid = 0;
    if len % 2 == 1 {
        mid = len / 2;
    }

    for i in 0..len {
        if mid > 0 && i == mid {
            continue;
        }
        if 'a' < palindrome[i] {
            palindrome[i] = 'a';
            return String::from_iter(palindrome);
        }
    }
    palindrome[len - 1] = 'b';
    return String::from_iter(palindrome);
}