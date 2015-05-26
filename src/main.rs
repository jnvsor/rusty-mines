extern crate structopt;
extern crate termion;

#[macro_use]
extern crate structopt_derive;

mod minefield;
mod game;

use game::Game;
use structopt::StructOpt;
use termion::terminal_size;

#[derive(StructOpt,Debug)]
#[structopt(name = "Rusty mines", about = "Simple minesweeper in rust")]
struct Argv {
    #[structopt(short = "w", long = "width", help = "Grid width")]
    width: Option<u16>,

    #[structopt(short = "h", long = "height", help = "Grid height")]
    height: Option<u16>,

    #[structopt(short = "m", long = "mines", help = "Number of mines")]
    mines: Option<u32>,
}

fn main() {
    let Argv { width, height, mines } = Argv::from_args();

    let (termwidth, termheight) = terminal_size().expect("Not running in a terminal!");

    let width = width.unwrap_or(termwidth);
    let height = height.unwrap_or(termheight - 2);
    let mines = mines.unwrap_or(width as u32 * height as u32 / 10);

    Game::create(width, height, mines).unwrap().run();
}
