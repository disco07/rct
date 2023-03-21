use crate::cell::Cell;
use regex::Regex;
use std::str;

pub trait Colorizer {
    fn color(&self, hex: &str) -> Cell;
    fn bg(&self, hex: &str) -> Cell;
    fn font(&self, font: Font) -> Cell;
}

pub enum Font {
    Bold = 1,
    Light = 2,
    Italic = 3,
    Underlined = 4,
    SlowBlinking = 5,
    Blinking = 6,
    Inverse = 7,
    Invisible = 8,
    Strikethrough = 9,
}

impl Colorizer for Cell {
    /// Colorizes [Cell] with hex color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::cell::ICell;
    /// use rct::styles::color::Colorizer;
    ///
    /// let colour = "string".cell().color("#ffffff");
    /// assert_eq!(colour.to_string(), "\u{1b}[38;2;255;255;255mstring\u{1b}[0m")
    /// ```
    fn color(&self, hex: &str) -> Cell {
        let color = new_ansi(hex, 38);
        // Create a new vector to hold the data with the color applied
        let mut data = vec![];
        for cell in &self.data {
            // Apply the color code to each cell and reset the color at the end
            let c = format!("{}{}\x1b[0m", color, cell);
            data.push(c);
        }

        // Return a new Cell with the colored data and the original height and width
        Cell {
            data,
            height: self.height,
            width: self.width,
        }
    }

    /// Colorizes [Cell] with hex color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::cell::ICell;
    /// use rct::styles::color::Colorizer;
    ///
    /// let colour = "string".cell().bg("#ffffff");
    /// assert_eq!(colour.to_string(), "\u{1b}[48;2;255;255;255mstring\u{1b}[0m")
    /// ```
    fn bg(&self, hex: &str) -> Cell {
        let color = new_ansi(hex, 48);
        // Create a new vector to hold the data with the color applied
        let mut data = vec![];
        for cell in &self.data {
            // Apply the color code to each cell and reset the color at the end
            let c = format!("{}{}\x1b[0m", color, cell);
            data.push(c);
        }

        // Return a new Cell with the colored data and the original height and width
        Cell {
            data,
            height: self.height,
            width: self.width,
        }
    }

    /// Colorizes [Cell] with hex color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::cell::ICell;
    /// use rct::styles::color::{Colorizer, Font};
    ///
    /// let colour = "string".cell().font(Font::Bold);
    /// assert_eq!(colour.to_string(), "\u{1b}[1mstring\u{1b}[0m")
    /// ```
    fn font(&self, font: Font) -> Cell {
        let color = format!("\x1B[{}m", font as usize);

        // Create a new vector to hold the data with the color applied
        let mut data = vec![];
        for cell in &self.data {
            // Apply the color code to each cell and reset the color at the end
            let c = format!("{}{}\x1b[0m", color, cell);
            data.push(c);
        }

        // Return a new Cell with the colored data and the original height and width
        Cell {
            data,
            height: self.height,
            width: self.width,
        }
    }
}

fn new_ansi(hex: &str, value: usize) -> String {
    let mut color = String::new();
    // Check if the hex code is valid (starts with '#' and has a length of 7)
    if hex.starts_with('#') && hex.len() == 7 {
        // Append the color code to the 'color' string using the ANSI escape code format
        color.push_str(&format!("\x1B[{};2;", value));
        color.push_str(
            format!(
                "{};{};{}m",
                i16::from_str_radix(&hex[1..3], 16).unwrap(),
                i16::from_str_radix(&hex[3..5], 16).unwrap(),
                i16::from_str_radix(&hex[5..7], 16).unwrap()
            )
            .as_str(),
        );
    }
    color
}

/// Transforms string colored to string.
/// ```
/// use rct::styles::color::split_colors;
/// let string = String::from("\u{1b}[38;2;255;255;255mstring\u{1b}[0m");
/// let split_color = split_colors(&string);
///
/// assert_eq!(split_color, "string")
/// ```
pub fn split_colors(color: &str) -> String {
    let re = Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();

    if re.is_match(color) {
        let strip_ansi_escapes = strip_ansi_escapes::strip(color).unwrap();
        let color = str::from_utf8(&strip_ansi_escapes).unwrap();
        return color.to_string();
    }

    color.to_string()
}

#[cfg(test)]
mod tests {
    use crate::cell::ICell;
    use crate::styles::color::{split_colors, Colorizer, Font};

    #[test]
    fn test_colorize_white() {
        let colour = "string".cell().color("#ffffff");
        assert_eq!(
            colour.to_string(),
            "\u{1b}[38;2;255;255;255mstring\u{1b}[0m"
        )
    }
    #[test]
    fn test_colorize_black() {
        let colour = "string".cell().color("#000000");
        assert_eq!(colour.to_string(), "\u{1b}[38;2;0;0;0mstring\u{1b}[0m")
    }
    #[test]
    fn test_bg_black() {
        let colour = "string".cell().bg("#000000");
        assert_eq!(colour.to_string(), "\u{1b}[48;2;0;0;0mstring\u{1b}[0m")
    }
    #[test]
    fn test_font() {
        let font = "string".cell().font(Font::Bold);
        assert_eq!(font.to_string(), "\u{1b}[1mstring\u{1b}[0m")
    }
    #[test]
    fn test_colorize_not_hex() {
        let colour = "string".cell().color("black");
        assert_eq!(colour.to_string(), "string\u{1b}[0m")
    }
    #[test]
    fn test_split_colors() {
        let string = String::from("\u{1b}[38;2;255;255;255mstring\u{1b}[0m");
        let split_color = split_colors(&string);
        assert_eq!(split_color, "string")
    }
}
