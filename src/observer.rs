use crate::solver::{Cell, Guess};

use std::io;
use std::io::Write;
use std::{thread, time};
use termion::{clear, color, cursor};

const GRID_SIZE: i32 = 5;

fn flush() {
    print!("{}", cursor::Hide);
    io::stdout().flush().unwrap();
}

fn draw_rectangle(left: i32, upper: i32, width: i32, height: i32, character: &str) {
    print!("{}", color::Fg(color::LightBlue));
    for x in left..(left + width + 1) {
        print!(
            "{}{}{}{}",
            cursor::Goto((x + 1) as u16, (upper + 1) as u16),
            character,
            cursor::Goto((x + 1) as u16, (upper + height + 1) as u16),
            character
        );
    }
    for y in upper..(upper + height + 1) {
        print!(
            "{}{}{}{}",
            cursor::Goto((left + 1) as u16, (y + 1) as u16),
            character,
            cursor::Goto((left + width + 1) as u16, (y + 1) as u16),
            character
        );
    }
    print!("{}", color::Fg(color::Black));
}

#[derive(Default)]
pub struct Highlight {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Drop for Highlight {
    fn drop(&mut self) {
        draw_rectangle(self.x, self.y, self.width, self.height, " ");
    }
}

pub trait GridObserver: Clone {
    fn highlight_block(&self, _x: i32, _y: i32) -> Highlight {
        Highlight::default()
    }
    fn highlight_row(&self, _y: i32) -> Highlight {
        Highlight::default()
    }
    fn highlight_column(&self, _x: i32) -> Highlight {
        Highlight::default()
    }
    fn highlight_cell(&self, _x: i32, _y: i32, _cell: &Cell, _selected: bool) {}
    fn clear_cell(&self, _x: i32, _y: i32, _cell: &Cell) {}
}

pub trait SolverObserver {
    fn display_guesses(&self, _guesses: &[Guess]) {}
}

#[derive(Clone)]
pub struct DummyGridObserver {}
impl GridObserver for DummyGridObserver {}

#[derive(Clone)]
pub struct TermObserver {}
impl TermObserver {
    pub fn new() -> TermObserver {
        print!("{}{}", clear::All, cursor::Hide);
        TermObserver {}
    }
}

impl Drop for TermObserver {
    fn drop(&mut self) {
        print!("{}", cursor::Show);
    }
}

impl GridObserver for TermObserver {
    fn highlight_block(&self, x: i32, y: i32) -> Highlight {
        let x = x * GRID_SIZE;
        let y = y * GRID_SIZE;
        let width = GRID_SIZE * 3;
        let height = GRID_SIZE * 3;
        draw_rectangle(x, y, width, height, "#");
        Highlight {
            x,
            y,
            width,
            height,
        }
    }

    fn highlight_row(&self, y: i32) -> Highlight {
        let x = 0;
        let y = y * GRID_SIZE;
        let width = GRID_SIZE * 9;
        let height = GRID_SIZE;
        draw_rectangle(x, y, width, height, "#");
        Highlight {
            x,
            y,
            width,
            height,
        }
    }

    fn highlight_column(&self, x: i32) -> Highlight {
        let x = x * GRID_SIZE;
        let y = 0;
        let width = GRID_SIZE;
        let height = GRID_SIZE * 9;
        draw_rectangle(x, y, width, height, "#");
        Highlight {
            x,
            y,
            width,
            height,
        }
    }

    fn highlight_cell(&self, cell_x: i32, cell_y: i32, cell: &Cell, selected: bool) {
        for digit in 1..10 {
            let character = if cell.possibles.contains(&digit) {
                digit.to_string()
            } else {
                " ".to_string()
            };
            if selected {
                print!("{}", color::Fg(color::Magenta));
            } else {
                print!("{}", color::Fg(color::Blue));
            }
            let x = (2 + GRID_SIZE * cell_x + (digit - 1) % 3) as u16;
            let y = (2 + GRID_SIZE * cell_y + (digit - 1) / 3) as u16;
            print!("{}{}", cursor::Goto(x, y), character);
            print!("{}", color::Fg(color::Black));
        }
        flush();
        thread::sleep(time::Duration::from_millis(1));
    }
    fn clear_cell(&self, cell_x: i32, cell_y: i32, cell: &Cell) {
        if cell.possibles.len() == 1 {
            print!("{}", color::Fg(color::Green));
        }
        for digit in 1..10 {
            let character = if cell.possibles.contains(&digit) {
                digit.to_string()
            } else {
                " ".to_string()
            };
            let x = (2 + GRID_SIZE * cell_x + (digit - 1) % 3) as u16;
            let y = (2 + GRID_SIZE * cell_y + (digit - 1) / 3) as u16;
            print!("{}{}", cursor::Goto(x, y), character)
        }
        print!("{}", color::Fg(color::Black));
    }
}

pub struct TermSolverObserver {}

impl TermSolverObserver {
    pub fn new() -> TermSolverObserver {
        TermSolverObserver {}
    }
}

impl SolverObserver for TermSolverObserver {
    fn display_guesses(&self, guesses: &[Guess]) {
        for (i, guess) in guesses.iter().enumerate() {
            print!(
                "{} ({}, {}): {} [",
                termion::cursor::Goto(5 * 9 + 2, (i + 1) as u16),
                guess.x,
                guess.y,
                guess.digit,
            );
            for digit in guess.remaining_possibles.iter() {
                print!("{}", digit);
                if digit != guess.remaining_possibles.last().unwrap() {
                    print!(", ");
                }
            }
            print!("]             ");
        }
    }
}
