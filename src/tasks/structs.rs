use chrono::prelude::*;

const DATE_FMT: &str = "%Y/%m/%d %H:%M:%S";

/// # to_string
/// private utility function to parse string and convert it to
/// a `Task` instance. Parsing is done by removing the prefix
/// of the string as specified by the `prefix` argument and
/// trimming and returning the result
/// # Arguments
/// - **source_vec**: a vector of string literals that contains
/// the target string
/// - **index**: the index of the target string in `source_vec`
/// - **prefix**: the prefix that is to be removed from the
/// target string.
/// # Returns
/// the parsed string
fn to_string(source_vec: &Vec<&str>, index: i32, prefix: &str) -> String {
    let clean_str = source_vec[index as usize].replace(prefix, "");
    let trimmed_str = clean_str.trim();
    trimmed_str.to_string()
}

/// # to_num
/// private utility function to parse string and convert it to
/// a `Task` instance. Parsing is done by removing the prefix
/// of the string as specified by the `prefix` argument and
/// trimming the result. The trimmed result is then parsed to
/// a number.
/// # Arguments
/// - **source_vec**: a vector of string literals that contains
/// the target string
/// - **index**: the index of the target string in `source_vec`
/// - **prefix**: the prefix that is to be removed from the
/// target string.
/// # Returns
/// `Some(number)` where number is the parsed number if the
/// function completes successfully else `None`.
fn to_num(source_vec: &Vec<&str>, index: i32, prefix: &str) -> Option<i32> {
    let trimmed_str = to_string(source_vec, index, prefix);
    trimmed_str.parse::<i32>().ok()
}

/// # to_datetime
/// parses the argument `date_str` to a `DateTime<Local>>` instance
/// i.e., a datetime that follows the system's local time settings.
///
/// It parses the input `date_str` which is assumed to have been
/// created as a local datetime but is being read as UTC datetime.
///
/// Hence, the result has the Local time offset removed from it
/// before converting the UTC Datetime to Local DateTime again.
/// # Arguments
/// - **date_str**: a string of datetime information which is
/// expected to match the format "%Y/%m/%d %H:%M:%S". The datetime
/// information should also include the local time offset which
/// the function balances out when reading it as UTC before
/// converting it back to Local DateTime.
/// # Returns
/// `Some(datetime)` where datetime is the local datetime equivalent
/// of `date_str` if the function completes successfully else `None`.
fn to_datetime(date_str: String) -> Option<DateTime<Local>> {
    let now = Local::now();
    let offset = now.offset();
    let timezone = now.timezone();
    let mut naive_date = NaiveDateTime::parse_from_str(&date_str, DATE_FMT).ok()?;
    naive_date = naive_date.checked_sub_offset(*offset)?;
    let local_date = timezone.from_utc_datetime(&naive_date);
    Some(local_date)
}

/// # to_datetime_from_vec
/// converts a target string from a source vector to a datetime
/// after removing its prefix as prescribed by the `prefix` argument
/// and trimming the result
/// # Arguments
/// - **source_vec**: the vector of string literals in which the
/// target string is contained
/// - **index**: the index of the target string in `source_vec`
/// - **prefix**: a string literal with which the target string is
/// prefixed.
/// # Returns
/// `Some(datetime)` where datetime is the local datetime equivalent
/// of `date_str` if the function completes successfully else `None`.
fn to_datetime_from_vec(
    source_vec: &Vec<&str>,
    index: i32,
    prefix: &str,
) -> Option<DateTime<Local>> {
    let date_str = to_string(source_vec, index, prefix);
    to_datetime(date_str)
}

/// enum `TaskStatus`
/// This enum represents both the type of task, and its state
/// during the program
///
/// # Variants
/// - **Active**: the task is active and is yet to be done and
/// has not been deleted.
/// - **Completed**: the task is completed.
/// - **Deleted**: the task is removed from the active tasks and
/// is not marked complete.
#[derive(Debug)]
pub enum TaskStatus {
    Active,
    Completed,
    Deleted,
}

impl TaskStatus {
    /// # show
    /// converts a `TaskStatus` property to a convenient String form
    /// for display or writing to files or stdout
    pub fn show(&self) -> String {
        match &self {
            TaskStatus::Active => String::from("Active"),
            TaskStatus::Completed => String::from("Completed"),
            TaskStatus::Deleted => String::from("Deleted"),
        }
    }

    /// # from
    /// creates a new instance of `TaskStatus` from a string input.
    pub fn from(status: &str) -> Self {
        match status {
            "Active" => TaskStatus::Active,
            "Completed" => TaskStatus::Completed,
            "Deleted" => TaskStatus::Deleted,
            _ => TaskStatus::Active,
        }
    }
}

/// enum `DateTimeOption`
/// An enum for holding DateTime data or None if
/// the datetime data does not need a value at the
/// moment.
///
/// # Variants
/// - **DateTime**: variant that holds the local datetime
/// - **None**: variant for unfilled or unavailable
/// datetime data
pub enum DateTimeOption {
    DateTime(DateTime<Local>),
    None,
}

impl DateTimeOption {
    /// # show
    /// converts the `DateTimeOption` to a String for
    /// easy display or writing to a file or stdout
    pub fn show(&self) -> String {
        match &self {
            DateTimeOption::DateTime(dt) => dt.format(DATE_FMT).to_string(),
            DateTimeOption::None => String::from("Not completed"),
        }
    }

    /// # from
    /// converts a string literal to a `DateTimeOption` based
    /// of its parsing. If the string is of the same format
    /// as the form of `DateTimeOption::None` parsed into a
    /// String i.e., `"Not Completed"`, the `None` variant
    /// is returned else the `DateTime` variant is returned.
    pub fn from(display: &str) -> DateTimeOption {
        match display {
            "Not Completed" => DateTimeOption::None,
            _ => {
                let datetime_value = to_datetime(display.to_string());
                match datetime_value {
                    Some(datetime) => DateTimeOption::DateTime(datetime),
                    None => DateTimeOption::None,
                }
            }
        }
    }
}

/// # Task
/// a struct that represents the data model for a task.
///
/// # Members
/// - **time_created**: `DateTime<Local>`
/// - **last_time_modified**: `DateTime<Local>`
/// - **time_finished**: `DateTimeOption`,
/// - **name**: `String`,
/// - **id**: `i32`,
/// - **status**: `TaskStatus`
///
pub struct Task {
    pub time_created: DateTime<Local>,
    pub last_time_modified: DateTime<Local>,
    pub time_finished: DateTimeOption,
    pub name: String,
    pub id: i32,
    pub status: TaskStatus,
}

impl Task {
    /// # from_name_id
    /// creates a new `Task` instance given a task name
    /// and a task id.
    ///
    /// its `status` is set to `TaskStatus::Active`
    ///
    /// its `time_created` is set to `Local::now()`
    ///
    /// its `last_time_modified` is set to `Local::now()`
    ///
    /// its `time_finished` is set to `DateTimeOption::None`
    /// # Arguments
    /// - **name**: the name of the new task
    /// - **id**: the task id for the new task
    /// # Returns
    /// `Task`
    pub fn from_name_id(name: String, id: i32) -> Task {
        Task {
            id,
            name,
            status: TaskStatus::Active,
            time_created: Local::now(),
            last_time_modified: Local::now(),
            time_finished: DateTimeOption::None,
        }
    }

    /// # from_str
    /// parses a string literal (which is expected to have been
    /// generated from the `show` method) to a Task instance.
    /// # Arguments
    /// - **display**: the string literal to be parsed to a
    /// `Task` instance
    /// # Returns
    /// `Some(Task)` if the string is parsed successfully else
    /// `None`.
    pub fn from_str(display: &str) -> Option<Task> {
        // remove the braces expected in the `display`
        let display = display.replace("{", "");
        let display = display.replace("}", "");
        let display = display.trim();
        let collection = display.split(", ").collect::<Vec<&str>>();
        assert_eq!(collection.len(), 6);

        // parse Task members `id` and `name` as `task_id` and `task_name`
        let task_id = to_num(&collection, 0, "Task ID: ")?;
        let task_name = to_string(&collection, 1, "Name: ");

        // parse Task member `status` as `task_status`
        let status_str = to_string(&collection, 2, "Status: ");
        let task_status = TaskStatus::from(&status_str);

        // parse Task members `time_created` and `last_time_modified`
        // as `time_created` and `time_modified`
        let time_created = to_datetime_from_vec(&collection, 3, "Created: ")?;
        let time_modified = to_datetime_from_vec(&collection, 4, "Last Modified: ")?;

        // parse Task member `time_finished`
        let finished_date_str = to_string(&collection, 5, "Finished: ");
        let time_finished = DateTimeOption::from(&finished_date_str);

        // return Task
        Some(Task {
            id: task_id,
            name: task_name,
            status: task_status,
            time_created,
            last_time_modified: time_modified,
            time_finished,
        })
    }

    /// # show
    /// converts a `Task` instance with its members to a String
    pub fn show(&self) -> String {
        format!(
            "{{ Task ID: {}, Name: {}, Status: {}, Created: {}, Last Modified: {}, Finished: {} }}",
            self.id,
            self.name,
            self.status.show(),
            self.time_created.format(DATE_FMT),
            self.last_time_modified.format(DATE_FMT),
            self.time_finished.show()
        )
    }

    /// # set
    /// edits the value of the old value of `Task` member `name`
    /// to the value of `new_name` and modifies the member
    /// `last_time_modified` to the current local datetime at
    /// the time the function is called.
    pub fn set(&mut self, new_name: String) {
        self.name = new_name;
        self.last_time_modified = Local::now();
    }
}

/// # Tasks
/// a struct that is used to track all tasks in the
/// program
///
/// # Members
/// - **active**: vector of active tasks
/// - **completed**: vector of completed tasks
/// - **deleted**: vector of deleted tasks
pub struct Tasks {
    pub active: Vec<Task>,
    pub completed: Vec<Task>,
    pub deleted: Vec<Task>,
}

impl Tasks {
    /// # new
    /// creates a new instance of `Tasks` with
    /// all its members set to empty vectors.
    pub fn new() -> Tasks {
        Tasks {
            active: Vec::<Task>::new(),
            completed: Vec::<Task>::new(),
            deleted: Vec::<Task>::new(),
        }
    }
}
