//!
//! Common Traits implemented across the Game
//!
use ::Canvas;


pub trait Render
{
    fn draw(&self, canvas: &mut Canvas);
}
