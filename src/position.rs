//!
//! Position Type
//!


#[derive(Debug)]
pub struct Position
{
    pub x : usize,
    pub y : usize,
}


impl Position
{
    pub fn new(x: usize, y: usize) -> Self
    {
        return Self {x: x, y: y};
    }
}
