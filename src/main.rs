use bombus_data::{get_cover, get_library};
use slint::{Image, Model, ModelRc, VecModel};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig};
use std::{
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

slint::include_modules!();

const PLATFORM_CONFIG: souvlaki::PlatformConfig<'_> = PlatformConfig {
    dbus_name: "bombus",
    display_name: "Bombus",
    hwnd: None,
};

/**
 * load the album cover based the artist and album title
 * setting the fallback if the cover does not exist
 */
fn load_cover_from_path(artist: &str, album: &str) -> Image {
    let safe_artist = artist.replace('/', "_");
    let safe_album = album.replace('/', "_");
    let fallback_cover = Image::load_from_path(Path::new("./src/assets/cover.jpg")).unwrap();

    Image::load_from_path(
        &Path::new("./artwork")
            .join(safe_artist)
            .join(safe_album)
            .with_extension("jpg"),
    )
    .unwrap_or(fallback_cover)
}

fn main() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;

    // load the library
    let library = || get_library().unwrap();

    let window_handle_weak = window.as_weak();
    thread::spawn(move || {
        window_handle_weak
            .upgrade_in_event_loop(move |handle| {
                let albums_model: Rc<VecModel<Album>> = Rc::new(VecModel::from(vec![]));

                // transform the library into structs that can be consumed by the ui
                // TODO serialize into the UI structs directly
                library().into_iter().for_each(|artist| {
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

                        let image = load_cover_from_path(&album.artist, &album.title);

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
        for artist in library().into_iter() {
            for album in artist.albums {
                let window_handle_clone = Arc::clone(&window_handle_weak_mutex);

                thread::spawn(move || {
                    let window_handle = window_handle_clone.lock().unwrap();

                    // TODO skip event loop is cover already exists
                    get_cover(&album.artist, &album.title).unwrap();

                    window_handle
                        .upgrade_in_event_loop(move |handle| {
                            // https://github.com/slint-ui/slint/discussions/2329#discussioncomment-5213994
                            let binding = handle.get_albums();
                            let ui_albums = binding
                                .as_any()
                                .downcast_ref::<slint::VecModel<Album>>()
                                .unwrap();

                            let ui_album_index = ui_albums.iter().position(|ui_album| {
                                // TODO use ids instead
                                ui_album.artist == album.artist && ui_album.title == album.title
                            });

                            if let Some(row) = ui_album_index {
                                ui_albums.row_data_tracked(row);
                                let mut data = ui_albums.row_data(row).unwrap();
                                data.image = load_cover_from_path(&data.artist, &data.title);
                                ui_albums.set_row_data(row, data);
                            }
                        })
                        .unwrap();

                    // drop mutex lock
                    drop(window_handle);
                });
            }
        }
    });

    let mut controls = MediaControls::new(PLATFORM_CONFIG).unwrap();

    // TODO respond to media control events
    controls
        .attach(|event: MediaControlEvent| println!("Event received: {:?}", event))
        .unwrap();

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

            // TODO move this placeholder logic to socket notifications
            let mut scheme = "file://".to_owned();
            let binding = album.image.path().unwrap().canonicalize().unwrap();
            let cover = binding.to_str().unwrap();
            scheme.push_str(cover);

            // Update the media metadata.
            controls
                .set_metadata(MediaMetadata {
                    title: Some(&album.tracks.row_data_tracked(0).unwrap().name),
                    artist: Some(&album.artist),
                    album: Some(&album.title),
                    duration: Some(Duration::from_secs(120)),
                    cover_url: Some(scheme.as_str()),
                })
                .unwrap();

            controls
                .set_playback(souvlaki::MediaPlayback::Playing {
                    progress: Some(souvlaki::MediaPosition(Duration::from_secs(0))),
                })
                .unwrap();
        });

    window
        .global::<Logic>()
        .on_track_clicked(move |track: Track| {
            println!("{}", track.name);
        });

    window.run()
}
