




## Examples

```rust
    use rust_cards::cards::Card;
    use rust_cards::hand::Hand;

    let mut deck = Hand::full_deck();
    deck = deck.shuffle

    let mut hand = Hand::from(
        vec![
            deck.pop_top(),
            deck.pop_top(),
            deck.pop_top(),
            deck.pop_top(),
            deck.pop_top(),
        ]
    )

    for card in hand {
        println("{}", card.string())
    }
```