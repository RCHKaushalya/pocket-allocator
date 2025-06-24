use std::{io:: {self, Write}, usize};

const HEAP_SIZE: usize = 1024;

#[derive(Clone, Copy, Debug)]
struct Block {
    start: usize,
    size: usize,
    is_free: bool,
}

struct Allocator {
    heap: [u8; HEAP_SIZE],
    blocks: [Block; 32],
}

impl Allocator {
    fn new() -> Self {
        Self {
            heap: [0; HEAP_SIZE],
            blocks: [Block { start: 0, size: 0, is_free: true }; 32],
        }
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        for block in self.blocks.iter_mut() {
            if block.is_free && block.size >= size {
                block.is_free = false;
                println!("Reused {} bytes at offset {}", size, block.start);
                return self.heap.as_mut_ptr().wrapping_add(block.start);
            }
        }

        let mut current_offset = 0;
        for block in self.blocks.iter() {
            if block.size > 0 {
                current_offset = block.start + block.size;
            }
        }

        if current_offset + size > HEAP_SIZE {
            println!("Not enough memory!");
            return std::ptr::null_mut();
        }

        for block in self.blocks.iter_mut() {
            if block.size == 0 {
                *block = Block {
                    start: current_offset,
                    size,
                    is_free: false,
                };
                println!("New allocation: {} bytes at offset {}", size, current_offset);
                return self.heap.as_mut_ptr().wrapping_add(current_offset);
            }
        }

        std::ptr::null_mut()
    }

    fn free(&mut self, ptr: *mut u8) {
        let base_ptr =  self.heap.as_mut_ptr();

        for block in self.blocks.iter_mut() {
            let block_ptr = unsafe {base_ptr.add(block.start)};

            if ptr == block_ptr {
                block.is_free = true;
                println!("freed block at offset {}", block.start);
                return;
            }
        }

        println!("Pointer not found. can not free");
    }

    fn status(&self) {
        println!("\n📦 Allocator status report:");

        let mut used = 0;
        let mut free = 0;

        for block in self.blocks.iter() {
            if block.size == 0 {
                continue;
            }

            if block.is_free {
                println!("🟩 Free block: offset {} size {}", block.start, block.size);
                free += block.size;
            } else {
                println!("🟥 Used block: offset {} size {}", block.start, block.size);
                used += block.size;
            }
        }

        println!("➡️ Total: {} bytes | Used: {} bytes | Free: {} bytes", HEAP_SIZE, used, HEAP_SIZE-used);
    }

    fn visualize(&self) {
        println!("\n🧱 Heap Visualization");

        let mut heap_line = String::new();

        for block in self.blocks.iter() {
            if block.size == 0 {
                continue;
            }

            let symbol = if block.is_free { '🟩' } else { '🟥' };
            for _ in 0..(block.size/8).max(1) {
                heap_line.push(symbol);
            }
        }

        println!("[{}]", heap_line);
    }
}

fn main() {
    let mut allocator = Allocator::new();
    let mut ptrs: Vec<*mut u8> = Vec::new();

    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "alloc" if parts.len() == 2 => {
                if let Ok(size) = parts[1].parse::<usize>() {
                    let ptr = allocator.allocate(size);
                    ptrs.push(ptr);
                }
            }

            "free" if parts.len() == 2 => {
                if let Ok(index) =parts[1].parse::<usize>() {
                    if index < ptrs.len() {
                        allocator.free(ptrs[index]);
                    } else {
                        println!("Invalid pointer index");
                    }
                }
            }

            "status" => {
                allocator.status();
            }

            "vis" | "visualize" => {
                allocator.visualize();
            }

            "exit" => {
                break;
            }

            _ => {
                println!("Commands: alloc <size> | free <index> | status | exit");
            }
        }
    }
}