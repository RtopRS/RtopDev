//! # Component representing a list of item
//! 
//! ## Example
//! ```rust
//! println!("TODO");
//! ```


pub struct ListView {
    rows: i32,
    cols: i32,
    items: Vec<ListItem>,
    primary_key: String,
    secondary_keys: Vec<String>,
    selected_line : i32,
    start_index: i32,
    sort_key: Option<String>,
    counter: i32,
    ordering: Option<std::cmp::Ordering>
}

impl ListView {
    pub fn new(rows: i32, cols: i32, items: &[ListItem], primary_key: String, secondary_keys: Vec<String>, sort_key: Option<String>, ordering: Option<std::cmp::Ordering>) -> ListView { // TODO: Update to add default value to some args
        let mut created_listview = ListView{rows, cols, counter: 0, items: items.to_vec(), primary_key: primary_key, secondary_keys, selected_line: 1, start_index: 0, sort_key, ordering};
        created_listview.sort();
        created_listview
    }

    /// Select the previous element if possible
    pub fn previous(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
            if self.selected_line > 1 {
                self.selected_line -= 1;
            } else {
                self.start_index -= 1;
            }
        }
    }

    /// Select the next element if possible
    pub fn next(&mut self) {
        if self.counter < self.items.len() as i32 - 1{
            self.counter += 1;
            if self.selected_line != self.rows - 1 {
                self.selected_line += 1;
            } else {
                self.start_index += 1;
            }
        }
    }

    /// Select the last element
    pub fn to_last(&mut self) {
        self.counter = self.items.len() as i32 - 1;
        self.selected_line = self.counter + 1;
        if self.selected_line > self.rows - 1 {
            self.start_index = self.counter - (self.rows - 2);
            self.selected_line = self.rows - 1;
        }
    }

    /// Select the first element
    pub fn to_first(&mut self) {
        self.counter = 0;
        self.selected_line = 1;
        self.start_index = 0;
    }

    /// Create the List and return a formatted string ready to be displayed in Rtop
    pub fn display(&mut self) -> String {
        let mut secondary_keys_len = std::collections::HashMap::new();
        for key in &self.secondary_keys {
            secondary_keys_len.insert(key.to_string(), key.len() + 2);
        }

        let mut displayed_items = &self.items[..];

        for item in displayed_items {
            for key_value in &item.data {
                if secondary_keys_len.contains_key(key_value.0) {
                    let tmp = secondary_keys_len[key_value.0];
                    if key_value.1.len() + 2 > tmp {
                        *secondary_keys_len.get_mut(key_value.0).unwrap() = key_value.1.len() + 2
                    }   
                }
            }
        }

        let mut secondary_cols = String::new();
        for key in &self.secondary_keys {
            if let Some(sort_key) = &self.sort_key {
                if key == sort_key {
                    if let Some(ordering) = self.ordering {
                        secondary_cols += &match ordering {
                            std::cmp::Ordering::Greater => format!("[[EFFECT_BOLD]]{}[[EFFECT_BOLD]]{}", key, " ".repeat(secondary_keys_len[key] - key.len())),
                            _ => format!("[[EFFECT_ITALIC]]{}[[EFFECT_ITALIC]]{}", key, " ".repeat(secondary_keys_len[key] - key.len()))
                        };
                    } else {
                        secondary_cols += &format!("{}{}", key, " ".repeat(secondary_keys_len[key] - key.len()))
                    }
                } else {
                    secondary_cols += &format!("{}{}", key, " ".repeat(secondary_keys_len[key] - key.len()))
                }
            } else {
                secondary_cols += &format!("{}{}", key, " ".repeat(secondary_keys_len[key] - key.len()))
            }
        }

        let mut tmp = 0; // TODO: rename this
        if let Some(sort_key) = &self.sort_key {
            if &self.primary_key != sort_key {
                if let Some(ordering) = self.ordering {
                    tmp = match ordering {
                        std::cmp::Ordering::Greater => 30,
                        _ => 32
                    };
                }
            }
        }
        
        let mut name_to_define = String::from(&self.primary_key); // TODO rename name_to_define
        if let Some(sort_key) = &self.sort_key {
            if &self.primary_key == sort_key {
                if let Some(ordering) = self.ordering {
                    name_to_define = match ordering {
                        std::cmp::Ordering::Greater => format!("[[EFFECT_BOLD]]{}[[EFFECT_BOLD]]", self.primary_key),
                        _ => format!("[[EFFECT_ITALIC]]{}[[EFFECT_ITALIC]]", self.primary_key)
                    };
                }
            }
        }


        let mut output_string = format!(" {}{}{}\n", name_to_define, " ".repeat(self.cols as usize - self.primary_key.len() - secondary_cols.len() - 2 + tmp), secondary_cols);
        if displayed_items.len() > (self.rows - 1) as usize {
            displayed_items = &self.items[self.start_index as usize..(self.start_index + self.rows - 1) as usize];
        }
        let mut i = 1;

        for item in displayed_items {
            let name = item.name.chars().into_iter().take(self.cols as usize - secondary_cols.len() - 4 + tmp).collect::<String>();
            if i == self.selected_line {
                output_string += &format!(" [[EFFECT_REVERSE]]{}{}", name," ".repeat(self.cols as usize - name.chars().count() - secondary_cols.len() - 2 + tmp))
            } else {
                output_string += &format!(" {}{}", name, " ".repeat(self.cols as usize - name.chars().count() - secondary_cols.len() - 2 + tmp));
            }

            for col in &self.secondary_keys {
                let len = secondary_keys_len[col];

                if item.data.contains_key(col) {
                    output_string += &format!("{}{}", item.data[col], " ".repeat(len - item.data[col].len()));
                } else {
                    output_string += &format!("{}", " ".repeat(len));
                }
            }

            if i == self.selected_line {
                output_string += "[[EFFECT_REVERSE]]";
            }
            output_string += "\n";

            i += 1;
        }   
        output_string
    }

    /// Resize the ListView
    pub fn resize(&mut self, height: i32, width: i32) {
        self.rows = height;
        self.cols = width;
        if self.selected_line > self.rows - 1 {
            self.selected_line = self.rows - 1;
            self.start_index = self.counter - (self.rows - 2);
        }
    }

    /// Update the list of ListItem contained in the ListView
    pub fn update_items(&mut self, items: &[ListItem]) {
        if items.len() < self.counter as usize + 1 {
            self.start_index -= self.counter + 1 - items.len() as i32;
            self.counter -= self.counter + 1 - items.len() as i32;
        }
        self.items = items.to_vec();

        self.sort();
    }

    /// Return the current selected ListItem
    pub fn select(&self) -> &ListItem {
        &self.items[self.counter as usize]
    }

    /// Sort the ListView items by the provided key
    pub fn sort_by(&mut self, key: Option<String>, ordering: Option<std::cmp::Ordering>) {
        self.sort_key = key;
        self.ordering = ordering;

        self.sort();
    }

    fn sort(&mut self) {
        if let (Some(sort_key), Some(ordering)) = (&self.sort_key, self.ordering) {
            if sort_key != &self.primary_key {
                self.items.sort_by(|a, b| (human_sort::compare(&a.data[sort_key].to_lowercase(), &b.data[sort_key].to_lowercase())));
            } else {
                self.items.sort_by(|a, b| (human_sort::compare(&a.name.to_lowercase(), &b.name.to_lowercase())));
            }
    
            if ordering == std::cmp::Ordering::Greater {
                self.items.reverse();
            }
        }
    }
}

/// Represent an item of a ListView
#[derive(Clone)]
pub struct ListItem {
    pub name: String,
    pub data: std::collections::HashMap<String, String>
}

impl ListItem {
    pub fn new(name: &str, data: &std::collections::HashMap<String, String>) -> ListItem {
        ListItem{name: name.to_string(), data: data.clone()}
    }
}