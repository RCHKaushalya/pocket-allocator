# 🧠 Rust Memory Allocator CLI

A minimal heap memory allocator written in Rust, built for learning systems programming concepts like allocation, fragmentation, and memory reuse.

## ✨ Features

- `alloc <size>`: allocate a block of memory
- `free <index>`: free an allocated block by its index (e.g., `free 0`)
- `status`: show used and free memory blocks
- `visualize`: ASCII bar that shows memory layout
- Auto-merging of adjacent free blocks
- CLI-style interactive REPL
- Pointer labels like `ptr[0]`, `ptr[1]`
- `help`: quick reference to commands

---

## 🚀 Getting Started

### Requirements
- Rust (latest stable)

### Run it:
```bash
cargo run

Then enter commands like:
> alloc 64
> free 0
> status
> visualize
> exit

📚 What I Learned
This project helped me:

Understand how low-level memory allocators work

Simulate block merging and heap layouts

Build a terminal-based tool with labeled pointer tracking

Visualize memory fragmentation using emojis 😄

🔮 Future Ideas
Auto-shrink memory map

Stress test with random allocation patterns

File logging for status snapshots

Convert into a bootable kernel
