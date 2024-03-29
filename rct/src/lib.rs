//! [![github]](https://github.com/disco07/rct)&ensp;[![crates-io]](https://crates.io/crates/rct)&ensp;[![docs-rs]](https://docs.rs/rct/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! # &emsp;A CLI Table Output for Rust 🦀 projects.
//!
//! ## Usage
//!
//! Add `rct` in your `dependencies` section
//!
//! ```toml
//! [dependencies]
//! rct = "0.2.1"
//! ```
//!
//! ### Basic usage
//! ```rust
//!     use rct::cell::ICell;
//!     use rct::table::Table;
//!
//!     let mut table = Table::new();
//!
//!     table
//!         .add_header(vec!["ID".cell(), "Title".cell(), "Price €".cell()])
//!         .add_row(vec![1.cell(),"Harry \nPotter".cell(), "14.87".cell()])
//!         .add_row(vec![2.cell(),"Spider-man".cell(),"18.80".cell()])
//!         .add_row(vec![3.cell(), "Avenger".cell(), "18.50".cell()]);
//!     table.view();
//!
//! ```
//!
//! ```markdown
//! ╔════╤════════════╤═════════╗
//! ║ ID │ Title      │ Price € ║
//! ╟────┼────────────┼─────────╢
//! ║ 1  │ Harry      │ 14.87   ║
//! ║    │ Potter     │         ║
//! ╟────┼────────────┼─────────╢
//! ║ 2  │ Spider-man │ 18.80   ║
//! ╟────┼────────────┼─────────╢
//! ║ 3  │ Avenger    │ 18.50   ║
//! ╚════╧════════════╧═════════╝
//! ```
//!
//! ### Add styles
//! ```
//!     use rct::cell::ICell;
//!     use rct::table::Table;
//!     use rct::styles::color::{Colorizer, Font};
//!
//!     let mut table = Table::new();
//!
//!     table
//!         .add_header(vec!["ID".cell(), "Title".cell(), "Price €".cell()])
//!         .add_row(vec![1.cell(),"Harry \nPotter".cell().color("#ff0000"), "14.87".cell()])
//!         .add_row(vec![2.cell(),"Spider-man".cell(),"18.80".cell()])
//!         .add_row(vec![3.cell(), "Avenger".cell(), "18.50".cell().font(Font::Italic)]);
//!     table.view();
//!
//! ```
//! ![Styled Table](../images/style_table.png)
//!
//! ### Use derive macro
//! `#[derive(ToTable)]` can be used to display a `Vec` or slice of `struct` as a table.
//! ```rust, no_run
//! use rct::ToTable;
//!
//! #[derive(ToTable)]
//! struct Movies {
//!     #[table(rename = "ID")]
//!     id: u32,
//!     #[table(rename = "Title")]
//!     title: String,
//!     #[table(rename = "Price €")]
//!     price: f32,
//! }
//!
//!     let movies = [
//!         Movies {
//!             id: 1,
//!             title: "Harry \nPotter".to_string(),
//!             price: 14.87,
//!         },
//!         Movies {
//!             id: 2,
//!             title: "Spider-man".to_string(),
//!             price: 18.80,
//!         },
//!     ];
//!
//!     let table = movies.into_iter().to_table();
//!
//!     println!("{}", table);
//!
//! ```
//!
//! ## Features
//!
//! - `derive`: Enables derive macro for creating tables using structs.

pub mod cell;
pub mod row;
pub mod styles;
pub mod table;

#[cfg(feature = "derive")]
#[cfg_attr(feature = "doc", doc(cfg(feature = "derive")))]
pub use rct_derive::ToTable;

pub use self::{
    cell::{Cell, ICell},
    row::Row,
    table::Table,
};
