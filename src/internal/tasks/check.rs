use crate::internal::tasks::prelude::Tasks;
use crate::internal::tasks::structs::{DateTimeOption, TaskStatus};
use crate::internal::tasks::utils::extract_task;
use chrono::Local;

/// # check
/// marks an active task as complete. The program displays all active tasks and prompts the user
/// to enter the task ID of the task to be marked complete. The task is then extracted, its status
/// is changed to `TaskStatus::Completed`, it's `time_finished` field is recorded, and it is removed
/// from the active tasks and appended to the completed tasks
/// # Arguments
/// - **name_in**: The name of the user
/// - **tasks**: A `Tasks` struct ref that holds the vector of tasks for active and completed
/// tasks respectively.
/// # Returns
/// `Some(())` or Some unit type if the function completes successfully else `None`.
pub fn check(name_in: &str, tasks: &mut Tasks) -> Option<()> {
    let active_tasks = &mut tasks.active;
    let completed_tasks = &mut tasks.completed;
    let prompt = "Please enter the Task ID of the task you wish to mark as complete: ";

    // extract the task index of the target task as per the user's selection
    let (task_idx, _) = extract_task(name_in, prompt, active_tasks)?;

    // remove and return target task from 'active tasks'
    let mut target_task = active_tasks.remove(task_idx);
    target_task.status = TaskStatus::Completed; // set status to completed
    target_task.time_finished = DateTimeOption::DateTime(Local::now()); // set current time to
                                                                        // time_finished.
    println!(
        "\nTask with details {} has been marked as complete",
        target_task.show()
    );
    completed_tasks.push(target_task);
    Some(())
}
