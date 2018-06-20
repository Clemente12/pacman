//!
//! Common Traits implemented across the Game
//!
use ::Key;
use ::Map;
use ::Canvas;


pub trait Render
{
    fn draw(&self, canvas: &mut Canvas);
}


pub trait Update
{
    fn update(&mut self, game: &mut Map, keyevent: &Option<Key>);
}
