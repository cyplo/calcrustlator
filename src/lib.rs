#[cfg(test)]
#[macro_use]
extern crate double;

pub trait ExpressionParser {
    fn with_expression(expression: &str);
}

pub struct StringExpressionParser;

impl StringExpressionParser {
}

impl ExpressionParser for StringExpressionParser {
    fn with_expression(expression: &str) {
        unimplemented!()
    }
}

pub struct Calcrustlator {
    result: i32
}

impl Calcrustlator {
    pub fn with_expression(expression: &str, parser: &ExpressionParser) -> Self {
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
        let parser = MockExpressionParser::default();
        let calcrustlator = Calcrustlator::with_expression(input, &parser);
    }

    mock_trait!(MockExpressionParser, with_expression(&str) -> &ExpressionParser);

    impl ExpressionParser for MockExpressionParser {
        mock_method!(with_expression(expression: &str) -> &ExpressionParser);
    }

}
