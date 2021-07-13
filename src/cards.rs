#![allow(dead_code)]

pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}
impl Suit {
    ///Returns Suit::Club if index out of range
    pub fn item_from_index(index: usize) -> Suit {
        match index {
            0 => Suit::Club,
            1 => Suit::Diamond,
            2 => Suit::Heart,
            3 => Suit::Spade,
            _ => Suit::Club
        }
    }

    pub fn string(suit: Suit) -> String {
        match suit {
            Suit::Club => String::from("♣"),
            Suit::Diamond => String::from("♦"),
            Suit::Heart => String::from("♥"),
            Suit::Spade => String::from("♠")
        }
    }
}


#[derive(PartialEq)]
pub enum Value {
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
    King,
    Queen,
    Ace,
}
impl Value {
    //TODO: give this function a helpful name
    ///returns Value::Ace if index out of range
    pub fn item_from_index(index: usize) -> Value {
        match index {
            0 => Value::Two,
            1 => Value::Three,
            2 => Value::Four,
            3 => Value::Five,
            4 => Value::Six,
            5 => Value::Seven,
            6 => Value::Eight,
            7 => Value::Nine,
            8 => Value::Ten,
            9 => Value::Jack,
            10 => Value::King,
            11 => Value::Queen,
            12 => Value::Ace,
            _ => Value::Ace
        }
    }

    pub fn string(value: Value) -> String {
        match value {
            Value::Two => String::from("2"),
            Value::Three => String::from("3"),
            Value::Four => String::from("4"),
            Value::Five => String::from("5"),
            Value::Six => String::from("6"),
            Value::Seven => String::from("7"),
            Value::Eight => String::from("8"),
            Value::Nine => String::from("9"),
            Value::Ten => String::from("10"),
            Value::Jack => String::from("J"),
            Value::King => String::from("K"),
            Value::Queen => String::from("Q"),
            Value::Ace => String::from("A"),
        }
    }
}


pub struct Card {
    pub suit: Suit,
    pub value: Value
}
impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Card {
            suit: suit,
            value: value
        }
    }
    
}


pub struct Hand {
    pub vec: Vec<Card>
}
impl Hand {
    pub fn from(vec: Vec<Card>) -> Hand {
        Hand {
            vec: vec
        }
    }

    pub fn full_deck() -> Hand {
        let mut h = vec![];

        for suit_index in 0..4 {
            for value_index in 0..13 {
                h.push(Card::new(
                    Value::item_from_index(value_index), 
                    Suit::item_from_index(suit_index)
                ))
            }
        }

        Hand::from(h)
    }

}


pub mod poker {

    pub enum PokerHand {
        HighCard,
        Pair,
        TwoPair,
        Triple,
        Straight,
        Flush,
        FullHouse,
        FourOfAKind,
        StraightFlush,
        RoyalFlush
    }

    //pub fn score_hand(hand: &Hand) -> PokerHand {
    //    
    //}
}


pub mod blackjack {
    use crate::cards;
    use crate::cards::Value;

    pub fn score_hand(hand: &cards::Hand) -> i32 {
        let mut score = 0;

        for card in &hand.vec {
            score += score_card(&card.value);
            if score > 21 && &card.value == &Value::Ace {
                score += -10
            }
        }

        score
    }

     ///scores ace as 11
    fn score_card(card: &Value) -> i32 {
        match card {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::King => 10,
            Value::Queen => 10,
            Value::Ace => 11,
        }
    }
}