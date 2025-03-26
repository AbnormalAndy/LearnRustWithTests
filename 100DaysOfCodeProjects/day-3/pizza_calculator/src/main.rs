use std::io;


fn pizza_input() -> String {
    println!("What size pizza do you want? S - M - L");
    let mut pizza_size = String::new();
    io::stdin().read_line(&mut pizza_size).unwrap();


    match pizza_size.to_lowercase().as_str() {
        "s\n" => return "small".to_string(),
        "m\n" => return "medium".to_string(),
        "l\n" => return "large".to_string(),
        _ => return "invalid".to_string(),
    };
}


fn pepperoni_input() -> bool {
    println!("Do you want pepperoni on your pizza? Y or N");
    let mut pepperoni = String::new();
    io::stdin().read_line(&mut pepperoni).unwrap();


    if pepperoni.to_lowercase().as_str() == "y\n" {
        return true
    } else {
        return false
    }
}


fn cheese_input() -> bool {
    println!("Do you want extra cheese on your pizza? Y or N");
    let mut cheese = String::new();
    io::stdin().read_line(&mut cheese).unwrap();


    if cheese.to_lowercase().as_str() == "y\n" {
        return true
    } else {
        return false
    }
}


fn pizza_calculator(pizza_size: String, add_pepperoni: bool, add_cheese: bool) -> i32 {
    let mut total: i32 = 0;


    // Match case insensitive.
    // Can to_lower() in input function.
    println!("{}", pizza_size);
    match pizza_size.as_str() {
        "small" => total += 15,
        "medium" => total += 20,
        "large" => total += 25,
        _ => println!("Invalid input."),
    };
    

    if add_pepperoni == true && pizza_size == "medium" || add_pepperoni == true && pizza_size == "large" {
        total += 2;
    };


    if add_pepperoni == true && pizza_size == "small" {
        total += 1;
    }


    if add_cheese == true {
        total += 1;
    };

    return total
}


fn main() {
    let size: String = pizza_input();
    let pepperoni: bool = pepperoni_input();
    let cheese: bool = cheese_input();


    let total: i32 = pizza_calculator(size, pepperoni, cheese);


    println!("Your total for the pizza is: ${}.00.", total);
}


