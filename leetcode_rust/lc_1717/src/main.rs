fn main() {
    println!("Hello, world!");
}

fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    // x for ab, y for ba
    // if y > x
    let mut b = 'b';
    let mut a = 'a';
    let mut x = x;
    let mut y = y;
    if x > y {
        b = 'a';
        a = 'b';
        (x, y) = (y, x);
    }
    let mut res = 0;
    let mut cnt_b = 0;
    let mut cnt_a = 0;
    for c in s.chars() {
        if c == b {
            cnt_b += 1;
        } else if c == a {
            if cnt_b > 0 {
                cnt_b -= 1;
                res += y;
            } else {
                cnt_a += 1;
            }
        } else {
            res = res + cnt_a.min(cnt_b) * x;
            cnt_a = 0;
            cnt_b = 0;
        }
    }
    res += cnt_a.min(cnt_b) * x;
    res
}
