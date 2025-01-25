use from_pest::FromPest;
use serde::Serialize;

use crate::Rule;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Prefix {
    Not,
    BitNot,
    Neg,
    Yield,
}

impl FromPest<'_> for Prefix {
    type Rule = Rule;

    type FatalError = from_pest::Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
    ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
        let next = pest.next().unwrap();
        match next.as_rule() {
            Rule::prefix_not => Ok(Prefix::Not),
            Rule::prefix_bit_not => Ok(Prefix::BitNot),
            Rule::prefix_neg => Ok(Prefix::Neg),
            Rule::prefix_yield => Ok(Prefix::Yield),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{Rule, expr::prefix::Prefix};

    #[test]
    fn test_prefix_not() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "!")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::Not);
    }

    #[test]
    fn test_prefix_bit_not() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "~")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::BitNot);
    }

    #[test]
    fn test_prefix_neg() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "-")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::Neg);
    }

    #[test]
    fn test_prefix_yield() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "<-")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::Yield);
    }
}
