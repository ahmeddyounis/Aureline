//! Docs authoring workspace, suggestion-panel, and validation-report records.
//!
//! This module groups the governed docs-authoring truth packets:
//! [`markdown_workspace`] owns the Markdown authoring workspace,
//! [`suggestion_panel`] owns the diff-first docs suggestion panel that proposes
//! prose edits to README/changelog/help/tutorial docs and ties each proposal
//! back to the code, schema, or release change that raised it, and
//! [`validation_report`] owns the example/link validation report that turns
//! documented examples and links into typed, reviewable validation rows with
//! explicit validation modes, last-checked time, environment/version scope, and
//! producing-validator context.
//!
//! The [`markdown_workspace`] module owns the runtime truth packet for the
//! governed Markdown authoring workspace: the source/split/rendered modes a
//! workspace exposes,
//! the stable command ids and keyboard parity that drive them, the remembered
//! mode preference, the always-available recovery back to raw source, the
//! CommonMark baseline and enabled-extension disclosure, the rendered-preview
//! sanitization posture, the diagram/math/custom-component capability posture,
//! the source/version/freshness badge, the initiating code/doc anchor context,
//! the mirror/offline state, and the browser-handoff availability.
//!
//! The records carry only inspectable metadata — stable refs, mode and command
//! tokens, capability postures, and disclosure notes. Raw Markdown bodies, raw
//! source files, rendered HTML, raw provider payloads, and credentials never
//! cross this boundary.

pub mod markdown_workspace;
pub mod suggestion_panel;
pub mod validation_report;
