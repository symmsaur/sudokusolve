use crate::cell::{Cell, EliminationError};
use crate::observer::{GridObserver, SolverObserver};
use bitmaps::Bitmap;

pub trait Grid: Clone {
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
}

#[derive(Clone)]
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
}

#[derive(Clone)]
pub struct Guess {
    pub x: i32,
    pub y: i32,
    pub digit: i32,
    pub remaining_possibles: Vec<i32>,
}

pub trait Solver {
    fn set_hint(&mut self, x: i32, y: i32, hint: i32);
    fn solve(&mut self);
}

pub struct SudokuSolver<TGrid: Grid, TObserver: SolverObserver> {
    grid: TGrid,
    observer: TObserver,
    cells_to_eliminate: Vec<(i32, i32)>,
}

struct SolverState<TGrid: Grid> {
    guess: Guess,
    grid: TGrid,
    eliminated: Bitmap<81>,
    solved: Vec<(i32, i32)>,
}

impl<TGrid: Grid, TObserver: SolverObserver> Solver for SudokuSolver<TGrid, TObserver> {
    fn set_hint(&mut self, x: i32, y: i32, hint: i32) {
        self.grid.set_hint(x, y, hint);
        if !self.cells_to_eliminate.contains(&(x, y)) {
            self.cells_to_eliminate.push((x, y));
        }
    }

    fn solve(&mut self) {
        let mut solved_cells = Bitmap::<81>::new();

        let mut state_stack: Vec<SolverState<TGrid>> = Vec::new();

        while solved_cells.len() < 81 {
            let mut fail = false;
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
                match self.grid.eliminate(x, y, digit, &mut push_cell) {
                    Ok(_) => {}
                    Err(_) => {
                        fail = true;
                        break;
                    }
                }
            }
            if solved_cells.len() == 81 {
                break;
            }
            if fail {
                while let Some(state) = state_stack.pop() {
                    // This guess was wrong, can we make a new one?
                    if state.guess.remaining_possibles.len() > 0 {
                        self.grid = state.grid;
                        solved_cells = state.eliminated;
                        self.cells_to_eliminate = state.solved;
                        self.grid
                            .cell_mut(state.guess.x, state.guess.y)
                            .eliminate_possible(state.guess.digit)
                            .expect("Should always be able to eliminate");
                        if self.grid.cell(state.guess.x, state.guess.y).num_possibles() == 1 {
                            self.cells_to_eliminate.push((state.guess.x, state.guess.y));
                        }
                        let digit = state.guess.remaining_possibles[0];

                        state_stack.push(SolverState {
                            guess: Guess {
                                x: state.guess.x,
                                y: state.guess.y,
                                digit,
                                remaining_possibles: state
                                    .guess
                                    .remaining_possibles
                                    .into_iter()
                                    .filter(|x| *x != digit)
                                    .collect(),
                            },
                            grid: self.grid.clone(),
                            solved: self.cells_to_eliminate.clone(),
                            eliminated: solved_cells.clone(),
                        });
                        self.grid.invalidate();
                        break;
                    }
                }
            } else {
                state_stack.push(SolverState {
                    guess: self.find_guess(),
                    grid: self.grid.clone(),
                    solved: self.cells_to_eliminate.clone(),
                    eliminated: solved_cells.clone(),
                });
            }

            let guess: &Guess = &state_stack.last().unwrap().guess;
            self.set_hint(guess.x, guess.y, guess.digit);
            let vec: Vec<Guess> = state_stack
                .iter()
                .map(|s| s.guess.clone())
                .collect::<Vec<Guess>>();
            self.observer.display_guesses(&vec);
        }
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
