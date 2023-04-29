use home::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

use crate::config::WriteConfig;

pub const EXTENSION: &str = "yml";

/// Trait for loading, saving config files
pub trait ConfigTrait {
    const LABEL: &'static str;

    fn load(write: WriteConfig) -> Result<Box<Self>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Default + Clone,
    {
        let home_dir = home_dir().unwrap();
        let config_path =
            home_dir.join(format!(".config/youtube-tui/{}.{}", Self::LABEL, EXTENSION));

        // The config struct
        let config: Self = {
            || -> Result<Self, Box<dyn Error>> {
                let content = match fs::read_to_string(&config_path) {
                    Ok(content) => content,
                    // If the config file does not exist returns Self::defult()
                    Err(_) => return Ok(Self::default()),
                };

                match serde_yaml::from_str::<Self>(&content) {
                    Ok(config) => Ok(config),
                    // If there is error parsing the json file, create a backup with current time and returns
                    // Self::default()
                    Err(_) => {
                        let mut new_path = config_path.clone();
                        new_path.pop();
                        new_path.push(format!(
                            "{}-{}.{}",
                            Self::LABEL,
                            chrono::offset::Local::now(),
                            EXTENSION
                        ));
                        fs::rename(&config_path, &new_path)?;
                        Ok(Self::default())
                    }
                }
            }
        }()?;

        // Overwrites the old config file with added options (if any),
        // but it also removes things like comments in the old config file

        match write {
            WriteConfig::Must => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&config_path)?;
                file.write_all(serde_yaml::to_string(&config)?.as_bytes())?
            }
            WriteConfig::Try => {
                if let Ok(mut file) = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&config_path)
                {
                    let _ = file.write_all(serde_yaml::to_string(&config)?.as_bytes());
                }
            }
            WriteConfig::Dont => {}
        }

        Ok(Box::new(config))
    }
}
