use std::cmp::min;

fn main() {
    println!("Hello, world!");
}

struct TextEditor {
    left: Vec<char>,
    right: Vec<char>,
}


impl TextEditor {
    fn new() -> Self {
        TextEditor {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
    fn insert(&mut self, s: String) {
        for c in s.chars() {
            self.left.push(c);
        }
    }
    fn delete(&mut self, k: i32) -> i32 {
        let cnt = min(k, self.left.len() as i32);
        for _ in 0..cnt {
            self.left.pop();
        }
        return cnt;
    }
    fn cursor_left(&mut self, k: i32) -> String {
        let cnt = min(k, self.left.len() as i32);
        for _ in 0..cnt {
            if let Some(c) = self.left.pop() {
                self.right.push(c);
            }
        }
        self.get_left_text()
    }
    fn cursor_right(&mut self, k: i32) -> String {
        let cnt = min(k, self.right.len() as i32);
        for _ in 0..cnt {
            if let Some(c) = self.right.pop() {
                self.left.push(c);
            }
        }
        self.get_left_text()
    }
    fn get_left_text(&self) -> String {
        let start = self.left.len().saturating_sub(10);
        self.left[start..].iter().collect()
    }
}