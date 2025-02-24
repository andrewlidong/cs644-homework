Rust was created by Mozilla in 2010.  

Rust provides memory safety guarantees without needing a garbage collector.  
Key Features: 
1. Memory Safety without Garbage Collection
	1. Ownership, borrowing, and lifetimes
		1. Use-after-free
		2. Dangling pointers
		3. Data races
	2. Statically enforces safe memory access at compile time.  
2. Ownership and Borrowing System
	1. Key Rules
		1. Each value has one owner at a time
		2. When the owner goes out of scope, the memory is automatically freed.  
		3. You can either borrow (immutable reference) or mutably borrow (mutable reference), but not both at the same time.  
3. No Null Pointers, No Undefined Behavior
4. Concurrency without Data Races
5. Strong Type System and Pattern Matching
6. Zero-Cost Abstractions
7. Cargo: Build-in Package Manager and Build System
8. Cross-Platform and WebAssembly Support

Pros
- Memory safety without garbage collection
- Concurrency safety (prevents race conditions)
- Fast performance, comparable to C++
- Great tooling (cargo, clippy, rustfmt)
- Active community and ecosystem (used by AWS, Microsoft, Google, Meta)

Cons
- Steep learning curve, especially for ownership and lifetimes
- Compilation times are slower than C/C++
- Limited library ecosystem compared to older languages like Python

When to use Rust?  
- System programming (operating systems, embedded systems)
- Performance-critical applications (game engines, compilers)
- WebAssembly (WASM) for high-performance web apps
- Cryptography and blockchain (Rust is used in Solana, Polkadot)
- Safe replacements for C/C++ in security-sensitive applications

