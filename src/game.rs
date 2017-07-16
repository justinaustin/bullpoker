use std::collections::HashMap;

use poker::{Hand, HandValue};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: u8,
    pub hand: Hand,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub current_bet: Option<HandValue>,
}

pub enum GameMove {
    NewGame(u8, u8),
    Bet(HandValue),
    Call(),
}

impl GameState {
    pub fn handle_gamemove(mut self, gm: GameMove) -> GameState {
        match gm {
            GameMove::NewGame(num_players, init_handsize) => {
                if let Some(gamestate) = GameState::init_game(num_players, init_handsize) {
                    gamestate
                } else {
                    panic!("TODO")
                }
            }
            GameMove::Bet(hv) => {
                self.turn_after(hv);
                return self.clone();
            }
            GameMove::Call() => {
                let _ = self.handle_call();
                return self.clone();
            }
        }
    }

    pub fn init_game(num_players: u8, init_handsize: u8) -> Option<GameState> {
        if num_players * init_handsize > 52 {
            return None;
        }
        let mut players = vec![];
        let mut deck = Hand::get_full_deck();
        let mut turn_order = vec![];

        for i in 0..num_players {
            let new_player = Player {
                name: i,
                hand: Hand::hand_from(&mut deck, init_handsize as usize),
            };
            players.push(new_player);
            turn_order.push(i);
        }
        Some(GameState {
            players: players,
            current_turn: 0,
            current_bet: None,
        })
    }

    fn turn_after(&mut self, new_bet: HandValue) {
        self.current_bet = Some(new_bet);
        self.current_turn += 1;
        if self.current_turn >= self.players.len() as usize {
            self.current_turn = 0;
        }
    }

    // called when the player 'calls' the previous player
    // returns true iff the player successfully called the bluff
    fn handle_call(&mut self) -> bool {
        if self.current_bet == None {
            println!("No current bet!");
        }
        // construct deck with all players hand
        let mut all_cards = Hand::empty_hand();
        for player in &self.players {
            all_cards.add_cards(player.hand.clone());
        }
        if all_cards.contains_handvalue(&self.current_bet.unwrap()) {
            // Calling bluff failed. Calling player loses a card
            let hand_size = self.players
                .get(self.current_turn)
                .unwrap()
                .hand
                .cards
                .len();
            if hand_size == 1 {
                // player out of game and person who knocked them out goes next
                self.players.remove(self.current_turn);
                self.current_turn -= 1;
                if self.current_turn < 0 {
                    self.current_turn = self.players.len() as usize - 1
                }
            } else {
                // player loses a card
                let index = self.current_turn;
                self.remove_card_from(index);
            }
            self.current_bet = None;
            false
        } else {
            // Previous player loses a card
            let previous_player = if self.current_turn == 0 {
                self.players.len() - 1
            } else {
                self.current_turn - 1
            };
            let hand_size = self.players.get(previous_player).unwrap().hand.cards.len();
            if hand_size == 1 {
                // player out of game and person who knocked them out goes next
                self.players.remove(previous_player);
                if self.current_turn != 0 {
                    self.current_turn -= 1;
                }
            } else {
                // player loses a card
                self.remove_card_from(previous_player);
                if self.current_turn == 0 {
                    self.current_turn = self.players.len() - 1;
                } else {
                    self.current_turn -= 1;
                }
            }
            self.current_bet = None;
            true
        }
    }

    // Removes a card from the input players hand
    fn remove_card_from(&mut self, index: usize) {
        let mut new_players = vec![];
        let mut i = 0;
        for player in &self.players {
            let hand_size = if i == index {
                player.hand.cards.len() - 1
            } else {
                player.hand.cards.len()
            };
            let new_player = Player {
                name: player.name.clone(),
                hand: Hand::random_hand(hand_size),
            };
            new_players.push(new_player);
            i += 1;
        }
        self.players = new_players;
    }

    // Redeals cards
    fn redeal(&mut self) {
        let mut new_players = vec![];
        let mut deck = Hand::get_full_deck();
        for player in &self.players {
            let hand_size = player.hand.cards.len();
            new_players.push(Player {
                name: player.name.clone(),
                hand: Hand::hand_from(&mut deck, hand_size),
            });
        }
        self.players = new_players;
    }

    pub fn display(&self) {
        for player in &self.players {
            println!("Player {}: ", player.name);
            println!("{}", player.hand.to_string());
        }
    }
}
