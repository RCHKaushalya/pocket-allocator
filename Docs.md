# 📘 Memory Allocator Documentation

This document provides a deeper look into the internals of the memory allocator CLI project, highlighting its architecture, mechanics, and learning points.

---

## 🧠 Project Overview

This Rust-based memory allocator simulates how low-level systems (like OS kernels or embedded runtimes) manage heap memory manually.

It uses a contiguous byte array (`heap`) and dynamically manages block metadata (`Vec<Block>`) to implement:
- Heap allocation (`alloc`)
- Deallocation (`free`)
- Defragmentation through adjacent block merging
- Memory visualization via CLI

---

## 🏗️ Architecture

### Block Structure

Each memory region is tracked using a `Block` struct:

```rust
struct Block {
    start: usize,      // Offset within the heap
    size: usize,       // Block size in bytes
    is_free: bool,     // Availability flag
}
```

### Allocator Core

```rust
struct Allocator {
    heap: [u8; HEAP_SIZE],  // Simulated memory buffer
    blocks: Vec<Block>,     // Dynamic block list
}
```

---

## ⚙️ Core Components

### allocate(size)

- Reuses any available free block (first-fit).
- Else appends a new block to the heap.
- Returns a pointer to the allocated region using pointer math:
  ```rust
  self.heap.as_mut_ptr().wrapping_add(offset)
  ```

### free(ptr)

- Matches the pointer to its corresponding block using offset math.
- Marks the block as free.
- Calls `merge_free_blocks()` to merge neighbors.

### merge_free_blocks()

- Coalesces adjacent free blocks by checking their physical contiguity (based on `start` + `size`).
- Shrinks the block list by removing merged entries.

### visualize()

- Prints an ASCII bar of the heap:
  - 🟥 = Used block
  - 🟩 = Free block
- Uses proportional scaling: 1 symbol = ~8 bytes.

### status()

- Lists all current blocks with offset, size, and whether they are used or free.
- Also prints total used and free heap space.

---

## 🪄 CLI Commands

| Command          | Description                              |
|------------------|------------------------------------------|
| `alloc <size>`   | Allocates a new block of memory           |
| `free <index>`   | Frees the pointer stored at `ptr[index]` |
| `status`         | Displays a memory usage report            |
| `visualize`      | Shows block layout as an emoji bar        |
| `help`           | Lists supported commands                  |
| `exit`           | Terminates the REPL                       |

---

## 💡 Design Notes

- First-fit allocation strategy
- No real alignment or padding enforced
- Pointer management simulated using indices like `ptr[0]`, `ptr[1]`
- Fragmentation reduced via real-time merging of adjacent free blocks
- Zero unsafe dereferencing—just pointer math

---

## 🧠 Learning Objectives

This allocator helped build intuition around:
- Manual memory handling and fragmentation patterns
- The role of metadata in heap design
- Designing command-line tools with internal state
- Rust fundamentals: slices, vectors, pointer math, enums
- Debugging tools and visualization using custom UI

---

## 🔮 Potential Improvements

- Use `Option<Block>` for slot reuse in freed entries
- Add tests for merging and boundary conditions
- Add `realloc()` or `defrag()` command
- Make visualizer show offsets and labels
- Abstract into a library (`lib.rs`) for reuse

---

## 👤 Author

**Rasindu** — learning memory management the hard (and fun) way 🧵