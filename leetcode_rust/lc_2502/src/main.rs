use std::vec;

fn main() {
    println!("Hello, world!");
    let mut allocator = Allocator::new(10);
    allocator.allocate(2, 10);
    allocator.allocate(1, 10);
    allocator.free_memory(10);
    allocator.allocate(5, 1);
    print!("{:?}", allocator.memory)
}

struct Allocator {
    n: usize,
    memory: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {

    fn new(n: i32) -> Self {
        Allocator {
            n: n as usize,
            memory: vec![0; n as usize],
        }
    }
    
    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut count = 0;
        for i in 0..self.n {
            if self.memory[i] == 0 {
                count += 1;
                if count == size {
                    for j in (i as i32 - size + 1)..=i as i32 {
                        self.memory[j as usize] = m_id;
                    }
                    return i as i32 - count + 1;
                }
            } else {
                count = 0;
            }
        }
        -1
    }
    
    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut count = 0;
        for i in 0..self.n {
            if self.memory[i] == m_id {
                count += 1;
                self.memory[i] = 0;
            }
        }
        count as _
    }
}
