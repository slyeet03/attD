use super::cursor::Cursor;

pub struct Selection {
    pub anchor: Anchor,
    pub cursor: Cursor,
}

pub struct Anchor {
    pub row: usize,
    pub col: usize,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.anchor.row == self.cursor.row && self.anchor.col == self.cursor.col
    }

    pub fn clear(&mut self) {
        self.anchor.row = self.cursor.row;
        self.anchor.col = self.cursor.col;
    }

    pub fn expand_to(&mut self, row: usize, col: usize) {
        self.cursor.row = row;
        self.cursor.col = col;
    }

    pub fn collapse_to(&mut self, row: usize, col: usize) {
        self.expand_to(row, col);
        self.clear();
    }

    pub fn range(&self) -> (usize, usize, usize, usize) {
        let start_row: usize;
        let start_col: usize;
        let end_row: usize;
        let end_col: usize;

        if self.anchor.row < self.cursor.row {
            // Anchor is above cursor
            start_row = self.anchor.row;
            start_col = self.anchor.col;
            end_row = self.cursor.row;
            end_col = self.cursor.col;
        } else if self.anchor.row > self.cursor.row {
            end_row = self.anchor.row;
            end_col = self.anchor.col;
            start_row = self.cursor.row;
            start_col = self.cursor.col;
        } else {
            if self.anchor.col <= self.cursor.col {
                start_row = self.anchor.row;
                start_col = self.anchor.col;
                end_row = self.cursor.row;
                end_col = self.cursor.col;
            } else {
                end_row = self.anchor.row;
                end_col = self.anchor.col;
                start_row = self.cursor.row;
                start_col = self.cursor.col;
            }
        }

        (start_row, start_col, end_row, end_col)
    }
}

