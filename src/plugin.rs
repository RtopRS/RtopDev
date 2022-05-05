pub trait Plugin {
    fn display(&mut self, height: i32, width: i32) -> String;

    fn update(&mut self) {} // called every time the loop for ac tualize data is done
    fn refresh_rate(&self) -> i32 { 333 } // set the time between two refresh loop execution
    fn on_input(&mut self, _key: String) { }
}