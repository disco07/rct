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
    /// Create a new table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::table::Table;
    /// let mut table = Table::new();
    /// ```
    ///
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

    /// Add a new row to the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::cell::ICell;
    /// use rct::table::Table;
    /// let mut table = Table::new();
    ///
    /// table
    ///     .add_row(vec![
    ///         1.cell(),
    ///         "Harry Potter".cell(),
    ///         "1".cell(),
    ///         "14.87".cell(),
    ///         "€".cell(),
    ///         "Harry Potter".cell(),
    ///         "2001-12-05 22:05:20".cell(),
    ///     ]);
    /// ```
    ///
    pub fn add_row<R: Into<Row>>(&mut self, row: R) -> &mut Table<T> {
        let row = row.into();
        self.rows.push(row);

        self
    }

    /// Add a header to the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::cell::ICell;
    /// use rct::table::Table;
    /// let mut table = Table::new();
    ///
    /// table
    ///     .add_header(vec![
    ///         "ID".cell(),
    ///         "Title".cell(),
    ///         "is_enabled".cell(),
    ///         "price".cell(),
    ///         "currency".cell(),
    ///         "description".cell(),
    ///         "created_at".cell(),
    ///     ]);
    /// ```
    ///
    pub fn add_header<R: Into<Row>>(&mut self, row: R) -> &mut Table<T> {
        let row = row.into();
        self.header = Some(row);

        self
    }

    /// Returns the vec of max columns length for the table.
    fn set_max_width(&mut self) -> Vec<usize> {
        // create a new vec of zero with size equal to number of columns
        let mut column_len: Vec<usize> = vec![0; self.rows[0].cells.len()];

        // set values of vec with the max column length
        if let Some(header) = &self.header {
            max_column_length(&mut column_len, header);
        }
        for row in self.rows.iter() {
            max_column_length(&mut column_len, row);
        }

        column_len
    }

    /// print the top header with the given border or default of table like this:
    /// ╔════════╤═══════════╗
    ///
    fn print_header(&mut self, column_len: &[usize]) {
        let mut view = self.border.top_left.to_string();
        // make an iterator to generate the border top and join this with border top middle
        let header: Vec<String> = column_len
            .iter()
            .map(|col| self.border.top.to_string().repeat(*col))
            .collect();
        view += &header.join(&self.border.top_mid.to_string());
        view += &self.border.top_right.to_string();

        printfl!(self.handle, "{}\n", view);
    }

    /// print the bottom with the given border or default of table like this:
    /// ╚════════╧════════════╝
    fn print_bottom(&mut self, column_len: &[usize]) {
        let mut view: String = self.border.bottom_left.to_string();
        // make an iterator to generate the border bottom and join this with border bottom middle
        let bottom: Vec<String> = column_len
            .iter()
            .map(|col| self.border.bottom.to_string().repeat(*col))
            .collect();
        view += &bottom.join(&self.border.bottom_mid.to_string());
        view += &self.border.bottom_right.to_string();

        printfl!(self.handle, "\n{}", view);
    }

    /// print the middle (jointures between two rows) of table.
    /// ╟────────┼──────────╢
    fn print_table_middle(&mut self, column_len: &[usize]) -> String {
        let mut view: String = self.border.left_mid.to_string();
        // make an iterator to generate the border middle
        let middle: Vec<String> = column_len
            .iter()
            .map(|col| self.border.mid.to_string().repeat(*col))
            .collect();
        view += &middle.join(&self.border.mid_mid.to_string());
        view += &self.border.right_mid.to_string();

        view
    }

    /// print every rows and header of table.
    fn print_lines(&mut self) {
        let mut contents = vec![];
        let width_column = self.set_max_width();

        if let Some(header) = self.header.as_ref() {
            contents.push(self.print_line(header, &width_column));
        }

        for row in self.rows.iter() {
            contents.push(self.print_line(row, &width_column));
        }

        self.print_header(&width_column);
        for (index, content) in contents.iter().enumerate() {
            self.draw(content, &width_column, (contents.len() - 1) == index);
        }
        self.print_bottom(&width_column);
    }

    fn print_line(&self, row: &Row, width_column: &[usize]) -> Vec<Vec<String>> {
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
                let size = content.len();
                content
                    .iter()
                    .enumerate()
                    .fold(vec![], move |mut acc, (index, cell)| {
                        match cell.get(i) {
                            Some(value) => line.push(
                                value.to_string()
                                    + &" ".repeat(
                                        *width_column.get(index).unwrap_or(&(0_usize))
                                            - min(
                                                *width_column.get(index).unwrap_or(&(0_usize)),
                                                value.chars().count(),
                                            ),
                                    ),
                            ),
                            None => line
                                .push(" ".repeat(*width_column.get(index).unwrap_or(&(0_usize)))),
                        }
                        if index + 1 == size {
                            acc.push(line.clone());
                        }
                        acc
                    })
            })
            .collect::<Vec<_>>()
    }

    fn draw(&mut self, rows: &Vec<Vec<String>>, width_column: &[usize], last_row: bool) {
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
                lines.push(self.print_table_middle(width_column));
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
