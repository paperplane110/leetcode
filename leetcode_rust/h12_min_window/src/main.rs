fn main() {
    println!("Hello, world!");
}

fn min_window(s: String, t: String) -> String {
    fn check(s_cnt: &[i32; 128], t_cnt: &[i32; 128]) -> bool {
        for i in b'a'..=b'z' {
            if s_cnt[i as usize] < t_cnt[i as usize] {
                return false;
            }
        }
        for i in b'A'..=b'Z' {
            if s_cnt[i as usize] < t_cnt[i as usize] {
                return false;
            }
        }
        true
    }

    let mut s_cnt = [0; 128];
    let mut t_cnt = [0; 128];

    for byte in t.bytes() {
        t_cnt[byte as usize] += 1;
    }

    let mut ans_l = 0;
    let mut ans_r = s.len();
    let mut l = 0;
    let s_arr = s.as_bytes();
    for (r, &c) in s_arr.iter().enumerate() {
        s_cnt[c as usize] += 1;
        while check(&s_cnt, &t_cnt) {
            if (r - l) < (ans_r - ans_l) {
                [ans_l, ans_r] = [l, r];
            }
            s_cnt[s_arr[l] as usize] -= 1;
            l += 1;
        }
    }
    if ans_r == s.len() {
        return String::new();
    } else {
        s[ans_l..=ans_r].to_string()
    }
}