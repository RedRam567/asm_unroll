use proc_macro::TokenStream;
use std::ops::Range;

/// Works like `asm!{}` but allows `for` loops of ranges which expand into unrolled integer literals.
///
/// For now only [`Range`]s of literal integers are supported.
/// I do plan on adding support for arrays.
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
// Horrible code but works. Parsing-as-you-go would be much better
// but this macro will not be used much. I would've used awk or something but I
// knew that would be even worse.
// Attribute macro might fix highlighting/ast but this was hard enough to do.
pub fn asm_ext(input: TokenStream) -> TokenStream {
    let src = input.to_string();
    let bytes = src.as_bytes();

    // Find where all the for loops start and end
    let mut for_headers: Vec<(String, Range<i64>, Range<usize>)> = Vec::new(); // ident, for_loop_range, span
    let mut ends: Vec<usize> = Vec::new(); // index of closing brace
    let mut is_in_quotes = false;
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        match byte {
            b'f' if !is_in_quotes => {
                let parsed = parse_for_header(&src, i);
                let span_end = parsed.2.end;
                for_headers.push(parsed);
                i = span_end; // skip rest of header span
            }
            b'}' if !is_in_quotes => ends.push(i),
            b'"' => {
                is_in_quotes = !is_in_quotes;
            }
            _ => (),
        }
        i += 1;
    }
    if is_in_quotes {
        panic!("bad number of quotes");
    }
    assert_eq!(
        for_headers.len(),
        ends.len(),
        "malformed source, missing or extra brackets"
    );

    // Go byte-by-byte
    // If not at header: push to string
    // If at header: unroll body to string
    // Parse to TokenStream
    let mut out = Vec::new();
    out.extend_from_slice(b"::core::arch::asm! {");
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        // Check if at start of header by checking every header TODO: horrible.
        let mut all = for_headers.iter().zip(ends.iter());
        if let Some(((ident, range, span), end_idx)) = all.find(|((_, _, span), _)| i == span.start)
        {
            // Unroll for loop body
            let ident = format!("{{{}}}", ident); // {ident}
            let brackets_start = span.end;
            let brackets_end = *end_idx;
            let body = &src[brackets_start..brackets_end];
            for i in range.clone() {
                out.extend_from_slice(body.replace(&ident, &i.to_string()).as_bytes());
            }
            i = brackets_end + 1; // skip writing src for body
        } else {
            // push raw src
            i += 1;
            out.push(byte);
            continue;
        };
    }
    out.extend_from_slice(b"}");
    String::from_utf8(out)
        .expect("BAD: output was not utf-8")
        .parse()
        .expect("error parsing output to TokenSream")
}

/// Parse a for loop header at an index in the format: "for i in 0..8 {".
/// Extracts ident, range, and span
fn parse_for_header(src: &str, index: usize) -> (String, Range<i64>, Range<usize>) {
    let endl = src[index..]
        .find('{')
        .expect("unexpected eof while looking for closing bracket of `for`");
    let bracket_idx = index + endl + 1;
    let for_header = &src[index..bracket_idx];

    let is_whitespace = |c: char| c.is_ascii_whitespace();
    let (_for, rest) = for_header.split_once(is_whitespace).expect("malformed for");
    let (ident, rest) = rest.split_once(is_whitespace).expect("malformed for");
    let (_in, rest) = rest.split_once(is_whitespace).expect("malformed for");
    let (range, _) = rest.split_once(is_whitespace).expect("malformed for");
    let range = parse_range(range);

    (ident.to_string(), range, index..bracket_idx)
}

/// i64..i64
fn parse_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once("..").expect("expected range dots ..");
    start.parse().expect("bad start range")..end.parse().expect("bad end range")
}
