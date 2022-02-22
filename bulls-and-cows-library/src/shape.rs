use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Circle,
    Club,
    Diamond,
    Heart,
    Star,
    Square,
    Spade,
    Triangle,
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=7) {
            0 => Shape::Circle,
            1 => Shape::Club,
            2 => Shape::Diamond,
            3 => Shape::Heart,
            4 => Shape::Star,
            5 => Shape::Square,
            6 => Shape::Spade,
            _ => Shape::Triangle,
        }
    }
}
