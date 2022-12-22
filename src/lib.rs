use std::str::FromStr;

/// A tiny color conversion library for TUI application builders
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a new `Color` with the given red, green, and blue values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tiny_color::Color;
    /// 
    /// let color = Color::new(255, 0, 0);
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Converts the color to hexadecimal format.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tiny_color::Color;
    /// 
    /// let color = Color::new(255, 0, 0);
    /// assert_eq!(color.to_hex(), "#FF0000");
    /// ```
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl FromStr for Color {
    type Err = String;

    /// Parses a hexadecimal color string and returns a `Color` struct.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tiny_color::Color;
    /// 
    /// let color = "#FF0000".parse::<Color>().unwrap();
    /// assert_eq!(color, Color::new(255, 0, 0));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 || !s.starts_with('#') {
            return Err(format!("invalid color string: {}", s));
        }
        let r = u8::from_str_radix(&s[1..3], 16).unwrap();
        let g = u8::from_str_radix(&s[3..5], 16).unwrap();
        let b = u8::from_str_radix(&s[5..7], 16).unwrap();
        Ok(Self::new(r, g, b))
    }
}
