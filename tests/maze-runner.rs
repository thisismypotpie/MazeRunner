use crate::*;

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
    let mut points = find_maze_points(&test_maze);
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
    let test_against = load_maze("hopefully no one picks this name".to_string());
    assert_eq!(test_maze, test_against);
}
