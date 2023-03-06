use crate::row::Row;
use std::cmp::min;
use std::io::{Stdout, Write};

/// Macro for writing to the giving writer.
/// Used in both pb.rs and multi.rs modules.
///
/// # Examples
///
/// ```
/// use std::io;
/// let w = io::stdout();
/// printfl!(w, "");
/// printfl!(w, "\r{}", out);
///
/// ```
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

impl Default for Table<Stdout> {
    fn default() -> Self {
        Self::new()
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
        let mut column_len: Vec<usize> = vec![0; self.rows[0].cells.len()];
        if let Some(header) = &self.header {
            max_column_length(&mut column_len, header);
        }
        for row in self.rows.iter() {
            max_column_length(&mut column_len, row);
        }

        column_len
    }

    fn print_header(&mut self, column_len: &[usize]) {
        let mut view = self.border.top_left.to_string();
        let len: Vec<String> = column_len
            .iter()
            .map(|col| self.border.top.to_string().repeat(*col))
            .collect();
        view += &len.join(&self.border.top_mid.to_string());
        view += &self.border.top_right.to_string();

        printfl!(self.handle, "{}\n", view);
    }

    fn print_bottom(&mut self, column_len: &[usize]) {
        let mut table: String = String::new();
        let mut count = 0;

        table += &self.border.bottom_left.to_string();
        for len in column_len.iter() {
            count += 1;
            table += &self.border.bottom.to_string().repeat(*len);
            if column_len.len() > count {
                table += &self.border.bottom_mid.to_string();
            }
        }
        table += &self.border.bottom_right.to_string();

        printfl!(self.handle, "\n{}", table);
    }

    fn print_table_middle(&mut self, column_len: &[usize]) -> String {
        let mut table: String = String::new();
        let mut count = 0;
        table += &self.border.left_mid.to_string();
        for len in column_len.iter() {
            count += 1;
            table += &self.border.mid.to_string().repeat(*len);
            if column_len.len() > count {
                table += &self.border.mid_mid.to_string();
            }
        }
        table += &self.border.right_mid.to_string();

        table
    }

    fn print_lines(&mut self) {
        let mut contents = vec![];
        let width_column = self.set_max_width();

        match self.header.as_ref() {
            Some(header) => {
                contents.push(self.print_line(header, width_column.clone()));
            }
            None => (),
        }
        for row in self.rows.iter() {
            contents.push(self.print_line(row, width_column.clone()));
        }

        self.print_header(&width_column);
        for (index, content) in contents.iter().enumerate() {
            self.draw(content, &width_column, (contents.len() - 1) == index);
        }
        self.print_bottom(&width_column);
    }

    fn print_line(&self, row: &Row, width_column: Vec<usize>) -> Vec<Vec<String>> {
        let content = row
            .cells
            .iter()
            .map(|cell| {
                cell.data.iter().fold(vec![], |mut acc, data| {
                    let mut value = String::from("");
                    value += " ";
                    value += data;
                    value += " ";
                    acc.push(value);
                    acc
                })
            })
            .collect::<Vec<_>>();

        let max_column = content.iter().map(|c| c.len()).max().unwrap_or(0);

        (0..max_column)
            .into_iter()
            .flat_map(|i| {
                let mut line = Vec::new();
                let width_column = width_column.clone();
                let size = content.len();
                content
                    .iter()
                    .enumerate()
                    .fold(vec![], move |mut acc, (index, cell)| {
                        match cell.get(i) {
                            Some(value) => line.push(
                                value.to_string()
                                    + &" ".repeat(
                                        *width_column.clone().get(index).unwrap_or(&(0_usize))
                                            - min(
                                                *width_column
                                                    .clone()
                                                    .get(index)
                                                    .unwrap_or(&(0_usize)),
                                                value.chars().count(),
                                            ),
                                    ),
                            ),
                            None => line.push(" ".repeat(
                                *width_column.clone().get(index).unwrap_or(&(0_usize)) + 1,
                            )),
                        }
                        if index + 1 == size {
                            acc.push(line.clone());
                        }
                        acc
                    })
            })
            .collect::<Vec<_>>()
    }

    fn draw(&mut self, rows: &Vec<Vec<String>>, width_column: &Vec<usize>, last_row: bool) {
        let mut lines = vec![];
        for (i, row) in rows.iter().enumerate() {
            let mut view = String::new();
            for (index, line) in row.iter().enumerate() {
                if index == 0 {
                    view += &self.border.left.to_string();
                } else {
                    view += &self.border.middle.to_string();
                }
                view += line;
            }
            view += &self.border.right.to_string();
            lines.push(view);
            if rows.len() - 1 == i && !last_row {
                lines.push(self.print_table_middle(&width_column));
            }
        }

        printfl!(self.handle, "{}", lines.join("\n"));
        if !last_row {
            println!("\r")
        }
    }

    /// Display the table on terminal.
    pub fn view(&mut self) {
        self.print_lines()
    }
}

fn max_column_length(column_len: &mut [usize], row: &Row) {
    let rows: Vec<_> = row.width();

    for (index, row) in rows.iter().enumerate() {
        let current_max = column_len.get(index).unwrap_or(&0);
        if *row > *current_max {
            column_len[index] = *row + 2;
        }
    }
}
