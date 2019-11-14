pub use self::ad_viewer::*;
pub use self::teapot::*;

mod ad_viewer;
mod teapot;

use std::collections::BTreeMap;

// Square – 250 x 250.
// Small Square – 200 x 200.
// Banner – 468 x 60.
// Leaderboard – 728 x 90.
// Inline Rectangle – 300 x 250.
// Large Rectangle – 336 x 280.
// Skyscraper – 120 x 600.
// Wide Skyscraper – 160 x 600.

pub const DEFAULT_SIZE: (u32, u32) = (500, 500);

#[derive(Clone, Debug, PartialEq)]
pub struct AdSpec {
    id: String,
    width: f32,
    height: f32,
    name: String,
    scale_x: f32,
    scale_y: f32,
}

impl AdSpec {
    pub fn new(w: u32, h: u32, name: &str) -> Self {
        let id = format!("{}x{}", w, h);
        let scale_x = w as f32 / DEFAULT_SIZE.0 as f32;
        let scale_y = h as f32 / DEFAULT_SIZE.1 as f32;
        AdSpec {
            id,
            width: w as f32,
            height: h as f32,
            name: name.to_string(),
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

impl Default for AdSpec {
    fn default() -> Self {
        AdSpec::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1, "Default 500x500")
    }
}

lazy_static! {
    #[allow(missing_docs)]
    pub static ref AD_SIZES_MAP: BTreeMap<&'static str, AdSpec> = {
        let mut map = BTreeMap::new();
        map.insert("300x250", AdSpec::new(300, 250, "Inline Rectangle"));
        map.insert("250x250", AdSpec::new(300, 250, "Square"));
        map.insert("728x90", AdSpec::new(728, 90, "Leaderboard"));
        map.insert("120x600", AdSpec::new(120, 600, "Skyscraper"));
        map
    };
}
