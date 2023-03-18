use crate::row::Row;
use std::cmp::min;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Table {
    header: Option<Row>,
    rows: Vec<Row>,
    border: Border,
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

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_lines())
    }
}

impl Table {
    /// Create a new table.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use rct::table::Table;
    /// let mut table = Table::new();
    /// ```
    ///
    pub fn new() -> Table {
        Self {
            header: None,
            rows: vec![],
            border: Default::default(),
        }
    }

    /// Add a new row to the table.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
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
    pub fn add_row<R: Into<Row>>(&mut self, row: R) -> &mut Table {
        let row = row.into();
        self.rows.push(row);

        self
    }

    /// Add a header to the table.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
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
    pub fn add_header<R: Into<Row>>(&mut self, row: R) -> &mut Table {
        let row = row.into();
        self.header = Some(row);

        self
    }

    /// Returns the vec of max columns length for the table.
    fn set_max_width(&self) -> Vec<usize> {
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
    fn print_header(&self, column_len: &[usize]) -> String {
        let mut view = self.border.top_left.to_string();
        // make an iterator to generate the border top and join this with border top middle
        let header: Vec<String> = column_len
            .iter()
            .map(|col| self.border.top.to_string().repeat(*col))
            .collect();
        view += &header.join(&self.border.top_mid.to_string());
        view += &self.border.top_right.to_string();
        view += "\n";

        view
    }

    /// print the bottom with the given border or default of table like this:
    /// ╚════════╧════════════╝
    fn print_bottom(&self, column_len: &[usize]) -> String {
        let mut view: String = self.border.bottom_left.to_string();
        // make an iterator to generate the border bottom and join this with border bottom middle
        let bottom: Vec<String> = column_len
            .iter()
            .map(|col| self.border.bottom.to_string().repeat(*col))
            .collect();
        view += &bottom.join(&self.border.bottom_mid.to_string());
        view += &self.border.bottom_right.to_string();

        view
    }

    /// print the middle (jointures between two rows) of table.
    /// ╟────────┼──────────╢
    fn print_table_middle(&self, column_len: &[usize]) -> String {
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
    fn print_lines(&self) -> String {
        let mut view = String::new();
        let mut contents = vec![];
        let width_column = self.set_max_width();

        if let Some(header) = self.header.as_ref() {
            contents.push(self.print_line(header, &width_column));
        }

        for row in self.rows.iter() {
            contents.push(self.print_line(row, &width_column));
        }

        view += &self.print_header(&width_column);
        for (index, content) in contents.iter().enumerate() {
            view += &self.draw(content, &width_column, (contents.len() - 1) == index);
        }
        view += &self.print_bottom(&width_column);
        view
    }

    fn print_line(&self, row: &Row, width_column: &[usize]) -> Vec<Vec<String>> {
        // Iterate over all cells and create a new vector of all them like this:
        // [["string1"], ["string2", "string3"], ["string4"]]
        let content = row
            .cells
            .iter()
            .map(|cell| {
                cell.data.iter().fold(vec![], |mut acc, data| {
                    let mut value = String::from(" ");
                    value += data;
                    value += " ";
                    acc.push(value);
                    acc
                })
            })
            .collect::<Vec<_>>();

        // Get the maximum of length of vector into a vector.
        // [["string1"], ["string2", "string3"], ["string4"]] --> 2
        let max_column = content.iter().map(|c| c.len()).max().unwrap_or(0);

        // Now, we create a new to separate each value into a vector of vectors.
        // We pass of this:
        // [["string1"], ["string2", "string3"], ["string4"]]
        //
        // to this:
        // [["string1"], ["string2"], ["", "string3", ""] ["string4"]]
        (0..max_column)
            .flat_map(|i| {
                let mut line = Vec::new();
                let size = content.len();
                content
                    .iter()
                    .enumerate()
                    .fold(vec![], move |mut acc, (index, cell)| {
                        // Check if cell(vec) got a value with index i.
                        // if value return value + whitespace or return whitespace,
                        // the whitespace equal to width of column minus the minimum between
                        // width of column and the length of value.
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

    /// Create all of the lines in rows with the border.
    fn draw(&self, rows: &Vec<Vec<String>>, width_column: &[usize], last_row: bool) -> String {
        let mut view = String::new();
        // We transform this vector:
        // [["string1"], ["string2"], ["", "string3", ""] ["string4"]]
        // to:
        // [║"string1"│, "string2"│, "string4║]
        // ╟──────┼──────────┼────────────╢
        // [║"      "│, "string3"│, "      "║]
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
            // do not add separators if it is the last line
            if rows.len() - 1 == i && !last_row {
                lines.push(self.print_table_middle(width_column));
            }
        }
        view += &lines.join("\n");
        view += "\n";

        view
    }

    /// Display the table on terminal.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
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
    ///
    /// table.view();
    /// ```
    pub fn view(&self) {
        println!("{}", self.print_lines());
    }
}

/// Calculates the max length for every column.
fn max_column_length(column_len: &mut [usize], row: &Row) {
    let rows: Vec<_> = row.width();
    println!("column_len debut => {:?}", column_len);
    println!("rows => {:?}", rows);

    for (index, row) in rows.iter().enumerate() {
        let current_max = column_len.get(index).unwrap_or(&0);
        if *row > *current_max {
            column_len[index] = *row + 2;
        }
    }
    println!("column_len fin => {:?}", column_len);
}
