use super::ParseResult;

#[test]
fn result_type_u32() {
    fn parser(_:&str) -> ParseResult<u32> {
        Ok((1, ""))
    }

    rule!(rule:u32 = parser);

    assert_eq!(rule(""), Ok((1, "")));
}

#[test]
fn result_type_vec_u32() {
    fn parser(_: &str) -> ParseResult<Vec<u32>> {
        Ok((vec![1, 2], ""))
    }

    rule!(rule:Vec<u32> = parser);

    assert_eq!(rule(""), Ok((vec![1, 2], "")));
}

#[test]
fn result_type_pair_u32() {
    fn parser(_: &str) -> ParseResult<(u32, u32)> {
        Ok(((1, 2), ""))
    }

    rule!(rule:(u32, u32) = parser);

    assert_eq!(rule(""), Ok(((1, 2), "")));
}
