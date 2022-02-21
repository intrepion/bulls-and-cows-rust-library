use crate::logic::shape::Shape;

pub struct Secret {
    code: Vec<Shape>,
}

impl Secret {
    pub fn new(code: Vec<Shape>) -> Secret {
        Secret { code }
    }

    pub fn get_code(&self) -> &Vec<Shape> {
        &self.code
    }
}
