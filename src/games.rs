#![allow(dead_code)]

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

    //pub fn score_hand(hand: &Hand) -> PokerHand {}
}


pub mod blackjack {
    //use crate::cards;
    use crate::hand::Hand;
    use crate::cards::Value;

    /// Scores a blackjack hand to the highest aount it can be without going over 21
    /// if the hand is worth more than 21, it returns 0.
    pub fn score_hand(hand: &Hand) -> i32 {
        let mut score = 0;
        let mut aces = 0;

        for card in &hand.vec {
            score += score_card(&card.value);
            if &card.value == &Value::Ace {
                aces += 1
            }
        }

        while score > 21 && aces > 0 {
            score -= 10;
            aces -= 1
        }
        if score > 21 {
            return 0
        }
        return score
    }

    /// scores ace as 11
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
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        }
    }
}