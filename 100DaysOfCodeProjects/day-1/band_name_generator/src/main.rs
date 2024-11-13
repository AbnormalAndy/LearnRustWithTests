use std::io;


fn main() {
    println!("Band Name Generator\n");

    println!("What city did you grow up in?");
    let mut input_city = String::new();
    io::stdin().read_line(&mut input_city).unwrap();
    // println!("City: {}.", input_city.trim_end_matches("\n"));

    println!("\nWhat is a pet's name?");
    let mut input_pet = String::new();
    io::stdin().read_line(&mut input_pet).unwrap();
    // println!("Pet's Name: {}.", input_pet.trim_end_matches("\n"));

    println!("\nYour Band Name: {} {}", input_city.trim_end_matches("\n"), input_pet.trim_end_matches("\n"));

    println!("\nSee you, space cowboy.");
}
