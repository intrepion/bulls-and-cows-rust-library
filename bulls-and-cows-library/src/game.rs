#[cfg(test)]
mod new_game_should {
    use super::Game;

    #[test]
    fn have_initial_state_of_the_game() {
        let game = Game::new();
    }
}

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
