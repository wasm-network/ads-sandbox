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


lazy_static! {
    #[allow(missing_docs)]
    pub static ref AD_SIZES_MAP: BTreeMap<&'static str, (f32, f32, &'static str)> = {
        let mut map = BTreeMap::new();
        map.insert("300x250", (300.0, 250.0, "Inline Rectangle"));
        map.insert("250x250", (300.0, 250.0, "Square"));
        map.insert("728x90", (728.0, 90.0, "Leaderboard"));
        map.insert("120x600", (120.0, 600.0, "Skyscraper"));
        map
    };
}
