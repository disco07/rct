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

trait Ro {}

impl Ro for Movies {
    
}

impl<T> ITable for T 
where T: Ro + 'static {
    fn to_table(self) -> rct::Table {
        unimplemented!()
    }
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
    let vec_movies = vec![Movies {
        id: 1,
        title: "Harry Potter".to_string(),
        is_enabled: false,
        price: 20.0,
        currency: "$".to_string(),
        description: "Movie".to_string(),
        created_at: "2023-01-01".to_string(),
    }];
    let table = movies.to_table();
    let v = vec_movies.t();
    
    println!("{}", table.to_string());
}
