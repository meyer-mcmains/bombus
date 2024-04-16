use std::sync::{Arc, Mutex};
use std::{rc::Rc, thread};

use bombus_data::{get_cover, get_library};
use slint::{ComponentHandle, Model, ModelExt, ModelRc, VecModel};
use utils::album_cover;
use utils::slint_modules::{Album, AppWindow, Track};

use crate::utils;

/// load the library from the server
/// - then transform the struct into a UI struct
/// - pull in images from disk or if missing download from server
///
/// ## TODO
/// - see if cloning of library can be removed
/// - serialize into UI struct directly (current not possible because Image cannot be serialized)
/// - more performance?
pub fn load_library(window: AppWindow) {
    // load the library
    let mut library = get_library().unwrap();

    // make a clone so we can operate on the library in 2 separate threads
    // TODO improve this
    let library_copy = library.clone();

    let window_handle_weak = window.as_weak();
    thread::spawn(move || {
        window_handle_weak
            .upgrade_in_event_loop(move |handle| {
                // sort artists by name ignoring `the `
                library.sort_by(|lhs, rhs| {
                    let lhs_artist_no_the = lhs.artist.to_lowercase().replace("the ", "");
                    let rhs_artist_no_the = rhs.artist.to_lowercase().replace("the ", "");
                    lhs_artist_no_the.cmp(&rhs_artist_no_the)
                });

                let albums_model: Rc<VecModel<Album>> = Rc::new(VecModel::from(vec![]));

                // transform the library into structs that can be consumed by the ui
                // TODO serialize into the UI structs directly
                // NOTE image does not currently implement serialize
                library.into_iter().for_each(|mut artist| {
                    // sort albums by year -> name
                    artist.albums.sort_by(|lhs, rhs| {
                        if lhs.year == rhs.year {
                            let _ = lhs.title.cmp(&rhs.title);
                        }

                        lhs.year.cmp(&rhs.year)
                    });

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
                            year: album.year.into(),
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
}
