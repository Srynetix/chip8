//! Window fonts.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use sdl2::ttf::Sdl2TtfContext;

pub use sdl2::ttf::Font;

use crate::core::error::CResult;

/// Missing font error.
#[derive(Debug)]
pub struct MissingFontError(String);

impl Error for MissingFontError {
    fn description(&self) -> &str {
        "Missing font"
    }
}

impl fmt::Display for MissingFontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Font is not found: {}", self.0)
    }
}

/// Font handler.
pub struct FontHandler<'ttf, 'a> {
    ttf_context: &'ttf Sdl2TtfContext,
    font_paths: HashMap<String, PathBuf>,
    font_data: HashMap<String, Font<'ttf, 'a>>,
}

impl<'ttf, 'a> FontHandler<'ttf, 'a> {
    /// Create font handler.
    ///
    /// # Arguments
    ///
    /// * `ttf_context` - TTF context.
    ///
    /// # Returns
    ///
    /// * Font handler instance.
    ///
    pub fn new(ttf_context: &'ttf Sdl2TtfContext) -> Self {
        Self {
            ttf_context,
            font_paths: HashMap::new(),
            font_data: HashMap::new(),
        }
    }

    fn create_font_name(&self, name: &str, font_size: u16) -> String {
        format!("{}:{}", name, font_size)
    }

    /// Register font path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path.
    /// * `name` - Name.
    ///
    pub fn register_font_path(&mut self, path: &Path, name: &str) {
        self.font_paths
            .insert(String::from(name), PathBuf::from(path));
    }

    /// Create font.
    ///
    /// # Arguments
    ///
    /// * `name` - Name.
    /// * `font_size` - Font size.
    ///
    /// # Returns
    ///
    /// * Font result.
    ///
    pub fn create_font(&mut self, name: &str, font_size: u16) -> CResult<&Font<'ttf, 'a>> {
        if !self.font_paths.contains_key(name) {
            Err(Box::new(MissingFontError(name.to_string())))
        } else {
            let path = &self.font_paths[name];
            let font = self.ttf_context.load_font(path, font_size)?;
            let font_name = self.create_font_name(name, font_size);

            self.font_data.insert(font_name.clone(), font);
            Ok(&self.font_data[&font_name])
        }
    }

    /// Get font.
    ///
    /// # Arguments
    ///
    /// * `name` - Name.
    /// * `font_size` - Font size.
    ///
    /// # Returns
    ///
    /// * Font option.
    ///
    pub fn get_font(&self, name: &str, font_size: u16) -> Option<&Font<'ttf, 'a>> {
        let font_name = self.create_font_name(name, font_size);
        self.font_data.get(&font_name)
    }

    /// Get or create font.
    ///
    /// # Arguments
    ///
    /// * `name` - Name.
    /// * `font_size` - Font size.
    ///
    /// # Returns
    ///
    /// * Font result.
    ///
    pub fn get_or_create_font(&mut self, name: &str, font_size: u16) -> CResult<&Font<'ttf, 'a>> {
        if !self.font_paths.contains_key(name) {
            Err(Box::new(MissingFontError(name.to_string())))
        } else {
            let font_name = self.create_font_name(name, font_size);
            if self.font_data.contains_key(&font_name) {
                Ok(&self.font_data[&font_name])
            } else {
                self.create_font(name, font_size)
            }
        }
    }
}
