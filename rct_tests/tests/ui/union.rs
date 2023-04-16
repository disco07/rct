use rct::ToTable;

#[derive(ToTable)]
union Test {
    price: f32,
    price2: f32,
}

fn main () {}