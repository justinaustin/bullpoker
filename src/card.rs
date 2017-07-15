use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Suit {
    pub fn to_string(&self) -> &str {
        let output = match self {
            &Suit::Clubs => "♣",
            &Suit::Diamonds => "♦",
            &Suit::Hearts => "♥",
            &Suit::Spades => "♠",
        };
        output
    }
}

impl Rank {
    pub fn to_string(&self) -> &str {
        let output = match self {
            &Rank::Two => "2",
            &Rank::Three => "3",
            &Rank::Four => "4",
            &Rank::Five => "5",
            &Rank::Six => "6",
            &Rank::Seven => "7",
            &Rank::Eight => "8",
            &Rank::Nine => "9",
            &Rank::Ten => "10",
            &Rank::Jack => "J",
            &Rank::Queen => "Q",
            &Rank::King => "K",
            &Rank::Ace => "A",
        };
        output
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            &Rank::Two => 2,
            &Rank::Three => 3,
            &Rank::Four => 4,
            &Rank::Five => 5,
            &Rank::Six => 6,
            &Rank::Seven => 7,
            &Rank::Eight => 8,
            &Rank::Nine => 9,
            &Rank::Ten => 10,
            &Rank::Jack => 11,
            &Rank::Queen => 12,
            &Rank::King => 13,
            &Rank::Ace => 14, 
        }
    }

    pub fn from_u8(num: u8) -> Option<Rank> {
        match num {
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            9 => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            14 => Some(Rank::Ace),
            _ => None,
        }
    }
    pub fn from_str(s: &str) -> Option<Rank> {
        match s {
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            "9" => Some(Rank::Nine),
            "10" => Some(Rank::Ten),
            "J" => Some(Rank::Jack),
            "Q" => Some(Rank::Queen),
            "K" => Some(Rank::King),
            "A" => Some(Rank::Ace),
            _ => None,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Card {
    pub fn to_single_string(&self, line: u8) -> String {
        match line {
            0 => "+-----+".to_owned(),
            1 => {
                let mut output = "|".to_owned();
                output.push_str(self.rank.to_string());
                if self.rank != Rank::Ten {
                    output.push(' ');
                }
                output.push_str(self.suit.to_string());
                output.push_str("  |");
                output
            }
            2 => {
                let mut output = "|  ".to_owned();
                output.push_str(self.suit.to_string());
                output.push_str("  |");
                output
            }
            3 => {
                let mut output = "|  ".to_owned();
                output.push_str(self.suit.to_string());
                if self.rank != Rank::Ten {
                    output.push(' ');
                }
                output.push_str(self.rank.to_string());
                output.push_str("|");
                output
            }
            _ => "+-----+".to_owned(),
        }
    }
    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        for i in 0..4 {
            output.push_str(&self.to_single_string(i));
            output.push_str("\n")
        }
        output.push_str(&self.to_single_string(4));
        output
    }
    pub fn get_all_with_rank(rank: Rank) -> [Card; 4] {
        let mut output = [Card {
            suit: Suit::Clubs,
            rank: rank,
        }; 4];
        output[1] = Card {
            suit: Suit::Diamonds,
            rank: rank,
        };
        output[2] = Card {
            suit: Suit::Hearts,
            rank: rank,
        };
        output[3] = Card {
            suit: Suit::Spades,
            rank: rank,
        };
        output
    }
}
