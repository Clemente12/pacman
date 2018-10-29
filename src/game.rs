//!
//! Main Game State
//!
use ::Position;
use ::Player;
use ::Key;
use ::Map;
use ::Ghost;
use ::Cell;
use ::Canvas;

use ::Update;
use ::Render;
use ::Reset;


#[derive(Debug)]
enum GameCondition
{
    GameOngoing,
    GameOver,
    GameWon,
}


#[derive(Debug)]
pub struct Game
{
    pub map    : Map,
    pub player : Player,
    pub ghosts : Vec<Ghost>,

    end : GameCondition
}



static BANNER_GAMEOVER: &'static str = "
########## ########## ####  #### ##########\n\
##         ##      ## ##  ##  ## ##        \n\
##         ##      ## ##  ##  ## ##        \n\
##    #### ########## ##      ## #######   \n\
##      ## ##      ## ##      ## ##        \n\
##      ## ##      ## ##      ## ##        \n\
########## ##      ## ##      ## ##########\n\
                                           \n\
########## ##      ## ########## ##########\n\
##      ## ##      ## ##         ##      ##\n\
##      ## ##      ## ##         ##      ##\n\
##      ## ##      ## #######    ##########\n\
##      ##  ##    ##  ##         ##    ##  \n\
##      ##   ##  ##   ##         ##     ## \n\
##########     ##     ########## ##      ##\n\
";

static BANNER_GAMEWON: &'static str = "
########## ########## ####  #### ##########\n\
##         ##      ## ##  ##  ## ##        \n\
##         ##      ## ##  ##  ## ##        \n\
##    #### ########## ##      ## #######   \n\
##      ## ##      ## ##      ## ##        \n\
##      ## ##      ## ##      ## ##        \n\
########## ##      ## ##      ## ##########\n\
                                           \n\
###        ### ########### ###       ##  ##\n\
 ##        ##  ##       ## ## ##     ##  ##\n\
 ##        ##  ##       ## ##  ##    ##  ##\n\
 ##        ##  ##       ## ##   ##   ##  ##\n\
  ##  ##  ##   ##       ## ##    ##  ##  ##\n\
   ## ## ##    ##       ## ##     ## ##    \n\
    ##  ##     ########### ##       ###  ##\n\
";


impl Game
{
    pub fn load(data: &str) -> Self
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

        // let map_width = map.width();
        // let map_height = map.height();

        if player.is_none() {
            panic!("No player!");
        }

        return Self {map: map, player: player.unwrap(), ghosts: ghosts, end: GameCondition::GameOngoing};
    }

    pub fn update(&mut self, key: &Option<Key>)
    {
        self.player.update(&mut self.map, &key);

        for ghost in self.ghosts.iter_mut() {
            ghost.update(&mut self.map, &key);
        }
    }

    pub fn endcondition(&mut self)
    {
        // ghost kills pacman (game over)
        for ghost in &mut self.ghosts
        {
            if self.player.pos == ghost.pos
            {
                self.player.lives -= 1;
                self.player.reset();
                ghost.reset();
            }
        }

        if self.player.lives <= 0 {
            self.end = GameCondition::GameOver;
        }

        // pacman eats all points (game won)
        let mut has_point = false;
        for cell in &self.map.cells
        {
            if cell.has_point()
            {
                has_point = true;
                break;
            }
        }

        if !has_point {
            self.end = GameCondition::GameWon;
        }
    }

    pub fn render(&self)
    {
        match self.end
        {
            GameCondition::GameWon => {
                println!("{}", BANNER_GAMEWON);
            },

            GameCondition::GameOver => {
                println!("{}", BANNER_GAMEOVER);
            },

            GameCondition::GameOngoing =>
            {
                // prepare canvas
                let mut canvas = Canvas::new();
                for _ in 0..self.map.height() {
                    canvas.push(vec![' '; self.map.width()]);
                }

                // draw objects
                self.map.draw(&mut canvas);
                self.player.draw(&mut canvas);

                for ghost in &self.ghosts {
                    ghost.draw(&mut canvas);
                }

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
                println!("Score: {}", self.player.score);
                println!("Lives: {}", self.player.lives);
            }
        }
    }
}
