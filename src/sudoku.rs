use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub value: u8,
    pub candidates: Vec<u8>,
    pub candidate_index: usize
}

pub fn print_sudoku_grid(sudoku_grid: &Vec<Vec<Cell>>) {
    for y in 0..sudoku_grid.len() {
        if y > 0 && y % 3 == 0 {
            println!("");
        }
        for x in 0..sudoku_grid[y].len() {
            if x > 0 && x % 3 == 0 {
                print!(" ");
            }
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
                candidates: cand,
                candidate_index: 0
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
                    cell.candidates.clear();
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
                    cell.candidates.clear();
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
                            cell.candidates.clear();
                        }
                    }
                }
            }
           i = i + 3;
        }
        j = j + 3;
    }
}

fn is_valid_cell_value(sudoku_grid: Vec<Vec<Cell>>, cell: &mut Cell, value: u8) -> bool {
    for x in 0..sudoku_grid.len() {
        let cell = &sudoku_grid[cell.y as usize][x];
        if cell.value != 0 && cell.value == value {
            return false;
        }
    }
    for y in 0..sudoku_grid.len() {
        let cell = &sudoku_grid[y][cell.x as usize];
        if cell.value != 0 && cell.value == value {
            return false;
        }
    }
    
    let mut i = 0;
    let mut j = 0;
    while j <= 6 {
        while i <= 6 {
            if cell.x >= i && cell.x < i + 3 && cell.y >= j && cell.y < j + 3 {
                let mut box_values = Vec::new();
                for jj in j..j+3 {
                    for ii in i..i+3 {
                        box_values.push(&sudoku_grid[jj as usize][ii  as usize].value);
                    }
                }
                if box_values.contains(&&value) {
                    //println!("cell: {:?}, i:{}, j:{}, box: {:?}, value {} is NOT valid", cell, i, j, box_values, value);
                    return false;
                } else {
                    //println!("cell: {:?}, box: {:?}, value {} is valid", cell, box_values, value);
                }
            }
            i = i + 3;
        }
        j = j + 3;
    }
    true
}

fn start_bruteforce_sudoku(sudoku_grid: &mut Vec<Vec<Cell>>) {
    let mut i = 0;
    let mut y = 0;
    let mut last_tested_cells: VecDeque<Cell> = VecDeque::new();

    while y < 9 {
        let mut x = 0;
        let mut backtraking = false;
        while x < 9 {
            if sudoku_grid[y][x].value == 0 || backtraking {
                let mut c = sudoku_grid[y][x].candidate_index;
                if backtraking {
                    c = c + 1;
                }
                let mut value = 0;
                let mut candidate_valid = false;

                while !candidate_valid && c < sudoku_grid[y][x].candidates.len() {
                    value = sudoku_grid[y][x].candidates[c];
                    let grid_clone = clone_sudoku_grid(sudoku_grid);
                    candidate_valid = is_valid_cell_value(grid_clone, &mut sudoku_grid[y][x], value);
                    if !candidate_valid {
                        c = c + 1;
                    }
                }
                let cell = &mut sudoku_grid[y][x];
                if candidate_valid {
                    //println!("{:?}, value: {} is a valid candidate", cell, value);
                    i = 0;
                    last_tested_cells.push_back(cell.clone());
                    cell.value = value;
                    cell.candidate_index = c;
                    x = x + 1;
                    backtraking = false;
                } else {
                    cell.value = 0;
                    cell.candidate_index = 0;
                    let last_cell = last_tested_cells.pop_back();
                    match last_cell {
                        Some(last_cell) => {
                            x = last_cell.x as usize;
                            y = last_cell.y as usize;
                            //println!("{:?}, no candidate found. Go to last empty cell x:{}", cell, x);
                        },
                        None => return
                    };
                    backtraking = true;
                    i = i + 1;
                    if i > 9999 {
                        return;
                    }
                }
            } else {
                x = x + 1;
                backtraking = false;
            }
        }
        y = y + 1;
    }
}

pub fn resolve_sudoku_grid(sudoku_grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut resolved_sudoku_grid: Vec<Vec<Cell>> = clone_sudoku_grid(&sudoku_grid);
    print_sudoku_grid(&resolved_sudoku_grid);

    println!("\n-- Clean candidates by lines and columns:");
    clean_candidates_by_lines_columns(&mut resolved_sudoku_grid);
    print_sudoku_grid(&resolved_sudoku_grid);

    println!("\n-- Clean candidates by 3x3 boxes:");
    clean_candidates_by_boxes(&mut resolved_sudoku_grid);
    print_sudoku_grid(&resolved_sudoku_grid);

    println!("\n-- Brute-forcing the sudoku grid:");
    start_bruteforce_sudoku(&mut resolved_sudoku_grid);
    resolved_sudoku_grid
}
