//! Module containing all essentials tools to create a Rtop's plugin

/// Trait representing a Widget. Must be implemented on your struct to declare it as a Widget.
pub trait Widget {
    /// Called every time Rtop need to update the widget display
    fn display(&mut self, height: i32, width: i32) -> String;

    /// Called every time `display` is called, Change the title of the window if None is not returned.
    fn title(&mut self) -> Option<String> { None }

    /// `Event` -- Called three time per second, even if the widget is not focused or not visible for the user.
    fn on_update(&mut self) {}

    /// `Event` -- Called when an user input somethings when the widget is focused
    /// <br>
    /// ```⚠️ Only called if input is defined to true when creating the widget```
    fn on_input(&mut self, _key: String) {}
}