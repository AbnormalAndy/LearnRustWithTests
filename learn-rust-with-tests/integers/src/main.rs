fn main() {
    println!("{}", add(2, 5));
}


fn add(x: i32, y: i32) -> i32 {
    return x + y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(1, 3);
        assert_eq!(result, 4);
    }
}


