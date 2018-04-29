#[cfg(test)]
#[macro_use]
extern crate double;

mod parser;
use parser::Parser;
use parser::SimpleParser;

pub struct Calcrustlator {
    result: i32
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Expression;

impl Calcrustlator {

    // hard to test with unit test, tested using feature test
    pub fn with_expression<'a>(expression: &'a str) -> Self {
        let parser = SimpleParser::default();
        Calcrustlator::with_expression_and_parser(expression, &parser)
    }

    fn with_expression_and_parser<'a>(expression: &'a str, parser: &Parser<'a>) -> Self {
        parser.parse(expression);
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

    // design follows through here
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
