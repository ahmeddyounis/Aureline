//! Runtime and toolchain detectors that feed execution-context truth.
//!
//! Detector modules in this namespace are read-only: they inspect declared
//! workspace state and caller-provided ambient facts, then emit provenance
//! records for the execution-context resolver and inspector surfaces.

pub mod node;
pub mod python;
