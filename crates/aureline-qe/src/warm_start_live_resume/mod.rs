//! Warm-start, prebuild, and live-resume conformance, interoperability,
//! operations / deployment, failure / recovery, and design-QA drill harness.
//!
//! The harness loads a drill corpus pinned at
//! `fixtures/workspace/m3/warm_start_and_live_resume/` and runs it against the
//! warm-start choice boundary owned by `aureline-shell`
//! ([`validate_warm_start_choice_card`]). Every positive drill loads one
//! [`WarmStartChoiceCard`], requires it to be contract-valid, re-pins the
//! cross-cutting warm-start guarantees, and matches the per-drill expectations
//! encoded in the corpus manifest — the source / support / runtime classes, the
//! offered lanes and their availability, the snapshot freshness / age /
//! invalidation facts, the environment-starter setup location, and the honesty
//! marker. Every negative drill applies a typed tamper to a contract-valid base
//! card and requires the warm-start contract to reject it with a finding
//! containing the recorded substring, so a warm-start path that presents a stale
//! snapshot as a live resume, masquerades a networked lane as a local open, drops
//! a same-weight escape hatch, lets the default widen trust, or hides a managed
//! attach stays rejected before a beta warm-start row hardens.
//!
//! Surfaces that need to reuse the matrix outside the test runner can call
//! [`load_corpus`] + [`run_corpus`] directly; the [`CorpusReport`] returned by
//! [`run_corpus`] is the canonical pass / fail truth.
//!
//! [`validate_warm_start_choice_card`]: aureline_shell::start_center::warm_start_choice::validate_warm_start_choice_card
//! [`WarmStartChoiceCard`]: aureline_shell::start_center::warm_start_choice::WarmStartChoiceCard

mod manifest;
mod runner;

pub use manifest::{
    CorpusManifest, LaneAvailabilityExpect, NegativeDrillSpec, PositiveDrillSpec, PositiveExpect,
    Tamper, CORPUS_DIR_REL, MANIFEST_FILE_NAME,
};
pub use runner::{
    corpus_dir_from_repo_root, load_corpus, run_corpus, run_corpus_from_repo_root, CorpusReport,
    DrillFailureReason, DrillOutcome, DrillReport,
};
