extern crate termion;

use std::vec::Vec;
use std::{thread, time};

use termion::{clear, cursor};

struct Cell {
    possibles: Vec<i32>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            possibles: (1..10).collect(),
        }
    }

    fn set_hint(self: &mut Cell, hint: i32) {
        assert!(self.possibles == (1..10).collect::<Vec<i32>>());
        self.possibles.clear();
        self.possibles.push(hint);
    }

    fn eliminate_possible(self: &mut Cell, digit: i32) {
        self.possibles.retain(|i| *i != digit);
    }

    fn print(self: &Cell, x_offset: i32, y_offset: i32) {
        for digit in self.possibles.iter() {
            let x = (1 + x_offset + (digit - 1) % 3) as u16;
            let y = (1 + y_offset + (digit - 1) / 3) as u16;
            print!("{}{}", cursor::Goto(x, y), digit)
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

    fn eliminate_in_block<F: FnMut((i32, i32))>(
        self: &mut Grid,
        x: i32,
        y: i32,
        digit: i32,
        mut mark_solved: F,
    ) {
        let block_start_x = (x / 3) * 3;
        let block_start_y = (y / 3) * 3;
        for y_mod in block_start_y..block_start_y + 3 {
            for x_mod in block_start_x..block_start_x + 3 {
                if x_mod == x && y_mod == y {
                    continue;
                }
                self.cell_mut(x_mod, y_mod).eliminate_possible(digit);
                if self.cell(x_mod, y_mod).possibles.len() == 1 {
                    mark_solved((x_mod, y_mod));
                }
            }
        }
    }

    fn eliminate_in_row<F: FnMut((i32, i32))>(
        self: &mut Grid,
        x: i32,
        y: i32,
        digit: i32,
        mut mark_solved: F,
    ) {
        for x_mod in 0..9 {
            if x_mod == x {
                continue;
            }
            self.cell_mut(x_mod, y).eliminate_possible(digit);
            if self.cell(x_mod, y).possibles.len() == 1 {
                mark_solved((x_mod, y));
            }
        }
    }

    fn eliminate_in_column<F: FnMut((i32, i32))>(
        self: &mut Grid,
        x: i32,
        y: i32,
        digit: i32,
        mut mark_solved: F,
    ) {
        for y_mod in 0..9 {
            if y_mod == y {
                continue;
            }
            self.cell_mut(x, y_mod).eliminate_possible(digit);
            if self.cell(x, y_mod).possibles.len() == 1 {
                mark_solved((x, y_mod));
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

struct SudokuSolver {
    grid: Grid,
    solved_cells: Vec<(i32, i32)>,
}

impl SudokuSolver {
    fn new() -> SudokuSolver {
        SudokuSolver {
            grid: Grid::new(),
            solved_cells: Vec::new(),
        }
    }

    fn set_hint(self: &mut SudokuSolver, x: i32, y: i32, hint: i32) {
        self.grid.set_hint(x, y, hint);
        assert!(!self.solved_cells.contains(&(x, y)));
        self.solved_cells.push((x, y));
    }

    fn solve(self: &mut SudokuSolver) {
        let mut eliminated_cells = Vec::new();
        while let Some((x, y)) = self.solved_cells.pop() {
            assert!(!eliminated_cells.contains(&(x, y)));
            eliminated_cells.push((x, y));

            let solved_cells_ref = &mut self.solved_cells;
            let mut push_cell = |pos| {
                if !eliminated_cells.contains(&pos) && !solved_cells_ref.contains(&pos) {
                    solved_cells_ref.push(pos);
                }
            };

            assert!(self.grid.cell(x, y).possibles.len() == 1);
            let digit = self.grid.cell(x, y).possibles[0];
            self.grid.eliminate_in_block(x, y, digit, &mut push_cell);
            self.grid.eliminate_in_row(x, y, digit, &mut push_cell);
            self.grid.eliminate_in_column(x, y, digit, &mut push_cell);

            // It's more fun if you can see the puzzle being solved
            print!("{}", clear::All);
            self.grid.print();
            println!();
            thread::sleep(time::Duration::from_millis(25));
        }
    }
}

fn main() {
    let mut solver = SudokuSolver::new();

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
