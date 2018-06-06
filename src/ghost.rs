//!
//! Ghost
//!
use ::Render;
use ::Canvas;
use ::Position;


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


impl Render for Ghost
{
    fn draw(&self, canvas: &mut Canvas)
    {
        canvas[self.pos.y][self.pos.x] = 'A'
    }
}
