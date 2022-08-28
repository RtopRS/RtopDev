pub struct VerticalBar {
    pub height: i32,
    pub width: i32
}
impl VerticalBar {
    pub fn display(&self, pourcent: f32) -> String {
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
            let tmp = &format!("[[EFFECT_COLOR_GREEN_BLACK]]{}[[EFFECT_COLOR_GREEN_BLACK]]\n", "█".repeat(self.width as usize));
            out += &tmp.repeat(self.height as usize);
        } else {
            let block_filled = (self.height as f32 * 8. * (pourcent / 100.)) as i32;
            let white_lines = (self.height - 1 - (self.height as f32 * (pourcent / 100.)) as i32) as usize;

            out += &format!("{}", format!("[[EFFECT_COLOR_GREEN_BLACK]]{}[[EFFECT_COLOR_GREEN_BLACK]]\n", " ".repeat(self.width as usize)).repeat(white_lines));
            out += &format!("[[EFFECT_COLOR_GREEN_BLACK]]{}[[EFFECT_COLOR_GREEN_BLACK]]\n", name_to_find[&(block_filled % 8)].repeat(self.width as usize));
            out += &format!("{}", format!("[[EFFECT_COLOR_GREEN_BLACK]]{}[[EFFECT_COLOR_GREEN_BLACK]]\n", "█".repeat(self.width as usize)).repeat(self.height as usize - white_lines - 1));
        }

        out
    }

    pub fn new(height: i32, width: i32) -> VerticalBar {
        VerticalBar{height, width}
    }
}

pub struct HorizontalBar {
    pub height: i32,
    pub width: i32
}
impl HorizontalBar {
    pub fn display(&self, pourcent: f32) -> String {
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
            out += &format!("[[EFFECT_COLOR_GREEN_BLACK]]{}[[EFFECT_COLOR_GREEN_BLACK]]\n", name_to_find[&8].repeat(self.width as usize)).repeat(self.height as usize);
        } else {
            out += &format!("[[EFFECT_COLOR_GREEN_BLACK]]{}{}{}[[EFFECT_COLOR_GREEN_BLACK]]\n", name_to_find[&8].repeat((block_filled / 8) as usize), name_to_find[&(block_filled % 8)], " ".repeat(white_lines)).repeat(self.height as usize);
        }

        

        out
    }

    pub fn new(height: i32, width: i32) -> HorizontalBar {
        HorizontalBar{height, width}
    }
}