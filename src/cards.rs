#![allow(dead_code)]

use rand::thread_rng;
use rand::Rng;

#[derive(Copy, Clone)]
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

    pub fn string(&self) -> String {
        match self {
            Suit::Club =>    String::from("♣"),
            Suit::Diamond => String::from("♦"),
            Suit::Heart =>   String::from("♥"),
            Suit::Spade =>   String::from("♠")
        }
    }

    pub fn random() -> Suit {
        let mut rng = thread_rng();

        Suit::item_from_index(rng.gen_range(0..3))
    }
}


#[derive(PartialEq)]
#[derive(Copy, Clone)]
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
    Queen,
    King,
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
            10 => Value::Queen,
            11 => Value::King,
            12 => Value::Ace,
            _ => Value::Ace
        }
    }

    pub fn string(&self) -> String {
        match self {
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

    pub fn random() -> Value {
        let mut rng = thread_rng();

        Value::item_from_index(rng.gen_range(0..12))
    }
}

#[derive(Copy, Clone)]
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
    
    pub fn random() -> Card {
        Card::new(
            Value::random(),
            Suit::random()
        )
    }

    pub fn display(&self) -> String {
        self.value.string() + &self.suit.string()
    }
}

