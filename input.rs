pub mod input {
    pub fn readline() -> String {
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }
}
