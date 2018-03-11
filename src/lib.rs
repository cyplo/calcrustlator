pub struct Calcrustlator {
    pub result: i32
}

impl Calcrustlator {
    pub fn with_expression(expression: &str) -> Self {
        Calcrustlator{
            result: 0
        }
    }
}
