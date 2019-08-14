extern crate termion;
use termion::clear;
use termion::color;
mod menu;
mod running_alg;
pub use menu::*;
pub use running_alg::*;

fn main() {
    println!("{}", clear::All); //ref 6
    println!("{}DISCLAIMER!!!!!!!!: Throughout the code there will be references to code that I used throughout the project that was not my own however they will not be listed in a traditional way.  Instead if you see a comment that has a 'Ref. #' this is a reference to a link in my references.txt page in my primary directory for tis project.  This will allow for cleaner and more readable code.  If you would like to see each reference link please go the to references.txt page.  Now back to the game. \n\n{}",color::Fg(color::Red),color::Fg(color::Reset));
    menu::main_menu();
}

#[cfg(test)]
mod tests {
    use crate::running_alg::*;
    #[test]
    fn test_start_finish_points() {
        let mut test_maze = Vec::new();
        let mut iter = 0;
        for s in 0..11 {
            test_maze.push(Vec::new());
            for i in 0..11 {
                if s == 10 || i == 10 || s == 0 || i == 0 {
                    test_maze[iter].push('x');
                } else if s == 3 && i == 7 {
                    test_maze[iter].push('s');
                } else if s == 5 && i == 9 {
                    test_maze[iter].push('f');
                } else {
                    test_maze[iter].push('_');
                }
            }
            iter += 1;
        }
        let points = find_maze_points(&test_maze);
        assert_eq!(points[0], 3);
        assert_eq!(points[1], 7);
        assert_eq!(points[2], 5);
        assert_eq!(points[3], 9);
    }

    #[test]
    fn test_save_and_load() {
        let mut test_maze = Vec::new();
        let mut iter = 0;
        for s in 0..11 {
            test_maze.push(Vec::new());
            for i in 0..11 {
                if s == 10 || i == 10 || s == 0 || i == 0 {
                    test_maze[iter].push('x');
                } else if s == 3 && i == 7 {
                    test_maze[iter].push('s');
                } else if s == 5 && i == 9 {
                    test_maze[iter].push('f');
                } else {
                    test_maze[iter].push('_');
                }
            }
            iter += 1;
        }
        assert!(save_maze_to_file(
            &mut test_maze,
            "hopefully no one picks this name".to_string()
        ));
        let _test_against = load_maze("hopefully no one picks this name".to_string());
    }

    #[test]
    fn test_display_small_maze() {
        let mut test_maze = Vec::new();
        let mut iter = 0;
        let player1 = Player {
            x: 1,
            y: 1,
            wall_smashes: 5,
            underfoot: 's',
        };
        for s in 0..11 {
            test_maze.push(Vec::new());
            for i in 0..11 {
                if s == 10 || i == 10 || s == 0 || i == 0 {
                    test_maze[iter].push('x');
                } else if s == 3 && i == 7 {
                    test_maze[iter].push('s');
                } else if s == 5 && i == 9 {
                    test_maze[iter].push('f');
                } else {
                    test_maze[iter].push('_');
                }
            }
            iter += 1;
        }
        let maze = Maze {
            start_x: 3,
            start_y: 7,
            finish_x: 5,
            finish_y: 9,
            map: test_maze.clone(),
        };
        assert!(display_maze(&maze, &player1));
    }

#[test]
fn test_display_large_maze(){

        let mut test_maze = Vec::new();
        let mut iter = 0;
        let player1 = Player {
            x: 1,
            y: 1,
            wall_smashes: 5,
            underfoot: 's',
        };
        for s in 0..60 {
            test_maze.push(Vec::new());
            for i in 0..80 {
                if s == 59 || i == 79 || s == 0 || i == 0 {
                    test_maze[iter].push('x');
                } else if s == 30 && i == 70 {
                    test_maze[iter].push('s');
                } else if s == 51 && i == 91 {
                    test_maze[iter].push('f');
                } else {
                    test_maze[iter].push('_');
                }
            }
            iter += 1;
        }
        let maze = Maze {
            start_x: 30,
            start_y: 70,
            finish_x: 51,
            finish_y: 91,
            map: test_maze.clone(),
        };
        assert!(display_maze(&maze, &player1));
}
}
