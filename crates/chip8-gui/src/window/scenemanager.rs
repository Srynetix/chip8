//! Scene manager.

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::EventPump;

use super::draw::DrawContext;
use super::scene::Scene;

/// Scene context.
#[derive(Debug)]
pub struct SceneContext {
    /// Current scene name.
    pub current_scene_name: Option<String>,
    /// Running.
    pub running: bool,
    /// Cache data.
    pub cache_data: HashMap<String, String>,
}

impl Default for SceneContext {
    fn default() -> Self {
        Self {
            current_scene_name: None,
            running: true,
            cache_data: HashMap::new(),
        }
    }
}

impl SceneContext {
    /// Create new context.
    ///
    /// # Returns
    ///
    /// * Scene context instance.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Quit.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Set current scene.
    ///
    /// # Arguments
    ///
    /// * `name` - Current scene name.
    ///
    pub fn set_current_scene(&mut self, name: &str) {
        self.current_scene_name = Some(String::from(name));
    }

    /// Set cache data.
    ///
    /// # Arguments
    ///
    /// * `key` - Key name.
    /// * `value` - Value.
    ///
    pub fn set_cache_data(&mut self, key: &str, value: String) {
        self.cache_data.insert(String::from(key), value);
    }

    /// Get cache data.
    ///
    /// # Arguments
    ///
    /// * `key` - Key name.
    ///
    /// # Returns
    ///
    /// * Cache data.
    ///
    pub fn get_cache_data(&self, key: &str) -> Option<String> {
        self.cache_data.get(key).cloned()
    }
}

/// Scene manager.
pub struct SceneManager {
    /// Last loaded scene.
    pub last_loaded_scene: Option<String>,
    /// Scenes.
    pub scenes: HashMap<String, Box<dyn Scene>>,
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
    /// Create new scene manager.
    ///
    /// # Returns
    ///
    /// * Scene manager instance.
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Get scene.
    ///
    /// # Arguments
    ///
    /// * `name` - Scene name.
    ///
    /// # Returns
    ///
    /// * Scene option.
    ///
    pub fn get_scene(&mut self, name: &str) -> Option<&mut dyn Scene> {
        if let Some(scene) = self.scenes.get_mut(name) {
            Some(&mut **scene)
        } else {
            None
        }
    }

    /// Register scene.
    ///
    /// # Arguments
    ///
    /// * `name` - Scene name.
    /// * `scene` - Scene box.
    ///
    pub fn register_scene(&mut self, name: &str, scene: Box<dyn Scene>) {
        self.scenes.insert(String::from(name), scene);
    }

    /// Get current scene.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    ///
    /// # Returns
    ///
    /// * Scene option.
    ///
    pub fn get_current_scene(&mut self, ctx: &mut SceneContext) -> Option<&mut dyn Scene> {
        let name = ctx.current_scene_name.as_ref().cloned();

        if let Some(name) = name {
            self.handle_scene_transition(ctx);
            self.get_scene(&name)
        } else {
            None
        }
    }

    /// Has scene changed.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Scene context.
    ///
    /// # Returns
    ///
    /// * `true` if scene changed.
    /// * `false` if not.
    ///
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

    fn handle_scene_transition(&mut self, ctx: &mut SceneContext) {
        // Check if scene changed.
        let changed = self.has_scene_changed(ctx);
        let last_loaded_scene = self.last_loaded_scene.as_ref().cloned();

        if changed {
            // Destroy previous scene.
            if let Some(scene) = last_loaded_scene {
                let scene = self.get_scene(&scene).expect("missing scene");
                scene.destroy(ctx);
            }

            // Load new scene.
            if let Some(scene) = &ctx.current_scene_name {
                let scene = self.get_scene(&scene).expect("missing scene");
                scene.init(ctx);

                // Set as last loaded.
                self.last_loaded_scene = ctx.current_scene_name.as_ref().cloned();
            }
        }
    }

    /// Run loop.
    ///
    /// # Arguments
    ///
    /// * `scene_context` - Scene context.
    /// * `draw_context` - Draw context.
    /// * `event_pump` - Event pump.
    ///
    pub fn run_loop(
        &mut self,
        scene_context: &mut SceneContext,
        draw_context: &mut DrawContext,
        event_pump: &mut EventPump,
    ) {
        while scene_context.running {
            let scene = self
                .get_current_scene(scene_context)
                .expect("missing scene");

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        scene_context.quit();
                    }
                    Event::KeyDown {
                        keycode: Some(kc), ..
                    } => scene.keydown(scene_context, kc),
                    Event::KeyUp {
                        keycode: Some(kc), ..
                    } => scene.keyup(scene_context, kc),
                    e => scene.event(scene_context, &e),
                }
            }

            scene.render(draw_context).unwrap();
            scene.update(scene_context, event_pump);

            draw_context.canvas.present();
        }
    }
}
