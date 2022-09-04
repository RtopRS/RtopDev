use std::fmt::Write;

pub struct VerticalBar {
    pub height: i32,
    pub width: i32,
    color: Color
}
impl VerticalBar {
    pub fn display(&self, pourcent: f32) -> String {
        let color = match self.color {
            Color::Red => "[[EFFECT_COLOR_RED_BLACK]]",
            Color::Green => "[[EFFECT_COLOR_GREEN_BLACK]]",
            Color::Yellow => "[[EFFECT_COLOR_YELLOW_BLACK]]",
            Color::Blue => "[[EFFECT_COLOR_BLUE_BLACK]]",
            Color::Magenta => "[[EFFECT_COLOR_MAGENTA_BLACK]]",
            Color::Cyan => "[[EFFECT_COLOR_CYAN_BLACK]]",
            Color::White => "[[EFFECT_COLOR_WHITE_BLACK]]"
        };

        let mut out = String::new();
        let mut name_to_find = std::collections::HashMap::new();
        name_to_find.insert(0, " ");
        name_to_find.insert(1, "▁");
        name_to_find.insert(2, "▂");
        name_to_find.insert(3, "▃");
        name_to_find.insert(4, "▄");
        name_to_find.insert(5, "▅");
        name_to_find.insert(6, "▆");
        name_to_find.insert(7, "▇");
        name_to_find.insert(8, "█");

        if pourcent == 100. {
            out += &format!("{}{}{}\n", color, "█".repeat(self.width as usize), color).repeat(self.height as usize);
        } else {
            let block_filled = (self.height as f32 * 8. * (pourcent / 100.)) as i32;
            let white_lines = (self.height - 1 - (self.height as f32 * (pourcent / 100.)) as i32) as usize;

            out += &format!("{}{}{}\n", color, " ".repeat(self.width as usize), color).repeat(white_lines);
            writeln!(&mut out, "{}{}{}", color, name_to_find[&(block_filled % 8)].repeat(self.width as usize), color).unwrap();
            out += &format!("{}{}{}\n", color, "█".repeat(self.width as usize), color).repeat(self.height as usize - white_lines - 1);
        }

        out
    }

    pub fn new(height: i32, width: i32, color: Option<Color>) -> VerticalBar {
        VerticalBar{height, width, color: color.unwrap_or(Color::Green)}
    }
}

pub struct HorizontalBar {
    pub height: i32,
    pub width: i32,
    color: Color
}
impl HorizontalBar {
    pub fn display(&self, pourcent: f32) -> String {
        let color = match self.color {
            Color::Red => "[[EFFECT_COLOR_RED_BLACK]]",
            Color::Green => "[[EFFECT_COLOR_GREEN_BLACK]]",
            Color::Yellow => "[[EFFECT_COLOR_YELLOW_BLACK]]",
            Color::Blue => "[[EFFECT_COLOR_BLUE_BLACK]]",
            Color::Magenta => "[[EFFECT_COLOR_MAGENTA_BLACK]]",
            Color::Cyan => "[[EFFECT_COLOR_CYAN_BLACK]]",
            Color::White => "[[EFFECT_COLOR_WHITE_BLACK]]"
        };

        let mut out = String::new();
        let mut name_to_find = std::collections::HashMap::new();
        name_to_find.insert(0, " ");
        name_to_find.insert(1, "▏");
        name_to_find.insert(2, "▎");
        name_to_find.insert(3, "▍");
        name_to_find.insert(4, "▌");
        name_to_find.insert(5, "▋");
        name_to_find.insert(6, "▊");
        name_to_find.insert(7, "▉");
        name_to_find.insert(8, "█");

        let block_filled = (self.width as f32 * 8. * (pourcent / 100.)) as i32;
        let white_lines = (self.width - 1 - (self.width as f32 * (pourcent / 100.)) as i32) as usize;

        if pourcent == 100. {
            out += &format!("{}{}{}\n", color, name_to_find[&8].repeat(self.width as usize), color).repeat(self.height as usize);
        } else {
            out += &format!("{}{}{}{}{}\n", color, name_to_find[&8].repeat((block_filled / 8) as usize), name_to_find[&(block_filled % 8)], " ".repeat(white_lines), color).repeat(self.height as usize);
        }

        out
    }

    pub fn new(height: i32, width: i32, color: Option<Color>) -> HorizontalBar {
        HorizontalBar{height, width, color: color.unwrap_or(Color::Green)}
    }
}

pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}