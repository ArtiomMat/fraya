use core::error;

use crate::video::Image;

/// A surface represents any entity that can show an image but needs an
/// explicit request to do so.
/// 
/// E.g. a PNG exporter, an OS window.
pub trait Surface {
    /// Returns `false` upon failure.
    fn update_image(&mut self, img: &Image) -> Result<(), Box<dyn error::Error>>;
}
