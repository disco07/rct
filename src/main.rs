use std::collections::HashMap;

pub struct Table {
    fields: HashMap<&'static str, &'static str>,
    table_view: TableView,
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

fn main() {
    println!("Hello, world!");
}
