extern crate rand;

mod card;
mod game;
mod logic;
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
        Ok(_) => {
            let vec: Vec<&str> = input.split(" ").collect();
            let mut rank_one = None;
            let mut rank_two = None;
            let mut card = None;
            let hand = vec[0].trim();
            if vec[1].trim().len() == 1 {
                rank_one = Rank::from_str(vec[1].trim());
            } else {
                let ch = vec[1].chars().nth(0).unwrap();
                let suit = match vec[1].chars().nth(1).unwrap() {
                    'C' => Suit::Clubs,
                    'D' => Suit::Diamonds,
                    'H' => Suit::Hearts,
                    'S' => Suit::Spades,
                    _ => panic!()
                };
                card = Some(Card {
                    rank: Rank::from_str(&ch.to_string()).unwrap(),
                    suit: suit
                });
            }
            if vec.len() == 3 {
                rank_two = Rank::from_str(vec[2].trim());
            }
            let handvalue = match hand {
                "high" => HandValue::HighCard(rank_one.unwrap()),
                "pair" => HandValue::OnePair(rank_one.unwrap()),
                "twopair" => HandValue::TwoPair(rank_one.unwrap(), rank_two.unwrap()),
                "triple" => HandValue::ThreeOfAKind(rank_one.unwrap()),
                "straight" => HandValue::Straight(rank_one.unwrap()),
                "flush" => HandValue::Flush(BPFlush { card: card.unwrap() }),
                "fullhouse" => HandValue::FullHouse(rank_one.unwrap(), rank_two.unwrap()),
                "quad" => HandValue::FourOfAKind(rank_one.unwrap()),
                "straightflush" => HandValue::StraightFlush(BPStraightFlush { card: card.unwrap() }),
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
