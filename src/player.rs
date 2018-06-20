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
    fn update(&mut self, map: &mut Map, key: &Option<Key>)
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
            let mut new_x = self.pos.x as i64;
            let mut new_y = self.pos.y as i64;

            match self.dir
            {
                Direction::Right => new_x += 1,
                Direction::Left  => new_x -= 1,
                Direction::Down  => new_y += 1,
                Direction::Up    => new_y -= 1,
                _ => ()
            }

            let map_width  = (map.width() as i64)  - 1;
            let map_height = (map.height() as i64) - 1;

            if new_x > map_width  { new_x = 0; }
            if new_x < 0          { new_x = map_width; }
            if new_y > map_height { new_y = 0; }
            if new_y < 0          { new_y = map_height; }

            let cell = map.get_cell_mut(new_x as usize, new_y as usize);

            if !cell.is_wall()
            {
                self.pos.x = new_x as usize;
                self.pos.y = new_y as usize;
            }

            if cell.has_point()
            {
                self.score += 10;
                cell.has_point = false;
            }
        }
    }
}


impl Render for Player
{
    fn draw(&self, canvas: &mut Canvas)
    {
        let icon;

        match self.dir
        {
            Direction::Right    => icon = '<',
            Direction::Left     => icon = '>',
            Direction::Down     => icon = '^',
            Direction::Up       => icon = 'v',
            Direction::Standing => icon = '<',
        }

        canvas[self.pos.y][self.pos.x] = icon;
    }
}
