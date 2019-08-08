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
use termion::color;
use rand::Rng;
//use std::fs::Write;
use std::fs::OpenOptions;
use std::io::prelude::*;

  pub struct Player {
  pub x: u64,
  pub y: u64,
  pub underfoot: char,
  }
  pub struct Maze{
  pub start_x: u64,
  pub start_y: u64,
  pub finish_x: u64,
  pub finish_y: u64,
  pub map: Vec<(Vec<(char)>)>,
  } 

fn save_maze_to_file(maze: &mut std::vec::Vec<(Vec<(char)>)>,name: String)
{
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
  path = path + "/mazes/";
  println!("Final path: {}",path);
   //ref 3 start
   let mut data:String = "".to_string();
   //let mut f = File::create(path+&name).expect("Could not find mazes directory.");
   let mut file = OpenOptions::new()
	.write(true)
	.append(true)
	.open(path+&name)
	.unwrap();
   for i in 0..maze.len()
   {
     data = maze[i].clone().into_iter().collect();     
     writeln!(file,"{}",data);
   }
   //ref 3 end
}
pub fn generate_maze(info: Vec<String>, name: String)
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
  let endx:usize = maze.len() as usize -2;
  let endy:usize = maze[0].len() as usize -2;
  recursive_maze_creation(&mut maze,1 as usize,endx,1 as usize, endy);

  //recursive_maze_creation(&mut std::vec::Vec::from(&mut maze[1..(height as usize-1)][1..(width as usize-1)] )); 
  let mut ran_x = rand::thread_rng().gen_range(1,maze.len()-2);
  let mut ran_y = rand::thread_rng().gen_range(1,maze[0].len()-2);
  maze[ran_x][ran_y]='s';
  ran_x = rand::thread_rng().gen_range(1,maze.len()-2);
  ran_y = rand::thread_rng().gen_range(1,maze[0].len()-2);
  maze[ran_x][ran_y]='f'; 
  save_maze_to_file(&mut maze,name);
  begin_game(maze);
}

//The idea for this algorithm was found in reference 7.
fn recursive_maze_creation(mut maze: &mut std::vec::Vec<(Vec<(char)>)>,xstart: usize, xend: usize, ystart: usize, yend: usize)
{
    /*  let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap(); */
  if xend - xstart <= 1 || yend - ystart <= 1
  {
    println!("Returning!");
    return;
  }
    println!("maze index range: X: {}-{} Y:{}-{}",xstart,xend,ystart,yend);
  let vert_wall = rand::thread_rng().gen_range(ystart+1,yend);
  let hor_wall = rand::thread_rng().gen_range(xstart+1,xend);
   println!("vert_wall: {}. Limit is: {}",vert_wall,yend);
   println!("hor_wall: {}. Limit is: {}",hor_wall,xend);
  for i in xstart..xend+1
  {
    println!("i is {}",i);
    //println!("x-index: {}",xstart+i);
    maze[i][vert_wall] = 'x'; 
  }
  for i in ystart..yend+1
  {
    println!("i is {}",i);
    //println!("y-index: {}",ystart+i);
    maze[hor_wall][i] = 'x';
  } 
  
  let mut hole_punch = rand::thread_rng().gen_range(1,5);
  let mut sides_chosen = 0;
  let mut one_chosen = false;
  let mut two_chosen = false;
  let mut three_chosen = false;
  let mut four_chosen = false;
 
  while sides_chosen < 3
  {
    println!("hole punch chose: {}",hole_punch);
    println!("sides chosen: {}",sides_chosen);
    //hole in vert wall before hor wall intersection.
    if hole_punch == 1 && one_chosen == false
    {
      println!("Choosing Range between {} and {}.",xstart,hor_wall);
      let mut vert_hole = rand::thread_rng().gen_range(xstart,hor_wall);    
      maze[vert_hole][vert_wall]= '_';
      one_chosen = true;
      sides_chosen+=1;
    }
    //hole in hor wall before vert wall intersection.
    else if hole_punch == 2 && two_chosen == false
    {
      println!("Choosing Range between {} and {}.",ystart,vert_wall);
      let mut hor_hole = rand::thread_rng().gen_range(ystart,vert_wall);
      maze[hor_wall][hor_hole] = '_';
      two_chosen = true;
      sides_chosen+=1;
    }
    //hole in vert wall after hor wall intersection.
    else if hole_punch == 3 && three_chosen == false
    {
      println!("Choosing Range between {} and {}.",hor_wall+1,xend);
      let mut vert_hole = 0;
      if hor_wall+1 == xend
      {
        vert_hole = hor_wall+1;
      }
      else
      { 
        vert_hole = rand::thread_rng().gen_range(hor_wall+1,xend);
      }
      maze[vert_hole][vert_wall]= '_';
      three_chosen = true;
      sides_chosen+=1;
    }
    //hole in hor wall after vert wall intersection.
    else if hole_punch == 4 && four_chosen == false
    {
      println!("Choosing Range between {} and {}.",vert_wall+1,yend);
      let mut hor_hole = 0;
      if vert_wall+1 == yend
      {
	hor_hole = vert_wall+1;
      }
      else
      {
        hor_hole = rand::thread_rng().gen_range(vert_wall+1,yend);
      }
      maze[hor_wall][hor_hole] = '_';
      four_chosen = true;
      sides_chosen+=1;
    }
    hole_punch = rand::thread_rng().gen_range(1,5);
  } 
   
    for i in xstart..xend+1
    {
      for j in ystart..yend+1
      {
          print!("{}",maze[i as usize][j as usize].to_string());
      }
	println!();
    }
  println!("Vert Wall: {}",vert_wall);
  println!("Hor Wall: {}",hor_wall);
  println!("Section one dimensions:   X:{}-{} Y:{}-{}",xstart,hor_wall-1,ystart,vert_wall-1);
  println!("Section two dimensions:   X:{}-{} Y:{}-{}",xstart,hor_wall-1,vert_wall+1,yend);
  println!("Section three dimensions: X:{}-{} Y:{}-{}",hor_wall+1,xend,ystart,vert_wall-1);
  println!("Section four dimensions:  X:{}-{} Y:{}-{}",hor_wall+1,xend,vert_wall+1,yend);
  let mut startx:usize = xstart;
  let mut endx:usize = hor_wall -1;
  let mut starty:usize = ystart;
  let mut endy:usize = vert_wall -1;
  recursive_maze_creation(&mut maze,xstart,hor_wall -1,ystart,vert_wall-1);
  recursive_maze_creation(&mut maze,xstart,hor_wall -1,vert_wall +1,yend);
  recursive_maze_creation(&mut maze,hor_wall +1,xend,ystart,vert_wall -1);
  recursive_maze_creation(&mut maze,hor_wall +1,xend,vert_wall+1, yend);
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

pub fn begin_game(maize: Vec<(Vec<(char)>)>)
{ 
  let mut player1 = Player{x:0,y:0,underfoot:'s'};
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
  //display_all_maze(&maze, &player1);
  display_maze(&maze,&player1);
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
	    print!("{}",color::Fg(color::Reset));
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
          //display_all_maze(&maze,&player1);
          display_maze(&maze,&player1);
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
          display_maze(&maze,&player1);
          //display_all_maze(&maze,&player1);
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
          display_maze(&maze,&player1);
          //display_all_maze(&maze,&player1);
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
          display_maze(&maze,&player1);
          //display_all_maze(&maze,&player1);
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
  if end_j >= maze.map[0].len() as i32
  {
    end_j = maze.map[0].len() as i32-1;
  }
    println!("Location:{},{} ",player1.x,player1.y);
    println!("Exit: {},{}",maze.finish_x,maze.finish_y);
    for i in start_i..end_i
    {
      for j in start_j..end_j+1
      {
          /*let mut color = color::Fg(color::Reset);
          if maze.map[i as usize][j as usize]=='_'
          {
            color = color::Fg(color::Green);
          }
          print!("{}",maze.map[i as usize][j as usize].to_string());*/
	
          if maze.map[i as usize][j as usize]=='_'
          {
	    print!("{}{}",color::Fg(color::Green),maze.map[i as usize][j as usize].to_string());
          }
          else if maze.map[i as usize][j as usize]=='U'
          {
	    print!("{}{}",color::Fg(color::Cyan),maze.map[i as usize][j as usize].to_string());
          }
          else if maze.map[i as usize][j as usize]=='f'
          {
	    print!("{}{}",color::Fg(color::LightYellow),maze.map[i as usize][j as usize].to_string());
          }
          else if maze.map[i as usize][j as usize]=='s'
          {
	    print!("{}{}",color::Fg(color::LightYellow),maze.map[i as usize][j as usize].to_string());
          }
	  else
	  {
	    print!("{}{}",color::Fg(color::Reset),maze.map[i as usize][j as usize].to_string());
	  }
      }
	println!();
    }
	    print!("{}",color::Fg(color::Reset));
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
