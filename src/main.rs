use std::rc::Rc;

use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // let ui_handle = ui.as_weak();

    let albums_model: Rc<VecModel<slint::Color>> = Rc::new(VecModel::from(vec![
        slint::Color::from_rgb_u8(81, 255, 234),
        slint::Color::from_rgb_u8(209, 0, 24),
        slint::Color::from_rgb_u8(229, 29, 196),
        slint::Color::from_rgb_u8(68, 196, 183),
        slint::Color::from_rgb_u8(176, 75, 234),
        slint::Color::from_rgb_u8(163, 38, 29),
        slint::Color::from_rgb_u8(92, 189, 249),
        slint::Color::from_rgb_u8(221, 77, 64),
        slint::Color::from_rgb_u8(9, 31, 109),
        slint::Color::from_rgb_u8(204, 4, 117),
        slint::Color::from_rgb_u8(46, 186, 86),
        slint::Color::from_rgb_u8(6, 12, 132),
        slint::Color::from_rgb_u8(226, 223, 40),
        slint::Color::from_rgb_u8(83, 244, 78),
        slint::Color::from_rgb_u8(102, 153, 0),
        slint::Color::from_rgb_u8(224, 113, 49),
        slint::Color::from_rgb_u8(40, 95, 183),
        slint::Color::from_rgb_u8(211, 169, 19),
        slint::Color::from_rgb_u8(68, 124, 255),
        slint::Color::from_rgb_u8(78, 219, 172),
    ]));

    let albums = ModelRc::from(albums_model.clone());
    ui.set_albums(albums);

    ui.run()
}
