use super::{ParseError, a, b, c};

#[test]
fn and_parser() {
    rule!(rule:() = &a);
    assert_eq!(rule("a"), Ok(((), "a")));
    assert_eq!(rule("b"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}

#[test]
fn and_parser_into_sequence() {
    rule!(rule:u32 = &a a);
    assert_eq!(rule("a"), Ok((1, "")));
    assert_eq!(rule("b"), Err(ParseError));
}

#[test]
fn parser_into_and_parser() {
    rule!(rule:() = a & b);
    assert_eq!(rule("a"), Err(ParseError));
    assert_eq!(rule("ac"), Err(ParseError));
    assert_eq!(rule("ab"), Ok(((), "b")));
}

#[test]
fn subexpression_into_and_parser() {
    rule!(rule:() = (a) & b);
    assert_eq!(rule("ab"), Ok(((), "b")));
}

#[test]
fn parser_into_and_parser_into_action() {
    rule!(rule:u32 = a & b => { 0 });
    assert_eq!(rule("ab"), Ok((0, "b")));
}

#[test]
fn subexpression_into_and_parser_into_action() {
    rule!(rule:u32 = (a) & b => { 0 });
    assert_eq!(rule("ab"), Ok((0, "b")));
}

#[test]
#[allow(unused_variables)]
fn labeled_parser_into_and_parser() {
    rule!(rule:() = x:a & b);
    assert_eq!(rule("ab"), Ok(((), "b")));
}

#[test]
fn labeled_parser_into_and_parser_into_action() {
    rule!(rule:u32 = x:a & b => { x });
    assert_eq!(rule("ab"), Ok((1, "b")));
}

#[test]
fn labeled_parser_into_and_parser_into_sequence_into_action() {
    rule!(rule:u32 = x:a & b b => { x });
    assert_eq!(rule("ab"), Ok((1, "")));
}

#[test]
fn labeled_parser_and_parser_into_labeled_parser_into_action() {
    rule!(rule:u32 = x:a &b y:b => { 10 * x + y });
    assert_eq!(rule("ab"), Ok((12, "")));
    assert_eq!(rule("a"), Err(ParseError));
}

#[test]
fn parser_into_and_subexpression() {
    rule!(rule:() = a & (b));
    assert_eq!(rule("ab"), Ok(((), "b")));
}

#[test]
fn parser_into_and_parser_into_and_parser() {
    rule!(rule:() = a & b & b);
    assert_eq!(rule("ab"), Ok(((), "b")));
}

#[test]
fn and_subexpression_into_sequence() {
    rule!(rule:u32 = (a & b) b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn parser_into_and_into_sequence() {
    rule!(rule:u32 = a & b b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn parser_and_parser_into_choice() {
    rule!(rule:() = a & b / (c => { () }));
    assert_eq!(rule("ab"), Ok(((), "b")));
    assert_eq!(rule("ac"), Ok(((), "")));
}
