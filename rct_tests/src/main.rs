use rct_derive::Table;

#[derive(Table)]
struct Movies {
    id: u32,
    title: String,
    is_enabled: bool,
    price: f32,
    currency: String,
    description: String,
    created_at: String,
}

fn main() {}
