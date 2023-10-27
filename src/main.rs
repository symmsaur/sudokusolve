extern crate bitmaps;
extern crate clap;
extern crate termion;

mod cell;
mod observer;
mod reader;
mod solver;
mod writer;

use std::fs::File;
use std::process::ExitCode;

use crate::observer::{DummyGridObserver, DummySolverObserver, TermObserver, TermSolverObserver};
use crate::solver::{ObserveableGrid, Solver, SudokuSolver};

use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    no_observe: bool,
}

fn main() -> ExitCode {
    let opts = Opts::parse();

    let output_filename = "output";
    if let Err(err) = File::create(output_filename) {
        println!("Failed to create file '{}', ({})", output_filename, err);
        return 1.into();
    }
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
        let input_filename = "testdata/easy";
        let solution = match reader::read(input_filename, &mut *solver, i) {
            Ok(()) => solver.solve(),
            Err(err) => {
                println!(
                    "Case {}: Failed to read file from file '{}', ({})",
                    i, input_filename, err
                );
                return 1.into();
            }
        };
        match solution {
            Some(solution) => match writer::write(output_filename, &solution) {
                Ok(_) => {}
                Err(err) => {
                    println!(
                        "Case {}: Failed to write to file '{}', ({})",
                        i, output_filename, err
                    );
                    return 1.into();
                }
            },
            None => {
                println!("Case {}: Failed to solve.", i)
            }
        };
    }
    0.into()
}
