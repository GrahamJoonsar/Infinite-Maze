use std::io::stdout;
use crossterm::{execute, cursor, terminal, event::{self, KeyEventState, KeyModifiers}};

use crate::maze::{Maze, FRAME_WIDTH, FRAME_HEIGHT, Direction};

mod maze;

fn main() {
    // Set up command line
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    let frame = [' '; FRAME_WIDTH*FRAME_HEIGHT];

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