use std::collections::HashMap;
use std::io::{Stdout, Write};

pub struct Table<T: Write> {
    fields: HashMap<String, HashMap<String, String>>,
    table_view: TableView,
    handle: T,
}

pub struct TableView {
    top: String,
    top_mid: String,
    top_left: String,
    top_right: String,
    bottom: String,
    bottom_mid: String,
    bottom_left: String,
    bottom_right: String,
    left: String,
    left_mid: String,
    middle: String,
    right: String,
    right_mid: String,
    mid: String,
    mid_mid: String,
}

impl Default for TableView {
    fn default() -> Self {
        Self {
            top: "═".to_string(),
            top_mid: "╤".to_string(),
            top_left: "╔".to_string(),
            top_right: "╗".to_string(),
            bottom: "═".to_string(),
            bottom_mid: "╧".to_string(),
            bottom_left: "╚".to_string(),
            bottom_right: "╝".to_string(),
            left: "║".to_string(),
            left_mid: "╟".to_string(),
            middle: "│ ".to_string(),
            right: "║".to_string(),
            right_mid: "╢".to_string(),
            mid: "─".to_string(),
            mid_mid: "┼".to_string(),
        }
    }
}

impl Table<Stdout> {
    /// Create a new ProgressBar with default configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread;
    /// use pbr::{ProgressBar, Units};
    ///
    /// let count = 1000;
    /// let mut pb = ProgressBar::new(count);
    /// pb.set_units(Units::Bytes);
    ///
    /// for _ in 0..count {
    ///    pb.inc();
    ///    thread::sleep_ms(100);
    /// }
    /// ```
    pub fn new() -> Table<Stdout> {
        let handle = ::std::io::stdout();
        Table::on(handle)
    }
}

impl<T: Write> Table<T> {
    pub fn on(handle: T) -> Table<T> {
        Self {
            fields: HashMap::new(),
            table_view: TableView::default(),
            handle
        }
    }

    pub fn add_field(&mut self, field_key: &str, field_name: &str) {
        self.fields.insert(field_key.to_string(), HashMap::from([
            ("name".to_string(), field_name.to_string()),
            ("key".to_string(), field_key.to_string()),
        ]));
    }

    fn create_table() {

    }
}

fn main() {
    println!("Hello, world!");
}
