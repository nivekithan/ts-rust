use std::path::PathBuf;

use path_absolutize::Absolutize;

pub fn convert_to_absolute_path(relative_file_name: &str, absolute_dir: &str) -> String {
    let mut absolute_path = PathBuf::from(absolute_dir);
    absolute_path.push(relative_file_name);
    return absolute_path
        .absolutize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}
