#[derive(Debug)]
pub struct Alert {
    prefix: String,
}

impl Alert {
    pub fn new(str: String) -> Self {
        Self { prefix: str }
    }
    pub fn info(&self, str: String) {
        println!("[{}] {}", self.prefix, str);
    }
}
