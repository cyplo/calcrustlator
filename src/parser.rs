use Expression;

pub trait Parser<'a> {
    fn parse(&self, expression: &'a str) -> Expression;
}

#[derive(Default)]
pub struct SimpleParser;
impl<'a> Parser<'a> for SimpleParser {
    fn parse(&self, expression: &'a str) -> Expression {
        Expression::default()
    }
}
