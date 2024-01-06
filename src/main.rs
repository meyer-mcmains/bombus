use slint::{Image, ModelRc, SharedString, VecModel};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig};
use std::{path::Path, rc::Rc, time::Duration};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let albums_model: Rc<VecModel<Album>> = Rc::new(VecModel::from(vec![
        Album {
            artist: SharedString::from("A Crowd of Small Adventures"),
            title: SharedString::from("A Decade in X-Rays"),
            image: Image::load_from_path(Path::new(
                "./artwork/A Crowd of Small Adventures/A Decade in X-Rays.jpg",
            ))
            .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("A Crowd of Small Adventures"),
            title: SharedString::from("Blood"),
            image: Image::load_from_path(Path::new(
                "./artwork/A Crowd of Small Adventures/Blood.jpg",
            ))
            .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("A Crowd of Small Adventures"),
            title: SharedString::from("The Evil Archipelago"),
            image: Image::load_from_path(Path::new(
                "./artwork/A Crowd of Small Adventures/The Evil Archipelago.jpg",
            ))
            .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("A Crowd of Small Adventures"),
            title: SharedString::from("Ruby Rose"),
            image: Image::load_from_path(Path::new(
                "./artwork/A Crowd of Small Adventures/Ruby Rose.jpg",
            ))
            .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("The Black Keys"),
            title: SharedString::from("Attack & Release"),
            image: Image::load_from_path(Path::new(
                "./artwork/The Black Keys/Attack & Release.jpg",
            ))
            .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("The Black Keys"),
            title: SharedString::from("Brother"),
            image: Image::load_from_path(Path::new("./artwork/The Black Keys/Brothers.jpg"))
                .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("The Black Keys"),
            title: SharedString::from("El Camino"),
            image: Image::load_from_path(Path::new("./artwork/The Black Keys/El Camino.jpg"))
                .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("The Black Keys"),
            title: SharedString::from("Rubber Factory"),
            image: Image::load_from_path(Path::new("./artwork/The Black Keys/Rubber Factory.jpg"))
                .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("The Black Keys"),
            title: SharedString::from("Thickfreakness"),
            image: Image::load_from_path(Path::new("./artwork/The Black Keys/Thickfreakness.jpg"))
                .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("Wyldest"),
            title: SharedString::from("Dream Chaos"),
            image: Image::load_from_path(Path::new("./artwork/Wyldest/Dream Chaos.jpg")).unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
        Album {
            artist: SharedString::from("Wyldest"),
            title: SharedString::from("Feed the Flowers Nightmares"),
            image: Image::load_from_path(Path::new(
                "./artwork/Wyldest/Feed the Flowers Nightmares.jpg",
            ))
            .unwrap(),
            tracks: ModelRc::new(VecModel::from(vec![
                Track {
                    number: 1,
                    name: SharedString::from("Beggar"),
                    duration: SharedString::from("04:49"),
                },
                Track {
                    number: 2,
                    name: SharedString::from("Hollow"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 3,
                    name: SharedString::from("Buddy"),
                    duration: SharedString::from("04:06"),
                },
                Track {
                    number: 4,
                    name: SharedString::from("Monthly Friend"),
                    duration: SharedString::from("03:31"),
                },
                Track {
                    number: 5,
                    name: SharedString::from("Heal"),
                    duration: SharedString::from("03:54"),
                },
                Track {
                    number: 6,
                    name: SharedString::from("Almost Bliss"),
                    duration: SharedString::from("03:08"),
                },
                Track {
                    number: 7,
                    name: SharedString::from("Glue"),
                    duration: SharedString::from("03:04"),
                },
                Track {
                    number: 8,
                    name: SharedString::from("Arrows"),
                    duration: SharedString::from("03:24"),
                },
                Track {
                    number: 9,
                    name: SharedString::from("Burn"),
                    duration: SharedString::from("03:23"),
                },
                Track {
                    number: 10,
                    name: SharedString::from("The Void"),
                    duration: SharedString::from("04:48"),
                },
            ])),
        },
    ]));

    let config = PlatformConfig {
        dbus_name: "bombus",
        display_name: "Bombus",
        hwnd: None,
    };

    let mut controls = MediaControls::new(config).unwrap();

    // TODO respond to media control events
    controls
        .attach(|event: MediaControlEvent| println!("Event received: {:?}", event))
        .unwrap();

    let albums = ModelRc::from(albums_model.clone());
    ui.set_albums(albums);

    let mut selected_index = i32::MAX;

    ui.global::<Logic>()
        .on_album_clicked(move |index, album: Album| {
            let next_selected_index = if index == selected_index {
                i32::MAX
            } else {
                index
            };

            selected_index = next_selected_index;

            ui_handle
                .upgrade()
                .unwrap()
                .set_selected_index(next_selected_index);

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

    ui.global::<Logic>().on_track_clicked(move |track: Track| {
        println!("{}", track.name);
    });

    ui.run()
}
