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

const INTRO_TEXT_1: &'static str = "Introducing";
const INTRO_TEXT_2: &'static str = "A Very Basic";
const INTRO_TEXT_3: &'static str = "Animated Ad";

const INTRO_1_ID: u32 = 221;
const INTRO_2_ID: u32 = 222;
const INTRO_3_ID: u32 = 223;

pub struct TeapotAdBuilder {}

impl TeapotAdBuilder {
    pub fn build_stage(&mut self, stage: &mut Stage, frame: &Rectangle, theme: &mut Theme, spec: &AdSpec) {
        stage.title = "Teapot Ad".to_string();
        log::debug!("build_stage frame={:?}", frame);
        log::debug!("build_stage spec={:?}", spec);
        let background = self.background_scene(frame, spec);
        stage.add_scene(background);

        // let body_frame = Rectangle::new
        let intro = self.intro_scene(frame, spec, theme);
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
    fn intro_scene(&self, frame: &Rectangle, spec: &AdSpec, theme: &mut Theme) -> Scene {
        let mut scene = Scene::new(frame.clone());

        let mut timeline = Timeline::new(frame.clone());

        let font_size = INTRO_FONT_SIZE * spec.scale_y;

        let label = self.make_intro_text(INTRO_TEXT_1, font_size, frame, theme, INTRO_1_ID);
        timeline.add_sprite(Box::new(label), 0.0);

        let label = self.make_intro_text(INTRO_TEXT_2, font_size, frame, theme, INTRO_2_ID);
        timeline.add_sprite(Box::new(label), 2.0);

        let label = self.make_intro_text(INTRO_TEXT_3, font_size, frame, theme, INTRO_3_ID);
        timeline.add_sprite(Box::new(label), 4.0);
        &timeline.play();
        scene.set_timeline(timeline);

        scene
    }

    fn make_intro_text(&self, text: &str, font_size: f32, frame: &Rectangle, theme: &mut Theme, id: u32) -> Label {
        let text_size = theme.default_font.measure_text(text, font_size);
        let origin = frame.center_origin(text_size);
        log::debug!("text_size={:?} origin={:?}", text_size, origin);
        let xpos = frame.x() + frame.width() + 10.0;
        let ypos = origin.1;
        let subframe = Rectangle::new((xpos, ypos), (text_size.0, font_size));

        // Intro #1
        let mut label = Label::new(subframe.clone());
        label.set_id(id);
        label.set_text(text);
        label.display = LabelDisplay::Text;
        label.layer.font_style = FontStyle::new(font_size, Color::BLACK);
        label.layer.lock_style = true;
        let final_x = -text_size.0 - 10.0;
        let mut tween = Tween::with(id, &label.layer)
            .to(&[position(origin.0, ypos)])
            .duration(0.5)
            .ease(Ease::SineIn)
            .to(&[])
            .duration(1.0)
            .to(&[position(final_x, ypos)])
            .duration(0.5)
            .ease(Ease::SineOut)
            ;
        // tween.debug = true;
        label.layer.set_animation(tween);
        label
    }
}