pub mod cards;
pub mod hand;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::hand;
    use crate::hand::Hand;

    use crate::cards;

    #[test]
    fn full_hand() {
        print_hand(&hand::Hand::full_deck() )
    }

    pub fn score_hand(hand: &Hand) -> i32 {
        let mut score = 0;
        let mut aces = 0;

        for card in &hand.vec {
            score += score_card(&card.value);
            if &card.value == &cards::Value::Ace {
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
    fn score_card(card: &cards::Value) -> i32 {
        match card {
            cards::Value::Two => 2,
            cards::Value::Three => 3,
            cards::Value::Four => 4,
            cards::Value::Five => 5,
            cards::Value::Six => 6,
            cards::Value::Seven => 7,
            cards::Value::Eight => 8,
            cards::Value::Nine => 9,
            cards::Value::Ten => 10,
            cards::Value::Jack => 10,
            cards::Value::Queen => 10,
            cards::Value::King => 10,
            cards::Value::Ace => 11,
        }
    }

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
            let s = score_hand(&hand);
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

    #[test]
    fn random_cards() {
        let now = Instant::now();
        for _i in 0..100 {
            println!("{}", cards::Card::random().string())
        }

        println!("{} ms", now.elapsed().as_millis())
    }

    

    //not a test
    fn print_hand(hand: &hand::Hand) {
        for card in &hand.vec {
            print!("{}", card.string())
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
