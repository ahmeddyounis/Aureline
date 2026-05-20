//! Command-truth and palette-authority conformance / interoperability harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/commands/m3/command_truth_and_authority/` and runs it against the
//! command-authority boundary owned by `aureline-commands`
//! ([`CommandAuthorityScenarioRecord`]). Every positive
//! `command_authority_scenario` drill MUST parse, validate, project, and satisfy
//! the per-drill expectations encoded in the corpus manifest — the canonical
//! command id, lifecycle state, preview/approval posture, the agreed enablement
//! decision across surfaces, the covered invocation surfaces (menu/button,
//! keybinding, palette, CLI/headless, AI, recipe, voice, browser companion), the
//! honest automation labels, lineage completeness, and rollback requirement.
//! Every negative drill MUST FAIL validation with an error whose message contains
//! `expected_failure_substring`, so a surface that widens authority, suppresses a
//! preview or approval requirement, lies about its automation labels, breaks
//! alias canonicalization, or drops an invocation-lineage join stays rejected
//! before a beta command row hardens.
//!
//! Surfaces that need the matrix outside the test runner can call [`load_corpus`]
//! + [`run_corpus`] directly; the [`CorpusReport`] returned by [`run_corpus`] is
//!   the canonical pass / fail truth.
//!
//! [`CommandAuthorityScenarioRecord`]: aureline_commands::CommandAuthorityScenarioRecord

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
