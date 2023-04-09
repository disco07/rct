use rct_derive::ToTable;
use rct::ITable;

#[derive(ToTable)]
struct Movies {
    id: u32,
    title: String,
    is_enabled: bool,
    price: f32,
    currency: String,
    description: String,
    created_at: String,
}

fn main() {
    let movies = Movies {
        id: 1,
        title: "Harry Potter".to_string(),
        is_enabled: false,
        price: 20.0,
        currency: "$".to_string(),
        description: "Movie".to_string(),
        created_at: "2023-01-01".to_string(),
    };
    let table = movies.to_table();
    println!("{}", table.to_string());
}
