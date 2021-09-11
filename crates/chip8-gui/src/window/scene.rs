//! Scene.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::core::error::CResult;

use super::draw::DrawContext;
use super::scenemanager::SceneContext;

/// Scene trait.
pub trait Scene {
    /// Initialize.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    ///
    fn init(&mut self, ctx: &mut SceneContext);

    /// Event.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    /// * `e` - Event.
    ///
    fn event(&mut self, ctx: &mut SceneContext, e: &Event);

    /// Update.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    /// * `event_pump` - Event pump.
    ///
    fn update(&mut self, ctx: &mut SceneContext, event_pump: &mut EventPump);

    /// Key up.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    /// * `code` - Keycode.
    ///
    fn keyup(&mut self, ctx: &mut SceneContext, code: Keycode);

    /// Key down.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    /// * `code` - Keycode.
    ///
    fn keydown(&mut self, ctx: &mut SceneContext, code: Keycode);

    /// Render.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Draw context.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    fn render(&mut self, ctx: &mut DrawContext) -> CResult;

    /// Destroy.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    ///
    fn destroy(&mut self, ctx: &mut SceneContext);
}
