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
        2 => println!("Game over. Would you like to play again? 'Yes' or 'No'."),
        3 => println!("You come across three doors. Which door would you like to go through? 'Red' or 'Yellow' or 'Blue'.");
        _ => return "Meow".to_string()
    }


    io::stdin().read_line(&mut answer).unwrap();
    return answer
}


fn decision_string(decision: String) -> i32 {
    // Will return the same string if chosen.
    match decision.to_lowercase().as_str() {
        "l\n" | "left\n" => return 1,
        "r\n" | "right\n" => return 2,
        "red\n" => return 3,
        "y\n" | "yes\n" => return 13,
        "n\n" | "no\n" => 14,
        _ => return 99,
    }
}


fn decision_output(decision: i32) -> String {
    match decision {
        1 => return "Went left down the road.".to_string(),
        2 => return "Fell into a hole.".to_string(),
        3 => return "Burned by fired.".to_string(),
        _ => return "Meow".to_string()
    }
}


fn main() {
    let mut decision_number = 0;
    let mut game_on = true;


    println!("Welcome to Treasure Island");
    println!("Your mission is to find the treasure.");


    while game_on == true {
        println!("{}", decision_number);
        decision_number = decision_string(decision_input(decision_number));
        println!("{}", decision_number);
        println!("{}", decision_output(decision_number));
        

        // Need win / lose condition.
        if decision_number == 99 {
            println!("Invalid input.");
            game_on = false;
        } else if decision_number == 14 {
            println!("Thank you for playing!");
            game_on = false;
        } else if decision_number == 13 {
            main();
        }
    }
}


