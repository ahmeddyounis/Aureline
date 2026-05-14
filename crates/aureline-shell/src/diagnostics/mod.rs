//! Shell diagnostic surfaces and diagnostic import consumers.
//!
//! This module owns shell-facing diagnostic projections that sit above the
//! language diagnostic bus. The first imported scanner lane lives here so
//! Problems and support-export views can consume imported evidence without
//! treating it as current live analysis.

pub mod experiments_inventory;
pub mod imported;
