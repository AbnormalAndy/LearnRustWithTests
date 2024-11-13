use std::io;

fn tip_calculator(total_bill: f32, tip_amount: f32, number_of_people: f32) -> f32 {
    let tip_total: f32 = (tip_amount / 100.0) + 1.0;
    return (total_bill / number_of_people) * tip_total;
}

fn main() {
    println!("\nWelcome to the Tip Calculator");

    let mut input_bill = String::new();
    println!("\nWhat was the total bill?");
    io::stdin().read_line(&mut input_bill).expect("Failed to read line");
    let bill: f32 = input_bill.trim().parse().expect("Invalid input");
    // println!("You entered: {}.", bill);

    let mut input_tip = String::new();
    println!("\nHow much tip would you like to give? 10, 12, 15, or 20?");
    io::stdin().read_line(&mut input_tip).expect("Failed to read line");
    let tip: f32 = input_tip.trim().parse().expect("Invalid input");
    // println!("You entered: {}.", tip);

    let mut input_people = String::new();
    println!("\nHow many people to split the bill?");
    io::stdin().read_line(&mut input_people).expect("Failed to read line");
    let people: f32 = input_people.trim().parse().expect("Invalid input");
    // println!("You entered: {}.", people);

    println!("\nEach Person Should Pay: ${:.2}.", tip_calculator(bill, tip, people));
}
