use super::{a, b, c};

#[test]
fn single_expression() {
    rule!(rule:u32 = a);
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn pair() {
    rule!(rule:u32 = a b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn parser_into_action() {
    rule!(rule:u32 = a => { 0 } b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn single_actioned() {
    rule!(rule:u32 = x:a => { x });
    assert_eq!(rule("a"), Ok((1, "")));
}

#[test]
fn pair_first_labeled() {
    rule!(rule:u32 = x:a b => { x });
    assert_eq!(rule("ab"), Ok((1, "")));
}

#[test]
fn pair_last_labeled() {
    rule!(rule:u32 = a y:b => { y });
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn pair_both_labeled() {
    rule!(rule:u32 = x:a y:b => { x + y });
    assert_eq!(rule("ab"), Ok((3, "")));
}

#[test]
fn triple_middle_labeled() {
    rule!(rule:u32 = a x:b c => { x });
    assert_eq!(rule("abc"), Ok((2, "")));
}

#[test]
fn triple_fist_labeled() {
    rule!(rule:u32 = x:a b c => { x });
    assert_eq!(rule("abc"), Ok((1, "")));
}
