//!
//! Ghost
//!
use ::Key;
use ::Map;
use ::Canvas;
use ::Position;

use ::Render;
use ::Update;


#[derive(Debug)]
pub struct Ghost
{
    pub pos       : Position,
    pub is_edible : bool,
}


impl Ghost
{
    pub fn new(pos: Position) -> Self
    {
        return Self {pos: pos, is_edible: false};
    }
}


impl Update for Ghost
{
    fn update(&mut self, map: &Map, _: &Option<Key>)
    {

    }
}


impl Render for Ghost
{
    fn draw(&self, canvas: &mut Canvas)
    {
        canvas[self.pos.y][self.pos.x] = 'A'
    }
}
