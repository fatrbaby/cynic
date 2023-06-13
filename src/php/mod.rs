use std::path::PathBuf;

#[derive(Debug)]
pub struct Php {
    version: String,
    binary: PathBuf,
}

impl Php {
    pub fn find() -> Box<Php> {
        let builds = php_discovery::discover();

        for build in builds.unwrap() {
            return Box::new(Php {
                version: build.version.to_string(),
                binary: (*build.binary).to_owned(),
            });
        }

        Box::new(Php {
            version: "".to_string(),
            binary: PathBuf::new(),
        })
    }
}
