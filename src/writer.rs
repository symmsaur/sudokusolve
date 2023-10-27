use std::fs::File;
use std::io::prelude::*;

pub fn write(filename: &str, solution: &Vec<i32>) -> std::io::Result<()> {
    let mut file = File::options().append(true).open(filename)?;
    let solution_string = solution
        .iter()
        .map(|digit| digit.to_string())
        .collect::<String>()
        + "\n";
    file.write_all(solution_string.as_bytes())?;
    Ok(())
}
