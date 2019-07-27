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
  impl Player{
    fn set_x(&self)-> &u64 {
      &self.x
    }
    fn set_y(&self)-> &u64 {
      &self.y
    }
    fn set_strategy(&self)-> &String {
      &self.strategy
    }
  }
  pub struct Maze{
  pub start_x: u64,
  pub start_y: u64,
  pub finish_x: u64,
  pub finish_y: u64,
  pub map: Vec<(Vec<(char)>)>,
  }
  impl Maze{
    fn set_start_x(&self)->&u64 {
      &self.start_x
    }
    fn set_start_y(&self)->&u64 {
      &self.start_y
    }
    fn set_finish_x(&self)->&u64 {
      &self.finish_x
    }
    fn set_finish_y(&self)->&u64 {
      &self.finish_y
    }
    fn set_map_coord(&self,x:usize,y:usize)->&char {
      &self.map[x][y] 
    }
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

pub fn begin_game(strat: String, maize: Vec<(Vec<(char)>)>)
{ 
  let mut player1 = Player{x:0,y:0,strategy:strat};
  let mut maze = Maze{start_x:0,start_y:0,finish_x:0,finish_y:0,map:maize.clone()};
  let mut points = find_maze_points(&maze.map); 
  player1.x = points[0];
  player1.y = points[1];
  maze.start_x = points[0];
  maze.start_y = points[1];
  maze.finish_x = points[2];
  maze.finish_y = points[3]; 
  print!("{}[2J", 27 as char);//Ref. 1
  println!("Ready sarge!{}{}{}{}",points[0],points[1],points[2],points[3]);
  maze.map[player1.x as usize][player1.y as usize]='U';
  display_maze(maze);
}


pub fn display_maze(maze: Maze)
{
  for i in 0..maze.map.len()
  {
    for j in 0..maze.map[i].len()
    {
      print!("{}",maze.map[i][j].to_string());
    }
    println!();
  }
}

fn find_maze_points(maze: &std::vec::Vec<(Vec<(char)>)>)->[u64;4]
{
let mut coordinates: [u64;4]=[(maze.len()+1)as u64,(maze.len()+1)as u64,(maze.len()+1)as u64,(maze.len()+1)as u64];
for i in 0..maze.len()
{
  for j in 0..maze[i].len()
  {
    if maze[i][j] == 's'
    {
      coordinates[0] = i as u64;
      coordinates[1] = j as u64;
    }
    else if maze[i][j] == 'f'
    {
      coordinates[2] = i as u64;
      coordinates[3] = j as u64;
    }
    if coordinates[0] != (maze.len()+1)as u64 &&coordinates[1] != (maze.len()+1)as u64 &&coordinates[2] != (maze.len()+1)as u64 &&coordinates[3] != (maze.len()+1)as u64
    {
      return coordinates;
    }
  }
}
coordinates
}
