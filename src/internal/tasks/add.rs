use crate::internal::inputs::prelude::input_str;
use crate::internal::tasks::structs::Task;

/// # add
/// prompts the user to enter the name of a new task and then adds that task to a vector of
/// active tasks available in the program
/// # Arguments
/// - **name_in**: The name of the user
/// - **active_tasks**: A mutable vector of tasks to which the new task is appended to
/// - **new_task_id**: The id that should be assigned to the new task that is created.
/// # Returns
/// `Some(())` or a Some of the unit type to signify success, while `None` is propagated if
/// the function errors.
pub fn add(name_in: &str, active_tasks: &mut Vec<Task>, new_task_id: i32) -> Option<()> {
    let prompt = format!("{name_in} please enter a new task: ");
    // get task name from user
    let new_task_name = input_str(prompt.as_str())?;
    // create new Task
    let new_task = Task::from_name_id(new_task_name, new_task_id);
    println!(
        "\nAdded Task with details: {} successfully",
        &new_task.show()
    );
    active_tasks.push(new_task);
    Some(())
}
