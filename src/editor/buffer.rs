use std::ops::Range;

pub struct Buffer {
    pub text: String,
    pub undo_stack: Vec<EditOperation>,
    pub redo_stack: Vec<EditOperation>,
}

pub struct EditOperation {
    pub start: usize,
    pub end: usize,
    pub new_text: String,
    pub old_text: String,
}

impl Buffer {
    pub fn insert(&mut self, pos: usize, ch: char) {
        let new_text = ch.to_string();

        self.text.insert(pos, ch);

        let op = EditOperation {
            start: pos,
            end: pos + ch.len_utf8(),
            old_text: String::new(),
            new_text: new_text.clone(),
        };

        self.undo_stack.push(op);
        self.redo_stack.clear();
    }

    pub fn insert_str(&mut self, pos: usize, s: &str) {
        self.text.insert_str(pos, s);

        let op = EditOperation {
            start: pos,
            end: pos + s.len(),
            old_text: String::new(),
            new_text: s.to_string(),
        };

        self.undo_stack.push(op);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            self.text.replace_range(op.start..op.end, "");

            self.text.insert_str(op.start, &op.old_text);

            let reverse_op = EditOperation {
                start: op.start,
                end: op.start + op.old_text.len(),
                old_text: op.new_text,
                new_text: op.old_text,
            };

            self.redo_stack.push(reverse_op);
        }
    }

    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            self.text.replace_range(op.start..op.end, "");

            self.text.insert_str(op.start, &op.new_text);

            let reverse_op = EditOperation {
                start: op.start,
                end: op.start + op.new_text.len(),
                old_text: op.old_text,
                new_text: op.new_text.clone(),
            };

            self.undo_stack.push(reverse_op);
        }
    }

    pub fn get_line(&self, n: usize) -> Option<String> {
        self.text.lines().nth(n).map(|line| line.to_string())
    }

    pub fn line_count(&self) -> usize {
        self.text.lines().count()
    }

    pub fn delete_range(&mut self, start: usize, end: usize) {
        let old = self.text[start..end].to_string();

        self.text.replace_range(start..end, "");

        let op = EditOperation {
            start,
            end,
            old_text: old,
            new_text: String::new(),
        };

        self.undo_stack.push(op);
        self.redo_stack.clear();
    }

    pub fn as_lines(&self) -> Vec<String> {
        self.text.lines().map(|l| l.to_string()).collect()
    }
}

