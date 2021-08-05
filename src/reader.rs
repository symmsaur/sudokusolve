use std::fs::File;
use std::io::prelude::*;

use crate::solver::{Grid, SudokuSolver};

#[allow(dead_code)] // Useful as hardcoded example
pub fn read_hardcoded<TGrid: Grid>(solver: &mut SudokuSolver<TGrid>) {
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
}

pub fn read<TGrid: Grid>(
    filename: &str,
    solver: &mut SudokuSolver<TGrid>,
    offset: usize,
) -> std::io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if let Some(first) = contents.lines().skip(offset).next() {
        for (i, c) in first.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                let x = (i % 9) as i32;
                let y = (i / 9) as i32;
                solver.set_hint(x, y, digit as i32);
            }
        }
    }
    Ok(())
}
