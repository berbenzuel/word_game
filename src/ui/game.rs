
use iced::{Element, Size, Theme};
use iced::widget::{Column, column, Button, Text, TextInput};
use iced::window::{Icon, Level, Position, Settings};
use iced::window::settings::PlatformSpecific;
use crate::game_card::GameCard;


const WIN_DEFAULT_SIZE:Size<f32> = Size::new(600.0, 600.0);




#[derive(Debug, Clone)]
pub enum Message {
    InputContentChanged(String),
    NextWord
}



pub struct Game {
    card: GameCard,
    mixed: String,
    input_content: String,
}
impl Default for Game {
    fn default() -> Self {
        let card = GameCard {
            word: String::from("Word_Game"),
            pos: String::new(),
            definitions: vec![String::from("to start typo Word_Game")]
        };

        Game {
            card: card.clone(),
            mixed: card.word,
            input_content: String::new(),
        }
    }
}


impl Game {

    pub fn update(&mut self, _message: Message) -> iced::Task<Message> {
        match _message {
            Message::NextWord => {self.on_next_word()}
            Message::InputContentChanged(str) => {
                self.input_content = str;
                if self.input_content.to_lowercase() == self.card.word.to_lowercase() {
                    self.on_next_word();
                }
            }
        }
        iced::Task::none()
    }

    pub fn view(&self) -> Element<Message> {

        let definitions = self.card.definitions.iter()
            .map(|x| Text::new(x))
                .collect::<Vec<Text>>();



        let column = column![
            Text::new(self.mixed.clone()),
            Text::new(self.card.definitions.first()
                .unwrap_or(&String::from("no definition"))
                .to_string()),

            Button::new("next").on_press(Message::NextWord),

            TextInput::new("your typo:", &self.input_content)
                .on_input(Message::InputContentChanged)
        ];
        Element::new(column)
    }

    fn on_next_word(&mut self) {

        use rand::seq::SliceRandom;

        self.card = GameCard::fetch_random();
        let mut chars = self.card.word.chars().collect::<Vec<char>>();
        let mut rng = rand::rng();
        chars.shuffle(&mut rng);
        self.mixed = chars.iter().collect::<String>();
        self.input_content = String::new();
    }


    pub fn title(&self) -> String {
        String::from("Word game")
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn window_settings() -> iced::window::Settings {


        //later add functionality to resize -> responsibility
        iced::window::Settings {
            size: WIN_DEFAULT_SIZE,
            position: Position::Default,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            decorations: true,
            transparent: false,
            level: Level::Normal,
            icon: None,
            platform_specific: PlatformSpecific::default(),
            exit_on_close_request: true
        }

    }
}
