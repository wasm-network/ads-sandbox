/// A custom AppDelegate
///
///
///
use super::*;
use crate::samples::*;

use tweek::{
    core::*,
    events::*,
    gui::*,
};

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::{ButtonState, Key, MouseButton},
    lifecycle::{Event, State, Window},
    Error, Result,
};

use std::any::TypeId;

const FPS_INTERVAL: usize = 40;
pub const FPS_TAG: u32 = 901;

//-- Main -----------------------------------------------------------------------

/// AppDelegate serves as a layer between the backend runloop and Tweek UI.
///
pub struct AppDelegate {
    controller: Box<dyn Controller>,
    theme: Theme,
    app_state: AppState,
    data_scene: Scene,
    frames: usize,
    did_launch: bool,
}

impl AppDelegate {
    /// Constructor
    pub fn new(screen: Vector) -> Self {

        let mut theme = Theme::default();
        theme.font_size = 18.0;
        theme.bg_color = Color::from_hex("#FFFFEE");

        // Add data scene for displaying FPS and other info
        let frame = Rectangle::new_sized(screen);
        let mut data_scene = Scene::new(frame);

        let x = 20.0;
        let y = screen.y - 40.0;
        let frame = Rectangle::new((x, y), (80.0, 20.0));
        let mut text = Text::new(frame, "");
        text.layer.font_style = FontStyle::new(14.0, Color::RED);
        text.set_id(FPS_TAG);
        data_scene.add_control(Box::new(text));

        let mut app_state = AppState::new();
        app_state.window_size = (screen.x, screen.y);

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let controller = Box::new(EmptyController{});

        let app = AppDelegate {
            controller,
            theme,
            app_state,
            data_scene,
            frames: 0,
            did_launch: false,
        };
        app
    }

    /// Application lifecycle event called before runloop starts
    pub fn application_ready(&mut self, screen: Vector) {

        let mut controller = AdViewer::new(Rectangle::new_sized(screen));
        // Load stage here
        controller.view_will_load(&mut self.theme);
        controller.set_theme(&mut self.theme);
        self.controller = Box::new(controller);
    }

}

// ************************************************************************************
// ************************************************************************************

#[allow(dead_code)]
#[allow(unused_variables)]
impl State for AppDelegate {
    fn new() -> Result<AppDelegate> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.controller.update(window, &mut self.app_state);
        for event in self.app_state.event_bus.into_iter() {
            self.controller.handle_event(&event);
        }

        self.frames += 1;
        if (self.frames % FPS_INTERVAL) == 0 {
            self.frames = 0;
            let out = format!("FPS: {:.2}", window.current_fps());
            self.data_scene.set_field_value(&FieldValue::Text(out), TypeId::of::<Text>(), FPS_TAG);
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(self.theme.bg_color)?;
        self.controller.render(&mut self.theme, window);
        self.data_scene.render(&mut self.theme, window);

        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("event={:?}", event);
            }
            Event::MouseMoved(pt) => {
                let _hover = self.controller.handle_mouse_at(pt, window);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.controller.handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                self.controller.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseWheel(xy) => {
                self.controller.handle_mouse_scroll(xy, &mut self.app_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    // self.controller.handle_key_command(key, window);
                }
            },
            Event::Typed(c) => {
                // self.controller.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}
