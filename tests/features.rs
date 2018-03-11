extern crate calcrustlator;

use calcrustlator::*;

#[test]
fn addition() {
    let input = "2 + 2";
    let calcrustlator = Calcrustlator::with_expression_and_parser(input, &SimpleParser::default());
    assert_eq!(calcrustlator.result(), 4);
}
