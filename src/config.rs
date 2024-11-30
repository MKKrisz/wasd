use serde_roxmltree;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Config {
    pub update_interval: u64,
    pub active_directory: Option<String>,
    pub source: Vec<Source>,
}
impl Config {
    pub fn new(path: &str) -> Config {
        let file = fs::read_to_string(&path).unwrap();
        serde_roxmltree::from_str(&file).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub name: String,
    #[serde(rename = "dir")]
    pub directory: String,
    pub interval: Vec<ActiveInterval>,
}
impl Source {
    pub fn is_active(this: &Self, active_dir: &str) -> bool {
        let symlink_name = Path::new(active_dir).join(&this.name);
        std::fs::exists(&symlink_name).expect("IO error")
    }
    pub fn activate(this: &Self, active_dir: &str) {
        let symlink_name = Path::new(active_dir).join(&this.name);
        if Self::is_active(this, active_dir) {return;}
        std::os::unix::fs::symlink(&this.directory, &symlink_name).expect("uhoh");
        println!("Activated {}", &this.name);
    }
    pub fn deactivate(this: &Self, active_dir: &str) {
        let symlink_name = Path::new(active_dir).join(&this.name);
        if !Self::is_active(this, active_dir) {return;}
        std::fs::remove_file(&symlink_name).expect("uhoh");
        println!("Deactivated {}", &this.name);
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveInterval {
    pub start: Timestamp,
    pub end: Timestamp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timestamp {
    pub hour: u8,
    #[serde(rename = "min")]
    pub minute: u8,
}
