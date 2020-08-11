# sudoku-solver-rs

## Naive brute-force rust program to resolve sudoku.

## Algo steps :

1) Parse a text file and build sudoku grid data structure. Each empty cell has a list of candidates values.
2) Read the sudoku grid by lines and columns and remove impossible candidates for each empty cell. If candidate list size is one, we found the value of a cell.
3) Read the sudoku grid by 3x3 boxes and remove impossible candidates for each empty cell of boxes. If candidate list size is one, we found the value of a cell.
4) Brute-force with backtracking naive algo the remaining empty cells
