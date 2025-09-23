use crate::utils::slint_modules::{
    AppWindow, Logic, NowPlayingState, PlayState as SlintPlayState, Track,
};
use bombus_data::*;
use slint::{ComponentHandle, Model, SharedString, Weak};
use souvlaki::{MediaControls, MediaMetadata};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

fn handle_play_state_change<F>(
    controls: &mut MediaControls,
    notification: &Notification,
    clear_now_playing: F,
) where
    F: FnOnce(),
{
    match notification.play_state {
        PlayState::Paused => {
            controls
                .set_playback(souvlaki::MediaPlayback::Paused {
                    progress: Some(souvlaki::MediaPosition(Duration::from_millis(
                        notification.position as u64,
                    ))),
                })
                .unwrap();
        }
        PlayState::Playing => {
            controls
                .set_playback(souvlaki::MediaPlayback::Playing {
                    progress: Some(souvlaki::MediaPosition(Duration::from_millis(
                        notification.position as u64,
                    ))),
                })
                .unwrap();
        }
        PlayState::Stopped => {
            clear_now_playing();

            controls
                .set_playback(souvlaki::MediaPlayback::Stopped)
                .unwrap();
        }
    }
}

/// using a socket listen for notification from musicbee
pub fn listen(
    library_selected: Arc<AtomicBool>,
    window_handle_weak: Weak<AppWindow>,
    controls: &mut MediaControls,
) {
    let mut socket: Option<TWebSocket> = None;

    // wait for a library to be selected
    while !library_selected.load(Ordering::Acquire) {
        // park the thread until a library is selected
        thread::park();
        let maybe_socket = create_socket();

        if let Some(value) = maybe_socket {
            socket = Some(value);
            break;
        }
    }

    while let Some(ref mut socket) = socket {
        let message = socket.read_message().expect("Error reading message");
        let notification = notification_to_json(message.into_text().unwrap());

        // fire event
        match notification.notification_type {
            NotificationTypes::PlayStateChanged => {
                let clear_now_playing = || {
                    window_handle_weak
                        .upgrade_in_event_loop(move |window| {
                            window
                                .global::<Logic>()
                                .set_now_playing_album(SharedString::from(""));

                            window
                                .global::<Logic>()
                                .set_now_playing_track(Track::default())
                        })
                        .unwrap()
                };

                handle_play_state_change(controls, &notification, clear_now_playing);
            }
            NotificationTypes::PluginStartup
            | NotificationTypes::TrackChanged
            | NotificationTypes::PlayingTracksChanged
            | NotificationTypes::NowPlayingListChanged => {
                let (_exists, path) =
                    get_cover(&notification.track.artist, &notification.track.album).unwrap();

                let mut cover_path: String = "file://".to_owned();
                let canonical_path = path.canonicalize().unwrap();
                cover_path.push_str(canonical_path.to_str().unwrap());

                controls
                    .set_metadata(MediaMetadata {
                        title: Some(&notification.track.title),
                        artist: Some(&notification.track.artist),
                        album: Some(&notification.track.album),
                        duration: Some(Duration::from_millis(notification.track.duration as u64)),
                        cover_url: Some(&cover_path),
                    })
                    .unwrap();

                // on darwin if the first play state from MusicBee is Paused,
                // than than the now playing controls will not update
                // it's possible that darwin requires a Playing state before a Paused one
                // in either cause just dispatch a Playing playback state
                // before calling handle_play_state_change normally
                match notification.play_state {
                    PlayState::Paused => {
                        controls
                            .set_playback(souvlaki::MediaPlayback::Playing {
                                progress: Some(souvlaki::MediaPosition(Duration::from_millis(
                                    notification.position as u64,
                                ))),
                            })
                            .unwrap();
                    }
                    _ => (),
                }

                handle_play_state_change(controls, &notification, || { /* do nothing */ });

                if !notification.track.uri.is_empty() {
                    window_handle_weak
                        .upgrade_in_event_loop(move |window| {
                            let binding = window.get_albums();
                            let now_playing_album = binding
                                .iter()
                                .find(|album| {
                                    album
                                        .tracks
                                        .iter()
                                        .any(|track| track.uri == notification.track.uri)
                                })
                                .unwrap();

                            window
                                .global::<Logic>()
                                .set_now_playing_album(now_playing_album.id);

                            window
                                .global::<Logic>()
                                .set_now_playing_state(NowPlayingState {
                                    position: notification.position.into(),
                                    playState: match notification.play_state {
                                        PlayState::Paused => SlintPlayState::Paused,
                                        PlayState::Playing => SlintPlayState::Playing,
                                        PlayState::Stopped => SlintPlayState::Stopped,
                                    },
                                });

                            window.global::<Logic>().set_now_playing_track(Track {
                                duration: notification.track.duration.into(),
                                length: notification.track.length.into(),
                                title: notification.track.title.into(),
                                number: notification.track.number,
                                uri: notification.track.uri.into(),
                                disk: notification.track.disk.unwrap_or_default(),
                            })
                        })
                        .unwrap();
                }
            }
            // if these notifications are ever relevant do something with them
            NotificationTypes::PlayCountersChanged
            | NotificationTypes::NowPlayingListEnded
            | NotificationTypes::VolumeLevelChanged => {}
        }
    }
}
