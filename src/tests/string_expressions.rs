use super::{a, b};

#[test]
fn single_short() {
    rule!(rule:&str = ["a"]);
    assert_eq!(rule("a"), Ok(("a", "")));
}

#[test]
fn single_long() {
    rule!(rule:&str = ["abc"]);
    assert_eq!(rule("abc"), Ok(("abc", "")));
}

#[test]
fn sequence() {
    rule!(rule:&str = ["a"] ["b"]);
    assert_eq!(rule("ab"), Ok(("b", "")));
}

#[test]
fn sequence_mixed() {
    rule!(rule:u32 = ["a"] a);
    assert_eq!(rule("aa"), Ok((1, "")));
}

#[test]
fn into_action() {
    rule!(rule:u32 = ["a"] => { 0 });
    assert_eq!(rule("a"), Ok((0, "")));
}

#[test]
fn into_action_into_choice() {
    rule!(rule:u32 = ["a"] => { 10 } / ["b"] => { 11 });
    assert_eq!(rule("a"), Ok((10, "")));
}

#[test]
fn labeled() {
    rule!(rule:&str = x:["a"] => { x });
    assert_eq!(rule("a"), Ok(("a", "")));
}

#[test]
fn chaining() {
    rule!(rule:u32 = ["a"] x:b => { x });
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn chaining_labeled() {
    rule!(rule:&str = x:["a"] b => { x });
    assert_eq!(rule("ab"), Ok(("a", "")));
}

#[test]
fn chained() {
    rule!(rule:u32 = x:a ["b"] => { x });
    assert_eq!(rule("ab"), Ok((1, "")));
}

#[test]
fn first_alternative() {
    rule!(rule:&str = ["a"] / ["b"]);
    assert_eq!(rule("a"), Ok(("a", "")));
}
