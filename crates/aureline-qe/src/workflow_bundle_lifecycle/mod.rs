//! Workflow-bundle lifecycle conformance, interoperability, certification
//! freshness, and failure / recovery drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/workflow_bundle_lifecycle/` and runs it against the
//! workflow-bundle lifecycle beta boundary owned by `aureline-workspace::bundles`
//! ([`WorkflowBundleReviewRecord`]). Every positive `workflow_bundle_review`
//! drill MUST parse, validate, project, and satisfy the per-drill expectations
//! encoded in the corpus manifest — the bundle/source/status/support classes,
//! the effective badge after evidence/dependency/mirror checks, the support
//! claim, the mirror/offline posture, the granular drift / removal / override
//! counts, the review and drift-resolution actions, whether removal preserves
//! user-owned assets, whether the rollback checkpoint restores bundle-owned
//! state, and the capability-dependency and lifecycle-sensitive markers that
//! must propagate across the certification, install/update, and export surfaces.
//! Every negative drill MUST FAIL validation with an error whose message
//! contains `expected_failure_substring`, so a bundle that over-claims stale
//! evidence, endangers a user-owned asset, widens trust, hides the diff, skips
//! the change-preview route, or leaks raw secrets stays rejected before a beta
//! bundle row hardens.
//!
//! Surfaces that need to reuse the matrix outside the test runner can call
//! [`load_corpus`] + [`run_corpus`] directly; the [`CorpusReport`] returned by
//! [`run_corpus`] is the canonical pass / fail truth.
//!
//! [`WorkflowBundleReviewRecord`]: aureline_workspace::WorkflowBundleReviewRecord

mod manifest;
mod runner;

pub use manifest::{
    drill_kind, CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL,
    MANIFEST_FILE_NAME,
};
pub use runner::{
    corpus_dir_from_repo_root, load_corpus, run_corpus, run_corpus_from_repo_root, CorpusReport,
    DrillFailureReason, DrillOutcome, DrillReport,
};
