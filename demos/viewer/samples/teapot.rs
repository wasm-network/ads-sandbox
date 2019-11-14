/// This file provides the Stage and Scene animation for the Teapot ad
///
///
use super::*;

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

const BG_HEADER_H: f32 = 160.0;

const INTRO_TEXT_1: &'static str = "Heading 1";

const INTRO_1_ID: u32 = 221;
const INTRO_2_ID: u32 = 222;
const INTRO_3_ID: u32 = 223;

pub struct TeapotAdBuilder {}

impl TeapotAdBuilder {
    pub fn build_stage(&mut self, stage: &mut Stage, frame: &Rectangle, theme: &mut Theme, spec: &AdSpec) {
        stage.title = "Teapot Ad".to_string();

        let background = self.background_scene(frame, spec);
        stage.add_scene(background);

        let intro = self.intro_scene(frame, spec);
        stage.add_scene(intro);


    }

    fn background_scene(&self, frame: &Rectangle, spec: &AdSpec) -> Scene {
        let mut scene = Scene::new(frame.clone()).with_id(201, "Ad scene");
        scene.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);
        scene.layer.bg_style = BackgroundStyle::Solid(Color::WHITE);
        let h = BG_HEADER_H * spec.scale_y;
        let rect = Rectangle::new(frame.pos, (frame.width(), h));
        let fill_color = Color::from_hex("#003300");
        let mut shape = DrawShape::rectangle(&rect, Some(fill_color), None, 0.0, 0.0);
        let header_bg = ShapeView::new(rect, ShapeDef::Rectangle).with_mesh(&mut shape);
        scene.add_view(Box::new(header_bg));

        scene
    }

    /// Create a Scene with 3 text elements that will animate from the right to the middle, wait,
    /// and then animate out to the left. Each sequentially
    fn intro_scene(&self, frame: &Rectangle, spec: &AdSpec) -> Scene {
        let mut scene = Scene::new(frame.clone());

        let mut timeline = Timeline::new(frame.clone());

        let font_size = INTRO_FONT_SIZE * spec.scale_y;
        let subframe = Rectangle::new_sized((frame.width(), font_size));
        let mut text = Text::new(subframe, INTRO_TEXT_1);
        text.set_id(INTRO_1_ID);
        text.layer.font_style = FontStyle::new(font_size, Color::BLACK);
        text.text_align(TextAlign::Center);
        text.layer.lock_style = true; // TODO: add style() function to Text

        let xpos = frame.width() + 10.0;
        let ypos = (frame.height() - subframe.height()) / 2.0;

        text.get_layer_mut().frame.pos.x = xpos;
        text.get_layer_mut().frame.pos.y = ypos;

        let text_size = text.get_content_size();
        let target_x = (frame.width() - text_size.x) / 2.0;

        let mut tween = Tween::with(INTRO_1_ID, &text.layer)
            .to(&[position(0.0, ypos)])
            .duration(1.0)
            ;
        tween.debug = true;
        tween.state = PlayState::Pending;
        &tween.play();
        text.layer.set_animation(tween);
        // text.layer.tween_type = TweenType::Move;
        // timeline.add_sprite(Box::new(text), 0.0);
        // timeline.stagger(0.125);
        // &timeline.play();
        // scene.set_timeline(timeline);
        scene.add_control(Box::new(text));
        scene
    }
}