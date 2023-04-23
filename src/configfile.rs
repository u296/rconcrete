use std::{
    fs,
    io::{BufReader, BufWriter, ErrorKind},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Default, Serialize, Deserialize)]
pub struct Configuration {
    pub known_servers: Vec<crate::Server>,
}

pub struct ConfigurationFile {
    file_path: PathBuf,
}

impl ConfigurationFile {
    pub fn build_default() -> Result<Self, Error> {
        Ok(Self {
            file_path: Self::get_default_file_path()?,
        })
    }

    fn get_default_file_path() -> Result<PathBuf, Error> {
        let mut dir = directories::ProjectDirs::from("", "", clap::crate_name!())
            .ok_or(Error::FailedToFetchConfigPath)?
            .config_local_dir()
            .to_owned();

        dir.push("config.json");

        let config_file = dir;

        Ok(config_file)
    }

    pub fn read(&self) -> Result<Configuration, Error> {
        match fs::File::open(&self.file_path) {
            Ok(config_file) => {
                let reader = BufReader::new(config_file);
                Ok(serde_json::from_reader(reader).map_err(Error::ConfigParseError)?)
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Ok(Default::default()),
                _ => Err(Error::IoError(e)),
            },
        }
    }

    pub fn write(&self, config: &Configuration) -> Result<(), Error> {
        match fs::create_dir_all(self.file_path.parent().unwrap()) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                _ => return Err(Error::IoError(e)),
            },
        }
        let config_file = fs::File::create(&self.file_path).map_err(Error::IoError)?;
        let writer = BufWriter::new(config_file);
        serde_json::to_writer_pretty(writer, config).map_err(Error::ConfigWriteError)
    }
}
