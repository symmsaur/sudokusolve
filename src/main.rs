extern crate termion;

mod observer;
mod reader;
mod solver;

use crate::observer::{TermObserver, TermSolverObserver};
use crate::solver::{ObserveableGrid, SudokuSolver};

fn main() {
    for i in 0..1011 {
        let observer = TermObserver::new();
        //let observer = observer::DummyObserver{};
        let grid = ObserveableGrid::new(observer);
        let solver_observer = TermSolverObserver::new();
        let mut solver = SudokuSolver::new(grid, solver_observer);

        match reader::read("testdata/easy", &mut solver, i) {
            Ok(()) => {
                solver.solve();
            }
            Err(msg) => {
                println!("Failed to read file ({})", msg);
            }
        }
    }
}
