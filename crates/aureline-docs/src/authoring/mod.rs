//! Docs authoring workspace, suggestion-panel, and validation-report records.
//!
//! This module groups the governed docs-authoring truth packets:
//! [`markdown_workspace`] owns the Markdown authoring workspace,
//! [`suggestion_panel`] owns the diff-first docs suggestion panel that proposes
//! prose edits to README/changelog/help/tutorial docs and ties each proposal
//! back to the code, schema, or release change that raised it,
//! [`validation_report`] owns the example/link validation report that turns
//! documented examples and links into typed, reviewable validation rows with
//! explicit validation modes, last-checked time, environment/version scope, and
//! producing-validator context, and [`release_docs_surface`] owns the dedicated
//! README/changelog/onboarding release-docs maintenance surfaces that make
//! branch/release/channel scope, pending suggestions, compare history, the
//! publish/export boundary, and local-versus-shared evidence scope visible
//! before edit and inspectable after the user leaves the surface, and
//! [`evidence_handoff`] owns the docs-evidence handoff packets that bind a prose
//! change or suggestion back to the files, symbols, API contracts, failing
//! examples, test runs, release objects, or human-authored notes that motivated
//! it — preserving local-only versus shared/export-safe scope, redaction state,
//! and mirror/offline continuity so review, support, AI explanation, and
//! release/public-truth surfaces can reopen the same docs causality Aureline
//! used in the authoring workspace.
//!
//! [`safe_rendered_preview`] owns the rendered-preview capability-boundary packet
//! that makes the per-capability request state, render posture, honest-degradation
//! fallback, raw/source escape, external-open path, and no-authority-expansion
//! guarantee inspectable for diagrams, front matter, math, callouts, remote
//! assets, and extension/custom components, so a richer rendered preview is never
//! an unlabeled active surface and never widens authority.
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

pub mod evidence_handoff;
pub mod markdown_workspace;
pub mod release_docs_surface;
pub mod safe_rendered_preview;
pub mod suggestion_panel;
pub mod validation_report;
