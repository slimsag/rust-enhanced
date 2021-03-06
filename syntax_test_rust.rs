// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

// Line comments
// <- comment.line.double-slash
// ^^^^^^^^^^^^^^ comment.line.double-slash
/// Line doc comments
// <- comment.line.documentation
// ^^^^^^^^^^^^^ comment.line.documentation

/*!
// <- comment.block.documentation
 // <- comment.block.documentation
//^ comment.block.documentation
Block doc comments
// ^^^^^^^^^^^^^^^^ comment.block.documentation
/* Nested comments */
// ^^^^^^^^^^^^^^^^^^ comment.block.documentation comment.block
*/

#[macro_use]
//          <- meta.annotation
extern crate std_web;
//    <- keyword.other
//     ^^^^^ keyword.other
//           ^^^^^^^ source
//                  ^ punctuation.terminator
/*#[macro_use]
//            <- comment.block
extern extern crate simd_rng_derive;*/
//                                    <- comment.block

// This one is just to visually confirm the testing comments don't intefere
#[macro_use]
extern crate std_web;
/*#[macro_use]
extern extern crate simd_rng_derive;*/

let c = 'c';
// <- storage.type
//    ^ keyword.operator
//      ^^^ string.quoted.single
let b = b'c';
// <- storage.type
//    ^ keyword.operator
//      ^ storage.type
//       ^^^ string.quoted.single

let s = "This is a string \x01_\u{007F}_\"_\'_\\_\r_\n_\t_\0";
// <- storage.type
//    ^ keyword.operator
//      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ string.quoted.double
//                        ^^^^ constant.character.escape
//                             ^^^^^^^^ constant.character.escape
//                                      ^^ constant.character.escape
//                                         ^^ constant.character.escape
//                                            ^^ constant.character.escape
//                                               ^^ constant.character.escape
//                                                  ^^ constant.character.escape
//                                                     ^^ constant.character.escape
//                                                        ^^ constant.character.escape
let r = r#"This is a raw string, no escapes! \x00 \0 \n"#;
// <- storage.type
//    ^ keyword.operator
//      ^ storage.type
//       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ string.quoted.double - constant.character.escape

let bytes = b"This won't escape unicode \u{0123}, but will do \x01_\"_\'_\\_\r_\n_\t_\0";
// <- storage.type
//        ^ keyword.operator
//          ^ storage.type
//           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ string.quoted.double
//                                                            ^^^^ constant.character.escape
//                                                                 ^^ constant.character.escape
//                                                                    ^^ constant.character.escape
//                                                                       ^^ constant.character.escape
//                                                                          ^^ constant.character.escape
//                                                                             ^^ constant.character.escape
//                                                                                ^^ constant.character.escape
//                                                                                   ^^ constant.character.escape

let raw_bytes = br#"This won't escape anything either \x01 \""#;
// <- storage.type
//            ^ keyword.operator
//              ^^ storage.type
//                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ string.quoted.double - constant.character.escape

let b_simple = b'a';
//             ^^^^ string.quoted.single
//             ^ storage.type.string
//              ^ punctuation.definition.string.begin
//                ^ punctuation.definition.string.end
//                 ^ punctuation.terminator
let b_newline = b'\n';
//              ^^^^^ string.quoted.single
//                ^^ string.quoted.single constant.character.escape
let b_nul = b'\0';
//            ^^ string.quoted.single constant.character.escape
let b_back = b'\\';
//             ^^ string.quoted.single constant.character.escape
let b_quote = b'\'';
//              ^^ string.quoted.single constant.character.escape
let b_esc_nul = b'\x00';
//                ^^^^ string.quoted.single constant.character.escape
let b_esc_255 = b'\xff';
//                ^^^^ string.quoted.single constant.character.escape
let b_esc_inv = b'\a';
//                ^^ invalid.illegal.byte
//                  ^ string.quoted.single punctuation.definition.string.end
let b_inv_len = b'abc';
//                ^ string.quoted.single
//                 ^^ invalid.illegal.byte
//                   ^ string.quoted.single punctuation.definition.string.end
let b_inv_uni = b'♥';
//                ^ invalid.illegal.byte
//                 ^ string.quoted.single punctuation.definition.string.end
let b_inv_empty = b'';
//                ^^^ string.quoted.single
//                 ^ punctuation.definition.string.begin
//                  ^ punctuation.definition.string.end
let b_unclosed1 = b'
// Avoid error on entire file.
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ comment.line.double-slash - invalid - string

let bs_newline = b"abc\n";
//               ^^^^^^^^ string.quoted.double
//                ^ punctuation.definition.string.begin
//                    ^^ constant.character.escape
//                      ^ punctuation.definition.string.end
//                       ^ punctuation.terminator
let bs_nul = b"abc\0";
//                ^^ string.quoted.double constant.character.escape
let bs_esc_nul = b"abc\x00";
//                    ^^^^ string.quoted.double constant.character.escape
let bs_esc_255 = b"abc\xff";
//                    ^^^^ string.quoted.double constant.character.escape
let bs_esc_inv = b"abc\a";
//                    ^^ string.quoted.double invalid.illegal.character.escape
//                      ^ string.quoted.double punctuation.definition.string.end - invalid

let char_newline = '\n';
//                 ^^^^ string.quoted.single
//                 ^ punctuation.definition.string.begin
//                  ^^ constant.character.escape
//                    ^ punctuation.definition.string.end
//                     ^ punctuation.terminator
let char_nul = '\0';
//              ^^ string.quoted.single constant.character.escape
let char_extra_inv = 'ab';
//                    ^ string.quoted.single
//                     ^ invalid.illegal.char
//                      ^ string.quoted.single punctuation.definition.string.end
let char_ascii_esc_nul = '\x00';
//                        ^^^^ string.quoted.single constant.character.escape
let char_ascii_esc_127 = '\x7f';
//                        ^^^^ string.quoted.single constant.character.escape
let char_ascii_inv_255 = '\xff';
//                        ^^^^ invalid.illegal.char
let char_uni_esc = '\u{3b1}';
//                  ^^^^^^^ string.quoted.single constant.character.escape
let char_uni_esc_empty = '\u{}';
//                        ^^^^ invalid.illegal.char
let char_uni_esc_under_start = '\u{_1_}';
//                              ^^^^^^^ invalid.illegal.char
let char_uni_esc_under1 = '\u{1_}';
//                         ^^^^^^ string.quoted.single constant.character.escape
let char_uni_esc_under2 = '\u{1_2__3___}';
//                         ^^^^^^^^^^^^^ string.quoted.single constant.character.escape
let char_uni_esc_under3 = '\u{10__FFFF}';
//                         ^^^^^^^^^^^^ string.quoted.single constant.character.escape
let char_uni_esc_extra = '\u{1234567}';
//                        ^^^^^^^^^^^ invalid.illegal.char

let s_ascii_inv_255 = "\xff";
//                     ^^ string.quoted.double invalid.illegal.character.escape
let s_uni_esc_empty = "\u{}";
//                     ^^^^ string.quoted.double invalid.illegal.character.escape
let s_uni_esc_under_start = "\u{_1_}";
//                           ^^^^^^^ string.quoted.double invalid.illegal.character.escape
let s_uni_esc_under1 = "\u{1_}";
//                      ^^^^^^ string.quoted.double constant.character.escape
let s_uni_esc_under2 = "\u{1_2__3___}";
//                      ^^^^^^^^^^^^^ string.quoted.double constant.character.escape
let s_uni_esc_under3 = "\u{10__FFFF}";
//                      ^^^^^^^^^^^^ string.quoted.double constant.character.escape
let s_uni_esc_extra = "\u{1234567}";
//                     ^^^^^^^^^^^ string.quoted.double invalid.illegal.character.escape

0;
// <- constant.numeric.integer.decimal
1_000u32;
// <- constant.numeric.integer.decimal
 // <- constant.numeric.integer.decimal
//^^^ constant.numeric.integer.decimal
//   ^^^ storage.type - constant.numeric.integer.decimal
1i64;
// <- constant.numeric.integer.decimal
 // <- storage.type - constant.numeric.integer.decimal
//^^ storage.type - constant.numeric.integer.decimal

0.2;
// <- constant.numeric.float
 // <- constant.numeric.float
//^ constant.numeric.float
1_000.0_;
// <- constant.numeric.float
 // <- constant.numeric.float
//^^^^^^ constant.numeric.float
1.0f32;
// <- constant.numeric.float
 // <- constant.numeric.float
//^ constant.numeric.float
// ^^^ storage.type - constant.numeric.float
0.;
// <- constant.numeric.float
 // <- constant.numeric.float
0f64;
// <- constant.numeric.float
 // <- storage.type - constant.numeric.float
//^^ storage.type - constant.numeric.float
1e+8;
// <- constant.numeric.float
 // <- constant.numeric.float
//^^ constant.numeric.float
1.0E-8234987_f64;
// <- constant.numeric.float
 // <- constant.numeric.float
//^^^^^^^^^^^ constant.numeric.float
//           ^^^ storage.type - constant.numeric.float

0x0;
// <- constant.numeric.integer.hexadecimal
 // <- constant.numeric.integer.hexadecimal
//^ constant.numeric.integer.hexadecimal
0xfa;
// <- constant.numeric.integer.hexadecimal
 // <- constant.numeric.integer.hexadecimal
//^^ constant.numeric.integer.hexadecimal
0xFA_01i32;
// <- constant.numeric.integer.hexadecimal
 // <- constant.numeric.integer.hexadecimal
//^^^^^ constant.numeric.integer.hexadecimal
//     ^^^ storage.type - constant.numeric.integer.hexadecimal

0b1;
// <- constant.numeric.integer.binary
 // <- constant.numeric.integer.binary
//^ constant.numeric.integer.binary
0b0_1u8;
// <- constant.numeric.integer.binary
 // <- constant.numeric.integer.binary
//^^^ constant.numeric.integer.binary
//   ^^ storage.type - constant.numeric.integer.binary

0o0;
// <- constant.numeric.integer.octal
 // <- constant.numeric.integer.octal
//^ constant.numeric.integer.octal
0o0000_0010u64;
// <- constant.numeric.integer.octal
 // <- constant.numeric.integer.octal
//^^^^^^^^^ constant.numeric.integer.octal
//         ^^^ storage.type - constant.numeric.integer.octal

0x12e15e35b500f16e2e714eb2b37916a5_u128;
// <- constant.numeric.integer.hexadecimal
 // <- constant.numeric.integer.hexadecimal
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant.numeric.integer.hexadecimal
//                                 ^^^^ storage.type - constant.numeric.integer.hexadecimal

extern crate foo;
// <- keyword.other
//^^^^ keyword.other
//    ^ - keyword.other
//     ^^^^^ keyword.other

mod trafile;
// <- storage.type.module
mod comment;
// <- storage.type.module
mod location;

pub use self::trafile::*;
// <- storage.modifier
//   ^ keyword.other
//      ^^^^^^^^^^^^^^^ meta.path

use std::fmt;
// <- keyword.other
//  ^^^^^ meta.path
//       ^^^ - meta.path
use foo::i32;
//  ^^^^^ meta.path
//       ^^^ - meta.path storage.type
use foo::Bar;
//  ^^^^^ meta.path
use foo::{Baz, QUX, quux};
//  ^^^^^ meta.path
//       ^^^^^^^^^^^^^^^^ meta.block
//             ^^^ constant.other

String my_var = format!("Hello {0}", "World");
// ^^^ support.type
//            ^ keyword.operator
//              ^^^^^^^ support.macro
//                     ^ punctuation.definition.group.begin
//                     ^^^^^^^^^^^^^^^^^^^^^^ meta.group
//                      ^^^^^^^^^^^ string.quoted.double
//                             ^^^ constant.other.placeholder
//                                          ^ punctuation.definition.group.end

my_var = format!("Hello {name}, how are you?",
//     ^ keyword.operator
//       ^^^^^^^ support.macro
//              ^ punctuation.definition.group.begin
//              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.group
//               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ string.quoted.double
//                      ^^^^^^ constant.other.placeholder
    name="John");
// ^^^^^^^^^^^^^ meta.group
//      ^ keyword.operator
//       ^^^^^^ string.quoted.double
//             ^ punctuation.definition.group.end

struct BasicStruct(i32);
// ^^^^^^^^^^^^^^^^^^^^ meta.struct
// <- storage.type.struct
//^^^^ storage.type.struct
//     ^^^^^^^^^^^ entity.name.struct
//                ^ punctuation.definition.group.begin
//                 ^^^ storage.type
//                    ^ punctuation.definition.group.end

#[derive(Debug)]
// <- meta.annotation.rust
//^^^^^^^^^^^^^^ meta.annotation.rust
struct PrintableStruct(Box<i32>);
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.struct
// <- storage.type.struct
//^^^^ storage.type.struct
//     ^^^^^^^^^^^^^^^ entity.name.struct
//                    ^ punctuation.definition.group.begin
//                     ^^^^^^^^ meta.generic
//                        ^ punctuation.definition.generic.begin
//                         ^^^ storage.type
//                            ^ punctuation.definition.generic.end
//                             ^ punctuation.definition.group.end


// fixes https://github.com/rust-lang/sublime-rust/issues/144
fn factory() -> Box<Fn(i32) -> i32> {
// <- storage.type.function
// ^^^^^^^ entity.name.function
//                  ^^^^^^^^^^^^^^ meta.generic
//                      ^^ storage.type
//                              ^^ storage.type
//                          ^^ source.rust meta.function.rust meta.function.return-type.rust

    Box::new(|x| x + 1)
}

impl fmt::Display for PrintableStruct {
// <- meta.impl
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
// <- storage.type.impl
//^^ storage.type.impl
//   ^^^^^ meta.path
//                ^^^ keyword.other
//                    ^^^^^^^^^^^^^^^ entity.name.impl
//                                    ^ meta.block punctuation.definition.block.begin
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function
//  ^^ storage.type.function
//     ^^^ entity.name.function
//        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function.parameters
//        ^ punctuation.definition.parameters.begin
//         ^ keyword.operator
//          ^^^^ variable.parameter
//                ^ variable.parameter
//                 ^ punctuation.separator
//                   ^ keyword.operator
//                    ^^^ storage.modifier
//                        ^^^^^ meta.path
//                                      ^ punctuation.definition.parameters.end
//                                        ^^ punctuation.separator
//                                           ^^^^^ meta.path
//                                                       ^ meta.block punctuation.definition.block.begin
        write!(f, "{}", self.0)
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function
//      ^^^^^^ support.macro
//            ^^^^^^^^^^^^^^^^^ meta.group
//            ^ punctuation.definition.group.begin
//                ^^^^ string.quoted.double
//                 ^^ constant.other.placeholder
//                            ^ punctuation.definition.group.end
        write!(f, "{:10}", self.0)
//                 ^^^^^ constant.other.placeholder
        eprint!("{:^10}", self.0)
//      ^^^^^^^ support.macro
//               ^^^^^^ constant.other.placeholder
        eprintln!("{:+046.89?}", self.0)
//      ^^^^^^^^^ support.macro
//                 ^^^^^^^^^^^ constant.other.placeholder
        assert!(true, "{:-^#10x}", self.0)
//      ^^^^^^^ support.macro
//                     ^^^^^^^^^ constant.other.placeholder
        debug_assert!(true, "{4j:#xf10}", self.0)
//      ^^^^^^^^^^^^^ support.macro
//                           ^^^^^^^^^^ string.quoted.double
        write!(f, "{{}}", self.0)
//                 ^^^^ constant.character.escape.rust
        write!(get_writer(), "{}", "{}")
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function
//      ^^^^^^ support.macro
//            ^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.group
//             ^^^^^^^^^^ support.function
//                           ^^^^ string.quoted.double
//                            ^^ constant.other.placeholder
//                                 ^^^^ string.quoted.double
//            ^ punctuation.definition.group.begin
//                                     ^ punctuation.definition.group.end
        writeln!(w)
// ^^^^^^^^^^^^^^^^ meta.function
//      ^^^^^^^^ support.macro
//              ^^^ meta.group
//              ^ punctuation.definition.group.begin
//                ^ punctuation.definition.group.end
        println!()
// ^^^^^^^^^^^^^^^ meta.function
//      ^^^^^^^^ support.macro
//              ^^ meta.group
//              ^ punctuation.definition.group.begin
//               ^ punctuation.definition.group.end
    }
// ^^ meta.function meta.block
//  ^ punctuation.definition.block.end
}
// <- meta.block punctuation.definition.block.end

let logical: bool = true;
//         ^ punctuation.separator
//           ^^^^ storage.type
//                ^ keyword.operator
//                  ^^^^ constant.language
let mut mutable = 12;
//  ^^^ storage.modifier

let ch = '∞';
//       ^^^ string.quoted.single

let tuple = (1.0, 0i32, "Hello");
//          ^^^^^^^^^^^^^^^^^^^^ meta.group
//          ^ punctuation.definition.group.begin
//           ^^^ constant.numeric.float
//                ^ constant.numeric.integer.decimal
//                 ^^^ storage.type
//                      ^^^^^^^ string.quoted.double
//                             ^ punctuation.definition.group.end

// Looks like tuples use meta.group so we'll use that for tuple arguments too
// fixes https://github.com/rust-lang/sublime-rust/issues/164
let f = |(x, y)| { x + y };
//      ^ punctuation.definition.parameters.begin
//       ^^^^^^ meta.group

let xs: [i32; 5] = [1, 2, 3, 4, 5];
//    ^ punctuation.separator
//      ^^^^^^^^ meta.group
//      ^ punctuation.definition.group.begin
//       ^^^ storage.type
//            ^ constant.numeric.integer.decimal
//             ^ punctuation.definition.group.end
//                 ^^^^^^^^^^^^^^^ meta.group
//                 ^ punctuation.definition.group.begin
//                               ^ punctuation.definition.group.end
let xsl = &xs;
//        ^ keyword.operator

type FnPointer = fn(i32) -> i32;
//   ^^^^^^^^^ entity.name.type
//               ^^ storage.type.function
//                 ^^^^^ meta.group
//                  ^^^ storage.type
//                       ^^ punctuation.separator
//                          ^^^ storage.type

type GenFnPointer = Bar<fn(i32) -> i32>;
//   ^^^^^^^^^^^^ entity.name.type
//                  ^^^^^^^^^^^^^^^^^^^ meta.generic
//                      ^^ storage.type.function
//                        ^^^^^ meta.group
//                         ^^^ storage.type
//                              ^^ punctuation.separator
//                                 ^^^ storage.type
//                                     ^ - meta.generic

type GenFnPointer2 = Bar<extern "C" fn()>;
//   ^^^^^^^^^^^^^ entity.name.type
//                   ^^^^^^^^^^^^^^^^^^^^ meta.generic
//                       ^^^^^^ keyword.other
//                              ^^^ string.quoted.double
//                                  ^^ storage.type.function
//                                       ^ - meta.generic

struct Nil;
// ^^^^^^^ meta.struct
//        ^ - meta.struct
struct Pair(i32, i32);
// ^^^^^^^^^^^^^^^^^^ meta.struct
//          ^^^ storage.type
//               ^^^ storage.type
//                   ^ - meta.struct

enum OperatingSystem
// <- storage.type.enum
// ^^^^^^^^^^^^^^^^^ meta.enum
//   ^^^^^^^^^^^^^^^ entity.name.enum
{
// <- meta.enum meta.block punctuation.definition.block.begin
    Osx,
    Windows,
    Linux,
    Bsd(String),
    //  ^^^^^^ support.type
    Info { field: i32, value: str }
    //   ^ meta.block meta.block punctuation.definition.block.begin
    //            ^^^ storage.type
    //                        ^^^ storage.type
    //                            ^ meta.block meta.block punctuation.definition.block.end
}
// <- meta.enum meta.block punctuation.definition.block.end

const ZERO: u64 = 0;
// <- storage.type
//    ^^^^ constant.other
//        ^ punctuation.separator
//          ^^^ storage.type
//              ^ keyword.operator
//                ^ constant.numeric.integer.decimal
static NAME: &'static str = "John";
// <- storage.type
//           ^ keyword.operator
//            ^^^^^^^ storage.modifier.lifetime
//                    ^^^ storage.type
//                        ^ keyword.operator
//                          ^^^^^^ string.quoted.double


let z = {
//      ^ meta.block punctuation.definition.block.begin
    2 * 5
//  ^ constant.numeric.integer.decimal
//    ^ keyword.operator
//      ^ constant.numeric.integer.decimal
};
// <- meta.block punctuation.definition.block.end

fn my_func(x: i32)
// <- storage.type.function
// ^^^^^^^ entity.name.function
//        ^^^^^^^^ meta.function.parameters
//        ^ punctuation.definition.parameters.begin
//         ^ variable.parameter
//          ^ punctuation.separator
//               ^ punctuation.definition.parameters.end
{
// <-  meta.function meta.block punctuation.definition.block.begin

}
// <-  meta.function meta.block punctuation.definition.block.end

let n = 5;

if n < 0 {
//       ^ meta.block punctuation.definition.block.begin
// <- keyword.control
    print!("{} is negative", n);
} else if n > 0 {
// <- meta.block punctuation.definition.block.end
// ^^^ keyword.control
//              ^ meta.block punctuation.definition.block.begin
//     ^^ keyword.control
    print!("{0} is positive", n);
} else {
// <- meta.block punctuation.definition.block.end
// ^^^ keyword.control
//     ^ meta.block punctuation.definition.block.begin
    print!("{} is zero", n);
// ^^^^^^^^^^^^^^^^^^^^^^^^^ meta.block
}
// <- meta.block punctuation.definition.block.end

let big_n =
//        ^ keyword.operator
    if n < 10 && n > -10 {
//                       ^ meta.block punctuation.definition.block.begin
        10 * n
    } else {
//  ^ meta.block punctuation.definition.block.end
//         ^ meta.block punctuation.definition.block.begin
        n / 2
    };
//  ^ meta.block punctuation.definition.block.end

'label_name: loop {
// ^^^^^^^^ entity.name.label
//         ^ punctuation.separator
//           ^^^^ keyword.control
//                ^ meta.block punctuation.definition.block.begin
    n += 1;
    if n / 2 == 5 {
//       ^ keyword.operator
        continue;
//      ^^^^^^^^ keyword.control
    }
    if n > 9 {
        break;
//      ^^^^^ keyword.control
    }
}

'label1: for _ in 0..100 {
    'label2 : loop {
//  ^^^^^^^ entity.name.label
//          ^ punctuation.separator.rust
        'label3: while true {
//      ^^^^^^^ entity.name.label
//             ^ punctuation.separator
            break 'label2;
//                ^^^^^^^ entity.name.label
//                       ^ punctuation.terminator
        }
        continue 'label1;
//               ^^^^^^^ entity.name.label
//                      ^ punctuation.terminator
    }
}

while n < 50 {
//^^^ keyword.control
    n = n * 2;
}

for i in 1..10 {
// <- keyword.control
//    ^^ keyword.operator
//       ^ constant.numeric.integer.decimal
//        ^^ keyword.operator
//          ^^ constant.numeric.integer.decimal
    println!("I: {}", i);
// ^^^^^^^^^^^^^^^^^^^^^^ meta.block
}
// <- meta.block punctuation.definition.block.end

let o = match n {
//      ^^^^^ keyword.control
    1 => "one",
//  ^ constant.numeric.integer.decimal
//    ^^ keyword.operator
//       ^^^^^ string.quoted.double
    2 => "two",
//  ^ constant.numeric.integer.decimal
//    ^^ keyword.operator
//       ^^^^^ string.quoted.double
    3...5 => "a few",
//  ^ constant.numeric.integer.decimal
//   ^^^ keyword.operator
//      ^ constant.numeric.integer.decimal
//        ^^ keyword.operator
//           ^^^^^^^ string.quoted.double
    _ => "lots",
//  ^ source.rust
//    ^^ keyword.operator
};

let mut j = BasicStruct(10);
//  ^^^ storage.modifier
//                      ^^ constant.numeric.integer.decimal

if let BasicStruct(i) = j {
// ^^^ storage.type
//                    ^ keyword.operator
//                        ^ meta.block punctuation.definition.block.begin
    println!("Basic value: {}", i);
}
// <- meta.block punctuation.definition.block.end

while let BasicStruct(k) = j {
//^^^ keyword.control
//    ^^^ storage.type
//                       ^ keyword.operator
//                           ^ meta.block punctuation.definition.block.begin
    println!("Constructed example: {}", j)
    j = BasicStruct(j + 1);
    if k > 20 {
        break;
        //^^^ meta.block meta.block keyword.control
    }
}
// <- meta.block punctuation.definition.block.end

fn foo<A>(i: u32, b: i64) -> u32 {
// <- storage.type.function
// ^^^ entity.name.function
//    ^ punctuation.definition.generic.begin
//      ^ punctuation.definition.generic.end
//       ^^^^^^^^^^^^^^^^ meta.function.parameters
//                           ^^^ storage.type
//                               ^ meta.block punctuation.definition.block.begin

}
// <- meta.block punctuation.definition.block.end

// Guards
match n {
// <- keyword.control
    a if n > 5 => println!("Big: {}", a),
//    ^^ keyword.control
//         ^ keyword.operator
//             ^^ keyword.operator
//                ^^^^^^^^ support.macro
    b if n <= 5 => println!("Small: {}", b),
//    ^^ keyword.control
//         ^^ keyword.operator
//              ^^ keyword.operator
//                 ^^^^^^^^ support.macro
//                                  ^^ constant.other.placeholder
}

// Binding
match my_func() {
// ^^ keyword.control
//    ^^^^^^^ support.function
//              ^ meta.block punctuation.definition.block.begin
    0 => println!("None"),
//  ^ constant.numeric.integer.decimal
//    ^^ keyword.operator
//       ^^^^^^^^ support.macro
    res @ 1...9 => println!("Digit: {}", res),
//      ^ keyword.operator
//         ^^^ keyword.operator
//                                  ^^ constant.other.placeholder
    _ => println!("Full number"),
//  ^ source.rust
//    ^^ keyword.operator
}
// <- meta.block punctuation.definition.block.end

fn my_other_func(e: OperatingSystem) -> u32 {
// ^^^^^^^^^^^^^ entity.name.function
//               ^ variable.parameter
//                ^ punctuation.separator
}

// Test highlighting/scope with struct field attributes
// https://github.com/rust-lang/sublime-rust/issues/120
pub struct Claim {
//  ^^^^^^^^ meta.struct
    pub claim_id: String,
//  ^^^ storage.modifier.rust
    pub patient_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
//                               ^^^^^^^^^^^^^^^ string.quoted.double
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub referring: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
//    ^^^^^ support.function
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub drug: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub ndc: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.annotation
    pub rendering: Option<String>,
    pub date: String,

}

struct Point
// ^^^^^^^^^ meta.struct
{
// <- meta.struct meta.block punctuation.definition.block.begin
    x: i32,
//  ^ variable.other.member
//   ^ punctuation.separator
//     ^^^ storage.type
    y: i32
//  ^ variable.other.member
//   ^ punctuation.separator
//     ^^^ storage.type
}
// <-  meta.block punctuation.definition.block.end

impl Point
//^^^^^^^^ meta.impl
{
// <- meta.impl meta.block punctuation.definition.block.begin
    fn new(x: i32, y: i32) -> Point
    // <- storage.type.function
    // ^^^ entity.name.function
    {
    // <- meta.function meta.block
        Point {x: x, y: y}
    }

    fn double(&mut self) {
    // ^^^^^^ entity.name.function
        self.x *= 2;
        self.y *= 2;
    }
}

impl !Send for Point {}
//^^^^^^^^^^^^^^^^^^^^^ meta.impl.rust
//   ^ meta.impl.rust keyword.operator.rust meta.impl.opt-out.rust

pub fn pub_function() -> bool
// <- storage.modifier
//  ^^ storage.type.function
//     ^^^^^^^^^^^^ entity.name.function
{
// <- meta.function
    return true
}

let inferred_closure = |i, j: u32| i + 1;
//  ^^^^^^^^^^^^^^^^ entity.name.function
//                     ^^^^^^^^^^^^^^^^^ meta.function.closure
//                     ^^^^^^^^^^^ meta.function.parameters
//                     ^ punctuation.definition.parameters.begin
//                      ^ variable.parameter
//                         ^ variable.parameter
//                          ^ punctuation.separator
//                            ^^^ storage.type
//                               ^ punctuation.definition.parameters.end
let closure = || -> i32 { | | 1 + 2 };
//  ^^^^^^^ entity.name.function
//            ^^^^^^^^^^^^^^^^^^^^^^^ meta.function.closure
//            ^^ meta.function.parameters
//            ^ punctuation.definition.parameters.begin
//             ^ punctuation.definition.parameters.end
//                  ^^^ storage.type
//                      ^^^^^^^^^^^^^ meta.block
//                      ^ meta.block punctuation.definition.block.begin
//                            ^ constant.numeric.integer.decimal
//                                ^ constant.numeric.integer.decimal
//                                  ^ meta.block punctuation.definition.block.end

let c = a | b;
//        ^ keyword.operator

call_func(|c| 1 + 2 + c);
//        ^^^^^^^^^^^^^ meta.function.closure
//        ^^^ meta.function.parameters

macro_rules! print_result {
    ($expr:expr) => (
        println!("{:?} = {:?}", stringify!($expr), $expr)
    )
}


pub fn from_buf_reader<T>(s: io::BufReader<T>) -> Result<isize, &'static str>
//                                                              ^ keyword.operator
    where T: io::Read
//  ^ keyword.other
{
    macro_rules! eat_numbers {
        ($count:expr, /*$comment:ident,*/  $msg:expr) => {{
        //            ^^^^^^^^^^^^^^^^^^^ meta.function.rust meta.block.rust meta.macro.rust meta.block.rust meta.group.rust comment.block.rust
        //                                               ^ meta.function meta.block meta.macro meta.block meta.block punctuation.definition.block.begin
        //                                                ^ meta.function meta.block meta.macro meta.block meta.block meta.block punctuation.definition.block.begin
            let parse_err = concat!("Err parsing value in ", $msg);
            try!{ eat_numbers(&mut lines, $count, parse_err, missing_err, too_many) }
        //  ^^^^ support.macro
        //      ^ meta.function meta.block meta.macro meta.block meta.block meta.block meta.block punctuation.definition.block.begin
        }}
    };
     // <- meta.function meta.block - meta.macro

    let mut starts_stops = eat_numbers!(relief_count_total * 2, "starts and stops");

    let starts = starts_stops.split_off(relief_count_total);
    let stops = starts_stops;

    unimplemented!();
}

pub mod my_mod {
//  ^^^^^^^^^^^^ meta.module
// <- storage.modifier
//  ^^^ storage.type.module
//      ^^^^^^ entity.name.module
//             ^ meta.block punctuation.definition.block.begin
}
// <- meta.module meta.block punctuation.definition.block.end

struct Val (f64,);
struct GenVal<T>(T,);
//     ^^^^^^^^^ meta.generic
//     ^^^^^^ entity.name.struct
//           ^ punctuation.definition.generic.begin - entity.name.struct
//             ^ punctuation.definition.generic.end

// impl of Val
impl Val {
    fn value(&self) -> &'a f64 { &self.0 }
    //                 ^ keyword.operator
    //                  ^^ storage.modifier.lifetime
}

// impl of GenVal for a generic type `T`
impl <'a, T> GenVal<T> {
//   ^ punctuation.definition.generic.begin
//    ^^ storage.modifier.lifetime
//         ^ punctuation.definition.generic.end
//           ^^^^^^ entity.name.impl
//                 ^^^ - entity.name.impl
//                 ^ punctuation.definition.generic.begin
//                   ^ punctuation.definition.generic.end
    fn value(&self) -> &T { &self.0 }
    //       ^ keyword.operator
    //                 ^ keyword.operator
}

fn print_debug<T: Debug> (t: &T) {
// ^^^^^^^^^^^ entity.name.function
//            ^ punctuation.definition.generic.begin - entity.name.function
//              ^ punctuation.separator
//                     ^ punctuation.definition.generic.end
//                        ^ variable.parameter
//                           ^ keyword.operator
    println!("{:?}", t);
//            ^^^^ constant.other.placeholder
}

impl<'a, T: MyTrait + OtherTrait> PrintInOption for T where
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//   ^^ storage.modifier.lifetime
//        ^ punctuation.separator
//                  ^ keyword.operator
//                                              ^^^ keyword.other
//                                                  ^ entity.name.impl
//                                                    ^^^^^ meta.where keyword.other
    Option<T>: Debug {
//^^^^^^^^^^^^^^^^^^^^ meta.impl
    fn print_in_option(self) {
//     ^^^^^^^^^^^^^^^ entity.name.function
        println!("{:?}", Some(self));
    }
}

pub trait Animal {
// <- storage.modifier
//  ^^^^^^^^^^^^^^ meta.trait
//               ^ meta.block punctuation.definition.block.begin
    fn noise(quiet: bool) {
        // Comment
    }
}
// <- meta.trait meta.block punctuation.definition.block.end

fn collect_vec() {
    let _: Vec<(usize, usize)> = (0..10).enumerate().collect::<Vec<_>>();
//         ^^^^^^^^^^^^^^^^^^^ meta.generic
//             ^ punctuation.definition.type.begin
//              ^^^^^ storage.type
//                     ^^^^^ storage.type
//                          ^ punctuation.definition.type.end
//                                                            ^^^^^^^^ meta.generic
//                                                             ^^^^^^ meta.generic meta.generic
//                                                                 ^ keyword.operator
    let _: Vec<(usize, usize)> = vec!();
//                               ^^^^ support.macro
    let _: Vec<(usize, usize)> = vec!{};
//                               ^^^^ support.macro
    let _: Vec<(usize, usize)> = vec![];
//                               ^^^^ support.macro
}

macro_rules! forward_ref_binop [
//                             ^ meta.macro meta.group punctuation.definition.group.begin
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
//        ^^^^ variable.parameter
//             ^^^^^ storage.type
//                    ^^^^^^^ variable.parameter
//                            ^^^^^ storage.type
//                                      ^^ variable.parameter
//                                         ^^ storage.type
//                                             ^^ variable.parameter
//                                                ^^ storage.type
//                                                    ^^ keyword.operator
//                                                       ^ meta.macro meta.group meta.block punctuation.definition.block.begin
        impl<'a, 'b> $imp<&'a $u> for &'b $t {
//      ^^^^ storage.type.impl
//          ^^^^^^^^ meta.generic
//           ^^ storage.modifier.lifetime
//               ^^ storage.modifier.lifetime
//                   ^^^^ variable.other
//                       ^^^^^^^^ meta.generic
//                        ^ keyword.operator
//                         ^^ storage.modifier.lifetime
//                            ^^ variable.other
//                                ^^^ keyword.other
//                                    ^ keyword.operator
//                                     ^^ storage.modifier.lifetime
//                                        ^^ variable.other
//                                           ^ meta.macro meta.group meta.block meta.impl meta.block punctuation.definition.block.begin
            type Output = <$t as $imp<$u>>::Output;
//                        ^^^^^^^^^^^^^^^^ meta.generic
//                                        ^^ meta.path

            #[inline]
//          ^^^^^^^^^ meta.annotation.rust
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
//          ^^ storage.type.function
//             ^^^^^^^ variable.other
//                     ^^^^ variable.language
//                                  ^ keyword.operator
//                                   ^^ storage.modifier.lifetime
//                                      ^^ variable.other
//                                          ^^ punctuation.separator
//                                             ^^^^^^^^^^^^^^^^ meta.generic
//                                                             ^^ meta.path
//                                                                      ^ meta.macro meta.group meta.block meta.impl meta.block meta.block punctuation.definition.block.begin
                $imp::$method(*self, *other)
//              ^^^^ variable.other
//                    ^^^^^^^ variable.other
//                            ^ keyword.operator
//                             ^^^^ variable.language
//                                   ^ keyword.operator
            }
        }
    }
]

macro_rules! alternate_group (
//                           ^ meta.macro meta.group punctuation.definition.group.begin
    ($a:expr) => (
//   ^^ variable.parameter
//      ^^^^ storage.type
        println!("Test {}!", $a)
    )
)

macro_rules! kleene_star {
    ($($arg:tt)+) => (
//   ^ meta.macro meta.block meta.group keyword.operator
//    ^ meta.macro meta.block meta.group punctuation.definition.group.begin
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^ meta.macro meta.block meta.group
//              ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                ^ meta.macro meta.block keyword.operator
        println!($($arg));
    ),
    ($($arg:tt)*) => (
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^ meta.macro meta.block meta.group
//              ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                ^ meta.macro meta.block keyword.operator
        println!($($arg)*);
    ),
    ($($arg:tt);+) => (
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^^ meta.macro meta.block meta.group
//               ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                 ^ meta.macro meta.block keyword.operator
        println!($($arg));
    ),
    ($($arg:tt),*) => (
//     ^^^^ meta.macro meta.block meta.group variable.other
//         ^^^^^^ meta.macro meta.block meta.group
//               ^ meta.macro meta.block meta.group punctuation.definition.group.end
//                 ^ meta.macro meta.block keyword.operator
        println!($($arg)*);
    )
}

pub fn next_lex<T:PartialOrd>(/* block */data: &mut [T] // line {
//                            ^^^^^^^^^^^ source.rust meta.function.rust meta.function.parameters.rust comment.block.rust
//                                                      ^^^^^^^^^ source.rust meta.function.rust meta.function.parameters.rust comment.line.double-slash.rust
    /* block2 */ data2: &mut [T]  // line
//  ^^^^^^^^^^^^ source.rust meta.function.rust meta.function.parameters.rust comment.block.rust
//                                ^^^^^^^ source.rust meta.function.rust meta.function.parameters.rust comment.line.double-slash.rust
    ) -> bool {
    unimplemented!();
}

pub fn next_lex2</* block */T/* comments */:/* everywhere */
//               ^^^^^^^^^^^ comment.block.rust
//                           ^^^^^^^^^^^^^^ comment.block.rust
//                                          ^^^^^^^^^^^^^^^^ comment.block.rust
    // Many comments
//  ^^^^^^^^^^^^^^^^ comment.line.double-slash.rust
    /* help */ PartialOrd // Possibly too many comments
//  ^^^^^^^^^^ comment.block.rust
//                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ comment.line.double-slash.rust
> (
    /* block2 */ data2: &mut [T]  // line
//  ^^^^^^^^^^^^ source.rust meta.function.rust meta.function.parameters.rust comment.block.rust
//                                ^^^^^^^ source.rust meta.function.rust meta.function.parameters.rust comment.line.double-slash.rust
    ) -> bool {
    unimplemented!();
}

pub fn new<T>() -> Fibonacci<T>
    where T: One + Zero,
//  ^^^^^ keyword.other.rust
    for <'a> &'a T: Add<Output = T>,
//  ^^^ keyword.other.rust
//      ^ punctuation.definition.generic.begin.rust
//       ^^ storage.modifier.lifetime.rust
//         ^ punctuation.definition.generic.end.rust
//           ^ keyword.operator.rust
//            ^^ storage.modifier.lifetime.rust
{
    unimplemented!();
}

pub fn new<T>() -> Fibonacci<T>
    where for <'a> &'a T: Add<Output = T>,
//  ^^^^^ meta.where keyword.other.rust
//        ^^^ keyword.other.rust
//            ^ punctuation.definition.generic.begin.rust
//             ^^ storage.modifier.lifetime.rust
//               ^ punctuation.definition.generic.end.rust
//                 ^ keyword.operator.rust
//                  ^^ storage.modifier.lifetime.rust
{
    unimplemented!();
}

impl<T> Fibonacci<T>
    where for <'a> &'a T: Add<Output = T>,
//  ^^^^^ keyword.other.rust
//        ^^^ keyword.other.rust
//            ^ punctuation.definition.generic.begin.rust
//             ^^ storage.modifier.lifetime.rust
//               ^ punctuation.definition.generic.end.rust
//                 ^ keyword.operator.rust
//                  ^^ storage.modifier.lifetime.rust
{
    unimplemented!();
}

impl<T> Iterator for Fibonacci<T>
    where T: Clone,
//  ^^^^^ keyword.other.rust
    for <'a> &'a T: Add<Output = T>,
//  ^^^ keyword.other.rust
//      ^ punctuation.definition.generic.begin.rust
//       ^^ storage.modifier.lifetime.rust
//         ^ punctuation.definition.generic.end.rust
//           ^ keyword.operator.rust
//            ^^ storage.modifier.lifetime.rust
{
    unimplemented!();
}

pub const FOO: Option<[i32; 1]> = Some([1]);
//                    ^ punctuation.definition.group.begin.rust
//                           ^ punctuation.definition.group.end.rust

pub fn macro_tests() {
    println!();
//  ^^^^^^^^ support.macro.rust
    println!("Example");
//  ^^^^^^^^ support.macro.rust
//          ^ punctuation.definition.group.begin
//           ^^^^^^^^^ string.quoted.double.rust
//                    ^ punctuation.definition.group.end
    println!("Example {} {message}", "test", message="hi");
//                    ^^ constant.other.placeholder.rust
//                       ^^^^^^^^^ constant.other.placeholder.rust
    panic!();
//  ^^^^^^ support.macro.rust
    panic!("Example");
//  ^^^^^^ support.macro.rust
//        ^ punctuation.definition.group.begin
//         ^^^^^^^^^ string.quoted.double.rust
//                  ^ punctuation.definition.group.end
    panic!("Example {} {message}", "test", message="hi");
//                  ^^ constant.other.placeholder.rust
//                     ^^^^^^^^^ constant.other.placeholder.rust
    format_args!("invalid type: {}, expected {}", unexp, exp);
//  ^^^^^^^^^^^^ support.macro.rust
//                              ^^ constant.other.placeholder.rust
//                                           ^^ constant.other.placeholder.rust
    unreachable!("{:?}", e);
//  ^^^^^^^^^^^^ support.macro.rust
//                ^^^^ constant.other.placeholder.rust
    unimplemented!("{:?}", e);
//  ^^^^^^^^^^^^^^ support.macro.rust
//                  ^^^^ constant.other.placeholder.rust
}

#[derive(Clone)]
pub struct GobletMiddleware<B: Backend + ?Sized + 'static> {
//         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//                                                ^^^^^^^ storage.modifier.lifetime
    pub derp: Arc<Api<B>>,
}

impl<B: Backend + ?Sized + 'static> GobletMiddleware<B> {
// <- meta.impl
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//                         ^^^^^^^ storage.modifier.lifetime
//                                  ^^^^^^^^^^^^^^^^^^^ meta.generic
    pub fn new(api: Arc<Api<B>>) -> GobletMiddleware<B> {
        GobletMiddleware { derp: api }
    }
}

impl<B: Backend + ?Sized + 'static> Key for GobletMiddleware<B> {
// <- meta.impl
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//                         ^^^^^^^ storage.modifier.lifetime
//                                      ^^^ keyword.other
//                                          ^^^^^^^^^^^^^^^^ entity.name.impl
//                                                          ^^^ meta.generic
    type Value = Arc<Api<B>>;
}

impl<T> From<AsRef<T>> for CliError<T> { }
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.impl
//                         ^^^^^^^^ entity.name.impl
//                                 ^^^ meta.generic

fn legal_dates_iter() -> Box<Iterator<Item = Date<UTC>>> {
//                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function.return-type meta.generic
//                                         ^ keyword.operator
    unimplemented!()
}

fn numbers() -> impl Iterator<Item = u64> {
//              ^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function.return-type
//              ^^^^ meta.function.return-type storage.type.impl
//                   ^^^^^^^^ meta.function.return-type support.type
//                           ^^^^^^^^^^^^ meta.function.return-type meta.generic
    Generator(move || for a in (0..10) { yield a; } })
//                                       ^^^^^ keyword.control
}

pub struct IterHolder<A> where A: Number {
//                   ^^^ meta.struct meta.generic
//                       ^^^^^ meta.struct meta.where keyword.other
//                                ^^^^^^ meta.struct meta.where
//                                       ^ meta.struct punctuation.definition.block.begin.rust
    num: A
}

pub struct IterHolder<A>
//                   ^^^ meta.struct meta.generic
where
//   <- meta.struct meta.where keyword.other
    A: Number {
//     ^^^^^^ meta.struct meta.where
    num: A
}

struct A<T>(T) where T: AsRef<str>;
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.struct
//                            ^^^ meta.struct meta.where storage.type
//                                ^ punctuation.terminator
//             ^^^^^ meta.struct meta.where keyword.other
pub struct A<T>(T)
//  ^^^^^^^^^^ meta.struct
//  ^^^^^^ meta.struct storage.type
where
//^^^ meta.struct meta.where keyword.other
    T: AsRef<str>;
//^^^^^^^^^^^^^^^ meta.struct
//           ^^^ meta.struct meta.where storage.type
//               ^ punctuation.terminator

fn foo<F: FnMut(i32, i32 /*asd*/) -> i32>(f: F) {
//                       ^^^^^^^ meta.generic comment
    let lam = |time: i32 /* comment */, other: i32| {
//                       ^^^^^^^^^^^^^ meta.function.parameters comment
    };
}

// mdo example
fn main() {
    // exporting the monadic functions for the Iterator monad (similar
    // to list comprehension)
    use mdo::iter::{bind, ret, mzero};

    let l = bind(1i32..11, move |z|
                 bind(1..z, move |x|
                      bind(x..z, move |y|
                           bind(if x * x + y * y == z * z { ret(()) }
                                else { mzero() },
                                move |_|
                                ret((x, y, z))
                                )))).collect::<Vec<_>>();
    println!("{:?}", l);

    // the same thing, using the mdo! macro
    let l = mdo! {
        z =<< 1i32..11;
        x =<< 1..z;
//        ^^^ keyword.operator
        y =<< x..z;
        when x * x + y * y == z * z;
        ret ret((x, y, z))
    }.collect::<Vec<_>>();
    println!("{:?}", l);
}

union Union {
//^^^ meta.union storage.type.union
//^^^^^^^^^^^ meta.union
//    ^^^^^ entity.name.union
//          ^ meta.block punctuation.definition.block.begin
    f: u32,
//  ^ meta.union meta.block variable.other.member
//   ^ meta.union meta.block punctuation.separator
//     ^^^ meta.union meta.block storage.type
}
// <- meta.union meta.block punctuation.definition.block.end

pub union Foo<'a, Y: Baz>
// <- storage.modifier
//  ^^^^^^^^^^^^^^^^^^^^^ meta.union
//  ^^^^^ meta.union storage.type.union
//        ^^^ meta.union meta.generic entity.name.union
//           ^ meta.union meta.generic meta.generic punctuation.definition.generic.begin
//            ^^ meta.union meta.generic meta.generic storage.modifier.lifetime
    where X: Whatever,
//  ^^^^^ meta.union meta.where keyword.other
//        ^ meta.union meta.where
//         ^ meta.union meta.where punctuation.separator
//           ^^^^^^^^^^ meta.union meta.where
{
// <- meta.union meta.block punctuation.definition.block.begin
    f: SomeType, // Comment beside a field
//               ^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.union meta.block comment.line.double-slash
}
// <- meta.union meta.block punctuation.definition.block.end

// Union was implemented in such a way that `union` is not a keyword.  Verify
// that we don't accidentally interpret it as a keyword.
fn union() {}
// ^^^^^ meta.function entity.name.function

let x: __m128i = __m128i::from_bits(f32x4::from_bits(m32x4::new(true, true, true, true)));
//     ^^^^^^^ storage.type.rust
//               ^^^^^^^ storage.type.rust
//                                  ^^^^^ meta.group.rust storage.type.rust
//                                                   ^^^^^ meta.group.rust meta.group.rust storage.type.rust
//                                                              ^^^^ meta.group.rust meta.group.rust meta.group.rust constant.language.rust

// Ensure that `mut` is a storage modifier.
impl<A> Thing for &'a mut A {}
//            ^^^ meta.impl keyword.other
//                ^ meta.impl keyword.operator
//                 ^^ meta.impl storage.modifier.lifetime
//                    ^^^ meta.impl storage.modifier
//                        ^ meta.impl entity.name.impl

pub ( crate ) struct S {}
// <- storage.modifier
//  ^ punctuation.definition.group.begin
//    ^^^^^ keyword.other
//          ^ punctuation.definition.group.end
//            ^^^^^^^^^^^ meta.struct
pub ( in foo::bar ) union U {}
//  ^ punctuation.definition.group.begin
//    ^^ keyword.other
//       ^^^^^^^^ meta.path
//                ^ punctuation.definition.group.end
//                  ^^^^^^^^^^ meta.union
pub ( in foo :: bar ) type T = i32;
//  ^ punctuation.definition.group.begin
//    ^^ keyword.other
//       ^^^ meta.path
//           ^^ meta.path
//              ^^^ meta.path
//                  ^ punctuation.definition.group.end
//                    ^^^^ storage.type.type
pub ( in ::foo ) fn f() {}
//       ^^^^^ meta.path
//               ^^^^^^^^^ meta.function
pub ( self ) mod m {}
//    ^^^^ keyword.other
//           ^^^^^^^^ meta.module
pub ( super ) use a::b;
//    ^^^^^ keyword.other
//            ^^^ keyword.other
pub ( in self ) enum E {A,B}
//    ^^ keyword.other
//       ^^^^ keyword.other
//              ^^^^^^^^^^^^ meta.enum
pub ( in super ) const CONST: i32 = 1;
//    ^^ keyword.other
//       ^^^^^ keyword.other
//               ^^^^^ storage.type
pub ( in super::super ) static STATIC: i32 = 1;
//    ^^ keyword.other
//       ^^^^^ keyword.other
//            ^^ meta.path
//              ^^^^^ keyword.other
//                      ^^^^^^ storage.type

struct S {
    pub f1: i32,
//  ^^^ meta.struct storage.modifier
//      ^^ meta.struct variable.other.member
    pub(crate) f2: i32,
//  ^^^ meta.struct storage.modifier
//     ^ meta.struct punctuation.definition.group.begin
//      ^^^^^ meta.struct keyword.other
//           ^ meta.struct punctuation.definition.group.end
//             ^^ meta.struct variable.other.member
    pub(in super::foo) f3: i32,
//  ^^^ meta.struct storage.modifier
//     ^ meta.struct punctuation.definition.group.begin
//      ^^ meta.struct keyword.other
//         ^^^^^ meta.struct keyword.other
//              ^^^^^ meta.struct meta.path
//                   ^ meta.struct punctuation.definition.group.end
//                     ^^ meta.struct variable.other.member
}

struct S (
    pub i32,
//  ^^^ meta.struct storage.modifier
//      ^^^ meta.struct storage.type
    pub(crate) i32,
//  ^^^ meta.struct storage.modifier
//     ^ meta.struct punctuation.definition.group.begin
//      ^^^^^ meta.struct keyword.other
//           ^ meta.struct punctuation.definition.group.end
//             ^^^ meta.struct storage.type
    pub(in super) i32,
//  ^^^ meta.struct storage.modifier
//     ^ meta.struct punctuation.definition.group.begin
//      ^^ meta.struct keyword.other
//         ^^^^^ meta.struct keyword.other
//              ^ meta.struct punctuation.definition.group.end
//                ^^^ meta.struct storage.type
);

// Various tests on `where`.
fn f<'b: 'a>(self) -> &'b mut [i32] where 'a: 'b { }
//                 ^^^^^^^^^^^^^^ meta.function meta.function.return-type
//                            ^ meta.function meta.function.return-type punctuation.definition.group.begin
//                             ^^^ meta.function meta.function.return-type storage.type
//                                ^ meta.function meta.function.return-type punctuation.definition.group.end
//                                  ^^^^^ meta.function meta.where keyword.other
//                                        ^^ meta.function meta.where storage.modifier.lifetime
//                                          ^ meta.function meta.where punctuation.separator
//                                            ^^ meta.function meta.where storage.modifier.lifetime
//                                               ^ meta.function meta.block punctuation.definition.block.begin
//                                                 ^ meta.function meta.block punctuation.definition.block.end

fn f<F>(func: F) -> usize
//               ^^ meta.function meta.function.return-type punctuation.separator
//                  ^^^^^ meta.function meta.function.return-type storage.type
    where F: Fn(usize) -> usize {}
//  ^^^^^ meta.function meta.where keyword.other
//        ^^^^^^^^^^^^^^^^^^^^^ meta.function meta.where
//         ^ punctuation.separator
//           ^^ support.type
//             ^ punctuation.definition.type.begin
//              ^^^^^ storage.type
//                   ^ punctuation.definition.type.end
//                     ^^ meta.function.return-type punctuation.separator
//                        ^^^^^ meta.function.return-type storage.type
//                              ^ meta.function meta.block punctuation.definition.block.begin
//                               ^ meta.function meta.block punctuation.definition.block.end

fn f<L, R>(lhs: L, rhs: R)
    where L: IntoIterator<Item=(&'a i32, &'a i32)>,
//  ^^^^^ meta.function meta.where keyword.other
//        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function meta.where
//           ^^^^^^^^^^^^ support.type
//                       ^^^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//                       ^ punctuation.definition.generic.begin
//                             ^ punctuation.definition.type.begin
//                              ^ keyword.operator
//                               ^^ storage.modifier.lifetime
//                                  ^^^ storage.type
//                                       ^ keyword.operator
//                                        ^^ storage.modifier.lifetime
//                                           ^^^ storage.type
//                                              ^ punctuation.definition.type.end
//                                               ^ punctuation.definition.generic.end
          R: IntoIterator<Item=(&'a i32, &'a i32)>, {}
//        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.function meta.where
//                                                  ^ meta.function meta.block punctuation.definition.block.begin
//                                                   ^ meta.function meta.block punctuation.definition.block.end
fn f<F: Fn(usize) -> usize>(func: f) {}
//  ^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//  ^ meta.generic punctuation.definition.generic.begin
//    ^ meta.generic punctuation.separator
//      ^^ meta.generic support.type
//        ^ meta.generic punctuation.definition.type.begin
//         ^^^^^ meta.generic storage.type
//              ^ meta.generic punctuation.definition.type.end
//                   ^^^^^ meta.generic meta.function.return-type storage.type
//                        ^ meta.generic punctuation.definition.generic.end
//                         ^ meta.function meta.function.parameters punctuation.definition.parameters.begin
//                          ^^^^ meta.function meta.function.parameters variable.parameter
fn f<L: IntoIterator<Item=(&'a i32, &'a i32)>>(lhs: L) {}
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.generic
//  ^ punctuation.definition.generic.begin
//                  ^ punctuation.definition.generic.begin
//                        ^ punctuation.definition.type.begin
//                          ^^ storage.modifier.lifetime
//                             ^^^ storage.type
//                                          ^ punctuation.definition.generic.begin
//                                           ^ punctuation.definition.generic.end
//                                            ^ meta.function meta.function.parameters punctuation.definition.parameters.begin

// dyn trait
fn f(x: dyn T, y: Box<dyn T + 'static>, z: &dyn T,
//      ^^^ meta.function.parameters storage.type.trait
//                    ^^^ meta.function.parameters meta.generic storage.type.trait
//                                          ^^^ meta.function.parameters storage.type.trait
     f: &dyn Fn(CrateNum) -> bool) -> dyn T {
//       ^^^ meta.function.parameters storage.type.trait
//                                    ^^^ meta.function.return-type storage.type.trait
    let x: &(dyn 'static + Display) = &BYTE;
//           ^^^ meta.group storage.type.trait
    let y: Box<dyn Display + 'static> = Box::new(BYTE);
//             ^^^ meta.generic storage.type.trait
    let _: &dyn (Display) = &BYTE;
//          ^^^ storage.type.trait
    let _: &dyn (::std::fmt::Display) = &BYTE;
//          ^^^ storage.type.trait
    const DT: &'static dyn C = &V;
//                     ^^^ storage.type.trait
    struct S {
        f: dyn T
//         ^^^ meta.struct storage.type.trait
    }
    type D4 = dyn (::module::Trait);
//            ^^^ storage.type.trait
}

// dyn is not a keyword in all situations (a "weak" keyword).
type A0 = dyn;
//        ^^^ -storage.type.trait
type A1 = dyn::dyn;
//        ^^^^^ meta.path -storage.type.trait
//             ^^^ -storage.type.trait
type A2 = dyn<dyn, dyn>;
//        ^^^ meta.generic -storage.type.trait
//            ^^^ meta.generic -storage.type.trait
//                 ^^^ meta.generic -storage.type.trait
// This is incorrect.  `identifier` should not match on the keyword `as`.
// However, avoiding keywords is a little complicated and slow.
type A3 = dyn<<dyn as dyn>::dyn>;
//        ^^^ meta.generic -storage.type.trait
//             ^^^ meta.generic storage.type.trait
//                    ^^^ meta.generic -storage.type.trait
//                          ^^^ meta.generic -storage.type.trait

fn lambdas() {
    let c = |foo,
//          ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.begin
//           ^^^ meta.function.parameters variable.parameter
             bar| {};
//           ^^^ meta.function.parameters variable.parameter
//              ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.end
    let c = |foo,  // weird, but should work
//          ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.begin
//           ^^^ meta.function.parameters variable.parameter
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^ comment.line
             bar| {};
//           ^^^ meta.function.parameters variable.parameter
//              ^ meta.function.closure meta.function.parameters punctuation.definition.parameters.end
}
