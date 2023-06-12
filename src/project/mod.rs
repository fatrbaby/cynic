use std::fs;
use std::fs::File;
use std::io::Write;
use directories::BaseDirs;
use std::path::Path;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Project {
    conf: Config,
}

impl Project {
    pub fn new(workdir: String) -> Project {
        let mut home = "~".to_owned();

        if let Some(base_dir) = BaseDirs::new() {
            home = (&base_dir.home_dir().to_string_lossy()).parse().unwrap();
        }

        Project {
            conf: Config::of(home, workdir),
        }
    }
}

impl Project {
    pub fn show_work_dir(&self) {
        println!("{:?}", self.conf);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    php_version: String,
    php_binary: String,
    port: u16,
    index: String,
}

impl Config {
    pub fn of(file_path: String, workdir: String) -> Config {
        let mut hasher = Sha256::new();
        hasher.update(workdir);
        let hash = format!("{:X}", hasher.finalize());
        let setting_path = format!("{}/{}/{}/setting.json", file_path, ".cynic", hash);
        let f = Path::new(&setting_path);

        if f.exists() {
            return load_config_from(f.to_string_lossy().to_string());
        }

        create_new_config(f.to_string_lossy().to_string())
    }
}

fn load_config_from(path: String) -> Config {
    let setting = fs::read_to_string(path).unwrap();

    let conf: Config = serde_json::from_str(&setting).unwrap();

    conf
}

fn create_new_config(path: String) -> Config {
    let conf = Config {
        php_version: "8.0".to_string(),
        php_binary: "/usr/local/bin/php".to_string(),
        port: 7788,
        index: "".to_string(),
    };

    let json = serde_json::to_string(&conf).expect("build config json fail.");

    let path = Path::new(&path);

    if let Some(dir) = path.parent() {
        if !dir.exists() {
            fs::create_dir_all(dir).expect("create setting directory fail.");
        }
    };

    match File::create(path) {
        Ok(mut file) => {
            file.write_all(json.as_bytes()).unwrap();
        }
        Err(why) => {
            println!("{}", why);
        }
    }

    conf
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};
    use hex_literal::hex;

    #[test]
    fn test_project_hash() {
        let result = Sha256::digest(b"/Users/fatrbaby/workspaces/rust/cynic");

        assert!(result[..], hex!("4eaecc89ede85b11f75192aad974c9be097e30937d8a6ff2a31593db190a54bd"));
    }
}