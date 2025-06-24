What is a Memory Allocator?
A memory allocator is responsible for handing out chunks of memory when a program requests it (via functions like malloc() in C), and reclaiming that memory when it’s no longer needed (free()). In an operating system, this behavior must be custom-implemented—there's no global memory manager available in kernel mode.

🦀 Rust Foundation
Rust normally protects you from unsafe memory handling using its ownership and borrowing system. But in systems-level programming, sometimes manual memory handling is required. That’s where unsafe, raw pointers, and manual layout come into play.

Key Rust concepts used:

*mut u8: A raw mutable pointer to a byte, used to simulate memory addresses.

.as_mut_ptr().add(offset): A way to get a pointer to a specific position in a byte array.

unsafe {}: Required when doing operations that the compiler can’t guarantee are memory-safe.

struct: Used to represent metadata about memory blocks (Block { start, size, is_free })

impl + self: We encapsulate everything in an Allocator struct for clean design.

🧠 What We've Built Conceptually
A simulated heap using [u8; 1024]

A block list that tracks allocated and free regions

A allocate() function that:

Looks for reusable blocks

Allocates new regions if needed

Returns raw pointers to available memory

A free() function that:

Marks a pointer's associated block as reusable

🔐 OS Relevance
This simulates how an OS manages memory:

Tracking physical memory layout

Handling fragmentation

Performing allocation without malloc

Building the very system that higher-level allocators (and programs) depend on.

By learning this in Rust, you’re getting a taste of C-level systems code, but with better compile-time guarantees and safer encapsulation.