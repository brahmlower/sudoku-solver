use colored::Colorize;
use std::{fmt, collections::HashSet};

use crate::board_index::BoardIndex;
use crate::diff::Diff;
use crate::diff::DiffBuilder;
use crate::diff::PatchFragment;
use crate::diff::ScalarDiffFragment;

pub struct CellFragmentBuilder {
    value: Option<[Option<u8>; 2]>,
    options: Option<[Vec<u8>; 2]>,
}

impl CellFragmentBuilder {
    pub fn new() -> CellFragmentBuilder {
        CellFragmentBuilder::default()
    }

    pub fn changed_value(&mut self, old: Option<u8>, new: Option<u8>) -> &mut CellFragmentBuilder {
        self.value = Some([old, new]);
        self
    }

    pub fn removed_options(&mut self, removed: Vec<u8>) -> &mut CellFragmentBuilder {
        if self.options.is_some() {
            self.options.as_mut().unwrap()[0] = removed;
        } else {
            self.options = Some([
                removed,
                vec![],
            ])
        }
        self
    }

    pub fn added_options(&mut self, added: Vec<u8>) -> &mut CellFragmentBuilder {
        if self.options.is_some() {
            self.options.as_mut().unwrap()[1] = added;
        } else {
            self.options = Some([
                vec![],
                added,
            ])
        }
        self
    }

    pub fn finalize(&mut self) -> CellFragment {
        let s = std::mem::take(self);
        CellFragment::new(s.value, s.options)
    }
}

impl Default for CellFragmentBuilder {
    fn default() -> Self {
        CellFragmentBuilder{
            value: None,
            options: None,
        }
    }
}

#[derive(Debug)]
pub struct CellFragment {
    value: Option<[Option<u8>; 2]>,
    options: Option<[Vec<u8>; 2]>,
}

impl CellFragment {
    pub fn new(
        value: Option<[Option<u8>; 2]>,
        options: Option<[Vec<u8>; 2]>
    ) -> CellFragment {
        CellFragment { value, options }
    }

    pub fn builder() -> CellFragmentBuilder {
        CellFragmentBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub index: BoardIndex,
    pub initial: bool,
    pub value: Option<u8>,
    pub options: Vec<u8>,
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

    pub fn diff_builder() -> DiffBuilder<CellFragment> {
        Diff::builder()
    }

    // fn entropy(&self) -> usize {
    //     if self.value.is_some() {
    //         return 0;
    //     }
    //     self.options.len()
    // }
}

// https://stackoverflow.com/a/63557337
fn subtract<T: std::cmp::PartialEq>(a: &mut Vec<T>, b: &Vec<T>) {
    a.retain(|x| !b.contains(x));
}

impl PatchFragment for Cell {
    type Fragment = CellFragment;

    fn apply_fragment(&mut self, fragment: &CellFragment) {
        // Apply value changes
        if fragment.value.is_some() {
            let [_, new] = fragment.value.unwrap();
            self.value = new;
        }

        // Apply options changes
        if fragment.options.is_some() {
            let [removed, added] = fragment.options.as_ref().unwrap();

            // Handle removed options
            subtract(&mut self.options, removed);

            // Handle added options
            self.options.extend(added)
        }
    }

    fn revert_fragment(&mut self, fragment: &CellFragment) {
        // Revert fragment value
        if fragment.value.is_some() {
            let [old, _] = fragment.value.unwrap();
            self.value = old;
        }

        // Revert fragment options
        if fragment.options.is_some() {
            let [removed, added] = fragment.options.as_ref().unwrap();

            // Handle removed options
            self.options.extend(removed);

            // Handle added options
            subtract(&mut self.options, added);
        }
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
            let value: String = num_to_superscript(self.options.len())
                .dimmed()
                .to_string();
            return write!(f, "{}", value);
        }
    }
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