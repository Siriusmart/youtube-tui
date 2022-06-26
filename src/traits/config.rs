use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fmt::Debug,
    fs::{self, rename},
    io::Write,
};

pub trait ConfigItem<'a> {
    type Struct: Serialize + DeserializeOwned + Debug + Clone + Default;
    const FILE_NAME: &'static str;

    fn load() -> Result<Self::Struct, Box<dyn Error>>
    where
        Self: Sized,
    {
        let mut config_path = home::home_dir().expect("Cannot get your home directory");
        let mut config = Self::Struct::default();
        config_path.push(".config");
        config_path.push("youtube-tui");
        config_path.push(Self::FILE_NAME);

        if config_path.exists() {
            let content = fs::read_to_string(config_path.as_os_str())?;
            match serde_yaml::from_str(&content) {
                Ok(config_from_str) => config = config_from_str,
                Err(_) => {
                    let mut new_path = config_path.clone();
                    new_path.pop();
                    new_path.push(format!("{}.old", Self::FILE_NAME));

                    rename(&config_path, &new_path)?;
                }
            }
        }

        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config_path.as_os_str())?;

        write!(file, "{}", serde_yaml::to_string(&config)?)?;

        Ok(config)
    }
}
