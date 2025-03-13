# rustchess

Highly efficient chess library for Rust, with complementary Python bindings.


## Installation

Install it from crates.io with:

```
cargo add rustchess
```


## Example usage

Import the crate and most of the necessary APIs are available in the root level of the
crate:

```rust
use rustchess::{Board, Move};

fn main () {
    let mut b = Board::default();
    b.push(Move::from_uci("e2e4"));
    println!("{b}");
}

```

