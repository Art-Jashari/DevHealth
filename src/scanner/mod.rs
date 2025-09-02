//! Scanner modules for development environment analysis
//!
//! This module contains various scanners that analyze different aspects
//! of development environments:
//!
//! - [`git`]: Git repository health and status analysis
//! - [`deps`]: Dependency health checking across multiple ecosystems
//! - [`system`]: System resource monitoring (planned)
//! - [`analytics`]: Project analytics and metrics (planned)

pub mod analytics;
pub mod deps;
pub mod git;
pub mod system;
