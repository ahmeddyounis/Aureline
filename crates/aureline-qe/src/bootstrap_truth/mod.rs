//! Repository-acquisition and post-open bootstrap truth conformance and
//! failure / recovery drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/bootstrap_truth_corpus/` and runs it against the
//! repository-acquisition beta projection owned by
//! `aureline-workspace::acquisition`. Every positive drill MUST parse,
//! project, and satisfy the per-drill projection expectations encoded in
//! the corpus manifest — acquisition verb, locator/transport identity,
//! checkout shape and cost band, credential posture, interrupted-recovery
//! branches, manual follow-up queue, honesty labels, guardrails, and the
//! export-safe evidence packet. Every negative drill MUST FAIL projection
//! with an error whose message contains `expected_failure_substring` so the
//! harness keeps protecting against silent hydrate/init/fetch, wrong-target
//! clone state, lost source/plan/queue lineage, and raw-secret export
//! regressions before beta acquisition claims harden.
//!
//! Surfaces that need to reuse the matrix outside the test runner can call
//! [`load_corpus`] + [`run_corpus`] directly; the [`CorpusReport`] returned
//! by [`run_corpus`] is the canonical pass / fail truth.

mod manifest;
mod runner;

pub use manifest::{
    CorpusManifest, GuardrailExpectations, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL,
    MANIFEST_FILE_NAME,
};
pub use runner::{
    corpus_dir_from_repo_root, load_corpus, run_corpus, run_corpus_from_repo_root, CorpusReport,
    DrillFailureReason, DrillOutcome, DrillReport,
};
