# awl
rendezvous service and client for udp hole punching

details in progresss at [the wiki](https://github.com/jafow/awl/wiki/overview)

## wip example
two crates: 1 for a binary that imports lib and implements client and/or server
(accepts cli args)
and other crate for the `lib.rs`.

src/main.js

```rust
extern crate awl;

use awl::Awl;

fn main() {
    println!("awl is a tool to punch holes with.");
    let a = Awl {
        ip_addr: "44.00.32.14".to_string(),
        port: 1337
    };

    a.my_ip();
}
```

and in src/lib.rs
```rust
/// Main constructor
pub struct Awl {
    pub ip_addr: String,
    pub port: u16,
}

impl Awl {
    pub fn my_ip(&self) {
       println!("My IP is {}", self.ip_addr);
    }
}

/// add all the actual client/server/connection etc here
/// or as own mods imported here
```
