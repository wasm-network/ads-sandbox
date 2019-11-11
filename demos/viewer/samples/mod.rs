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

#[derive(Clone, Debug, PartialEq)]
pub struct AdSpec {
    width: f32,
    height: f32,
    name: String,
}

impl AdSpec {
    pub fn new(w: f32, h: f32, name: &str) -> Self {
        AdSpec {
            width: w,
            height: h,
            name: name.to_string(),
        }
    }
}

impl Default for AdSpec {
    fn default() -> Self {
        AdSpec {
            width: 500.0,
            height: 500.0,
            name: "Default 500x500".to_string(),
        }
    }
}

lazy_static! {
    #[allow(missing_docs)]
    pub static ref AD_SIZES_MAP: BTreeMap<&'static str, AdSpec> = {
        let mut map = BTreeMap::new();
        map.insert("300x250", AdSpec::new(300.0, 250.0, "Inline Rectangle"));
        map.insert("250x250", AdSpec::new(300.0, 250.0, "Square"));
        map.insert("728x90", AdSpec::new(728.0, 90.0, "Leaderboard"));
        map.insert("120x600", AdSpec::new(120.0, 600.0, "Skyscraper"));
        map
    };
}
