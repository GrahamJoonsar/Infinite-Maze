use std::io::stdout;
use crossterm::{execute, cursor, terminal, event::{self, KeyEventState, KeyModifiers}};

fn main() {
    // Set up command line
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    let mut frame = [' '; FRAME_WIDTH*FRAME_HEIGHT];
    frame[(FRAME_HEIGHT/2) * FRAME_WIDTH + FRAME_WIDTH/2 + 1] = 'r';
    //frame[(FRAME_HEIGHT/2) * FRAME_WIDTH + FRAME_WIDTH/2 - 1] = 'l';
    frame[(FRAME_HEIGHT/2 - 1) * FRAME_WIDTH + FRAME_WIDTH/2] = 'u';
    frame[(FRAME_HEIGHT/2 + 1) * FRAME_WIDTH + FRAME_WIDTH/2] = 'd';
    let mut maze = Maze::new(frame);

    // So you dont have to look at this mess in the match statement
    let k_right: event::Event = event::Event::Key(event::KeyEvent { code: event::KeyCode::Right, modifiers: KeyModifiers::NONE, kind: event::KeyEventKind::Press, state: KeyEventState::NONE});
    let k_left:  event::Event = event::Event::Key(event::KeyEvent { code: event::KeyCode::Left, modifiers: KeyModifiers::NONE, kind: event::KeyEventKind::Press, state: KeyEventState::NONE});
    let k_up:    event::Event = event::Event::Key(event::KeyEvent { code: event::KeyCode::Up, modifiers: KeyModifiers::NONE, kind: event::KeyEventKind::Press, state: KeyEventState::NONE});
    let k_down:  event::Event = event::Event::Key(event::KeyEvent { code: event::KeyCode::Down, modifiers: KeyModifiers::NONE, kind: event::KeyEventKind::Press, state: KeyEventState::NONE});

    loop {
        // Move cursor to top left
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        let e = event::read().unwrap();
        if      e == k_right { maze.move_player(Direction::Right); } 
        else if e == k_left  { maze.move_player(Direction::Left);  }
        else if e == k_up    { maze.move_player(Direction::Up);    }
        else if e == k_down  { maze.move_player(Direction::Down);  }
        else                 { continue; } 

        maze.draw();
    }

}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Both of these should be odd in order to center player
const FRAME_WIDTH:  usize = 25;
const FRAME_HEIGHT: usize = 11;

struct Maze {
    frame: [char; FRAME_WIDTH*FRAME_HEIGHT],
}

// For creating more of the maze
struct MazeGenerator;
impl MazeGenerator {
    fn gen_left(prev_strip: [char; FRAME_HEIGHT]) -> [char; FRAME_HEIGHT] {
        [' '; FRAME_HEIGHT]
    }

    fn gen_right(prev_strip: [char; FRAME_HEIGHT]) -> [char; FRAME_HEIGHT] {
        [' '; FRAME_HEIGHT]
    }
    
    fn gen_up(prev_strip: [char; FRAME_WIDTH]) -> [char; FRAME_WIDTH] {
        [' '; FRAME_WIDTH]
    }

    fn gen_down(prev_strip: [char; FRAME_WIDTH]) -> [char; FRAME_WIDTH] {
        [' '; FRAME_WIDTH]
    }
}

impl Maze {
    // Creating a new maze
    fn new(frame: [char; FRAME_WIDTH*FRAME_HEIGHT]) -> Maze{
        Maze { frame }
    }

    fn move_player(&mut self, dir: Direction){
        // Determine if valid move
        let move_to: char;
        match dir {
            Direction::Left => {
                move_to = self.frame[(FRAME_HEIGHT/2) * FRAME_WIDTH + FRAME_WIDTH/2 - 1];
            },
            Direction::Right => {
                move_to = self.frame[(FRAME_HEIGHT/2) * FRAME_WIDTH + FRAME_WIDTH/2 + 1];
            },
            Direction::Up => {
                move_to = self.frame[(FRAME_HEIGHT/2 - 1) * FRAME_WIDTH + FRAME_WIDTH/2];
            },
            Direction::Down => {
                move_to = self.frame[(FRAME_HEIGHT/2 + 1) * FRAME_WIDTH + FRAME_WIDTH/2];
            },
        }
        // Actually moving the frame
        if move_to == ' ' {
            match dir {
                Direction::Left => {
                    // move the frame
                    for i in 0..FRAME_HEIGHT {
                        for j in (1..FRAME_WIDTH).rev() {
                            self.frame[(i*FRAME_WIDTH) + j] = self.frame[(i*FRAME_WIDTH) + j - 1];
                        }
                    }
                    // Generate more of maze
                    let mut old_maze: [char; FRAME_HEIGHT] = [' '; FRAME_HEIGHT];
                    for i in 0..FRAME_HEIGHT {
                        old_maze[i] = self.frame[(i*FRAME_WIDTH)];
                    }
                    let more_maze = MazeGenerator::gen_left(old_maze);
                    for i in 0..FRAME_HEIGHT {
                        self.frame[(i*FRAME_WIDTH)] = more_maze[i];
                    }
                }
                Direction::Right => {
                    for i in 0..FRAME_HEIGHT {
                        for j in 0..(FRAME_WIDTH-1) {
                            self.frame[(i*FRAME_WIDTH) + j] = self.frame[(i*FRAME_WIDTH) + j + 1];
                        }
                    }
                    // Generate more of maze
                    let mut old_maze: [char; FRAME_HEIGHT] = [' '; FRAME_HEIGHT];
                    for i in 0..FRAME_HEIGHT {
                        old_maze[i] = self.frame[(i*FRAME_WIDTH + FRAME_WIDTH-1)];
                    }
                    let more_maze = MazeGenerator::gen_right(old_maze);
                    for i in 0..FRAME_HEIGHT {
                        self.frame[(i*FRAME_WIDTH + FRAME_WIDTH-1)] = more_maze[i];
                    }
                }
                Direction::Up => {
                    for i in (1..FRAME_HEIGHT).rev() {
                        for j in 0..FRAME_WIDTH {
                            self.frame[(i*FRAME_WIDTH) + j] = self.frame[((i-1)*FRAME_WIDTH + j)];
                        }
                    }
                    // Generate more of maze
                    let mut old_maze: [char; FRAME_WIDTH] = [' '; FRAME_WIDTH];
                    for i in 0..FRAME_WIDTH {
                        old_maze[i] = self.frame[i];
                    }
                    let more_maze = MazeGenerator::gen_up(old_maze);
                    for i in 0..FRAME_WIDTH {
                        self.frame[i] = more_maze[i];
                    }
                }
                Direction::Down => {
                    for i in 0..(FRAME_HEIGHT-1) {
                        for j in 0..FRAME_WIDTH {
                            self.frame[(i*FRAME_WIDTH) + j] = self.frame[((i+1)*FRAME_WIDTH + j)];
                        }
                    }
                    // Generate more of maze
                    let mut old_maze: [char; FRAME_WIDTH] = [' '; FRAME_WIDTH];
                    for i in 0..FRAME_WIDTH {
                        old_maze[i] = self.frame[(FRAME_HEIGHT-1)*FRAME_WIDTH + i];
                    }
                    let more_maze = MazeGenerator::gen_down(old_maze);
                    for i in 0..FRAME_WIDTH {
                        self.frame[(FRAME_HEIGHT-1)*FRAME_WIDTH + i] = more_maze[i];
                    }
                }
            }
        }
    }

    fn draw (&self) {
        println!("+{}+", "-".repeat(FRAME_WIDTH));
        for i in 0..FRAME_HEIGHT {
            print!("{}", '|');
            for j in 0..FRAME_WIDTH {
                if (i == FRAME_HEIGHT/2) && (j == FRAME_WIDTH/2) {
                    print!("{}", '@');
                } else {
                    print!("{}", self.frame[i*FRAME_WIDTH + j]);
                }
            }
            print!("{}", '|');
            println!();
        }
        println!("+{}+", "-".repeat(FRAME_WIDTH));
    }
}