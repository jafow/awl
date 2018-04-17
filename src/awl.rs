/// Main constructor.
pub struct Awl {
    pub ip_addr: String,
    pub port: u16,
}

impl Awl {
    pub fn my_ip(&self) {
       println!("My IP is {}", self.ip_addr);
    }
}
