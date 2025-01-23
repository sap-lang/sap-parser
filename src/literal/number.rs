use from_pest::FromPest;
use pest_ast::FromPest;
use serde::Serialize;

use crate::Rule;

fn helper_parse_int(str: &str) -> i64 {
    let str = str.replace("_", "");
    if str.len() > 2 {
        let prefix = &str[0..2];
        match prefix {
            "0x" | "0X" => i64::from_str_radix(&str[2..], 16).unwrap(),
            "0b" | "0B" => i64::from_str_radix(&str[2..], 2).unwrap(),
            "0o" | "0O" => i64::from_str_radix(&str[2..], 8).unwrap(),
            _ => str.parse::<i64>().unwrap(),
        }
    } else {
        str.parse::<i64>().unwrap()
    }
}

fn parse_int(span: pest::Span) -> i64 {
    let str = span.as_str();
    helper_parse_int(str)
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::bin_int))]
pub struct BinInt {
    #[pest_ast(outer(with(parse_int)))]
    pub value: i64,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::oct_int))]
pub struct OctInt {
    #[pest_ast(outer(with(parse_int)))]
    pub value: i64,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::hex_int))]
pub struct HexInt {
    #[pest_ast(outer(with(parse_int)))]
    pub value: i64,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::dec_int))]
pub struct DecInt {
    #[pest_ast(outer(with(parse_int)))]
    pub value: i64,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::int))]
pub enum Int {
    BinInt(BinInt),
    OctInt(OctInt),
    HexInt(HexInt),
    DecInt(DecInt),
}

impl Int {
    pub fn value(&self) -> i64 {
        match self {
            Int::BinInt(bin_int) => bin_int.value,
            Int::OctInt(oct_int) => oct_int.value,
            Int::HexInt(hex_int) => hex_int.value,
            Int::DecInt(dec_int) => dec_int.value,
        }
    }
}

impl Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.value())
    }
}

fn parse_exponent_part(span: pest::Span) -> i64 {
    let str = span.as_str();
    if str.find('-').is_some() {
        -helper_parse_int(&str[2..])
    } else if str.find('+').is_some() {
        helper_parse_int(&str[2..])
    } else {
        helper_parse_int(&str[1..])
    }
}

#[derive(Debug, Clone)]
pub struct ExponentPart {
    pub value: i64,
}

impl FromPest<'_> for ExponentPart {
    type Rule = Rule;

    type FatalError = from_pest::Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
    ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
        let pair = pest.next().unwrap();
        let value = parse_exponent_part(pair.as_span());
        Ok(ExponentPart { value })
    }
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::float1))]
pub struct Float1 {
    pub value: Int,
    pub exponent_part: ExponentPart,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::float2))]
pub struct Float2 {
    pub value: Int,
    pub sub: Int,
    pub exponent_part: Option<ExponentPart>,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::float3))]
pub struct Float3 {
    pub sub: Int,
    pub exponent_part: Option<ExponentPart>,
}

#[derive(Debug, Clone, FromPest)]
#[pest_ast(rule(Rule::float))]
pub enum Float {
    Float1(Float1),
    Float2(Float2),
    Float3(Float3),
}

pub fn digits(n: f64) -> f64 {
    n.log10().floor() + 1.0
}

impl Float {
    pub fn value(&self) -> f64 {
        match self {
            Float::Float1(float1) => {
                let value = float1.value.value() as f64;
                let exponent = float1.exponent_part.value as f64;
                value * 10_f64.powf(exponent)
            }
            Float::Float2(float2) => {
                let value = float2.value.value() as f64;
                let sub = float2.sub.value() as f64;
                let sub = sub / 10_f64.powf(digits(sub));
                let exponent = float2.exponent_part.clone().unwrap().value as f64;
                value + sub * 10_f64.powf(exponent)
            }
            Float::Float3(float3) => {
                // 0.sub * 10^exponent
                let sub = float3.sub.value() as f64;
                let sub = sub / 10_f64.powf(digits(sub));
                let exponent = float3.exponent_part.clone().unwrap().value as f64;
                sub * 10_f64.powf(exponent)
            }
        }
    }
}

impl Serialize for Float {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.value())
    }
}

#[derive(Debug, Clone, FromPest, Serialize)]
#[pest_ast(rule(Rule::number))]
pub enum SapNumber {
    Float(Float),
    Int(Int),
}

impl PartialEq for SapNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SapNumber::Float(float1), SapNumber::Float(float2)) => {
                float1.value() == float2.value()
            }
            (SapNumber::Int(int1), SapNumber::Int(int2)) => int1.value() == int2.value(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::{Parser, iterators::Pairs};

    use crate::{Rule, SapParser};

    #[test]
    fn test_bin_int() {
        let pair = SapParser::parse(Rule::bin_int, "0b10_10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let bin_int = crate::literal::number::BinInt::from_pest(&mut pairs).unwrap();
        assert_eq!(bin_int.value, 10);
    }

    #[test]
    fn test_oct_int() {
        let pair = SapParser::parse(Rule::oct_int, "0o10_10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let oct_int = crate::literal::number::OctInt::from_pest(&mut pairs).unwrap();
        assert_eq!(oct_int.value, 520);
    }

    #[test]
    fn test_hex_int() {
        let pair = SapParser::parse(Rule::hex_int, "0x10_10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let hex_int = crate::literal::number::HexInt::from_pest(&mut pairs).unwrap();
        assert_eq!(hex_int.value, 4112);
    }

    #[test]
    fn test_dec_int() {
        let pair = SapParser::parse(Rule::dec_int, "10_10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let dec_int = crate::literal::number::DecInt::from_pest(&mut pairs).unwrap();
        assert_eq!(dec_int.value, 1010);
    }

    #[test]
    fn test_int() {
        let pair = SapParser::parse(Rule::int, "0b10_10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let int = crate::literal::number::Int::from_pest(&mut pairs).unwrap();
        match int {
            crate::literal::number::Int::BinInt(bin_int) => {
                assert_eq!(bin_int.value, 10);
            }
            _ => panic!("wrong int type"),
        }
    }

    #[test]
    fn test_exponent_part() {
        let pair = SapParser::parse(Rule::exponent_part, "e-10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let exponent_part = crate::literal::number::ExponentPart::from_pest(&mut pairs).unwrap();
        assert_eq!(exponent_part.value, -10);
    }

    #[test]
    fn test_float1() {
        let pair = SapParser::parse(Rule::float1, "0b10_10e-10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let float1 = crate::literal::number::Float1::from_pest(&mut pairs).unwrap();
        assert_eq!(float1.value.value(), 10);
        assert_eq!(float1.exponent_part.value, -10);
    }

    #[test]
    fn test_float2() {
        let pair = SapParser::parse(Rule::float2, "0b10_10.0b10e-0b10_10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let float2 = crate::literal::number::Float2::from_pest(&mut pairs).unwrap();
        assert_eq!(float2.value.value(), 10);
        assert_eq!(float2.sub.value(), 2);
        assert_eq!(float2.exponent_part.unwrap().value, -10);
    }

    #[test]
    fn test_float3() {
        let pair = SapParser::parse(Rule::float3, ".0b10e-10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let float3 = crate::literal::number::Float3::from_pest(&mut pairs).unwrap();
        assert_eq!(float3.sub.value(), 2);
        assert_eq!(float3.exponent_part.unwrap().value, -10);
    }

    #[test]
    fn test_float() {
        let pair = SapParser::parse(Rule::float, "0b10_10e-10")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = Pairs::single(pair.clone());
        let float = crate::literal::number::Float::from_pest(&mut pairs).unwrap();
        assert_eq!(float.value(), 0.000000001);
    }
}
