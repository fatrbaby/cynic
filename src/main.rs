mod project;

use std::env::current_dir;
use project::Project;

fn main() {
    let cwd = current_dir().expect("can not get current path");
    let p = Project::new(cwd.as_path());

    p.show_setting();
}

