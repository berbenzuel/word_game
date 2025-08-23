use std::error::Error;
use std::fmt::Display;
use iced::Task;
use serde::de::Unexpected::Str;
use serde::Deserialize;
use log::error;

//"suggestion": "a, adv, conj, interj, n, p, prep, pron, v"
#[doc = "The part of speech of a word."]
#[derive(Debug, Clone, Default)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Particle,
    #[default]
    Undefined
}
impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameCard {
    pub word: String,
    pub part_of_speech: PartOfSpeech,
    pub definition: String,
    pub shuffled: String
}

#[derive(Deserialize)]
struct RandomWord {
    word: String,
    pos: String,
    definitions: Vec<String>,
}

impl GameCard {
    pub fn new(word: String, definition: String, part_of_speech: PartOfSpeech) -> GameCard {
        GameCard {
            word: word.clone(),
            part_of_speech,
            definition,
            shuffled: Self::shuffle_word(&word),
        }
    }
    pub fn fetch_random() -> Result<GameCard, Box<dyn std::error::Error>> {
        if cfg!(debug_assertions) {
            return Ok(GameCard::new("TEST".to_string(), "this is simple test word".to_string(), PartOfSpeech::Noun));
        }

        let response = reqwest::blocking::get("https://api.msmc.cc/api/dictionary/random")?;
        let deserialized: RandomWord = response.json()?;
        Ok(GameCard::from(deserialized))
    }

    pub fn shuffle_word(word: &String) -> String {
        use rand::seq::SliceRandom;

        let mut chars = word.chars().collect::<Vec<char>>();
        chars.shuffle(&mut rand::rng());
        chars.iter().collect::<String>()
    }
}


impl From<RandomWord> for GameCard {
    fn from(random_word: RandomWord) -> Self {
        GameCard {
            word: random_word.word.clone(),
            shuffled: Self::shuffle_word(&random_word.word),
            part_of_speech:
                match random_word.pos.as_str() {
                    "a." => PartOfSpeech::Adjective,
                    "adv." => PartOfSpeech::Adverb,
                    "conj." => PartOfSpeech::Conjunction,
                    "interj." => PartOfSpeech::Interjection,
                    "n." => PartOfSpeech::Noun,
                    "p." => PartOfSpeech::Particle,
                    "prep." => PartOfSpeech::Preposition,
                    "pron." => PartOfSpeech::Pronoun,
                    "v." => PartOfSpeech::Verb,
                    _ => PartOfSpeech::Undefined
                },
            definition: random_word.definitions.first()
                .unwrap_or(&String::from("undefined")).clone()

        }
    }

}
// #[test]
// fn from_test()

