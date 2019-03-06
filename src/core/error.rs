//! Error module

use std::error::Error;

/// Common error
pub type CResult<T = ()> = Result<T, Box<dyn Error>>;
