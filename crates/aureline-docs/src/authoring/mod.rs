//! Markdown authoring workspace records.
//!
//! This module owns the runtime truth packet for the governed Markdown
//! authoring workspace: the source/split/rendered modes a workspace exposes,
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
