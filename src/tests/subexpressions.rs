use super::{ParseError, a, b};

#[test]
fn single() {
   rule!(rule:u32 = (a));
   assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn labeled() {
    rule!(rule:u32 = x:(a));
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn labeled_into_action() {
    rule!(rule:u32 = x:(a) => { x });
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn sequence() {
    rule!(rule:u32 = (a) (b));
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn containing_choice() {
    rule!(rule:u32 = (a / b));
    assert_eq!(rule("a"), Ok((1, "")));
    assert_eq!(rule("b"), Ok((2, "")));
    assert_eq!(rule("c"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}

#[test]
fn nested() {
    rule!(rule:u32 = ((a)));
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn into_action_sequence() {
    rule!(rule:u32 = (a) => { 0 } b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn into_action_into_choice() {
    rule!(rule:u32 = (a) => { 10 } / (b) => { 11 });
    assert_eq!(rule("a"), Ok((10, "")));
    assert_eq!(rule("b"), Ok((11, "")));
    assert_eq!(rule("c"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}
