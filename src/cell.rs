//!
//! Single Map Cell
//!
use ::Render;
use ::Canvas;
use ::Position;


#[derive(Debug)]
pub struct Cell
{
    pub pos : Position,

    pub has_point  : bool,
    pub has_wall   : bool,
    pub has_cherry : bool,
}


impl Render for Cell
{
    fn draw(&self, canvas: &mut Canvas)
    {
        if self.has_point  {
            return canvas[self.pos.y][self.pos.x] = '.';
        } else if self.has_wall {
            return canvas[self.pos.y][self.pos.x] = '#';
        } else if self.has_cherry {
            return canvas[self.pos.y][self.pos.x] = 'W';
        } else {
            return canvas[self.pos.y][self.pos.x] = ' ';
        }
    }
}
