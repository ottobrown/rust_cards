#![allow(dead_code)]
use rand::thread_rng;
use rand::seq::SliceRandom;

//use crate::cards;
use crate::cards::Card;
use crate::cards::Suit;
use crate::cards::Value;

//#[derive(Copy, Clone)]
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

    pub fn shuffle(&mut self) {
        //TODO: make this return a new value instead of mutating the original one.
        let mut rng = thread_rng();
        self.vec.shuffle(&mut rng);
    }

    pub fn reverse(&self) -> Hand {
        let mut new_vec: Vec<Card> = vec![];

        for i in &self.vec {
            new_vec.push(*i)
        }
        new_vec.reverse();

        Hand::from(
            new_vec
        )
    }

    /// removes the first indexed card in the hand and returns its value
    /// 
    /// mutates the hand; doesn't return a new hand
    pub fn pop_top(&mut self) -> Card {
        self.vec.reverse();
        let first_card = self.vec.pop().unwrap();
        self.vec.reverse();
        first_card
    }

    /// removes the last indexed card in the hand and returns its value
    /// 
    /// mutates the hand; doesn't return a new hand
    pub fn pop_bottom(&mut self) -> Card {
        self.vec.pop().unwrap()
    }

    pub fn push_top(&self, card: Card) -> Hand {
        self.reverse().push_bottom(card).reverse()
    }

    pub fn push_bottom(&self, card: Card) -> Hand {
        let mut clone = self.vec.clone();
        clone.push(card);
        Hand::from(
            clone
        )
    }

    //TODO: this
    //pub fn insert(&self, index: usize) -> Hand {}
}