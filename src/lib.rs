pub struct Calcrustlator {
    result: i32
}

impl Calcrustlator {
    pub fn with_expression(expression: &str) -> Self {
        Calcrustlator{
            result: 0
        }
    }

    pub fn result(&self) -> i32 {
        self.result
    }
}
