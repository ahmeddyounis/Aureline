//! Read-only Project Doctor alpha probes and support/export projections.
//!
//! This crate owns the first executable Project Doctor alpha lane. It consumes
//! typed, redaction-safe evidence records from the entry, execution-context,
//! search/index, trust, Git, provider/auth, and restore surfaces and emits
//! stable findings with evidence refs, confidence, and exact recovery or
//! escalation paths.
//!
//! The [`stabilize_project_doctor_probes_finding_codes_explainability_and`]
//! module promotes the beta probe-pack catalog to a stable lane with
//! explainability factors, formal unsupported-state reporting, and
//! chain-of-custody support packets.

#![doc(html_root_url = "https://docs.rs/aureline-doctor/0.0.0")]

pub mod finalize_diagnosis_and_evidence_packets_for_wrong_target;
pub mod finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos;
pub mod probe_packs;
pub mod probes;
pub mod stabilize_project_doctor_probes_finding_codes_explainability_and;
