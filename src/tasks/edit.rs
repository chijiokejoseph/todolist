use crate::inputs::prelude::input_str;
use crate::tasks::structs::Task;
use crate::tasks::utils::extract_task;

/// # edit
/// edits a task's name and changes the task name to a new name specified by the user.
/// A view of the active tasks is printed out to the user and the user is prompted to enter
/// the task ID of the target task to be edited before being prompted to enter a new name
/// for the task.
/// # Arguments
/// - **name_in**: The name of the user
/// - **active_tasks**: A mutable reference to a list of active tasks.
/// # Returns
/// `Some(())` or Some unit type if the function completes successfully else `None`.
pub fn edit(name_in: &str, active_tasks: &mut Vec<Task>) -> Option<()> {
    let prompt = "Please enter the Task ID of the task you wish to edit: ";
    let (_, target_task) = extract_task(name_in, prompt, active_tasks)?;
    let new_task_name = input_str("Enter the new task name: ")?;
    let old_task_name = target_task.name.clone();
    target_task.set(new_task_name);
    println!(
        "\nTask with ID '{}' and old Name '{}' has been edited to new name '{}'",
        target_task.id, old_task_name, target_task.name,
    );
    Some(())
}
