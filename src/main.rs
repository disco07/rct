use std::cmp::max;
use std::collections::{BTreeMap, HashMap};
use std::io::{Stdout, Write};

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

#[derive(Clone)]
pub struct Table<T: Write> {
    fields: BTreeMap<u32, HashMap<String, String>>,
    table_view: TableView,
    handle: T,
}

#[derive(Clone)]
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
            fields: BTreeMap::new(),
            table_view: TableView::default(),
            handle,
        }
    }

    pub fn add_field(&mut self, field_key: u32, field_name: &str) {
        self.fields.insert(
            field_key,
            HashMap::from([
                ("key".to_string(), field_key.to_string()),
                ("name".to_string(), field_name.to_string()),
            ]),
        );
    }

    fn create_table(&self) -> String {
        let mut header_data: HashMap<u32, String> = HashMap::new();
        let mut column_len: HashMap<u32, u32> = HashMap::new();
        let mut table: String = String::new();

        for field in self.fields.iter() {
            header_data.insert(
                *field.0,
                field.1.get("name").unwrap().to_string(),
            );

            match column_len.get(field.0) {
                None => column_len.insert(*field.0, 0),
                Some(_) => None,
            };
            column_len.insert(
                *field.0,
                max(
                    *column_len.get(field.0).unwrap(),
                    field.1.get("name").unwrap().to_string().len() as u32,
                ),
            );
        }

        table += &self.print_table_top(&column_len);
        table += &self.print_table_row(header_data, &column_len);

        table
    }

    fn print_table_top(&self, column_len: &HashMap<u32, u32>) -> String {
        let fields: BTreeMap<_, _> = column_len.into_iter().collect();
        let mut table: String = String::new();
        let mut count = 0;

        table += &self.table_view.top_left;
        for len in fields.iter() {
            count += 1;
            table += &self.table_view.top.repeat((**len.1 as usize) + 2);
            if column_len.len() > count {
                table += &self.table_view.top_mid;
            }
        }
        table += &self.table_view.top_right;
        table += "\n";

        table
    }

    fn print_table_row(
        &self,
        fields: HashMap<u32, String>,
        column_len: &HashMap<u32, u32>,
    ) -> String {
        let mut table: String = String::new();
        let mut count = 0;
        let mut space_count = 0;
        let rows: BTreeMap<_, _> = fields.into_iter().collect();
        table += &self.table_view.left;
        for row in rows {
            count += 1;

            if space_count == 0 && count == 1 {
                table += &" ".repeat(1);
            }
            table += &row.1.trim();
            table += &" ".repeat(1);

            if column_len.len() > count {
                table += &self.table_view.middle;
            }
            space_count = 1;
        }
        table += &self.table_view.right;
        table += "\n";

        table
    }

    pub fn view(&mut self) {
        printfl!(self.handle, "\r{}", self.create_table());
    }
}

fn main() {
    let mut table = Table::new();

    table.add_field(1, "First Name");
    table.add_field(2, "Last Name");
    table.add_field(3, "DOB");
    table.add_field(4, "Admin");
    table.add_field(5, "Last Seen");

    table.view()
}
