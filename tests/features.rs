extern crate calcrustlator;

use calcrustlator::*;

#[test]
fn addition() {
    let input = "2 + 2";
    let calcrustlator = Calcrustlator::with_expression(input);
    assert_eq!(calcrustlator.result(), 4);
}
