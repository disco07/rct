use std::fmt::Display;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub data: Vec<String>,
    pub height: usize,
    pub width: usize,
}

pub trait ICell {
    fn cell(self) -> Cell;
}

impl<T> ICell for T where T: Display {
    fn cell(self) -> Cell {
        let data: Vec<String>  = self
            .to_string()
            .lines()
            .map(ToString::to_string)
            .collect();
        Cell {
            data: data.clone(),
            height: data.clone().len(),
            width: data.clone().iter().map(|string| string.width()).max().unwrap(),
        }
    }
}

impl ICell for Cell {
    fn cell(self) -> Cell {
        self.to_owned()
    }
}