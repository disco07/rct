use crate::row::Row;
use crate::styles::borders::Border;
use crate::styles::color::split_colors;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Table {
    header: Option<Row>,
    rows: Vec<Row>,
    border: Border,
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

    /// Given a vector of cells, where each cell contains a vector of strings,
    /// returns a vector of vectors, where each inner vector represents a row of the output.
    /// The output format is a matrix where each value is padded with spaces to match the desired column width.
    ///
    /// # Arguments
    ///
    /// *`cells` - A vector of `Cell` structures, where each cell contains a vector of strings.
    /// *`width_column` - A slice of integers representing the desired width of each column.
    ///
    /// # Example
    ///
    /// let cells = vec![
    /// Cell { data: vec!["a"] },
    /// Cell { data: vec!["b", "c", "d"] },
    /// Cell { data: vec!["e"] }
    /// ];
    ///
    /// let width_column = vec![3, 3, 3];
    ///
    /// let output = print_line(&cells, &width_column);
    ///
    /// assert_eq!(output, vec![
    /// vec![" a ", " b ", " e "],
    /// vec![" ", " c ", " "],
    /// vec![" ", " d ", " "]
    /// ]);
    ///
    fn print_line(&self, row: &Row, width_column: &[usize]) -> Vec<Vec<String>> {
        // Collect the cell contents into a vector of vectors, adding spaces around each value
        // Example input:
        //   ["a", "b,c,d", "e"] --> [[" a "], [" b ", " c ", " d "], [" e "]]
        let content = row
            .cells
            .iter()
            .map(|cell| {
                cell.data
                    .iter()
                    .map(|data| format!(" {} ", data))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Get the maximum number of cells in any column
        // Example input:
        //   [[" a "], [" b ", " c ", " d "], [" e "]] --> max_column = 3
        let max_column = content.iter().map(|c| c.len()).max().unwrap_or(0);

        // Build a vector of vectors, with each inner vector representing a row of the output
        // Example output:
        //   [[" a ", " b ", " e "], ["   ", " c ", "   "], ["   ", " d ", "   "]]
        (0..max_column)
            .map(|i| {
                let mut row = Vec::new();
                for (index, col) in content.iter().enumerate() {
                    let value = col.get(i).unwrap_or(&String::new()).to_owned();
                    let width = *width_column.get(index).unwrap_or(&0);
                    // Add padding to the cell value to match the desired column width
                    let padding =
                        " ".repeat(width.saturating_sub(split_colors(&value).chars().count()));
                    row.push(format!("{}{}", value, padding));
                }
                row
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
    for (index, row) in rows.iter().enumerate() {
        let current_max = column_len.get(index).unwrap_or(&0);
        if *row > *current_max {
            column_len[index] = *row + 2;
        }
    }
}
