fn main() {
    println!("{}", hello_world());
}


fn hello_world() -> String {
    return "Hello, World!".to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = hello_world();
        assert_eq!(result, "Hello, World!");
    }
}
