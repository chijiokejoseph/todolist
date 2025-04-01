use super::inputs::prelude::input_str;
use super::paths::get_name_path;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// # new_name
/// called with the argument `name_path` which is the
/// path to a 'names.txt' file where the program saves the
/// user's name. This function is called when a new user
/// is detected and prompts the user to enter his name
/// before storing that name back to `name_path`
/// # Arguments
/// - **name_path**: path to a 'names.txt' file where the
/// program saves the user's name.
/// # Returns
/// `Some(name)` where name is a String containing the user's
/// name if the function completes successfully else `None`.
fn new_name(name_path: &PathBuf) -> Option<String> {
    // prompt and get user's name input
    let new_name_prompt: &str =
        "This is your first time using the Todo List \nPlease enter your name: ";
    let name_input = input_str(new_name_prompt)?;

    // extract the parent dir of the name_path
    let name_path_parent = name_path.parent()?;
    // create the parent dir if it does not exist
    fs::create_dir_all(name_path_parent).ok()?;
    // create the 'name.txt' file and open it in read-write mode
    let mut name_file = File::create(name_path).ok()?;
    // write the user's input name to the newly created file
    match name_file.write(name_input.as_bytes()) {
        Ok(_) => Some(name_input),
        Err(_) => {
            println!("WARNING: Could not save your name {name_input} to file");
            println!("Please 🙏🙏 try again");
            None
        }
    }
}

/// # get_name
/// returns the name of the user by reading the data stored in
/// its input argument `name_path`. NB: The argument to
/// `name_path` must exist before passing into this function
/// # Arguments
/// - **name_path**: the path to a 'names.txt' file where the
/// name of the user is stored.
/// # Returns
/// `Some(name)` where name is a String read from `name_path`
/// after reading its file contents and trimming it. May return
/// `None` if function fails.
fn get_name(name_path: &PathBuf) -> Option<String> {
    // open file stored in `name_path` in read mode
    let mut name_file = match File::open(name_path) {
        Ok(f) => f,
        Err(_) => {
            println!("WARNING: Could not open file {}", name_path.display());
            println!("Please 🙏🙏 try again");
            return None;
        }
    };

    // read contents of the file as a String
    let mut buf = String::new();
    match name_file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => {
            println!("WARNING: Could not read file {}", name_path.display());
            println!("Please 🙏🙏 try again");
            return None;
        }
    }

    // return trimmed version of file content
    // i.e., the user's name as String
    Some(buf.trim().to_string())
}

/// # ret_name
/// returns the user's name by either prompting the user
/// to enter a name for a new user or reading the user's
/// stored name for an existing user.
/// # Arguments
/// none
/// # Returns
/// `Some(name)` where name is the name of the user if the
/// operation completes successfully else `None`.
fn ret_name() -> Option<String> {
    let name_path = get_name_path();
    if name_path.exists() {
        get_name(&name_path)
    } else {
        new_name(&name_path)
    }
}

/// # ret_name_loop
/// repeats the operation to get the user's name in the
/// case of program failure during name retrieval. Breaks
/// and returns upon success.
/// # Arguments
/// - **trails**: the number of times the program attempts
/// to retrieve the user's name
/// # Returns
/// `Some(name)` where name is the name of the user if the
/// operation completes successfully else `None`.
pub fn ret_name_loop(trials: i32) -> Option<String> {
    for _ in 1..=trials {
        match ret_name() {
            Some(name) => {
                return Some(name);
            }
            None => continue,
        }
    }
    None
}
