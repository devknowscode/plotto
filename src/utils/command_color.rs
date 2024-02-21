use crossterm::style::Color;

pub enum CommandColor {
    Blue,
}

impl CommandColor {
    pub fn get_color(&self) -> Color {
        match self {
            CommandColor::Blue => Color::Rgb {
                r: 105,
                g: 149,
                b: 210,
            },
        }
    }
}
