# Rust Simple Text Calculator # ![tests](https://github.com/nsclass/rust-calculator/workflows/tests/badge.svg)

Simple calculator by using post fix algorithm in Rust

## Examples

```rust
let input = "1 + 2 * (3 + 4) / 2"; // expected 1234+*2/+
let (result, _trace_details) = calculate_str(input, true).unwrap();
assert_eq!(result, 8.);
```
