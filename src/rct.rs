use crate::color::Colorizer;
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
    data: Vec<BTreeMap<u32, String>>,
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

/// Create a new table CLI with default configuration.
///
/// # Examples
///
/// ```no_run
/// use std::collections::BTreeMap;
/// use rct::rct::Table;
///
/// let mut table = Table::new();
/// let mut data: Vec<BTreeMap<u32, String>> = vec![];
/// data.push(BTreeMap::from([
///         (1, "62".to_string()),
///         (2, "Harry Potter".to_string()),
///         (3, "1".to_string()),
///         (4, "14.87".to_string()),
///         (5, "€".to_string()),
///         (6, "Harry Potter".to_string()),
///         (7, "2001-12-05 10:34:39".to_string()),
///  ]));
///  data.push(BTreeMap::from([
///         (1, "72".to_string()),
///         (2, "Spider-man".to_string()),
///         (3, "0".to_string()),
///         (4, "18.80".to_string()),
///         (5, "€".to_string()),
///         (6, "Spider-man, No Way Home.".to_string()),
///         (7, "2021-12-17 22:15:00".to_string()),
///  ]));
///
///  table.add_field(1, "id");
///  table.add_field(2, "title");
///  table.add_field(3, "is_enabled");
///  table.add_field(4, "price");
///  table.add_field(5, "currency");
///  table.add_field(6, "description");
///  table.add_field(7, "created_at");
///  table.add_data(data);
///
/// table.view()
///
/// ```
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

    pub fn add_data(&mut self, data: Vec<BTreeMap<u32, String>>) {
        self.data = data;
    }

    /// Add fields title to table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rct::rct::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_field(1, "ID");
    /// table.add_field(2, "Title");
    ///
    /// ```
    pub fn add_field(&mut self, field_key: u32, field_name: &str) -> &mut Table<T> {
        self.fields.insert(
            field_key,
            HashMap::from([
                ("key".to_string(), field_key.to_string()),
                ("name".to_string(), field_name.to_string()),
                ("color".to_string(), "".to_string()),
            ]),
        );
        self
    }

    /// Add color to the text.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rct::rct::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_field(1, "ID").add_color("#ff0000");
    /// table.add_field(2, "Title").add_color("#0000ff");
    ///
    /// ```
    pub fn add_color(&mut self, color: &str) -> &mut Table<T> {
        let key: u32 = self.fields.keys().last().unwrap().to_string().parse().unwrap();
        self.fields.insert(
            *self.fields.keys().last().unwrap(),
            HashMap::from([
                (
                    "key".to_string(),
                    key.to_string(),
                ),

                ("name".to_string(), self.fields.get(&key).unwrap().get("name").unwrap().to_string()),
                ("color".to_string(), color.to_string()),
            ]),
        );

        self
    }

    fn create_table(&self) -> String {
        let mut header: HashMap<u32, String> = HashMap::new();
        let mut column_len: HashMap<u32, u32> = HashMap::new();
        let mut cell: Vec<HashMap<u32, String>> = vec![];
        let mut table: String = String::new();

        for field in self.fields.iter() {
            header.insert(*field.0, field.1.get("name").unwrap().to_string());

            match column_len.get(field.0) {
                None => column_len.insert(*field.0, 0),
                Some(_) => None,
            };
            column_len.insert(
                *field.0,
                max(
                    *column_len.get(field.0).unwrap(),
                    field.1.get("name").unwrap().to_string().chars().count() as u32,
                ),
            );
        }

        if self.data.len() > 0 {
            for data in self.data.iter() {
                cell.push(HashMap::from_iter(data.to_owned()));
                for c in data {
                    column_len.insert(
                        *c.0,
                        max(
                            *column_len.get(c.0).unwrap(),
                            c.1.to_string().chars().count() as u32,
                        ),
                    );
                }
            }
        }

        table += &self.print_table_top(&column_len);
        table += &self.print_table_row(header, &column_len);
        table += &self.print_table_middle(&column_len);

        let mut count = 0;
        for i in 0..cell.len() {
            count += 1;
            table += &self.print_table_row(cell.get(i).unwrap().to_owned(), &column_len);
            if cell.len() > count {
                table += &self.print_table_middle(&column_len);
            } else {
                table += &self.print_table_bottom(&column_len);
            }
        }

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
            table += &row
                .1
                .trim()
                .colorize(self.fields.get(&row.0).unwrap().get("color").unwrap());
            table += &" ".repeat(
                (*column_len.get(&row.0).unwrap() - row.1.chars().count() as u32 + 1) as usize,
            );

            if column_len.len() > count {
                table += &self.table_view.middle;
            }
            space_count = 1;
        }
        table += &self.table_view.right;
        table += "\n";

        table
    }

    fn print_table_middle(&self, column_len: &HashMap<u32, u32>) -> String {
        let mut table: String = String::new();
        let fields: BTreeMap<_, _> = column_len.into_iter().collect();
        let mut count = 0;
        table += &self.table_view.left_mid;
        for len in fields.iter() {
            count += 1;
            table += &self.table_view.mid.repeat((**len.1 as usize) + 2);
            if column_len.len() > count {
                table += &self.table_view.mid_mid;
            }
        }
        table += &self.table_view.right_mid;
        table += "\n";

        table
    }

    fn print_table_bottom(&self, column_len: &HashMap<u32, u32>) -> String {
        let fields: BTreeMap<_, _> = column_len.into_iter().collect();
        let mut table: String = String::new();
        let mut count = 0;

        table += &self.table_view.bottom_left;
        for len in fields.iter() {
            count += 1;
            table += &self.table_view.bottom.repeat((**len.1 as usize) + 2);
            if column_len.len() > count {
                table += &self.table_view.bottom_mid;
            }
        }
        table += &self.table_view.bottom_right;
        table += "\n";

        table
    }

    /// Display the table on terminal.
    pub fn view(&mut self) {
        printfl!(self.handle, "\r{}", self.create_table());
    }
}
