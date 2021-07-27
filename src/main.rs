extern crate termion;

mod observer;

use std::vec::Vec;

use crate::observer::{GridObserver, TermObserver};

pub struct Cell {
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
}

trait Grid {
    fn cell_mut(&mut self, x: i32, y: i32) -> &mut Cell;
    fn cell(&self, x: i32, y: i32) -> &Cell;
    fn set_hint(&mut self, x: i32, y: i32, hint: i32);
    fn eliminate_in_block<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    );
    fn eliminate_in_row<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: F,
    );
    fn eliminate_in_column<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: F,
    );
}

struct ObserveableGrid<TObserver: GridObserver> {
    cells: Vec<Cell>, // Should have exactly 81 elements
    observer: TObserver,
}

impl<TObserver: GridObserver> ObserveableGrid<TObserver> {
    fn new(observer: TObserver) -> ObserveableGrid<TObserver> {
        ObserveableGrid {
            cells: (0..81).map(|_| Cell::new()).collect(),
            observer,
        }
    }
}

impl<TObserver: GridObserver> Grid for ObserveableGrid<TObserver> {
    fn cell_mut(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[(y * 9 + x) as usize]
    }

    fn cell(&self, x: i32, y: i32) -> &Cell {
        &self.cells[(y * 9 + x) as usize]
    }

    fn set_hint(&mut self, x: i32, y: i32, hint: i32) {
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        self.cell_mut(x, y).set_hint(hint);
        self.observer.clear_cell(x, y, self.cell(x, y));
    }

    fn eliminate_in_block<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    ) {
        let block_start_x = (x / 3) * 3;
        let block_start_y = (y / 3) * 3;
        self.observer.highlight_block(block_start_x, block_start_y);
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        for y_mod in block_start_y..block_start_y + 3 {
            for x_mod in block_start_x..block_start_x + 3 {
                if x_mod == x && y_mod == y {
                    continue;
                }
                self.observer
                    .highlight_cell(x_mod, y_mod, self.cell(x_mod, y_mod), false);
                self.cell_mut(x_mod, y_mod).eliminate_possible(digit);
                self.observer
                    .clear_cell(x_mod, y_mod, self.cell(x_mod, y_mod));
                if self.cell(x_mod, y_mod).possibles.len() == 1 {
                    mark_solved((x_mod, y_mod));
                }
            }
        }
        self.observer.clear_cell(x, y, self.cell(x, y));
        self.observer.clear_block(block_start_x, block_start_y);
    }

    fn eliminate_in_row<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mut mark_solved: F,
    ) {
        self.observer.highlight_row(y);
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        for x_mod in 0..9 {
            if x_mod == x {
                continue;
            }
            self.observer
                .highlight_cell(x_mod, y, self.cell(x_mod, y), false);
            self.cell_mut(x_mod, y).eliminate_possible(digit);
            self.observer.clear_cell(x_mod, y, self.cell(x_mod, y));
            if self.cell(x_mod, y).possibles.len() == 1 {
                mark_solved((x_mod, y));
            }
        }
        self.observer.clear_cell(x, y, self.cell(x, y));
        self.observer.clear_row(y);
    }

    fn eliminate_in_column<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mut mark_solved: F,
    ) {
        self.observer.highlight_column(x);
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        for y_mod in 0..9 {
            if y_mod == y {
                continue;
            }
            self.observer
                .highlight_cell(x, y_mod, self.cell(x, y_mod), false);
            self.cell_mut(x, y_mod).eliminate_possible(digit);
            self.observer.clear_cell(x, y_mod, self.cell(x, y_mod));
            if self.cell(x, y_mod).possibles.len() == 1 {
                mark_solved((x, y_mod));
            }
        }
        self.observer.clear_cell(x, y, self.cell(x, y));
        self.observer.clear_column(x);
    }
}

struct SudokuSolver<TGrid: Grid> {
    grid: TGrid,
    solved_cells: Vec<(i32, i32)>,
}

impl<TGrid: Grid> SudokuSolver<TGrid> {
    fn new(grid: TGrid) -> SudokuSolver<TGrid> {
        SudokuSolver {
            grid: grid,
            solved_cells: Vec::new(),
        }
    }

    fn set_hint(&mut self, x: i32, y: i32, hint: i32) {
        self.grid.set_hint(x, y, hint);
        assert!(!self.solved_cells.contains(&(x, y)));
        self.solved_cells.push((x, y));
    }

    fn solve(&mut self) {
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
        }
    }
}

fn main() {
    let observer = TermObserver::new();
    let grid = ObserveableGrid::new(observer);
    let mut solver = SudokuSolver::new(grid);

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
}
