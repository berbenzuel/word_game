use iced::{application, Size};
use iced::window::{Level, Position, Settings};
use iced::window::settings::PlatformSpecific;
use crate::ui::game::Game;

mod ui;
mod game_card;
mod tests;

const WIN_DEFAULT_SIZE:Size<f32> = Size::new(600.0, 600.0);


fn main() -> iced::Result{
    let app = application(Game::title, Game::update, Game::view)
        .theme(Game::theme)
        .window(Settings {
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
        });


    app.run()

}

