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
#.#...........#.# ####### #.#...........#.#
#.######.######.#         #.######.######.#
#...............  #######  ...............#
#.#############.# #     # #.#############.#
#.............#.# #     # #.#.............#
#.#############.# ####### #.#############.#
#...............#         #...............#
####### ######### ### ### ####### #########
                # #AAAAA# #
####### ######### ####### ####### #########
#...............#         #...............#
#.######.######.# ####### #.######.######.#
#.#...........#.# #     # #.#...........#.#
#.######.######.# #     # #.######.######.#
#...............  #######  ...............#
#.#############.#    <    #.#############.#
#.............#.# ####### #.#.............#
#.#############.# #     # #.#############.#
#...............# #     # #...............#
################# ####### #################
";


#[derive(Debug)]
struct Cell 
{
    has_point  : bool,
    has_wall   : bool,
    has_cherry : bool,
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

#[derive(Debug)]
struct Map 
{
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug)]
struct Game
{
    map    : Map,
    player : Player,
    ghosts : Vec<Ghost>,
}


impl fmt::Display for Cell 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        if self.has_point  { 
            return write!(f, "."); 
        } else if self.has_wall { 
            return write!(f, "#"); 
        } else if self.has_cherry { 
            return write!(f, "W"); 
        } else {
            return write!(f, " ");
        }
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
                cell.fmt(f)?;
            }

            write!(f, "\n");
        }

        return Ok(());
    }
}



impl fmt::Display for Game 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        return self.map.fmt(f);
    }
}


impl Map 
{
    fn new(cells: Vec<Vec<Cell>>) -> Self 
    {
        return Self {cells: cells};
    }
}


impl Game
{
    fn load(data: &str) -> Self
    {
        let mut cells:  Vec<Vec<Cell>> = Vec::new();
        let mut ghosts: Vec<Ghost>     = Vec::new();
        let mut player: Option<Player> = None;
        
        let mut y = 0;

        for l in data.split('\n') 
        {
            let mut line = Vec::<Cell>::new();
            let mut x = 0;

            for c in l.chars()
            {
                let cell = Cell {
                    has_point:  c == '.',
                    has_wall:   c == '#',
                    has_cherry: c == 'W',
                };
                
                if c == '<' {
                    player = Some(Player::new(Position::new(x, y)))
                }

                if c == 'A' {
                    let ghost = Ghost::new(Position::new(x, y));
                    ghosts.push(ghost);
                }

                line.push(cell);
                x += 1;
            }

            cells.push(line);
            y += 1;
        }

        let map = Map::new(cells);

        if player.is_none() {
            panic!("No player!");
        }

        return Self {map: map, player: player.unwrap(), ghosts: ghosts};
    }
}


impl Position
{
    fn new(x: u64, y: u64) -> Self
    {
        return Self {x: x, y: y};
    }
}

impl Ghost
{
    fn new(pos: Position) -> Self
    {
        return Self {pos: pos, is_edible: false};
    }
}

impl Player
{
    fn new(pos: Position) -> Self
    {
        return Self {pos: pos, score: 0};
    }
}


fn main() 
{
    let game = Game::load(&MAP);

    loop
    {
        println!("{}", game);

        print!("\x1bc");
        
        sleep(Duration::from_millis(20));
    }
}
