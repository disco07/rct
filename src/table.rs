use std::io::{Stdout, Write};
use crate::row::Rows;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Table<T: Write> {
    header: Option<Rows>,
    rows: Vec<Rows>,
    border: Border,
    handle: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    /// Create a new table CLI with default configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rct::rct::Table;
    /// let mut table = Table::new();
    ///
    /// ```
    pub fn new() -> Table<Stdout> {
        let handle = std::io::stdout();
        Table::on(handle)
    }
}

impl<T: Write> Table<T> {
    pub fn on(handle: T) -> Table<T> {
        Self {
            fields: BTreeMap::new(),
            table_view: TableView::default(),
            data: vec![],
            handle,
        }
    }
}