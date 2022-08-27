pub struct Chart {
    pub cols: i32,
    pub rows: i32,
    pub show_percent: bool
}

impl Chart {
    pub fn display(&self, percents: &[i32]) -> String {
        let mut data = percents.to_vec();
        if percents.len() >= ((self.cols * 2) - 1) as usize {
            data = percents[percents.len() + 2 - (self.cols * 2) as usize..].to_vec();
        }
        data.reverse();

        let space_to_add = self.cols as f32 - (data.len() as f32 / 2.);
        let mut chart_chars: std::collections::HashMap<&str, String> = std::collections::HashMap::new();

        chart_chars.insert("10", String::from("⡀"));
        chart_chars.insert("20", String::from("⡄"));
        chart_chars.insert("30", String::from("⡆"));
        chart_chars.insert("40", String::from("⡇"));

        chart_chars.insert("01", String::from("⢀"));
        chart_chars.insert("02", String::from("⢠"));
        chart_chars.insert("03", String::from("⢰"));
        chart_chars.insert("04", String::from("⢸"));

        chart_chars.insert("11", String::from("⣀"));
        chart_chars.insert("21", String::from("⣄"));
        chart_chars.insert("31", String::from("⣆"));
        chart_chars.insert("41", String::from("⣇"));

        chart_chars.insert("12", String::from("⣠"));
        chart_chars.insert("22", String::from("⣤"));
        chart_chars.insert("32", String::from("⣦"));
        chart_chars.insert("42", String::from("⣧"));

        chart_chars.insert("13", String::from("⣰"));
        chart_chars.insert("23", String::from("⣴"));
        chart_chars.insert("33", String::from("⣶"));
        chart_chars.insert("43", String::from("⣷"));

        chart_chars.insert("14", String::from("⣸"));
        chart_chars.insert("24", String::from("⣼"));
        chart_chars.insert("34", String::from("⣾"));
        chart_chars.insert("44", String::from("⣿"));

        chart_chars.insert("01", String::from("⢀"));
        chart_chars.insert("02", String::from("⢠"));
        chart_chars.insert("03", String::from("⢰"));
        chart_chars.insert("04", String::from("⢸"));
        chart_chars.insert("00", String::from(" "));

        let mut graph: String = String::new();

        let mut graph_rows = self.rows;
        if self.show_percent {
            graph_rows = self.rows - 1;
        }

        for row in 0..graph_rows {
            let mut i = 0;
            while i < data.len() {
                let percent_one = data[i as usize];
                let mut tmp_one = percent_one as f32 / 100. * self.rows as f32 * 4.;
                if tmp_one < 1. {
                    tmp_one = 1.;
                }
                let mut full_block_to_add_one = tmp_one as i32 - (4 * row);

                let mut percent_two = 0;
                let mut tmp_two = percent_two as f32 / 100. * self.rows as f32 * 4.;
                let mut full_block_to_add_two = tmp_two as i32 - (4 * row);

                if i as usize + 1 < data.len() {    
                    percent_two = data[i as usize + 1];
                    tmp_two = percent_two as f32 / 100. * self.rows as f32 * 4.;
                    if tmp_two < 1. {
                        tmp_two = 1.;
                    }
                    full_block_to_add_two = tmp_two as i32 - (4 * row);
                }

                if full_block_to_add_one < 0 {
                    full_block_to_add_one = 0;
                }
                if full_block_to_add_two < 0 {
                    full_block_to_add_two = 0;
                }

                if full_block_to_add_one > 4 {
                    full_block_to_add_one = 4;
                }
                if full_block_to_add_two > 4 {
                    full_block_to_add_two = 4;
                }

                graph = format!("{}{}", graph, chart_chars[&format!("{}{}",full_block_to_add_two, full_block_to_add_one).to_string()[..]]);
                i += 2
            }

            graph = format!("\n{}{}", graph, " ".repeat(space_to_add as usize));
        }

        if self.show_percent && !data.is_empty() {
            graph = format!("{}%{}{}", graph, data[0].to_string().chars().rev().collect::<String>()," ".repeat((self.cols - data[0].to_string().chars().count() as i32 - 2) as usize));
        }

        graph.chars().rev().collect::<String>()
    }

    pub fn resize(&mut self, cols: i32, rows: i32) {
        self.cols = cols;
        self.rows = rows;
    }
}