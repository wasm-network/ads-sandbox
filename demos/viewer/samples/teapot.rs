/// This file provides the Stage and Scene animation for the Teapot ad
///
///

use tweek::{
    core::*,
    events::*,
    gui::*,
};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
};

pub struct TeapotAd {
}

impl TeapotAd {
    pub fn build_stage(&mut self, frame: Rectangle) -> Stage {
        let stage = Stage::new(frame);
        stage
    }
}