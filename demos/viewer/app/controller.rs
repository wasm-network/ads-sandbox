use super::*;

use quicksilver::{
    geom::Vector,
    lifecycle::Window
};

use tweek::{
    core::{AppState},
    gui::{Theme},
    events::*,
};

/// See: https://developer.apple.com/documentation/uikit/uimodaltransitionstyle
pub enum ModalDisplayStyle {
    None,
    CoverVertical,
    FlipHorizontal,
    CrossDissolve,
}

pub enum TransitionState {
    None,
    Starting,
    Running,
    Finishing,
    Completed,
}

/// The Controller trait will behave like iOS controllers that are generally view controllers
/// that can load the objects to display in a Scene. Alternatively, a Controller could also be
/// a NavController, so that a modal controller could actually start a new navigation stack in
/// a modal view.
pub trait Controller {

    /// Provides the title for NavController and NavBar to display
    /// TODO: return struct with more information.
    fn screen_title(&self) -> &str { "" }

    /// Get next view controller to navigate to given a specified NavEvent (e.g. next, back, etc)
    // fn nav_target_for_event(&mut self, _event: &NavEvent, _ctx: &mut AppContext) -> Option<NavTarget> { None }

    /// This is the first stage in the view lifecycle after new() is called. Here is where you should
    /// layout subviews, load data, and prepare for display.
    /// TODO: pass theme as param
    fn view_will_load(&mut self, theme: &mut Theme);

    /// Set the theme. This starts from the AppDelegate and passes down to the controller(s)
    fn set_theme(&mut self, theme: &mut Theme);

    /// Method to signal that a controller will be leaving or entering the parent controller
    fn view_will_transition(&mut self, _event: NavEvent) {}

    /// Top-down notification of events to child objects
    fn handle_event(&mut self, event: &EventBox) {}

    /// The sync method is called from Quicksilver's update loop and eventually gets passed down
    /// to the Scene and lower level Tweek gui objects. It carries the AppContext as a mutable ref
    /// which contains the EventBus where events are propogated up (and possibly down, TBD)
    fn update(&mut self, window: &mut Window, state: &mut AppState);

    /// This is generally a passthru method to the Tweek gui components
    fn render(&mut self, theme: &mut Theme, window: &mut Window);

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_at(&mut self, _pt: &Vector, _window: &mut Window) -> bool { false }

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_down(&mut self, _pt: &Vector, _state: &mut AppState) -> bool { false }

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut AppState) -> bool { false }

    /// This is generally a passthru method to the Tweek gui controls
    fn handle_mouse_scroll(&mut self, _pt: &Vector, _state: &mut AppState) {}
}

/// A default controller that can be used as a placeholder during startup
pub struct EmptyController {}

impl Controller for EmptyController {
    fn view_will_load(&mut self, theme: &mut Theme) {}
    fn set_theme(&mut self, theme: &mut Theme) {}
    fn update(&mut self, window: &mut Window, state: &mut AppState) {}
    fn render(&mut self, theme: &mut Theme, window: &mut Window) {}
}

// UNUSED
pub trait Container {
    fn render_views(&mut self, theme: &mut Theme, window: &mut Window);
    fn handle_mouse_down(&mut self, _pt: Vector) {}
    fn handle_mouse_up(&mut self, _pt: Vector) {}
}

