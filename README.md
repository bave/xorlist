# xorlist

![Rust](https://github.com/bave/xorlist/workflows/Rust/badge.svg)

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

    xl.push_back(1);
    print!("xl.push_back(1)\n");
    xl.push_back(2);
    print!("xl.push_back(2)\n");
    xl.push_front(0);
    print!("xl.push_front(0)\n");
    println!();
    print!("xl.len()    : {}\n", xl.len());
    print!("pop_front() : {:?}\n", xl.pop_front());
    print!("xl.len()    : {}\n", xl.len());
    print!("pop_back()  : {:?}\n", xl.pop_back());
    print!("xl.len()    : {}\n", xl.len());

    println!();
    print!("for i in &xl {{ print!(\"{{}}\", i); }}\n");
    for i in &xl {
        print!("{} ", i);
    }
    println!();

    print!("for i in &mut xl {{ *i = *i + 1; print!(\"{{}}\", i); }}\n");
    for i in &mut xl {
        *i = *i + 1;
        print!("{} ", i);
    }
    println!();

    //xl moved ownership
    print!("for i in xl {{ print!(\"{{}}\", i + 1); }}\n");
    for i in xl {
        print!("{} ", i + 1);
    }
    println!();
    println!();

    let mut xl = xorlist::XorList::new() as xorlist::XorList<u64>;
    for i in 0..5 {
        xl.push_back(i);
    }

    let xl_iter_map_collect = xl.iter().map(|x| x + 1).collect::<Vec<u64>>();
    print!("{:?}\n", xl_iter_map_collect);

    let xl_iter_mut_map_collect = xl.iter_mut().map(|x| *x + 1).collect::<Vec<u64>>();
    print!("{:?}\n", xl_iter_mut_map_collect);

    //xl moved ownership
    let xl_into_iter_map_collect = xl.into_iter().map(|x| x + 1).collect::<Vec<u64>>();
    print!("{:?}\n", xl_into_iter_map_collect);
}
```

---

## shell

```shell
% cargo run
xl.push_back(1)
xl.push_back(2)
xl.push_front(0)

xl.len()    : 3
pop_front() : Some(0)
xl.len()    : 2
pop_back()  : Some(2)
xl.len()    : 1

for i in &xl { print!("{}", i); }
1
for i in &mut xl { *i = *i + 1; print!("{}", i); }
2
for i in xl { print!("{}", i + 1); }
3

[1, 2, 3, 4, 5]
[1, 2, 3, 4, 5]
[1, 2, 3, 4, 5]
```

