use std::path::Path;

use bombus_data::persist;
use slint::{Image, SharedPixelBuffer};

const PLACEHOLDER_IMAGE: &[u8; 32346] = include_bytes!("../assets/cover.jpg");

/// load the album cover based the artist and album title
/// setting the fallback if the cover does not exist
pub fn load(artist: &str, album: &str) -> Image {
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

    let artwork_cache = persist::get_artwork_cache_directory();

    Image::load_from_path(
        &artwork_cache
            .join(safe_artist)
            .join(safe_album)
            .with_extension("jpg"),
    )
    .unwrap_or(fallback_cover)
}

/// load the album cover from an already verified path
pub fn load_from_path(path: &Path) -> Image {
    Image::load_from_path(path).unwrap()
}
