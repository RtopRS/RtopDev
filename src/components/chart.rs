//! # Components representing a Graph
//!
//! ## Example
//! ```rust
//! use rtop_dev::components::chart::Chart;
//!
//!
//! let data: Vec<i32> = vec!(20, 15, 14, 20, 8, 0, 9);
//! let mut chart = Chart::new(0, 0, Some(20), Some(true), Some(String::from("Out of 20")));
//! chart.resize(50, 25); // Resize the chart with a width of 50 and a height of 25 cells
//! let result = chart.display(&data);
//! ```

use std::fmt::Write;

/// Represent a sheet of data in the form of a graph
pub struct Chart {
    /// Represent the width of the chart in cells
    pub cols: i32,

    /// Represent the height of the chart in cells
    pub rows: i32,

    /// Define if the chart should have or not the current value displayed at top of it
    pub show_unit: bool,

    /// Define the max value to display a progress of 100% on the chart
    pub higher_value: i32,

    /// Define the suffix displayed after the current value if it displayed
    pub unit_suffix: String,
}

impl Chart {
    /// # Create the chart and return a formatted string ready to be displayed in Rtop
    /// ## Arguments
    /// * `percents` - List of data used to create the graph
    pub fn display(&self, percents: &[i32]) -> String {
        let mut data = percents.to_vec();
        if percents.len() >= (self.cols * 2) as usize {
            data = percents[percents.len() - (self.cols * 2) as usize..].to_vec();
        }
        data.reverse();

        let space_to_add = self.cols as f32 - (data.len() as f32 / 2.);
        let mut chart_chars: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

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

        let mut graph = String::new();

        let mut graph_rows = self.rows;
        if self.show_unit {
            graph_rows = self.rows - 1;
        }

        let mut final_graph = String::new();
        if !data.is_empty() {
            if self.show_unit {
                writeln!(
                    &mut final_graph,
                    "{}{}{}",
                    " ".repeat(
                        (self.cols
                            - data[0].to_string().chars().count() as i32
                            - self.unit_suffix.chars().count() as i32)
                            as usize
                    ),
                    data[0],
                    self.unit_suffix
                )
                .unwrap();
            }

            let mut tmp = vec![];

            for row in 0..graph_rows {
                let mut i = 0;
                while i < data.len() {
                    let percent_one = data[i];
                    let mut tmp_one =
                        percent_one as f32 / self.higher_value as f32 * self.rows as f32 * 4.;
                    if tmp_one < 1. {
                        tmp_one = 1.;
                    }
                    let mut full_block_to_add_one = tmp_one as i32 - (4 * row);

                    let mut percent_two = 0;
                    let mut tmp_two =
                        percent_two as f32 / self.higher_value as f32 * self.rows as f32 * 4.;
                    let mut full_block_to_add_two = tmp_two as i32 - (4 * row);

                    if i as usize + 1 < data.len() {
                        percent_two = data[i + 1];
                        tmp_two =
                            percent_two as f32 / self.higher_value as f32 * self.rows as f32 * 4.;
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

                    graph = format!(
                        "{}{}",
                        graph,
                        chart_chars
                            [&format!("{}{}", full_block_to_add_two, full_block_to_add_one)[..]]
                    );
                    i += 2
                }

                tmp.push(graph);
                graph = String::new();
            }

            tmp.reverse();
            for line in tmp {
                writeln!(
                    &mut final_graph,
                    "{}{}",
                    " ".repeat(space_to_add as usize),
                    line.chars().rev().collect::<String>()
                )
                .unwrap();
            }
        }

        final_graph
    }

    /// # Resize the Chart
    /// ## Arguments
    /// * `rows` - The new height of the Chart
    /// * `cols` - The new width of the Chart
    pub fn resize(&mut self, cols: i32, rows: i32) {
        self.cols = cols;
        self.rows = rows;
    }

    /// # Create a new chart
    /// ## Arguments
    /// * `cols` - The width of the Chart in cells
    /// * `rows` - The height of the Chart in cells
    /// * `higher_value` - *`Optional`* - If supplied, set the value that a number in the data must be to be concidered as max. Otherwise, it will be 100.<br>
    /// For example, setting it to 50 and display number 40 in the Chart will display a line at 80% of the Chart's height
    /// * `show_unit` - *`Optional`* - If supplied, select if the current data value should be shown on the top right corner of the Chart
    /// * `unit_suffix` - *`Optional`* - If supplied, set the suffix displayed after the current data.<br>
    /// **⚠️ Work only if `show_unit` is set to True. Otehrwise, the option has no affect.**
    pub fn new(
        cols: i32,
        rows: i32,
        higher_value: Option<i32>,
        show_unit: Option<bool>,
        unit_suffix: Option<String>,
    ) -> Self {
        Self {
            cols,
            rows,
            higher_value: higher_value.unwrap_or(100),
            show_unit: show_unit.unwrap_or(false),
            unit_suffix: unit_suffix.unwrap_or_else(|| String::from("%")),
        }
    }
}
