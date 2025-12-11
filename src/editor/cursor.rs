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
}
