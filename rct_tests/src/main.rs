use rct::ToTable;

#[derive(ToTable)]
struct Movies {
    #[table(name = "ID", color = "#00ff00")]
    id: u32,
    #[table(name = "Title", bg = "#0000ff")]
    title: String,
    #[table(name = "Price €", font = "Font::Blinking")]
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
        }
    ];

    let table = movies.into_iter().to_table();
    
    println!("{}", table.to_string());
}
