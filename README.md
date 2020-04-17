The arr crate provides a single fixed-sized array data-structure that is purely heap-based.

[Crates.io](https://crates.io/crates/arr)
[Docs](https://docs.rs/arr/latest/arr/)

Include:
```
[dependencies]
arr = "0.5.0"
```

Basic usage:
```
use arr::Array;

// Allocate a 16MB chunk of zeros as 16 byte sub-arrays
let huge_array = Array<[u8; 16]>::zero(1 << 20);
println!("{}", huge_array.len());
```

The motivation for this crate comes from the limitation of rust to easily allocate heap-based arrays without blowing the stack.

This likely will blow your stack:
```
let huge_array: [[u8; 16]; 1 << 20] = [[0u8; 16]; 1 << 20];
```
