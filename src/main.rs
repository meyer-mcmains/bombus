use crate::utils::slint_modules::{Album, AppWindow, Logic, Theme, Track};
use crate::utils::theme::get_theme;
use bombus_data::*;
use slint::{ComponentHandle, Image, Model, ModelExt, ModelRc, SharedPixelBuffer, VecModel};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig};
use std::{
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod utils;

const PLATFORM_CONFIG: souvlaki::PlatformConfig<'_> = PlatformConfig {
    dbus_name: "bombus",
    display_name: "Bombus",
    hwnd: None,
};

const PLACEHOLDER_IMAGE: &[u8; 32346] = include_bytes!("./assets/cover.jpg");

/// load the album cover based the artist and album title
/// setting the fallback if the cover does not exist
fn load_cover(artist: &str, album: &str) -> Image {
    let safe_artist = artist.replace('/', "_");
    let safe_album = album.replace('/', "_");
    let source_image = image::load_from_memory(PLACEHOLDER_IMAGE)
        .unwrap()
        .into_rgba8();
    let fallback_cover = slint::Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
        source_image.as_raw(),
        source_image.width(),
        source_image.height(),
    ));

    let artwork_cache = get_artwork_cache_directory();

    Image::load_from_path(
        &artwork_cache
            .join(safe_artist)
            .join(safe_album)
            .with_extension("jpg"),
    )
    .unwrap_or(fallback_cover)
}

/// load the album cover from an already verified path
fn load_cover_from_path(path: &Path) -> Image {
    Image::load_from_path(path).unwrap()
}

fn handle_play_state_change(controls: &mut MediaControls, notification: Notification) {
    match notification.play_state {
        PlayState::Paused => {
            controls
                .set_playback(souvlaki::MediaPlayback::Paused {
                    progress: Some(souvlaki::MediaPosition(Duration::from_millis(
                        notification.position,
                    ))),
                })
                .unwrap();
        }
        PlayState::Playing => {
            controls
                .set_playback(souvlaki::MediaPlayback::Playing {
                    progress: Some(souvlaki::MediaPosition(Duration::from_millis(
                        notification.position,
                    ))),
                })
                .unwrap();
        }
        PlayState::Stopped => {
            controls
                .set_playback(souvlaki::MediaPlayback::Stopped)
                .unwrap();
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;

    let color = get_theme();
    window.global::<Theme>().set_color(color);

    // load the library
    let library = get_library().unwrap();
    // make a clone so we can operate on the library in 2 separate threads
    // TODO improve this
    let library_copy = library.clone();

    let window_handle_weak = window.as_weak();
    thread::spawn(move || {
        window_handle_weak
            .upgrade_in_event_loop(move |handle| {
                let albums_model: Rc<VecModel<Album>> = Rc::new(VecModel::from(vec![]));

                // transform the library into structs that can be consumed by the ui
                // TODO serialize into the UI structs directly
                library.into_iter().for_each(|artist| {
                    artist.albums.into_iter().for_each(|album| {
                        let tracks: Vec<Track> = album
                            .tracks
                            .into_iter()
                            .map(|track| Track {
                                duration: track.length.into(),
                                name: track.name.into(),
                                number: track.number as i32,
                            })
                            .collect();

                        let image = load_cover(&album.artist, &album.title);

                        let album: Album = Album {
                            id: album.album_id.into(),
                            artist: album.artist.into(),
                            title: album.title.into(),
                            image,
                            tracks: ModelRc::new(VecModel::from(tracks)),
                        };

                        albums_model.push(album);
                    })
                });

                handle.set_albums(albums_model.into());
            })
            .unwrap();
    });

    let window_handle_weak_mutex = Arc::new(Mutex::new(window.as_weak()));
    thread::spawn(move || {
        for artist in library_copy.into_iter() {
            for album in artist.albums {
                let window_handle_clone = Arc::clone(&window_handle_weak_mutex);

                thread::spawn(move || {
                    let window_handle = window_handle_clone.lock().unwrap();

                    let (exists, path) = get_cover(&album.artist, &album.title).unwrap();

                    // TODO skip event loop is cover already exists
                    if !exists {
                        window_handle
                            .upgrade_in_event_loop(move |handle| {
                                // https://github.com/slint-ui/slint/discussions/2329#discussioncomment-5213994
                                let binding = handle.get_albums();
                                let ui_albums = binding
                                    .as_any()
                                    .downcast_ref::<slint::VecModel<Album>>()
                                    .unwrap();

                                let ui_album_index = ui_albums
                                    .iter()
                                    .position(|ui_album| ui_album.id == album.album_id);

                                if let Some(row) = ui_album_index {
                                    ui_albums.row_data_tracked(row);
                                    let mut data = ui_albums.row_data(row).unwrap();
                                    data.image = load_cover_from_path(&path);
                                    ui_albums.set_row_data(row, data);
                                }
                            })
                            .unwrap();
                    }

                    // drop mutex lock
                    drop(window_handle);
                });
            }
        }
    });

    let mut selected_index = i32::MAX;

    let window_handle_weak = window.as_weak();
    window
        .global::<Logic>()
        .on_album_clicked(move |index, album: Album| {
            let next_selected_index = if index == selected_index {
                i32::MAX
            } else {
                index
            };

            selected_index = next_selected_index;

            window_handle_weak
                .upgrade_in_event_loop(move |window| window.set_selected_index(selected_index))
                .unwrap();

            play_album(&album.artist, &album.title);
        });

    window
        .global::<Logic>()
        .on_track_clicked(move |track: Track| {
            println!("{}", track.name);
        });

    let mut controls = MediaControls::new(PLATFORM_CONFIG).unwrap();

    controls
        .attach(|event: MediaControlEvent| {
            match event {
                MediaControlEvent::Pause | MediaControlEvent::Play | MediaControlEvent::Toggle => {
                    play_pause();
                }
                MediaControlEvent::Next => next_track(),
                MediaControlEvent::Previous => previous_track(),
                // if these notifications are ever relevant do something with them
                MediaControlEvent::Stop
                | MediaControlEvent::Seek(_)
                | MediaControlEvent::SeekBy(_, _)
                | MediaControlEvent::SetPosition(_)
                | MediaControlEvent::SetVolume(_)
                | MediaControlEvent::Raise
                | MediaControlEvent::Quit
                | MediaControlEvent::OpenUri(_) => {}
            }
        })
        .unwrap();

    thread::spawn(move || {
        let mut socket = create_socket();

        loop {
            let message = socket.read_message().expect("Error reading message");
            let notification = notification_to_json(message.into_text().unwrap());

            // fire event
            match notification.notification_type {
                NotificationTypes::PlayStateChanged => {
                    handle_play_state_change(&mut controls, notification);
                }
                NotificationTypes::PluginStartup
                | NotificationTypes::TrackChanged
                | NotificationTypes::PlayingTracksChanged
                | NotificationTypes::NowPlayingListChanged => {
                    let (_exists, path) =
                        get_cover(&notification.artist, &notification.album).unwrap();

                    let mut cover_path: String = "file://".to_owned();
                    let canonical_path = path.canonicalize().unwrap();
                    cover_path.push_str(canonical_path.to_str().unwrap());

                    controls
                        .set_metadata(MediaMetadata {
                            title: Some(&notification.track),
                            artist: Some(&notification.artist),
                            album: Some(&notification.album),
                            duration: Some(Duration::from_millis(notification.duration)),
                            cover_url: Some(&cover_path),
                        })
                        .unwrap();

                    handle_play_state_change(&mut controls, notification);
                }
                // if these notifications are ever relevant do something with them
                NotificationTypes::PlayCountersChanged
                | NotificationTypes::NowPlayingListEnded
                | NotificationTypes::VolumeLevelChanged => {}
            }
        }
    });

    window.run()
}
