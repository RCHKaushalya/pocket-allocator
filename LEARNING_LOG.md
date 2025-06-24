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

Memory Reporting
> This mimics how real OS tools like top, htop, or even the kernel itself track memory use. I learned how to read my allocator’s internal block list to debug usage and fragmentation. I’m thinking like a system now—inspecting memory layout is key to efficient low-level programming.

> In Rust, new() is a widely-used naming convention for constructor functions, but it's not built into the language. Unlike class-based languages with reserved constructor keywords, Rust lets me define custom constructors in impl blocks using any name. I’ve learned that Foo::new() is idiomatic, but not required.

> In Rust, .iter() gives me read-only access to each item in a collection, while .iter_mut() gives me the ability to mutate them. This ties directly into Rust’s strict borrowing rules—mutability is explicit, so the compiler can prevent data races or unexpected changes in memory.

Simulated Shell Interaction
> By wrapping allocator logic in a CLI, I now understand how user programs and system allocators interact through syscalls or shell commands. This builds the intuition of creating tools like top, malloc, or even a kernel monitor. I also learned how to capture user input and parse basic arguments in Rust.