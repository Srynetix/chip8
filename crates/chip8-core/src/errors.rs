//! Error module.

use std::error::Error;

/// Common result.
pub type CResult<T = ()> = Result<T, Box<dyn Error>>;
