//! # Component representing a list of item
//!
//! ## Example
//! ```rust
//! use rtop_dev::components::listview::ListView;
//! use rtop_dev::components::listview::ListItem;
//! use rtop_dev::components::listview::Ordering;
//!
//!
//! let mut items: Vec<ListItem> = vec!();
//!
//! for i in 0..5 {
//!
//!     let mut item_data = std::collections::HashMap::new();
//!     item_data.insert(String::from("key1"), format!("Value{}", i));
//!
//!     items.push(
//!         ListItem::new(&format!("Item {}", i), &item_data)
//!     );
//! }
//!
//! let mut listview = ListView::new(50, 25, &items, String::from("Name"), vec!(String::from("key1")), None, None);
//!
//! listview.display();
//!
//! // ...
//!
//! listview.sort_by(Some(String::from("key1")), Some(Ordering::Inversed)); // Sort by "key1" in descending order
//! ```

use std::fmt::Write;

/// Display list of [`ListItem`] with table header, ordering and other stuffs
pub struct ListView {
    cols: i32,
    rows: i32,
    items: Vec<ListItem>,
    primary_key: String,
    secondary_keys: Vec<String>,
    selected_line: i32,
    start_index: i32,
    sort_key: Option<String>,
    counter: i32,
    ordering: Option<Ordering>,
}

impl ListView {
    /// # Create a new `ListView`
    /// ## Arguments
    /// * `cols` - Represent the width of the chart in cells
    /// * `rows` - Represent the height of the chart in cells
    /// * `items` - List of [`ListItem`] to be displayed in the `ListView`
    /// * `primary_key` - Table name, displayed on the left of the `ListView` header, the value will be filled with the `name` filed of the [`ListItem`]
    /// * `secondary_keys` - List of all secondary columns, displayed on the right of the `ListView` header, the value will be filled with the value associated with the column name in the `data` field of the [`ListItem`]
    /// * `sort_key` - *`Optional`* - If supplied, the items will be ordered by the value of the selected column.<br>
    /// **⚠️ The `sort_key` must be one of the `secondary_keys` or the `primary_key`, otherwise, no sort will be applied**
    /// * `ordering` - *`Optional`* - If supplied, change the ordering direction
    pub fn new(
        cols: i32,
        rows: i32,
        items: &[ListItem],
        primary_key: String,
        secondary_keys: Vec<String>,
        sort_key: Option<String>,
        ordering: Option<Ordering>,
    ) -> Self {
        let mut created_listview = Self {
            rows,
            cols,
            counter: 0,
            items: items.to_vec(),
            primary_key,
            secondary_keys,
            selected_line: 1,
            start_index: 0,
            sort_key,
            ordering,
        };
        created_listview.sort();
        created_listview
    }

    /// # Select the previous element if possible
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

    /// # Select the next element if possible
    pub fn next(&mut self) {
        if self.counter < self.items.len() as i32 - 1 {
            self.counter += 1;
            if self.selected_line == self.rows - 1 {
                self.start_index += 1;
            } else {
                self.selected_line += 1;
            }
        }
    }

    /// # Select the last element
    pub fn to_last(&mut self) {
        self.counter = self.items.len() as i32 - 1;
        self.selected_line = self.counter + 1;
        if self.selected_line > self.rows - 1 {
            self.start_index = self.counter - (self.rows - 2);
            self.selected_line = self.rows - 1;
        }
    }

    /// # Select the first element
    pub fn to_first(&mut self) {
        self.counter = 0;
        self.selected_line = 1;
        self.start_index = 0;
    }

    /// # Create the List and return a formatted String ready to be displayed in Rtop
    pub fn display(&mut self) -> String {
        let mut secondary_keys_len = std::collections::HashMap::new();
        for key in &self.secondary_keys {
            secondary_keys_len.insert(key.to_string(), key.len() + 2);
        }

        let mut displayed_items = &*self.items;

        for item in displayed_items {
            for key_value in &item.data {
                if secondary_keys_len.contains_key(key_value.0) {
                    let tmp = secondary_keys_len[key_value.0];
                    if key_value.1.len() + 2 > tmp {
                        *secondary_keys_len.get_mut(key_value.0).unwrap() = key_value.1.len() + 2;
                    }
                }
            }
        }

        let mut secondary_cols = String::new();
        for key in &self.secondary_keys {
            if let Some(sort_key) = &self.sort_key {
                if key == sort_key {
                    if let Some(ordering) = &self.ordering {
                        secondary_cols += &if ordering == &Ordering::Default {
                            format!(
                                "[[EFFECT_BOLD]]{}[[EFFECT_BOLD]]{}",
                                key,
                                " ".repeat(secondary_keys_len[key] - key.len())
                            )
                        } else {
                            format!(
                                "[[EFFECT_ITALIC]]{}[[EFFECT_ITALIC]]{}",
                                key,
                                " ".repeat(secondary_keys_len[key] - key.len())
                            )
                        };
                    } else {
                        write!(
                            &mut secondary_cols,
                            "{}{}",
                            key,
                            " ".repeat(secondary_keys_len[key] - key.len())
                        )
                        .unwrap();
                    }
                } else {
                    write!(
                        &mut secondary_cols,
                        "{}{}",
                        key,
                        " ".repeat(secondary_keys_len[key] - key.len())
                    )
                    .unwrap();
                }
            } else {
                write!(
                    &mut secondary_cols,
                    "{}{}",
                    key,
                    " ".repeat(secondary_keys_len[key] - key.len())
                )
                .unwrap();
            }
        }

        let mut tmp = 0; // TODO: rename this
        if let Some(sort_key) = &self.sort_key {
            if &self.primary_key != sort_key && self.secondary_keys.contains(sort_key) {
                if let Some(ordering) = &self.ordering {
                    tmp = match ordering {
                        Ordering::Default => 30,
                        Ordering::Inversed => 32,
                    };
                }
            }
        }

        let mut name_to_define = String::from(&self.primary_key); // TODO rename name_to_define
        if let Some(sort_key) = &self.sort_key {
            if &self.primary_key == sort_key {
                if let Some(ordering) = &self.ordering {
                    name_to_define = if ordering == &Ordering::Default {
                        format!("[[EFFECT_BOLD]]{}[[EFFECT_BOLD]]", self.primary_key)
                    } else {
                        format!("[[EFFECT_ITALIC]]{}[[EFFECT_ITALIC]]", self.primary_key)
                    };
                }
            }
        }

        let mut output_string = format!(
            "{}{}{}\n",
            name_to_define,
            " ".repeat(self.cols as usize - self.primary_key.len() - secondary_cols.len() + tmp),
            secondary_cols
        );
        if displayed_items.len() > (self.rows - 1) as usize {
            displayed_items =
                &self.items[self.start_index as usize..(self.start_index + self.rows - 1) as usize];
        }
        let mut i = 1;

        for item in displayed_items {
            let name = item
                .name
                .chars()
                .into_iter()
                .take(self.cols as usize - secondary_cols.len() + tmp)
                .collect::<String>();
            if i == self.selected_line {
                write!(
                    &mut output_string,
                    "[[EFFECT_REVERSE]]{}{}",
                    name,
                    " ".repeat(
                        self.cols as usize - name.chars().count() - secondary_cols.len() + tmp
                    )
                )
                .unwrap();
            } else {
                write!(
                    &mut output_string,
                    "{}{}",
                    name,
                    " ".repeat(
                        self.cols as usize - name.chars().count() - secondary_cols.len() + tmp
                    )
                )
                .unwrap();
            }

            for col in &self.secondary_keys {
                let len = secondary_keys_len[col];

                if item.data.contains_key(col) {
                    write!(
                        &mut output_string,
                        "{}{}",
                        item.data[col],
                        " ".repeat(len - item.data[col].len())
                    )
                    .unwrap();
                } else {
                    output_string.push_str(&" ".repeat(len));
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

    /// # Resize the `ListView`
    /// ## Arguments
    /// * `rows` - The new height of the `ListView`
    /// * `cols` - The new width of the `ListView`
    pub fn resize(&mut self, rows: i32, cols: i32) {
        self.rows = rows;
        self.cols = cols;
        if self.selected_line > self.rows - 1 {
            self.selected_line = self.rows - 1;
            self.start_index = self.counter - (self.rows - 2);
        }
    }

    /// # Update the list of `ListItem` contained in the `ListView`
    /// ## Arguments
    /// * `items` - New list of items to be displayed in the `ListView`<br>
    /// **⚠️ The `items` must include the same `primary_key` and `secondary_keys` as the previous set of [`ListItem`]**
    pub fn update_items(&mut self, items: &[ListItem]) {
        if items.len() < self.counter as usize + 1 {
            self.start_index -= self.counter + 1 - items.len() as i32;
            self.counter -= self.counter + 1 - items.len() as i32;
        }
        self.items = items.to_vec();

        self.sort();
    }

    /// # Return the current selected `ListItem`
    pub fn select(&self) -> &ListItem {
        &self.items[self.counter as usize]
    }

    /// # Update the way of sorting and sort the items
    /// ## Arguments
    /// * `key` - *`Optional`* - If provided, update the sorting key
    /// * `ordering` - *`Optional`* - If provided, update the sorting order
    pub fn sort_by(&mut self, key: Option<String>, ordering: Option<Ordering>) {
        self.sort_key = key;
        self.ordering = ordering;

        self.sort();
    }

    fn sort(&mut self) {
        if let (Some(sort_key), Some(ordering)) = (&self.sort_key, &self.ordering) {
            if sort_key == &self.primary_key {
                self.items.sort_by(|a, b| {
                    human_sort::compare(&b.name.to_lowercase(), &a.name.to_lowercase())
                });
            } else {
                self.items.sort_by(|a, b| {
                    human_sort::compare(
                        &b.data
                            .get(sort_key)
                            .unwrap_or(&String::new())
                            .to_lowercase(),
                        &a.data
                            .get(sort_key)
                            .unwrap_or(&String::new())
                            .to_lowercase(),
                    )
                });
            }

            if *ordering == Ordering::Inversed {
                self.items.reverse();
            }
        }
    }
}

/// Represent an item of a [`ListView`]
#[derive(Clone)]
pub struct ListItem {
    /// Represent the "ID" of the item, it will be used as the value of the `primary_key` when displayed in a [ListView]
    pub name: String,
    /// A collection of key / value pair. Each pair will be used and displayed in the secondary keys column of the [ListView]
    pub data: std::collections::HashMap<String, String>,
}

impl ListItem {
    /// # Create a new `ListItem`
    /// ## Arguments
    /// * `name` - The name of the item, used as the `primary_key` value in the [`ListView`]
    /// * `data` - Pair of key / value, each value associated with a key will be used in the corresponding cols corresponding to his key in the [`ListView`]
    pub fn new(name: &str, data: &std::collections::HashMap<String, String>) -> Self {
        Self {
            name: String::from(name),
            data: data.clone(),
        }
    }
}

/// Represent the Sort Order of a [`ListView`]
/// The default ordering is the alphabetical order for String and descending for the number
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub enum Ordering {
    Default,
    Inversed,
}
