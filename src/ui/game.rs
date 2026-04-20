use std::rc::Rc;
use iced::{Alignment, Element, Left, Padding, Theme};
use iced::widget::{column, Button, Text, TextInput, Container, container, row};
use iced::widget::pane_grid::Axis::Vertical;
use crate::game_card::GameCard;
use crate::WIN_DEFAULT_SIZE;

const SHUFFLED_TEXT_FONT_SIZE: u16 = 40;

const APPLICATION_TITLE_LOWERCASE: &str = "word game";




#[derive(Debug, Clone)]
pub enum Message {
    InputContentChanged(String),
    NextWord,
    StartGame,
    Help
}

enum Status {
    Fetching,
    Fetched,
}
enum Screen {
    Menu,
    Game,
    Error(Box<dyn std::error::Error>)
}


pub struct Game {
    input_content: Option<String>,
    game_card: Option<GameCard>,
    game_content: Option<String>,
    screen: Screen,
    status: Option<Status>,
}

impl<'a> Default for Game{
    fn default() -> Self {

        Game {
            screen: Screen::Menu,
            input_content: None,
            game_card: None,
            status: None,
            game_content: None
        }
    }
}


impl<'a> Game{

    pub fn update(&mut self, _message: Message) -> iced::Task<Message> {
        match _message {
            Message::NextWord => {self.on_next_word()}
            Message::InputContentChanged(str) => {
                self.input_content = Some(str.clone());
                if let Some(to_check) = self.game_card.clone()
                    && str.to_uppercase() == to_check.word.to_uppercase() {
                    self.on_next_word();
                }
            }
            Message::StartGame => {
                self.screen = Screen::Game;
                self.on_next_word();
            }
            Message::Help => {
                let card = self.game_card.clone().unwrap().word;
                self.game_content = Some(card);
            }

        }
        iced::Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {

        let element =
            match &self.screen {
                Screen::Menu => self.menu(),
                Screen::Game => self.game(),
                Screen::Error(error) => self.error_win(error)
            };

        Element::new(element)


    }

    fn game(&self) -> Container<'_, Message> {
        let card_cloned = self.game_card.clone().unwrap_or_default();
        container (
        column![
            container(Text::new(self.game_content.clone().unwrap_or_default())
                        .size(SHUFFLED_TEXT_FONT_SIZE))
                .center_x(WIN_DEFAULT_SIZE.width)
                .align_bottom(50),

            container(Text::new(format!("~{}~",card_cloned.part_of_speech.to_string())))
                .align_top(30)
                .center_x(WIN_DEFAULT_SIZE.width),
            container(
                row![

                    container(TextInput::new("guess", &self.input_content.clone().unwrap())
                        .on_input(Message::InputContentChanged)
                        .width(WIN_DEFAULT_SIZE.width - 200f32 )
                        .align_x(Alignment::Center)
                    ),

                    container(Button::new("?")
                        .on_press(Message::Help)
                        .width(30)
                    )
                ].spacing(5).padding(Padding::new(0f32).left(100))

            ),



            container(Text::new(format!("-{}",card_cloned.definition))
                .align_x(Alignment::Start)
                .width(WIN_DEFAULT_SIZE.width - 50f32 )
            ).center_x(WIN_DEFAULT_SIZE.width)
            .padding(Padding::new(0f32).left(100).right(100))


        ].spacing(5)
        ).center(WIN_DEFAULT_SIZE.height)

    }

    fn error_win(&self, error: &'a Box<dyn std::error::Error>) -> Container<'_, Message> {
        container(
            column![
            Text::new(error.to_string()),
            Button::new("retry").on_press(Message::NextWord),
        ]).center_x(WIN_DEFAULT_SIZE.width)
            .center_y(WIN_DEFAULT_SIZE.height)
    }

    fn menu(&self) -> Container<'_, Message> {

        container(Button::new("start").on_press(Message::StartGame))
            .center_x(WIN_DEFAULT_SIZE.width)
            .center_y(WIN_DEFAULT_SIZE.height)

    }

    fn on_next_word(&mut self) {
        self.input_content = Some(String::new());
        match GameCard::fetch_random() {
            Ok(game_card) => {
                self.game_card = Some(game_card.clone());
                self.game_content = Some(game_card.shuffled)
            }
            Err(error) => {
                self.screen = Screen::Error(error);
            }

        }

    }


    pub fn title(&self) -> String {
        match &self.screen {
            Screen::Menu => format!("{}", APPLICATION_TITLE_LOWERCASE),
            Screen::Game => match &self.status {
                Some(Status::Fetching) => format!("{} - fetching", APPLICATION_TITLE_LOWERCASE),
                Some(Status::Fetched) => format!("{} - guess", APPLICATION_TITLE_LOWERCASE),
                None => format!("{}", APPLICATION_TITLE_LOWERCASE)
            },
            Screen::Error(_) => format!("{} - oops error", APPLICATION_TITLE_LOWERCASE),
        }

    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }

}
