use crate::internal::tasks::prelude::Tasks;
use crate::internal::tasks::structs::TaskStatus;
use crate::internal::tasks::utils::extract_task;

/// # delete
/// deletes a task from the active tasks. Prints out a view of active tasks
/// to the user and then prompts the user to enter the task id of the target task
/// to be deleted. The target task status is set to `TaskStatus::Deleted` before
/// being removed from the active tasks and added to the deleted tasks.
/// # Arguments
/// - **name_in**: The name of the user
/// - **tasks**: A mutable ref to Tasks which stores the active, completed and deleted tasks.
/// # Returns
/// `Some(())` or Some unit type if the function completes successfully else `None`.
pub fn delete(name_in: &str, tasks: &mut Tasks) -> Option<()> {
    let active_tasks = &mut tasks.active;
    let deleted_tasks = &mut tasks.deleted;
    let prompt = "Please enter the Task ID of the task you wish to delete: ";
    let (task_idx, _) = extract_task(name_in, prompt, active_tasks)?;
    let mut target_task = active_tasks.remove(task_idx);
    target_task.status = TaskStatus::Deleted;
    println!(
        "\nTask with details {} has been deleted",
        target_task.show()
    );
    deleted_tasks.push(target_task);
    Some(())
}
