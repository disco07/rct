use crate::cell::Cells;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rows {
    pub cells: Vec<Cells>,
}
