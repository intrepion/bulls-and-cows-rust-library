#[cfg(test)]
mod game_should {
    use super::Game;

    #[test]
    fn have_initial_state_when_creating_new() {
        let game = Game::new();

        let _actual = game.get_move_history();
    }
}

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn get_move_history(&self) {}
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
