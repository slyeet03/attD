use super::buffer::Buffer;

pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn move_left(&mut self, buffer: &Buffer) {
        if self.col > 0 {
            self.col -= 1;
        } else if self.col == 0 {
            if self.row != 0 {
                self.row -= 1;
                self.col = buffer
                    .get_line(self.row)
                    .map(|line| line.len())
                    .unwrap_or(0);
            }
        }
    }

    pub fn move_right(&mut self, buffer: &Buffer) {
        let line_len = buffer
            .get_line(self.row)
            .map(|line| line.len())
            .unwrap_or(0);

        if self.col < line_len {
            self.col += 1;
        } else if self.col == line_len {
            if self.row != buffer.line_count() {
                self.row += 1;
                self.col = 0;
            }
        }
    }

    pub fn move_up(&mut self, buffer: &Buffer) {
        if self.row != 0 {
            self.row -= 1;
            let line_len = buffer
                .get_line(self.row)
                .map(|line| line.len())
                .unwrap_or(0);

            if self.col > line_len {
                self.col = line_len;
            }
        }
    }

    pub fn move_down(&mut self, buffer: &Buffer) {
        if self.row != buffer.line_count() {
            self.row += 1;
            let line_len = buffer
                .get_line(self.row)
                .map(|line| line.len())
                .unwrap_or(0);

            if self.col > line_len {
                self.col = line_len;
            }
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.col = 0;
    }

    pub fn move_to_end(&mut self, buffer: &Buffer) {
        let line_len = buffer
            .get_line(self.row)
            .map(|line| line.len())
            .unwrap_or(0);
        self.col = line_len;
    }

    pub fn to_offset(&self, row: usize, col: usize, buffer: &Buffer) -> usize {
        let mut current_row = 0;
        let mut offset = 0;

        for line in buffer.text.split_inclusive('\n') {
            if current_row == row {
                return offset + col.min(line.trim_end_matches('\n').len());
            }
            offset += line.len();
            current_row += 1;
        }

        offset
    }

    pub fn from_offset(&self, offset: usize, buffer: &Buffer) -> (usize, usize) {
        let mut remaining = offset;
        let mut row = 0;

        for line in buffer.text.split_inclusive('\n') {
            if remaining <= line.len() {
                let clean = line.trim_end_matches('\n');
                return (row, remaining.min(clean.len()));
            }

            remaining -= line.len();
            row += 1;
        }

        (row, 0)
    }
}
