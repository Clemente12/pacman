//!
//!
//! stdlib docs: https://doc.rust-lang.org/std/
//!
extern crate pacman;

use pacman::Game;
use pacman::Input;
use pacman::Key;

use std::sync::mpsc::channel;
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


fn main()
{
    let (insnk, insrc) = channel();
    let events = Input::new(insnk);
    let _      = events.start();

    let mut game = Game::load(&MAP);

    loop
    {
        let mut keys = Vec::new();
        let mut key  = None;

        loop
        {
            if let Ok(key) = insrc.try_recv() {
                keys.push(key);
            } else {
                break;
            }
        }

        if keys.len() == 3
        {
            let arrow = (keys[0], keys[1], keys[2]);

            match arrow
            {
                (27, 91, 67) => key = Some(Key::ArrowRight),   // game.player.pos.x += 1,
                (27, 91, 68) => key = Some(Key::ArrowLeft),    // game.player.pos.x -= 1,
                (27, 91, 66) => key = Some(Key::ArrowDown),    // game.player.pos.y += 1,
                (27, 91, 65) => key = Some(Key::ArrowUp),      // game.player.pos.y -= 1,

                _ => ()
            }
        }

        println!("{:?}", keys);

        game.update(&key);
        game.render();

        print!("\x1bc");

        sleep(Duration::from_millis(200));
    }
}
