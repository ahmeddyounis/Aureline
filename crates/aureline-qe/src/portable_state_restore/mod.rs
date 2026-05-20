//! Portable-state and restore-provenance conformance, interoperability, and
//! failure / recovery drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/portable_state_and_restore_conformance/` and runs it
//! against the portable-state / restore-provenance beta boundary owned by
//! `aureline-workspace::serialization`. Every positive `restore_provenance_card`
//! drill MUST parse, validate, and satisfy the per-drill expectations encoded
//! in the corpus manifest — source event, schema outcome, resulting fidelity,
//! the controlled downgrade label, the missing-surface dependencies that
//! reopen as placeholders, the named high-risk exclusions, and the
//! compare/export refs that keep the prior artifact available. Every positive
//! `alpha_migration` drill additionally migrates an older alpha package forward
//! and proves the migration keeps layers separated, machine-local hints
//! excluded, path/host redaction available, live authority un-rehydrated, and
//! the inspector / export / import review surfaces reviewable. Every negative
//! drill MUST FAIL validation with an error whose message contains
//! `expected_failure_substring`, so a restore that widens meaning, hides the
//! prior artifact, strands a placeholder, or reuses a placeholder id stays
//! rejected before a beta continuity row hardens.
//!
//! Surfaces that need to reuse the matrix outside the test runner can call
//! [`load_corpus`] + [`run_corpus`] directly; the [`CorpusReport`] returned by
//! [`run_corpus`] is the canonical pass / fail truth.

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
