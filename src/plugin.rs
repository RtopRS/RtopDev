pub trait Plugin {
    fn display(&mut self, height: i32, width: i32) -> String;
    fn title(&mut self) -> String { String::from("") }

    fn on_update(&mut self) {}
    fn on_input(&mut self, _key: String) {}
}