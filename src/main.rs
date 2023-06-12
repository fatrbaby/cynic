mod project;

use std::env::current_dir;
use project::Project;

fn main() {
    let p = Project::new(get_cwd());

    p.show_work_dir();
}

fn get_cwd() -> String {
    let cwd = current_dir().expect("can not get current path");
    cwd.to_string_lossy().to_string()
}
