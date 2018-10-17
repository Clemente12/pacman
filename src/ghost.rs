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
struct DirProb
{
    direction   : Direction,
    probability : f32
}


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

// let ghost = Ghost {origin: _, pos: _, direc}

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
        let map_width  = (map.width() as i64)  - 1;
        let map_height = (map.height() as i64) - 1;

        let prob = rand::random::<f32>();

        let mut dirprobs = Vec::new();

        if (self.pos.x as i64 + 1 > map_height) || ! map.get_cell(self.pos.x + 1, self.pos.y).is_wall() {
            dirprobs.push(DirProb {direction: Direction::Right, probability: 0.0});
        }

        if (self.pos.x as i64 - 1 < 0) || ! map.get_cell(self.pos.x - 1, self.pos.y).is_wall() {
            dirprobs.push(DirProb {direction: Direction::Left, probability: 0.0});
        }

        if (self.pos.y as i64 + 1 > map_height) || ! map.get_cell(self.pos.x, self.pos.y + 1).is_wall() {
            dirprobs.push(DirProb {direction: Direction::Down, probability: 0.0});
        }

        if (self.pos.y as i64 - 1 < 0) || ! map.get_cell(self.pos.x, self.pos.y - 1).is_wall() {
            dirprobs.push(DirProb {direction: Direction::Up, probability: 0.0});
        }

        let mut prob_space = 1.0;
        let mut prob_acc   = 0.0;
        let mut prob_count = dirprobs.len();

        // assign half of the otherwise corresponding probability to the inverse direction
        for dirprob in dirprobs.iter_mut()
        {
            if dirprob.direction.inverse() == self.direction
            {
                dirprob.probability  = (1.0 / prob_count as f32) * 0.2;

                prob_space -= dirprob.probability;
                prob_count -= 1;
            }
        }

        // assign the rest of the probability space to the remaining directions
        let portion = prob_space / prob_count as f32;

        for dirprob in dirprobs.iter_mut()
        {
            if dirprob.direction.inverse() != self.direction {
                dirprob.probability = portion;
            };
        }

        // select direction from probability wheel
        for dirprob in dirprobs.iter()
        {
            prob_acc += dirprob.probability;

            if prob < prob_acc
            {
                self.direction = dirprob.direction;
                break;
            }
        }

        // move if we have a moving direction
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
        canvas[self.pos.y][self.pos.x] = 'A';
    }
}
