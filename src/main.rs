use std::env;
use std::fs::File;
use std::io::{BufReader, Error};
mod sudoku;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }
    println!("Sudoku grid text file path : {:?}", &args[1]);
    let input = File::open(&args[1])?;
    let buffered = BufReader::new(input);

    let sudoku_grid = match sudoku::build_sudoku_grid(buffered) {
        Err(e) => return Err(e),
        Ok(sudoku_grid) => sudoku_grid
    };

    let resolved_sudoku_grid = sudoku::resolve_sudoku_grid(sudoku_grid);
    sudoku::print_sudoku_grid(&resolved_sudoku_grid);
    sudoku::write_sudoku_grid(&resolved_sudoku_grid, "sudoku_resolved.txt")?;
    Ok(())
}
