fn pizza_calculator(pizza_size: &str, add_pepperoni: bool, add_cheese: bool) -> i32 {
    let size = pizza_size;


    let mut total: i32 = 0;


    match size {
        "s" => total += 15,
        "m" => total += 20,
        "l" => total += 25,
        _ => println!("Incorrect input."),
    };
    

    if add_pepperoni == true {
        total += 2;
    };


    if add_cheese == true {
        total += 1;
    };

    return total
}


fn main() {
    let size: &str = "m";
    let pepperoni: bool = true;
    let cheese: bool = true;


    let total: i32 = pizza_calculator(size, pepperoni, cheese);


    println!("{}", total);
}


// TO-DO
// - Pizza Size
//     - S = 15; M = 20; L = 25
// - Pepperoni
//     - S = +2; M / L = +3
// - Extra Cheese
//     - +1
//
// - Functions to take input.
//     - What size pizza do you want? S, M, L
//     - Do you want pepperoni on your pizza? Y or N
//     - Do you want extra cheese? Y or N
