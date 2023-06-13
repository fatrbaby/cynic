mod project;
mod php;

use project::Project;
use std::env::current_dir;

fn main() {
    let cwd = current_dir().expect("can not get current path");
    let p = Project::new(cwd.as_path());

    p.show_setting();
}
