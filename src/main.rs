use std::fs::File;
use std::io::{BufReader, Error};
mod sudoku;

fn main() -> Result<(), Error> {
    let path = "sudoku.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let sudoku_grid = match sudoku::build_sudoku_grid(buffered) {
        Err(e) => return Err(e),
        Ok(sudoku_grid) => sudoku_grid
    };
    println!("Sudoku grid loaded:");
    sudoku::print_sudoku_grid(&sudoku_grid);

    let resolved_sudoku_grid = sudoku::resolve_sudoku_grid(&sudoku_grid);
    println!("Sudoku grid resolved:");
    sudoku::print_sudoku_grid(&resolved_sudoku_grid);
    sudoku::write_sudoku_grid(&resolved_sudoku_grid, "sudoku_resolved.txt")?;
   
    Ok(())
}
