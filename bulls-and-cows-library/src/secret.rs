#[cfg(test)]
mod secret_test {
    mod get_code_should {
        use super::super::Secret;
        use crate::shape::Shape;

        #[test]
        fn be_empty_when_creating_new() {
            let secret = Secret::new(vec![]);

            let actual = secret.get_code();
            let expected = vec![];

            assert_eq!(actual, expected);
        }

        #[test]
        fn be_independent_to_any_changes_in_secret() {
            let secret = Secret::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);

            let actual = secret.get_code();
            let expected = vec![Shape::Circle, Shape::Triangle, Shape::Square, Shape::Star];

            assert_eq!(actual, expected);
        }
    }
}

use crate::shape::Shape;

pub struct Secret {
    code: Vec<Shape>,
}

impl Secret {
    pub fn new(code: Vec<Shape>) -> Secret {
        Secret { code }
    }

    pub fn get_code(&self) -> Vec<Shape> {
        self.code.clone()
    }
}
