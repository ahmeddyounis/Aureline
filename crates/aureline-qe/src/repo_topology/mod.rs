//! Repo-topology beta conformance and recovery-drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/repo_topology_corpus/` and runs it against
//! the four repo-topology beta descriptors and the cross-surface
//! projection owned by `aureline-workspace::repo_topology`. Every
//! positive drill MUST parse, project, and satisfy the per-drill
//! projection expectations encoded in the corpus manifest. Every
//! negative drill MUST FAIL projection with an error whose message
//! contains `expected_failure_substring` so the harness keeps
//! protecting against silent topology widening, descriptor
//! cross-binding, and raw-body export regressions.
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
