use std::fmt::{Display, Formatter};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct Cell {
    pub data: Vec<String>,
    pub height: usize,
    pub width: usize,
}

pub trait ICell {
    fn cell(self) -> Cell;
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.join("\n"))
    }
}

impl<T> ICell for T
where
    T: Display,
{
    /// This will change each values contained the display Trait in string value
    fn cell(self) -> Cell {
        let data: Vec<String> = self.to_string().lines().map(ToString::to_string).collect();
        Cell {
            data: data.clone(),
            height: data.len(),
            width: data.iter().map(|string| string.width()).max().unwrap(),
        }
    }
}
