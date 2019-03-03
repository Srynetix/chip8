//! Scene

use std::error::Error;

use sdl2::EventPump;

use super::draw::DrawContext;
use super::scenemanager::SceneContext;

/// Scene trait
pub trait Scene {
    /// Initialize
    fn init(&mut self);
    /// Input
    fn input(&mut self, event_pump: &mut EventPump, ctx: &mut SceneContext);
    /// Render
    fn render(&self, ctx: &mut DrawContext) -> Result<(), Box<dyn Error>>;
    /// Destroy
    fn destroy(&mut self);
}
