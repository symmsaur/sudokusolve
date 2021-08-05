use crate::Cell;

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

pub trait GridObserver: Clone {
    fn highlight_block(&self, _x: i32, _y: i32) {}
    fn clear_block(&self, _x: i32, _y: i32) {}
    fn highlight_row(&self, _y: i32) {}
    fn clear_row(&self, _y: i32) {}
    fn highlight_column(&self, _x: i32) {}
    fn clear_column(&self, _x: i32) {}
    fn highlight_cell(&self, _x: i32, _y: i32, _cell: &Cell, _selected: bool) {}
    fn clear_cell(&self, _x: i32, _y: i32, _cell: &Cell) {}
}

#[derive(Clone)]
pub struct DummyObserver {}
impl GridObserver for DummyObserver {}

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
    fn highlight_block(&self, x: i32, y: i32) {
        draw_rectangle(
            x * GRID_SIZE,
            y * GRID_SIZE,
            GRID_SIZE * 3,
            GRID_SIZE * 3,
            "#",
        );
    }
    fn clear_block(&self, x: i32, y: i32) {
        draw_rectangle(
            x * GRID_SIZE,
            y * GRID_SIZE,
            GRID_SIZE * 3,
            GRID_SIZE * 3,
            " ",
        );
    }
    fn highlight_row(&self, y: i32) {
        draw_rectangle(0, y * GRID_SIZE, GRID_SIZE * 9, GRID_SIZE, "#");
    }
    fn clear_row(&self, y: i32) {
        draw_rectangle(0, y * GRID_SIZE, GRID_SIZE * 9, GRID_SIZE, " ");
    }
    fn highlight_column(&self, x: i32) {
        draw_rectangle(x * GRID_SIZE, 0, GRID_SIZE, GRID_SIZE * 9, "#");
    }
    fn clear_column(&self, x: i32) {
        draw_rectangle(x * GRID_SIZE, 0, GRID_SIZE, GRID_SIZE * 9, " ");
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
        thread::sleep(time::Duration::from_millis(10));
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
