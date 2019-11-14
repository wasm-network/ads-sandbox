/// This is a Controller implementation that provides a UI for displaying an ad in multiple size
/// formats. A toolbar at the top allows for selection of different ad sizes. In the main viewer area,
/// the ad sample is centered and surrounded by blank space and off-screen assets.
use super::*;
use crate::app::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color},
    lifecycle::{Window},
};

use tweek::{
    core::AppState,
    events::*,
    gui::*,
    tools::*,
};

use stretch::{
    geometry::*,
    node::{Node, Stretch},
    result::Layout,
    style::*
};

const TOOLBAR_ID: u32 = 100;
const MAIN_ID: u32 = 200;

const TOOLBAR_H: f32 = 50.0;
const TOOLBAR_BTN_W: f32 = 70.0;
const TOOLBAR_BTN_H: f32 = 32.0;


#[allow(dead_code)]
pub struct AdViewer {
    frame: Rectangle,
    stage: Stage,
    ad_spec: AdSpec,
}

impl AdViewer {
    pub fn new(frame: Rectangle) -> AdViewer {
        let stage = Stage::new(frame.clone());
        let ad_spec = AdSpec::default();
        let controller = AdViewer {
            frame,
            stage,
            ad_spec,
        };
        controller
    }

    fn assemble_stage(&mut self, frame: Rectangle, theme: &mut Theme, spec: AdSpec) -> Stage {
        let mut stage = Stage::new(frame.clone());

        // 1. Set up tools and background
        let tools_bg = self.tools_scene(&frame);
        stage.add_scene(tools_bg);

        // 2. Build ad
        let body_frame = Rectangle::new((0.0, TOOLBAR_H), (frame.width(), frame.height() - TOOLBAR_H));
        let mut ad_frame = Rectangle::new_sized((self.ad_spec.width, self.ad_spec.height));

        // Center the ad_frame inside the body_frame offset by the toolbar height
        let offset = Vector::new(0.0, TOOLBAR_H);
        let ad_frame = LayoutHelper::center_frame(&body_frame, &ad_frame, offset);

        // Finish the Stage using the specified ad
        let mut builder = TeapotAdBuilder {};
        builder.build_stage(&mut stage, &ad_frame, theme, &spec);

        stage
    }

    /// Setup toolbar buttons
    fn tools_scene(&mut self, frame: &Rectangle) -> Scene {

        let mut scene = Scene::new(frame.clone()).with_id(TOOLBAR_ID, "Toolbar");

        let fill_color = Color::from_hex("#555555");
        let mut shape = DrawShape::rectangle(&frame, Some(fill_color), None, 0.0, 0.0);
        let bg = ShapeView::new(*frame, ShapeDef::Rectangle).with_mesh(&mut shape);
        scene.add_view(Box::new(bg));

        let rect = Rectangle::new(frame.pos, (frame.width(), TOOLBAR_H));
        let fill_color = Color::from_hex("#333333");
        let mut shape = DrawShape::rectangle(&rect, Some(fill_color), None, 0.0, 0.0);
        let bg = ShapeView::new(rect.clone(), ShapeDef::Rectangle).with_mesh(&mut shape);
        scene.add_view(Box::new(bg));

        const SPACING: f32 = 8.0;
        let mut xpos = SPACING;
        let ypos = (rect.height() - TOOLBAR_BTN_H) / 2.0;

        // Toolbar buttons to add/remove

        for (key, props) in AD_SIZES_MAP.iter() {
            let subframe = scene.sub_frame((xpos, ypos), (TOOLBAR_BTN_W, TOOLBAR_BTN_H));
            let mut button = Button::new(subframe).with_text(key);
            button.layer.font_style = FontStyle::new(14.0, Color::BLACK);
            scene.add_control(Box::new(button));
            // TODO: add callback
            xpos += (TOOLBAR_BTN_W + SPACING);
        }

        scene
    }

    /// Create scene for main body area.
    /// TODO: Allow loading of specific ad samples
    ///
    fn main_scene(&self, scene_frame: &Rectangle) -> Scene {
        let mut scene = Scene::new(scene_frame.clone()).with_id(2, "Main");
        scene.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#999999"), 1.0);

        let layout = self.main_scene_layout(&scene_frame);

        // First button is child path: /Body/Column0/row0
        let item = &layout.children[1].children[0].children[0];
        let frame = Rectangle::new((item.location.x, item.location.y), (item.size.width, item.size.height));
        let mut button = Button::new(frame).with_text("Normal");
        button.layer.external_id = Some(Box::new(item.id));
        scene.add_control(Box::new(button));

        scene
    }

    /// Main Scene layout :
    ///
    ///
    /// Column 1:
    /// * Text
    /// * Buttons: normal and primary
    /// * Text Field
    /// * Text Area
    /// Column 2:
    /// * ListBox
    /// * Checkbox
    /// * OptionGroup with radio buttons
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    fn main_scene_layout(&self, frame: &Rectangle) -> NodeLayout {

        const HEADER_H: f32 = 50.0;
        let body_padding = Rect {
            start: Dimension::Points(10.0),
            end: Dimension::Points(10.0),
            top: Dimension::Points(10.0),
            bottom: Dimension::Points(10.0),
            ..Default::default()
        };
        let item_padding = Rect {
            start: Dimension::Points(10.0),
            end: Dimension::Points(10.0),
            top: Dimension::Points(10.0),
            bottom: Dimension::Points(10.0),
            ..Default::default()
        };
        let mut builder = LayoutBuilder::new().with_style(Style {
                size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(frame.height()) },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
        });
        let header_node = builder.add_row(builder.root, HEADER_H, None);
        let column_w = frame.width()/2.0;
        let body_node = builder.add_row(builder.root, frame.height() - HEADER_H, None);
        let column0 = builder.add_column(body_node, frame.width()/2.0, None);
        let node = builder.add_object(column0, Size { width: column_w, height: 50.0 });
        let node = builder.add_object(column0, Size { width: column_w, height: 50.0 });
        let node = builder.add_object(column0, Size { width: column_w, height: 200.0 });
        let abs_layout = builder.absolute_layout(builder.root, (200.0, 0.0));
        eprintln!("node_layout={:#?}", abs_layout);
        abs_layout
    }
}

impl Controller for AdViewer {

    fn view_will_load(&mut self, theme: &mut Theme) {
        self.stage = self.assemble_stage(self.frame.clone(), theme, AdSpec::default());
        self.stage.notify(&DisplayEvent::Ready);
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        self.stage.set_theme(theme);
    }

    fn screen_title(&self) -> &str {
        "Theme Builder"
    }

    // fn nav_target_for_event(&mut self, event: &NavEvent, _ctx: &mut AppContext) -> Option<NavTarget> {
    //     match event {
    //         NavEvent::Next => {
    //             let controller = SettingsController::new(self.frame.clone());
    //             let target = NavTarget {
    //                 nav_event: event.clone(),
    //                 controller: Rc::new(RefCell::new(controller))
    //             };
    //             return Some(target);
    //         }
    //         _ => ()
    //     }
    //     None
    // }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        // This is just placeholder code for future consideration of what kinds of events
        // might get queued within this controller.
        // let mut nav_event: Option<NavEvent> = None;
        // if let Some(event) = self.events.borrow_mut().queue().first() {
        //     match event.action {
        //         Action::Button(tag) => {
        //             match tag {
        //                 BACK_BUTTON => { nav_event = Some(NavEvent::Back) },
        //                 NEXT_BUTTON => { nav_event = Some(NavEvent::Next) },
        //                 _ => {}
        //             }
        //         },
        //         Action::Selected(idx) => { nav_event = Some(NavEvent::Selected(idx)) },
        //         // _ => {}
        //     }
        // }
        // if let Some(evt) = nav_event {
        //     ctx.event_bus.register_event(evt);
        // }

        let _ = self.stage.update(window, state);

    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let _ = self.stage.render(theme, window);
        // let _ = self.navbar.render(theme, window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        self.stage.handle_mouse_at(pt, window)

    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        println!(">>> handle_mouse_down");
        // if let Some(ref mut rc) = self.nav.upgrade() {
        //     let mut nav = rc.borrow_mut();
        //     (&mut *nav).notify("Booo");
        //     // rc.borrow_mut().notify("Mouse down");
        // }
        self.stage.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        self.stage.handle_mouse_up(pt, state)
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut AppState) {
        self.stage.handle_mouse_scroll(pt, state);
    }
}

