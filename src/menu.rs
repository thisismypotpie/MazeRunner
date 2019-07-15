use std::io::{stdin,stdout,Write};
use std::process::exit;

fn create_random_maze(){
  print!("{}[2J", 27 as char);//Ref. 1
  println!("Please select the size of your maze. The minimum is 10x10 with a max size of 120x120.  All you need to do is type a single number and the maze will be created in a x by x maze based on the number you typed in.  If you would like to go back, please type 'back'.");
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
    if s.to_lowercase()  == "back"{
        print!("{}[2J", 27 as char);//Ref. 1
	return main_menu();
    }
    let _test = s.parse::<i32>();
     if _test.is_err(){
	return create_random_maze();
    }
     let _num:i32 = s.parse().unwrap();
     if _num < 10 || _num > 120{
	return create_random_maze();
    }
  maze_solving_strategy();
}

fn load_in_maze(){
  print!("{}[2J", 27 as char);//Ref. 1
  maze_solving_strategy();

}

fn maze_solving_strategy(){
  print!("{}[2J", 27 as char);//Ref. 1
  println!("We made it this far!");
  //1. Right hand rule.
  //2. I'll solve it myself.
}

pub fn main_menu() {
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
    if s == "1" {
	create_random_maze();	
    }
    else if s == "2"{
	load_in_maze();
    }
    else if s == "3"{
  	print!("{}[2J", 27 as char);//Ref. 1
	println!("Help coming soon...");
	return main_menu();
    }
    else if s == "4"{
  	print!("{}[2J", 27 as char);//Ref. 1
        println!("Bye-bye, come back soon!");
        exit(0);
    }
    else{ 
	println!("Please enter a number betwee one and four.");
	return main_menu();
    }
}
