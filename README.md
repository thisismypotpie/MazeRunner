# maze_runner
Hello!  Welcome to the Maze Runner Version 1.0!

Before beginning I would like to point out that all references in code are in a references.txt file.  Here each reference is assigned a number and a comment is added in the code using the reference number.  If you see such a comment in the code, it matches with the references.txt file.  

When cargo is run you will be taken into the main menu and have the following options:

1. Create a randomly generated maze.  This options will allow you to create a randomly generated maze by inputting a width and height of the maze, you may select anywhere from 20 to 1,000. You also get to name your maze.  The name is important as any maze that is radnomly generated will be saved to the "mazes" folder in src, there you can reload and replay any mazes yyou find particularly interesting.

2. Load in a maze from the "mazes" folder in src/mazes. Any maze you want to load in needs to be in the file src/mazes and must be a regular file.  It is also important to note that cargo must be run in either src or in the mazeRunner folder, as the file finder for getting to "mazes" assumes the user runs "cargo run" in either the mazeRunner folder or the src folder. 

3. The help section will tell you how to play the game and what features you have when playing.  It is important to note that when creating a maze or loading a maze, if you type in "back" you will be taken back to the main menu.

4. Exit will close the program and bring the user out of the game.


When you play the game from loading a maze or creating a new one, there are two major components, your HUD and the map of the maze you may use.  The HUD has a few features that I will point out:
1. The HUD contains a line letting the user know if they press q they will be taken back to the main menu.
2. The HUD will let the player know where in the maze they are located.
3. The HUD will let the player know where the exit tile of the maze is.
4. The HUD will let the player know how many wall they can smash through.

Additionally there is also a map of where the player is.  The player may move using the arrow keys or WASD.
There are several characters to resemble different things in the maze. Here is a legend:
x = wall
U = player
_ = walkable space
s = starting point
f = finishing point
v = extra wall smash 



The user is also allows to go into mazes and create their own maze.  There are rules to follow that will throw an error when the user breaks these rules.
1. Each created maze must be rectangular.  The length and height do not need to be the same but each row should be the same length.  If you do not do this, you will encounter a message that will bring you back to the main menu.
2. Each created maze must have a starting point (s) and a ending point (f).  If you fail to include these in your maze and attempt to load it in, you will get a message that will send you back to main menu.


Those are all of the basics needed to get started with this game.  Have fun and dont get lost in the mazes!


ONE FINAL NOTE:
As you run clippy you will notice that there are errors and warnings that appear.  In the proces of cleaning up this project, I gave each and every error 45 minutes to solve.  In many ccases the warning or error could be solved however there were some errors and warning in which their advice ending up making larger errors which I could not solve.  If I could not solve the warning in 45 mimnutes, I moved on to the next one.  Here are the following statisitcs from clippy when I started vs when I finished:
errors: 19/25 errors solved.
warnings: 34/37 errors solved.

