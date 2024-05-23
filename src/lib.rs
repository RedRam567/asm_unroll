use proc_macro::TokenStream;
use std::ops::Range;

/// Works like `asm!{}` but allows `for` loops of ranges which expand into unrolled integer literals.
/// For now only [`Range`]s of literal integers are supported. I plan on adding support for arrays.
/// # Notes
/// Breaks syntax highlighting and is opaque to LSP, in VS Code at least.
/// Also "inline macro" with Rust Analyzer is broken for me. It returns nothing, just deletes it.
/// `cargo expand` works however, which is strange.
/// # Examples
/// ```rust no_run
/// # const LEN: usize = 4096;
/// use asm_unroll::asm_ext;
///
/// fn do_some_important_math() -> u64 {
///     let output: u64;
///     let mem = &[0xBEEF, LEN];
///     unsafe {
///         asm_ext! {
///             "mov {output}, 0",
///             // This loop is unrolled 8 times.
///             for i in 0..8 {
///                 // `{i}` is replaced with integer literals.
///                 // The assembler folds all these constants into a single value.
///                 "add {output}, [{mem} + {i} * ({i} + {i}) - {i} * 1337 * 0]",
///                 "add {output}, {i}",
///             }
///             mem = in(reg) mem, // ptr to mem
///             output = out(reg) output,
///         };
///     }
///     output
/// }
/// ```
#[proc_macro]
// Attribute macro might fix highlighting/ast but this was hard enough to do.
pub fn asm_ext(input: TokenStream) -> TokenStream {
    let src = input.to_string();
    let bytes = src.as_bytes();

    let mut out = Vec::with_capacity(bytes.len() + 64); // about that much
    out.extend_from_slice(b"::core::arch::asm! {");
    
    // Go byte-by-byte, replace fors as they come, push to `out`, parse `out` to TokenStream
    let mut is_in_quotes = false;
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        match byte {
            // TODO: check for non-asm, =, or options
            b'f' if !is_in_quotes => {
                // Find where for loop starts and ends
                let ForLoop { ident, range, body_span } = parse_for(&src, i);
                let ident = format!("{{{}}}", ident); // {ident}
                let body = &src[body_span.clone()];

                // Unroll body
                for i in range {
                    out.extend_from_slice(body.replace(&ident, &i.to_string()).as_bytes());
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

    out.extend_from_slice(b"}");
    String::from_utf8(out).expect("BAD: output was somehow not utf-8")
        .parse()
        .expect("error parsing output to TokenSream")
}


/// i64..i64
fn parse_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once("..").expect("expected range dots ..");
    start.parse().expect("bad start range")..end.parse().expect("bad end range")
}

/// Get ident, range, and body span
fn parse_for(src: &str, index: usize) -> ForLoop {
    let body_start = src[index..].find('{').expect("didn't find for loop open brace") + index + 1;

    let mut is_in_quotes = false;
    let body_end = src[body_start..]
        .find(|c: char| {
            match c {
                '"' => {
                    is_in_quotes = !is_in_quotes;
                }
                '}' if !is_in_quotes => return true,
                _ => (),
            }
            false
        })
        .expect("didn't find for loop closing brace")
        + body_start;

    let is_whitespace = |c: char| c.is_ascii_whitespace();
    let s = &src[index..];
    let (_for, rest) = s.split_once(is_whitespace).expect("malformed for");
    let (ident, rest) = rest.split_once(is_whitespace).expect("malformed for");
    let (_in, rest) = rest.split_once(is_whitespace).expect("malformed for");
    let (range, _) = rest.split_once(is_whitespace).expect("malformed for");
    let range = parse_range(range);

    ForLoop { ident, range, body_span: body_start..body_end }
}

/// ident, range, and body span
#[derive(Debug)]
struct ForLoop<'a> {
    ident: &'a str,
    /// for i in range
    range: Range<i64>,
    /// not including braces
    body_span: Range<usize>,
}
