use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub data: Vec<String>,
}

pub trait ICell {
    fn cell(self) -> Cell;
}

impl<T> ICell for T where T: Display {
    fn cell(self) -> Cell {
        Cell {
            data: self
                .to_string()
                .lines()
                .map(ToString::to_string)
                .collect(),
        }
    }
}

impl ICell for Cell {
    fn cell(self) -> Cell {
        self.to_owned()
    }
}