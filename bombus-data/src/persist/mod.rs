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
    pub color: u32,
}

type Libraries = Vec<Library>;

/// return the library list file - create it if it does not already exist
pub fn get_libraries_path() -> PathBuf {
    let library_list = get_cache_directory().join("libraries.json");

    if !library_list.exists() {
        let empty_list: Libraries = vec![];
        serde_json::to_writer(&File::create(&library_list).unwrap(), &empty_list).unwrap();
    }

    library_list
}

/// add a new library
pub fn add_library(name: String, ip: &str, color: u32) {
    let library = Library {
        name,
        ip: Ipv4Addr::from_str(&ip).unwrap(),
        color,
    };

    let library_list_path = get_libraries_path();
    let mut library_list: Libraries =
        serde_json::from_reader(&File::open(&library_list_path).unwrap()).unwrap();

    library_list.push(library);
    serde_json::to_writer(&File::create(&library_list_path).unwrap(), &library_list).unwrap();
}

/// remove a library
pub fn remove_library(name: String) -> usize {
    let library_list_path = get_libraries_path();
    let mut library_list: Libraries =
        serde_json::from_reader(&File::open(&library_list_path).unwrap()).unwrap();

    let remove_index = library_list.iter().position(|library| library.name == name);

    match remove_index {
        Some(value) => {
            library_list.remove(value);
            serde_json::to_writer(&File::create(&library_list_path).unwrap(), &library_list)
                .unwrap();
            value
        }
        None => todo!(),
    }
}

/// get the list of libraries
pub fn get_libraries() -> Libraries {
    let libraries_path = get_libraries_path();
    serde_json::from_reader(&File::open(&libraries_path).unwrap()).unwrap()
}
