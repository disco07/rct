use std::io::{Stdout, Write};
use crate::row::Row;

// Macro for writing to the giving writer.
// Used in both pb.rs and multi.rs modules.
//
// # Examples
//
// ```
// let w = io::stdout();
// printfl!(w, "");
// printfl!(w, "\r{}", out);
//
// ```
macro_rules! printfl {
   ($w:expr, $($tt:tt)*) => {{
        $w.write_all(&format!($($tt)*).as_bytes()).ok().expect("write() fail");
        $w.flush().ok().expect("flush() fail");
    }}
}

#[derive(Debug, Clone)]
pub struct Table<T: Write> {
    header: Option<Row>,
    rows: Vec<Row>,
    border: Border,
    handle: T,
}

#[derive(Debug, Clone)]
pub struct Border {
    top: char,
    top_mid: char,
    top_left: char,
    top_right: char,
    bottom: char,
    bottom_mid: char,
    bottom_left: char,
    bottom_right: char,
    left: char,
    left_mid: char,
    middle: char,
    right: char,
    right_mid: char,
    mid: char,
    mid_mid: char,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: '═',
            top_mid: '╤',
            top_left: '╔',
            top_right: '╗',
            bottom: '═',
            bottom_mid: '╧',
            bottom_left: '╚',
            bottom_right: '╝',
            left: '║',
            left_mid: '╟',
            middle: '│',
            right: '║',
            right_mid: '╢',
            mid: '─',
            mid_mid: '┼',
        }
    }
}

impl Table<Stdout> {
    pub fn new() -> Table<Stdout> {
        let handle = std::io::stdout();
        Table::on(handle)
    }
}

impl<T: Write> Table<T> {
    pub fn on(handle: T) -> Table<T> {
        Self {
            header: None,
            rows: vec![],
            border: Default::default(),
            handle,
        }
    }

    pub fn add_row<R: Into<Row>>(&mut self, row: R) -> &mut Table<T> {
        let row = row.into();
        self.rows.push(row);

        self
    }

    pub fn add_header<R: Into<Row>>(&mut self, row: R) -> &mut Table<T> {
        let row = row.into();
        self.header = Some(row);

        self
    }

    fn set_max_width(&mut self) -> Vec<usize> {
        let mut column_len: Vec<usize> = vec![0; self.rows.len()];
        if let Some(header) = &self.header {
            max_column_length(&mut column_len, header);
        }
        for row in self.rows.iter() {
            max_column_length(&mut column_len, row);
        }

        column_len
    }

    fn print_header(&mut self, column_len: &Vec<usize>) {
        let mut view = self.border.top_left.to_string();
        let len: Vec<String> = column_len.iter().map(|col| self.border.top.to_string().repeat(*col + 2)).collect();
        view += len.join(self.border.top_mid.to_string().as_str()).as_str();
        view += self.border.top_right.to_string().as_str();

        printfl!(self.handle, "\r{}", view);
    }

    fn print_lines(&mut self) {
        let mut content = vec![];
        let width_column = self.set_max_width();
        for row in self.rows.iter() {
            content.push(self.print_line(row, width_column.clone()));
        }
        print!("{:?}", content);
    }
    
    fn print_line(&self, row: &Row, width_column: Vec<usize>) -> Vec<Vec<String>> {
        let mut content: Vec<Vec<String>> = vec![];

        for cell in row.cells.iter() {
            let mut cell_content = vec![];
            for data in cell.data.iter() {
                let mut value = String::from("");
                value += &" ".repeat(1);
                value += data;
                value += &" ".repeat(cell.width);
                cell_content.push(value);
            }
            content.push(cell_content);
        }

        let max_column = content
            .iter()
            .map(|c|c.len())
            .max()
            .unwrap_or(0);

        let mut row_content = Vec::new();

        for i in 0..max_column {
            let mut line = Vec::new();
            for (index, cell) in content.iter().enumerate() {
                match cell.get(i) {
                    Some(value) => line.push(value.to_string()),
                    None => line.push(" ".repeat(*width_column.get(index).unwrap_or(&(0 as usize)))),
                }
            }
            row_content.push(line);
        }
        // print!("{:?}", width_column);
        row_content
    }

    /// Display the table on terminal.
    pub fn view(&mut self) {
        self.print_lines()
    }
}

fn max_column_length(column_len: &mut Vec<usize>, row: &Row) {
    let rows: Vec<_> = row.width();

    for row in rows.iter().enumerate() {
        let current_max = column_len.get(row.0).unwrap();
        if *row.1 > *current_max {
            column_len[row.0] = *row.1;
        }
    }
}