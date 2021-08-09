use crate::observer::{GridObserver, SolverObserver};

#[derive(Clone)]
pub struct Cell {
    pub possibles: Vec<i32>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            possibles: (1..10).collect(),
        }
    }

    fn set_hint(self: &mut Cell, hint: i32) {
        self.possibles.clear();
        self.possibles.push(hint);
    }

    fn eliminate_possible(self: &mut Cell, digit: i32) -> Result<(), EliminationError> {
        self.possibles.retain(|i| *i != digit);
        if self.possibles.len() == 0 {
            Err(EliminationError {})
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct EliminationError {}

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
                if self.cell(x_mod, y_mod).possibles.len() == 1 {
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
            if self.cell(x_mod, y).possibles.len() == 1 {
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
            if self.cell(x, y_mod).possibles.len() == 1 {
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
    solved_cells: Vec<(i32, i32)>,
}

impl<TGrid: Grid, TObserver: SolverObserver> Solver for SudokuSolver<TGrid, TObserver> {
    fn set_hint(&mut self, x: i32, y: i32, hint: i32) {
        self.grid.set_hint(x, y, hint);
        if !self.solved_cells.contains(&(x, y)) {
            self.solved_cells.push((x, y));
        }
    }

    fn solve(&mut self) {
        let mut eliminated_cells = Vec::new();

        let mut guesses: Vec<Guess> = Vec::new();
        let mut old_grids = Vec::new();
        let mut solved_stack = Vec::new();
        let mut eliminated_stack = Vec::new();

        while eliminated_cells.len() < 81 {
            let mut fail = false;
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
                match self.grid.eliminate(x, y, digit, &mut push_cell) {
                    Ok(_) => {}
                    Err(_) => {
                        fail = true;
                        break;
                    }
                }
            }
            if eliminated_cells.len() == 81 {
                break;
            }
            if fail {
                while let Some(guess) = guesses.pop() {
                    let grid = old_grids.pop().unwrap();
                    let eliminated_old = eliminated_stack.pop().unwrap();
                    let solved_old = solved_stack.pop().unwrap();
                    // This guess was wrong, can we make a new one?
                    if guess.remaining_possibles.len() > 0 {
                        self.grid = grid;
                        eliminated_cells = eliminated_old;
                        self.solved_cells = solved_old;
                        self.grid
                            .cell_mut(guess.x, guess.y)
                            .eliminate_possible(guess.digit)
                            .expect("Should always be able to eliminate");
                        if self.grid.cell(guess.x, guess.y).possibles.len() == 1 {
                            self.solved_cells.push((guess.x, guess.y));
                        }
                        let digit = guess.remaining_possibles[0];

                        guesses.push(Guess {
                            x: guess.x,
                            y: guess.y,
                            digit,
                            remaining_possibles: guess
                                .remaining_possibles
                                .into_iter()
                                .filter(|x| *x != digit)
                                .collect(),
                        });
                        old_grids.push(self.grid.clone());
                        solved_stack.push(self.solved_cells.clone());
                        eliminated_stack.push(eliminated_cells.clone());
                        self.grid.invalidate();
                        break;
                    }
                }
            } else {
                guesses.push(self.find_guess());
                old_grids.push(self.grid.clone());
                solved_stack.push(self.solved_cells.clone());
                eliminated_stack.push(eliminated_cells.clone());
            }

            let guess: &Guess = &guesses.last().unwrap();
            self.set_hint(guess.x, guess.y, guess.digit);
            self.observer.display_guesses(&guesses);
        }
    }
}

impl<TGrid: Grid, TObserver: SolverObserver> SudokuSolver<TGrid, TObserver> {
    pub fn new(grid: TGrid, observer: TObserver) -> SudokuSolver<TGrid, TObserver> {
        SudokuSolver {
            grid,
            observer,
            solved_cells: Vec::new(),
        }
    }

    fn find_guess(&self) -> Guess {
        for num_digits in 2..10 {
            for y in 0..9 {
                for x in 0..9 {
                    let cell = self.grid.cell(x, y);
                    if cell.possibles.len() == num_digits {
                        let digit = *cell.possibles.first().unwrap();
                        return Guess {
                            x,
                            y,
                            digit,
                            remaining_possibles: cell
                                .possibles
                                .iter()
                                .cloned()
                                .filter(|x| *x != digit)
                                .collect(),
                        };
                    }
                }
            }
        }
        panic!("No guess available");
    }
}
