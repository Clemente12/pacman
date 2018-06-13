//!
//! Game Map
//!
use ::Render;
use ::Canvas;
use ::Cell;


#[derive(Debug)]
pub struct Map
{
    cells: Vec<Cell>,
}


impl Map
{
    pub fn new(cells: Vec<Cell>) -> Self
    {
        return Self {cells: cells};
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell
    {
        for cell in &self.cells
        {
            if cell.pos.x == x && cell.pos.y == y {
                return cell;
            }
        }

        unreachable!()
    }

    pub fn width(&self) -> usize
    {
        let mut x = 0;

        for cell in &self.cells
        {
            if cell.pos.x > x {
                x = cell.pos.x;
            }
        }

        return x + 1;
    }

    pub fn heigth(&self) -> usize
    {
        let mut y = 0;

        for cell in &self.cells
        {
            if cell.pos.y > y {
                y = cell.pos.y;
            }
        }

        return y + 1;
    }
}


impl Render for Map
{
    fn draw(&self, canvas: &mut Canvas)
    {
        for cell in &self.cells {
            cell.draw(canvas);
        }
    }
}