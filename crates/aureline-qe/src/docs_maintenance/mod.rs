//! Docs preview / maintenance integrity conformance drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/docs/m3/docs_maintenance_corpus/` and runs it against the
//! canonical docs-maintenance records and validation owned by
//! `aureline-docs::maintenance`. It never re-implements the ruleset: it parses
//! each fixture into the real record type, runs the canonical validator, and
//! compares the result against the manifest's pinned truth.
//!
//! Every positive drill MUST validate cleanly (zero findings) and match its
//! pinned expectations — preview mode and sanitization posture, CommonMark
//! baseline, suggestion trigger and apply posture, finding class / detection /
//! validation mode, suppression attribution, and the branch / release /
//! channel / audience scope. Positive payloads are additionally scanned for
//! raw-URL or raw-body export leaks so the redaction contract lives on the
//! corpus itself.
//!
//! Every negative drill MUST FAIL validation with at least one finding whose
//! `check_id` contains `expected_violation_check_id`, so hidden renderer
//! extensions, silently-applied suggestions, unscoped README / changelog
//! updates, dropped suppression attribution, and wrong-branch / wrong-channel
//! maintenance (caught as review-packet drift against the seeded contract)
//! stay rejected before any beta docs-authoring claim hardens.
//!
//! Surfaces that need the matrix outside the test runner can call
//! [`load_corpus`] + [`run_corpus`] directly; the [`CorpusReport`] returned by
//! [`run_corpus`] is the canonical pass / fail truth.

mod manifest;
mod runner;

pub use manifest::{
    CorpusManifest, DrillRecordType, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL,
    MANIFEST_FILE_NAME,
};
pub use runner::{
    corpus_dir_from_repo_root, load_corpus, run_corpus, run_corpus_from_repo_root, CorpusReport,
    DrillFailureReason, DrillOutcome, DrillReport,
};
