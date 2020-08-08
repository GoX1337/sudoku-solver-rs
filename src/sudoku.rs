use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

#[derive(Debug, Clone)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub value: u8,
    pub candidates: Vec<u8>
}

pub fn print_sudoku_grid(sudoku_grid: &Vec<Vec<Cell>>) {
    for y in 0..sudoku_grid.len() {
        for x in 0..sudoku_grid[y].len() {
            print!("{:?} ", sudoku_grid[y][x].value);
        }
        println!("");
    }
}

pub fn write_sudoku_grid(sudoku_grid: &Vec<Vec<Cell>>, path: &str) -> Result<(), Error> {
    let mut output = File::create(path)?;
    for y in 0..sudoku_grid.len() {
        for x in 0..sudoku_grid[y].len() {
            write!(output, "{} ", sudoku_grid[y][x].value)?;
        }
        write!(output, "\n")?;
    }
    Ok(())
}

pub fn build_sudoku_grid(buffered: BufReader<std::fs::File>) -> Result<Vec<Vec<Cell>>, Error> {
    let mut sudoku_grid: Vec<Vec<Cell>> = Vec::new();
    let candidate_list: Vec<u8> = (1..10).collect();
    let mut y = 0;

    for line_str in buffered.lines() {
        let line = match line_str {
            Err(e) => return Err(e),
            Ok(line) => line
        };
        let line_vals: Vec<&str> = line.split(",").collect();
        let mut y_cells: Vec<Cell> = Vec::new();

        for i in 0..line_vals.len() {
            let val = line_vals[i].parse().unwrap();
            let mut cand;
            if val == 0 {
                cand = candidate_list.clone();
                cand.retain(|x| *x != val);
            } else {
                cand = Vec::new();
            }

            let cell = Cell {
                x: i as u8,
                y: y,
                value: val,
                candidates: cand
            };
            y_cells.push(cell);
        }
        y = y + 1;
        sudoku_grid.push(y_cells);
    }
    Ok(sudoku_grid)
}

pub fn clone_sudoku_grid(sudoku_grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut clone: Vec<Vec<Cell>> = Vec::new();
    for y in 0..sudoku_grid.len() {
        let mut line = Vec::new();
        for x in 0..sudoku_grid[y].len() {
            line.push(sudoku_grid[y][x].clone());
        }
        clone.push(line);
    }
    clone
}

fn clean_candidates_by_lines_columns(sudoku_grid: &mut Vec<Vec<Cell>>) {
    for y in 0..sudoku_grid.len() {
        let mut line_values = Vec::new();
        for x in 0..sudoku_grid[y].len() {
            let cell = &sudoku_grid[y][x];
            if cell.value != 0 {
                line_values.push(cell.value);
            }
        }
        for x in 0..sudoku_grid[y].len() {
            let cell = &mut sudoku_grid[y][x];
            if cell.value == 0 {
                for c in 0..line_values.len() {
                    cell.candidates.retain(|x| *x != line_values[c]);
                }
                if cell.candidates.len() == 1 {
                    println!("Value {} found at x:{},y:{}", cell.candidates[0], cell.x, cell.y);
                    cell.value = cell.candidates[0];
                }
            }
        }
    }

    for x in 0..sudoku_grid.len() {
        let mut line_values = Vec::new();
        for y in 0..sudoku_grid.len() {
            let cell = &sudoku_grid[y][x];
            if cell.value != 0 {
                line_values.push(cell.value);
            }
        }
        for y in 0..sudoku_grid.len() {
            let cell = &mut sudoku_grid[y][x];
            if cell.value == 0 {
                for c in 0..line_values.len() {
                    cell.candidates.retain(|x| *x != line_values[c]);
                }
                if cell.candidates.len() == 1 {
                    println!("Value {} found at x:{},y:{}", cell.candidates[0], cell.x, cell.y);
                    cell.value = cell.candidates[0];
                }
            }
        }
    }
}

fn clean_candidates_by_boxes(sudoku_grid: &mut Vec<Vec<Cell>>) {
    let mut j = 0;
    while j < 9 {
        let mut i = 0;
        while i < 9 {
            let mut box_values = Vec::new();
            for x in i..i+3 {
                for y in j..j+3 {
                    let cell = &mut sudoku_grid[y][x];
                    if cell.value != 0 {
                        box_values.push(cell.value);
                    }
                }
            }
            for x in i..i+3 {
                for y in j..j+3 {
                    let cell = &mut sudoku_grid[y][x];
                    if cell.value == 0 {
                        for c in 0..box_values.len() {
                            cell.candidates.retain(|x| *x != box_values[c]);
                        }
                        if cell.candidates.len() == 1 {
                            println!("Value {} found at x:{},y:{}", cell.candidates[0], cell.x, cell.y);
                            cell.value = cell.candidates[0];
                        }
                    }
                }
            }
           i = i + 3;
        }
        j = j + 3;
    }
}

pub fn resolve_sudoku_grid(sudoku_grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut resolved_sudoku_grid: Vec<Vec<Cell>> = clone_sudoku_grid(&sudoku_grid);
    print_sudoku_grid(&resolved_sudoku_grid);

    println!("\nClean candidates by lines and columns:");
    clean_candidates_by_lines_columns(&mut resolved_sudoku_grid);

    println!("\nClean candidates by 3x3 boxes:");
    clean_candidates_by_boxes(&mut resolved_sudoku_grid);
    println!("");
    resolved_sudoku_grid
}
