
#[derive(Debug)]
pub struct Console {
    prefix: String,
}
impl Console {
    pub fn new(prefix: String) -> Self {
        Console { prefix }
    }
    pub fn log(&self, str: String) {
        println!("[{}] {}", self.prefix, str)
    }
}
