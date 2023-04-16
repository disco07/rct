use rct::styles::color::{Colorizer, Font};
use rct::ToTable;

#[derive(ToTable)]
struct Movies<T, S> {
    #[table(rename = "ID", color = "#00ff00")]
    id: T,
    #[table(rename = "Title", bg = "#ff0000")]
    title: S,
    #[table(rename = "Price €", font = "Font::Bold")]
    price: f32,
}

fn main() {
    let movies = [
        Movies {
            id: 1,
            title: "Harry \nPotter".to_string(),
            price: 14.87,
        },
        Movies {
            id: 2,
            title: "Spider-man".to_string(),
            price: 18.80,
        },
    ];

    let table = movies.into_iter().to_table();

    println!("{}", table.to_string());
}
