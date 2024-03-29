use crate::cell::{Cell, EliminationError};
use crate::observer::{GridObserver, SolverObserver};
use bitmaps::Bitmap;

pub trait Grid: Clone + std::fmt::Debug {
    fn cell_mut(&mut self, x: i32, y: i32) -> &mut Cell;
    fn cell(&self, x: i32, y: i32) -> &Cell;
    fn set_hint(&mut self, x: i32, y: i32, hint: i32);
    fn eliminate<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    ) -> Result<(), EliminationError>;
    fn invalidate(&self);
    fn dump_solution(&self) -> Option<Vec<i32>>;
}

#[derive(Clone, Debug)]
pub struct ObserveableGrid<TObserver: GridObserver> {
    cells: Vec<Cell>, // Should have exactly 81 elements
    observer: TObserver,
}

impl<TObserver: GridObserver> ObserveableGrid<TObserver> {
    pub fn new(observer: TObserver) -> ObserveableGrid<TObserver> {
        ObserveableGrid {
            cells: (0..81).map(|_| Cell::new()).collect(),
            observer,
        }
    }

    fn eliminate_in_block<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    ) -> Result<(), EliminationError> {
        let block_start_x = (x / 3) * 3;
        let block_start_y = (y / 3) * 3;
        let _highlight = self.observer.highlight_block(block_start_x, block_start_y);
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        for y_mod in block_start_y..block_start_y + 3 {
            for x_mod in block_start_x..block_start_x + 3 {
                if x_mod == x && y_mod == y {
                    continue;
                }
                self.observer
                    .highlight_cell(x_mod, y_mod, self.cell(x_mod, y_mod), false);
                self.cell_mut(x_mod, y_mod).eliminate_possible(digit)?;
                self.observer
                    .clear_cell(x_mod, y_mod, self.cell(x_mod, y_mod));
                if self.cell(x_mod, y_mod).num_possibles() == 1 {
                    mark_solved((x_mod, y_mod));
                }
            }
        }
        self.observer.clear_cell(x, y, self.cell(x, y));
        Ok(())
    }

    fn eliminate_in_row<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    ) -> Result<(), EliminationError> {
        let _highlight = self.observer.highlight_row(y);
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        for x_mod in 0..9 {
            if x_mod == x {
                continue;
            }
            self.observer
                .highlight_cell(x_mod, y, self.cell(x_mod, y), false);
            self.cell_mut(x_mod, y).eliminate_possible(digit)?;
            self.observer.clear_cell(x_mod, y, self.cell(x_mod, y));
            if self.cell(x_mod, y).num_possibles() == 1 {
                mark_solved((x_mod, y));
            }
        }
        self.observer.clear_cell(x, y, self.cell(x, y));
        Ok(())
    }

    fn eliminate_in_column<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    ) -> Result<(), EliminationError> {
        let _highlight = self.observer.highlight_column(x);
        self.observer.highlight_cell(x, y, self.cell(x, y), true);
        for y_mod in 0..9 {
            if y_mod == y {
                continue;
            }
            self.observer
                .highlight_cell(x, y_mod, self.cell(x, y_mod), false);
            self.cell_mut(x, y_mod).eliminate_possible(digit)?;
            self.observer.clear_cell(x, y_mod, self.cell(x, y_mod));
            if self.cell(x, y_mod).num_possibles() == 1 {
                mark_solved((x, y_mod));
            }
        }
        self.observer.clear_cell(x, y, self.cell(x, y));
        Ok(())
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

    fn eliminate<F: FnMut((i32, i32))>(
        &mut self,
        x: i32,
        y: i32,
        digit: i32,
        mark_solved: &mut F,
    ) -> Result<(), EliminationError> {
        self.eliminate_in_block(x, y, digit, mark_solved)?;
        self.eliminate_in_row(x, y, digit, mark_solved)?;
        self.eliminate_in_column(x, y, digit, mark_solved)?;
        Ok(())
    }

    fn invalidate(&self) {
        for y in 0..9 {
            for x in 0..9 {
                self.observer.clear_cell(x, y, self.cell(x, y));
            }
        }
    }

    fn dump_solution(&self) -> Option<Vec<i32>> {
        let maybe_solution: Vec<Option<i32>> = self.cells.iter().map(|c| c.solution()).collect();
        if maybe_solution.iter().all(|i| i.is_some()) {
            Some(maybe_solution.iter().map(|i| i.unwrap()).collect())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Guess {
    pub x: i32,
    pub y: i32,
    pub digit: i32,
    pub remaining_possibles: Vec<i32>,
}

pub trait Solver {
    fn set_hint(&mut self, x: i32, y: i32, hint: i32);
    fn solve(&mut self) -> Option<Vec<i32>>;
}

pub struct SudokuSolver<TGrid: Grid, TObserver: SolverObserver> {
    grid: TGrid,
    observer: TObserver,
    cells_to_eliminate: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct SolverState<TGrid: Grid> {
    guess: Guess,
    grid: TGrid,
    solved_cells: Bitmap<81>,
    cells_to_eliminate: Vec<(i32, i32)>,
}

impl<TGrid: Grid, TObserver: SolverObserver> Solver for SudokuSolver<TGrid, TObserver> {
    fn set_hint(&mut self, x: i32, y: i32, hint: i32) {
        self.grid.set_hint(x, y, hint);
        if !self.cells_to_eliminate.contains(&(x, y)) {
            self.cells_to_eliminate.push((x, y));
        }
    }

    fn solve(&mut self) -> Option<Vec<i32>> {
        let mut solved_cells = Bitmap::<81>::new();
        let mut state_stack: Vec<SolverState<TGrid>> = Vec::new();

        while solved_cells.len() < 81 {
            match self.eliminate_all(solved_cells) {
                Ok(new_solved_cells) => {
                    solved_cells = new_solved_cells;
                }
                Err(_) => {
                    self.backtrack_and_make_new_guess(&mut state_stack, &mut solved_cells);
                    continue;
                }
            }
            if solved_cells.len() == 81 {
                break;
            }
            state_stack.push(SolverState {
                guess: self.find_guess(),
                grid: self.grid.clone(),
                cells_to_eliminate: self.cells_to_eliminate.clone(),
                solved_cells: solved_cells.clone(),
            });

            let guess: &Guess = &state_stack.last().unwrap().guess;
            self.set_hint(guess.x, guess.y, guess.digit);
            let vec: Vec<Guess> = state_stack
                .iter()
                .map(|s| s.guess.clone())
                .collect::<Vec<Guess>>();
            self.observer.display_guesses(&vec);
        }
        self.grid.dump_solution()
    }
}

impl<TGrid: Grid, TObserver: SolverObserver> SudokuSolver<TGrid, TObserver> {
    pub fn new(grid: TGrid, observer: TObserver) -> SudokuSolver<TGrid, TObserver> {
        SudokuSolver {
            grid,
            observer,
            cells_to_eliminate: Vec::new(),
        }
    }

    // FIXME: ugly procedure should be untangled
    fn backtrack_and_make_new_guess(
        &mut self,
        state_stack: &mut Vec<SolverState<TGrid>>,
        solved_cells: &mut Bitmap<81>,
    ) {
        // This guess was wrong, can we make a new one?
        while let Some(old_state) = state_stack.pop() {
            if old_state.guess.remaining_possibles.len() == 0 {
                continue;
            }
            self.grid = old_state.grid;
            *solved_cells = old_state.solved_cells;
            self.cells_to_eliminate = old_state.cells_to_eliminate;
            // Eliminate old guess.
            self.grid
                .cell_mut(old_state.guess.x, old_state.guess.y)
                .eliminate_possible(old_state.guess.digit)
                .expect("Should always be able to eliminate");
            if self
                .grid
                .cell(old_state.guess.x, old_state.guess.y)
                .num_possibles()
                == 1
            {
                self.cells_to_eliminate
                    .push((old_state.guess.x, old_state.guess.y));
            }
            let digit = old_state.guess.remaining_possibles[0];

            state_stack.push(SolverState {
                guess: Guess {
                    x: old_state.guess.x,
                    y: old_state.guess.y,
                    digit,
                    remaining_possibles: old_state
                        .guess
                        .remaining_possibles
                        .into_iter()
                        .filter(|x| *x != digit)
                        .collect(),
                },
                grid: self.grid.clone(),
                cells_to_eliminate: self.cells_to_eliminate.clone(),
                solved_cells: solved_cells.clone(),
            });
            self.grid.invalidate();
            break;
        }
    }

    fn eliminate_all(
        &mut self,
        mut solved_cells: Bitmap<81>,
    ) -> Result<Bitmap<81>, EliminationError> {
        while let Some((x, y)) = self.cells_to_eliminate.pop() {
            assert!(!solved_cells.get((y * 9 + x) as usize));
            solved_cells.set((y * 9 + x) as usize, true);

            let cells_to_eliminate_ref = &mut self.cells_to_eliminate;
            let mut push_cell = |pos| {
                if !cells_to_eliminate_ref.contains(&pos)
                    && !solved_cells.get((&pos.1 * 9 + &pos.0) as usize)
                {
                    cells_to_eliminate_ref.push(pos);
                }
            };

            assert!(self.grid.cell(x, y).num_possibles() == 1);
            let digit = self.grid.cell(x, y).first_possible().unwrap();
            self.grid.eliminate(x, y, digit, &mut push_cell)?;
        }
        Ok(solved_cells)
    }

    fn find_guess(&self) -> Guess {
        for num_digits in 2..10 {
            for y in 0..9 {
                for x in 0..9 {
                    let cell = self.grid.cell(x, y);
                    if cell.num_possibles() == num_digits {
                        let digit = cell.first_possible().unwrap();
                        return Guess {
                            x,
                            y,
                            digit,
                            remaining_possibles: cell.possibles_except(digit),
                        };
                    }
                }
            }
        }
        panic!("No guess available");
    }
}
