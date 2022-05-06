<h1 align="center">
  Rtop Dev
</h1>
<p align="center">
    <a href="https://www.rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
    </a>
    <a href="https://github.com/RTopRS/Rtop">
        <img src="https://img.shields.io/badge/Git-F05032?style=for-the-badge&logo=git&logoColor=white" alt="Use git">
    </a>
</p>
<h3 align="center">
    <strong>Developement librairy for making Rtop's Plugin</strong>
</h3>

## Quick Example
Rtop let you create plugin for adding custom widgets.
Firstly, create a new librairy project 
```
cargo new --lib MyPlugin
```
After that, update your `Cargo.toml` file. It should look like this
```toml
[package]
name = "MyPlugin"
version = "0.1.0"
edition = "2021"

[dependencies]
rtop_dev = "0.1.2"

[lib]
name = "my_plugin"
crate-type = ["dylib"]
```
Then, edit your `src/lib.rs` for having somethings like this.
```rust
struct FooWidget {}

impl rtop_dev::plugin::Plugin for FooWidget {
    fn display(&mut self, _height: i32, _width: i32) -> String {
        String::from("Hello World RTop!")
    }
}

#[no_mangle]
pub extern "Rust" fn init_foo() -> (Box<dyn rtop_dev::plugin::Plugin>, bool) {
    (Box::new(FooWidget{}), false)
}

```
For building your lib, simply run 
```
cargo build --lib --release
```
Your compilated plugin should be located here `target/release/libmy_plugin.so`

**Remember these things, For each widget you want to create, you must make a function called `init_{WIDGET}` which return a `Box<dyn rtop_dev::plugin::Plugin` and a `bool` that defines if your widget should receive input from the user or not.
Don't forget to add `#[no_mangle]` in front of each `init` function. Otherwise, it will not be exported**


## Contributors
[<img width="45" src="https://avatars.githubusercontent.com/u/63391793?v=4" alt="SquitchYT">](https://github.com/SquitchYT)

## License
**[RTop](https://github.com/RTopRS/Rtop) | [Mozilla Public License 2.0](https://github.com/RTopRS/Rtop/blob/main/LICENSE)**