use super::{a, b, c};
use super::ParseError;

#[test]
fn single_choice() {
    rule!(rule:u32 = a / b);
    assert_eq!(rule("a"), Ok((1, "")));
    assert_eq!(rule("b"), Ok((2, "")));
    assert_eq!(rule("c"), Err(ParseError));
}

#[test]
fn double_choice() {
    rule!(rule:u32 = a / b / c);
    assert_eq!(rule("c"), Ok((3, "")));
}

//TODO: Test early exit i.e. choice is ordered

#[test]
fn labeled() {
    rule!(rule:u32 = x: (a / b) => { x });
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn labeled_first_alternative() {
    //Note that `x` can't be used in the action because the
    //scope of `x` is limitted to the first branch
    rule!(rule:u32 = (x: a) / b => { 0 });
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn labeled_second_alternative() {
    //Again `x` can't be used in the action
    rule!(rule:u32 = a / (x: b) / c => { 0 });
    assert_eq!(rule("b"), Ok((2, "")));
}

#[test]
fn labeled_first_alternative_no_parens() {
    //This rule is equivalent to
    //rule!(rule:u32 = (x: a) / b => { 0 });
    rule!(rule:u32 = x: a / b => { 0 });
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn labeled_second_alternative_no_parens() {
    //This rule is equivalent to
    //rule!(rule:u32 = a / (x: b) / c => { 0 });
    rule!(rule:u32 = a / x: b / c => { 0 });
    assert_eq!(rule("b"), Ok((2, "")));
}

#[test]
fn first_alternative_with_action() {
    rule!(rule:u32 = (x: a => { x }) / b);
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn first_alternative_with_action_no_parens() {
    rule!(rule:u32 = x:a => { x } / b);
    assert_eq!(rule("a"), Ok((1, "")));
}
