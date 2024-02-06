use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    net::TcpStream,
    path::PathBuf,
    time::Duration,
};

use native_tls::TlsStream;
use reqwest::{
    blocking::{Client, RequestBuilder},
    Error,
};
use serde::{Deserialize, Serialize};
use tungstenite::{connect, stream::Stream, WebSocket};
pub mod persist;

//https://transform.tools/json-to-rust-serde

const BASE_URL: &str = "http://192.168.86.57:1200";
const BASE_SOCKET: &str = "ws://192.168.86.57:1200/notification";

/// create and return a http client
fn create_client() -> Client {
    // get a client builder
    Client::builder().build().unwrap()
}

/// make a GET request to the BASE_URL at a specific path``
fn get(path: &str) -> RequestBuilder {
    create_client().get(format!("{BASE_URL}/api/{path}"))
}

/// make a POST request to the BASE_URL at a specific path
fn post(path: &str) -> RequestBuilder {
    create_client().post(format!("{BASE_URL}/api/{path}"))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Root {
    pub artist: String,
    pub albums: Vec<Album>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub year: String,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Track {
    pub id: String,
    pub album: String,
    pub artist: String,
    pub disk: Option<u64>,
    pub length: String,
    pub duration: i64,
    pub title: String,
    pub number: i32,
    pub uri: String,
}

/// Get the library from musicbee
/// if the request fails load the offline cache
///
/// # TODO
/// - handle response error and no cache
/// - load offline library first and merge in any changes from musicbee??
pub fn get_library() -> Result<Vec<Root>, Error> {
    let library_path = persist::get_cache_directory().join("library.json");
    let response = get("library").timeout(Duration::from_secs(1)).send();

    match response {
        Ok(_) => {
            let json = response?.json::<Vec<Root>>()?;
            // save library to cache
            serde_json::to_writer(&File::create(library_path).unwrap(), &json).unwrap();
            return Ok(json);
        }

        Err(_) => {
            return Ok(serde_json::from_reader(&File::open(&library_path).unwrap()).unwrap());
        }
    }
}

pub fn get_cover(artist: &str, album: &str) -> Result<(bool, PathBuf), Error> {
    let safe_album = album.replace("/", "_");
    let safe_artist = artist.replace("/", "_");
    let os_artist = OsStr::new(&safe_artist);
    let os_album = OsStr::new(&safe_album);

    let artwork_cache = persist::get_artwork_cache_directory();

    // create path to file
    let file = artwork_cache
        .join(os_artist)
        .join(os_album)
        .with_extension("jpg");

    if !file.exists() {
        let params = [("artist", artist), ("album", album), ("thumbnail", "true")];
        let response = get("artwork").form(&params).send()?;
        let bytes = response.bytes()?;

        fs::create_dir_all(artwork_cache.join(os_artist)).unwrap();

        let mut buffer = File::create(file.clone()).unwrap();
        buffer.write_all(&bytes).unwrap();

        return Ok((false, file));
    }

    Ok((true, file))
}

pub fn play_album(artist: &str, album: &str) {
    let params = [("artist", artist), ("album", album)];
    post("play-album").form(&params).send().unwrap();
}

pub fn play_pause() {
    post("play-pause").send().unwrap();
}

pub fn next_track() {
    post("next-track").send().unwrap();
}

pub fn previous_track() {
    post("previous-track").send().unwrap();
}

pub fn create_socket() -> WebSocket<Stream<TcpStream, TlsStream<TcpStream>>> {
    let (socket, _response) = connect(BASE_SOCKET).expect("Can't connect");
    socket
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum NotificationTypes {
    PluginStartup,
    PlayCountersChanged,
    TrackChanged,
    NowPlayingListEnded,
    PlayStateChanged,
    VolumeLevelChanged,
    NowPlayingListChanged,
    PlayingTracksChanged,
}

#[derive(Clone, Debug, Deserialize)]
pub enum PlayState {
    Playing,
    Paused,
    Stopped,
}

fn default_notification_position() -> i64 {
    0
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Notification {
    pub play_state: PlayState,
    #[serde(default = "default_notification_position")]
    pub position: i64,
    #[serde(rename = "Notification")]
    pub notification_type: NotificationTypes,
    pub track: Track,
    // pub sound_graph: Vec<f64>,
}

pub fn notification_to_json(notification: String) -> Notification {
    println!("{}", notification);
    serde_json::from_str(&notification).unwrap()
}
