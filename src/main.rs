extern crate bitmaps;
extern crate clap;
extern crate termion;

mod cell;
mod observer;
mod reader;
mod solver;

use crate::observer::{DummyGridObserver, DummySolverObserver, TermObserver, TermSolverObserver};
use crate::solver::{ObserveableGrid, Solver, SudokuSolver};

use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    no_observe: bool,
}

fn main() {
    let opts = Opts::parse();

    for i in 0..1011 {
        let mut solver: Box<dyn Solver> = if !opts.no_observe {
            let observer = TermObserver::new();
            let grid = ObserveableGrid::new(observer);
            let solver_observer = TermSolverObserver::new();
            Box::new(SudokuSolver::new(grid, solver_observer))
        } else {
            let observer = DummyGridObserver {};
            let grid = ObserveableGrid::new(observer);
            let solver_observer = DummySolverObserver {};
            Box::new(SudokuSolver::new(grid, solver_observer))
        };
        match reader::read("testdata/easy", &mut *solver, i) {
            Ok(()) => {
                solver.solve();
            }
            Err(msg) => {
                println!("Failed to read file ({})", msg);
            }
        }
    }
}
