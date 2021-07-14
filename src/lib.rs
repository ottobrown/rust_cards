mod cards;
mod hand;
mod games;

#[cfg(test)]
mod tests {
    use crate::hand;
    use crate::hand::Hand;

    use crate::cards;
    use cards::Suit;
    use cards::Value;

    use crate::games;

    #[test]
    fn blackjack_game() {
        let mut deck = Hand::full_deck();
        deck.shuffle();

        let mut hand = Hand::from(vec![
            deck.pop_top(),
            deck.pop_top()
        ]);

        loop {
            print_hand(&hand);
            println!(": ");
            let s = games::blackjack::score_hand(&hand);
            if s != 0 {println!("{}", s)}
            else {
                println!("You busted!");

                if get_input("continue? [Y/N]").to_lowercase() == "y" {
                    blackjack_game()
                }
                else {
                    return 
                }
            }

            let choice = get_input("Hit or stand?");

            if choice == "hit" ||
            choice == "h" {
                hand = hand.push_bottom(deck.pop_top());
            }
            else {
                println!("final score: {}", s);
                if get_input("continue? [Y/N]").to_lowercase() == "y" {
                    blackjack_game()
                }
                else {
                    return 
                }
            }
        }

    }

    //not a test
    fn print_hand(hand: &hand::Hand) {
        for card in &hand.vec {
            let v = cards::Value::string(card.value);
            let s = cards::Suit::string(card.suit);

            print!("{}{} ", v, s);
        }
    }

    fn get_input(prompt: &str) -> String{
        println!("{}", prompt);
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }


}
