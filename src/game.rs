use poker::Hand;

pub struct Player {
    pub hand: Hand
}

pub struct GameState {
    pub players: Vec<Player>,
    pub current_turn: u8
}

// Make GameError for result and change earlier Option to result
// pub fn init_game(players: u8, init_handsize: u8) -> Result<GameState, &'static' str> {

// }
