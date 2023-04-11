use rct_derive::ToTable;

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
    let movies = [Movies {
        id: 1,
        title: "Harry Potter".to_string(),
        is_enabled: false,
        price: 20.0,
        currency: "$".to_string(),
        description: "Movie".to_string(),
        created_at: "2001-01-01".to_string(),
    },Movies {
        id: 2,
        title: "Harry Potter Deux".to_string(),
        is_enabled: false,
        price: 20.0,
        currency: "$".to_string(),
        description: "Movie".to_string(),
        created_at: "2002-01-01".to_string(),
    }];
    
    let table = movies.into_iter().to_table();
    
    println!("{}", table.to_string());
}
