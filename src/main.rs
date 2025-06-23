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
}

fn main() {
    let mut allocator = Allocator::new();

    let ptr1 = allocator.allocate(64);
    let ptr2 = allocator.allocate(128);
    println!("Pointers: {:?} {:?}", ptr1, ptr2);
}
  