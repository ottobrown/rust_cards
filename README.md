# rust-cards
---

A simple library for simulating playing cards

## Structure
A Card is a struct with 2 fields, elements of the enums Suit and Value.
A Hand is a struct consisting of a Vec of Card

    Suit ----↓
            Card ----→ [Card Card Card Card Card] --→ Hand
    Value ---↑

## Examples

Creates a deck, shuffles it, and pops the top 5 cards onto a Hand
```rust
use rust_cards::cards::Card;
use rust_cards::hand::Hand;


fn main() {
    let mut deck = Hand::full_deck();
    deck = deck.shuffle();

    //pops the top 5 cards off the deck and puts them in a hand 
    let top_five_cards = Hand::from(
        vec![
            deck.pop_top(),
            deck.pop_top(),
            deck.pop_top(),
            deck.pop_top(),
            deck.pop_top(),
        ]
    );

    for card in top_five_cards.vec {
        println!("{}", card.string())
    }

}
```

creates a hand consinsting of A♠, 2♥, 3♦, 4♣
```rust
use rust_cards::cards::Card;
use rust_cards::hand::Hand;
use rust_cards::cards::Suit;
use rust_cards::cards::Value;

fn main() {
    let set_hand = Hand::from(
        vec![
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Three, Suit::Diamond),
            Card::new(Value::Four, Suit::Club)
        ]
    );

    for card in set_hand.vec {
        println!("{}", card.string())
    }
    //A♠
    //2♥
    //3♦
    //4♣
}
```

##License
[MIT](https://choosealicense.com/licenses/mit/) or [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)