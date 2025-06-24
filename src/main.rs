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
    blocks: Vec<Block>,
}

impl Allocator {
    fn new() -> Self {
        Self {
            heap: [0; HEAP_SIZE],
            blocks: Vec::new(),
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

        let ptr = self.heap.as_mut_ptr().wrapping_add(current_offset);
        self.blocks.push(Block {
            start: current_offset,
            size,
            is_free: false,
        });

        println!("New allocation: {} bytes at offset {}", size, current_offset);
        ptr
    }


    fn free(&mut self, ptr: *mut u8) {
        let base_ptr =  self.heap.as_mut_ptr();

        for block in self.blocks.iter_mut() {
            let block_ptr = unsafe {base_ptr.add(block.start)};

            if ptr == block_ptr {
                block.is_free = true;
                println!("freed block at offset {}", block.start);
                self.merge_free_blocks();
                return;
            }
        }

        println!("Pointer not found. can not free");
    }

    fn status(&self) {
        println!("\n📦 Allocator status report:");

        let mut used = 0;
        let mut _free = 0;

        for block in self.blocks.iter() {
            if block.size == 0 {
                continue;
            }

            if block.is_free {
                println!("🟩 Free block: offset {} size {}", block.start, block.size);
                _free += block.size;
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

    fn merge_free_blocks(&mut self) {
        let mut i = 0;
        while i+1 < self.blocks.len() {
            let current = &self.blocks[i];
            let next = &self.blocks[i+1];

            if current.is_free && next.is_free && current.start+current.size == next.start {
                let new_size = current.size+next.size;
                self.blocks[i].size = new_size;

                self.blocks.remove(i + 1);
                continue;
            }

            i += 1;
        }
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
                    let index =  ptrs.len();
                    ptrs.push(ptr);
                    println!("-> Stored as ptr[{}]", index);
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

            "help" => {
                println!("\n📖 Available commands:");
                println!("  alloc <size>     → Allocate memory (e.g. alloc 64)");
                println!("  free <index>     → Free pointer by index (e.g. free 0)");
                println!("  status           → Show allocator status report");
                println!("  visualize        → Show memory layout as an ASCII bar");
                println!("  exit             → Exit the allocator CLI");
                println!("  help             → Show this command guide");
            }

            _ => {
                println!("Unknown command. Type 'help' for a list of supported commands.");
            }
        }
    }
}