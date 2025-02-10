use crate::inputs::prelude::input_num;
use crate::tasks::prelude::view;
use crate::tasks::structs::{Task, Tasks};
use std::cmp::max;

/// # ret_last_id
/// returns the task id of the last task in the
/// vector of tasks passed in as an argument if
/// the vector has a len greater than 0 else it
/// returns 0
/// # Arguments
/// - **tasks**: a vector of tasks
/// # Returns
/// 0 if `tasks` has a length of 0 else it returns
/// the `id` member of the last `Task` instance in
/// the vector `tasks`.
fn ret_last_id(tasks: &Vec<Task>) -> i32 {
    let tasks_len = tasks.len();
    if tasks_len == 0 {
        return 0;
    }
    let last_id = tasks_len - 1;
    let last_task = &tasks[last_id];
    last_task.id
}

/// # ret_last_task_id
/// returns the `Task` member `id` of the most recent
/// task created in the program. This is done by getting
/// the maximum of the last task ids for each member of
/// the `Tasks` instance (i.e., the various types of tasks
/// in the program stored as vectors of tasks.)
/// # Arguments
/// - **tasks**: a `Tasks` instance that stores all the
/// different vectors of tasks in the program
/// # Returns
/// the `Task` member `id` of the most recent task as
/// an `i32`.   
pub fn ret_last_task_id(tasks: &Tasks) -> i32 {
    let last_active_id = ret_last_id(&tasks.active);
    let last_completed_id = ret_last_id(&tasks.completed);
    let last_deleted_id = ret_last_id(&tasks.deleted);
    let max_id = max(last_active_id, last_completed_id);
    max(max_id, last_deleted_id)
}

/// # extract_task
/// this prints out a view of active tasks in the program
/// and the user is prompted to select the target task
/// for an operation using the tasks' task id.
/// # Arguments
/// - **name_in**: The name of the user
/// - **prompt**: The prompt that is printed out to the user
/// when asking for the task id of task to select.
/// - **active_tasks**: The list of active tasks in the
/// program.
/// # Returns
/// `Some(num, target_task)` where `num` is the index of
/// the target task in the list of active tasks and `target_task`
/// is the selected task in the list of active tasks if the user's
/// input matches a task id of a task in the list of active tasks,
/// else `None`.
pub fn extract_task<'a>(
    name_in: &'a str,
    prompt: &'a str,
    active_tasks: &'a mut Vec<Task>,
) -> Option<(usize, &'a mut Task)> {
    view(name_in, active_tasks);
    // get task id entered by the user
    let input_task_id = input_num(prompt)?;
    // extract the index of the target task
    let target_task_idx = active_tasks
        .iter()
        .position(|each_task| each_task.id == input_task_id);
    // print warning if the index is None
    if let None = target_task_idx {
        println!("No task found with Task ID = {input_task_id}");
    }
    // extract a tuple of task index and target task.
    let index_task_tuple = active_tasks
        .iter_mut()
        .enumerate()
        .find(|(_, each_task)| each_task.id == input_task_id)?;
    println!("You have selected the task with the following details: ");
    println!("{}", index_task_tuple.1.show());
    // return the extracted tuple of task index and target task.
    Some(index_task_tuple)
}
