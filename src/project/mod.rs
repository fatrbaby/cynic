use std::fs;
use std::fs::File;
use std::io::Write;
use directories::BaseDirs;
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Project {
    setting: Setting,
}

impl Project {
    pub fn new(workdir: &Path) -> Project {
        match BaseDirs::new() {
            None => {
                Project {
                    setting: Setting::of(Path::new("~"), workdir),
                }
            }
            Some(base_dir) => {
                Project {
                    setting: Setting::of(base_dir.home_dir(), workdir),
                }
            }
        }
    }
}

impl Project {
    pub fn show_setting(&self) {
        println!("{:?}", self.setting);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    php_version: String,
    php_binary: String,
    port: u16,
    index: String,
}

impl Setting {
    pub fn of(path: &Path, workdir: &Path) -> Setting {
        let mut hasher = Sha256::new();
        hasher.update(workdir.to_string_lossy().as_bytes());
        let hash = format!("{:X}", hasher.finalize());

        let mut setting_path = PathBuf::new();
        setting_path.push(path);
        setting_path.push(".cynic");
        setting_path.push(hash);
        setting_path.push("setting.json");

        if setting_path.exists() {
            return load_config_from(setting_path.as_path());
        }

        create_new_config(setting_path.as_path())
    }
}

fn load_config_from(path: &Path) -> Setting {
    let setting = fs::read_to_string(path).unwrap();

    let conf: Setting = serde_json::from_str(&setting).unwrap();

    conf
}

fn create_new_config(path: &Path) -> Setting {
    let conf = Setting {
        php_version: "8.0".to_string(),
        php_binary: "/usr/local/bin/php".to_string(),
        port: 7788,
        index: "".to_string(),
    };

    let json = serde_json::to_string(&conf).expect("build config json fail.");

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
mod tests {}