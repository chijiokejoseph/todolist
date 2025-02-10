use std::io::{stdin, stdout, Write};

/// # input_str
/// takes a string literal as its only argument and prompts the user with that string literal.
/// The user's input is then retrieve trimmed and returned as `Some(String)`. If there is an
/// error occurs during the operation, the error is printed and `None` is returned.
///
/// # Arguments
/// - **prompt**: A string literal that is printed to the user.
/// # Returns
/// User input as `Some(String)` else `None` if the function errors.
pub fn input_str(prompt: &str) -> Option<String> {
    print!("\n{prompt}");
    // create `String` buffer to store data that will be read.
    let receiver = &mut String::new();
    // flush stdout to remove the buffering side effect
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => {
            println!("\nError writing to stdout. Error: {}", e);
            return None;
        }
    };
    // read data from stdin and store in the `String` buffer
    match stdin().read_line(receiver) {
        Ok(_) => (),
        Err(e) => {
            println!("\nError reading from stdin. Error: {}", e);
            return None;
        }
    }
    // return trimmed `String` of the entered input.
    Some(receiver.trim().to_string())
}

/// # input_num
/// takes a string literal as its only argument and then prompts the user with this string literal.
/// Retrieves the user's input and then attempts to parse it to an integer before returning it.
/// # Arguments
/// - **prompt**: A prompt message displayed to the user
/// # Returns
/// The user's input as a number of value `Some(i32)` else if the function errors during retrieving
/// the user's input or during parsing the input to a number, `None` is returned.
pub fn input_num(prompt: &str) -> Option<i32> {
    // read user input as string or `String`
    let user_input = input_str(prompt)?;

    // parse user input to i32
    let num_input = match user_input.parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("\nInvalid input entered. Could not parse '{user_input}' to a number");
            return None;
        }
    };

    // return parsed input
    Some(num_input)
}

/// # input_option
/// prompts a user with a string literal to which the user is to respond
/// based on a set of options that have been printed out to the user.
/// # Arguments
/// - **prompt**: A string literal that is used to prompt the user
/// - **options**: A vector of possible values that the user is to pick from.
/// The provided options are printed out with assigned option numbers starting from 1.
/// The user is expected to enter a number matching the corresponding option.
/// - **title**: A string literal which is printed out as the heading for the provided options
/// # Returns
/// the selected option as `Some(&str)` if the function completes successfully else `None`.
pub fn input_option<'a>(
    prompt: &'a str,
    options: &Vec<&'a str>,
    title: &'a str,
) -> Option<&'a str> {
    // print `options title`
    println!("\n{title}");
    println!("{}", "_".repeat(20));

    // print options
    for (opt_idx, option) in options.iter().enumerate() {
        let opt_num = opt_idx + 1;
        let option_print = *option;
        println!("{opt_num}. {option_print}");
    }
    // get input option num
    let num_input = input_num(prompt)?;

    // get index of target option
    let idx_input = num_input - 1;

    // validate entered option number
    if num_input < 1 || num_input > options.len() as i32 {
        println!(
            "You have entered an invalid option number. Expected an input within range 1 - {}",
            options.len()
        );
    }

    // extract and return selected option
    let selected_option = options.get(idx_input as usize)?;
    println!("\nYou have selected {selected_option}");
    Some(*selected_option)
}
