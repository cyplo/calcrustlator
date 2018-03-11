extern crate calcrustlator;

#[cfg(test)]
mod calcrustlator_must {

    use calcrustlator::*;

    #[test]
    fn add() {
        let input = "2 + 2";
        let calcrustlator = Calcrustlator::with_expression(input);
        assert_eq!(calcrustlator.result, 4);
    }
}