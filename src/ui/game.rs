
use iced::{Element, Size, Theme};
use iced::widget::{Column, column, Button, Text, TextInput, Container, container};
use iced::window::{Icon, Level, Position, Settings};
use iced::window::settings::PlatformSpecific;
use crate::game_card::GameCard;






#[derive(Debug, Clone)]
pub enum Message {
    InputContentChanged(String),
    NextWord,
    StartGame
}


enum Screen {
    Menu,
    Game
}
impl Screen {

}

pub struct Game {
    input_content: Option<String>,
    game_card: Option<GameCard>,
    screen: Screen,
}

impl Default for Game {
    fn default() -> Self {


        Game {
            screen: Screen::Menu,
            input_content: None,
            game_card: None,
        }
    }
}


impl Game {

    pub fn update(&mut self, _message: Message) -> iced::Task<Message> {
        match _message {
            Message::NextWord => {self.on_next_word()}
            Message::InputContentChanged(str) => {

                if let Some(str) = self.input_content.clone()
                    && let Some(to_check) = self.game_card.clone()
                    && str == to_check.word {
                    self.on_next_word();
                }
            }
            Message::StartGame => {self.screen = Screen::Game}
        }
        iced::Task::none()
    }

    pub fn view(&self) -> Element<Message> {

        let element =
            match &self.screen {
                Screen::Menu => self.menu(),
                Screen::Game => self.game(),
            };

        Element::new(element)


    }

    fn game(&self) -> Column<'_, Message> {
        let game_card = GameCard::fetch_random();
        column![
            Text::new(game_card.word.clone()),
            Text::new(game_card.definition.clone()),

            Button::new("next").on_press(Message::NextWord),

            TextInput::new("your typo:", &self.input_content.clone().unwrap())
                .on_input(Message::InputContentChanged)
        ]
    }

    fn menu(&self) -> Column<'_, Message> {
        column![
            Button::new("start").on_press(Message::StartGame),
            ]
    }

    fn on_next_word(&mut self) {
        self.game_card = Some(GameCard::fetch_random());
        self.input_content = Some(String::new());
    }


    pub fn title(&self) -> String {
        String::from("Word game")
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

}
