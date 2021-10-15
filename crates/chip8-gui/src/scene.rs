use std::collections::HashMap;

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
    pub fn new() -> Self {
        Default::default()
    }

    /// Quit.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Set current scene.
    pub fn set_current_scene(&mut self, name: &str) {
        self.current_scene_name = Some(String::from(name));
    }

    /// Set cache data.
    pub fn set_cache_data(&mut self, key: &str, value: String) {
        self.cache_data.insert(String::from(key), value);
    }

    /// Get cache data.
    pub fn get_cache_data(&self, key: &str) -> Option<String> {
        self.cache_data.get(key).cloned()
    }
}

/// Scene trait.
pub trait Scene {
    /// Initialize.
    fn init(&mut self, ctx: &mut SceneContext);

    /// Update.
    fn update(&mut self, ctx: &mut SceneContext);

    /// Render.
    fn render(&mut self);

    /// Destroy.
    fn destroy(&mut self, ctx: &mut SceneContext);
}

/// Scene manager.
#[derive(Default)]
pub struct SceneManager {
    /// Last loaded scene.
    pub last_loaded_scene: Option<String>,
    /// Scenes.
    pub scenes: HashMap<String, Box<dyn Scene>>,
}

pub enum SceneRunResult {
    Continue,
    Stop,
}

impl SceneManager {
    /// Create new scene manager.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get scene.
    pub fn get_scene(&mut self, name: &str) -> Option<&mut dyn Scene> {
        if let Some(scene) = self.scenes.get_mut(name) {
            Some(&mut **scene)
        } else {
            None
        }
    }

    /// Register scene.
    pub fn register_scene(&mut self, name: &str, scene: Box<dyn Scene>) {
        self.scenes.insert(String::from(name), scene);
    }

    /// Get current scene.
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
                let scene = self.get_scene(scene).expect("missing scene");
                scene.init(ctx);

                // Set as last loaded.
                self.last_loaded_scene = ctx.current_scene_name.as_ref().cloned();
            }
        }
    }

    /// Step scene
    pub fn step(&mut self, scene_context: &mut SceneContext) -> SceneRunResult {
        if scene_context.running {
            let scene = self
                .get_current_scene(scene_context)
                .expect("missing scene");

            scene.render();
            scene.update(scene_context);

            SceneRunResult::Continue
        } else {
            SceneRunResult::Stop
        }
    }
}
