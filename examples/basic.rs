use rct::cell::ICell;
use rct::table::Table;

fn main() {
    let mut table = Table::new();

    table.add_header(vec!["ID".cell(), "Last Name".cell(), "First Name".cell()])
        .add_row(vec![1.cell(), "KONE".cell(), "Drissa".cell()])
        .add_row(vec![1.cell(), "KONE".cell(), "Yaya".cell()])
        .add_row(vec![1.cell(), "KONE".cell(), "Yacouba".cell()])
    ;

    table.view()
}
