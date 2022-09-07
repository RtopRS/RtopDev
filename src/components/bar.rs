//! # Components representing a ProgressBar
//! 
//! ## Example
//! ```rust
//! println!("TODO");
//! ```

use std::fmt::Write;

pub struct VerticalBar {
    rows: i32,
    cols: i32,
    color: Color
}
impl VerticalBar {
    /// # Return a formatted String ready to be display in rtop
    /// ## Arguments
    /// 
    /// * `pourcent` - Represent the progress of the VerticlaBar<br>
    /// **⚠️ The `pourcent` must be between 0.0 and 100.0**
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
        let mut bar_parts = std::collections::HashMap::new();
        bar_parts.insert(0, " ");
        bar_parts.insert(1, "▁");
        bar_parts.insert(2, "▂");
        bar_parts.insert(3, "▃");
        bar_parts.insert(4, "▄");
        bar_parts.insert(5, "▅");
        bar_parts.insert(6, "▆");
        bar_parts.insert(7, "▇");
        bar_parts.insert(8, "█");

        if pourcent == 100. {
            out += &format!("{}{}{}\n", color, "█".repeat(self.width as usize), color).repeat(self.height as usize);
        } else {
            let block_filled = (self.height as f32 * 8. * (pourcent / 100.)) as i32;
            let white_lines = (self.height - 1 - (self.height as f32 * (pourcent / 100.)) as i32) as usize;

            out += &format!("{}{}{}\n", color, " ".repeat(self.width as usize), color).repeat(white_lines);
            writeln!(&mut out, "{}{}{}", color, bar_parts[&(block_filled % 8)].repeat(self.width as usize), color).unwrap();
            out += &format!("{}{}{}\n", color, "█".repeat(self.width as usize), color).repeat(self.height as usize - white_lines - 1);
        }

        out
    }

    /// # Create a new VerticalBar
    /// ## Arguments
    /// 
    /// * `cols` - Represent the width of the bar in cells
    /// * `rows` - Represent the height of the bar in cells
    /// * `color` - *`Optional`* - If supplied, set the color of the progress of the bar, otehrwise, it will be green
    pub fn new(rows: i32, cols: i32, color: Option<Color>) -> VerticalBar {
        VerticalBar{rows, cols, color: color.unwrap_or(Color::Green)}
    }
}

pub struct HorizontalBar {
    rows: i32,
    cols: i32,
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
        let mut bar_parts = std::collections::HashMap::new();
        bar_parts.insert(0, " ");
        bar_parts.insert(1, "▏");
        bar_parts.insert(2, "▎");
        bar_parts.insert(3, "▍");
        bar_parts.insert(4, "▌");
        bar_parts.insert(5, "▋");
        bar_parts.insert(6, "▊");
        bar_parts.insert(7, "▉");
        bar_parts.insert(8, "█");

        let block_filled = (self.width as f32 * 8. * (pourcent / 100.)) as i32;
        let white_lines = (self.width - 1 - (self.width as f32 * (pourcent / 100.)) as i32) as usize;

        if pourcent == 100. {
            out += &format!("{}{}{}\n", color, bar_parts[&8].repeat(self.width as usize), color).repeat(self.height as usize);
        } else {
            out += &format!("{}{}{}{}{}\n", color, bar_parts[&8].repeat((block_filled / 8) as usize), bar_parts[&(block_filled % 8)], " ".repeat(white_lines), color).repeat(self.height as usize);
        }

        out
    }

    /// # Create a new HorizontalBar
    /// ## Arguments
    /// 
    /// * `cols` - Represent the width of the bar in cells
    /// * `rows` - Represent the height of the bar in cells
    /// * `color` - *`Optional`* - If supplied, set the color of the progress of the bar, otehrwise, it will be green
    pub fn new(rows: i32, cols: i32, color: Option<Color>) -> HorizontalBar {
        HorizontalBar{rows, cols, color: color.unwrap_or(Color::Green)}
    }
}

/// Represent a Color of progress for the Bar
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}