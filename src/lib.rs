use proc_macro::TokenStream;
use std::io::Write;
use std::ops::Range;

// static mut DEPTH: usize = 0;

/// #[asm_unroll]
/// asm! {
///     "xor eax, eax",
///     for i in 0..8 {
///         "add rax, {i}"
///     }
/// }
// #[proc_macro_attribute]
// pub fn asm_unroll(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     // let item: Item = syn::parse(input.clone()).expect("Failed to parse input.");

//     // for token in input.clone() {
//     //     println!("token: {token:#?}");
//     //     // token.
//     // }
//     // input

//     // let Item::Macro(macr) = item else { panic!() };
//     // for item in macr.mac.tokens {
//     //     println!("item: {item:?}");
//     // }
//     // input

//     // let item: Item = syn::parse(input.clone()).expect("Failed to parse input.");

//     // if let Item::Macro()

//     // unsafe { DEPTH += 1 };
//     // eprintln!("{}", unsafe { DEPTH });

//     let mut saw_ident = false;
//     let mut saw_bang = false;
//     let ts = input
//         .into_iter()
//         .map(|tt| {
//             match tt.clone() {
//                 TokenTree::Group(group) => {
//                     if saw_ident && saw_bang {
//                         println!("ident: {} bang: {} {:#?}", saw_ident, saw_bang, &tt);
//                         // tt
//                         eprintln!("LETS GO");
//                         TokenTree::Group(group)
//                     } else {
//                         saw_ident = false;
//                         saw_bang = false;
//                         // tt
//                         let delim = group.delimiter();
//                         let stream = group.stream();
//                         let stream = asm_unroll(_attr.clone(), stream);
//                         TokenTree::Group(Group::new(delim, stream))
//                     }
//                 }
//                 TokenTree::Ident(_) => {
//                     saw_ident = true;
//                     tt
//                 }
//                 TokenTree::Punct(ref punct) => {
//                     if punct.as_char() == '!' {
//                         saw_bang = true;
//                     } else {
//                         saw_ident = false;
//                         saw_bang = false;
//                     }
//                     tt
//                 }
//                 TokenTree::Literal(_) => {
//                     saw_ident = false;
//                     saw_bang = false;
//                     tt
//                 }
//             }
//         })
//         .collect();

//     // unsafe {DEPTH-=1};
//     ts
// }

// fn do_the_unrolling(input: TokenStream) -> TokenStream {
//     // let asm_inner =
//     // let inner: Item = syn::parse(input.clone()).expect("Failed to parse input.");
//     input.into_iter().map(|tt| {
//         // literals and punct
//         // sometimes ident, ident, dient lietal .. literal
//         match tt {
//             TokenTree::Group(_) => todo!(),
//             TokenTree::Ident(_) => todo!(),
//             TokenTree::Punct(_) => todo!(),
//             TokenTree::Literal(_) => todo!(),
//         }
//         tt
//     }).collect()
// }

// #[proc_macro]
// pub fn unroll_for(input: TokenStream) -> TokenStream {
//     let mut saw_for: bool;
//     let mut loop_ident: Option<String>;
//     let mut saw_in: bool;
//     let mut start: Option<i128>;
//     let mut saw_dot1: bool;
//     let mut saw_dot2: bool;
//     let mut saw_eq: bool;
//     let mut end: Option<i128>;
//     let mut done: bool;

//     let set_or_panic = |bool: &mut bool| {
//         if *bool {
//             panic!("malformed, already saw this");
//         } else {
//             *bool = true;
//         }
//     };
//     // TokenStream::extend(&mut input.clone(), None);

//     let iter =
//     // input
//     //     .into_iter()
//     //     .map(|tt| {
//     //         // literals and punct
//     //         // sometimes ident, ident, dient lietal .. literal
//     //         match tt {
//     //             TokenTree::Group(_) => todo!(),
//     //             TokenTree::Ident(ref ident) => match ident.to_string().as_str() {
//     //                 "for" => set_or_panic(&mut saw_for),
//     //                 "in" => set_or_panic(&mut saw_in),
//     //                 name => {
//     //                     if loop_ident.is_none() {
//     //                         loop_ident = Some(name.to_string());
//     //                     } else {
//     //                         panic!("malformed, loop var already set");
//     //                     }
//     //                 }
//     //             },
//     //             TokenTree::Punct(_) => todo!(),
//     //             TokenTree::Literal(literal) => {
//     //                 let replaced = literal.to_string().replace("i", "5");
//     //                 TokenTree::Literal(Literal::string(&replaced))
//     //             }
//     //         }
//     //     })
//     //     .collect()
// }
// #[proc_macro]
// pub fn unroll_for(input: TokenStream) -> TokenStream {
//     // dbg!(input.clone());
//     // let parsed: Item = syn::parse(input.clone()).expect("Failed to parse input.");
//     // dbg!(parsed);
//     // // let Item::
//     // // dbg!(parsed);
//     // let Item::
//     input
// }

// #[proc_macro]
// pub fn unroll_for(input: TokenStream) -> TokenStream {
//     let mut buf = String::new();
//     let src = input.to_string();
//     let mut i = 0;
//     for i in 0..8 {
//         // str.replace(from, to)
//         buf.push_str(&replace_with(&src, "{i}", |_,_,_| i.to_string()).to_string());
//     }

//     let x = syn::parse_str(&buf).unwrap();
//     x
// }

// /// almost works, but returned stuff seems like it needs to "one item"
// /// simple version
// /// macro! {
// /// "for i in 0..8"
// /// "asm {i}"
// /// }
// #[proc_macro]
// pub fn unroll_for(input: TokenStream) -> TokenStream {
//     // Parse first line
//     let first = input.clone().into_iter().next().unwrap();
//     let TokenTree::Literal(literal) = first else {
//         panic!()
//     };
//     let str = literal.to_string();
//     let mut iter = str.split_ascii_whitespace();
//     iter.next().unwrap(); // for
//     let ident = format!("{{{}}}", iter.next().unwrap().trim_matches('"'));
//     iter.next().unwrap(); // in
//     let range = parse_range(iter.next().unwrap().trim_matches('"'));

//     // let mut out = TokenStream::new();
//     // for i in range {
//     //     let i = i.to_string();
//     //     let new = input.clone().into_iter().skip(2).map(|tt| {
//     //         if let TokenTree::Literal(ref literal) = tt {
//     //             let new = Literal::string(&literal.to_string().replace(&ident, &i));
//     //             dbg!(&new);
//     //             TokenTree::Literal(new)
//     //         } else {
//     //             tt
//     //         }
//     //     });
//     //     out.extend(new);
//     // }
//     // dbg!(&out);
//     // out
//     "[\"abc\", \"abcd\"]".parse().unwrap()
// }

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
// absolutely horrible code but works. parsing-as-you-go would be MUCH better
// but this macro will not be used much. I would've used awk or something but I
// knew that would be even worse.
// 
pub fn asm_ext(input: TokenStream) -> TokenStream {
    let src = input.to_string();

    // Find where all the for loops start and end
    let mut for_headers: Vec<(String, Range<i64>, Range<usize>)> = Vec::new(); // ident, for_loop_range, span
    let mut ends: Vec<usize> = Vec::new(); // index of closing brace
    let mut is_in_quotes = false;
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        match byte {
            b'f' if !is_in_quotes => {
                let parsed = parse_for_header(&src, i);
                let span_end = parsed.2.end;
                for_headers.push(parsed);
                i = span_end; // skip rest of for header span
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

    // Delete for loop and braces to prepare for next step.
    let og_len = src.len();
    let mut src = src;
    // Replace for loop header with spaces. "delete" it
    for (_, _, span) in for_headers.iter() {
        let span = span.clone();
        let len = span.len();
        // very bad way to make n length string of a character
        let spaces = String::from_utf8(vec![b' '; len]).unwrap();
        src.replace_range(span, &spaces);
    }
    // Remove end bracket
    for i in ends.iter().copied() {
        src.replace_range(i..i + 1, " ");
    }
    assert_eq!(og_len, src.len());

    // Go byte-by-byte
    // If not at header: push to string
    // If at header: unroll body to string
    // Parse to TokenStream
    let mut out = Vec::new();
    out.extend_from_slice(b"::core::arch::asm! {");
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        i += 1;
        // Find if at start of header by checking every header TODO: horrible.
        let mut all = for_headers.iter().zip(ends.iter());
        let Some(((ident, range, span), end_idx)) = all.find(|((_, _, span), _)| i == span.start)
        else {
            out.push(byte);
            continue;
        };

        // Unroll for loop body
        let ident = format!("{{{}}}", ident); // {ident}
        let brackets_start = span.end;
        let brackets_end = *end_idx;
        let body = &src[brackets_start..brackets_end];
        for i in range.clone() {
            write!(out, "{}", body.replace(&ident, &i.to_string())).unwrap();
        }
        i = brackets_end; // skip writing src for body
    }
    out.extend_from_slice(b"}");
    String::from_utf8(out).expect("BAD: output was not utf-8").parse().expect("error parsing output to TokenSream")
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

fn parse_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once("..").expect("expected range dots ..");
    start.parse().expect("bad start range")..end.parse().expect("bad end range")
}

// /// https://users.rust-lang.org/t/pre-rfc-str-replace-with-function-to-replace-text-with-closure/72170
// fn replace_with<'a, 'b, F, S>(this: &'a str, pattern: &'b str, mut replacer: F) -> Cow<'a, str>
// where
//     F: FnMut(usize, usize, &'a str) -> S,
//     S: AsRef<str>,
// {
//     let mut result = String::new();
//     let mut lastpos = 0;

//     for (idx, (pos, substr)) in this.match_indices(pattern).enumerate() {
//         result.push_str(&this[lastpos..pos]);
//         lastpos = pos + substr.len();
//         let replacement = replacer(idx, pos, substr);
//         result.push_str(replacement.as_ref());
//     }

//     if lastpos == 0 {
//         Cow::Borrowed(this)
//     } else {
//         result.push_str(&this[lastpos..]);
//         Cow::Owned(result)
//     }
// }
