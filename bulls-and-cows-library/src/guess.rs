#[cfg(test)]
mod guess_test {
    mod get_code_should {
    }
}

use crate::shape::Shape;

#[derive(Clone, Debug, PartialEq)]
pub struct Guess {
    code: Vec<Shape>,
}

impl Guess {
    pub fn new(code: Vec<Shape>) -> Guess {
        Guess { code }
    }

    pub fn get_code(&self) -> Vec<Shape> {
        self.code.clone()
    }
}
