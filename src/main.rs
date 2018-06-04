//!
//!
//! stdlib docs: https://doc.rust-lang.org/std/
//!

use std::time::Duration;
use std::thread::sleep;


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


type Canvas = Vec<Vec<char>>;


trait Render
{
    fn draw(&self, canvas: &mut Canvas);
}

#[derive(Debug)]
struct Position
{
    x : usize, 
    y : usize,
}

#[derive(Debug)]
struct Cell 
{
    pos        : Position,
    has_point  : bool,
    has_wall   : bool,
    has_cherry : bool,
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
    cells: Vec<Cell>,
}

#[derive(Debug)]
struct Game
{
    map    : Map,
    player : Player,
    ghosts : Vec<Ghost>,
}


impl Render for Cell 
{
    fn draw(&self, canvas: &mut Canvas)
    {
        if self.has_point  { 
            return canvas[self.pos.y][self.pos.x] = '.';
        } else if self.has_wall { 
            return canvas[self.pos.y][self.pos.x] = '#'; 
        } else if self.has_cherry { 
            return canvas[self.pos.y][self.pos.x] = 'W'; 
        } else {
            return canvas[self.pos.y][self.pos.x] = ' ';
        }
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


impl Map 
{
    fn new(cells: Vec<Cell>) -> Self 
    {
        return Self {cells: cells};
    }

    fn width(&self) -> usize
    {
        let mut x = 0;

        for cell in &self.cells
        {
            if cell.pos.x > x {
                x = cell.pos.x;
            } 
        } 
        
        return x;
    }

    fn heigth(&self) -> usize
    {
        let mut y = 0;

        for cell in &self.cells
        {
            if cell.pos.y > y {
                y = cell.pos.y;
            } 
        } 
        
        return y;       
    }
}


impl Game
{
    fn load(data: &str) -> Self
    {
        let mut cells:  Vec<Cell>      = Vec::new();
        let mut ghosts: Vec<Ghost>     = Vec::new();
        let mut player: Option<Player> = None;
        
        let mut y = 0;

        for l in data.split('\n') 
        {
            let mut x = 0;

            for c in l.chars()
            {
                let cell = Cell {
                    pos:        Position::new(x, y),
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

                cells.push(cell);
                x += 1;
            }

            y += 1;
        }

        let map = Map::new(cells);

        if player.is_none() {
            panic!("No player!");
        }

        return Self {map: map, player: player.unwrap(), ghosts: ghosts};
    }

    fn render(&self)
    {
        // prepare canvas
        let mut canvas = Canvas::new();
        for _ in 0..self.map.heigth() {
            canvas.push(Vec::with_capacity(self.map.width()));
        }

        // draw objects
        self.map.draw(&mut canvas);
        // self.player.draw(&mut canvas);

        // for ghost in &self.ghosts {
            // ghost.draw(&mut canvas);
        // }

        // output to screen
        let mut buffer = String::new();
        for row in canvas 
        {
            for col in row {
                buffer.push(col);
            }

            buffer.push('\n');
        }

        println!("{}", buffer);
    }
}


impl Position
{
    fn new(x: usize, y: usize) -> Self
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
        game.render();
        
        print!("\x1bc");
        
        sleep(Duration::from_millis(20));
    }
}
