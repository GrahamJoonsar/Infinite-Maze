use rand::Rng;

// Both of these should be odd in order to center player
pub const FRAME_WIDTH:  usize = 65;
pub const FRAME_HEIGHT: usize = 21;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Maze {
    frame: [char; FRAME_WIDTH*FRAME_HEIGHT],
}


impl Maze {
    // Creating a new maze
    pub fn new(frame: [char; FRAME_WIDTH*FRAME_HEIGHT]) -> Maze {
        Maze { frame }
    }

    pub fn move_player(&mut self, dir: Direction){
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

    pub fn draw (&self) {
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

// Generating more of the maze, hopefully with a wave function collapse algorithm
struct MazeGenerator;
impl MazeGenerator {
    fn gen_more(prev: &mut [char]) {
        // Superpositions of our new cells
        let mut cells = Vec::new();
        for _ in 0..prev.len() {cells.push(Superposition::new());}

        let mut changed = false;

        // Actually generating more 
        loop {
            if !changed { // Didnt change, must pick a random cell to collapse
                for i in 0..cells.len() {
                    if cells[i].get_collapsed() == '?' {
                        cells[i].force_collapse();
                        break;
                    }
                }
            } else {
                changed = false;
            }
            for i in 0..(cells.len()) {
                if cells[i].get_collapsed() == '?' {
                    if i > 0 {
                        let above = MazeGenerator::attempt_collapse(
                            &Superposition::from_char(&prev[i]),
                            &cells[i-1], &Superposition::from_char(&prev[i-1]));
                        if above.get_collapsed() != '?' {
                            cells[i] = above;
                            changed = true;
                        }
                    }
                    if i < cells.len()-1 {
                        let below = MazeGenerator::attempt_collapse(
                            &Superposition::from_char(&prev[i]),
                            &cells[i+1], &Superposition::from_char(&prev[i+1]));
                        if below.get_collapsed() != '?' {
                            cells[i] = below;
                            changed = true;
                        }
                    }
                }
            }

            if MazeGenerator::fully_collapsed(&cells) { break; }
        }

        for i in 0..cells.len() {
            prev[i] = cells[i].get_collapsed();
        }
    }

    fn attempt_collapse(h: &Superposition, v: &Superposition, d: &Superposition) -> Superposition {
        let (horiz, vert, diag) = ((*h).get_collapsed(), (*v).get_collapsed(), (*d).get_collapsed());
        if horiz == '?' || vert == '?' || diag == '?' {
            return Superposition::from_char(&'?');
        }

        if horiz == vert && vert == diag {
            return Superposition::from_char(&Superposition::inverse(diag));
        }

        if horiz == vert && diag == Superposition::inverse(horiz) {
            return Superposition::from_char(&horiz);
        }

        return Superposition::new();
    }

    fn fully_collapsed(cells: &Vec<Superposition>) -> bool {
        for c in cells {
            if c.get_collapsed() == '?' {
                return false;
            }
        }
        true
    }
}

//#[derive(Copy, Clone)]
struct Superposition{
    empty: bool,
    full: bool,
}
impl Superposition {
    fn new () -> Superposition {
        Superposition { empty: true, full: true }
    }

    fn from_char (c: &char) -> Superposition {
        match c {
            '?' => Superposition { empty: true, full: true   },
            ' ' => Superposition { empty: true, full: false  },
            '#' => Superposition { empty: false, full: true  },
            '0' => Superposition { empty: false, full: false },
            _   => Superposition { empty: true, full: true   },
        }
    }

    fn force_collapse(&mut self) {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.5 {
            self.empty = false;
        } else {
            self.full = false;
        }
    }

    fn inverse(c: char) -> char {
        match c {
            '#' => ' ',
            ' ' => '#',
            _   =>  c ,
        }
    }

    fn get_collapsed(&self) -> char {
        if self.empty && self.full {
            '?'
        } else if self.empty {
            ' '
        } else if self.full {
            '#'
        } else {
            '0'
        }
    }
}