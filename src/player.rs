//!
//! PacMan himself
//!
use ::Render;
use ::Canvas;
use ::Position;


#[derive(Debug)]
pub struct Player
{
    pub pos   : Position,
    pub score : u64,
}


impl Player
{
    pub fn new(pos: Position) -> Self
    {
        return Self {pos: pos, score: 0};
    }
}


impl Render for Player
{
    fn draw(&self, canvas: &mut Canvas)
    {
        canvas[self.pos.y][self.pos.x] = '<';
    }
}
