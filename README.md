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
        ip_addr: "127.0.0.1".to_string(),
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

## CLI
can be run as a cli for starting a service or requesting a client connection.

### server
```bash
$ awl server <host> <port>
```
where <host> and <port> are optional, defaulting to localhost:15555

### client
```
$ awl client -s <rendezvous_server_ip> -p <rendezvous_server_port> <target_host> <target_port>
```
where <target_host> and <target_port> are optional, defaulting to :15556.

`rendezvous_server_ip` is the public IP address that server is running on

`rendezvous_server_port` is optional, defaults to awl server default port 15555
