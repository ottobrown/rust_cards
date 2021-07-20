#![allow(dead_code)]
use rand::thread_rng;
use rand::seq::SliceRandom;

//use crate::cards;
use crate::cards::Card;
use crate::cards::Suit;
use crate::cards::Value;

#[derive(Clone)]
pub struct Hand {
    pub vec: Vec<Card>
}
impl Hand {
    /// returns a hand from a Vec of Card
    pub fn from(vec: Vec<Card>) -> Hand {
        Hand {
            vec: vec
        }
    }

    /// returns a Hand containing a full deck
    /// 
    /// ordered 2, 3, 4, ... J, Q, K, A
    /// Suits ordered Club(♣), Diamond(♦), Heart(♥), Spade(♠)
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

    ///Randomizes the order of the hand
    /// 
    /// returns a new value; does not mutate the original value
    pub fn shuffle(&self) -> Hand {
        let mut rng = thread_rng();

        let mut clone = self.vec.clone();
        clone.shuffle(&mut rng);

        Hand::from(
            clone
        )
    }

    /// reverses the hand
    /// i.e. the first index becomes the last index, the second index becomes the second-last index, etc.Hand
    /// 
    /// returns a new value; does not mutate the original value
    pub fn reverse(&self) -> Hand {
        let mut clone = self.vec.clone();
        clone.reverse();

        Hand::from(
            clone
        )
    }

    /// removes the first indexed card in the hand and returns its value
    /// 
    /// mutates the hand; doesn't return a new hand
    /// 
    /// ```
    /// let mut h = Hand::from(
    ///     vec![
    ///         Card::new(Value::Ace, Suit::Spade)
    ///         Card::new(Value::Two, Suit::Spade)
    ///         Card::new(Value::Three, Suit::Spade)
    ///     ]
    /// );
    /// // A♠, 2♠, 3♠
    /// 
    /// let popped = h.pop();
    /// // h == 2♠, 3♠
    /// // popped == A♠
    /// ```
    /// 
    pub fn pop_top(&mut self) -> Card {
        self.vec.reverse();
        let first_card = self.vec.pop().unwrap();
        self.vec.reverse();
        first_card
    }

    /// removes the last indexed card in the hand and returns its value
    /// 
    /// mutates the hand; doesn't return a new hand
    /// 
    /// ```
    /// let mut h = Hand::from(
    ///     vec![
    ///         Card::new(Value::Ace, Suit::Spade)
    ///         Card::new(Value::Two, Suit::Spade)
    ///         Card::new(Value::Three, Suit::Spade)
    ///     ]
    /// );
    /// // A♠, 2♠, 3♠
    /// 
    /// let popped = h.pop();
    /// // h == A♠, 2♠
    /// // popped == 3♠
    /// ```
    /// 
    pub fn pop_bottom(&mut self) -> Card {
        self.vec.pop().unwrap()
    }

    /// Pushes a card to the top of the hand
    /// ```
    /// let mut h = Hand::from(
    ///     vec![
    ///         Card::new(Value::Ace, Suit::Spade)
    ///         Card::new(Value::Two, Suit::Spade)
    ///         Card::new(Value::Three, Suit::Spade)
    ///     ]
    /// );
    /// // A♠, 2♠, 3♠
    /// 
    /// h = h.push(Card::new(Value::King, Suit::Spade))
    /// // K♠, A♠, 2♠, 3♠
    /// ```
    pub fn push_top(&self, card: Card) -> Hand {
        self.reverse().push_bottom(card).reverse()
    }

    /// Pushes a card to the bottom of the hand
    /// ```
    /// let mut h = Hand::from(
    ///     vec![
    ///         Card::new(Value::Ace, Suit::Spade)
    ///         Card::new(Value::Two, Suit::Spade)
    ///         Card::new(Value::Three, Suit::Spade)
    ///     ]
    /// );
    /// // A♠, 2♠, 3♠
    /// 
    /// h = h.push(Card::new(Value::Four, Suit::Spade))
    /// // A♠, 2♠, 3♠, 4♠
    /// ```
    pub fn push_bottom(&self, card: Card) -> Hand {
        let mut clone = self.vec.clone();
        clone.push(card);
        Hand::from(
            clone
        )
    }

    pub fn insert(&self, index: usize, card: Card) -> Hand {
        let mut clone = self.vec.clone();
        clone.insert(index, card);

        Hand::from(
            clone
        )
    }

    /// Stacks a hand on top of another Hand
    /// 
    /// ```
    /// 
    /// ```
    pub fn stack_top(&self, hand: Hand) -> Hand {
        let mut copy = self.clone();
        for card in hand.reverse().vec {
            copy = copy.push_top(card);
        }

        copy
    }
    pub fn stack_bottom(&self, hand: Hand) -> Hand {
        let mut copy = self.clone();

        for card in hand.vec {
            copy = copy.push_bottom(card);
        }

        copy
    }

}