use std::cmp::Ordering;

use rand::{Rng, thread_rng};

use crate::card::{Card, Suit, Rank};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BPFlush {
    pub card: Card,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BPStraightFlush {
    pub card: Card,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandValue {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(BPFlush),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(BPStraightFlush),
}

impl PartialOrd for BPFlush {
    fn partial_cmp(&self, other: &BPFlush) -> Option<Ordering> {
        Some(other.card.cmp(&self.card))
    }
}

impl Ord for BPFlush {
    fn cmp(&self, other: &BPFlush) -> Ordering {
        other.card.cmp(&self.card)
    }
}

impl PartialOrd for BPStraightFlush {
    fn partial_cmp(&self, other: &BPStraightFlush) -> Option<Ordering> {
        Some(other.card.cmp(&self.card))
    }
}

impl Ord for BPStraightFlush {
    fn cmp(&self, other: &BPStraightFlush) -> Ordering {
        other.card.cmp(&self.card)
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    // returns an empty hand
    pub fn empty_hand() -> Hand {
        Hand { cards: vec![] }
    }

    // returns a standard 52 card deck
    pub fn get_full_deck() -> Hand {
        let mut cards = vec![];
        for rank_u8 in 2..15 {
            let rank = Rank::from_u8(rank_u8).unwrap();
            let to_add = Card::get_all_with_rank(rank);
            for card in &to_add {
                cards.push(*card);
            }
        }
        thread_rng().shuffle(&mut cards);
        Hand { cards: cards }
    }

    // returns a random hand with length of the input
    pub fn random_hand(size: usize) -> Hand {
        let mut full_deck = Hand::get_full_deck();
        thread_rng().shuffle(&mut full_deck.cards);
        let mut hand = Hand { cards: vec![] };
        for i in 0..size {
            hand.cards.push(full_deck.cards[i]);
        }
        hand
    }

    // returns a hand from input deck with length of the input
    pub fn hand_from(deck: &mut Hand, size: usize) -> Hand {
        let mut hand = Hand { cards: vec![] };
        for i in 0..size {
            hand.cards.push(deck.cards[i]);
            deck.cards.remove(i);
        }
        hand
    }

    // adds the input cards to the deck
    pub fn add_cards(&mut self, new_cards: Hand) {
        for card in new_cards.cards {
            self.cards.push(card)
        }
    }

    // print horizontal representation of cards to stdout
    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        for i in 0..5 {
            for card in &self.cards {
                output.push_str(&card.to_single_string(i));
                output.push_str(" ");
            }
            output.push_str("\n");
        }
        output
    }

    // returns a list of cards in the hand with the input rank
    fn get_cards_with_rank(&self, rank: Rank) -> Vec<&Card> {
        let mut output: Vec<&Card> = vec![];
        for card in &self.cards {
            if card.rank == rank {
                output.push(card)
            }
        }
        output
    }

    // returns a list of cards in the hand with the input suit
    fn get_cards_with_suit(&self, suit: Suit) -> Vec<&Card> {
        let mut output: Vec<&Card> = vec![];
        for card in &self.cards {
            if card.suit == suit {
                output.push(card)
            }
        }
        output
    }

    // returns whether or not the handvalue is in the hand
    pub fn contains_handvalue(&self, value: &HandValue) -> bool {
        match value {
            &HandValue::StraightFlush(bfsf) => {
                if !self.cards.contains(&bfsf.card) {
                    return false;
                }
                if bfsf.card.rank < Rank::Six {
                    return false;
                }
                let top = bfsf.card;
                let rank_u8 = top.rank.to_u8();
                let mut is_sf = true;
                for i in (rank_u8 - 4)..rank_u8 {
                    let rank = Rank::from_u8(i).unwrap();
                    let card = Card {
                        suit: top.suit,
                        rank: rank,
                    };
                    if !self.cards.contains(&card) {
                        is_sf = false
                    }
                }
                if is_sf {
                    return true;
                }
                return false;
            }
            &HandValue::FourOfAKind(rank) => {
                let cards = Card::get_all_with_rank(rank);
                for card in &cards {
                    if !self.cards.contains(&card) {
                        return false;
                    }
                }
                return true;
            }
            &HandValue::FullHouse(top_r, bot_r) => {
                let mut total_three = 0;
                let top_cards = Card::get_all_with_rank(top_r);
                for card in &top_cards {
                    if self.cards.contains(&card) {
                        total_three += 1;
                    }
                }
                if total_three < 3 {
                    return false;
                }
                let mut total_two = 0;
                let bot_cards = Card::get_all_with_rank(bot_r);
                for card in &bot_cards {
                    if self.cards.contains(&card) {
                        total_two += 1;
                    }
                }
                return total_two >= 2;
            }
            &HandValue::Flush(bff) => {
                if !self.cards.contains(&bff.card) {
                    return false;
                }
                let top = bff.card;
                let mut total = 1;
                let rank_u8 = top.rank.to_u8();
                for rank in 2..rank_u8 {
                    let rank_card = Rank::from_u8(rank).unwrap();
                    let card = Card {
                        suit: top.suit,
                        rank: rank_card,
                    };
                    if self.cards.contains(&card) {
                        total += 1
                    }
                }
                if total >= 5 {
                    return true;
                }
            return false;
            }
            &HandValue::Straight(rank) => {
                if rank < Rank::Six {
                    return false;
                }
                let rank_u8 = rank.to_u8();
                for potential_rank in (rank_u8 - 4)..(rank_u8 + 1) {
                    let potential_cards =
                        Card::get_all_with_rank(Rank::from_u8(potential_rank).unwrap());
                    let mut contains_one = false;
                    for card in &potential_cards {
                        if self.cards.contains(&card) {
                            contains_one = true;
                        }
                    }
                    if !contains_one {
                        return false;
                    }
                }
                return true;
            }
            &HandValue::ThreeOfAKind(rank) => {
                let potential_cards = Card::get_all_with_rank(rank);
                let mut total = 0;
                for card in &potential_cards {
                    if self.cards.contains(&card) {
                        total += 1;
                    }
                }
                return total >= 3;
            }
            &HandValue::TwoPair(top_r, bot_r) => {
                let mut top_two = 0;
                let top_cards = Card::get_all_with_rank(top_r);
                for card in &top_cards {
                    if self.cards.contains(&card) {
                        top_two += 1;
                    }
                }
                if top_two < 2 {
                    return false;
                }
                let mut bot_two = 0;
                let bot_cards = Card::get_all_with_rank(bot_r);
                for card in &bot_cards {
                    if self.cards.contains(&card) {
                        bot_two += 1;
                    }
                }
                return bot_two >= 2;
            }
            &HandValue::OnePair(rank) => {
                let mut total = 0;
                let potential_cards = Card::get_all_with_rank(rank);
                for card in &potential_cards {
                    if self.cards.contains(&card) {
                        total += 1;
                    }
                }
                return total >= 2;
            }
            &HandValue::HighCard(rank) => {
                let mut total = 0;
                let potential_cards = Card::get_all_with_rank(rank);
                for card in &potential_cards {
                    if self.cards.contains(&card) {
                        total += 1;
                    }
                }
                return total >= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::poker::{Hand, HandValue, BPFlush, BPStraightFlush};
    use crate::card::{Card, Suit, Rank};

    #[test]
    fn test_contains_high_card() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Seven,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(&HandValue::HighCard(Rank::Three)));
        assert!(!hand.contains_handvalue(&HandValue::HighCard(Rank::Nine)));
    }
    #[test]
    fn test_contains_one_pair() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Seven,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(&HandValue::OnePair(Rank::Ten)));
        assert!(!hand.contains_handvalue(&HandValue::OnePair(Rank::Three)));
    }
    #[test]
    fn test_contains_two_pair() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(
            &HandValue::TwoPair(Rank::Ten, Rank::Five),
        ));
        assert!(!hand.contains_handvalue(
            &HandValue::TwoPair(Rank::Five, Rank::Ace),
        ));
    }
    #[test]
    fn test_contains_three_of_a_kind() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Spades,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(
            &HandValue::ThreeOfAKind(Rank::Five),
        ));
        assert!(!hand.contains_handvalue(
            &HandValue::ThreeOfAKind(Rank::Ten),
        ));
    }
    #[test]
    fn test_contains_straight() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Four,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Six,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Seven,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Eight,
            suit: Suit::Clubs,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(&HandValue::Straight(Rank::Seven)));
        assert!(hand.contains_handvalue(&HandValue::Straight(Rank::Eight)));
        assert!(!hand.contains_handvalue(&HandValue::Straight(Rank::Nine)));
    }
    #[test]
    fn test_contains_flush() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Six,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Jack,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::King,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(
            &HandValue::Flush(BPFlush { rank: Rank::Ace }),
        ));
        assert!(!hand.contains_handvalue(
            &HandValue::Flush(BPFlush { rank: Rank::Jack }),
        ));
    }
    #[test]
    fn test_contains_full_house() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::King,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(
            &HandValue::FullHouse(Rank::Five, Rank::Ace),
        ));
        assert!(!hand.contains_handvalue(
            &HandValue::FullHouse(Rank::Ace, Rank::Five),
        ));
    }
    #[test]
    fn test_contains_four_of_a_kind() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Clubs,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Hearts,
        });
        cards.push(Card {
            rank: Rank::Five,
            suit: Suit::Spades,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        });
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(&HandValue::FourOfAKind(Rank::Five)));
        assert!(!hand.contains_handvalue(&HandValue::FourOfAKind(Rank::Ace)));
    }
    #[test]
    fn test_contains_straight_flush() {
        let mut cards = vec![];
        cards.push(Card {
            rank: Rank::Ace,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::King,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Queen,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Jack,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Ten,
            suit: Suit::Diamonds,
        });
        cards.push(Card {
            rank: Rank::Nine,
            suit: Suit::Spades,
        });
        cards.push(Card {
            rank: Rank::Queen,
            suit: Suit::Clubs,
        });
        let hand = Hand { cards: cards };
        assert!(hand.contains_handvalue(&HandValue::StraightFlush(
            BPStraightFlush { rank: Rank::Ace },
        )));
        assert!(!hand.contains_handvalue(&HandValue::StraightFlush(
            BPStraightFlush { rank: Rank::King },
        )));
    }
    #[test]
    fn test_get_full_deck() {
        let hand = Hand::get_full_deck();
        assert_eq!(hand.cards.len(), 52)
    }
}
