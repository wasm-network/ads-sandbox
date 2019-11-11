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

//-- Main -----------------------------------------------------------------------

/// AppDelegate serves as a layer between the backend runloop and Tweek UI.
///
pub struct AppDelegate {
    stage: Stage,
    theme: Theme,
    app_state: AppState,
    frames: usize,
    did_launch: bool,
}

impl AppDelegate {
    /// Constructor
    pub fn new(screen: Vector) -> Self {

        let mut theme = Theme::default();
        theme.font_size = 18.0;
        theme.bg_color = Color::from_hex("#FFFFEE");


        let mut app_state = AppState::new();
        app_state.window_size = (screen.x, screen.y);

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let stage = Stage::new(frame);

        let app = AppDelegate {
            stage,
            theme,
            app_state,
            frames: 0,
            did_launch: false,
        };
        app
    }

    /// Application lifecycle event called before runloop starts
    pub fn application_ready(&mut self) {


        self.stage.scenes.clear();
        // Load stage here
        self.stage.set_theme(&mut self.theme);
        self.stage.notify(&DisplayEvent::Ready);
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
        for event in self.app_state.event_bus.into_iter() {
        }
        self.stage.update(window, &mut self.app_state);

        // self.frames += 1;
        // if (self.frames % FPS_INTERVAL) == 0 {
        //     self.frames = 0;
        //     let out = format!("FPS: {:.2}", window.current_fps());
        //     self.nav_scene.set_field_value(&FieldValue::Text(out), TypeId::of::<Text>(), FPS_TAG);
        // }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(self.theme.bg_color)?;
        self.stage.render(&mut self.theme, window);
        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("event={:?}", event);
            }
            Event::MouseMoved(pt) => {
                let _hover = self.stage.handle_mouse_at(pt, window);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.stage.handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                self.stage.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseWheel(xy) => {
                self.stage.handle_mouse_scroll(xy, &mut self.app_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    self.stage.handle_key_command(key, window);
                }
            },
            Event::Typed(c) => {
                self.stage.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}
