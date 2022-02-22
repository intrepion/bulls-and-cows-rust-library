#[cfg(test)]
mod guess_test {
    mod get_code_should {
        use super::super::Guess;
        use crate::shape::Shape;

        #[test]
        fn be_empty_when_creating_new() {
            let guess = Guess::new(vec![]);

            let actual = guess.get_code();
            let expected = vec![];

            assert_eq!(actual, expected);
        }

        #[test]
        fn be_independent_to_any_changes() {
            let guess = Guess::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);

            let actual = guess.get_code();
            let mut _changed_guess = guess;
            _changed_guess = Guess::new(vec![
                Shape::Club,
                Shape::Diamond,
                Shape::Heart,
                Shape::Spade,
            ]);
            let expected = vec![Shape::Circle, Shape::Triangle, Shape::Square, Shape::Star];

            assert_eq!(actual, expected);
        }
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
