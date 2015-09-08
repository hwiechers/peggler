// Adapted from https://marwes.github.io/2015/08/28/combine-1.0.0.html
#[macro_use]
extern crate peggler;

use std::collections::HashMap;

use peggler::{ParseError, ParseResult};

#[derive(PartialEq, Debug)]
pub struct Ini {
    pub global: HashMap<String, String>,
    pub sections: HashMap<String, HashMap<String, String>>
}

fn any_char(input: &str) -> ParseResult<char> {
    let mut char_indices = input.char_indices();
    match char_indices.next() {
        Some((_, char)) => match char_indices.next() {
            Some((index, _)) => Ok((char, &input[index..])),
            None => Ok((char, &""[..])),
        },
        None => Err(ParseError),
    }
}

rule!(property:(String, String) =
        key: (!(["="] / ["["] / [";"]) any_char)+
        ["="]
        value: (!(["\n"] / [";"]) any_char)+
        => { (key.into_iter().collect(),
              value.into_iter().collect()) });

rule!(comment:() = [";"] (!(["\n"]) any_char)* => { () });
rule!(whitespace:() =
      ((([" "] / ["\t"] / ["\r"]/ ["\n"]) => { () }) / comment)*
      => { () });

rule!(properties:HashMap<String, String> =
      props:(prop:property whitespace => { prop })*
      => { props.into_iter().collect() } );

rule!(section:(String, HashMap<String, String>) =
      title: (["["] title:((!(["]"]) any_char))+ ["]"] => { title })
      whitespace
      props: properties
      => { (title.into_iter().collect(), props) });

rule!(ini:Ini = whitespace global:properties sections:section*
      => {Ini { global: global,
                sections: sections.into_iter().collect()
           }});

#[test]
fn test_property() {
    assert_eq!(
        property("key=value"),
        Ok((("key".to_string(), "value".to_string()), "")));
}

#[test]
fn test_comment() {
    assert_eq!(comment(";abc abc"), Ok(((), "")));
    assert_eq!(comment(";abc abc\n"), Ok(((), "\n")));
}

#[test]
fn test_whitespace() {
    assert_eq!(whitespace(" \t\r\n"), Ok(((), "")));
    assert_eq!(whitespace(";abc abc\n"), Ok(((), "")));
}

#[test]
fn test_properties() {
    let mut expected = HashMap::new();
    expected.insert("key1".to_string(), "value1".to_string());
    expected.insert("key2".to_string(), "value2".to_string());

    assert_eq!(properties("key1=value1\nkey2=value2"), Ok((expected, "")));
}

#[test]
fn test_section() {
    let input = "[section]\nkey1=value1\nkey2=value2";

    let mut props = HashMap::new();
    props.insert("key1".to_string(), "value1".to_string());
    props.insert("key2".to_string(), "value2".to_string());

    assert_eq!(
        section(input),
        Ok((("section".to_string(), props), "")));
}

#[test]
fn test_ini() {
    let input = r#"
        key=value

        [section]
        key1=value1; Comment
        key2=value2
        "#;
    let mut expected = Ini {
        global: HashMap::new(),
        sections: HashMap::new()
    };
    expected.global.insert("key".to_string(), "value".to_string());

    let mut section = HashMap::new();
    section.insert("key1".to_string(), "value1".to_string());
    section.insert("key2".to_string(), "value2".to_string());
    expected.sections.insert(String::from("section"), section);

    assert_eq!(ini(input), Ok((expected, "")));
}
