use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl From<Vec<Cell>> for Row {
    fn from(value: Vec<Cell>) -> Self {
        Row { cells: value }
    }
}

impl Row {
    /// Get the cell's width in row.
    pub fn width(&self) -> Vec<usize> {
        self.cells.iter().map(|cells| cells.width).collect()
    }
}
