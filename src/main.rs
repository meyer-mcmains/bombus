use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use bombus_data::{
    get_cover, get_library, next_track, persist, play_album, play_pause, previous_track,
};
use slint::{ComponentHandle, Model, ModelExt, ModelRc, VecModel};
use souvlaki::{MediaControlEvent, MediaControls, PlatformConfig};

use utils::album_cover;
use utils::slint_modules::{Album, AppWindow, Logic, Theme, Track};
use utils::theme;

mod notifications;
mod utils;

const PLATFORM_CONFIG: souvlaki::PlatformConfig<'_> = PlatformConfig {
    dbus_name: "bombus",
    display_name: "Bombus",
    hwnd: None,
};

fn main() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;

    let color = theme::get();
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
                // NOTE image does not currently implement serialize
                library.into_iter().for_each(|artist| {
                    artist.albums.into_iter().for_each(|album| {
                        let tracks: Vec<Track> = album
                            .tracks
                            .into_iter()
                            .map(|track| Track {
                                length: track.length.into(),
                                title: track.title.into(),
                                number: track.number,
                                uri: track.uri.into(),
                            })
                            .collect();

                        let image = album_cover::load(&album.artist, &album.title);

                        let album: Album = Album {
                            id: album.id.into(),
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
                                    .position(|ui_album| ui_album.id == album.id);

                                if let Some(row) = ui_album_index {
                                    ui_albums.row_data_tracked(row);
                                    let mut data = ui_albums.row_data(row).unwrap();
                                    data.image = album_cover::load_from_path(&path);
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

    // on track clicked
    window
        .global::<Logic>()
        .on_track_clicked(move |track: Track| {
            println!("{}", track.title);
        });

    // on library added
    window
        .global::<Logic>()
        .on_add_library(move |name, ip, color| {
            persist::add_library(name.to_string(), ip.as_str(), color.to_string());
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

    // listen for notifications from musicbee
    let window_handle_weak = window.as_weak();
    thread::spawn(move || notifications::listen(window_handle_weak, &mut controls));

    window.run()
}
