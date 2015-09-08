use super::{a, b};

#[test]
fn parser_option() {
    rule!(rule:Option<u32> = a?);
    assert_eq!(rule("a"), Ok((Some(1), "")));
    assert_eq!(rule("b"), Ok((None, "b")));
    assert_eq!(rule(""), Ok((None, "")));
}

#[test]
fn subexpression_option() {
    rule!(rule:Option<u32> = (a)?);
    assert_eq!(rule("a"), Ok((Some(1), "")));
}

#[test]
fn parser_option_in_subexpression() {
    rule!(rule:Option<u32> = (a?));
    assert_eq!(rule("a"), Ok((Some(1), "")));
}

#[test]
fn subexpression_option_in_subexpression() {
    rule!(rule:Option<u32> = ((a)?));
    assert_eq!(rule("a"), Ok((Some(1), "")));
}

#[test]
fn parser_option_first_in_sequence() {
    rule!(rule:u32 = a? b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn subexpression_option_first_in_sequence() {
    rule!(rule:u32 = (a)? b);
    assert_eq!(rule("ab"), Ok((2, "")));
}

#[test]
fn parser_option_option() {
    rule!(rule:Option<Option<u32>> = a??);
    assert_eq!(rule("a"), Ok((Some(Some(1)), "")));
}

#[test]
fn parser_option_into_choice() {
    rule!(rule:Option<u32> = a? / b?);
    assert_eq!(rule("a"), Ok((Some(1), "")));
}

#[test]
fn subexpresion_option_in_subexpression_option() {
    rule!(rule:Option<Option<u32>> = ((a)?)?);
    assert_eq!(rule("a"), Ok((Some(Some(1)), "")));
}

#[test]
fn string_option() {
    rule!(rule:Option<&str> = ["a"]?);
    assert_eq!(rule("a"), Ok((Some("a"), "")));
    assert_eq!(rule("b"), Ok((None, "b")));
    assert_eq!(rule(""), Ok((None, "")));
}

#[test]
fn string_option_into_sequence() {
    rule!(rule:u32 = ["a"]? b);
    assert_eq!(rule("ab"), Ok((2, "")));
}
#[test]
fn string_option_into_choice() {
    rule!(rule:Option<&str> = ["a"]? / ["b"]?);
    assert_eq!(rule("a"), Ok((Some("a"), "")));
}

#[test]
fn labeled_parser_option() {
    rule!(rule:Option<u32> = x:a?);
    assert_eq!(rule("a"), Ok((Some(1), "")));
}

#[test]
fn subexpression_option_containing_label() {
    rule!(rule:Option<u32> = (x:a)?);
    assert_eq!(rule("a"), Ok((Some(1), "")));
    assert_eq!(rule("b"), Ok((None, "b")));
}

#[test]
fn labeled_parser_into_action_in_sub_optional() {
    rule!(rule:Option<u32> = (x:a => { x })?);
    assert_eq!(rule("a"), Ok((Some(1), "")));
    assert_eq!(rule("b"), Ok((None, "b")));
}

#[test]
fn labeled_parser_option_into_action() {
    rule!(rule:Option<u32> = x:a? => { x });
    assert_eq!(rule("a"), Ok((Some(1), "")));
    assert_eq!(rule("b"), Ok((None, "b")));
}

#[test]
fn labeled_parser_option_into_sequence_into_action() {
    rule!(rule:Option<u32> = x:a? b => { x });
    assert_eq!(rule("ab"), Ok((Some(1), "")));
    assert_eq!(rule("b"), Ok((None, "")));
}

//TODO: Labeled string options
