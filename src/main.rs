
mod menu;
mod running_alg;

fn main() {

  //This line was found at: https://stackoverflow.com/questions/34837011/how-to-clear-terminal-screen-in-rust-after-new-line-is-printing/34837038
  print!("{}[2J", 27 as char);
  menu::main_menu(); 
}
