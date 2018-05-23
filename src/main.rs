//!
//!
//! stdlib docs: https://doc.rust-lang.org/std/
//!

use std::time::Duration;
use std::thread::sleep;

use std::fmt;
 

static MAP: &'static str = "
################# ####### #################
#...............# #     # #...............#
#.######.######.# #     # #.######.######.#
#.#.....A.....#.# ####### #.#.....A.....#.#
#.######.######.#         #.######.######.#
#...............  #######  ...............#
#.#############.# #     # #.#############.#
#.......A.....#.# #     # #.#.....A.......#
#.#############.# ####### #.#############.#
#...............#         #...............#
####### ######### ### ### ####### #########
                # #  <  # #
####### ######### ####### ####### #########
#...............#         #...............#
#.######.######.# ####### #.######.######.#
#.#.....A.....#.# #     # #.#.....A.....#.#
#.######.######.# #     # #.######.######.#
#...............  #######  ...............#
#.#############.#         #.#############.#
#.......A.....#.# ####### #.#.....A.......#
#.#############.# #     # #.#############.#
#...............# #     # #...............#
################# ####### #################
";


#[derive(Debug)]
struct Cell 
{
    has_point  : bool,
    has_ghost  : bool,
    has_wall   : bool,
    has_cherry : bool,
}


impl fmt::Display for Cell 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        if self.has_point  { return write!(f, "."); }
        if self.has_ghost  { return write!(f, "A"); }
        if self.has_wall   { return write!(f, "#"); }
        if self.has_cherry { return write!(f, "W"); }

        Ok(())        
    }
}


#[derive(Debug)]
struct Map 
{
    cells: Vec<Vec<Cell>>,
}


impl Map 
{
    fn load(data: &str) -> Self
    {
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for l in data.split('\n') 
        {
            let mut line = Vec::<Cell>::new();

            for c in l.chars()
            {
                let cell = Cell {
                    has_point:  c == '.',
                    has_ghost:  c == 'A',
                    has_wall:   c == '#',
                    has_cherry: c == 'W',
                };

                line.push(cell);
            }

            cells.push(line);
        }

        return Self { cells: cells };
    }
}


impl fmt::Display for Map 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        for row in &self.cells 
        {
            for cell in row 
            {
                if cell.has_point  { return write!(f, "."); }
                if cell.has_ghost  { return write!(f, "A"); }
                if cell.has_wall   { return write!(f, "#"); }
                if cell.has_cherry { return write!(f, "W"); }
            }

            write!(f, "\n");
        }

        Ok(())        
    }
}


#[derive(Debug)]
struct Game
{
    map    : Map,
    player : Player,
    ghosts : Vec<Ghost>,
}

#[derive(Debug)]
struct Position
{
    x : u64, 
    y : u64,
}

#[derive(Debug)]
struct Player
{
    pos   : Position,
    score : u64,
}

#[derive(Debug)]
struct Ghost
{
    pos       : Position,
    is_edible : bool,
}


fn main() 
{
    let m = Map::load(&MAP);
    
    loop
    {
        println!("{}", m);

        print!("\x1bc");
        
        sleep(Duration::from_millis(20));
    }
}
