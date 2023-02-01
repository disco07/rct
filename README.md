[![Rust](https://github.com/disco07/rct/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/disco07/rct/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/rct.svg)](https://crates.io/crates/rct)
[![Documentation](https://docs.rs/rct/badge.svg)](https://docs.rs/rct/)
# rct
A CLI Table Output for Rust 🦀 projects.

## Installation
Add from command line.
```
cargo add rct@0.1.1
```
Or add this to your Cargo.toml file.
```
[dependencies]
rct = "0.1.1"

# Or add from github main branch.
rct = { git = "https://github.com/disco07/rct.git", branch = "main" }

```

## Usage
### Basic usage
```rust
fn main() {
    use rct::rct::Table;
    use std::collections::BTreeMap;
    
    let mut table = Table::new();
    let mut data: Vec<BTreeMap<u32, String>> = vec![];
    data.push(BTreeMap::from([
        (1, "62".to_string()),
        (2, "Harry Potter".to_string()),
        (3, "1".to_string()),
        (4, "14.87".to_string()),
        (5, "€".to_string()),
        (6, "Harry Potter".to_string()),
        (7, "2018-12-12 10:34:39".to_string()),
    ]));

    data.push(BTreeMap::from([
        (1, "72".to_string()),
        (2, "Spider-man".to_string()),
        (3, "0".to_string()),
        (4, "18.80".to_string()),
        (5, "€".to_string()),
        (6, "Spider-man, No Way Home.".to_string()),
        (7, "2018-12-12 10:34:39".to_string()),
    ]));

    table.add_field(1, "id");
    table.add_field(2, "title");
    table.add_field(3, "is_enabled");
    table.add_field(4, "price");
    table.add_field(5, "currency");
    table.add_field(6, "description");
    table.add_field(7, "created_at");
    table.add_data(data);

    table.view()
}
```
![Basic bar](images/basic.png)

## Contributing 🤝
Contributions, issues, and feature requests are welcome!

Feel free to check the issues page.

## 📝 License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
