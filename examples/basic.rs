use rct::rct::Table;
use std::collections::BTreeMap;

fn main() {
    let mut table = Table::new();
    let mut data: Vec<BTreeMap<u32, String>> = vec![];
    data.push(BTreeMap::from([
        (1, "62".to_string()),
        (2, "Harry Potter".to_string()),
        (3, "1".to_string()),
        (4, "14.87".to_string()),
        (5, "€".to_string()),
        (6, "Harry Potter".to_string()),
        (7, "2001-12-05 10:34:39".to_string()),
    ]));

    data.push(BTreeMap::from([
        (1, "72".to_string()),
        (2, "Spider-man".to_string()),
        (3, "0".to_string()),
        (4, "18.80".to_string()),
        (5, "€".to_string()),
        (6, "Spider-man, No Way Home.".to_string()),
        (7, "2021-12-17 22:15:00".to_string()),
    ]));

    table.add_field(1, "id");
    table.add_field(2, "title");
    table.add_field(3, "is_enabled");
    table.add_field(4, "price");
    table.add_field(5, "currency");
    table.add_field(6, "description");
    table.add_field(7, "created_at");
    table.add_data(data);

    table.view()
}
