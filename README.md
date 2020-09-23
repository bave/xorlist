# xorlist

![CI](https://github.com/bave/xorlist/workflows/workflows/CI/badge.svg)

xorlist (rust library test)

```
[/tmp/test_xorlist]$ tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```
---

## Cargo.toml

```markdown
[package]
name = "test_xorlist"
version = "0.1.0"
authors = ["bave <inoue.tomoya@gmail.com>"]
edition = "2018"
[dependencies]
xorlist = { git = "https://github.com/bave/xorlist.git" }
```

---

## src/main.rs
```rust
extern crate xorlist;

use std::process::exit;

fn main() {
    let mut xl = xorlist::XorList::new() as xorlist::XorList<u64>;
    xl.push_front(1);
    xl.push_back(2);
    print!("{}\n", xl.pop_front().unwrap());
    print!("{}\n", xl.pop_front().unwrap());
    print!("bye\n");
    exit(0);
}
```

---

## shell

```shell
% cargo run
1
2
bye
```

