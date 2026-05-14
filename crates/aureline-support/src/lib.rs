//! Support-bundle manifest, redaction defaults, local preview, and exact-build capture.
//!
//! This crate is the live shell's first trustworthy support-export path. It
//! mints a structured support-bundle manifest with redaction defaults, exact-
//! build identity, and a local preview that the chrome can show before any
//! share or upload step.
//!
//! ## What this seed owns
//!
//! - The [`bundle::SupportBundleManifest`] record — the canonical truth
//!   model for what a bundle would contain on export. It mirrors the
//!   boundary schema at
//!   `/schemas/support/support_bundle_manifest.schema.json` so the chrome
//!   and the export writer never invent divergent shapes.
//! - The [`bundle::SupportBundlePreviewItem`] row — one inspectable line in
//!   the local preview. Mirrors
//!   `/schemas/support/support_bundle_preview_item.schema.json`.
//! - The [`bundle::redaction::LocalFirstDefaults`] profile — the default
//!   redaction posture for a local-first export. Mirrors the seed profile
//!   at `/fixtures/support/redaction_profiles/local_first_default.yaml`.
//! - The [`bundle::ExactBuildCapture`] join — quotes
//!   [`aureline_build_info::build_identity`] and
//!   [`aureline_build_info::release_channel_class`] verbatim so the
//!   manifest carries the same exact-build identity as the running binary.
//! - The [`bundle::SupportBundlePreview`] projection — the read-only view
//!   the shell renders before letting the user export.
//!
//! ## What this seed does NOT own
//!
//! - Live byte-level redaction implementation, upload transport, hosted
//!   intake, or ticket routing. Those belong to later milestones.
//! - The full diagnostic_artifact_matrix item set. The seed surfaces the
//!   minimum row classes needed to prove the protected walk and the
//!   failure drill (metadata + secret-bearing prohibited).
//! - Live Project Doctor probe ownership. The [`project_doctor`] module
//!   consumes the checked-in read-only alpha probe pack and the runtime
//!   support projection from `aureline-doctor`, but it does not apply repairs.
//! - The [`recovery_ladder`] alpha evaluator — consumes typed recovery
//!   evidence and emits bounded safe-mode, quarantine, open-without-restore,
//!   and cache/index repair decisions plus support/release projections.
//! - The [`repair`] alpha preview compiler — consumes checked-in repair seed
//!   cases and emits typed transaction, preview, outcome, and journal records
//!   before any guided repair can apply.
//!
//! ## Failure-drill posture
//!
//! When a caller queues a row whose contents would carry secret-bearing
//! material, the manifest forces the row's redaction state to `prohibited`,
//! emits an [`bundle::ExcludedClass`] entry with an explicit reason, and
//! adds the support-pack item id to the manifest's
//! `prohibited_items_confirmed_absent` list. The protected-walk preview
//! never exports raw secret bytes even if a caller mis-classifies them.

#![doc(html_root_url = "https://docs.rs/aureline-support/0.0.0")]

pub mod bundle;
pub mod project_doctor;
pub mod recovery_ladder;
pub mod repair;
