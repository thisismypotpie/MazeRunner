extern crate termion;
use termion::clear;
use termion::color;
mod menu;
mod running_alg;

fn main() {
    println!("{}", clear::All); //ref 6
    println!("{}DISCLAIMER!!!!!!!!: Throughout the code there will be references to code that I used throughout the project that was not my own however they will not be listed in a traditional way.  Instead if you see a comment that has a 'Ref. #' this is a reference to a link in my references.txt page in my primary directory for tis project.  This will allow for cleaner and more readable code.  If you would like to see each reference link please go the to references.txt page.  Now back to the game. \n\n{}",color::Fg(color::Red),color::Fg(color::Reset));
    menu::main_menu();
}
