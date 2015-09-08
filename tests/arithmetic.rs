// Adapted from PEG.js example - https://github.com/pegjs/pegjs/blob/master/examples/arithmetics.pegjs

#[macro_use]
extern crate peggler;

use peggler::{ParseError, ParseResult};

fn digits(input: &str) -> ParseResult<&str> {
    let mut end : usize;

    let mut char_indices = input.char_indices();
    loop {
        match char_indices.next() {
            Some((index, char)) => {
                if !char.is_digit(10) {
                    end = index;
                    break;
                }
            },
            None => {
                end = input.len();
                break;
            }
        }
    }

    if end > 0 {
        Ok((&input[..end], &input[end..]))
    } else {
        Err(ParseError)
    }
}

rule!(expression:i32 =
        first:term
        rest: (ws op:(["+"] / ["-"]) ws term:term => {(op, term)})*
        => {
            rest.iter().fold(first, |acc, &item| {
                let &(op, term) = &item;
                match op {
                    "+" => acc + term,
                    "-" => acc - term,
                    _ => unreachable!()
                }
            })
        }
);

rule!(term:i32 =
        first:factor
        rest:(ws op:(["*"] / ["/"]) ws factor:factor => {(op, factor)})*
        => {
            rest.iter().fold(first, |acc, &item| {
                let &(op, factor) = &item;
                match op {
                    "*" => acc * factor,
                    "/" => acc / factor,
                    _ => unreachable!()
                }
            })
        }
);

rule!(factor:i32 = (["("] ws expr:expression ws [")"] => { expr }) / integer);

rule!(integer:i32 = digits:digits => { digits.parse::<i32>().unwrap() });

rule!(ws:() = ([" "] / ["\t"] / ["\n"] / ["\r"])* => { () });

#[test]
fn test_digits() {
    assert_eq!(digits(""), Err(ParseError));
    assert_eq!(digits("x"), Err(ParseError));

    assert_eq!(digits("0"), Ok(("0", "")));
    assert_eq!(digits("1"), Ok(("1", "")));
    assert_eq!(digits("9"), Ok(("9", "")));

    assert_eq!(digits("123"), Ok(("123", "")));

    assert_eq!(digits("123x"), Ok(("123", "x")));
}

#[test]
fn test_expression() {
    assert_eq!(expression("1 + 2"), Ok((3, "")));
    assert_eq!(expression("1 + 2 + 3"), Ok((6, "")));
    assert_eq!(expression("1 - 2"), Ok((-1, "")));
    assert_eq!(expression("1 - 2 - 3"), Ok((-4, "")));
    assert_eq!(expression("1 - 2 + 3"), Ok((2, "")));

    assert_eq!(expression("1 + 2 * 3"), Ok((7, "")));
    assert_eq!(expression("1 + (2 * 3)"), Ok((7, "")));
    assert_eq!(expression("(1 + 2) * 3"), Ok((9, "")));
}

#[test]
fn test_term() {
    assert_eq!(term("1"), Ok((1, "")));
    assert_eq!(term("1 * 2"), Ok((2, "")));
    assert_eq!(term("1 * 2 * 3"), Ok((6, "")));
    assert_eq!(term("1 / 2"), Ok((0, "")));
    assert_eq!(term("3 / 2 / 1"), Ok((1, "")));
    assert_eq!(term("3 / 2 * 2"), Ok((2, "")));
}

#[test]
fn test_factor() {
    assert_eq!(term("1"), Ok((1, "")));
    assert_eq!(term("(1)"), Ok((1, "")));
    assert_eq!(term("((1))"), Ok((1, "")));
}

#[test]
fn test_integer() {
    assert_eq!(integer("0"), Ok((0, "")));
    assert_eq!(integer("1"), Ok((1, "")));
    assert_eq!(integer("01"), Ok((1, "")));
    assert_eq!(integer("12"), Ok((12, "")));
    assert_eq!(integer("123"), Ok((123, "")));
}

#[test]
fn test_ws() {
    assert_eq!(ws(" "), Ok(((), "")));
    assert_eq!(ws("  "), Ok(((), "")));
    assert_eq!(ws(" \t\n\r"), Ok(((), "")));
}
