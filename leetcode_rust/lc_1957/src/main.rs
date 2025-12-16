
fn main() {
    println!("Hello, world!");
}

fn make_fancy_string(s: String) -> String {
    let mut res = String::new();
    let mut cnt = 0;
    let mut current = ' ';
    for c in s.chars() {
        if c == current {
            cnt += 1;
            if cnt >= 3 {
                continue;
            } else {
                res.push(c);
            }
        } else {
            current = c;
            cnt = 1;
            res.push(c)
        }
    }
    res
}