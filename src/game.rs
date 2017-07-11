use poker::{Hand, HandValue};

pub struct Player {
    pub hand: Hand
}

pub struct GameState {
    pub players: Vec<Player>,
    pub current_turn: u8,
    pub current_bet: Option<HandValue>
}

pub fn init_game(num_players: u8, init_handsize: u8) -> Option<GameState> {
    if num_players * init_handsize > 52 {
        return None
    }
    let mut players = vec!();
    let mut deck = Hand::get_full_deck();
    
    for i in 0..num_players {
        let new_player = Player { hand: Hand::hand_from(&mut deck, init_handsize as usize) };
        players.push(new_player);
    }
    Some(GameState {
        players: players,
        current_turn: 0,
        current_bet: None
    })
}
