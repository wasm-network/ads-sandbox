/// This file provides the Stage and Scene animation for the Teapot ad
///
///

use tweek::{
    core::*,
    events::*,
    gui::*,
    tools::*,
};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
};

use stretch::{
    geometry::*,
    node::{Node, Stretch},
    result::Layout,
    style::*
};

const INTRO_FONT_SIZE: f32 = 80.0;
const TITLE_FONT_SIZE: f32 = 68.0;
const SUBTITLE_FONT_SIZE: f32 = 55.0;

pub struct TeapotAd {}

impl TeapotAd {
    pub fn build_stage(&mut self, stage: &mut Stage, frame: &Rectangle) {
        stage.title = "Teapot Ad".to_string();

        let background = self.background_scene(&frame);
        stage.add_scene(background);

    }

    fn background_scene(&self, frame: &Rectangle) -> Scene {
        let mut scene = Scene::new(frame.clone()).with_id(201, "Ad background");
        scene.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);

        let rect = Rectangle::new(frame.pos, (frame.width(), 160.0));
        let fill_color = Color::from_hex("#003300");
        let mut shape = DrawShape::rectangle(&rect, Some(fill_color), None, 0.0, 0.0);
        let header_bg = ShapeView::new(rect, ShapeDef::Rectangle).with_mesh(&mut shape);
        scene.add_view(Box::new(header_bg));

        scene
    }

    /// Create a Scene with 3 text elements that will animate from the right to the middle, wait,
    /// and then animate out to the left. Each sequentially
    fn intro_scene(&self, frame: &Rectangle) -> Scene {
        let mut scene = Scene::new(frame.clone());

        // let rect = Rectangle::new(())
        // let heading1 = Text::new()



        scene
    }
}