use std::{fs::File, net::Ipv4Addr, path::PathBuf, str::FromStr};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// get the project cache directory
pub fn get_cache_directory() -> PathBuf {
    let project_dirs = ProjectDirs::from("app", "meyer-mcmains", "bombus").unwrap();
    PathBuf::from(project_dirs.cache_dir())
}

/// return the location of the artwork cache
pub fn get_artwork_cache_directory() -> PathBuf {
    let artwork_cache = get_cache_directory().join("artwork");
    artwork_cache
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Library {
    pub name: String,
    pub ip: Ipv4Addr,
    pub color: String,
}

type Libraries = Vec<Library>;

/// return the library list file - create it if it does not already exist
pub fn get_libraries() -> PathBuf {
    let library_list = get_cache_directory().join("libraries.json");

    if !library_list.exists() {
        let empty_list: Libraries = vec![];
        serde_json::to_writer(&File::create(&library_list).unwrap(), &empty_list).unwrap();
    }

    library_list
}

pub fn add_library(name: String, ip: &str, color: String) {
    let library = Library {
        name,
        ip: Ipv4Addr::from_str(&ip).unwrap(),
        color,
    };

    let library_list_path = get_libraries();
    let mut library_list: Libraries =
        serde_json::from_reader(&File::open(&library_list_path).unwrap()).unwrap();

    library_list.push(library);
    serde_json::to_writer(&File::create(&library_list_path).unwrap(), &library_list).unwrap();
}
