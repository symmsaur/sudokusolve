extern crate termion;

use std::vec::Vec;

use termion::clear;
use termion::cursor;

struct Cell {
    possible: Vec<i32>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            possible: (1..10).collect(),
        }
    }

    fn set_hint(self: &mut Cell, hint: i32) {
        assert!(self.possible == (1..10).collect::<Vec<i32>>());
        self.possible.clear();
        self.possible.push(hint);
    }

    fn remove_possible(self: &mut Cell, number: i32) {
        self.possible.retain(|i| *i != number);
    }

    fn print(self: &Cell, x_offset: i32, y_offset: i32) {
        for number in self.possible.iter() {
            let x = (1 + x_offset + (number - 1) % 3) as u16;
            let y = (1 + y_offset + (number - 1) / 3) as u16;
            print!("{}{}", cursor::Goto(x, y), number)
        }
    }
}

struct Grid {
    cells: Vec<Cell>, // Should have exactly 81 elements
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: (0..81).map(|_| Cell::new()).collect(),
        }
    }

    fn cell_mut(self: &mut Grid, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[(y * 9 + x) as usize]
    }

    fn cell(self: &Grid, x: i32, y: i32) -> &Cell {
        &self.cells[(y * 9 + x) as usize]
    }

    fn set_hint(self: &mut Grid, x: i32, y: i32, hint: i32) {
        let cell = self.cell_mut(x, y);
        cell.set_hint(hint);
    }

    fn propagate_block<F: FnMut((i32, i32))>(
        self: &mut Grid,
        x: i32,
        y: i32,
        number: i32,
        mut solved_fn: F,
    ) {
        let block_start_x = (x / 3) * 3;
        let block_start_y = (y / 3) * 3;
        for y_mod in block_start_y..block_start_y + 3 {
            for x_mod in block_start_x..block_start_x + 3 {
                if x_mod == x && y_mod == y {
                    continue;
                }
                self.cell_mut(x_mod, y_mod).remove_possible(number);
                if self.cell(x_mod, y_mod).possible.len() == 1 {
                    solved_fn((x_mod, y_mod));
                }
            }
        }
    }

    fn propagate_row<F: FnMut((i32, i32))>(self: &mut Grid, x: i32, y: i32, number: i32, mut solved_fn: F) {
        for x_mod in 0..9 {
            if x_mod == x {
                continue;
            }
            self.cell_mut(x_mod, y).remove_possible(number);
            if self.cell(x_mod, y).possible.len() == 1 {
                solved_fn((x_mod, y));
            }
        }
    }

    fn propagate_column<F: FnMut((i32, i32))>(self: &mut Grid, x: i32, y: i32, number: i32, mut solved_fn: F) {
        for y_mod in 0..9 {
            if y_mod == y {
                continue;
            }
            self.cell_mut(x, y_mod).remove_possible(number);
            if self.cell(x, y_mod).possible.len() == 1 {
                solved_fn((x, y_mod));
            }
        }
    }

    fn print(self: &Grid) {
        for y in 0..9 {
            for x in 0..9 {
                self.cell(x, y).print(x * 5, y * 5);
            }
        }
    }
}

struct Solver {
    grid: Grid,
    cell_stack: Vec<(i32, i32)>,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            grid: Grid::new(),
            cell_stack: Vec::new(),
        }
    }

    fn set_hint(self: &mut Solver, x: i32, y: i32, hint: i32) {
        self.grid.set_hint(x, y, hint);
        assert!(!self.cell_stack.contains(&(x, y)));
        self.cell_stack.push((x, y));
    }

    fn solve(self: &mut Solver) {
        let mut finished = Vec::new();
        while let Some((x, y)) = self.cell_stack.pop() {
            assert!(self.grid.cell(x, y).possible.len() == 1);
            assert!(!finished.contains(&(x, y)));
            println!("len {}", self.cell_stack.len());
            println!("x {} y {}", x, y);
            finished.push((x, y));
            let number = self.grid.cell(x, y).possible[0];
            let cell_stack_ref = &mut self.cell_stack;
            let mut push_cell = |pos| {
                if !finished.contains(&pos) && !cell_stack_ref.contains(&pos) {
                    println!("Pushing ({}, {})", pos.0, pos.1);
                    cell_stack_ref.push(pos);
                }
            };
            self.grid.propagate_block(x, y, number, &mut push_cell);
            self.grid.propagate_row(x, y, number, &mut push_cell);
            self.grid.propagate_column(x, y, number, &mut push_cell);
        }
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.set_hint(0, 0, 5);
    solver.set_hint(1, 0, 3);
    solver.set_hint(4, 0, 7);

    solver.set_hint(0, 1, 6);
    solver.set_hint(3, 1, 1);
    solver.set_hint(4, 1, 9);
    solver.set_hint(5, 1, 5);

    solver.set_hint(1, 2, 9);
    solver.set_hint(2, 2, 8);
    solver.set_hint(7, 2, 6);

    solver.set_hint(0, 3, 8);
    solver.set_hint(4, 3, 6);
    solver.set_hint(8, 3, 3);

    solver.set_hint(0, 4, 4);
    solver.set_hint(3, 4, 8);
    solver.set_hint(5, 4, 3);
    solver.set_hint(8, 4, 1);

    solver.set_hint(0, 5, 7);
    solver.set_hint(4, 5, 2);
    solver.set_hint(8, 5, 6);

    solver.set_hint(1, 6, 6);
    solver.set_hint(6, 6, 2);
    solver.set_hint(7, 6, 8);

    solver.set_hint(3, 7, 4);
    solver.set_hint(4, 7, 1);
    solver.set_hint(5, 7, 9);
    solver.set_hint(8, 7, 5);

    solver.set_hint(4, 8, 8);
    solver.set_hint(7, 8, 7);
    solver.set_hint(8, 8, 9);
    solver.solve();
    print!("{}", clear::All);
    solver.grid.print();
    println!();
}
