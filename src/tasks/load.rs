use crate::paths::get_file_path;
use crate::tasks::structs::{Task, TaskStatus, Tasks};
use std::fs;
use std::fs::File;
use std::io::Read;

/// # load_tasks
/// reads a list of tasks from storage based on the `status` argument passed into the function
///
/// - if status == TaskStatus::Active
///     - it reads the list of tasks from 'Active.txt'
/// - elif status == TaskStatus::Completed
///     - it reads the list of tasks from 'Completed.txt'
/// - else (status == TaskStatus::Deleted)
///     - it reads the list of tasks from 'Deleted.txt'
///
/// If the operation completes successfully, the trimmed text from the read file is returned.
/// # Arguments
/// - **status**: a TaskStatus object that determines the type of tasks to read from storage
/// # Returns
/// `Some(String)` containing data read from storage if the function completes successfully
/// else `None`.  
fn load_tasks(status: TaskStatus) -> Option<String> {
    // determine the filename without extension based on `status` argument
    let task_name = match status {
        TaskStatus::Active => "Active",
        TaskStatus::Completed => "Completed",
        TaskStatus::Deleted => "Deleted",
    };
    let filename = String::from(task_name) + ".txt";
    // get the file's `save_path` using the `get_file_path` fn
    let save_path = get_file_path(filename.as_str());
    // ensure that all parent dirs exist
    let parent_path = save_path.parent()?;
    fs::create_dir_all(parent_path).ok()?;

    // open `save_path` as a file
    let mut save_file = match File::open(&save_path) {
        Ok(f) => f,
        Err(_) => {
            println!("WARNING: Could not open file {}", save_path.display());
            println!("Please 🙏🙏 try again");
            return None;
        }
    };

    // read data from file to the String buffer `buf`
    let mut buf = String::new();
    match save_file.read_to_string(&mut buf) {
        Ok(_) => {
            println!("Loaded tasks in {filename}");
        }
        Err(_) => {
            println!("WARNING: Could not read file {}", save_path.display());
            println!("Please 🙏🙏 try again");
            return None;
        }
    }
    // return a trimmed `String` from the `buf`
    Some(buf.trim().to_string())
}

/// # fill
/// reads a certain type of tasks from file storage and parses each line of text that is returned
/// to the `Task` struct used to store data for tasks. All the tasks are returned as a vector of
/// `Task` which are empty if there are no tasks stored or its file has not yet been created.
/// # Arguments
/// - **status**: a `TaskStatus` variant that determines the type of tasks to read in. See also
/// `load_tasks`
/// # Returns
/// `Some(Vec<Task>)` if the function reads any task from storage else `None`
fn fill(status: TaskStatus) -> Option<Vec<Task>> {
    // load tasks based on the `status` argument i.e., based on the type of tasks to load in.
    let tasks_str = load_tasks(status)?;

    // return a vector of string literals from the loaded string of tasks by separating the strings
    // at newlines.
    let tasks_str_vec = tasks_str.split("\n").collect::<Vec<&str>>();

    // read each element (line) of `tasks_str_vec` and parse it to a valid `Task` object.
    let mut tasks_vec = Vec::new();
    for task_str in tasks_str_vec {
        // trim returned line from storage stored in `tasks_str_vec`
        let trimmed_task = task_str.trim();
        // skip empty lines
        if trimmed_task.is_empty() {
            continue;
        }
        // get `Task` object from the trimmed string literal
        let loaded_task = Task::from_str(task_str)?;
        // store returned task in tasks_vec
        tasks_vec.push(loaded_task);
    }
    Some(tasks_vec)
}

/// # load
/// attempts to load tasks for each type of tasks i.e.,
/// - active
/// - completed
/// - deleted
///
/// from the file storage. After loading, it appends the vector of
/// tasks read for each type of tasks to the corresponding member of
/// the mutable `Tasks` argument passed into the function.
/// if the file does not exist or is empty, an empty vector is appended.
/// # Arguments
/// - **tasks**: A mutable `Tasks` object that is meant to hold the
/// vector of tasks for each task type in its `active`, `completed`
/// and `deleted` members
/// # Returns
/// `Some(())` if the operation completes successfully else `None`.
pub fn load(tasks: &mut Tasks) -> Option<()> {
    let mut active_tasks = fill(TaskStatus::Active)?;
    tasks.active.append(&mut active_tasks);
    let mut completed_tasks = fill(TaskStatus::Completed)?;
    tasks.completed.append(&mut completed_tasks);
    let mut deleted_tasks = fill(TaskStatus::Deleted)?;
    tasks.deleted.append(&mut deleted_tasks);
    Some(())
}
