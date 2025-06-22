const HEAP_SIZE: usize = 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[derive(Clone, Copy, Debug)]
struct Block {
    start: usize,
    size: usize,
    is_free: bool,
}

static mut BLOCKS: [Block; 32] = [Block {start: 0, size: 0, is_free: true}; 32];

fn main() {
    println!("Hello, world!");
}
