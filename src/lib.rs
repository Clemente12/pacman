//!
//! PacMan Game Library
//!
extern crate termios;

mod cell;
mod game;
mod ghost;
mod input;
mod map;
mod player;
mod position;
mod traits;

pub use traits::Render;
pub use position::Position;

pub use cell::Cell;
pub use map::Map;
pub use ghost::Ghost;
pub use player::Player;
pub use game::Game;

pub use input::Input;
pub use input::InputSource;
pub use input::InputSink;

type Canvas = Vec<Vec<char>>;
