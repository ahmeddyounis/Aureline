//! History-rewrite, stash-recovery, reflog, and conflict-session
//! conformance harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/git/m3/history_rewrite_corpus/` and runs it against the
//! beta history-rewrite contract owned by `aureline-git`. Every
//! positive drill MUST parse, validate, project, and satisfy the
//! per-drill projection expectations encoded in the corpus manifest.
//! Every negative drill MUST FAIL validation with the recorded
//! `expected_failure_substring` so the harness keeps protecting against
//! silent scope widening, lost stash provenance, mislabeled reflog
//! recovery, and raw-body export regressions.
//!
//! Surfaces that need to reuse the matrix outside the test runner can
//! call [`load_corpus`] + [`run_corpus`] directly; the
//! [`CorpusReport`] returned by [`run_corpus`] is the canonical
//! pass / fail truth.

mod manifest;
mod runner;

pub use manifest::{
    CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL, MANIFEST_FILE_NAME,
};
pub use runner::{
    load_corpus, run_corpus, run_corpus_from_repo_root, CorpusReport, DrillFailureReason,
    DrillOutcome, DrillReport,
};
