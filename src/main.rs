use std::{
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use bombus_data::{next_track, persist, play_album, play_pause, previous_track};
use slint::{Color, ComponentHandle, Model, VecModel};
use souvlaki::{MediaControlEvent, MediaControls, PlatformConfig};

use utils::slint_modules::{Album, AppWindow, Library, Logic, Theme, Track};
use utils::theme;

mod library;
mod notifications;
mod utils;

const PLATFORM_CONFIG: souvlaki::PlatformConfig<'_> = PlatformConfig {
    dbus_name: "bombus",
    display_name: "Bombus",
    hwnd: None,
};

fn main() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;

    let library_selected = Arc::new(AtomicBool::new(false));

    let color = theme::get();
    window.global::<Theme>().set_color(color);

    // load libraries
    let libraries = persist::get_libraries();
    let libraries_model: Rc<VecModel<Library>> = Rc::new(VecModel::from(vec![]));

    libraries.into_iter().for_each(|library| {
        let library = Library {
            name: library.name.into(),
            ip: library.ip.to_string().into(),
            color: Color::from_argb_encoded(library.color),
        };
        libraries_model.push(library);
    });

    window.set_libraries(libraries_model.into());

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
    let library_selected_clone = library_selected.clone();
    let notifications_tread = thread::spawn(move || {
        notifications::listen(library_selected_clone, window_handle_weak, &mut controls)
    });

    // on album clicked
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
    let window_handle_weak = window.as_weak();
    window
        .global::<Logic>()
        .on_add_library(move |name, ip, color| {
            persist::add_library(name.to_string(), ip.as_str(), color.as_argb_encoded());
            let binding = window_handle_weak.upgrade().unwrap().get_libraries();
            let ui_libraries = binding
                .as_any()
                .downcast_ref::<VecModel<Library>>()
                .unwrap();
            ui_libraries.push(Library { name, ip, color })
        });

    // on library selected
    let window_handle_weak = window.as_weak();
    let library_selected_clone = library_selected.clone();
    window.global::<Logic>().on_load_library(move |library| {
        // load the library
        bombus_data::set_connection(library.ip.to_string());
        library::load_library(window_handle_weak.upgrade().unwrap());

        // unpark the socket thread
        library_selected_clone.store(true, Ordering::Release);
        notifications_tread.thread().unpark();

        // show the frame after selecting a library
        let upgraded_window = window_handle_weak.upgrade().unwrap();
        upgraded_window.set_has_frame(true);
    });

    // on library removed
    let window_handle_weak = window.as_weak();
    window.global::<Logic>().on_remove_library(move |library| {
        let remove_index = persist::remove_library(library.name.to_string());

        let binding = window_handle_weak.upgrade().unwrap().get_libraries();
        let ui_libraries = binding
            .as_any()
            .downcast_ref::<VecModel<Library>>()
            .unwrap();
        ui_libraries.remove(remove_index);
    });

    window.run()
}
