use rct::styles::color::{Colorizer, Font};
use rct::ICell;
use rct::Table;

fn main() {
    let mut table = Table::new();

    table
        .add_header(vec![
            "ID".cell(),
            "Title".cell(),
            "is_enabled".cell(),
            "price".cell(),
            "currency".cell(),
            "description".cell(),
            "created_at".cell(),
        ])
        .add_row(vec![
            1.cell(),
            "Harry \nPotter".cell().color("#ff0000"),
            "1".cell(),
            "14.87".cell(),
            "€".cell(),
            "Harry Potter".cell(),
            "2001-12-05 22:05:20"
                .cell()
                .bg("#0000ff")
                .font(Font::Blinking),
        ])
        .add_row(vec![
            2.cell(),
            "Spider-man".cell().font(Font::Italic),
            "0".cell(),
            "18.80".cell().font(Font::Bold),
            "€".cell(),
            "Spider-man, No Way Home.".cell().font(Font::Strikethrough),
            "2018-12-12 09:04:50".cell(),
        ])
        .add_row(vec![
            3.cell(),
            "Avenger".cell().color("#00ff00"),
            "1".cell(),
            "18.50".cell(),
            "€".cell(),
            "Avenger".cell(),
            "2017-10-12 10:34:39".cell(),
        ]);

    table.view();
}
