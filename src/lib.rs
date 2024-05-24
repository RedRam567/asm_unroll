//! Provides a macro with extra functionality compared to [`asm!()`](std::arch::asm).
//! [`asm_ext!()`](macro@asm_ext) allows inline for-loops whose bodies are unrolled into asm lines with literal values.
//! See [the macro](macro@asm_ext) for more info.
//! 
//! ## Example
//! 
//! ```rust no_run
//! use asm_unroll::asm_ext;
//! 
//! pub fn sum_array(array: &[i64; 8]) -> i64 {
//!     let output: i64;
//! 
//!     unsafe {
//!         asm_ext!(
//!             // quickly zero a register
//!             "xor {output:e}, {output:e}",
//!             // This loop is unrolled and `{i}` is replaced with a literal.
//!             for i in 0..8 {
//!                 "add {output}, [{array} + 8*{i}]",
//!             }
//!             // inputs:
//!             array = in(reg) array,
//!             // outputs:
//!             output = out(reg) output,
//!             options(nostack),
//!         );
//!     }
//! 
//!     output
//! }
//! ```
//! Compiles to this assembly:
//! ```asm
//! push rax
//! 
//! xor eax, eax
//! add rax, qword ptr [rdi]
//! add rax, qword ptr [rdi + 8]
//! add rax, qword ptr [rdi + 16]
//! add rax, qword ptr [rdi + 24]
//! add rax, qword ptr [rdi + 32]
//! add rax, qword ptr [rdi + 40]
//! add rax, qword ptr [rdi + 48]
//! add rax, qword ptr [rdi + 56]
//! 
//! pop rcx
//! ret
//! ```

use proc_macro::TokenStream;
use std::ops::Range;

/// Works like [`asm!()`](std::arch::asm) but allows for-loops of [`Range`]s or
/// arrays whose bodies are unrolled and loop variables are replaced with literals.
/// Nested for-loops are currently not supported.
/// # Example
/// ```rust no_run
/// # const LEN: usize = 4096;
/// use asm_unroll::asm_ext;
///
/// fn do_some_important_math() -> u64 {
///     let output: u64;
///     let mem = &[0xBEEF, LEN];
///     unsafe {
///         asm_ext!(
///             "mov {output}, 0",
///             // This loop is unrolled 8 times.
///             for i in 0..8 {
///                 // `{i}` is replaced with integer literals.
///                 // The assembler folds all these constants into a single value.
///                 "add {output}, [{mem} + {i} * ({i} + {i}) - {i} * 1337 * 0]",
///                 "add {output}, {i}",
///             }
///             // Arrays are supported. Strings are substituted without quotes.
///             for rhs in [1, 2, "rdx", "{output}"] {
///                 "mov rax, {rhs}",
///             }
///             // Substitutions can be anywhere.
///             for inst in ["add", "sub"] {
///                 "{inst} rax, 1",
///             }
///             // inputs:
///             mem = in(reg) mem, // ptr to mem
///             // outputs:
///             output = out(reg) output,
///             // clobbers:
///             out("rax") _,
///             out("rdx") _,
///             options(nostack),
///         );
///     }
///     output
/// }
/// ```
/// # Notes
/// This macro breaks syntax highlighting and is opaque to LSP, in VS Code at least.
/// Also the "inline macro" action of Rust Analyzer is broken for me. It returns nothing for this macro.
/// It just deletes it. `cargo expand` works however, which is strange.
/// 
/// You can fix the syntax highlighting with the [Color My Text](https://marketplace.visualstudio.com/items?itemName=JatinSanghvi.color-my-text)
/// extension and this in your `settings.json`.
/// ```jsonc
/// "colorMyText.configurations": [
///     {
///         "paths": ["*.rs"],
///         "rules": [
///             // match text within braces containing no whitespace
///             {"patterns": ["\\{[^} ]*\\}"], "color": "BrightCyan"}
///         ]
///     }
/// ],
/// // override ansi color.
/// "workbench.colorCustomizations": {
///     // Dark+ variable blue
///     "terminal.ansiBrightCyan": "#9cdcfe"
/// },
/// ```
#[proc_macro]
// Attribute macro might fix highlighting/ast but this was hard enough to do.
pub fn asm_ext(input: TokenStream) -> TokenStream {
    let src = input.to_string();
    let bytes = src.as_bytes();

    let mut out = Vec::with_capacity(bytes.len() + 64); // about that much
    out.extend_from_slice(b"::core::arch::asm!(");

    let is_for = |bytes: &[u8]| -> bool {
        debug_assert!(bytes[0] == b'f');
        let Some(last) = char::from_u32(bytes[3] as u32) else { return false };
        let last_is_white = last.is_ascii_whitespace();
        bytes.len() >= 4 && bytes[1] == b'o' && bytes[2] == b'r' && last_is_white
    };

    // Go byte-by-byte, replace fors as they come, push to `out`, parse `out` to TokenStream
    let mut is_in_quotes = false;
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        match byte {
            b'f' if !is_in_quotes && is_for(&bytes[i..]) => {
                // Find where for loop starts and ends
                let ForLoop { ident, range_or_array: range, body_span } = parse_for(&src, i);
                let ident = format!("{{{}}}", ident); // {ident}
                let body = &src[body_span.clone()];

                // TODO: add empty asm line for more readable dissassembly
                // Unroll body
                for i in range.into_dyn_iter() {
                    out.extend_from_slice(body.replace(&ident, &i).as_bytes());
                }

                // skip to end of for loop
                i = body_span.end + 1;
                continue;
            }
            b'"' => {
                is_in_quotes = !is_in_quotes;
            }
            _ => (),
        }
        // Push unmodified byte
        out.push(byte);
        i += 1;
    }

    if is_in_quotes {
        panic!("bad quoting");
    }

    out.extend_from_slice(b")");
    String::from_utf8(out)
        .expect("BAD: output was somehow not utf-8")
        .parse()
        .expect("error parsing output to TokenSream")
}

/// ident, range, and body span
#[derive(Debug)]
struct ForLoop<'a> {
    ident: &'a str,
    /// for i in range
    // range: Range<i64>,
    range_or_array: RangeOrArray<'a>,
    /// not including braces
    body_span: Range<usize>,
}

/// Get ident, range, and body span
fn parse_for(src: &str, index: usize) -> ForLoop {
    fn is_non_quoted_char(char: char, is_in_quotes: &mut bool) -> impl FnMut(char) -> bool + '_ {
        move |c: char| {
            if c == '"' {
                *is_in_quotes = !*is_in_quotes;
            } else if c == char && !*is_in_quotes {
                return true;
            }
            false
        }
    }
    let mut is_in_quotes = false;

    let open_brace = src[index..]
        .find(is_non_quoted_char('{', &mut is_in_quotes))
        .expect("didn't find for loop open brace")
        + index
        + 1;
    if is_in_quotes {
        panic!("bad quoting");
    }

    let close_brace = src[open_brace..]
        .find(is_non_quoted_char('}', &mut is_in_quotes))
        .expect("didn't find for loop closing brace")
        + open_brace;
    if is_in_quotes {
        panic!("bad quoting");
    }

    let is_whitespace = |c: char| c.is_ascii_whitespace();
    let s = &src[index..];
    let (_for, rest) = s.split_once(is_whitespace).expect("malformed for");
    let (ident, rest) = rest.split_once(is_whitespace).expect("malformed for");
    let (_in, rest) = rest.split_once(is_whitespace).expect("malformed for");
    let (expression, _) = rest
        .split_once(is_non_quoted_char('{', &mut is_in_quotes))
        .expect("malformed for");
    if is_in_quotes {
        panic!("bad quoting");
    }

    let range_or_array = parse_range_or_array(expression);
    ForLoop { ident, range_or_array, body_span: open_brace..close_brace }
}

#[derive(Debug)]
enum RangeOrArray<'a> {
    Range(Range<i64>),
    Array(Vec<&'a str>),
}

impl<'a> RangeOrArray<'a> {
    // nice and simple dyn instead of complicated custom iter impl
    fn into_dyn_iter(self) -> Box<dyn Iterator<Item = String> + 'a> {
        match self {
            RangeOrArray::Range(range) => Box::new(range.map(|r| r.to_string())),
            RangeOrArray::Array(array) => Box::new(array.into_iter().map(|a| a.to_string())),
        }
    }
}

/// i64..i64
fn parse_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once("..").expect("expected range dots ..");
    start.parse().expect("bad start range")..end.parse().expect("bad end range")
}

fn parse_array(s: &str) -> Vec<&str> {
    // TODO: breaks array has strings with these characters
    s.split(|c| matches!(c, '[' | ']' | ','))
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() {
                Some(s.trim_matches('"'))
            } else {
                None
            }
        })
        .collect()
}

#[allow(clippy::needless_lifetimes)] // I've always had trouble with '_ lifetimes
fn parse_range_or_array<'a>(s: &'a str) -> RangeOrArray<'a> {
    let s = s.trim();
    if s.starts_with('[') {
        RangeOrArray::Array(parse_array(s))
    } else {
        RangeOrArray::Range(parse_range(s))
    }
}
