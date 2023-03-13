[![Rust](https://github.com/disco07/rct/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/disco07/rct/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/rct.svg)](https://crates.io/crates/rct)
[![Documentation](https://docs.rs/rct/badge.svg)](https://docs.rs/rct/)
[![codecov](https://codecov.io/gh/disco07/rct/branch/main/graph/badge.svg)](https://codecov.io/gh/disco07/rct)
# rct
A CLI Table Output for Rust 🦀 projects.

## Installation
Add from command line.
```
cargo add rct@0.1.2
```
Or add this to your Cargo.toml file.
```
[dependencies]
rct = "0.1.2"

# Or add from github main branch.
rct = { git = "https://github.com/disco07/rct.git", branch = "main" }

```

## Usage
### Basic usage
```rust
fn main() {
    use rct::cell::ICell;
    use rct::table::Table;

    let mut table = Table::new();

    table
        .add_header(vec![
            "ID".cell(),
            "Title".cell(),
            "is_enabled".cell(),
            "price".cell(),
            "currency".cell(),
            "description".cell(),
            "created_at".cell(),
        ])
        .add_row(vec![
            1.cell(),
            "Harry \nPotter".cell(),
            "1".cell(),
            "14.87".cell(),
            "€".cell(),
            "Harry Potter".cell(),
            "2001-12-05 22:05:20".cell(),
        ])
        .add_row(vec![
            2.cell(),
            "Spider-man".cell(),
            "0".cell(),
            "18.80".cell(),
            "€".cell(),
            "Spider-man, No Way Home.".cell(),
            "2018-12-12 09:04:50".cell(),
        ])
        .add_row(vec![
            3.cell(),
            "Avenger".cell(),
            "1".cell(),
            "18.50".cell(),
            "€".cell(),
            "Avenger".cell(),
            "2017-10-12 10:34:39".cell(),
        ]);

    table.view()
}
```
![Basic bar](images/basic.PNG)

## Contributing 🤝
Contributions, issues, and feature requests are welcome!

Feel free to check the issues page.

## 📝 License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
