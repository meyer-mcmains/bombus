use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    net::TcpStream,
    path::{Path, PathBuf},
};

use native_tls::TlsStream;
use serde::{Deserialize, Deserializer, Serialize};

use tungstenite::{connect, stream::Stream, WebSocket};
use ureq::{get, post, Error};

//https://transform.tools/json-to-rust-serde

const BASE_URL: &str = "http://192.168.86.57:1200";
const BASE_SOCKET: &str = "ws://192.168.86.57:1201";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub artist: String,
    pub albums: Vec<Album>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub album_id: String,
    pub title: String,
    pub artist: String,
    pub year: String,
    #[serde(skip_serializing)]
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub track_id: String,
    pub artist: String,
    pub disk: Option<i64>,
    pub length: String,
    pub name: String,
    pub number: i64,
    pub path: String,
}

pub fn get_library() -> Result<Vec<Root>, Error> {
    let path = [BASE_URL, "/library"].join("");
    let json: Vec<Root> = get(&path).call()?.into_json()?;
    // uncomment below to pull library from file
    // let json: Vec<Root> =
    //     serde_json::from_reader(&File::open("offline_library.json").unwrap()).unwrap();
    let albums = json.iter().fold(vec![], |mut acc, artist| {
        acc.extend(artist.albums.clone());
        acc
    });

    serde_json::to_writer(&File::create("library.json").unwrap(), &albums).unwrap();
    Ok(json)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub data: String,
    pub is_dark: bool,
    pub color: String,
}

pub fn get_cover(artist: &str, album: &str) -> Result<(bool, PathBuf), Error> {
    let safe_album = album.replace("/", "_");
    let safe_artist = artist.replace("/", "_");
    let os_artist = OsStr::new(&safe_artist);
    let os_album = OsStr::new(&safe_album);

    // create path to file
    let file = Path::new("artwork")
        .join(os_artist)
        .join(os_album)
        .with_extension("jpg");

    if !file.exists() {
        let path = [BASE_URL, "/artwork"].join("");

        let json: Cover = get(&path)
            .query("artist", artist)
            .query("album", album)
            .query("thumbnail", "true")
            .call()?
            .into_json()?;

        let bytes = base64::decode(json.data).unwrap();

        fs::create_dir_all(Path::new("artwork").join(os_artist))?;

        let mut buffer = File::create(file.clone())?;
        buffer.write_all(&bytes)?;

        return Ok((false, file));
    }

    Ok((true, file))
}

pub fn play_album(artist: &String, album: &String) {
    let path = [BASE_URL, "/play-album"].join("");

    post(&path)
        .query("artist", artist)
        .query("album", album)
        .send_bytes(&[0])
        .ok()
        .unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltAlbum {
    pub album_id: String,
    pub title: String,
    pub artist: String,
    pub year: String,
    #[serde(default = "default_position")]
    pub position: usize,
    #[serde(default = "default_position")]
    pub sorted_position: usize,
}

pub fn get_library_from_file() -> Vec<AltAlbum> {
    serde_json::from_reader(&File::open("library.json").unwrap()).unwrap()
}

pub fn save_library_to_file(library: Vec<AltAlbum>) {
    serde_json::to_writer_pretty(&File::create("library.json").unwrap(), &library).unwrap();
}

fn default_position() -> usize {
    0
}

pub fn create_socket() -> WebSocket<Stream<TcpStream, TlsStream<TcpStream>>> {
    let (socket, _response) = connect(BASE_SOCKET).expect("Can't connect");
    socket
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum NotificationTypes {
    Startup,
    PlayCountersChanged,
    TrackChanged,
    NowPlayingListEnded,
    PlayStateChanged,
    VolumeLevelChanged,
    NowPlayingListChanged,
    PlayingTracksChanged,
}

fn default_notification_position() -> u64 {
    0
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    #[serde(default, deserialize_with = "bool_from_string")]
    pub play_state: bool,
    #[serde(
        default = "default_notification_position",
        deserialize_with = "seconds_from_milliseconds"
    )]
    pub position: u64,
    #[serde(rename = "notification")]
    pub notification_type: NotificationTypes,
    #[serde(
        default = "default_notification_position",
        deserialize_with = "seconds_from_milliseconds"
    )]
    pub duration: u64,
    pub sound_graph: Vec<f64>,
}

// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "Playing" => Ok(true),
        "Paused" => Ok(false),
        "Stopped" => Ok(false),
        _ => Ok(false),
    }
}

fn seconds_from_milliseconds<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(u64::deserialize(deserializer)? / 1000)
}

pub fn notification_to_json(notification: String) -> Notification {
    println!("{}", notification);
    serde_json::from_str(&notification).unwrap()
}
