use rct::styles::color::{Colorizer, Font};
use rct::ToTable;

#[derive(ToTable)]
struct Tests<T, S> {
    #[table(rename = "ID", color = "#00ff00")]
    id: T,
    #[table(rename = "Title", bg = "#ff0000")]
    title: S,
    #[table(rename = "Price €", font = "Bold")]
    price: f32,
}