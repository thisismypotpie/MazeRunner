use std::fs::File;
//use std::io::BufReader;
use std::io::Read;
//use std::path::Path;
use std::env;

pub fn generate_maze()
{
	println!("We are in the maze generator.");
}


pub fn load_maze(file_name: String){
  let p = env::current_dir().unwrap();//ref 4
  let mut path = p.display().to_string();
  println!("Current directory: {}",path);
  let test = &path[path.len()-3..path.len()];
  println!("About to test: {}",test);
  if test != "src"
  {
    println!("Adding src to path");
    path = path + "/src";
    println!("New path {}",path);
  } 
  path = path + "/mazes/" + &file_name;
  println!("Final path: {}",path);
  //start ref 3
  let mut data = String::new();
  let mut file = File::open(path).expect("Could not find directory 'mazes'");
  file.read_to_string(&mut data).expect("Unable to read a line.");
  //end ref 3
  let mut maze = Vec::new();
  let mut iter = 0;
  let lines = data.split('\n');
  for s in lines
  {
    maze.push(Vec::new());
    for i in s.chars()
    {
      maze[iter].push(i); 
    }
    iter+= 1;
  }
}

/*
pub fn display_maze(maze: Vec)
{
  for i in 0..maze.len()
  {
    for j in 0..maze[i].len()
    {
      print!("{}",maze[i][j].to_string());
    }
    println!();
  }
}*/
