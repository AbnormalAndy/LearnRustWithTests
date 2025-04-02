use std::io;


// Question Decision
fn decision_input(decision: i32) -> String {
    let mut answer = String::new();
    match decision {
        // Correct Path Choices
        0 => println!("You are at a crossroad. Where do you want to go? Type 'Left' or 'Right'."),
        1 => println!("You come across a lake. What would you like to do? Type 'Swim' or 'Wait'."),
        2 => println!("You come across three doors. Which door would you like to go through? 'Green' or 'Pink' or 'Blue'."),


        // Game Over Choices
        20 | 21 | 22 | 23 => println!("Game over. Would you like to play again? 'Yes' or 'No'."),


        // Win Choices
        9 => println!("You win! Would you like to play again? 'Yes' or 'No'."),


        // Default / Error Return
        _ => println!("Invalid decision input.")
    }


    io::stdin().read_line(&mut answer).unwrap();
    return answer
}


// Decision Choice
fn decision_string(decision: String) -> i32 {
    match decision.to_lowercase().as_str() {
        // Correct Path Choices
        "l\n" | "left\n" => return 1,
        "w\n" | "wait\n" => 2,


        // Game Over Choices
        "r\n" | "right\n" => return 20,
        "s\n" | "swim\n" => return 21,
        "g\n" | "green\n" => return 22,
        "b\n" | "blue\n" => return 23,


        // Win Choices
        "p\n" | "pink\n" => return 9,


        // Continue Game Choices
        "n\n" | "no\n" => return 12,
        "y\n" | "yes\n" => return 13,


        // Default / Error Return 
        _ => return 11,
    }
}


// Outcome Decision
fn decision_output(decision: i32) -> String {
    match decision {
        1 => return "Went left down the road.".to_string(),
        2 => return "Waited to cross the lake.".to_string(),

        // Game Over Choices
        20 => return "Went right down the road. Fell into a hole.".to_string(),
        21 => return "Swam across the lake. Attacked by trout.".to_string(),
        22 => return "Chose the green door. Burned by fired.".to_string(),
        23 => return "Chose the blue door. Eaten by beasts.".to_string(),

        // Win Choices
        9 => return "Chose the pink door. Found the treasure!".to_string(),

        // Continue Game Choices
        12 => return "Thank you for playing!".to_string(),
        13 => return "Restarting game...".to_string(),

        // Default / Error Return
        _ => return "Invalid decision output.".to_string()
    }
}


fn main() {
    let mut decision_number = 0;
    let mut game_on = true;


    println!("Welcome to Treasure Island");
    println!("Your mission is to find the treasure.");


    while game_on == true {
        // DEBUG Statement
        //println!("{}", decision_number);
        decision_number = decision_string(decision_input(decision_number));


        // DEBUG Statement
        //println!("{}", decision_number);
        println!("{}", decision_output(decision_number));


        if decision_number == 11 {
            println!("Invalid decision string.");
            game_on = false;
        } else if decision_number == 12 {
            game_on = false;
        } else if decision_number == 13 {
            println!("Welcome to Treasure Island");
            println!("Your mission is to find the treasure.");
            decision_number = 0;
        }
        

        // DEBUG Statement
        //println!("{}", decision_number);
    }
}


