extern crate calcrustlator;

use calcrustlator::*;

#[test]
fn addition() {
    let input = "2 + 2";
    let parser = StringExpressionParser::new();
    let calcrustlator = Calcrustlator::with_expression(input, &parser);
    assert_eq!(calcrustlator.result(), 4);
}
