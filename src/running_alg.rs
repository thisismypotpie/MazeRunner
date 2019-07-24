use std::fs::File;
//use std::io::BufReader;
use std::io::Read;
//use std::path::Path;
use std::env;

  pub struct Player {
  pub x: u64,
  pub y: u64,
  pub strategy: String,
  }
  pub struct Maze_locations{
  pub start_x: u64,
  pub start_y: u64,
  pub finish_x: u64,
  pub finish_y: u64,
  pub maze: Vec<(Vec<(char)>)>,
  }

pub fn generate_maze()
{
	println!("We are in the maze generator.");
}


pub fn load_maze(file_name: String)-> Vec<(Vec<(char)>)>{
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
  maze
}

pub fn begin_game(strategy: String, maze: Vec<(Vec<(char)>)>)
{ 
  //let mut player1 = Player{x:0,y:0};
  let mut points = find_maze_points(maze); 
  print!("{}[2J", 27 as char);//Ref. 1
  //display_maze(maze);
}


pub fn display_maze(maze: Vec<(Vec<(char)>)>)
{
  for i in 0..maze.len()
  {
    for j in 0..maze[i].len()
    {
      print!("{}",maze[i][j].to_string());
    }
    println!();
  }
}

fn find_maze_points(maze: Vec<(Vec<(char)>)>)->[u64;4]
{
let mut coordinates: [u64;4]=[0,0,0,0];
coordinates
}
