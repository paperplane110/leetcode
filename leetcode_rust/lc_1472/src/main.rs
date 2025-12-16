use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
    let mut browser = BrowserHistroy::new("leetcode.com".to_string());
    browser.visit("google.com".to_string());
    browser.visit("facebook.com".to_string());
    browser.visit("youtube.com".to_string());
    println!("{}", browser.back(3));
    browser.visit("bilibli.com".to_string());
    println!("{}", browser.forward(2));
    println!("{:?}", browser.stack)
}

struct BrowserHistroy {
    stack: Vec<String>,
    ptr: usize,
}

impl BrowserHistroy {
    fn new(homepage: String) -> Self {
        Self {
            stack: vec![homepage],
            ptr: 0,
        }
    }
    fn visit(&mut self, url: String) {
        self.ptr += 1;
        self.stack.truncate(self.ptr);
        self.stack.push(url);
    }
    fn back(&mut self, step: i32) -> String {
        self.ptr = max(0, self.ptr - step as usize);
        self.stack[self.ptr].clone()
    }
    fn forward(&mut self, step: i32) -> String {
        self.ptr = min(self.stack.len() - 1, self.ptr + step as usize);
        self.stack[self.ptr].clone()
    }
}