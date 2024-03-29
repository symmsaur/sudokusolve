use crate::cell::Cell;
use crate::solver::Guess;

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
    active: bool,
}

impl Drop for Highlight {
    fn drop(&mut self) {
        if self.active {
            draw_rectangle(self.x, self.y, self.width, self.height, " ");
        }
    }
}

pub trait GridObserver: Clone + std::fmt::Debug {
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
    fn display_guesses(&mut self, _guesses: &[Guess]) {}
}

#[derive(Clone, Debug)]
pub struct DummyGridObserver {}
impl GridObserver for DummyGridObserver {}

#[derive(Clone, Debug)]
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
            active: true,
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
            active: true,
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
            active: true,
        }
    }

    fn highlight_cell(&self, cell_x: i32, cell_y: i32, cell: &Cell, selected: bool) {
        for digit in 1..10 {
            let character = if cell.is_possible(digit) {
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
        thread::sleep(time::Duration::from_millis(20));
    }
    fn clear_cell(&self, cell_x: i32, cell_y: i32, cell: &Cell) {
        if cell.num_possibles() == 1 {
            print!("{}", color::Fg(color::Green));
        }
        for digit in 1..10 {
            let character = if cell.is_possible(digit) {
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

pub struct DummySolverObserver {}
impl SolverObserver for DummySolverObserver {}

pub struct TermSolverObserver {
    prev_num_guesses: usize,
}

impl TermSolverObserver {
    pub fn new() -> TermSolverObserver {
        TermSolverObserver {
            prev_num_guesses: 0,
        }
    }
}

impl SolverObserver for TermSolverObserver {
    fn display_guesses(&mut self, guesses: &[Guess]) {
        for i in guesses.len()..self.prev_num_guesses {
            print!(
                "{}                                  ",
                cursor::Goto(5 * 9 + 2, (i + 1) as u16),
            );
        }
        self.prev_num_guesses = guesses.len();
        for (i, guess) in guesses.iter().enumerate() {
            print!(
                "{} ({}, {}): {} [",
                cursor::Goto(5 * 9 + 2, (i + 1) as u16),
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
