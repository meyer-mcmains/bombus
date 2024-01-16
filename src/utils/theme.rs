use cacao::core_foundation::base::TCFType;
use regex::Regex;

use crate::utils::slint_modules::Color;
trait ToRgbaColor {
    fn to_rgba_color(&self) -> slint::Color;
}

impl ToRgbaColor for cacao::color::Color {
    /// HACK get system colors from cg_color
    /// - use format to get cg_color as a string value
    /// - use regex to get the rgba value
    fn to_rgba_color(&self) -> slint::Color {
        let cg_color_string = format!("{:?}", self.cg_color().as_CFType());

        let rgba_regex = Regex::new(r"\d\.?\d* \d\.?\d* \d\.?\d* \d\.?\d*").unwrap();
        let rgba: Vec<&str> = rgba_regex
            .find(&cg_color_string)
            .unwrap()
            .as_str()
            .split(' ')
            .collect();

        slint::Color::from_argb_f32(
            rgba[3].parse::<f32>().unwrap(),
            rgba[0].parse::<f32>().unwrap(),
            rgba[1].parse::<f32>().unwrap(),
            rgba[2].parse::<f32>().unwrap(),
        )
    }
}

/// get the system colors from macos
pub fn get_theme() -> Color {
    Color {
        // these are crashing
        // system_background_tertiary: cacao::color::Color::SystemBackgroundTertiary.to_rgba_color(),
        // system_background_secondary: cacao::color::Color::SystemBackgroundSecondary.to_rgba_color(),
        // system_background: cacao::color::Color::SystemBackground.to_rgba_color(),
        system_fill_tertiary: cacao::color::Color::SystemFillTertiary.to_rgba_color(),
        system_fill_secondary: cacao::color::Color::SystemFillSecondary.to_rgba_color(),
        system_fill_quaternary: cacao::color::Color::SystemFillQuaternary.to_rgba_color(),
        system_fill: cacao::color::Color::SystemFill.to_rgba_color(),
        separator: cacao::color::Color::Separator.to_rgba_color(),
        placeholder_text: cacao::color::Color::PlaceholderText.to_rgba_color(),
        under_page_background: cacao::color::Color::MacOSUnderPageBackgroundColor.to_rgba_color(),
        window_background: cacao::color::Color::MacOSWindowBackgroundColor.to_rgba_color(),
        link: cacao::color::Color::Link.to_rgba_color(),
        label_tertiary: cacao::color::Color::LabelTertiary.to_rgba_color(),
        label_secondary: cacao::color::Color::LabelSecondary.to_rgba_color(),
        label_quaternary: cacao::color::Color::LabelQuaternary.to_rgba_color(),
        label: cacao::color::Color::Label.to_rgba_color(),
        // system black and white are in gamma color space so just hard code them
        // system_black: cacao::color::Color::SystemBlack.to_rgba_color(),
        // system_white: cacao::color::Color::SystemWhite.to_rgba_color(),
        system_white: slint::Color::from_rgb_u8(255, 255, 255),
        system_black: slint::Color::from_rgb_u8(0, 0, 0),
        system_red: cacao::color::Color::SystemRed.to_rgba_color(),
        system_orange: cacao::color::Color::SystemOrange.to_rgba_color(),
        system_yellow: cacao::color::Color::SystemYellow.to_rgba_color(),
        system_green: cacao::color::Color::SystemGreen.to_rgba_color(),
        system_teal: cacao::color::Color::SystemTeal.to_rgba_color(),
        system_blue: cacao::color::Color::SystemBlue.to_rgba_color(),
        system_indigo: cacao::color::Color::SystemIndigo.to_rgba_color(),
        system_purple: cacao::color::Color::SystemPurple.to_rgba_color(),
        system_pink: cacao::color::Color::SystemPink.to_rgba_color(),
        system_brown: cacao::color::Color::SystemBrown.to_rgba_color(),
        system_gray: cacao::color::Color::SystemGray.to_rgba_color(),
        // system grey 2 - 6 are not available on macos
        // system_gray_2: cacao::color::Color::SystemGray2.to_rgba_color(),
        // system_gray_3: cacao::color::Color::SystemGray3.to_rgba_color(),
        // system_gray_4: cacao::color::Color::SystemGray4.to_rgba_color(),
        // system_gray_5: cacao::color::Color::SystemGray5.to_rgba_color(),
        // system_gray_6: cacao::color::Color::SystemGray6.to_rgba_color(),
    }
}
