//! Module containing all essentials tools to create a Rtop's plugin

/// Trait representing a Widget. Must be implemented on your struct to declare it as a Widget.
pub trait Widget {
    /// Called every time Rtop need to update the plugin display
    fn display(&mut self, height: i32, width: i32) -> String;

    /// Called every time `display` is called, if the string returned is not null, Rtop will replace the widget title with the newly provided string
    fn title(&mut self) -> String { String::from("") }

    /// `Event` -- Called three time per second, even if the plugin is not focused or not visible for the user.
    fn on_update(&mut self) {}

    /// `Event` -- Called when an user input somethings when the plugin is focused
    /// <br>
    /// ```⚠️ Only called if input is defined to true when creating the plugin```
    fn on_input(&mut self, _key: String) {}
}