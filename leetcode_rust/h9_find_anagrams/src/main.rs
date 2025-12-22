fn main() {
    println!("Hello, world!");
}

fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut cnt_p = vec![0; 26];
    for c in p.bytes() {
        cnt_p[(c - b'a') as usize] += 1;
    }

    let s = s.as_bytes();
    let mut ans = vec![];
    let mut cnt_s = vec![0; 26];
    for (right, &c) in s.iter().enumerate() {
        cnt_s[(c - b'a') as usize] += 1;

        if right < p.len() - 1 {
            continue;
        }
        let left = right - p.len() + 1;

        if cnt_p == cnt_s {
            ans.push(left as i32);
        }

        cnt_s[(s[left] - b'a') as usize] -= 1;
    }
    ans
}

fn find_anagrams_1(s: String, p: String) -> Vec<i32> {
    let ns = s.len();
    let np = p.len();

    let mut ans = Vec::new();
    let mut cnt_s = [0; 26];
    let mut cnt_p = [0; 26];

    for b in p.bytes() {
        cnt_p[(b - b'a') as usize] += 1;
    }
    let s = s.as_bytes();

    for right in 0..ns {
        // into a window
        cnt_s[(s[right] - b'a') as usize] += 1;

        // window width reached 'p' width
        if right >= np - 1 {
            let left = right - np + 1;

            if cnt_s == cnt_p {
                ans.push(left as i32);
            }

            // leave window
            cnt_s[(s[left] - b'a') as usize] -= 1;
        }
    }
    ans
}