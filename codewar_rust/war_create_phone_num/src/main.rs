fn main() {
    println!("Hello, world!");
}

fn create_phone_number_me(numbers: &[u8]) -> String {
    let mut res = String::new();
    for (i, n_ref) in numbers.iter().enumerate() {
        if i == 0 {
            res.push('(');
        }
        if i == 3 {
            res.push(')');
            res.push(' ');
        }
        if i == 6 {
            res.push('-');
        }
        res.push(char::from_digit(*n_ref as u32, 10).unwrap());
    }
    return res;
}

fn create_phone_number(numbers: &[u8]) -> String {
    let s = numbers.iter().map(|i| i.to_string()).collect::<Vec<String>>();

    format!("({}) {}-{}", &s[0..3].join(""), &s[3..6].join(""), &s[6..].join(""))
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
