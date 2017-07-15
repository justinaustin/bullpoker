extern crate rand;

mod card;
mod game;
mod poker;

use std::io;

use card::{Card, Suit, Rank};
use game::{Player, GameMove, GameState};
use poker::{Hand, HandValue, BPFlush, BPStraightFlush};

fn main() {
    println!("Welcome to BullPoker!");
    let mut state = GameState::init_game(1, 0).unwrap();
    loop {
        println!("Current Bet: {:?}", state.current_bet);
        println!(
            "Player {} - What is your next move? (new, bet, call)",
            state.players[state.current_turn].name
        );
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                match input.trim() {
                    "new" => {
                        let (players, cards) = parse_players_cards();
                        state = state.handle_gamemove(GameMove::NewGame(players, cards));
                    }
                    "bet" => {
                        let handvalue = parse_handvalue();
                        state = state.handle_gamemove(GameMove::Bet(handvalue));
                    }
                    "call" => {
                        state = state.handle_gamemove(GameMove::Call());
                    }
                    "quit" => break,
                    _ => println!("Invalid input!"),
                };
                state.display();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn parse_handvalue() -> HandValue {
    println!("Enter Your Bet (e.g. quad A):");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let vec: Vec<&str> = input.split(" ").collect();
            let hand = vec[0].trim();
            let rank_str = vec[1].trim();
            let rank_one = Rank::from_str(rank_str).unwrap();
            let rank_two = if vec.len() == 3 {
                let rank_one_str = vec[2].trim();
                Rank::from_str(rank_one_str).unwrap()
            } else {
                Rank::Two
            };
            let handvalue = match hand {
                "high" => HandValue::HighCard(rank_one),
                "pair" => HandValue::OnePair(rank_one),
                "twopair" => HandValue::TwoPair(rank_one, rank_two),
                "triple" => HandValue::ThreeOfAKind(rank_one),
                "straight" => HandValue::Straight(rank_one),
                "flush" => HandValue::Flush(BPFlush { rank: rank_one }),
                "fullhouse" => HandValue::FullHouse(rank_one, rank_two),
                "quad" => HandValue::FourOfAKind(rank_one),
                "straightflush" => HandValue::StraightFlush(BPStraightFlush { rank: rank_one }),
                _ => panic!(),
            };
            handvalue
        }
        Err(error) => {
            println!("error: {}", error);
            panic!()
        }
    }
}

fn parse_players_cards() -> (u8, u8) {
    println!("Enter # players & # cards per hand (e.g. 5 6): ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let vec: Vec<&str> = input.split(" ").collect();
            let players = vec[0].parse().unwrap();
            let cards = vec[1].trim().parse().unwrap();
            (players, cards)
        }
        Err(error) => {
            println!("error: {}", error);
            panic!()
        }
    }
}
