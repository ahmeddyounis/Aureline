//! Input modeling and deterministic keybinding resolution.
//!
//! This crate is responsible for turning keyboard sequences into stable command
//! ids with deterministic precedence, winning-source attribution, and
//! conflict-explainability. It exists so the shell, keybinding UI, migration,
//! help, and support surfaces can share one resolver contract rather than
//! re-implementing shortcut logic locally.
//!
//! Frozen boundaries referenced by this crate:
//!
//! - `docs/ux/keybinding_resolver_contract.md`
//! - `schemas/commands/keybinding_resolver.schema.json`
//! - `schemas/config/keybindings.schema.json`

#![doc(html_root_url = "https://docs.rs/aureline-input/0.0.0")]

pub mod keybindings;

