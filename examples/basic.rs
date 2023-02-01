use rct::rct::Table;
use std::collections::BTreeMap;

fn main() {
    let mut table = Table::new();
    let mut data: Vec<BTreeMap<u32, String>> = vec![];
    data.push(BTreeMap::from([
        (1, "Drissa".to_string()),
        (2, "Kone".to_string()),
        (3, "07th April 1991".to_string()),
        (4, "Yes".to_string()),
        (5, "10 minutes".to_string()),
    ]));
    data.push(BTreeMap::from([
        (1, "Yaya".to_string()),
        (2, "Kone".to_string()),
        (3, "07th April 1991".to_string()),
        (4, "No".to_string()),
        (5, "9 minutes".to_string()),
    ]));

    table.add_field(1, "First Name");
    table.add_field(2, "Last Name");
    table.add_field(3, "DOB");
    table.add_field(4, "Admin");
    table.add_field(5, "Last Seen");
    table.add_data(data);

    table.view()
}
