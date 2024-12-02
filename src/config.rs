use serde_roxmltree;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Config {
    pub update_interval: u64,
    pub active_directory: Option<String>,
    #[serde(default)]
    pub source: Vec<Source>,
}
impl Config {
    pub fn new(path: &str) -> Config {
        let file = fs::read_to_string(&path).unwrap();
        serde_roxmltree::from_str(&file).unwrap()
    }
}

impl Config {
    pub fn mirror_fs_state(this: &mut Self) {
        for src in &mut this.source {
            Source::mirror_fs_state(src, &this.active_directory.clone().unwrap());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub name: String,
    #[serde(rename = "dir")]
    pub directory: String,
    #[serde(default)]
    pub interval: Vec<ActiveInterval>,
    #[serde(skip)]
    pub active: bool,
}

impl Source {
    pub fn is_active(this: &Self) -> bool {
        this.active
        // Old impl.
        //let symlink_name = Path::new(active_dir).join(&this.name);
        //std::fs::exists(&symlink_name).expect("IO error")
    }

    pub fn activate(this: &mut Self, active_dir: &str) {
        let symlink_name = Path::new(active_dir).join(&this.name);
        if Self::is_active(this) {return;}
        std::os::unix::fs::symlink(&this.directory, &symlink_name).expect("uhoh");
        this.active = true;
        println!("Activated {}", &this.name);
    }

    pub fn deactivate(this: &mut Self, active_dir: &str) {
        let symlink_name = Path::new(active_dir).join(&this.name);
        if !Self::is_active(this) {return;}
        match std::fs::remove_file(&symlink_name) {
            Ok(()) => {},
            Err(e) => {println!("Could not remove symlink! (Maybe doesn't exist): {} {}", e, symlink_name.to_str().unwrap());}
        };
        this.active = false;
        println!("Deactivated {}", &this.name);
    }

    pub fn mirror_fs_state(this: &mut Self, active_dir: &str) {
        let symlink_name = Path::new(active_dir).join(&this.name);
        this.active = std::fs::exists(&symlink_name).expect("IO error");
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
    #[serde(default)]
    pub minute: u8,
}
