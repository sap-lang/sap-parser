// use crate::Rule;
// use pest::iterators::{Pair, Pairs};
use regex::{Captures, Regex};
use serde::Serialize;

fn handle_special_escape(str: String) -> String {
    // hex 8 digit
    let regex_pattern4 = Regex::new(r"\\U([0-9a-fA-F]{8})").unwrap();
    if regex_pattern4.is_match(&str) {
        return regex_pattern4
            .replace_all(&str, |cap: &Captures| {
                let hex = &cap[1];
                let hex_int = u32::from_str_radix(hex, 16).unwrap();
                let char = char::from_u32(hex_int).unwrap();
                format!("{}", char)
            })
            .to_string();
    }

    // hex 4 digit
    let regex_pattern3 = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();
    if regex_pattern3.is_match(&str) {
        return regex_pattern3
            .replace_all(&str, |cap: &Captures| {
                let hex = &cap[1];
                let hex_int = u32::from_str_radix(hex, 16).unwrap();
                let char = char::from_u32(hex_int).unwrap();
                format!("{}", char)
            })
            .to_string();
    }

    // hex 2 digit
    let regex_pattern2 = Regex::new(r"\\x([0-9a-fA-F]{2})").unwrap();
    if regex_pattern2.is_match(&str) {
        return regex_pattern2
            .replace_all(&str, |cap: &Captures| {
                let hex = &cap[1];
                let hex_int = u32::from_str_radix(hex, 16).unwrap();
                let char = char::from_u32(hex_int).unwrap();
                format!("{}", char)
            })
            .to_string();
    }

    // ascii_oct_digit
    let regex_pattern1 = Regex::new(r"\\([0-7]{1,3})").unwrap();
    if regex_pattern1.is_match(&str) {
        regex_pattern1
            .replace_all(&str, |cap: &Captures| {
                let oct = &cap[1];
                let oct_int = u32::from_str_radix(oct, 8).unwrap();
                let char = char::from_u32(oct_int).unwrap();
                format!("{}", char)
            })
            .to_string()
    } else {
        str
    }
}

fn handle_c_escape(str: &str) -> String {
    str.replace(r#"\r"#, "\x0d")
        .replace(r"\\", "\\r")
        .replace(r#"\a"#, "\x07")
        .replace(r#"\b"#, "\x08")
        .replace(r#"\e"#, "\x1b")
        .replace(r#"\f"#, "\x0c")
        .replace(r#"\n"#, "\x0a")
        .replace(r#"\t"#, "\x09")
        .replace(r#"\v"#, "\x0b")
        .replace(r#"\?"#, "\x3f")
        .replace(r#"\""#, "\"")
}

fn handle_escape(str: String) -> String {
    let c_escaped = handle_c_escape(&str);
    let res = handle_special_escape(c_escaped);
    res.replace("\\r", r"\")
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::escaped_string_fragment))]
pub struct EscapedStringFragment {
    #[pest_ast(outer(with(span_to_string), with(handle_escape)))]
    pub value: String,
}

use pest_ast::FromPest;

use crate::Rule;
use crate::span_to_string;

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::normal_string_fragment))]
pub struct NormalStringFragment {
    #[pest_ast(outer(with(span_to_string)))]
    pub value: String,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::normal_string_inner))]
pub enum NormalStringInner {
    NormalStringFragment(NormalStringFragment),
    EscapedStringFragment(EscapedStringFragment),
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::normal_string))]
pub struct NormalString {
    pub inner: Vec<NormalStringInner>,
}

impl NormalString {
    pub fn value(&self) -> String {
        self.inner
            .iter()
            .map(|inner| match inner {
                NormalStringInner::NormalStringFragment(fragment) => fragment.value.clone(),
                NormalStringInner::EscapedStringFragment(fragment) => fragment.value.clone(),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Serialize for NormalString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let inner = self
            .inner
            .iter()
            .map(|inner| match inner {
                NormalStringInner::NormalStringFragment(fragment) => fragment.value.clone(),
                NormalStringInner::EscapedStringFragment(fragment) => fragment.value.clone(),
            })
            .collect::<Vec<String>>()
            .join("");
        serializer.serialize_str(&inner)
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::raw_string_inner))]
pub struct RawStringInner {
    #[pest_ast(outer(with(span_to_string)))]
    pub value: String,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::raw_string))]
pub struct RawString {
    pub inner: RawStringInner,
}

impl Serialize for RawString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner.value)
    }
}

#[derive(Debug, Clone, FromPest, Serialize)]
#[pest_ast(rule(Rule::string))]
pub enum SapString {
    NormalString(NormalString),
    RawString(RawString),
}

impl PartialEq for SapString {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SapString::NormalString(a), SapString::NormalString(b)) => a.value() == b.value(),
            (SapString::RawString(a), SapString::RawString(b)) => a.inner.value == b.inner.value,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::{Parser, iterators::Pairs};

    use crate::SapParser;
    use from_pest::FromPest;

    use super::*;

    #[test]
    fn test_handle_escape() {
        let escapes = [
            (r"\ue000", "\u{e000}"),
            (r"\a", "\x07"),
            (r"\b", "\x08"),
            (r"\e", "\x1b"),
            (r"\f", "\x0c"),
            (r#"\\"#, "\\"),
            (r#"\\x65"#, "\\x65"),
            (r"\n", "\x0a"),
            (r#"\""#, "\""),
            (r"\x65", "e"),
            (r"\U0010ffff", "\u{10ffff}"),
        ];
        for (e, r) in escapes {
            assert_eq!(handle_escape(e.to_string()), r);
        }
    }

    #[test]
    fn test_parse_normal_string() {
        let str = r#""hello\nworld!""#;
        let pair = SapParser::parse(Rule::normal_string, str)
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let normal_string = NormalString::from_pest(&mut pairs).unwrap();
        println!("{:?}", normal_string);
    }

    #[test]
    fn test_parse_raw_string() {
        let str = r###"r##"hello
        #"#
        \nworld!"##"###;
        let pair = SapParser::parse(Rule::raw_string, str)
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let raw_string = RawString::from_pest(&mut pairs).unwrap();
        println!("{:?}", raw_string);
    }
}
