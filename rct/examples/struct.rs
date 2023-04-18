use rct::ToTable;

#[derive(ToTable)]
struct Movies<T, S> {
    #[table(rename = "ID")]
    id: T,
    #[table(rename = "Title")]
    title: S,
    #[table(rename = "Price €")]
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

    println!("{table}");
}