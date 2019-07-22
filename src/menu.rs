use std::io::{stdin,stdout,Write};
use std::process::exit;
mod running_alg;

fn create_random_maze()-> i32{
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
	return -1;
    }
    let _test = s.parse::<i32>();
     if _test.is_err(){
	return create_random_maze();
    }
     let _num:i32 = s.parse().unwrap();
     if _num < 10 || _num > 1000{
	return create_random_maze();
    }
    return _num;
}

pub fn maze_solving_strategy()-> String{
  print!("{}[2J", 27 as char);//Ref. 1
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
  print!("{}[2J", 27 as char);//Ref. 1
  println!("What is the name of the file you are loading? Make sure that the maze you are loading is in the maze directory above src.");
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
	println!("Size: {}",create_random_maze().to_string());	
        maze_solving_strategy();
    }
    else if s == "2"{//load in a mze.
        print!("{}[2J", 27 as char);//Ref. 1
        println!("File name: {}",load_in_maze().to_string());
        maze_solving_strategy();
    }
    else if s == "3"{//help options
  	print!("{}[2J", 27 as char);//Ref. 1
	println!("Help coming soon...");
    }
    else if s == "4"{//exit
  	print!("{}[2J", 27 as char);//Ref. 1
        println!("Bye-bye, come back soon!");
        exit(0);
    }
    else{ 
	println!("Please enter a number betwee one and four.");
    }
    main_menu()
}
