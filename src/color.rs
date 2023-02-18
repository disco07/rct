pub trait Colorizer {
    fn colorize(&self, hex: &str) -> String;
}

impl Colorizer for str {
    fn colorize(&self, hex: &str) -> String {
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
        format!("{}{}\x1b[0m", color, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Colorizer;
    #[test]
    fn test_colorize_white() {
        let colour = "string".colorize("#ffffff");
        assert_eq!(colour, "\u{1b}[38;2;255;255;255mstring\u{1b}[0m")
    }
    #[test]
    fn test_colorize_black() {
        let colour = "string".colorize("#000000");
        assert_eq!(colour, "\u{1b}[38;2;0;0;0mstring\u{1b}[0m")
    }
    #[test]
    fn test_colorize_not_hex() {
        let colour = "string".colorize("black");
        assert_eq!(colour, "string\u{1b}[0m")
    }
}
