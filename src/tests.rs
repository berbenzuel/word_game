
#[cfg(test)]
mod tests {

    mod game_card_tests {
        use crate::game_card::*;
        #[test]
        fn test_shuffle_word() {
            let word = String::from("test");
            let shuffled = GameCard::shuffle_word(&word);
            assert_ne!(word, shuffled);
        }
    }

    mod game_tests {

    }



}