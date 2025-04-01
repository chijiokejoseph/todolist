use crate::internal::tasks::structs::Task;

/// # view
/// prints out all the tasks passed into the function as
/// the argument `tasks_list`
/// # Arguments
/// - **name_in**: The name of the user
/// - **tasks_list**: a list of tasks that should be printed
/// # Returns
/// none
pub fn view(name_in: &str, tasks_list: &Vec<Task>) {
    println!("\nTasks View");
    if tasks_list.len() > 0 {
        println!("{name_in}, your tasks are printed below");
    } else {
        println!("{name_in}, you have no tasks to view");
    }

    for (task_idx, task) in tasks_list.iter().enumerate() {
        println!("{}. {}", task_idx + 1, task.show());
    }
}
