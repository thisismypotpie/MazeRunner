extern crate termion;
use crate::running_alg::begin_game;
use crate::running_alg::generate_maze;
use crate::running_alg::load_maze;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;
use termion::clear;
use termion::color;

fn create_random_maze(message: String) {
    println!("{}", clear::All); //ref 6
    println!("{}", message);
    let title = ["Width", "Height"];
    let mut maze_info = Vec::new();
    for t in title.iter() {
        println!(
            "Please select the {} of your maze, minimum is 20, maximum is 1,000. Type 'back' to go back.",
            t
        );
        //Ref. 2 begin
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        //Ref. 2 end
        if s.to_lowercase() == "back" {
            println!("{}", clear::All); //ref 6
            return main_menu();
        }
        let _test = s.parse::<i32>();
        if _test.is_err() {
            return create_random_maze("Please enter only a number.".to_string());
        }
        let _num: i32 = s.parse().unwrap();
        if _num < 20 || _num > 1000 {
            return create_random_maze(
                "Please enter only numbers between 20 and 1000.".to_string(),
            );
        }
        maze_info.push(s);
    }
    println!("What would you like to name the maze? Type 'back' to go back.");
    //Ref. 2 begin
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    if s.to_lowercase() == "back" {
        println!("{}", clear::All); //ref 6
        return main_menu();
    } else if s.to_lowercase() == "" {
        return create_random_maze(
            "Your maze needs to have a name greater than zero characters.".to_string(),
        );
    }
    if maze_info.len() < 2 {
        println!("Error: Width or height had no input, returning to main menu.");
        //start ref 8
        let mut stdout = stdout();
        stdout
            .write(b"Press Enter to retrn to main menu...")
            .unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
        //end ref 8
        return main_menu();
    }
    generate_maze(maze_info, s);
}

pub fn load_in_maze() -> String {
    println!("{}", clear::All); //ref 6
    println!("What is the name of the file you are loading? Make sure that the maze you are loading is in the maze directory above src.  Type 'back to go back to main menu.'");
    //Ref. 2 begin
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    s.to_string()
}
pub fn main_menu() {
    println!("Welcome to the Maze Running Simulator. \n Please Choose from the following options: \n 1. Run Randomly Generated Maze \n 2. Load in Maze to Run \n 3. Help \n 4. Exit");

    //Ref. 2 begin
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    //Ref. 2 end
    if s == "1" {
        create_random_maze("".to_string());
    } else if s == "2" {
        let choice = load_in_maze();
        if choice.to_lowercase() != "back" {
            begin_game(load_maze(choice));
            exit(0);
        } else {
            main_menu();
        }
    } else if s == "3" {
        println!("{}", clear::All); //ref 6
        println!("Welcome to the maze helper.  Here is a rundown of available features to get you started.\nIn the main menu there are two main ways of play.  You can either: \n 1. Create a randomly generated maze to solve. \n 2. Load in a maze to solve.\nIf you decide to generate a maze you will be prompted to input a width and length of a maze as well as a name.  Whatever maze is generated will ba saved to your maze folder and can be loaded from the main menu.\n\nWhen you play the game, there are a few characters to be aware of: \n");
        println!(
            "{}U{} = The player",
            color::Fg(color::Cyan),
            color::Fg(color::Reset)
        );
        println!("X = Wall");
        println!(
            "{}_{} = Walkable Space",
            color::Fg(color::Green),
            color::Fg(color::Reset)
        );
        println!(
            "{}s{} = Start Point",
            color::Fg(color::LightYellow),
            color::Fg(color::Reset)
        );
        println!(
            "{}f{} = Finish Point",
            color::Fg(color::LightYellow),
            color::Fg(color::Reset)
        );
        println!(
            "{}v{} = Extra Wall Smash",
            color::Fg(color::Magenta),
            color::Fg(color::Reset)
        );
        println!("When you are playing the coordinates for where you, the player, are, where the finish marker is, and how many wall smashes you have.");
        println!("Good luck and have fun!");
        //start ref 8
        let mut stdout = stdout();
        stdout
            .write(b"Press Enter to retrn to main menu...")
            .unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
        //end ref 8
        println!("{}", clear::All); //ref 6
        main_menu();
    } else if s == "4" {
        println!("{}", clear::All); //ref 6
        println!("Bye-bye, come back soon!");
        exit(0);
    } else {
        println!("Please enter a number betwee one and four.");
        main_menu()
    }
}
