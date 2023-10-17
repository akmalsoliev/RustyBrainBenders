pub trait Information {
    fn info(&self) -> String;
}

impl Information for i32 {
    fn info(&self) -> String {
        self.to_string()
    }
}

impl Information for &str {
    fn info(&self) -> String {
        self.to_string()
    }
}
