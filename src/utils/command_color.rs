use crossterm::style::Color;

pub enum CommandColor {
    Blue,
    Yellow,
    Red,
}

impl CommandColor {
    pub fn get_color(&self) -> Color {
        match self {
            CommandColor::Blue => Color::Rgb {
                r: 105,
                g: 149,
                b: 210,
            },
            CommandColor::Yellow => Color::Rgb {
                r: 243,
                g: 185,
                b: 96,
            },
            CommandColor::Red => Color::Rgb {
                r: 208,
                g: 71,
                b: 72,
            },
        }
    }
}
