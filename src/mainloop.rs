use crate::inputs::prelude::*;
use crate::tasks::prelude::*;

/// # mainloop
/// runs the main block of all the program's operations once.
/// # Arguments
/// - **name_in**: The name of the user
/// - **tasks**: A mutable struct that models all the
/// different tasks in the program stored in its members
/// as vectors of tasks
/// # Returns
/// `Some(bool)` where bool is a `bool` value that indicates if
/// the `mainloop` function should be rerun. May return `None`
/// if any part of the function fails. In the case `None` is
/// returned, the `mainloop` function is automatically called
/// again.
pub fn mainloop(name_in: &str, tasks: &mut Tasks) -> Option<bool> {
    // initialize main menu
    let options = vec![
        "Add task",
        "View active tasks",
        "View completed tasks",
        "View deleted tasks",
        "Edit task name",
        "Mark task as complete",
        "Delete task",
        "Exit program",
    ];

    // get user's menu selection
    let choice = match input_option("Select an option: ", &options, "Menu") {
        Some(choice) => choice,
        None => return Some(true),
    };

    // get the task id of the most recent task in the program
    let last_task_id = ret_last_task_id(tasks);

    // create new task id
    let new_task_id = last_task_id + 1;

    // extract all types of tasks in the program
    let active_tasks = &mut tasks.active;
    let completed_tasks = &mut tasks.completed;
    let deleted_tasks = &mut tasks.deleted;

    // get the index of the user's selected menu operation
    let choice_idx = options
        .iter()
        .position(|&each_option| each_option == choice)?;

    // convert the previous index into a count from 1
    let choice_num = choice_idx + 1;

    // match the user's selected index and perform the valid
    // operation for each index else return None for invalid
    // values.
    match choice_num as i32 {
        1 => add(name_in, active_tasks, new_task_id)?,
        2 => view(name_in, active_tasks),
        3 => view(name_in, completed_tasks),
        4 => view(name_in, deleted_tasks),
        5 => edit(name_in, active_tasks)?,
        6 => check(name_in, tasks)?,
        7 => delete(name_in, tasks)?,
        8 => {
            save(name_in, tasks)?;
            return Some(false);
        }
        _ => return None,
    };
    println!();
    // return rerun as true so that this function is called again.
    Some(true)
}
