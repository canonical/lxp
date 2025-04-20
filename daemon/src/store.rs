use std::{fs::{self, DirEntry}, path::PathBuf, str::FromStr};

use anyhow::{bail, Error, Result};
use serde::{de::DeserializeOwned, Serialize};


pub fn store<T>(page: &String, book: &str, content: &T) -> Result<()> where T: Serialize {
    let file: PathBuf = PathBuf::from_str(format!("{}.yaml", page).as_str())?;

    match dirs::home_dir() {
        Some(home_directory) => {
            let directory: PathBuf = home_directory.join("lxp").join(book);
            fs::create_dir_all(&directory)?;

            let raw_content: String = serde_yaml::to_string(content)?;
            fs::write(&directory.join(file), raw_content)?;
            Ok(())
        },
        None => bail!("Could not find /home/$USER/ directory"),
    }
}

pub fn retrieve<T>(page: &String, book: &str) -> Result<T> where T: DeserializeOwned {
    let file: PathBuf = PathBuf::from_str(format!("{}.yaml", page).as_str())?;

    match dirs::home_dir() {
        Some(home_directory) => {
            let directory: PathBuf = home_directory.join("lxp").join(book);
            let raw_content: String = fs::read_to_string(&directory.join(file))?;
            let content = serde_yaml::from_str(&raw_content)?;
            Ok(content)
        },
        None => bail!("Could not find /home/$USER/ directory"),
    }
}

pub fn list<T>(book: &str) -> Result<Vec<T>> where T: DeserializeOwned {
    match dirs::home_dir() {
        Some(home_directory) => {
            let directory: PathBuf = home_directory.join("lxp").join(book);

            if !directory.exists() {
                return Ok(Vec::new());
            }

            let mut pages: Vec<T> = Vec::new();
            for entry in fs::read_dir(&directory)? {
                let entry: DirEntry = entry?;
                let path: PathBuf = entry.path();
                let page: String = entry.file_name()
                    .into_string()
                    .map_err(|os_string| Error::msg(format!("Could parse \"{:?}\"", os_string)))?
                    .strip_suffix(".yaml")
                    .unwrap_or_default()
                    .to_string();

                let is_yaml: bool = path.extension().map_or(false, |extension| extension == "yaml" || extension == "yml");

                if path.is_file() && is_yaml {
                    let page: T = retrieve(&page, book)?;
                    pages.push(page);
                }
            }

            Ok(pages)
        },
        None => bail!("Could not find /home/$USER/ directory"),
    }
}
