
pub trait BoardIndex {
    fn board_index(&self) -> u8;

    // Returns the row number for the index
    fn board_row(&self) -> u8 {
        self.board_index() / 9
    }

    // Returns the column number for the index
    fn board_col(&self) -> u8 {
        self.board_index() % 9
    }

    // Returns the box number for the index
    fn board_box(&self) -> u8 {
        let row = self.board_row();
        let col = self.board_col();
        ((row / 3) * 3) + (col / 3)
    }

    fn index_above(&self) -> Option<u8> {
        let row = self.board_index() / 9;
        if row == 0 {
            return None;
        }
        let col = self.board_index() % 9;
        let value = ((row - 1) * 9) + col;
        return Some(value);
    }

    fn index_below(&self) -> Option<u8> {
        let row = self.board_index() / 9; // 80/9 = 8
        if row == 8 {
            return None;
        }
        let col = self.board_index() % 9;
        let value = ((row + 1) * 9) + col;
        return Some(value);
    }

    fn index_left(&self) -> Option<u8> {
        let col = self.board_index() % 9;
        if col == 0 {
            return None;
        }
        let row = self.board_index() / 9;
        let value = (row * 9) + (col - 1);
        return Some(value);
    }

    fn index_right(&self) -> Option<u8> {
        let col = self.board_index() % 9;
        if col == 8 {
            return None;
        }
        let row = self.board_index() / 9;
        let value = (row * 9) + (col + 1);
        return Some(value);
    }

    fn entangled_row_indexes(&self) -> Vec<u8> {
        let index = self.board_index();
        let start = self.board_row();
        (start..(start+9))
            .filter(|i| *i != index)
            .collect()
    }

    fn entangled_col_indexes(&self) -> Vec<u8> {
        let index = self.board_index();
        let start = self.board_col();
        (start..81)
            .step_by(9)
            .filter(|i| *i != index)
            .collect()
    }

    fn entangled_box_indexes(&self) -> Vec<u8> {
        let index = self.board_index();
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
            .collect()
    }

    fn entangled_indexes(&self) -> Vec<u8> {
        [
            self.entangled_box_indexes(),
            self.entangled_col_indexes(),
            self.entangled_row_indexes(),
        ].concat()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct MockCell {
        index: u8
    }

    impl BoardIndex for MockCell {
        fn board_index(&self) -> u8 {
            self.index
        }
    }

    #[test]
    fn test_entangled_col_indexes_at_col_0() {
        let cell = MockCell{ index: 0 };
        assert_eq!(cell.board_col(), 0);
        let col_indexes = cell.entangled_col_indexes();
        let expected_col_indexes: Vec<u8> = vec![9, 18, 27, 36, 45, 54, 63, 72];
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_col_indexes_at_col_8() {
        let cell = MockCell{ index: 17 };
        assert_eq!(cell.board_col(), 8);
        let col_indexes = cell.entangled_col_indexes();
        let expected_col_indexes: Vec<u8> = vec![8, 26, 35, 44, 53, 62, 71, 80];
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_box_indexes_at_box_0() {
        let cell = MockCell{ index: 10 };
        assert_eq!(cell.board_box(), 0);
        let col_indexes = cell.entangled_box_indexes();
        let expected_col_indexes: Vec<u8> = vec![0, 1, 2, 9, 11, 18, 19, 20];
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_box_indexes_at_box_2() {
        let cell = MockCell{ index: 8 };
        assert_eq!(cell.board_box(), 2);
        let col_indexes = cell.entangled_box_indexes();
        let expected_col_indexes: Vec<u8> = vec![6, 7, 15, 16, 17, 24, 25, 26];
        assert_eq!(col_indexes, expected_col_indexes);
    }

    #[test]
    fn test_entangled_box_indexes_at_box_7() {
        let cell = MockCell{ index: 57 };
        assert_eq!(cell.board_box(), 7);
        let col_indexes = cell.entangled_box_indexes();
        let expected_col_indexes: Vec<u8> = vec![58, 59, 66, 67, 68, 75, 76, 77];
        assert_eq!(col_indexes, expected_col_indexes);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
