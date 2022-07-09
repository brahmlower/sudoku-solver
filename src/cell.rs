use colored::Colorize;
use std::fmt;

use crate::board_index::BoardIndex;

#[derive(Debug, Clone)]
pub struct Cell {
    pub index: BoardIndex,
    pub initial: bool,
    pub value: Option<u8>,
    pub options: Vec<u8>,
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
            let value: String = num_to_superscript(self.options.len())
                .dimmed()
                .to_string();
            return write!(f, "{}", value);
        }
    }
}

impl Cell {
    pub fn new(index: BoardIndex, input: u8, is_initial: bool) -> Cell {
        let value = if input == 0 { None } else { Some(input) };
        let options: Vec<u8>;
        if value.is_some() {
            options = vec![];
        } else {
            options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        }
        Cell {
            index: index,
            initial: is_initial,
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