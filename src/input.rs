//!
//! Unbuffered but blocking IO
//!
use ::termios;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use std::io;
use std::io::Read;
use std::io::Write;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread::Builder    as ThreadBuilder;
use std::thread::JoinHandle as ThreadHandle;


pub type InputSource = Sender<u8>;
pub type InputSink   = Receiver<u8>;


pub struct Input
{
    tx:           InputSource,
    orig_termios: Termios,
}


impl Input
{
    pub fn new(input_source: InputSource) -> Self
    {
        let orig = Termios::from_fd(0).unwrap();

        Self {tx: input_source, orig_termios: orig}
    }

    pub fn start(self) -> ThreadHandle<()>
    {
        // modify input behaviour (reset in drop)
        let mut new = self.orig_termios.clone();
        new.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(0, TCSANOW, &mut new).unwrap();

        let handle = ThreadBuilder::new()
            .name("INPUT".to_string())
            .spawn(move ||
            {
                let stdout     = io::stdout();
                let mut reader = io::stdin();
                let mut buffer = [0; 1];        // read exactly one byte

                loop
                {
                    stdout.lock()
                        .flush()
                        .unwrap();

                    reader.read_exact(&mut buffer)
                        .unwrap();

                    self.tx.send(buffer[0])
                        .unwrap();
                }
            })
            .expect("Unable to spawn input events thread");

        return handle;
    }
}


impl Drop for Input
{
    fn drop(&mut self)
    {
        // reset the stdin to original termios data
        tcsetattr(0, TCSANOW, & self.orig_termios)
            .unwrap();
    }
}


// fn main()
// {
//     let (insnk, insrc) = channel();
//     let events         = Input::new(insnk);
//     let _handle        = events.start();

//     loop
//     {
//         let chr = insrc.recv()
//             .expect("Unable to receive input events");

//         println!("Input event char: {}", chr);
//     }
// }
