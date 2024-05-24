# Asm Unroll

[![Crates.io](https://img.shields.io/crates/v/asm_unroll)](https://crates.io/crates/asm_unroll)
[![docs.rs](https://img.shields.io/docsrs/asm_unroll)](https://docs.rs/asm_unroll/0.1.0/asm_unroll)

Provides a macro with extra functionality compared to normal inline assembly in Rust.
`asm_ext!()` allows inline for-loops whose bodies are unrolled into asm lines with literal values.

## Example

```rust
use asm_unroll::asm_ext;

pub fn sum_array(array: &[i64; 8]) -> i64 {
    let output: i64;

    unsafe {
        asm_ext!(
            // quickly zero a register
            "xor {output:e}, {output:e}",
            // This loop is unrolled and `{i}` is replaced with a literal.
            for i in 0..8 {
                "add {output}, [{array} + 8*{i}]",
            }
            // inputs:
            array = in(reg) array,
            // outputs:
            output = out(reg) output,
            options(nostack),
        );
    }

    output
}
```
Compiles to this assembly:
```asm
push rax

xor eax, eax
add rax, qword ptr [rdi]
add rax, qword ptr [rdi + 8]
add rax, qword ptr [rdi + 16]
add rax, qword ptr [rdi + 24]
add rax, qword ptr [rdi + 32]
add rax, qword ptr [rdi + 40]
add rax, qword ptr [rdi + 48]
add rax, qword ptr [rdi + 56]

pop rcx
ret
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.