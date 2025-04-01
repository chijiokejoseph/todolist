mod mainloop;

use todolist::get_name_path;
use todolist::{load, Tasks};
use mainloop::mainloop;
use todolist::ret_name_loop;
use std::io::{stdout, Write};

/// # main
/// The main entry point to the program. This function
/// tries to read in the user's name if it exists or
/// prompt the user to enter a new name before rerunning
/// the `mainloop` function until the user explicitly
/// asks the program to stop.
/// # Arguments
/// none
/// # Returns
/// none
fn main() {
    // instantiate `Tasks` instance
    let mut tasks = Tasks::new();
    // loads previous tasks if there are any stored by previous runs.
    if get_name_path().exists() {
        load(&mut tasks);
    }
    // attempt to retrieve the user's name
    let trials = 5;
    let name_value = ret_name_loop(trials);
    let name = match name_value {
        Some(text) => text,
        None => {
            println!(
                "\nCould not retrieve user's name successfully after {} trials. Exiting..",
                trials
            );
            return ();
        }
    };
    let name = name.as_str();
    stdout().flush().unwrap_or_default();
    println!("\nHello {name}. Your Todo List Manager here 👋");
    println!("How may I help you today?");

    // rerun the `mainloop` function through this loop
    loop {
        // get the result of the mainloop function
        let rerun_option = mainloop(name, &mut tasks);
        match rerun_option {
            // handle boolean cases by breaking the loop if the
            // returned boolean (understood as `rerun`) is false
            Some(rerun) => {
                if !rerun {
                    break;
                }
            }
            None => (),
        }
    }
}
