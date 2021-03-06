use std::collections::HashSet;
use std::fmt;

mod board_index;
mod cell;
mod diff;

use board_index::BoardLocation;
use board_index::BoardIndex;
use cell::Cell;

use crate::cell::CellFragment;
use crate::diff::Diff;
use crate::diff::PatchDiff;

#[derive(Debug, Clone)]
struct Board {
    cells: [Cell; 81],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

fn cells_to_values(cells: Vec<&Cell>) -> Vec<u8> {
    cells.into_iter()
        .filter(|v| v.value.is_some())
        .map(|v| v.value.clone().unwrap())
        .collect()
}

impl Board {
    fn new(cells: [Cell; 81]) -> Board {
        Board { cells: cells }
    }

    fn render(&self) -> String {
        format!(
            "┏━━━━━━━┳━━━━━━━┳━━━━━━━┓\n{}\n{}\n{}\n┣━━━━━━━╋━━━━━━━╋━━━━━━━┫\n{}\n{}\n{}\n┣━━━━━━━╋━━━━━━━╋━━━━━━━┫\n{}\n{}\n{}\n┗━━━━━━━┻━━━━━━━┻━━━━━━━┛",
            self.render_row(0),
            self.render_row(1),
            self.render_row(2),
            self.render_row(3),
            self.render_row(4),
            self.render_row(5),
            self.render_row(6),
            self.render_row(7),
            self.render_row(8),
        )
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

    fn get_cells(&self, indexes: &Vec<BoardIndex>) -> Vec<&Cell> {
        indexes
            .iter()
            .map(|bi| &self.cells[bi.value as usize])
            .collect()
    }

    pub fn get_cell(&self, index: &BoardIndex) -> &Cell {
        &self.cells[index.value as usize]
    }

    pub fn get_cell_mut(&mut self, index: &BoardIndex) -> &mut Cell {
        &mut self.cells[index.value as usize]
    }

    pub fn collapse_cell(&mut self, index: &BoardIndex) {
        let cell = self.get_cell(index);
        if cell.value.is_some() {
            return;
        }

        let entangled_indexes = index.entangled_indexes();

        let entangled_cells = self.get_cells(&entangled_indexes);

        let mut existing_values = cells_to_values(entangled_cells);

        existing_values.sort();
        existing_values.dedup();

        let s1: HashSet<u8> = cell.options.iter().cloned().collect();
        let s2: HashSet<u8> = existing_values.iter().cloned().collect();
        let result: Vec<u8> = (&s1 - &s2).into_iter().collect();

        if result.len() == 0 {
            println!("Comparing s1 to s2: {:?}, {:?}", s1, s2);
            panic!("Diff between available options and possible options was 0");
        }

        if result.len() == 1 {
            let collapsed_value = *result.first().unwrap();
            self.get_cell_mut(index).value = Some(collapsed_value);
            for ebi in &entangled_indexes {
                let cell = self.get_cell_mut(&ebi);
                cell.options.retain(|i| *i != collapsed_value);
            }
        } else {
            self.get_cell_mut(index).options = result;
        }
    }

    pub fn unsolved_indexes(&self) -> Vec<BoardIndex> {
        let indexes: Vec<BoardIndex> = self
            .cells
            .clone()
            .into_iter()
            .filter(|c| c.value.is_none())
            .map(|c| c.index)
            .collect();
        indexes
    }
}

fn c(index: u8, value: u8) -> Cell {
    let bi = BoardIndex::new(index).unwrap();
    Cell::new(bi, value, false)
}

fn i(index: u8, value: u8) -> Cell {
    let bi = BoardIndex::new(index).unwrap();
    Cell::new(bi, value, true)
}

fn main() {
    // https://sudoku.com/easy/
    // let cells: [Cell; 81] = [
    //     c(0, 0),
    //     i(1, 4),
    //     c(2, 0),
    //     i(3, 6),
    //     c(4, 0),
    //     i(5, 2),
    //     c(6, 0),
    //     i(7, 3),
    //     i(8, 1),
    //     c(9, 0),
    //     c(10, 0),
    //     c(11, 0),
    //     c(12, 0),
    //     c(13, 0),
    //     i(14, 1),
    //     i(15, 6),
    //     c(16, 0),
    //     i(17, 9),
    //     i(18, 6),
    //     c(19, 0),
    //     c(20, 0),
    //     i(21, 5),
    //     i(22, 4),
    //     c(23, 0),
    //     i(24, 8),
    //     i(25, 2),
    //     i(26, 7),
    //     c(27, 0),
    //     c(28, 0),
    //     i(29, 2),
    //     i(30, 7),
    //     i(31, 6),
    //     c(32, 0),
    //     c(33, 0),
    //     i(34, 8),
    //     c(35, 0),
    //     i(36, 5),
    //     c(37, 0),
    //     i(38, 6),
    //     c(39, 0),
    //     c(40, 0),
    //     c(41, 0),
    //     c(42, 0),
    //     i(43, 7),
    //     i(44, 4),
    //     c(45, 0),
    //     i(46, 8),
    //     i(47, 7),
    //     c(48, 0),
    //     c(49, 0),
    //     i(50, 5),
    //     c(51, 0),
    //     i(52, 6),
    //     i(53, 2),
    //     i(54, 1),
    //     i(55, 6),
    //     c(56, 0),
    //     c(57, 0),
    //     i(58, 8),
    //     c(59, 0),
    //     c(60, 0),
    //     i(61, 5),
    //     c(62, 0),
    //     i(63, 8),
    //     i(64, 2),
    //     c(65, 0),
    //     c(66, 0),
    //     c(67, 0),
    //     i(68, 7),
    //     c(69, 0),
    //     i(70, 9),
    //     c(71, 0),
    //     i(72, 7),
    //     c(73, 0),
    //     c(74, 0),
    //     c(75, 0),
    //     c(76, 0),
    //     i(77, 6),
    //     i(78, 2),
    //     c(79, 0),
    //     c(80, 0),
    // ];

    // let mut b = Board::new(cells);
    // println!("Starting 🤖\n{}", b);

    // for i in 1..=10 {
    //     let num_unsolved = b.unsolved_indexes().len();
    //     if num_unsolved == 0 {
    //         println!("Solved! 💪\n{}", b);
    //         break;
    //     }
    //     println!("Starting itteration {} ({} unsolved):\n{}", i, num_unsolved, b);

    //     // Do one attempt at each cell starting from top-left to bottom-right
    //     for index in b.unsolved_indexes() {
    //         b.collapse_cell(&index);
    //     }
    // }
    let index = BoardIndex::new(0).unwrap();
    let mut cell = Cell::new(index, 0, true);
    println!("Cell is: {:?}", cell);

    let diff = Cell::diff_builder()
        .add_fragment(||
            CellFragment::builder()
                .removed_options(vec![1, 2, 3])
                .finalize()
        )
        .finalize();
    println!("CellDiff is: {:?}", diff);


    cell.apply_diff(&diff);
    println!("Cell is now: {:?}", cell);

    cell.revert_diff(&diff);
    println!("Cell is now: {:?}", cell);

}
