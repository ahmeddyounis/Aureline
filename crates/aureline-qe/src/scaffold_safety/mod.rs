//! Scaffold and generated-project safety conformance and failure / recovery
//! drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/scaffold_safety_corpus/` and runs it against the
//! scaffold-safety beta projection owned by
//! `aureline-workspace::scaffold`. Every positive drill MUST parse, project,
//! and satisfy the per-drill projection expectations encoded in the corpus
//! manifest — provider / signature / generation identity, declared
//! hook / network / registry / remote-image / dependency side effects,
//! create-empty / set-up-later handoffs and the rollback boundary, the
//! honesty labels a surface renders, the seven typed guardrails, the
//! disclosure verdict, and the reconstructable generated-project lineage.
//! Every negative drill MUST FAIL projection with an error whose message
//! contains `expected_failure_substring`, so undeclared-hook execution,
//! smuggled "declared" tasks, and sibling descriptor / plan binding stay
//! rejected before beta creation claims harden.
//!
//! The corpus additionally pins, as **failing** positive drills, the three
//! conditions the acceptance criteria require the corpus to catch rather than
//! tolerate: a plan that writes before review, a side effect that is not
//! declared before execution, and a run whose authoritative result is a
//! hidden project database. Each pins the individual guardrail predicate that
//! must flip to `false`.
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
