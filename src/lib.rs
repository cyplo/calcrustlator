#[cfg(test)]
#[macro_use]
extern crate double;

pub struct Calcrustlator {
    result: i32
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Expression;

pub trait Parser<'a> {
    fn parse(&self, expression: &'a str) -> Expression;
}

#[derive(Default)]
pub struct SimpleParser;
impl<'a> Parser<'a> for SimpleParser {
    fn parse(&self, expression: &'a str) -> Expression {
        unimplemented!()
    }
}

impl Calcrustlator {
    pub fn with_expression_and_parser<'a>(expression: &str, parser: &Parser) -> Self {
        Calcrustlator{
            result: 0
        }
    }

    pub fn result(&self) -> i32 {
        self.result
    }
}

#[cfg(test)]
mod must {

    use super::*;

    #[test]
    fn pass_expression_to_parser() {
        let input = "2 + 2";
        let mock_parser = MockParser::default();
        let calcrustlator = Calcrustlator::with_expression_and_parser(input, &mock_parser);
        assert_eq!(mock_parser.parse.called_with(input), true);
    }

    mock_trait!(MockParser, parse(String) -> Expression);

    impl<'a> Parser<'a> for MockParser {
        mock_method!(parse(&self, expression: &str) -> Expression, self, {
            self.parse.call(expression.to_owned())
        });
    }
}
