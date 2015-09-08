use super::{a, b};
use super::ParseError;

#[test]
fn parser_plus() {
    rule!(rule:Vec<u32> = a+);
    assert_eq!(rule("a"), Ok((vec![1], "")));
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
    assert_eq!(rule("b"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}

#[test]
fn subexpression_plus() {
    rule!(rule:Vec<u32> = (a)+);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn labeled_parser_plus() {
    rule!(rule:Vec<u32> = x:a+);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn labeled_subexpression_plus() {
    rule!(rule:Vec<u32> = x:(a)+);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn parser_plus_in_action() {
    rule!(rule:Vec<u32> = x:a+ => { x });
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn subexpression_plus_in_action() {
    rule!(rule:Vec<u32> = x:(a)+ => { x });
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn subexpession_plus_into_action_into_sequence() {
    rule!(rule:u32 = (a)+ => { 0 } b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn subexpession_plus_into_action_into_choice() {
    rule!(rule:u32 = (a)+ => { 0 } / b);
    assert_eq!(rule("aaa"), Ok((0, "")));
}

#[test]
fn labeled_subexpession_plus_into_action_into_choice() {
    rule!(rule:Vec<u32> = x:(a)+ => { x } / b+);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn parser_plus_in_subexpression() {
    rule!(rule:Vec<u32> = (a+));
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn subexpression_plus_in_subexpression() {
    rule!(rule:Vec<u32> = ((a)+));
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn parser_plus_first_in_pair() {
    rule!(rule:u32 = a+ b);
    assert_eq!(rule("aaab"), Ok((2, "")));
}

#[test]
fn subexpression_plus_first_in_pair() {
    rule!(rule:u32 = (a)+ b);
    assert_eq!(rule("aaab"), Ok((2, "")));
}

#[test]
fn parser_plus_last_in_pair() {
    rule!(rule:Vec<u32> = a b+);
    assert_eq!(rule("abbb"), Ok((vec![2, 2, 2], "")));
}

#[test]
fn parser_plus_in_subexpression_optional() {
    rule!(rule:Option<Vec<u32>> = (a+)?);
    assert_eq!(rule("aaa"), Ok((Some(vec![1, 1, 1]), "")));
}

#[test]
fn parser_plus_plus() {
    rule!(rule:Vec<Vec<u32>> = a++);
    assert_eq!(rule("aaa"), Ok((vec![vec![1, 1, 1]], "")));
}

#[test]
fn parser_plus_in_subexpression_plus() {
    rule!(rule:Vec<Vec<u32>> = (a+)+);
    assert_eq!(rule("aaa"), Ok((vec![vec![1, 1, 1]], "")));
}

#[test]
fn string_plus() {
    rule!(rule:Vec<&str> = ["a"]+);
    assert_eq!(rule("aaa"), Ok((vec!["a", "a", "a"], "")));
}

#[test]
fn parser_plus_into_choice() {
    rule!(rule:Vec<u32> = a+ / b+);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn labeled_parser_plus_into_action() {
    rule!(rule:Vec<u32> = x:a+ => { x });
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
    assert_eq!(rule("b"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}

#[test]
fn labeled_parser_plus_into_sequence_into_action() {
    rule!(rule:Vec<u32> = x:a+ b => { x });
    assert_eq!(rule("aaab"), Ok((vec![1, 1, 1], "")));
    assert_eq!(rule("aaa"), Err(ParseError));
    assert_eq!(rule("c"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}

#[test]
fn subexpression_plus_into_choice() {
    rule!(rule:Vec<u32> = (a)+ / (b)+);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn string_plus_into_choice() {
    rule!(rule:Vec<&str> = ["a"]+ / ["b"]+);
    assert_eq!(rule("aaa"), Ok((vec!["a", "a", "a"], "")));
    assert_eq!(rule(""), Err(ParseError));
}
