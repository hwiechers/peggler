use super::{ParseError, a, b};

#[test]
fn not_parser() {
    rule!(rule:() = !a);
    assert_eq!(rule("a"), Err(ParseError));
    assert_eq!(rule("b"), Ok(((), "b")));
    assert_eq!(rule(""), Ok(((), "")));
}

#[test]
fn not_parser_into_parser() {
    rule!(rule:u32 = !a b);
    assert_eq!(rule("a"), Err(ParseError));
    assert_eq!(rule("b"), Ok((2, "")));
}

#[test]
fn parser_into_not_parser() {
    rule!(rule:() = a !b);
    assert_eq!(rule("a"), Ok(((), "")));
    assert_eq!(rule("ac"), Ok(((), "c")));
    assert_eq!(rule("ab"), Err(ParseError));
}

#[test]
fn subexpression_into_not_parser() {
    rule!(rule:() = (a) !b);
    assert_eq!(rule("ac"), Ok(((), "c")));
}

#[test]
fn parser_into_not_parser_into_action() {
    rule!(rule:u32 = a !b => { 0 });
    assert_eq!(rule("ac"), Ok((0, "c")));
}

#[test]
fn subexpression_into_not_parser_into_action() {
    rule!(rule:u32 = (a) !b => { 0 });
    assert_eq!(rule("ac"), Ok((0, "c")));
}

#[test]
#[allow(unused_variables)]
fn labeled_parser_into_not_parser() {
    rule!(rule:() = x:a !b);
    assert_eq!(rule("ac"), Ok(((), "c")));
}
#[test]
fn labeled_parser_into_not_parser_into_action() {
    rule!(rule:u32 = x:a !b => { x });
    assert_eq!(rule("ac"), Ok((1, "c")));
}

#[test]
fn labeled_subexpression_into_not_parser_into_action() {
    rule!(rule:u32 = x:(a) !b => { x });
    assert_eq!(rule("ac"), Ok((1, "c")));
}
