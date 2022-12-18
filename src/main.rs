use std::io::stdout;
use crossterm::{execute, cursor, terminal, event::{self, KeyEventState, KeyModifiers}};
use rand::seq::SliceRandom;
use std::collections::HashSet;

fn main() {
    // Set up command line
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    let mut frame = ['#'; FRAME_WIDTH*FRAME_HEIGHT];

    for i in 0..FRAME_HEIGHT {
        frame[i*FRAME_WIDTH + FRAME_WIDTH/2] = ' ';
    }
    for i in 0..FRAME_WIDTH {
        frame[(FRAME_HEIGHT/2)*FRAME_WIDTH + i] = ' ';
    }

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
const FRAME_WIDTH:  usize = 65;
const FRAME_HEIGHT: usize = 21;

struct Maze {
    frame: [char; FRAME_WIDTH*FRAME_HEIGHT],
}

// For creating more of the maze
struct MazeGenerator;
impl MazeGenerator {
    // Get horizontal adjacencies
    fn get_horizontal(seg: i32) -> Vec<i32> {
        match seg {
            1 => vec![3, 4, 6, 7, 8],
            2 => vec![4, 5, 6, 8],
            3 => vec![1, 3, 4, 7, 8],
            4 => vec![1, 2, 3, 7],
            5 => vec![2, 6, 7, 8],
            6 => vec![1, 2, 5, 6, 8],
            7 => vec![1, 3, 4, 5],
            8 => vec![1, 2, 3, 5, 6],
            _ => vec![],
        }
    }
    // Get vertical adjacencies
    fn get_vertical(seg: i32) -> Vec<i32> {
        match seg {
            1 | 2 | 3 | 4 => vec![1, 3, 5, 7],
            5 | 6 | 7 | 8 => vec![2, 4, 6, 8],
            _ => vec![],
        }
    }

    // Sorry for using clone, I am not good enoufh at rust to avoid it
    // Also this function will not be called every frame or something
    fn intersection(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let h: HashSet<i32> = HashSet::from_iter((*a).clone());
        let mut new_vec: Vec<i32> = Vec::new();
        for n in (*b).clone() {
            if h.contains(&n) {
                new_vec.push(n);
            }
        }
        new_vec
    }

    // This is the main algorithm, which I will attempt to explain.
    // Each possible segment of 3 length is represented by a number from 1-8
    // Each segment has segments that can be chanined horizontally and vertically chained to it
    // Using those rules, we can generate more maze that looks actually like a maze
    fn gen_more(prev: &mut [char]) {
        let mut horizontals: Vec<Vec<i32>> = Vec::new();
        for i in (0..(prev.len()-2)).step_by(2) {
            horizontals.push(MazeGenerator::get_horizontal(MazeGenerator::get_seg_num(prev[i], prev[i+1], prev[i+2])));
        }

        let mut final_segs: Vec<i32> = vec!(*horizontals[0].choose(&mut rand::thread_rng()).unwrap());
        for i in 1..(horizontals.len()) {
            final_segs.push(*MazeGenerator::intersection(
                &MazeGenerator::get_vertical(final_segs[i-1]), horizontals.get(i).unwrap())
                .choose(&mut rand::thread_rng()).unwrap());
        }
        (prev[0], prev[1], prev[2]) = MazeGenerator::get_seg(final_segs[0]);
        for i in (3..final_segs.len()).step_by(2) {
            (_, prev[i], prev[i+1]) = MazeGenerator::get_seg((i-2) as i32);
        }
    }

    fn get_seg_num(first: char, mid: char, end: char) -> i32 {
        match (first, mid, end) {
            ('#', '#', '#') => 1,
            (' ', '#', '#') => 2,
            ('#', ' ', '#') => 3,
            (' ', ' ', '#') => 4,
            ('#', '#', ' ') => 5,
            (' ', '#', ' ') => 6,
            ('#', ' ', ' ') => 7,
            (' ', ' ', ' ') => 8,
            _ => -1,
        }
    }

    fn get_seg (n: i32) -> (char, char, char) {
        match n {
            1 => ('#', '#', '#'),
            2 => (' ', '#', '#'),
            3 => ('#', ' ', '#'),
            4 => (' ', ' ', '#'),
            5 => ('#', '#', ' '),
            6 => (' ', '#', ' '),
            7 => ('#', ' ', ' '),
            8 => (' ', ' ', ' '),
            _ => (' ', ' ', ' ')
        }
    }
}

impl Maze {
    // Creating a new maze
    fn new(frame: [char; FRAME_WIDTH*FRAME_HEIGHT]) -> Maze {
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
                    MazeGenerator::gen_more(&mut old_maze);
                    for i in 0..FRAME_HEIGHT {
                        self.frame[(i*FRAME_WIDTH)] = old_maze[i];
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
                    MazeGenerator::gen_more(&mut old_maze);
                    for i in 0..FRAME_HEIGHT {
                        self.frame[(i*FRAME_WIDTH + FRAME_WIDTH-1)] = old_maze[i];
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
                    MazeGenerator::gen_more(&mut old_maze);
                    for i in 0..FRAME_WIDTH {
                        self.frame[i] = old_maze[i];
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
                    MazeGenerator::gen_more(&mut old_maze);
                    for i in 0..FRAME_WIDTH {
                        self.frame[(FRAME_HEIGHT-1)*FRAME_WIDTH + i] = old_maze[i];
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