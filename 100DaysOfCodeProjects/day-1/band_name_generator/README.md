Band Name Generator

1. Create a greeting for your program.

2. Ask the user for the city that they grew up in.

3. Ask the user for the name of a pet.

4. Combine the name of their city and pet and show them their band name.

5. Make sure the input cursor shows on a new line.

Learning Points: std::io was utilized to retrieve user input.

Pain Points: .trim_end_matches("\n") was used to remove the newline. This had to be used on each variables use. Attempting to mutate the original variable or a new variable would cause the compiler  to state "help: try using a conversion method" `.to_string()` -- expected `String`, found `&str`.