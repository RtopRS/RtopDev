//! Rtop SDK to create plugin, made with ❤️ for you.
//! ## Example Usage
//! Firstly, create a new project for your plugin!
//! ```
//! cargo new --lib MyPlugin
//! ```
//! After that, update your `Cargo.toml` file. It should look like that:
//! ```toml
//! [package]
//! name = "MyPlugin"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! rtop_dev = "^1.0.0"
//!
//! [lib]
//! name = "my_plugin"
//! crate-type = ["cdylib"]
//!
//! [profile.release]
//! codegen-units = 1
//! panic = "abort"
//! strip = true
//! lto = true
//! ```
//! Then, edit your `src/lib.rs` to have somethings like this:
//! ```rust
//! struct FooWidget {}
//!
//! impl rtop_dev::plugin::Plugin for FooWidget {
//!     fn display(&mut self, _height: i32, _width: i32) -> String {
//!         String::from("Hello World RTop!")
//!     }
//! }
//!
//! #[no_mangle]
//! pub extern "Rust" fn init_foo() -> (Box<dyn rtop_dev::plugin::Plugin>, bool) {
//!     (Box::new(FooWidget{}), false)
//! }

pub mod components;
pub mod widget;
