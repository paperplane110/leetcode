use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let char_list: Vec<char> = s.chars().collect();
    let mut start: usize = 0;
    let mut char2idx: HashMap<char, usize> = HashMap::new();
    char2idx.insert(char_list[start], start);
    let mut temp = 1;
    let mut max = 0;

    for end in 1..char_list.len() {
        let char = char_list[end];
        if let Some(&idx) = char2idx.get(&char)
            && idx >= start
        {
            // 存在，且在窗口内
            max = max.max(temp);
            start = idx + 1;
            temp = (end - start + 1) as i32;
        } else {
            // 不存在，或在窗口外
            temp += 1;
        }
        char2idx.insert(char, end);
    }
    max = max.max(temp);
    max
}

fn length_of_longest_substring_1(s: String) -> i32 {
    let mut char2Idx: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    let mut max_len = 0;
    for (end, ch) in s.chars().enumerate() {
        if let Some(&pre_idx) = char2Idx.get(&ch) {
            if pre_idx >= start {
                start = pre_idx + 1;
            }
        }
        char2Idx.insert(ch, end);
        max_len = max_len.max(end - start + 1);
    }
    max_len as i32
}

fn length_of_longest_substring_2(s: String) -> i32 {
    let mut code2pos = [0usize; 128];
    let mut start = 0;
    let mut max_len = 0;
    for (end, ch) in s.bytes().enumerate() {
        let code = ch as usize;
        if code2pos[code] > start {
            start = code2pos[code];
        }
        code2pos[code] = end + 1;
        max_len = max_len.max(end - start + 1);
    }
    max_len as i32
}