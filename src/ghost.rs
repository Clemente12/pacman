//!
//! Ghost
//!
use ::rand;
use ::Key;

use ::Map;
use ::Canvas;
use ::Position;
use ::Direction;

use ::Render;
use ::Update;
use ::Reset;


#[derive(Debug)]
pub struct Ghost
{
    pub origin    : Position,
    pub pos       : Position,
    pub direction : Direction,
    pub is_edible : bool,
}


impl Ghost
{
    pub fn new(pos: Position) -> Self
    {
        return Self {origin: pos, pos: pos, direction: Direction::Standing, is_edible: false};
    }
}

impl Reset for Ghost
{
    fn reset(&mut self)
    {
        self.pos = self.origin;
    }
}

impl Update for Ghost
{
    fn update(&mut self, map: &mut Map, _: &Option<Key>)
    {
        let prob = rand::random::<f32>();

        let mut dirs = Vec::new();

        if ! map.get_cell(self.pos.x + 1, self.pos.y).is_wall() {
            dirs.push(Direction::Right);
        }
        if ! map.get_cell(self.pos.x - 1, self.pos.y).is_wall() {
            dirs.push(Direction::Left);
        }
        if ! map.get_cell(self.pos.x, self.pos.y + 1).is_wall() {
            dirs.push(Direction::Down);
        }
        if ! map.get_cell(self.pos.x, self.pos.y - 1).is_wall() {
            dirs.push(Direction::Up);
        }

        let portion = 1.0 / dirs.len() as f32;

        for (i, dir) in dirs.iter().enumerate()
        {
            if prob < portion * (i as f32 + 1.0)
            {
                self.direction = *dir;
                break;
            }
        }

        if self.direction != Direction::Standing
        {
            let mut new_x = self.pos.x as i64;
            let mut new_y = self.pos.y as i64;

            match self.direction
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

            self.pos.x = new_x as usize;
            self.pos.y = new_y as usize;
        }
    }
}


impl Render for Ghost
{
    fn draw(&self, canvas: &mut Canvas)
    {
        canvas[self.pos.y][self.pos.x] = 'A'
    }
}
