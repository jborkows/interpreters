use crate::lexers::{ColumnNumber, LineNumber, SourceCharecter};

pub struct Lines {
    lines: Vec<String>,
    current_line: LineNumber,
    current_column: ColumnNumber,
}

impl Lines {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            current_line: LineNumber(0),
            current_column: ColumnNumber(0),
        }
    }
}

impl Iterator for Lines {
    type Item = SourceCharecter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line.0 >= self.lines.len() as u16 {
            return None;
        }
        let line = &self.lines[self.current_line.0 as usize];
        if self.current_column.0 >= line.len() as u16 {
            self.current_line.0 += 1;
            self.current_column.0 = 0;
            return self.next();
        }
        let ch = line.chars().nth((self.current_column.0) as usize).unwrap();
        let charecter = SourceCharecter::new(ch, self.current_column + 1, self.current_line + 1);
        self.current_column.0 += 1;
        Some(charecter)
    }
}
