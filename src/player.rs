//!
//! PacMan himself
//!
use ::Key;
use ::Map;
use ::Canvas;
use ::Position;

use ::Update;
use ::Render;


#[derive(Debug, Eq, PartialEq)]
pub enum Direction
{
    Standing,
    Left,
    Right,
    Up,
    Down,
}


#[derive(Debug)]
pub struct Player
{
    pub pos   : Position,
    pub dir   : Direction,
    pub score : u64,
}


impl Player
{
    pub fn new(pos: Position) -> Self
    {
        return Self {pos: pos, dir: Direction::Standing, score: 0};
    }
}


impl Update for Player
{
    fn update(&mut self, map: &Map, key: &Option<Key>)
    {
        if let Some(k) = key
        {
            match k
            {
                Key::ArrowRight => self.dir = Direction::Right,
                Key::ArrowLeft  => self.dir = Direction::Left,
                Key::ArrowDown  => self.dir = Direction::Down,
                Key::ArrowUp    => self.dir = Direction::Up,
            }
        }

        if self.dir != Direction::Standing
        {
            let mut new_x = self.pos.x;
            let mut new_y = self.pos.y;

            match self.dir
            {
                Direction::Right => new_x += 1,
                Direction::Left  => new_x -= 1,
                Direction::Down  => new_y += 1,
                Direction::Up    => new_y -= 1,
                _ => ()
            }

            let cell = map.get_cell(new_x, new_y);

            if !cell.is_wall()
            {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }
    }
}


impl Render for Player
{
    fn draw(&self, canvas: &mut Canvas)
    {
        canvas[self.pos.y][self.pos.x] = '<';
    }
}
