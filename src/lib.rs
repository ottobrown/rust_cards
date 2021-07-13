mod cards;
use cards::Hand;

#[cfg(test)]
mod tests {
    use crate::cards;


    #[test]
    fn it_works() {
        let deck = cards::Hand::full_deck();

        for card in deck.vec {
            let v = cards::Value::string(card.value);
            let s = cards::Suit::string(card.suit);

            println!("{}{}", v, s)
        }
    }
}
