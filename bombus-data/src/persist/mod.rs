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
