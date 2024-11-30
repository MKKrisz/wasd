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

    let mut last_check_time = chrono::offset::Local::now();
    let mut time_now: DateTime<Local>;

    loop {
        let config_mod_time = fs::metadata(&config_file_path).unwrap().modified().unwrap();
        if config_last_modified != config_mod_time {
            config = config::Config::new(&config_file_path);
            config_last_modified = config_mod_time;
            println!("Config changed!");
        }
        time_now = chrono::offset::Local::now();
        
        for src in &config.source {
            for interval in &src.interval {
                let start = NaiveTime::from_hms_opt(interval.start.hour.into(), interval.start.minute.into(), 0).unwrap();
                let end = NaiveTime::from_hms_opt(interval.end.hour.into(), interval.end.minute.into(), 0).unwrap();

                if time_now.time() > start && last_check_time.time() < start {
                    Source::activate(src, &config.active_directory.clone().unwrap());
                }
                if time_now.time() > end && last_check_time.time() < end {
                    Source::deactivate(src, &config.active_directory.clone().unwrap());
                }
            }
        }

        last_check_time = time_now;
        std::thread::sleep(std::time::Duration::from_secs(config.update_interval));
    }
}
