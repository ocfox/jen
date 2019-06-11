//! Parent module for helper function exports.
//!
//! This module contains functions to construct helpers which can
//! be baked into the `Generator` to control additional features
//! against the internal templating engine.
use tera::GlobalFn;

mod custom;
mod fake;

/// Returns a named vector containing named helper pairs.
///
/// This function returns all built-in helpers, so you can append
/// your own helpers as necessary. In the case of a name clash with
/// a built-in helper, the behaviour should be deemed undefined.
pub fn builtin() -> Vec<(&'static str, GlobalFn)> {
    let mut vec = custom::helpers();
    vec.append(&mut fake::helpers());
    vec
}
