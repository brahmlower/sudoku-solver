use std::fmt;


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct BoardIndex {
    pub value: u8
}

impl fmt::Debug for BoardIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for BoardIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BoardIndex {
    pub fn new(value: u8) -> Option<BoardIndex> {
        if value <= 80 {
            return Some(BoardIndex {value});
        } else {
            return None;
        }
    }
}

impl BoardLocation for BoardIndex {
    fn board_index(&self) -> &BoardIndex {
        self
    }
}

pub trait BoardLocation {
    fn board_index(&self) -> &BoardIndex;

    // Returns the row number for the index
    fn board_row(&self) -> u8 {
        self.board_index().value / 9
    }

    // Returns the column number for the index
    fn board_col(&self) -> u8 {
        self.board_index().value % 9
    }

    // Returns the box number for the index
    fn board_box(&self) -> u8 {
        let row = self.board_row();
        let col = self.board_col();
        ((row / 3) * 3) + (col / 3)
    }

    fn index_above(&self) -> Option<BoardIndex> {
        let index = self.board_index().value;
        let row = index / 9;
        if row == 0 {
            return None;
        }
        let col = index % 9;
        let value = ((row - 1) * 9) + col;
        return Some(
            BoardIndex::new(value).unwrap()
        );
    }

    fn index_below(&self) -> Option<BoardIndex> {
        let index = self.board_index().value;
        let row = index / 9;
        if row == 8 {
            return None;
        }
        let col = index % 9;
        let value = ((row + 1) * 9) + col;
        return Some(
            BoardIndex::new(value).unwrap()
        );
    }

    fn index_left(&self) -> Option<BoardIndex> {
        let index = self.board_index().value;
        let col = index % 9;
        if col == 0 {
            return None;
        }
        let row = index / 9;
        let value = (row * 9) + (col - 1);
        return Some(
            BoardIndex::new(value).unwrap()
        );
    }

    fn index_right(&self) -> Option<BoardIndex> {
        let index = self.board_index().value;
        let col = index % 9;
        if col == 8 {
            return None;
        }
        let row = index / 9;
        let value = (row * 9) + (col + 1);
        return Some(
            BoardIndex::new(value).unwrap()
        );
    }

    fn entangled_row_indexes(&self) -> Vec<BoardIndex> {
        let index = self.board_index().value;
        let start = self.board_row();
        ((start*9)..(start*9+9))
            .filter(|i| *i != index)
            .map(|i| BoardIndex::new(i).unwrap())
            .collect()
    }

    fn entangled_col_indexes(&self) -> Vec<BoardIndex> {
        let index = self.board_index().value;
        let start = self.board_col();
        (start..81)
            .step_by(9)
            .filter(|i| *i != index)
            .map(|i| BoardIndex::new(i).unwrap())
            .collect()
    }

    fn entangled_box_indexes(&self) -> Vec<BoardIndex> {
        let index = self.board_index().value;
        let box_index = self.board_box();
        let col_start = box_index % 3 * 3;
        let row_start = box_index / 3 * 3;
        let range1_start = (row_start + 0) * 9 + col_start;
        let range2_start = (row_start + 1) * 9 + col_start;
        let range3_start = (row_start + 2) * 9 + col_start;
        let range1 = range1_start..(range1_start + 3);
        let range2 = range2_start..(range2_start + 3);
        let range3 = range3_start..(range3_start + 3);
        [
            range1.collect::<Vec<u8>>(),
            range2.collect::<Vec<u8>>(),
            range3.collect::<Vec<u8>>(),
        ].concat()
            .into_iter()
            .filter(|i| *i != index)
            .map(|i| BoardIndex::new(i).unwrap())
            .collect()
    }

    fn entangled_indexes(&self) -> Vec<BoardIndex> {
        let mut entangled_indexes = [
            self.entangled_box_indexes(),
            self.entangled_col_indexes(),
            self.entangled_row_indexes(),
        ].concat();
        entangled_indexes.sort();
        entangled_indexes.dedup();
        entangled_indexes
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_entangled_row_indexes_at_row_0() {
        let index = BoardIndex::new(5).unwrap();
        assert_eq!(index.board_row(), 0);
        let row_indexes = index.entangled_row_indexes();
        let expected_row_indexes: Vec<BoardIndex> = vec![0, 1, 2, 3, 4, 6, 7, 8]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(row_indexes, expected_row_indexes);
    }

    #[test]
    fn test_entangled_row_indexes_at_row_1() {
        let index = BoardIndex::new(10).unwrap();
        assert_eq!(index.board_row(), 1);
        let row_indexes = index.entangled_row_indexes();
        let expected_row_indexes: Vec<BoardIndex> = vec![9, 11, 12, 13, 14, 15, 16, 17]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(row_indexes, expected_row_indexes);
    }

    #[test]
    fn test_entangled_row_indexes_at_row_7() {
        let index = BoardIndex::new(66).unwrap();
        assert_eq!(index.board_row(), 7);
        let row_indexes = index.entangled_row_indexes();
        let expected_row_indexes: Vec<BoardIndex> = vec![63, 64, 65, 67, 68, 69, 70, 71]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(row_indexes, expected_row_indexes);
    }

    #[test]
    fn test_entangled_col_indexes_at_col_0() {
        let index = BoardIndex::new(0).unwrap();
        assert_eq!(index.board_col(), 0);
        let col_indexes = index.entangled_col_indexes();
        let expected_col_indexes: Vec<BoardIndex> = vec![9, 18, 27, 36, 45, 54, 63, 72]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_col_indexes_at_col_8() {
        let index = BoardIndex::new(17).unwrap();
        assert_eq!(index.board_col(), 8);
        let col_indexes = index.entangled_col_indexes();
        let expected_col_indexes: Vec<BoardIndex> = vec![8, 26, 35, 44, 53, 62, 71, 80]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_box_indexes_at_box_0() {
        let index = BoardIndex::new(10).unwrap();
        assert_eq!(index.board_box(), 0);
        let col_indexes = index.entangled_box_indexes();
        let expected_col_indexes: Vec<BoardIndex> = vec![0, 1, 2, 9, 11, 18, 19, 20]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_box_indexes_at_box_2() {
        let index = BoardIndex::new(8).unwrap();
        assert_eq!(index.board_box(), 2);
        let col_indexes = index.entangled_box_indexes();
        let expected_col_indexes: Vec<BoardIndex> = vec![6, 7, 15, 16, 17, 24, 25, 26]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_box_indexes_at_box_7() {
        let index = BoardIndex::new(57).unwrap();
        assert_eq!(index.board_box(), 7);
        let col_indexes = index.entangled_box_indexes();
        let expected_col_indexes: Vec<BoardIndex> = vec![58, 59, 66, 67, 68, 75, 76, 77]
            .iter()
            .map(|i| BoardIndex::new(*i).unwrap())
            .collect();
        assert_eq!(col_indexes, expected_col_indexes);
    }
}
