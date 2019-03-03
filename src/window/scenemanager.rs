//! Scene manager

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use super::draw::DrawContext;
use super::scene::Scene;

/// Scene context
#[derive(Debug)]
pub struct SceneContext {
    /// Current scene name
    pub current_scene_name: Option<String>,
    /// Running
    pub running: bool,
}

impl Default for SceneContext {
    fn default() -> Self {
        Self {
            current_scene_name: None,
            running: true,
        }
    }
}

impl SceneContext {
    /// New context
    pub fn new() -> Self {
        Default::default()
    }

    /// Quit
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Set current scene
    pub fn set_current_scene(&mut self, name: &str) {
        self.current_scene_name = Some(String::from(name));
    }
}

/// Scene manager
pub struct SceneManager {
    /// Last loaded scene
    pub last_loaded_scene: Option<String>,
    /// Scenes
    pub scenes: HashMap<String, Box<Scene>>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self {
            scenes: HashMap::new(),
            last_loaded_scene: None,
        }
    }
}

impl SceneManager {
    /// Create new scene manager
    pub fn new() -> Self {
        Default::default()
    }

    /// Get scene
    pub fn get_scene(&mut self, name: &str) -> Option<&mut Scene> {
        if let Some(scene) = self.scenes.get_mut(name) {
            Some(&mut **scene)
        } else {
            None
        }
    }

    /// Register scene
    pub fn register_scene(&mut self, name: &str, scene: Box<Scene>) {
        self.scenes.insert(String::from(name), scene);
    }

    /// Get current scene
    pub fn get_current_scene(&mut self, ctx: &mut SceneContext) -> Option<&mut Scene> {
        let name = ctx.current_scene_name.as_ref().cloned();

        if let Some(name) = name {
            self.handle_scene_transition(ctx);
            self.get_scene(&name)
        } else {
            None
        }
    }

    /// Has scene changed
    pub fn has_scene_changed(&self, ctx: &mut SceneContext) -> bool {
        match (&ctx.current_scene_name, &self.last_loaded_scene) {
            (Some(current), Some(last)) => {
                if current == last {
                    return false;
                }
            }
            (None, None) => {
                return false;
            }
            _ => {}
        }

        true
    }

    /// Handle scene transition
    fn handle_scene_transition(&mut self, ctx: &mut SceneContext) {
        // Check if scene changed
        let changed = self.has_scene_changed(ctx);
        let last_loaded_scene = self.last_loaded_scene.as_ref().cloned();

        if changed {
            // Destroy previous scene
            if let Some(scene) = last_loaded_scene {
                let scene = self.get_scene(&scene).expect("missing scene");
                scene.destroy();
            }

            // Load new scene
            if let Some(scene) = &ctx.current_scene_name {
                let scene = self.get_scene(&scene).expect("missing scene");
                scene.init();
            }
        }
    }

    /// Run loop
    pub fn run_loop(
        &mut self,
        ctx: &mut SceneContext,
        draw_context: &mut DrawContext,
        event_pump: &mut EventPump,
    ) {
        while ctx.running {
            let scene = self.get_current_scene(ctx).expect("missing scene");
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::Quit { .. } => {
                        ctx.quit();
                    }
                    _ => {}
                }
            }

            scene.render(draw_context).unwrap();
            scene.input(event_pump, ctx);

            draw_context.canvas.present();
        }
    }
}
