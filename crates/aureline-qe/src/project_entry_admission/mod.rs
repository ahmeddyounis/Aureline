//! Project-entry and workspace-admission conformance, interoperability, failure
//! / recovery, and switching-parity drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/project_entry_and_admission/` and runs it against the
//! project-entry review boundary owned by `aureline-workspace`
//! ([`build_project_entry_review`]). Every positive drill carries one
//! [`ProjectEntryReviewRequest`]; the harness builds the review record, requires
//! it to be contract-valid, and matches the per-drill expectations encoded in the
//! corpus manifest — the verb-specific review sheet, the source-labelled access
//! class, the first-useful entry source and landing surface, the resulting mode,
//! the primary next action, the destination-collision posture, the Blocking now /
//! Recommended soon / Optional later readiness counts, the deferred work, and the
//! import inspect/write posture. It also pins, on every drill, the cross-cutting
//! entry guarantees: no silent trust grant, no setup or task / hook execution, no
//! route auto-trust or auto-install, a preserved entry intent, redaction safety,
//! and a deep-link parity row that always requires deep-link intent review. Every
//! negative drill applies a typed tamper to the built record and requires the
//! entry contract to reject it with a finding containing the recorded substring,
//! so an entry path that widens trust, leaks credentials, writes before review,
//! drops the collision choice, drifts a cross-surface parity row, loses
//! failed-attempt inputs, or lets detection auto-trust / auto-install stays
//! rejected before a beta entry row hardens.
//!
//! Surfaces that need to reuse the matrix outside the test runner can call
//! [`load_corpus`] + [`run_corpus`] directly; the [`CorpusReport`] returned by
//! [`run_corpus`] is the canonical pass / fail truth.
//!
//! [`build_project_entry_review`]: aureline_workspace::build_project_entry_review
//! [`ProjectEntryReviewRequest`]: aureline_workspace::ProjectEntryReviewRequest

mod manifest;
mod runner;

pub use manifest::{
    CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, PositiveExpect, Tamper, CORPUS_DIR_REL,
    MANIFEST_FILE_NAME,
};
pub use runner::{
    corpus_dir_from_repo_root, load_corpus, run_corpus, run_corpus_from_repo_root, CorpusReport,
    DrillFailureReason, DrillOutcome, DrillReport,
};
