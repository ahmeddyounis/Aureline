//! Aureline quality-engineering conformance harnesses.
//!
//! This crate hosts reusable conformance and failure / recovery drill
//! harnesses for risky Aureline beta surfaces. Each subcontract loads a
//! drill corpus that pins the truth a beta-claimed surface must keep,
//! exposes a typed runner that compares observed projections against
//! per-drill expectations, and reports a structured pass / fail summary
//! that other harnesses (UI checks, support-export parity reviews) can
//! quote without re-parsing the corpus.
//!
//! See `git_history_rewrite::run_corpus_from_repo_root` for the
//! risky-Git lane entry point, `repo_topology::run_corpus_from_repo_root`
//! for the M3 repo-topology evidence lane,
//! `bootstrap_truth::run_corpus_from_repo_root` for the M3
//! repository-acquisition and post-open bootstrap truth lane,
//! `scaffold_safety::run_corpus_from_repo_root` for the M3 scaffold and
//! generated-project safety lane, `docs_maintenance::run_corpus_from_repo_root`
//! for the M3 docs preview / maintenance integrity lane, and
//! `portable_state_restore::run_corpus_from_repo_root` for the M3 portable-state
//! and restore-provenance continuity lane.

#![doc(html_root_url = "https://docs.rs/aureline-qe/0.0.0")]

pub mod bootstrap_truth;
pub mod docs_maintenance;
pub mod git_history_rewrite;
pub mod portable_state_restore;
pub mod repo_topology;
pub mod scaffold_safety;
