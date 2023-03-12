use rct::cell::ICell;
use rct::table::Table;

#[test]
fn basic_table() {
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
            "Harry \nPotter".cell(),
            "1".cell(),
            "14.87".cell(),
            "€".cell(),
            "Harry Potter".cell(),
            "2001-12-05 22:05:20".cell(),
        ])
        .add_row(vec![
            2.cell(),
            "Spider-man".cell(),
            "0".cell(),
            "18.80".cell(),
            "€".cell(),
            "Spider-man, No Way Home.".cell(),
            "2018-12-12 09:04:50".cell(),
        ])
        .add_row(vec![
            3.cell(),
            "Avenger".cell(),
            "1".cell(),
            "18.50".cell(),
            "€".cell(),
            "Avenger".cell(),
            "2017-10-12 10:34:39".cell(),
        ]);

    let expected = "
╔════╤════════════╤════════════╤═══════╤══════════╤══════════════════════════╤═════════════════════╗
║ ID │ Title      │ is_enabled │ price │ currency │ description              │ created_at          ║
╟────┼────────────┼────────────┼───────┼──────────┼──────────────────────────┼─────────────────────╢
║ 1  │ Harry      │ 1          │ 14.87 │ €        │ Harry Potter             │ 2001-12-05 22:05:20 ║
║    │ Potter     │            │       │          │                          │                     ║
╟────┼────────────┼────────────┼───────┼──────────┼──────────────────────────┼─────────────────────╢
║ 2  │ Spider-man │ 0          │ 18.80 │ €        │ Spider-man, No Way Home. │ 2018-12-12 09:04:50 ║
╟────┼────────────┼────────────┼───────┼──────────┼──────────────────────────┼─────────────────────╢
║ 3  │ Avenger    │ 1          │ 18.50 │ €        │ Avenger                  │ 2017-10-12 10:34:39 ║
╚════╧════════════╧════════════╧═══════╧══════════╧══════════════════════════╧═════════════════════╝
    ";

    table.view();
    assert_eq!("\n".to_string(), expected)
}
