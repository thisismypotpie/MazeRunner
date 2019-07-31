extern crate termion;
use termion::clear;
use std::io::{stdin,stdout,Write};
use std::process::exit;
use crate::running_alg::load_maze;
use crate::running_alg::begin_game;
use crate::running_alg::generate_maze;

fn create_random_maze(message: String){
  println!("{}", clear::All);//ref 6
  println!("{}",message);
  let title = ["Width","Height"];
  let mut maze_info = Vec::new();
  for t in title.iter()
  {
    println!("Please select the {} of your maze, minimum is 10, maximum is 1,000.",t);
    //Ref. 2 begin
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    if s.to_lowercase() == "back"
    {
      println!("{}", clear::All);//ref 6
      return ;
    }
    let _test = s.parse::<i32>();
     if _test.is_err(){
	return create_random_maze("Please enter only a number.".to_string());
    }
     let _num:i32 = s.parse().unwrap();
     if _num < 10 || _num > 1000{
	return create_random_maze("Please enter only numbers between 10 and 1000.".to_string());
    }
    maze_info.push(s);
  } 
  println!("What would you like to name the maze?");  
    //Ref. 2 begin
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    if s.to_lowercase() == "back"
    {
      println!("{}", clear::All);//ref 6
      return;
    }
    else if s.to_lowercase() == ""
    {
      return create_random_maze("Your maze needs to have a name greater than zero characters.".to_string());
    }
    let strat = maze_solving_strategy();
    generate_maze(maze_info,strat);
}

pub fn maze_solving_strategy()-> String{
  
  println!("{}", clear::All);//ref 6
  println!("Please select a strategy for solving the maze.\n 1. Right hand rule \n 2. Solve on my own.");
    //Ref. 2 begin
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    if s != "1" && s != "2"
    {
      return maze_solving_strategy();
    }
    return s;  
}
pub fn load_in_maze()-> String//If a 2 is returned from main menu, this function is called from main.
{ 
  println!("{}", clear::All);//ref 6
  println!("What is the name of the file you are loading? Make sure that the maze you are loading is in the maze directory above src.  Type 'back to go back to main menu.'");
    //Ref. 2 begin
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    return s.to_string();
}
pub fn main_menu(){
	println!("Welcome to the Maze Running Simulator. \n Please Choose from the following options: \n 1. Run Randomly Generated Maze \n 2. Load in Maze to Run \n 3. Help \n 4. Exit");


    //Ref. 2 begin
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    if s == "1" {//create a random maze
        create_random_maze("".to_string());
    }
    else if s == "2"{//load in a mze.
        let choice = load_in_maze();
        if choice.to_lowercase() != "back"
	{
	  begin_game(maze_solving_strategy(),load_maze(choice));
          exit(0);
	}	
    }
    else if s == "3"{//help options
	println!("Help coming soon...");
    }
    else if s == "4"{//exit
        println!("Bye-bye, come back soon!");
        exit(0);
    }
    else{ 
	println!("Please enter a number betwee one and four.");
    }
    main_menu()
}
