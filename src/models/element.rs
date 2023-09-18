// Elements represent all card-like components in the game. be it characters, supports or units.

use image::RgbImage;

pub trait Element {
    fn get_name(&self) -> String;
    fn get_points(&self) -> u32;
    fn get_icon(&self) -> Option<RgbImage> {None}
    fn generate_card(&self) -> Option<RgbImage> {None}
}