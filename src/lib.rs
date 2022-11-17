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

#![allow(
    clippy::implicit_return,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_docs_in_private_items,
    clippy::separated_literal_suffix,
    clippy::missing_inline_in_public_items,
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::mod_module_files,
    clippy::else_if_without_else,
    clippy::unused_self,
    clippy::cast_precision_loss,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::exit,
    clippy::too_many_lines,
    clippy::exhaustive_structs,
    clippy::single_char_lifetime_names,
    clippy::integer_division,
    clippy::separated_literal_suffix,
    clippy::else_if_without_else,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation,
    clippy::integer_arithmetic,
    clippy::default_numeric_fallback,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::float_arithmetic,
    clippy::pattern_type_mismatch,
    clippy::as_conversions
)]
#![deny(
    clippy::needless_return,
    clippy::str_to_string,
    clippy::implicit_clone,
    clippy::needless_pass_by_value,
    clippy::semicolon_if_nothing_returned,
    clippy::wildcard_imports,
    clippy::single_match_else,
    clippy::single_match,
    clippy::let_underscore_drop,
    clippy::expect_used,
    clippy::suboptimal_flops,
    clippy::redundant_else
)]

pub mod components;
pub mod widget;
