//! Scene

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use super::draw::DrawContext;
use super::scenemanager::SceneContext;
use crate::error::CResult;

/// Scene trait
pub trait Scene {
    /// Initialize
    fn init(&mut self, ctx: &mut SceneContext);
    /// Event
    fn event(&mut self, ctx: &mut SceneContext, e: &Event);
    /// Update
    fn update(&mut self, ctx: &mut SceneContext, event_pump: &mut EventPump);
    /// Key up
    fn keyup(&mut self, ctx: &mut SceneContext, code: Keycode);
    /// Key down
    fn keydown(&mut self, ctx: &mut SceneContext, code: Keycode);
    /// Render
    fn render(&mut self, ctx: &mut DrawContext) -> CResult;
    /// Destroy
    fn destroy(&mut self, ctx: &mut SceneContext);
}
