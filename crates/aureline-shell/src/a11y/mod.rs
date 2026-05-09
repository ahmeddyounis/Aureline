//! Shell accessibility bridge groundwork.
//!
//! This module maps the live desktop shell surfaces onto the shared
//! accessibility-tree contract (`docs/accessibility/accessibility_tree_contract.md`).
//! The initial implementation focuses on non-editor shell surfaces that are
//! reachable in the native shell runtime: shell zones, the Start Center action
//! list, the command palette search overlay, embedded docs/help boundary chrome,
//! and placeholder tool panels such as the terminal.

pub mod shell_bridge;
pub mod tree_contract;
