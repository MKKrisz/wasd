// Constantly running service
// Checks the config file if it changed
// Checks the wallpapers in the active folder defined in config
// If some are missing, deletes them
// If some are there when they don't need to, it deletes them

mod config;

use std::fs;
use std::env;

use chrono::NaiveTime;
use chrono::DateTime;
use chrono::Local;

use crate::config::Config;
use crate::config::Source;
use crate::config::Timestamp;

fn main() {
    if env::consts::OS == "windows" {
        panic!("Hülye vagy");
    }

    let config_file_path = env::vars().filter(|x| {x.0 == "HOME"}).next().unwrap().1 + "/.config/wasd/config.xml";
    if !fs::exists(&config_file_path).expect("expect") {
        println!("Config file does not exist!!");
        return;
    }
    let mut config_last_modified = fs::metadata(&config_file_path).unwrap().modified().unwrap();
    let mut config = Config::new(&config_file_path);

    let mut time_now: DateTime<Local>;
    
    Config::mirror_fs_state(&mut config);
    loop {
        let config_mod_time = fs::metadata(&config_file_path).unwrap().modified().unwrap();
        if config_last_modified != config_mod_time {
            config = config::Config::new(&config_file_path);
            Config::mirror_fs_state(&mut config);
            config_last_modified = config_mod_time;
            println!("Config changed!");
        }
        time_now = chrono::offset::Local::now();
        
        for src in &mut config.source {
            let mut wanted_state = false;
            for interval in &src.interval {
                if Timestamp::between(&interval.start, &interval.end, &time_now.time()){
                    wanted_state = true;
                }
            }
            if wanted_state {Source::activate(src, &config.active_directory.clone().unwrap());}
            else {Source::deactivate(src, &config.active_directory.clone().unwrap());}
        }

        std::thread::sleep(std::time::Duration::from_secs(config.update_interval));
    }
}
