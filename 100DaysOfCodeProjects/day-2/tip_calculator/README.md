Tip Calculator

Instructions

- If the bill was $150.00, split between 5 people, with 12% tip.

- Each person should pay (150.00/5) * 1.12 = 33.6.

- Format the result to 2 decimal places = 33.60.

- Thus everyone's share of the total bill is $30.00 plus a $3.60 tip.

Example Input:
Welcome to the Tip Calculator
What was the total bill? $124.56
How much tip would you like to give? 10, 12, or 15? 12
How many people to split the bill? 7

Example Output:
Each person should pay: $19.93

Learning Points: .unwrap() and .expect() are similar but .expect() will display a desired panic message as an argument. Gathering user input, especially integers, seems very convoluted and not as straight forward as other languages. For Example:

- Create Variable (let mut x)
- Ask User for Input (println!(""))
- Utilize io::stdin().read_line(&mut x).expect("Panic Message")
- Convert from String to Integer (x.trim().parse().expect("Panic Message"))

.trim() is used to trim any leading or trailing whitespaces. .parse() is used to convert the string to number. .expect() specifies that program will panic if string cannot be converted to number.

Pain Points: Remembering types while creating a function.

