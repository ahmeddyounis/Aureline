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
//! for the M3 repo-topology evidence lane, and
//! `bootstrap_truth::run_corpus_from_repo_root` for the M3
//! repository-acquisition and post-open bootstrap truth lane.

#![doc(html_root_url = "https://docs.rs/aureline-qe/0.0.0")]

pub mod bootstrap_truth;
pub mod git_history_rewrite;
pub mod repo_topology;
