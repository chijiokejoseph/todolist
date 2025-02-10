use crate::paths::get_file_path;
use crate::tasks::prelude::Tasks;
use crate::tasks::structs::{Task, TaskStatus};
use std::fs;
use std::fs::File;
use std::io::Write;

/// # save_tasks
/// writes a list of tasks to storage based on the `status`
/// argument passed into the function
///
/// - if status == TaskStatus::Active
///     - it reads the list of tasks from 'Active.txt'
/// - elif status == TaskStatus::Completed
///     - it reads the list of tasks from 'Completed.txt'
/// - else (status == TaskStatus::Deleted)
///     - it reads the list of tasks from 'Deleted.txt'
///
/// each task is converted to a string and appended with
/// the newline separator to an empty string to create
/// a printout of all the tasks. This printout is then
/// written to memory.
/// # Arguments
/// - **tasks_list**: a vector of tasks from which the tasks
/// that are written to memory are gotten
/// - **status**: a `TaskStatus` enum that is used to determine
/// the name of the file to write tasks to. This filename is
/// directly linked to the type of tasks being stored.
/// # Returns
/// `Some(())` if the operation completes successfully else `None`.
fn save_tasks(tasks_list: &Vec<Task>, status: TaskStatus) -> Option<()> {
    let task_name = match status {
        TaskStatus::Active => "Active",
        TaskStatus::Completed => "Completed",
        TaskStatus::Deleted => "Deleted",
    };
    let filename = String::from(task_name) + ".txt";
    let save_path = get_file_path(filename.as_str());
    let mut task_print = String::new();
    for task in tasks_list {
        task_print.push_str(task.show().as_str());
        task_print.push_str("\n");
    }
    let parent_path = save_path.parent()?;
    fs::create_dir_all(parent_path).ok()?;
    let mut save_file = File::create(save_path).ok()?;
    match save_file.write(task_print.as_bytes()) {
        Ok(_) => Some(()),
        Err(_) => {
            println!("Error saving {task_name} tasks to {filename}");
            None
        }
    }
}

/// # save
/// saves all different type of tasks available in the program to
/// memory.
/// # Arguments
/// - **name_in**: The name of the user
/// - **tasks**: mutable `Tasks` struct whose members hold a
/// vector of tasks for each task type respectively. The
/// saved tasks are gotten from its members.
/// # Returns
/// `Some(())` if the function completes successfully else `None`
pub fn save(name_in: &str, tasks: &mut Tasks) -> Option<()> {
    save_tasks(&tasks.active, TaskStatus::Active)?;
    save_tasks(&tasks.completed, TaskStatus::Completed)?;
    save_tasks(&tasks.deleted, TaskStatus::Deleted)?;
    println!("So sad 😔 to see you go {name_in}. Visit again soon.");
    Some(())
}
