extern crate termion;
extern crate rand;
use std::process::exit;
use std::fs::File;
use std::io::Read;
use std::env;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdout,stdin,Write};
use termion::clear;
use rand::Rng;

  pub struct Player {
  pub x: u64,
  pub y: u64,
  pub strategy: String,
  pub underfoot: char,
  }
  pub struct Maze{
  pub start_x: u64,
  pub start_y: u64,
  pub finish_x: u64,
  pub finish_y: u64,
  pub map: Vec<(Vec<(char)>)>,
  } 
pub fn generate_maze(info: Vec<String>, strat: String)
{
  let mut maze = Vec::new();
  let mut iter = 0;
  let width:u64 =info[0].parse::<u64>().unwrap();
  let height:u64 = info[1].parse::<u64>().unwrap(); 
  for s in 0..height
  {
    maze.push(Vec::new());
    for i in 0..width
    {
	if s == height-1 || i == width-1 || s ==0 || i == 0
	{
          maze[iter].push('x');
	}
        else
        {
          maze[iter].push('_'); 
        }
    }
    iter+= 1;
  }            
  recursive_maze_creation(&mut std::vec::Vec::from(&mut maze[1..height as usize-2][1..width as usize-2] )); 
  let ran_x = rand::thread_rng().gen_range(0,maze.len()-2);
  let ran_y = rand::thread_rng().gen_range(0,maze[0].len()-2);
  maze[ran_x][ran_y]='s';
  begin_game(strat,maze);
  /*
  let start_x = rand::thread_rng().gen_range(0,height);
  let start_y = rand::thread_rng().gen_range(0,width);
  maze[start_x as usize][start_y as usize] = 's';    
  let mut trailblazer = Player{x:start_x,y:start_y,strategy:strat.clone(),underfoot:'s'};
  let mut direction_chosen = rand::thread_rng().gen_range(1,5);
  let names = ["left","up","right","down"];
  let mut line_length:u64 = rand::thread_rng().gen_range(1,(maze.len()/10))as u64;
  let moves:u64 = rand::thread_rng().gen_range((maze.len()/2),2*(maze.len()/3)) as u64;
  println!("Moves: {}",moves);
  for i in 0..moves
  {
    println!("moves_left: {}",moves -i);
    println!("Trailblazer coordinates: {},{}",trailblazer.x,trailblazer.y); 
    println!("Direction chosen: {}",names[direction_chosen as usize-1]); 
    println!("Length: {}",line_length);
    direction_chosen = rand::thread_rng().gen_range(1,5);
    line_length = rand::thread_rng().gen_range(1,(maze.len()/10)) as u64;
  }
  maze[trailblazer.x as usize][trailblazer.y as usize] = 'f';
  begin_game(strat,maze);*/
}

//The idea for this algorithm was found in reference 7.
fn recursive_maze_creation(mut maze: &mut std::vec::Vec<(Vec<(char)>)>)
{
  if maze.len()as u64 == 1 && maze[0].len() as u64 == 1
  {
    return;
  }
  let vert_wall = rand::thread_rng().gen_range(0,maze.len()-2);
  let hor_wall = rand::thread_rng().gen_range(0,maze[0].len()-2);
  let height = maze.len().clone();
  let width = maze[0].len().clone();
  for i in 0..maze.len()
  {
    maze[i][vert_wall] = 'x'; 
  }
  for i in 0..maze[0].len()
  {
    maze[hor_wall][i] = 'x';
  }   
  println!("Section one bounds: (0,{}),({},{})",vert_wall-1,vert_wall-1,hor_wall-1);
  //println!("Section two bounds: ({},{}),({},{})",vert_wall+1,hor_wall-1,hor_wall-1,maze[0].len());
  //println!("Section three bounds: ({},{}),({},{})",hor_wall+1,vert_wall-1,vert_wall-1,maze.len());
  //println!("Section four bounds: ({},{}),({},{})",hor_wall+1,maze[0].len(),vert_wall+1,maze.len());
  recursive_maze_creation(&mut std::vec::Vec::from(&mut maze[0..hor_wall][0..vert_wall] ));
  //recursive_maze_creation(&mut std::vec::Vec::from(&mut maze[0..hor_wall][vert_wall..width] ));
  //recursive_maze_creation(&mut std::vec::Vec::from(&mut maze[hor_wall..height][0..vert_wall] ));
  //recursive_maze_creation(&mut std::vec::Vec::from(&mut maze[hor_wall..height][vert_wall..width] ));
}

pub fn load_maze(file_name: String)-> Vec<(Vec<(char)>)>{
  let p = env::current_dir().unwrap();//ref 3
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
  let mut player1 = Player{x:0,y:0,strategy:strat,underfoot:'s'};
  let mut maze = Maze{start_x:0,start_y:0,finish_x:0,finish_y:0,map:maize.clone()};
  let mut points = find_maze_points(&maze.map); 
  player1.x = points[0];
  player1.y = points[1];
  maze.start_x = points[0];
  maze.start_y = points[1];
  maze.finish_x = points[2];
  maze.finish_y = points[3]; 
  println!("{}",clear::All);//ref 6
  maze.map[player1.x as usize][player1.y as usize]='U';
  display_all_maze(&maze, &player1);
  //display_maze(&maze,&player1);
  game_loop(player1,maze);
}


fn game_loop(mut player1: Player, mut maze: Maze)
{
   let mut direction= ' ';
   while player1.underfoot != 'f'
   {
     direction = get_input_direction();
     if direction == 'e'
     {
       exit(0);
     }   
     if direction == 'u'&& player1.x > 0
     {
       if maze.map[player1.x as usize-1][player1.y as usize]=='x'
	{
	  println!("You smack face first into a wall.");
	  continue;
	}	
	else
	{
          maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
          player1.x= player1.x -1;
          player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
          maze.map[player1.x as usize][player1.y as usize] = 'U'; 
          display_all_maze(&maze,&player1);
          //display_maze(&maze,&player1);
	}
     } 
     else if direction == 'l'&& player1.y > 0

     {
       if maze.map[player1.x as usize][player1.y as usize-1]=='x'
	{
	  println!("You smack face first into a wall.");
	  continue;

	}	
	else
	{
          maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
          player1.y= player1.y -1;
          player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
          maze.map[player1.x as usize][player1.y as usize] = 'U'; 
          //display_maze(&maze,&player1);
          display_all_maze(&maze,&player1);
	}
     } 
     else if direction == 'd'&& player1.x +1 < maze.map.len() as u64 -1

     {
       if maze.map[player1.x as usize+1][player1.y as usize]=='x'
	{
	  println!("You smack face first into a wall.");
	  continue;

	}	
	else
	{
          maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
          player1.x= player1.x +1;
          player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
          maze.map[player1.x as usize][player1.y as usize] = 'U'; 
          //display_maze(&maze,&player1);
          display_all_maze(&maze,&player1);
	}
     } 
     else if direction == 'r'&& player1.y + 1 < maze.map[player1.x as usize].len()as u64

     {
       if maze.map[player1.x as usize][player1.y as usize+1]=='x'
	{
	  println!("You smack face first into a wall.");
	  continue;

	}	
	else
	{
          maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
          player1.y= player1.y +1;
          player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
          maze.map[player1.x as usize][player1.y as usize] = 'U'; 
          //display_maze(&maze,&player1);
          display_all_maze(&maze,&player1);
	}
     } 
     else
     {
       println!("You cannot go that way!");
     }
   }
}


//beign reference 6
fn get_input_direction()->char
{

  let stdin = stdin();
   //println!("Press any key to continue...");
   let mut stdout = stdout().into_raw_mode().unwrap();
   stdout.flush().unwrap();
   for c in stdin.keys()
   {
     match c.unwrap()
     { 
       Key::Up => return 'u',
       Key::Right => return 'r',
       Key::Left => return 'l',
       Key::Down => return 'd',
       Key::Char('w') => return 'u',
       Key::Char('a') => return 'l',
       Key::Char('s') => return 'd',
       Key::Char('d') => return 'r',
       Key::Ctrl(c) => return'e',
       _         => return 'x',
     }
  }
  stdout.flush().unwrap();
  write!(stdout, "{}",termion::cursor::Show).unwrap();
  'x'
}
//end reference 6

fn find_range_values_to_display(maze: &Maze, player1: &Player, start: &mut i32, end: &mut i32)
{
  if *start < 0
  {
    *end -= *start;      
    *start = 0;
  }
  else if *end > maze.map.len() as i32 -1
  {
    *start -= *end  - maze.map.len()as i32 -1; 
    *end = maze.map.len() as i32 -1;
  }
}
pub fn display_all_maze(maze: &Maze, player1: &Player)
{
    println!("Location:{},{} ",player1.x,player1.y);
    for i in 0..maze.map.len()
    {
      for j in 0..maze.map[i].len()
      {
          print!("{}",maze.map[i as usize][j as usize].to_string());
      }
	println!();
    }

}
pub fn display_maze(maze: &Maze, player1: &Player)
{
  println!("{}",clear::All);//ref 6
  let vert_window:u64 = 10; 
  let hor_window:u64 = 20;
  let mut start_i:i32 = player1.x as i32 - vert_window as i32;
  let mut end_i:i32 = player1.x as i32 + vert_window as i32;
  let mut start_j:i32 = player1.y as i32 - hor_window as i32;
  let mut end_j:i32 = player1.y as i32+ hor_window as i32;
  if start_i < 0
  {
    start_i = 0;
  }
  if end_i >= maze.map.len() as i32
  {
    end_i = maze.map.len() as i32-1;
  }
  if start_j < 0
  {
    start_j = 0;
  }
  if end_j >= maze.map.len() as i32
  {
    end_j = maze.map.len() as i32-1;
  }
    println!("Location:{},{} ",player1.x,player1.y);
    for i in start_i..end_i
    {
      for j in start_j..end_j
      {
          print!("{}",maze.map[i as usize][j as usize].to_string());
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
