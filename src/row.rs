use crate::cell::Cell;

#[derive(Debug, Clone, Default)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl From<Vec<Cell>> for Row {
    fn from(value: Vec<Cell>) -> Self {
        Row {
            cells: value,
        }
    }
}

impl Row {
    pub fn width(&self) -> Vec<usize> {
        self.cells
            .iter()
            .map(|cells| {
            cells.width
        }).collect()
    }

    pub fn columns(&self) -> usize {
        self.cells.len()
    }
}