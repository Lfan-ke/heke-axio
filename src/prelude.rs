//! The I/O Prelude.
//!
//! The purpose of this module is to alleviate imports of many common I/O traits
//! by adding a glob import to the top of I/O heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use axio::prelude::*;
//! ```

/// Re-exports commonly used I/O traits.
pub use super::{BufRead, Read, Seek, Write};
