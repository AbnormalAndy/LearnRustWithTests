use std::io;


// Function to take input and return output.
// - Asks question; returns number for output.
// Function to evaluate previous function output.
// - Gives output of number.

fn decision_input(decision: i32) -> String {
    let mut answer = String::new();
    match decision {
        0 => println!("You are at a crossroad. Where do you want to go? Type 'Left' or 'Right'."),
        1 => println!("You come across a lake. What would you like to do? Type 'Swim' or 'Wait'."),
        _ => return "Meow".to_string()
    }
    io::stdin().read_line(&mut answer).unwrap();
    return answer
}


fn decision_string(decision: String) -> i32 {
    match decision.to_lowercase().as_str() {
        "l\n" | "left\n" => return 1,
        _ => return 99,
    }
}


fn main() {
    let mut decision_number = 0;
    //let mut decision_string = String::new();

    println!("Welcome to Treasure Island");
    println!("Your mission is to find the treasure.");
    decision_number = decision_string(decision_input(decision_number));
    println!("{}", decision_number);
}


