use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    net::TcpStream,
    path::PathBuf,
    sync::Mutex,
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

pub type TWebSocket = WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>;

static BASE_URL: Mutex<String> = Mutex::new(String::new());
static BASE_SOCKET: Mutex<String> = Mutex::new(String::new());

/// set the library connection
/// # TODO
/// - allow the user to use a different port (backend needs to support this first)
pub fn set_connection(ip: String) {
    *BASE_URL.lock().unwrap() = format!("http://{}:1200", ip);
    *BASE_SOCKET.lock().unwrap() = format!("ws://{}:1200/notification", ip);
}

/// create and return a http client
fn create_client() -> Client {
    // get a client builder
    Client::builder().build().unwrap()
}

/// make a GET request to the BASE_URL at a specific path``
fn get(path: &str) -> RequestBuilder {
    let base = BASE_URL.lock().unwrap();
    create_client().get(format!("{base}/api/{path}"))
}

/// make a POST request to the BASE_URL at a specific path
fn post(path: &str) -> RequestBuilder {
    let base = BASE_URL.lock().unwrap();
    create_client().post(format!("{base}/api/{path}"))
}

// https://transform.tools/json-to-rust-serde
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
    let response = get("library").timeout(Duration::from_secs(10)).send();

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

pub fn create_socket() -> Option<WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>> {
    let base = BASE_SOCKET.lock().unwrap();

    if base.is_empty() {
        None
    } else {
        let (socket, _response) = connect(base.to_string()).expect("Can't connect");
        Some(socket)
    }
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
