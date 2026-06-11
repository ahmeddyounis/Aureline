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
//!
//! The
//! [`ship_project_doctor_explainability_panes_evidence_refs_and_cross_surface_parity`]
//! module turns each finding into an inspectable explainability pane that exposes
//! the finding code, the probe id and version, the evidence refs, the affected
//! scope, and — the central addition — *why a candidate repair is or is not
//! available*. Each pane pins a canonical CLI exit class derived from its
//! diagnosis state and the locale-invariant machine-meaning keys, so the desktop
//! pane, CLI/headless rows, support exports, incident packets, and public-truth
//! surfaces carry the same finding identity, repair-availability reason, and exit
//! semantics without localized copy changing machine meaning.
//!
//! The
//! [`m5_diagnosis_latency_recovery_ladders_and_support_parity`] module adds the
//! M5 field-readiness and supportability lane. It owns one checked-in packet of
//! seeded blocked-user scenarios — one per M5 recovery lane — that each pin the
//! initiating findings, the chosen blocked-user recovery-ladder rung (safe mode,
//! quarantine, open-without-restore, cache/index repair, restricted reopen, or a
//! typed repair), a per-percentile first-actionable diagnosis-latency budget and
//! its observed corpus measurements, corpus freshness, and a metadata-safe
//! support-bundle/escalation linkage that preserves finding ids, repair ids,
//! scope, and durable evidence without overcapturing content. A non-inheriting
//! promotion gate validates every scenario's published promotion action and
//! narrowing reason against the decision recomputed from its own drill outcome,
//! freshness, p90 latency state, and escalation completeness, so a stale corpus,
//! a breached latency budget, missing evidence, or an unhanded-off drill narrows
//! or blocks that scenario's M5 promotion automatically.
//!
//! The [`guided_repair_transaction_receipts`] module turns each guided repair
//! into an auditable repair-transaction receipt. A receipt declares the repair
//! id, initiating findings, failure family, impacted state classes,
//! preconditions, disclosed host/boundary, checkpoint (or its explicit absence),
//! verification plan, and reversal class *before mutation begins*, then records
//! the staged review/dry-run/checkpoint/apply/verify and (when needed)
//! rollback-or-compensate outcome. Its terminal completion state distinguishes
//! fixed, partially repaired, reduced-but-not-resolved, verification-inconclusive,
//! exact rollback, and compensating rollback instead of a generic
//! success/failure, and the packet enforces that durable user state is never
//! reset without a checkpoint or guarded reversal and that a missing checkpoint
//! never masquerades as easy reversibility.
//!
//! The
//! [`container_engine_summaries_open_in_container_preflights_and_route_time_bound_truth`]
//! module makes the container and devcontainer boundary legible before a user
//! reopens, rebuilds, or attaches in an M5 remote-preview or incident workflow.
//! Each scenario names the active engine class, its reachability, support class,
//! and certification note; the workspace mode, local/remote/managed boundary
//! label, and the target identity being rebuilt or attached; an open-in-container
//! preflight that discloses the definition source, rebuild-versus-reuse decision,
//! trust-gated lifecycle hooks, extension installs, published ports, writable
//! mounts, affected services/images, and a non-empty stay-local alternative; and
//! live/buffered/snapshot log truth with an export-safe time range and redaction
//! posture. A non-inheriting preflight gate routes an unreachable or
//! policy-blocked engine to a non-dead-end offer-alternative outcome, and forces
//! explicit disclosure for an unsupported engine, a trust-gated hook, or any
//! rebuild/port/mount side effect, so a window is never treated as merely "in a
//! container" and trust-gated hooks never run silently.

#![doc(html_root_url = "https://docs.rs/aureline-doctor/0.0.0")]

pub mod container_engine_summaries_open_in_container_preflights_and_route_time_bound_truth;
pub mod extend_project_doctor_probes_finding_codes_and_unsupported_state_reporting_across_feature_lanes;
pub mod finalize_diagnosis_and_evidence_packets_for_wrong_target;
pub mod finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos;
pub mod freeze_the_m5_project_doctor_guided_repair_and_container_or_devcontainer_maturity_matrix;
pub mod guided_repair_transaction_receipts;
pub mod m5_diagnosis_latency_recovery_ladders_and_support_parity;
pub mod probe_packs;
pub mod probes;
pub mod ship_project_doctor_explainability_panes_evidence_refs_and_cross_surface_parity;
pub mod stabilize_project_doctor_probes_finding_codes_explainability_and;
