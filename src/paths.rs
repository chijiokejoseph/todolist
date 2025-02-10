use std::env;
use std::path::PathBuf;

const NAME_PATH: &str = "names.txt";
const DATA_PATH: &str = "data";

/// # get_path
/// returns a specialized path to hold data in the program.
/// This path is relative to the main function block, and
/// the returned path is of the form "../data" i.e., go back
/// to the parent dir of main i.e., 'src' and then look for
/// 'data'.
/// # Arguments
/// none
/// # Returns
/// path that holds the parent dir (`data`) in which the data
/// in the program is stored.
pub fn get_path() -> PathBuf {
    // get current work dir
    let work_dir_option = match env::current_dir() {
        Ok(cwd) => Some(cwd),
        Err(_) => None,
    };

    // set default dir if getting current work dir
    // does not succeed.
    let default = String::from(r"C:\todolist");

    // get parent dir of the current work dir
    // set to default dir if getting current work dir fails
    let parent_dir = match work_dir_option {
        Some(cwd) => cwd,
        None => PathBuf::from(&default),
    };

    // convert parent dir to String
    let mut data_path = match parent_dir.to_str() {
        Some(path) => String::from(path),
        None => default,
    };

    // adds the data dir to the previous path String
    data_path.push('\\');
    data_path.push_str(DATA_PATH);

    // returns the new path (String) as PathBuf
    PathBuf::from(data_path)
}

/// # get_name_path
/// returns the path to the 'names.txt' file which is
/// used in storing the name of the user and is in
/// turn stored in the DATA_PATH of the program.
///
/// - DATA_PATH => "../data" relative to src/main
///
/// - returned path => "../data/names.txt' relative
/// to src/main
///
/// This DATA_PATH is obtained by calling `get_path`
///
/// # Arguments
/// none
/// # Returns
/// path to the 'names.txt' file used to store the
/// name of the user of the program.
pub fn get_name_path() -> PathBuf {
    get_path().join(NAME_PATH)
}

/// # get_file_path
/// returns the path to any file passed into the
/// function through the `filename` argument beginning
/// with the DATA_PATH of the program as its main dir.
///
///     DATA_PATH => "../data" relative to src/main
///     returned path => "../data/<filename>" relative
///     to src/main
///
/// This DATA_PATH is obtained by calling `get_path`
///
/// # Arguments
/// - **filename**: the name and extension of the file
/// whose path is being returned with DATA_PATH as its
/// main dir.
/// # Returns
/// the path to `filename` with DATA_PATH as its main dir.
pub fn get_file_path(filename: &str) -> PathBuf {
    get_path().join(filename)
}
