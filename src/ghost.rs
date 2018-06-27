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

    }
}


impl Render for Ghost
{
    fn draw(&self, canvas: &mut Canvas)
    {
        canvas[self.pos.y][self.pos.x] = 'A'
    }
}
