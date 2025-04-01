pub mod internal;

pub use internal::inputs::prelude::{input_num, input_option, input_str};
pub use internal::name::ret_name_loop;
pub use internal::paths::{get_file_path, get_name_path, get_path};
pub use internal::tasks::prelude::{
    add, check, delete, edit, load, ret_last_task_id, save, view, Tasks,
};
