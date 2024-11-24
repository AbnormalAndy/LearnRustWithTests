fn main() {
    println!("{}", hello("Meow", "SPANISH"));
}


const ENGLISHHELLOPREFIX: &str = "Hello, ";
const SPANISHHELLOPREFIX: &str = "Hola, ";
const FRENCHHELLOPREFIX: &str = "Bonjour, ";


fn hello(mut name: &str, language: &str) -> String {
    if name == "" {
        name = "World";
    }
    let language = language.to_lowercase();
    let prefix = prefix(&language);
    return format!("{prefix}{name}!")
}


fn prefix(language: &str) -> &str {
    match language {
        "spanish" => return SPANISHHELLOPREFIX,
        "french" => return FRENCHHELLOPREFIX,
        _ => return ENGLISHHELLOPREFIX,
    }
}


/*
fn prefix(language: &str) -> &str {
    if language.to_lowercase() == "spanish" {
        return SPANISHHELLOPREFIX
    } else if language.to_lowercase() == "french" {
       return FRENCHHELLOPREFIX
    } else {
        return ENGLISHHELLOPREFIX
    }
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_empty() {
        let result = hello("", "ENGLISH");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_hello() {
        let result = hello("Meowth", "ENGLISH");
        assert_eq!(result, "Hello, Meowth!");
    }

    #[test]
    fn test_hello_spanish_empty() {
        let result = hello("", "SPANISH");
        assert_eq!(result, "Hola, World!");
    }

    #[test]
    fn test_hello_spanish() {
        let result = hello("Ada", "SPANISH");
        assert_eq!(result, "Hola, Ada!");
    }

    #[test]
    fn test_hello_french_empty() {
        let result = hello("", "FRENCH");
        assert_eq!(result, "Bonjour, World!");
    }

    #[test]
    fn test_hello_french() {
        let result = hello("Kat", "FRENCH");
        assert_eq!(result, "Bonjour, Kat!");
    }
}
