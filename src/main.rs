extern crate rand;

mod card;
mod game;
mod poker;

use card::{Card, Suit, Rank};
use poker::{Hand, HandValue, BPFlush, BPStraightFlush};

fn main() {
    let mut i = 1;
    let mut winning_hand = Hand::random_hand(1);
    loop {
        let mut hand = Hand::random_hand(5);
        if hand.contains_handvalue(&HandValue::StraightFlush(
            BPStraightFlush { rank: Rank::Ace },
        ))
        {
            winning_hand = hand;
            break;
        }
        i += 1;
    }
    winning_hand.cards.sort();
    println!(
        "Royal Flush took {} tries!\n{}",
        i,
        winning_hand.to_string()
    );
}
