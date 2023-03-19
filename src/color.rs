use crate::cell::Cell;

pub trait Colorizer {
    fn color(&self, hex: &str) -> Cell;
}

impl Colorizer for Cell {
    /// Colorizes [Cell] with hex color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rct::cell::ICell;
    /// use rct::color::Colorizer;
    ///
    /// let colour = "string".cell().color("#ffffff");
    /// assert_eq!(colour.to_string(), "\u{1b}[38;2;255;255;255mstring\u{1b}[0m")
    /// ```
    fn color(&self, hex: &str) -> Cell {
        let mut color = String::new();
        if hex.starts_with('#') && hex.len() == 7 {
            color.push_str("\x1B[38;2;");
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
        let mut data = vec![];
        for cell in &self.data {
            let c = format!("{}{}\x1b[0m", color, cell);
            data.push(c);
        }

        Cell {
            data,
            height: self.height,
            width: self.width,
        }
    }
}

/// Transforms string colored to string.
/// ```
/// use rct::color::split_colors;
/// let string = String::from("\u{1b}[38;2;255;255;255mstring\u{1b}[0m");
/// let split_color = split_colors(&string);
///
/// assert_eq!(split_color, "string  ")
/// ```
pub fn split_colors(color: &str) -> String {
    if color.contains("[38;2;") {
        let (_, c) = color.split_once('m').unwrap();
        let color_splited = c
            .to_string()
            .split("\u{1b}")
            .map(String::from)
            .collect::<Vec<_>>();
        return (color_splited[0].to_string() + &" ".repeat(2)).to_string();
    }

    color.to_string()
}

#[cfg(test)]
mod tests {
    use crate::cell::ICell;
    use crate::color::Colorizer;
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
    fn test_colorize_not_hex() {
        let colour = "string".cell().color("black");
        assert_eq!(colour.to_string(), "string\u{1b}[0m")
    }
}
