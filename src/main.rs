use colored::Colorize;
use std::collections::HashSet;
use std::fmt;

fn num_to_superscript(number: usize) -> &'static str {
    match number {
        1 => "\u{00B9}",
        2 => "\u{00B2}",
        3 => "\u{00B3}",
        4 => "\u{2074}",
        5 => "\u{2075}",
        6 => "\u{2076}",
        7 => "\u{2077}",
        8 => "\u{2078}",
        9 => "\u{2079}",
        _ => "",
    }
}

// fn cell_above(index: i32) -> Option<i32> {
//     let row = index / 9;
//     if row == 0 {
//         return None;
//     }
//     let col = index % 9;
//     let value = ((row - 1) * 9) + col;
//     return Some(value);
// }

// fn cell_below(index: i32) -> Option<i32> {
//     let row = index / 9; // 80/9 = 8
//     if row == 8 {
//         return None;
//     }
//     let col = index % 9;
//     let value = ((row + 1) * 9) + col;
//     return Some(value);
// }

// fn cell_left(index: i32) -> Option<i32> {
//     let col = index % 9;
//     if col == 0 {
//         return None;
//     }
//     let row = index / 9;
//     let value = (row * 9) + (col - 1);
//     return Some(value);
// }

// fn cell_right(index: i32) -> Option<i32> {
//     let col = index % 9;
//     if col == 8 {
//         return None;
//     }
//     let row = index / 9;
//     let value = (row * 9) + (col + 1);
//     return Some(value);
// }

#[derive(Debug, Clone)]
struct Cell {
    initial: bool,
    value: Option<u8>,
    options: Vec<u8>,
}

impl std::default::Default for Cell {
    fn default() -> Cell {
        Cell::new(0, false)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_some() {
            let mut value = format!("{}", self.value.unwrap());
            if self.initial {
                value = value.bold().to_string();
            } else {
                value = value.dimmed().to_string();
            }
            return write!(f, "{}", value);
        } else {
            let value: String = num_to_superscript(self.options.len()).dimmed().to_string();
            return write!(f, "{}", value);
        }
    }
}

impl Cell {
    pub fn new(input: u8, initial: bool) -> Cell {
        let value = if input == 0 { None } else { Some(input) };
        let options: Vec<u8>;
        if value.is_some() {
            options = vec![];
        } else {
            options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        }
        Cell {
            initial: initial,
            value: value,
            options: options,
        }
    }

    // fn entropy(&self) -> usize {
    //     if self.value.is_some() {
    //         return 0;
    //     }
    //     self.options.len()
    // }
}

#[derive(Debug, Clone)]
struct Board {
    cells: [Cell; 81],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┏━━━━━━━┳━━━━━━━┳━━━━━━━┓\n").unwrap();
        write!(f, "{}\n", self.render_row(0)).unwrap();
        write!(f, "{}\n", self.render_row(1)).unwrap();
        write!(f, "{}\n", self.render_row(2)).unwrap();
        write!(f, "┣━━━━━━━╋━━━━━━━╋━━━━━━━┫\n").unwrap();
        write!(f, "{}\n", self.render_row(3)).unwrap();
        write!(f, "{}\n", self.render_row(4)).unwrap();
        write!(f, "{}\n", self.render_row(5)).unwrap();
        write!(f, "┣━━━━━━━╋━━━━━━━╋━━━━━━━┫\n").unwrap();
        write!(f, "{}\n", self.render_row(6)).unwrap();
        write!(f, "{}\n", self.render_row(7)).unwrap();
        write!(f, "{}\n", self.render_row(8)).unwrap();
        write!(f, "┗━━━━━━━┻━━━━━━━┻━━━━━━━┛").unwrap();
        Ok(())
    }
}

fn index_to_col(index: u8) -> u8 {
    index % 9
}

fn index_to_row(index: u8) -> u8 {
    index / 9
}

fn index_to_box(index: u8) -> u8 {
    let col = index_to_col(index);
    let row = index_to_row(index);
    let result = ((row / 3) * 3) + (col / 3);
    // println!("index {} with result {}. (row {}, col {})", index, result, row, col);
    result
}

impl Board {
    fn new(cells: [Cell; 81]) -> Board {
        Board { cells: cells }
    }

    pub fn collapse_cell(&mut self, index: u8) {
        let cell = &self.cells[index as usize];
        if cell.value.is_some() {
            return;
        }

        let mut existing_values: Vec<u8> = vec![];
        // Get the cells in the column
        let col = index_to_col(index);
        let col_values: Vec<u8> = self
            .get_col(col)
            .iter()
            .filter(|c| c.value.is_some())
            .map(|c| c.value.unwrap())
            .collect();
        // println!("col_values: {:?}", col_values);
        existing_values = [existing_values, col_values].concat();

        // Get the cells in the row
        let row = index_to_row(index);
        let row_values = self
            .get_row(row)
            .iter()
            .filter(|c| c.value.is_some())
            .map(|c| c.value.unwrap())
            .collect();
        // println!("row_values: {:?}", row_values);
        existing_values = [existing_values, row_values].concat();

        // Get the cells in the box
        let box_ = index_to_box(index);

        let box_values = self
            .get_box(box_ as i32)
            .iter()
            .filter(|c| c.value.is_some())
            .map(|c| c.value.unwrap())
            .collect();
        // println!("box index: {:?}", box_);
        // println!("box_values: {:?}", box_values);
        existing_values = [existing_values, box_values].concat();

        existing_values.sort();
        existing_values.dedup();

        // println!("existing options: {:?}", cell.options);
        // println!("existing values: {:?}", existing_values);

        let s1: HashSet<u8> = cell.options.iter().cloned().collect();
        let s2: HashSet<u8> = existing_values.iter().cloned().collect();
        let result: Vec<u8> = (&s1 - &s2).into_iter().collect();

        // println!("result: {:?}", result);
        // if result.len() == 0 {
        //     println!("{}", self);
        // }

        // let mut new_cell = Cell::new(0);
        if result.len() == 1 {
            self.cells[index as usize].value = Some(*result.get(0).unwrap());
        } else {
            self.cells[index as usize].options = result;
        }

        // self.cells[index] = new_cell;
    }

    pub fn unsolved_cells(&self) -> Vec<u8> {
        let indexes: Vec<u8> = self
            .cells
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c.value.is_none())
            .map(|(i, _)| i as u8)
            .collect();
        indexes
    }

    fn render_row(&self, row_num: usize) -> String {
        format!(
            "┃ {} {} {} ┃ {} {} {} ┃ {} {} {} ┃",
            self.cells[(row_num * 9) + 0],
            self.cells[(row_num * 9) + 1],
            self.cells[(row_num * 9) + 2],
            self.cells[(row_num * 9) + 3],
            self.cells[(row_num * 9) + 4],
            self.cells[(row_num * 9) + 5],
            self.cells[(row_num * 9) + 6],
            self.cells[(row_num * 9) + 7],
            self.cells[(row_num * 9) + 8],
        )
    }

    // index will be 0-8, represeting the box to retieve, 0 starting in the
    // top left and 8 being in the bottom right
    pub fn get_box(&self, box_num: i32) -> [Cell; 9] {
        let row = (box_num / 3) * 3; // 8 / 3 => 2 / 3 => 6
        let col = (box_num * 3) % 9;
        let mut cells: [Cell; 9] = [
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];
        // first row
        for pos in 0..3 {
            let index = (row * 9) + col + pos;
            cells[pos as usize] = self.cells[index as usize].clone();
        }
        // second row
        for pos in 0..3 {
            let index = ((row + 1) * 9) + col + pos;
            cells[pos as usize + 3] = self.cells[index as usize].clone();
        }
        // third row
        for pos in 0..3 {
            let index = ((row + 2) * 9) + col + pos;
            cells[pos as usize + 6] = self.cells[index as usize].clone();
        }
        cells
    }

    // index will be 0-8, representing the 0-indexed column to retrieve,
    // starting from left to right
    pub fn get_col(&self, col_num: u8) -> [Cell; 9] {
        let mut cells: [Cell; 9] = [
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];
        for pos in 0..9 {
            let index = (pos * 9) + col_num;
            cells[pos as usize] = self.cells[index as usize].clone();
        }
        cells
    }

    // index will be 0-8, representing the 0-indexed row to retrieve,
    // starting from the top to bottom
    pub fn get_row(&self, row_num: u8) -> [Cell; 9] {
        let mut cells: [Cell; 9] = [
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];
        for pos in 0..9 {
            let index = (row_num * 9) + pos;
            cells[pos as usize] = self.cells[index as usize].clone();
        }
        cells
    }
}

fn c(value: u8) -> Cell {
    Cell::new(value, false)
}

fn i(value: u8) -> Cell {
    Cell::new(value, true)
}

fn main() {
    // https://sudoku.com/easy/
    let cells: [Cell; 81] = [
        c(0),
        i(4),
        c(0),
        i(6),
        c(0),
        i(2),
        c(0),
        i(3),
        i(1),
        c(0),
        c(0),
        c(0),
        c(0),
        c(0),
        i(1),
        i(6),
        c(0),
        i(9),
        i(6),
        c(0),
        c(0),
        i(5),
        i(4),
        c(0),
        i(8),
        i(2),
        i(7),
        c(0),
        c(0),
        i(2),
        i(7),
        i(6),
        c(0),
        c(0),
        i(8),
        c(0),
        i(5),
        c(0),
        i(6),
        c(0),
        c(0),
        c(0),
        c(0),
        i(7),
        i(4),
        c(0),
        i(8),
        i(7),
        c(0),
        c(0),
        i(5),
        c(0),
        i(6),
        i(2),
        i(1),
        i(6),
        c(0),
        c(0),
        i(8),
        c(0),
        c(0),
        i(5),
        c(0),
        i(8),
        i(2),
        c(0),
        c(0),
        c(0),
        i(7),
        c(0),
        i(9),
        c(0),
        i(7),
        c(0),
        c(0),
        c(0),
        c(0),
        i(6),
        i(2),
        c(0),
        c(0),
    ];

    let mut b = Board::new(cells);
    println!("Starting 🤖\n{}", b);

    for _i in 1..=10 {
        let num_unsolved = b.unsolved_cells().len();
        if num_unsolved == 0 {
            println!("Solved! 💪\n{}", b);
            break;
        }
        // println!("Starting itteration {} ({} unsolved):\n{}", i, num_unsolved, b);

        // Do one attempt at each cell starting from top-left to bottom-right
        for index in b.unsolved_cells() {
            b.collapse_cell(index);
        }
    }
}
