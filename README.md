# unimpl_symbols
Macros to indicate unimplemented symbols

# Usage
## unimplemented_functions!
```rust
unimplemented_functions! {
    pub fn function1(a: usize) {}

    pub fn function2(b: i32) -> std::io::Result<usize> {
        return Ok(1usize)
    }

    fn private_function() {}
}
```
