#[derive(Debug, Clone)]
pub struct Border {
    pub(crate) top: char,
    pub(crate) top_mid: char,
    pub(crate) top_left: char,
    pub(crate) top_right: char,
    pub(crate) bottom: char,
    pub(crate) bottom_mid: char,
    pub(crate) bottom_left: char,
    pub(crate) bottom_right: char,
    pub(crate) left: char,
    pub(crate) left_mid: char,
    pub(crate) middle: char,
    pub(crate) right: char,
    pub(crate) right_mid: char,
    pub(crate) mid: char,
    pub(crate) mid_mid: char,
}

pub enum BorderStyle {
    Default,
    Simple,
    Empty,  
}

impl Border {
    pub fn simple() -> Self {
        Self {
            top: '-',
            top_mid: '+',
            top_left: '+',
            top_right: '+',
            bottom: '-',
            bottom_mid: '+',
            bottom_left: '+',
            bottom_right: '+',
            left: '|',
            left_mid: '+',
            middle: '│',
            right: '|',
            right_mid: '+',
            mid: '-',
            mid_mid: '+',
        }
    }

    pub fn empty() -> Self {
        Self {
            top: ' ',
            top_mid: ' ',
            top_left: ' ',
            top_right: ' ',
            bottom: ' ',
            bottom_mid: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            left: ' ',
            left_mid: ' ',
            middle: ' ',
            right: ' ',
            right_mid: ' ',
            mid: ' ',
            mid_mid: ' ',
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: '═',
            top_mid: '╤',
            top_left: '╔',
            top_right: '╗',
            bottom: '═',
            bottom_mid: '╧',
            bottom_left: '╚',
            bottom_right: '╝',
            left: '║',
            left_mid: '╟',
            middle: '│',
            right: '║',
            right_mid: '╢',
            mid: '─',
            mid_mid: '┼',
        }
    }
}
