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
//!
//! The
//! [`freeze_the_m5_project_doctor_guided_repair_and_container_or_devcontainer_maturity_matrix`]
//! module owns the canonical M5 Doctor/repair/container maturity matrix. It
//! certifies, for every blocked-user recovery capability and every deployment
//! profile, whether the cell carries a current feature scorecard,
//! diagnosis-latency corpus, compatibility story, and rollback path of its own,
//! and runs a non-inheriting promotion gate that narrows any stale,
//! engine-unavailable, latency-breached, or evidence-missing cell before
//! publication. Because each row's published maturity and narrowing action are
//! validated against the recomputed gate decision, release/public-truth surfaces
//! can prove underqualified cells narrow automatically instead of inheriting
//! trust from an adjacent cell.
//!
//! The
//! [`extend_project_doctor_probes_finding_codes_and_unsupported_state_reporting_across_feature_lanes`]
//! module extends Project Doctor to the M5 feature lanes — notebook kernels,
//! request/API auth and environments, database targets, profiler/replay
//! instrumentation, remote preview routes, sync/offboarding/device registry,
//! companion handoff, and incident packets. It pins one read-only probe family
//! per lane, reuses one finding schema and one human-readable vocabulary across
//! desktop, CLI/headless, and support surfaces, and keeps unsupported, partial,
//! stale, policy-blocked, and target-mismatch states reported explicitly instead
//! of collapsed into generic "unavailable" copy.

#![doc(html_root_url = "https://docs.rs/aureline-doctor/0.0.0")]

pub mod extend_project_doctor_probes_finding_codes_and_unsupported_state_reporting_across_feature_lanes;
pub mod finalize_diagnosis_and_evidence_packets_for_wrong_target;
pub mod finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos;
pub mod freeze_the_m5_project_doctor_guided_repair_and_container_or_devcontainer_maturity_matrix;
pub mod probe_packs;
pub mod probes;
pub mod stabilize_project_doctor_probes_finding_codes_explainability_and;
