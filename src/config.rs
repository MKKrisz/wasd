use serde_roxmltree;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::NaiveTime;

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
        if Self::is_active(this) {return;}
        if let Ok(iter) = std::fs::read_dir(&this.directory) {
            for res in iter {
                let file = res.unwrap();
                let file_name = fs::DirEntry::file_name(&file);
                let link_name = Path::new(active_dir).join(this.name.clone() + "_" + &file_name.into_string().unwrap());
                match std::os::unix::fs::symlink(file.path(), &link_name) {
                    Err(e) => {println!("Could not create symlink: {}", e);},
                    _ => {}
                };
            }
        }

        this.active = true;
        println!("Activated {}", &this.name);
    }

    pub fn deactivate(this: &mut Self, active_dir: &str) {
        if !Self::is_active(this) {return;}

        if let Ok(iter) = std::fs::read_dir(&this.directory) {
            for res in iter {
                let file = res.unwrap();
                let file_name = fs::DirEntry::file_name(&file);
                let link_name = Path::new(active_dir).join(this.name.clone() + "_" + &file_name.into_string().unwrap());
                match std::fs::remove_file(&link_name) {
                    Err(e) => {println!("Could not remove file: {}", e);},
                    _ => {}
                };
            }
        }

        this.active = false;
        println!("Deactivated {}", &this.name);
    }

    pub fn mirror_fs_state(this: &mut Self, active_dir: &str) {
        let mut partially_active = false;
        let mut fully_active = true;
        if let Ok(iter) = std::fs::read_dir(&this.directory) {
            for res in iter {
                let file = res.unwrap();
                let file_name = fs::DirEntry::file_name(&file);
                let link_name = Path::new(active_dir).join(this.name.clone() + "_" + &file_name.into_string().unwrap());
                let linked = std::fs::exists(&link_name).unwrap();
                if linked {
                    partially_active = true;
                }
                if !linked {
                    fully_active = false;
                }
            }
        }
        if partially_active && !fully_active {
            println!("Activating partially active source, ignore warnings till \"done\"");
            Source::activate(this, active_dir);
            println!("done");
        }
        this.active = partially_active;
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

impl Timestamp {
    pub fn between(start: &Self, end: &Self, t: &chrono::NaiveTime) -> bool {
        let start = NaiveTime::from_hms_opt(start.hour.into(), start.minute.into(), 0).unwrap();
        let end = NaiveTime::from_hms_opt(end.hour.into(), end.minute.into(), 0).unwrap();

        if (end - start) < chrono::TimeDelta::zero() {
            return !(end <= *t && t < &start)
        }
        start <= *t && t < &end
    }
}
