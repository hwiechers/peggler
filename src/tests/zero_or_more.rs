use super::{ParseError, a, b};

#[test]
fn parser_star() {
    rule!(rule:Vec<u32> = a*);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
    assert_eq!(rule("b"), Ok((vec![], "b")));
    assert_eq!(rule(""), Ok((vec![], "")));
}

#[test]
fn subexpression_star() {
    rule!(rule:Vec<u32> = (a)*);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn labeled_parser_star() {
    rule!(rule:Vec<u32> = x:a*);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn labeled_subexpression_star() {
    rule!(rule:Vec<u32> = x:(a)*);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn parser_star_in_subexpression() {
    rule!(rule:Vec<u32> = (a*));
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn parser_star_first_in_pair() {
    rule!(rule:u32 = a* b);
    assert_eq!(rule("aaab"), Ok((2, "")));
}

#[test]
fn subexpression_star_first_in_pair() {
    rule!(rule:u32 = (a)* b);
    assert_eq!(rule("aaab"), Ok((2, "")));
}

#[test]
fn string_star() {
    rule!(rule:Vec<&str> = ["a"]*);
    assert_eq!(rule("aaa"), Ok((vec!["a", "a", "a"], "")));
}

#[test]
fn parser_star_into_choice() {
    rule!(rule:Vec<u32> = a* / b*);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn string_star_into_choice() {
    rule!(rule:Vec<&str> = ["a"]* / ["b"]*);
    assert_eq!(rule("aaa"), Ok((vec!["a", "a", "a"], "")));
    assert_eq!(rule(""), Ok((vec![], "")));
}

#[test]
fn subexpression_star_into_choice() {
    rule!(rule:Vec<u32> = (a)* / (b)*);
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
}

#[test]
fn labeled_parser_star_into_action() {
    rule!(rule:Vec<u32> = x:a* => { x });
    assert_eq!(rule("aaa"), Ok((vec![1, 1, 1], "")));
    assert_eq!(rule("b"), Ok((vec![], "b")));
    assert_eq!(rule(""), Ok((vec![], "")));
}

#[test]
fn labeled_parser_star_into_sequence_into_action() {
    rule!(rule:Vec<u32> = x:a* b => { x });
    assert_eq!(rule("aaab"), Ok((vec![1, 1, 1], "")));
    assert_eq!(rule("aaa"), Err(ParseError));
    assert_eq!(rule("c"), Err(ParseError));
    assert_eq!(rule(""), Err(ParseError));
}
