use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::complete::{alphanumeric1, char, multispace0, multispace1},
    character::{is_alphabetic, is_alphanumeric},
    combinator::{all_consuming, map, opt, verify},
    multi::{many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Children {
    Elements(Vec<Element>),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Children,
}

pub fn get_attr<T: std::str::FromStr>(element: &Element, key: &str) -> Option<T> {
    element.attributes.get(key).and_then(|v| v.parse().ok())
}

pub fn get_required_attr<T: std::str::FromStr>(element: &Element, key: &str) -> Result<T, Error> {
    get_attr(element, key)
        .ok_or_else(|| Error::RequiredAttribute(element.name.to_string(), key.to_string()))
}

pub fn validate_name(element: &Element, expected_name: &str) -> Result<(), Error> {
    if element.name == expected_name {
        Ok(())
    } else {
        Err(Error::NotSupportElement(
            element.name.to_string(),
            expected_name.to_string(),
        ))
    }
}

fn trimmed<'a, F, T>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, T>
where
    F: FnMut(&'a str) -> IResult<&'a str, T>,
{
    delimited(multispace0, inner, multispace0)
}

fn tag_name(input: &str) -> IResult<&str, &str> {
    verify(alphanumeric1, |s: &str| is_alphabetic(s.as_bytes()[0]))(input)
}

fn attribute_value(input: &str) -> IResult<&str, &str> {
    take_while(|chr: char| is_alphanumeric(chr as u8) || chr == '-')(input)
}

fn quoted_attribute_value(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), attribute_value, char('"'))(input)
}

fn attribute(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(tag_name, char('='), quoted_attribute_value)(input)
}

fn attributes(input: &str) -> IResult<&str, HashMap<String, String>> {
    map(separated_list0(multispace1, attribute), |attributes| {
        attributes
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    })(input)
}

fn tag_name_with_attributes(input: &str) -> IResult<&str, Element> {
    let parser = pair(tag_name, opt(preceded(multispace1, attributes)));
    map(parser, |(name, attributes)| Element {
        name: name.to_string(),
        attributes: attributes.unwrap_or_default(),
        children: Children::Text("".to_string()),
    })(input)
}

fn child_elements(input: &str) -> IResult<&str, Children> {
    let elements = map(many1(element), Children::Elements);
    let text = map(take_till(|c| c == '<'), |s: &str| {
        Children::Text(s.to_string())
    });
    alt((elements, text))(input)
}

fn parent_element(input: &str) -> IResult<&str, Element> {
    let (input, mut parent) = trimmed(delimited(
        char('<'),
        trimmed(tag_name_with_attributes),
        char('>'),
    ))(input)?;

    let (input, children) = child_elements(input)?;
    parent.children = children;

    let (input, _) = trimmed(delimited(
        tag("</"),
        trimmed(tag(&parent.name[..])),
        char('>'),
    ))(input)?;

    Ok((input, parent))
}

fn single_element(input: &str) -> IResult<&str, Element> {
    trimmed(delimited(
        char('<'),
        trimmed(tag_name_with_attributes),
        tag("/>"),
    ))(input)
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((single_element, parent_element))(input)
}

pub fn parse(input: &str) -> Result<Element, nom::Err<nom::error::Error<&str>>> {
    all_consuming(element)(input).map(|result| result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_when_a_element_is_single() {
        assert_eq!(
            parse("<A />").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("".to_string())
            }
        );
    }

    #[test]
    fn it_can_parse_when_there_are_some_spaces() {
        assert_eq!(
            parse("  <A />  ").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("".to_string())
            }
        );
    }

    #[test]
    fn it_can_parse_when_there_is_no_space_between_tag_name_and_slash() {
        assert_eq!(
            parse("<A/>").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("".to_string())
            }
        );
    }

    #[test]
    fn it_can_parse_when_a_element_has_whitespaces() {
        assert_eq!(
            parse("<  A  />").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("".to_string()),
            }
        );
    }

    #[test]
    fn it_can_parse_when_a_element_has_no_children() {
        assert_eq!(
            parse("<A></A>").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("".to_string()),
            }
        );
    }

    #[test]
    fn it_can_parse_when_a_element_has_attributes() {
        assert_eq!(
            parse("<A a=\"b\" />").unwrap(),
            Element {
                name: String::from("A"),
                attributes: [(String::from("a"), String::from("b"))]
                    .into_iter()
                    .collect(),
                children: Children::Text("".to_string())
            }
        );
    }

    #[test]
    fn it_can_parse_when_attributes_have_hyphen() {
        assert_eq!(
            parse("<A a=\"b-c\" />").unwrap(),
            Element {
                name: String::from("A"),
                attributes: [(String::from("a"), String::from("b-c"))]
                    .into_iter()
                    .collect(),
                children: Children::Text("".to_string())
            }
        );
    }

    #[test]
    fn it_can_parse_when_element_has_children() {
        assert_eq!(
            parse("<A><B /></A>").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Elements(vec![Element {
                    name: String::from("B"),
                    attributes: HashMap::new(),
                    children: Children::Text("".to_string()),
                }]),
            }
        );
    }

    #[test]
    fn it_can_parse_when_element_has_grandchildren() {
        assert_eq!(
            parse("<A><B><C></C></B></A>").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Elements(vec![Element {
                    name: String::from("B"),
                    attributes: HashMap::new(),
                    children: Children::Elements(vec![Element {
                        name: String::from("C"),
                        attributes: HashMap::new(),
                        children: Children::Text("".to_string()),
                    }]),
                }]),
            }
        );
    }

    #[test]
    fn it_can_parse_when_child_is_text() {
        assert_eq!(
            parse("<A>text</A>").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("text".to_string()),
            }
        );
    }

    #[test]
    fn it_can_parse_when_child_is_multi_byte_text() {
        assert_eq!(
            parse("<A>テスト</A>").unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("テスト".to_string()),
            }
        );
    }

    #[test]
    fn it_can_parse_when_it_has_newlines_and_whitespaces() {
        let text = r#"
            <A></A>
        "#;
        assert_eq!(
            parse(text).unwrap(),
            Element {
                name: String::from("A"),
                attributes: HashMap::new(),
                children: Children::Text("".to_string()),
            }
        );
    }

    #[test]
    fn it_cannot_parse_when_child_has_text_and_tags() {
        assert!(parse("<A>text<B></B></A>").is_err());
    }

    #[test]
    fn it_cannot_parse_when_a_pair_is_broken() {
        assert!(parse("<A><B><C></B></C></A>").is_err());
    }

    #[test]
    fn it_has_multiple_roots() {
        assert!(parse("<A /><B />").is_err());
    }

    #[test]
    fn it_has_multiple_roots_without_using_self_closing() {
        assert!(parse("<A></A><B></B>").is_err());
    }

    #[test]
    fn it_cannot_parse_when_a_element_has_no_a_close_tag() {
        assert!(parse("<A>").is_err());
    }

    #[test]
    fn it_cannot_parse_when_a_element_has_no_an_open_tag() {
        assert!(parse("</A>").is_err());
    }

    #[test]
    fn it_cannot_parse_when_a_element_has_no_its_name() {
        assert!(parse("<>").is_err());
    }

    #[test]
    fn it_cannot_parse_when_root_is_text() {
        assert!(parse("text").is_err());
    }

    #[test]
    fn it_cannot_parse_when_it_is_empty() {
        assert!(parse("").is_err());
    }
}
