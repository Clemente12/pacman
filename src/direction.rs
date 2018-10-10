//!
//! Enumeration Representing Cardinal Directions
//!


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction
{
    Standing,
    Left,
    Right,
    Up,
    Down,
}


impl Direction
{
    pub inverse(&self) -> Direction
    {
        match self
        {
            Direction::Standing => Direction::Standing,
            Direction::Left     => Direction::Right,
            Direction::Right    => Direction::Left,
            Direction::Up       => Direction::Down,
            Direction::Down     => Direction::Up,
        }
    }
}
