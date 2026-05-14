//! Embedded surface boundary chrome and browser-handoff wiring.
//!
//! Embedded docs/help, marketplace/account, service dashboards, and extension
//! web-like surfaces must stay honest about where content comes from and which
//! actions remain host-owned. This module anchors the render-side boundary card
//! contract and provides the smallest runnable shell integration for exercising
//! owner/origin chrome plus a system-browser handoff escape hatch.

pub mod boundary_alpha;
pub mod boundary_card;
pub mod boundary_fallback_alpha;
pub mod docs_help;
