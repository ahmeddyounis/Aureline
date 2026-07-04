//! Execution-context object model and resolver seed.
//!
//! This crate owns inspectable execution-context and task-event runtime
//! contracts. It exposes one [`execution_context::ExecutionContext`] object, a
//! small [`execution_context::ExecutionContextResolver`] that mints contexts
//! for launch-capable surfaces, a canonical [`tasks::TaskEventStream`] for
//! task/test/debug lifecycle truth, and [`tests::TestAttemptAlphaPacket`] for
//! launch-wedge test identity, session, attempt, watch, and imported-CI truth.
//! [`managed_alpha::ManagedWorkspaceAlphaRecord`] adds the bounded
//! managed-workspace suspend/resume/reattach inspection lane. Downstream event
//! and export lanes carry
//! [`provenance::ExecutionEventProvenance`] so target truth survives after the
//! live run surface is gone.
//!
//! Surfaces (terminal pane, task seed, debug-prep seed, provider/auth entry
//! points, activity center, status bar, support / export flows) read structured
//! execution-context records through this crate; they do not derive runtime
//! truth from terminal state alone or fork local copies of host / target /
//! toolchain identity.
//!
//! [`testing_triage::TestTrustPacket`] composes the beta test-runner and
//! test-quality projections into release-visible watch, flaky, snapshot, and
//! quarantine debt summaries without re-parsing raw runner output.
//!
//! [`session_plans_attempt_records_and_execution_lineage::SessionAttemptLedgerPacket`]
//! makes execution attributable: it lands canonical session plans and an
//! append-only attempt-record history with per-attempt runtime / toolchain / env
//! lineage, so local reruns, notebook-backed tests, remote targets, and imported
//! CI joins normalize onto one ledger without an imported verdict ever reading as
//! a local rerun.
//!
//! [`stability_verdicts_quarantines_and_release_visibility::StabilityVerdictQuarantinePacket`]
//! converts those repeated attempt outcomes into governed, evidence-based stability
//! verdicts and quarantine records: a flaky state becomes a controlled badge state
//! over a visible evidence window, and a quarantine carries an owner, an expiry, a
//! restore condition, and a release-visibility class — so an expired quarantine
//! reopens its scope and fails readiness instead of silently persisting as local UI
//! state, and an imported verdict never reads as a locally verified pass.
//!
//! [`coverage_overlays_and_snapshot_golden_review::CoverageReviewPacket`] makes the
//! coverage and snapshot evidence drawn over the editor and review surfaces
//! trustworthy: coverage overlays carry a controlled provenance (verified current
//! run, imported CI artifact, cached local result, stale prior result) and explicit
//! branch-versus-line measures with changed-line emphasis, coverage merge sheets
//! disclose included / excluded runs and omitted shards / platforms instead of
//! implying complete certainty, and snapshot / golden review cards preserve artifact
//! kind, counts, baseline scope, and a raw fallback so a binary-only change is never
//! blind-accepted and an imported baseline never reads as a local accept.
//!
//! [`test_generation_suggestion_cards_and_diff_first_apply::TestGenerationProposalPacket`]
//! brings AI-assisted test generation into the same governed lane: a suggestion card
//! names its target symbols / files, the uncovered-path or bug source that motivated
//! it, its assumptions, the files it would write, and a sandbox-validation posture,
//! and binds to reopenable discovery / session / coverage evidence objects rather
//! than free-text justification. A generated test flows through the same preview /
//! diff / apply / revert pipeline as any other change — it cannot be applied without
//! an isolated sandbox pass, cannot bypass preview-first, cannot silently widen
//! beyond its evidenced scope, and an imported proposal is held read-only instead of
//! reading as a local apply.
//!
//! [`certify_test_discovery_session_watch_coverage_flaky_snapshot_evidence_quality::TestEvidenceCertificationPacket`]
//! makes those lanes release-bearing on every claimed M5 framework / notebook /
//! CI-import row: each row certifies its discovery, session, watch, coverage,
//! flaky, snapshot, and selector-portability proof against a freshness window, and a
//! row that loses current, reopenable proof auto-narrows below its claim — with a
//! recorded trigger and a precise label — rather than coasting on an adjacent green
//! row. Product, docs/help, review, support, and release-control surfaces ingest this
//! one certification instead of narrating test maturity by hand.
//!
//! [`freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::DiagnosticTruthLaneMatrixPacket`]
//! freezes one diagnostic-truth lane across every claimed M5 diagnostic-producing
//! surface (notebook, framework pack, request / data tooling, preview runtime,
//! package lane, language provider, editor-structural guard, and imported
//! scanner). Each row binds the frozen source-kind, imported-versus-live origin,
//! freshness, anchor-remap, collection-completeness, cluster-meaning, and
//! quality-session vocabularies, and a claimed row that cannot identify a source,
//! origin, proven freshness, remap state, collection completeness, or governing
//! quality session auto-downgrades below its claim with a recorded trigger and a
//! precise label — so Problems, the editor, review, CLI/headless, AI evidence, and
//! support export ingest one diagnostic identity instead of inferring
//! provider-local meanings.
//!
//! [`normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins::NormalizedDiagnosticRecordSetPacket`]
//! takes the next step from surface matrix to per-finding truth: it normalizes
//! each M5 finding onto the single canonical
//! [`diagnostics::DiagnosticRecord`] and proves the record-level guarantees the
//! finding surfaces depend on — a stable canonical id that survives ordinary
//! refresh, adapter refresh, and surface hops within one anchor/remap family; a
//! reopen handle for the editor, Problems, review, CLI/headless, AI evidence, and
//! support export that resolves to that same id without provider-specific
//! translation loss; and typed suppression/baseline joins kept attached to the
//! record's own refs rather than hidden in feature-local metadata. A record that
//! cannot prove its stable identity, cannot reopen from a required surface, or
//! lacks normalized provenance auto-downgrades below its claim with a recorded
//! trigger and a precise label.
//!
//! [`m5_diagnostic_source_descriptors_and_collection_snapshots::DiagnosticSourceAndCollectionPacket`]
//! answers a different honesty question: where did a finding set come from, what
//! scope was actually analyzed, and what was omitted or still streaming when the
//! user inspected it? It reuses the canonical
//! [`diagnostics::DiagnosticSource`] descriptor — producer identity, tool
//! version, target / environment fingerprint, confidence, raw-payload ref, and
//! imported-versus-live origin across the editor-structural, language-service,
//! build/task, runtime/test, scanner-import, policy, and heuristic families — and
//! adds a new collection-snapshot object that names the workspace / workset /
//! target scope, a completeness label, freshness, a streaming state with a
//! resumable cursor, the materialized diagnostic refs, and the omitted scopes and
//! reasons. A partial, filtered, streaming, or aborted snapshot auto-downgrades
//! below its claim rather than masquerading as a complete whole-workspace
//! enumeration, so Problems, review, saved views, CLI/headless, and support export
//! preserve source and completeness truth instead of flattening it to a generic
//! provider name.
//!
//! [`cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets::DiagnosticClusterSetPacket`]
//! adds the ergonomic view those normalized records need without losing their
//! distinct truth: a [`cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets::DiagnosticDisplayCluster`]
//! groups several findings — the same line flagged by a language service, an
//! imported scanner, and a build task — into one compact row carrying a stable
//! cluster id, a primary anchor, the contributing diagnostic refs, a typed dedupe
//! reason, aggregate counts, and a dominant display state, while keeping one
//! source-preserving detail sheet per constituent. Clustering stays a view over
//! real records: the synthetic-finding flag stays false, every constituent keeps
//! its own provenance, target / environment ref, policy state, and
//! imported-versus-live class, and Problems, review, support export, and AI
//! evidence each receive a projection that exposes the dedupe reason and full
//! membership so a user can audit why several findings were shown as one summary.
//! The validator refuses a cluster that flattens unlike sources into a synthetic
//! finding, drops a member's provenance, cannot recover a constituent, or
//! serializes a lossy support export.
//!
//! [`record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes::AnchorRemapHistorySetPacket`]
//! makes anchor drift explicit instead of silently dropping, "fixing", or
//! relabeling findings when M5 artifacts move under edits, cell changes, generator
//! churn, or imported-snapshot comparison. Where the canonical
//! [`diagnostics::DiagnosticAnchorRemap`] carries a finding's *current* anchor and
//! remap state, this lane records the append-only history that produced it: each
//! [`record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes::AnchorRemapHistoryEntry`]
//! pairs an old anchor ref with a new anchor ref, the resulting remap state
//! (`exact`, `contextual`, `stale`, `unmapped`, or `imported_static`), the typed
//! evidence basis that admitted it, a from/to revision pair, the actor/tool, and
//! the drift lane (file edit, notebook cell identity change, generated-artifact
//! churn, or imported scan/replay comparison). The history is sequence-ordered with
//! continuous revision pairs and a continuous anchor chain, and the editor,
//! Problems, review, CLI, and support surfaces each receive a projection exposing
//! the current state and full trail. The validator refuses a history that is not
//! append-only, silently repairs an anchor whose state disagrees with its evidence
//! basis, breaks the anchor or revision chain, or hides the trail from a required
//! surface.
//!
//! [`m5_quality_action_proposals_and_sessions::QualitySessionLedgerPacket`] turns
//! every mutating quality route on the claimed M5 surfaces into one inspectable
//! proposal/session ledger. Where [`quality::QualityActionProposal`] and
//! [`quality::QualitySession`] own the single preview/apply/validate/revert
//! contract, this lane proves that the format-on-type, format-on-save, manual
//! quick-fix and fix-all, headless lint autofix, review-apply baseline/suppression,
//! and imported-scan comparison routes all serialize through it: every mutating
//! action is a typed proposal inside a typed session, on-type / on-save / manual /
//! headless / review-apply / import-comparison sessions report through one typed
//! [`quality::QualitySessionOutcomeClass`] rather than divergent per-provider
//! status text, generated / lockfile / manifest / protected mutations reuse the
//! same lifecycle instead of a weaker bar, and the UI, Problems, review, CLI, and
//! support surfaces each receive a projection exposing the outcome, safety class,
//! rollback note, and validation refs. The validator refuses a ledger that omits a
//! required trigger path or action class, lets a result token diverge from its
//! typed class, grants a generated/protected mutation a weaker bar, drops a
//! mutating proposal's rollback boundary, or hides the truth from a required
//! surface.
//!
//! [`m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::DiagnosticQualityParityPacket`]
//! binds those threads into the governance state release-visible debt and
//! support/export truth depend on. A
//! [`m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::DiagnosticQualitySnapshot`]
//! captures the active quality-profile ref and fingerprint, the rule-pack / tool
//! versions in force, the recent collection ids, the suppression / baseline state
//! and release-visible debt count, the imported scanner session refs, and the last
//! save-participant outcomes, auto-downgrading below its claim when that evidence
//! cannot be proven. A
//! [`m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::DiagnosticDeltaPacket`]
//! compares an imported SARIF / scanner / CI side against a live local rerun and
//! states a compatibility verdict with explicit notes, so imported, CI, runtime,
//! and local-rerun findings can never impersonate one another and a profile /
//! rule-pack / tool / anchor mismatch blocks an exact-delta claim. The packet adds
//! a release-debt projection that keeps owner / expiry / baseline / suppression
//! truth assembled from the snapshots, and a consumer projection asserting
//! Problems, review, CLI/headless, support export, AI evidence, and release debt
//! all reference the same manifests. The validator refuses a packet that flattens
//! unlike sources, renders imported evidence as live truth, lets a non-compatible
//! delta omit its note or its two sides impersonate one another, drops release-debt
//! truth, or fails to downgrade a snapshot whose evidence does not back its claim.
//!
//! [`certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth::DiagnosticTruthCertificationPacket`]
//! is the capstone gate over those lanes: it makes the normalized
//! record/source/collection/remap/session model **release-bearing** per claimed M5
//! diagnostic row — notebook, framework, request/data, preview/runtime, package,
//! imported-scanner, and review/support/CLI. Each row certifies its record-identity,
//! source-descriptor, collection-snapshot, anchor-remap, and (when it owns a
//! mutating fix route) quality-session proof against a freshness window, naming a
//! proof currency and a reopenable proof ref per dimension. A row that loses current
//! proof — or leans on imported proof to back a local claim — auto-narrows to an
//! effective grade strictly below its claim, with a recorded trigger and a precise
//! label, and the narrowed set is published as a waiver-and-downgrade log. The
//! validator refuses a packet that erases a finding's source kind, lets an imported
//! scanner row read as a live local rerun, hides a partial/streaming collection,
//! silently repairs an anchor instead of recording append-only remap evidence, or
//! lets a mutating route bypass the typed quality-action lifecycle — so the editor,
//! Problems, review, CLI/headless, support export, AI evidence, and release-visible
//! debt ingest one certification instead of narrating diagnostic maturity by hand.
//!
//! [`m5_problem_records_source_task_correlation_and_rerun_jump_parity::M5ProblemRecordSetPacket`]
//! takes the causality matrix down to the individual Problems row. Where the
//! [`m5_execution_evidence_causality_matrix`] froze one lane per surface family,
//! this lane freezes one [`m5_problem_records_source_task_correlation_and_rerun_jump_parity::ProblemRecord`]
//! per run-derived finding: stable id, source tool/run refs, file/span anchor,
//! structured-versus-heuristic parse class, confidence tier, and raw-output
//! backlink, correlated to its editor decoration, timeline entry, source task, and
//! owning output channel. The record re-derives an effective status and a
//! per-action availability for the three canonical actions (jump to source, open
//! owning output, rerun or inspect the originating task/session) so a stale,
//! superseded, downgraded, or lineage-broken finding stays visibly classified and
//! reopenable rather than reading as a clean, fully actionable row — and an
//! authority-gated or imported-read-only rerun is surfaced honestly rather than
//! silently dropped. It reuses the matrix's frozen problem-source, confidence,
//! freshness, origin, channel, and proof vocabularies instead of forking a private
//! bottom-panel truth model.
//!
//! [`m5_execution_evidence_projection_overlays::M5ExecutionEvidenceProjectionSetPacket`]
//! takes the same causal chain out to the **projected overlay**: a coverage gutter,
//! a flaky-test history strip, a perf-regression note, a notebook-output verdict, a
//! pipeline annotation, or a review-side marker re-rendered away from the run that
//! produced it. Each [`m5_execution_evidence_projection_overlays::ExecutionEvidenceProjection`]
//! binds its overlay to the original run/step/provider/artifact lineage, the
//! revision-remap quality that maps origin anchors onto the current revision/cursor,
//! the freshness/stale/superseded state, and the reopen-to-origin target, then
//! re-derives an effective claim per rendering surface so old evidence shown on a
//! fresh surface can never quietly read as current truth, and a rendering surface can
//! never render wider than the projection's effective claim. It reuses the matrix's
//! frozen origin, confidence, freshness, reopen, and proof vocabularies.
//!
//! [`m5_task_problem_output_chronology_reuse::M5ChronologyReuseSetPacket`] closes the
//! loop by taking the same causal chain into the **timeline**: one durable
//! run-lifecycle event — start, progress, retry, cancel, failure, or completion —
//! written once and *reused* across the activity center, the history/timeline, an
//! exported issue packet, a support bundle, and an AI-evidence packet rather than each
//! surface re-summarising what ran. Each
//! [`m5_task_problem_output_chronology_reuse::ChronologyEntry`] binds its
//! actor/action/object/outcome grammar to the canonical task/run/channel/problem
//! objects, the provider/adapter and target scope it ran against, and its retry
//! lineage, then re-derives an effective claim per reuse surface so a failure shown in
//! the activity center, a support bundle, and an AI-evidence packet resolves to one
//! canonical run/channel/problem id, exported packets stay reviewable without the
//! originating UI state, and an imported/remote/pipeline chronology never reads as live
//! local authority. It reuses the matrix's frozen origin, confidence, freshness,
//! reopen, and proof vocabularies.
//!
//! [`m5_output_channel_virtualization_trust_and_freshness::M5OutputChannelSetPacket`]
//! takes the same causal chain down to the **individual output channel**: a raw log
//! stream, a trusted structured report, an HTML report bundle, a generated artifact, or
//! a trace/profile output. Each
//! [`m5_output_channel_virtualization_trust_and_freshness::OutputChannelRecord`] binds
//! its channel to the original run/step/provider/artifact lineage, the stream-first
//! virtualization profile that keeps a large log searchable and exportable without full
//! materialization, the content trust class and pin/export controls that keep
//! safe-preview distinct from active/open-in-external content, and the
//! live/cached/stale freshness with fetched-at and provider-unreachable cues. It then
//! re-derives an effective claim per rendering surface so a large log never forces full
//! materialization into shell memory, a user can always tell raw / safe-preview /
//! trusted-structured / untrusted-active content apart before copying, exporting, or
//! opening, and a provider-backed channel can never masquerade as live after a freshness
//! threshold or a lost connection. It reuses the matrix's frozen origin, channel,
//! confidence, freshness, reopen, and proof vocabularies.
//!
//! [`m5_structured_versus_heuristic_fallback_drills::M5FallbackEvidenceDrillSetPacket`]
//! turns the same causal chain into a **proof corpus and failure-drill suite**: one
//! parse-evidence case — a native structured diagnostic, a normalized task event, an
//! imported provider annotation, or a heuristic text parse — exercised through a failure
//! drill (malformed output, stale run, superseded retry, reconnect, lost channel, partial
//! export, imported evidence, or output-channel virtualization) and rendered onto the
//! claimed M5 tooling profiles. Each
//! [`m5_structured_versus_heuristic_fallback_drills::FallbackDrillCase`] re-derives an
//! effective claim so a heuristic fallback always reads visibly distinct from
//! native/structured evidence on every profile, the confidence label and raw-output
//! backlink survive malformed output and reconnect-heavy workflows, stale/superseded
//! state and imported overlays never read as fresh local truth, and a failure in causal
//! linking or confidence labeling automatically narrows the affected profile claims. It
//! reuses the matrix's frozen origin, problem-source, channel, confidence, freshness,
//! reopen, and proof vocabularies.
//!
//! [`certify_m5_problems_output_and_execution_evidence_truth::ProblemsOutputEvidenceCertificationPacket`]
//! is the **capstone qualification gate** that binds those Problems/output/evidence
//! lanes into the M5 promotion model. It publishes one
//! [`certify_m5_problems_output_and_execution_evidence_truth::ProfileQualification`]
//! per claimed M5 tooling profile, graded across the seven causal-chain dimensions —
//! Problems correlation, output-channel identity, evidence-projection lineage,
//! causal-link integrity, confidence honesty, stale/superseded handling, and
//! reopen-to-origin parity. A profile keeps its claim only while every dimension's
//! invariant holds and its proof is current and reopenable: a broken invariant,
//! missing proof, or imported-on-local proof auto-narrows it to `blocked`, honestly
//! labeled stale proof narrows it to `retest_pending`, and a read-only overlay holds
//! at `limited` — each with a recorded trigger and a precise narrowed label. The
//! packet also carries explicit release-evidence rows for the four release-bearing
//! integrity axes, and asserts About/help, service-health, compatibility, release,
//! support, and AI surfaces ingest one qualification state instead of restating
//! tooling claims by hand.
//!
//! The reviewer-facing landing page is
//! [`/docs/runtime/execution_context_seed.md`](../../../docs/runtime/execution_context_seed.md).
//! The cross-tool boundary schema is
//! [`/schemas/runtime/execution_context.schema.json`](../../../schemas/runtime/execution_context.schema.json).

#![doc(html_root_url = "https://docs.rs/aureline-runtime/0.0.0")]

pub mod add_shared_task_test_request_database_notebook_preview_ai_publish_and_support_execution_lifecycle_component_consumers;
pub mod browser_runtime_inspection_qualification;
pub mod build_intelligence;
pub mod build_test_event_interoperability;
pub mod capability_negotiation;
pub mod capsule_resolver;
pub mod certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth;
pub mod certify_m5_problems_output_and_execution_evidence_truth;
pub mod certify_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_truth_across_claimed_execution_surfaces;
pub mod certify_test_discovery_session_watch_coverage_flaky_snapshot_evidence_quality;
pub mod cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets;
pub mod coverage_overlays_and_snapshot_golden_review;
pub mod debug;
pub mod dependencies;
pub mod detectors;
pub mod diagnostics;
pub mod discovery;
pub mod drift_repair;
pub mod dry_run_explain;
pub mod durable_test_items_and_partial_discovery;
pub mod env_inspect;
pub mod execution_context;
pub mod finalize_environment_and_toolchain_manager_parity_across_ui;
pub mod finalize_request_workspace_and_api_request_execution_context;
pub mod freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix;
pub mod freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix;
pub mod freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix;
pub mod harden_breakpoint_call_stack_variables_watch_evaluate_and;
pub mod harden_build_target_discovery_adapter_confidence_labels_and;
pub mod harden_coverage_flaky_test_snapshot_golden_and_baseline;
pub mod harden_environment_capsule_resolution;
pub mod harden_the_stable_profiler_and_tracing_hooks_needed;
pub mod host_boundary;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_execution_lifecycle_auto_narrowing;
pub mod implement_the_m5_debug_session_header_thread_process_tree_and_dump_crash_artifact_card_primitive;
pub mod implement_the_m5_input_request_prompt_and_artifact_publish_row_primitive;
pub mod implement_the_m5_rerun_comparison_sheet_and_retry_scope_review_primitive;
pub mod implement_the_m5_run_attempt_header_and_attempt_selector_primitive;
pub mod language_hosts;
pub mod launch_profiles;
pub mod log_metric_slice_and_incident_timeline_contract;
pub mod m5_adapter_confidence_labels;
pub mod m5_adapter_hierarchy_negotiation;
pub mod m5_automation_certification;
pub mod m5_automation_contract_baseline;
pub mod m5_cross_surface_event_reuse;
pub mod m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas;
pub mod m5_diagnostic_source_descriptors_and_collection_snapshots;
pub mod m5_environment_status_strips;
pub mod m5_event_interop_certification;
pub mod m5_execution_evidence_causality_matrix;
pub mod m5_execution_evidence_projection_overlays;
pub mod m5_interop_conformance;
pub mod m5_label_parity;
pub mod m5_output_channel_virtualization_trust_and_freshness;
pub mod m5_problem_records_source_task_correlation_and_rerun_jump_parity;
pub mod m5_quality_action_proposals_and_sessions;
pub mod m5_replay_bundles;
pub mod m5_structured_versus_heuristic_fallback_drills;
pub mod m5_task_event_adapter_policy;
pub mod m5_task_event_envelope_bus;
pub mod m5_task_problem_output_chronology_reuse;
pub mod macro_sessions;
pub mod managed_alpha;
pub mod managed_workspace_lifecycle_beta;
pub mod materialize_artifact_family_quality_governance;
pub mod normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins;
pub mod packages;
pub mod parameter_review;
pub mod preview_drift;
pub mod profiler_trace_replay_regression_qualification;
pub mod provenance;
pub mod publish_execution_plane_certification_packets_for_local_remote;
pub mod quality;
pub mod queue_governor_and_admission_control;
pub mod queue_session_terminal_governance;
pub mod recipe_builder;
pub mod recipes;
pub mod record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes;
pub mod remote_helper_skew_beta;
pub mod request_workspace;
pub mod request_workspace_contracts;
pub mod rerun;
pub mod resource_governor;
pub mod run_history;
pub mod run_lineage;
pub mod runtime_continuity_surface_qualification;
pub mod sandbox;
pub mod scanner_import;
pub mod scanner_import_quality_parity;
pub mod scope_compatible_selection_objects_and_widened_selection_review;
pub mod session_plans_attempt_records_and_execution_lineage;
pub mod shared_debug_alpha;
pub mod shared_terminal_alpha;
pub mod stability_verdicts_quarantines_and_release_visibility;
pub mod stabilize_debugger_host_and_adapter_negotiation;
pub mod stabilize_execution_context_resolver;
pub mod stabilize_problem_records_output_channels_and_execution_evidence;
pub mod stabilize_task_discovery_launch_profiles_rerun_last_behavior;
pub mod stabilize_the_artifact_manager_preview_runtime_inspectors_and;
pub mod stabilize_the_test_explorer_inline_results_watch_mode;
pub mod support_matrix_beta;
pub mod target_discovery;
pub mod targets;
pub mod task_events;
pub mod tasks;
pub mod test_generation_suggestion_cards_and_diff_first_apply;
pub mod testing;
pub mod testing_identity;
pub mod testing_quality;
pub mod testing_triage;
pub mod tests;
pub mod topology_inspector;
pub mod trace_replay_alpha;

pub use browser_runtime_inspection_qualification::{
    current_stable_browser_runtime_inspection_qualification_input,
    current_stable_browser_runtime_inspection_qualification_packet,
    AttachProtocolState as BrowserRuntimeAttachProtocolState, BrowserRuntimeDowngradeRuleClass,
    BrowserRuntimeEvidenceClass, BrowserRuntimeFindingKind, BrowserRuntimeFindingSeverity,
    BrowserRuntimeInspectionQualificationArtifactError,
    BrowserRuntimeInspectionQualificationFinding, BrowserRuntimeInspectionQualificationPacket,
    BrowserRuntimeInspectionQualificationPacketInput, BrowserRuntimeInspectionQualificationRow,
    BrowserRuntimeInspectionQualificationSupportExport, BrowserRuntimePromotionState,
    BrowserRuntimeQualificationRowClass, BrowserRuntimeSupportClass, BrowserRuntimeTargetKind,
    InspectionDataState as BrowserRuntimeInspectionDataState,
    RuntimeInspectionConsumerSurface as BrowserRuntimeInspectionConsumerSurface,
    RuntimeMutationActionClass as BrowserRuntimeMutationActionClass,
    RuntimeObjectClass as BrowserRuntimeObjectClass,
    SessionFreshnessState as BrowserRuntimeSessionFreshnessState,
    SourceMapQualityState as BrowserRuntimeSourceMapQualityState,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_ARTIFACT_DOC_REF,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_DOC_REF,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_FIXTURE_DIR,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_RECORD_KIND,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_REF,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_VERSION,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use build_intelligence::{
    current_stable_adapter_confidence_support_export, AdapterHealthReason, AdapterHealthState,
    AdapterHealthStrip, AdapterIdentity, ArtifactSourceClass, BuildIntelligenceAction,
    BuildIntelligenceActionClass, BuildIntelligenceCoverageManifest, BuildIntelligenceLaneType,
    BuildIntelligenceReceipt, BuildIntelligenceRunConfigCard, BuildIntelligenceSupportExport,
    BuildIntelligenceTargetRow, DiscoveryDiffChangeClass, DiscoveryDiffItem, DiscoveryDiffReview,
    HighTrustActionPosture, ImportedLiveState, RefreshLineage, TargetExactnessStatus,
    ADAPTER_CONFIDENCE_TOOLING_ARTIFACT_DOC_REF, ADAPTER_CONFIDENCE_TOOLING_DOC_REF,
    ADAPTER_CONFIDENCE_TOOLING_FIXTURE_DIR, ADAPTER_CONFIDENCE_TOOLING_SCHEMA_REF,
    ADAPTER_HEALTH_STRIP_RECORD_KIND, BUILD_INTELLIGENCE_COVERAGE_MANIFEST_RECORD_KIND,
    BUILD_INTELLIGENCE_RECEIPT_RECORD_KIND, BUILD_INTELLIGENCE_RUN_CONFIG_CARD_RECORD_KIND,
    BUILD_INTELLIGENCE_SCHEMA_VERSION, BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND,
    BUILD_INTELLIGENCE_TARGET_ROW_RECORD_KIND, DISCOVERY_DIFF_REVIEW_RECORD_KIND,
};
pub use build_test_event_interoperability::{
    current_stable_build_test_event_interoperability_input,
    current_stable_build_test_event_interoperability_packet, AdapterCapabilityNegotiation,
    AdapterCapabilityState as BuildTestAdapterCapabilityState, BuildTestConsumerProjection,
    BuildTestConsumerSurface, BuildTestEventConfidence as BuildTestInteropConfidence,
    BuildTestEventEnvelope, BuildTestEventInteroperabilityPacket,
    BuildTestEventInteroperabilityPacketInput, BuildTestEventInteroperabilitySupportExport,
    BuildTestEventKind as BuildTestInteropEventKind, BuildTestEventProvenance,
    BuildTestEventSourceKind as BuildTestInteropSourceKind, BuildTestInteropFindingKind,
    BuildTestInteropFindingSeverity, BuildTestInteropLane, BuildTestInteropPromotionState,
    BuildTestInteropValidationFinding, BuildTestPayloadKind as BuildTestInteropPayloadKind,
    RawPayloadReference, RawPayloadRetentionClass, ReplayExportParity,
    BUILD_TEST_EVENT_INTEROPERABILITY_ARTIFACT_DOC_REF, BUILD_TEST_EVENT_INTEROPERABILITY_DOC_REF,
    BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR,
    BUILD_TEST_EVENT_INTEROPERABILITY_PACKET_ARTIFACT_REF,
    BUILD_TEST_EVENT_INTEROPERABILITY_RECORD_KIND, BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_REF,
    BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_VERSION,
    BUILD_TEST_EVENT_INTEROPERABILITY_SUPPORT_EXPORT_RECORD_KIND,
};
pub use capability_negotiation::{
    CapabilityEffectClass, CapabilityNegotiationParseError, CapabilityRequirementClass,
    CompatibilityWindow, CompatibilityWindowStatus, DroppedHelperCapability,
    EffectiveCapabilityPosture, HelperCapabilityRequest, HelperCapabilityRequirement,
    HelperCapabilityResponse, MissingCapabilityReasonClass, NegotiationOutcome,
    HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
};
pub use capsule_resolver::beta::{
    evaluate_capsule_drift, CapsuleBetaDriftOutcome, CapsuleBetaDriftRow, CapsuleBetaParsedFields,
    CapsuleBetaPrecedenceRow, CapsuleBetaSourceBaseline, CapsuleBetaSourceClass,
    CapsuleBetaSourceConfidence, CapsuleBetaSourceCoverageRow, CapsuleBetaSourceNote,
    CapsuleBetaSourceParse, ComposeParsedFields, DevcontainerParsedFields,
    EnvironmentCapsuleBetaCoverageManifest, EnvironmentCapsuleBetaDriftEvaluation,
    EnvironmentCapsuleBetaResolution, EnvironmentCapsuleBetaResolver,
    EnvironmentCapsuleBetaResolverConfig, EnvironmentCapsuleBetaSupportExport, NixParsedFields,
    NodeParsedFields, PythonParsedFields, ENVIRONMENT_CAPSULE_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    ENVIRONMENT_CAPSULE_BETA_DRIFT_RECORD_KIND, ENVIRONMENT_CAPSULE_BETA_RESOLUTION_RECORD_KIND,
    ENVIRONMENT_CAPSULE_BETA_RESOLVER_VERSION, ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
    ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use capsule_resolver::{
    EnvironmentCapsuleHint, EnvironmentCapsuleResolution, EnvironmentCapsuleResolver,
    EnvironmentCapsuleResolverConfig, PrebuildFingerprintStub, ProjectArchetypeHint,
    ENVIRONMENT_CAPSULE_RESOLUTION_RECORD_KIND, ENVIRONMENT_CAPSULE_RESOLUTION_SCHEMA_VERSION,
    ENVIRONMENT_CAPSULE_RESOLVER_VERSION, PREBUILD_FINGERPRINT_STUB_RECORD_KIND,
};
pub use debug::{
    DapHostSupervisor, DapHostSupervisorConfig, DapHostSupervisorError,
    DebugAdapterCapabilityClass, DebugAdapterCapabilityRequest, DebugAdapterCapabilityResponse,
    DebugAdapterIdentity, DebugAdapterNegotiationInput, DebugAdapterNegotiationOutcome,
    DebugAdapterTransportClass, DebugSessionEventClass, DebugSessionExitReasonClass,
    DebugSessionIdentity, DebugSessionLaunchSpec, DebugSessionLifecycleEvent, DebugSessionMode,
    DebugSessionRestartCause, DebugSessionSnapshot, DebugSessionStateClass,
    DebugSessionSupportPacket, DebugSessionTargetIdentity, DEBUG_SESSION_EVENT_RECORD_KIND,
    DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION, DEBUG_SESSION_RECORD_KIND,
    DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND,
};
pub use dependencies::{
    manifest_delta_token, validation_task_tokens, AdvisoryAffectedRange, AdvisoryLifecycleClass,
    AdvisorySeverityClass, AdvisorySourceClass, AdvisoryTruthClass, DebtReleaseVisibilityClass,
    DependencyAdvisoryRecord, DependencyAdvisoryRecordSeed, DependencyDebtKindClass,
    DependencyDebtPacket, DependencyDebtPacketSeed, DependencyDebtRow, DependencyEdgeRecord,
    DependencyFreshnessClass, DependencyGraphRecord, DependencyIntelligenceViolation,
    DependencyProvenanceClass, DependencyRecord, DependencyRecordSeed, DependencyRelationshipClass,
    DependencyResolutionClass, DependencySourceClass, LicenseDecisionClass,
    LockfileMutationPreview, LockfilePreviewActionClass, LockfilePreviewOutcomeClass,
    SuppressionRef, SuppressionStateClass, DEPENDENCY_ADVISORY_RECORD_KIND,
    DEPENDENCY_DEBT_PACKET_RECORD_KIND, DEPENDENCY_GRAPH_RECORD_KIND,
    DEPENDENCY_INTELLIGENCE_REVIEWER_VERSION, DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
    DEPENDENCY_RECORD_KIND, LOCKFILE_MUTATION_PREVIEW_RECORD_KIND,
};
pub use detectors::node::{
    NodePackageManagerKind, NodePackageManagerRequirement, NodePackageManagerResolution,
    NodeRuntimeResolution, NodeToolchainAmbiguity, NodeToolchainDetection, NodeToolchainDetector,
    NodeToolchainDetectorConfig, NodeToolchainFallbackPath, NodeToolchainProvenanceCard,
    NodeToolchainProvenanceDisposition, NodeToolchainResolutionState, NodeToolchainSourceKind,
    NodeToolchainSubject, NODE_TOOLCHAIN_DETECTION_RECORD_KIND,
    NODE_TOOLCHAIN_DETECTION_SCHEMA_VERSION, NODE_TOOLCHAIN_DETECTOR_VERSION,
};
pub use detectors::python::{
    PythonEnvironmentAmbiguity, PythonEnvironmentDetection, PythonEnvironmentDetector,
    PythonEnvironmentDetectorConfig, PythonEnvironmentFallbackPath, PythonEnvironmentManagerKind,
    PythonEnvironmentManagerRequirement, PythonEnvironmentManagerResolution,
    PythonEnvironmentProvenanceCard, PythonEnvironmentProvenanceDisposition,
    PythonEnvironmentResolutionState, PythonEnvironmentSourceKind, PythonEnvironmentSubject,
    PythonInterpreterResolution, PYTHON_ENVIRONMENT_DETECTION_RECORD_KIND,
    PYTHON_ENVIRONMENT_DETECTION_SCHEMA_VERSION, PYTHON_ENVIRONMENT_DETECTOR_VERSION,
};
pub use discovery::package_scripts::{
    PackageScriptBlockReason, PackageScriptDescriptor, PackageScriptDiscoverer,
    PackageScriptDiscovererConfig, PackageScriptDiscovery, PackageScriptDiscoveryState,
    PackageScriptDispatch, PackageScriptLaunchReadiness, PackageScriptLifecycleHook,
    PackageScriptMissingRuntimeState, PackageScriptRerunLineage, PackageScriptRerunMode,
    PackageScriptRunContract, PackageScriptRunner, PackageScriptRuntimeStatus,
    PackageScriptShellMode, PackageScriptSource, PackageScriptSourceKind,
    PackageScriptWarningClass, PACKAGE_SCRIPT_DISCOVERER_VERSION,
    PACKAGE_SCRIPT_DISCOVERY_RECORD_KIND, PACKAGE_SCRIPT_DISCOVERY_SCHEMA_VERSION,
    PACKAGE_SCRIPT_RUN_CONTRACT_RECORD_KIND,
};
pub use discovery::pytest::{
    PytestBlockReason, PytestDiscoverer, PytestDiscovererConfig, PytestDiscovery,
    PytestDiscoveryIssue, PytestDiscoveryIssueKind, PytestDiscoveryState, PytestDispatch,
    PytestInvocationMode, PytestLaunchReadiness, PytestMissingRuntimeState, PytestRerunLineage,
    PytestRerunMode, PytestRunContract, PytestRunSelection, PytestRunner, PytestRuntimeStatus,
    PytestSelectionKind, PytestSourceKind, PytestTestDescriptor, PytestTestFileDescriptor,
    PytestTestKind, PytestWarningClass, PYTEST_DISCOVERER_VERSION, PYTEST_DISCOVERY_RECORD_KIND,
    PYTEST_DISCOVERY_SCHEMA_VERSION, PYTEST_RUN_CONTRACT_RECORD_KIND,
};
pub use discovery::toolchains::{
    ToolchainDetectionEntry, ToolchainDetectionEvidence, ToolchainDetectionSourceKind,
    ToolchainPresenceState, WorkspaceToolchainDetector, WorkspaceToolchainDetectorConfig,
    WorkspaceToolchainDiscovery, WorkspaceToolchainKind, WORKSPACE_TOOLCHAIN_DETECTOR_VERSION,
    WORKSPACE_TOOLCHAIN_DISCOVERY_RECORD_KIND, WORKSPACE_TOOLCHAIN_DISCOVERY_SCHEMA_VERSION,
};
pub use drift_repair::{
    DriftReasonClass, DriftRepairAction, DriftRepairActionClass, DriftRepairAuthorityImpactClass,
    RemoteDriftRepairDiagnosticsPacket, RemoteDriftRepairGuidance,
    REMOTE_DRIFT_REPAIR_BETA_DIAGNOSTICS_PACKET_RECORD_KIND,
    REMOTE_DRIFT_REPAIR_BETA_GUIDANCE_RECORD_KIND, REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION,
};
pub use dry_run_explain::{
    canonical_reused_contract_refs as dry_run_explain_reused_contract_refs,
    current_dry_run_explain_first_consumers_input, seeded_blocked_preview,
    seeded_consumer_preview as seeded_dry_run_explain_consumer_preview,
    seeded_dry_run_explain_export_roundtrip, seeded_dry_run_explain_first_consumers_packet,
    validate_dry_run_explain_first_consumers_packet, ArtifactDestination, ArtifactDestinationClass,
    BlockerClass, DryRunExplainConsumerBinding, DryRunExplainError, DryRunExplainExport,
    DryRunExplainFinding, DryRunExplainFindingKind, DryRunExplainFindingSeverity,
    DryRunExplainFirstConsumersCliHeadlessView, DryRunExplainFirstConsumersInput,
    DryRunExplainFirstConsumersPacket, DryRunExplainFirstConsumersSupportExport,
    DryRunExplainInvariantsBlock, DryRunExplainPreview, DryRunExplainSupportActionRow,
    DryRunExplainSupportConsumerRow, DryRunPreviewRunHistoryRow, IdempotenceClass, PredictedWrite,
    PreviewedAction, SideEffectClass as DryRunSideEffectClass, TrustPolicyBlocker, WriteKind,
    APPROVAL_POSTURE_NONE, APPROVAL_POSTURE_REQUIRED, DRY_RUN_EXPLAIN_DOC_REF,
    DRY_RUN_EXPLAIN_EXPORT_RECORD_KIND, DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND, DRY_RUN_EXPLAIN_FIRST_CONSUMERS_ID,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_RECORD_KIND, DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_REF,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SCHEMA_VERSION,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND, DRY_RUN_EXPLAIN_FIXTURE_DIR,
    DRY_RUN_EXPLAIN_PACKET_RECORD_KIND, DRY_RUN_EXPLAIN_PACKET_SCHEMA_REF,
    DRY_RUN_PREVIEW_RUN_HISTORY_ROW_RECORD_KIND, PREVIEW_POSTURE_NO_SAFE_PREVIEW,
    PREVIEW_POSTURE_SUPPORTED,
};
pub use env_inspect::{
    seeded_env_inspect_resolver, seeded_env_inspect_snapshot, seeded_env_inspect_support_export,
    EnvInspectCoreField, EnvInspectDegradationLabel, EnvInspectDegradationSeverity,
    EnvInspectRedactionClass, EnvInspectSection, EnvInspectSeededScenario, EnvInspectSnapshot,
    EnvInspectSupportExport, ENV_INSPECT_SCHEMA_VERSION, ENV_INSPECT_SNAPSHOT_RECORD_KIND,
    ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use execution_context::beta::{
    evaluate_ticket_drift, lane_for_context, lane_for_target_class,
    ExecutionContextBetaCoverageManifest, ExecutionContextBetaLane,
    ExecutionContextBetaLaneCoverageRow, ExecutionContextBetaLaneSample,
    ExecutionContextBetaSupportExport, TicketDriftBinding, TicketDriftEvaluation, TicketDriftField,
    TicketDriftOutcome, TicketDriftRow, EXECUTION_CONTEXT_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    EXECUTION_CONTEXT_BETA_SCHEMA_VERSION, EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND,
    EXECUTION_CONTEXT_TICKET_DRIFT_RECORD_KIND,
};
pub use execution_context::{
    ActorClass, CacheDisposition, CapsuleDriftState, ConfidenceLevel, DegradedFieldReason,
    DegradedFieldRecord, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextEffectClass,
    ExecutionContextExplanation, ExecutionContextReasonCode, ExecutionContextReasonSource,
    ExecutionContextRequest, ExecutionContextResolver, ExecutionContextResolverConfig,
    ExecutionRouteClass, ExecutionRouteOrigin, IdentityMode, InvocationSubject, MixedVersionDrift,
    MixedVersionDriftState, MixedVersionReason, PolicyAndTrust, PrebuildInvalidationReason,
    PrebuildMetadata, PrebuildReuseState, Provenance, ReachabilityState, ResolverInputDecision,
    ResolverInputField, ResolverInputSource, ScopeClass, SurfaceClass, TargetClass,
    TargetConfidence, TargetConfidenceReason, TargetIdentity, ToolchainClass, ToolchainIdentity,
    TrustState, EXECUTION_CONTEXT_RECORD_KIND, EXECUTION_CONTEXT_SCHEMA_VERSION,
};
pub use finalize_environment_and_toolchain_manager_parity_across_ui::{
    current_stable_inspector_parity_truth_packet, InspectorFieldClass,
    InspectorParityConfidenceClass, InspectorParityConsumerProjection,
    InspectorParityConsumerSurface, InspectorParityDowngradeAutomationClass,
    InspectorParityEvidenceClass, InspectorParityFindingKind, InspectorParityFindingSeverity,
    InspectorParityKnownLimitClass, InspectorParityLaneClass, InspectorParityPromotionState,
    InspectorParityRow, InspectorParityRowClass, InspectorParitySupportClass,
    InspectorParityTruthArtifactError, InspectorParityTruthPacket, InspectorParityTruthPacketInput,
    InspectorParityTruthSupportExport, InspectorParityValidationFinding, ParitySurfaceClass,
    RecoveryStateClass, INSPECTOR_PARITY_TRUTH_ARTIFACT_DOC_REF, INSPECTOR_PARITY_TRUTH_DOC_REF,
    INSPECTOR_PARITY_TRUTH_FIXTURE_DIR, INSPECTOR_PARITY_TRUTH_PACKET_ARTIFACT_REF,
    INSPECTOR_PARITY_TRUTH_PACKET_RECORD_KIND, INSPECTOR_PARITY_TRUTH_SCHEMA_REF,
    INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION, INSPECTOR_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use finalize_request_workspace_and_api_request_execution_context::{
    current_stable_request_execution_context_truth_packet,
    AuthSourceModeClass as RequestExecutionAuthSourceModeClass,
    ConfidenceClass as RequestExecutionConfidenceClass,
    ConnectionStateClass as RequestExecutionConnectionStateClass,
    ConsumerProjection as RequestExecutionConsumerProjection,
    ConsumerProjectionSurface as RequestExecutionConsumerProjectionSurface,
    ConsumerSurfaceClass as RequestExecutionConsumerSurfaceClass,
    DowngradeAutomationClass as RequestExecutionDowngradeAutomationClass,
    EvidenceClass as RequestExecutionEvidenceClass, FindingKind as RequestExecutionFindingKind,
    FindingSeverity as RequestExecutionFindingSeverity,
    KnownLimitClass as RequestExecutionKnownLimitClass,
    PromotionState as RequestExecutionPromotionState, RequestExecutionContextRow,
    RequestExecutionContextTruthArtifactError, RequestExecutionContextTruthPacket,
    RequestExecutionContextTruthPacketInput, RequestExecutionContextTruthSupportExport,
    RequestExecutionLaneClass, RequestExecutionRowClass, RequestExecutionSupportClass,
    StreamingResponseStateClass as RequestExecutionStreamingResponseStateClass,
    ValidationFinding as RequestExecutionValidationFinding,
    WedgeClass as RequestExecutionWedgeClass, REQUEST_EXECUTION_CONTEXT_TRUTH_ARTIFACT_DOC_REF,
    REQUEST_EXECUTION_CONTEXT_TRUTH_DOC_REF, REQUEST_EXECUTION_CONTEXT_TRUTH_FIXTURE_DIR,
    REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_ARTIFACT_REF,
    REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_RECORD_KIND, REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_REF,
    REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_VERSION,
    REQUEST_EXECUTION_CONTEXT_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_breakpoint_call_stack_variables_watch_evaluate_and::{
    current_stable_debug_fidelity_truth_packet, ConsumerSurface as DebugFidelityConsumerSurface,
    DebugFidelityConfidenceClass, DebugFidelityConsumerProjection, DebugFidelityLaneClass,
    DebugFidelityRow, DebugFidelityRowClass, DebugFidelityTruthArtifactError,
    DebugFidelityTruthPacket, DebugFidelityTruthPacketInput, DebugFidelityTruthSupportExport,
    DowngradeAutomationClass as DebugFidelityDowngradeAutomationClass,
    EvidenceClass as DebugFidelityEvidenceClass, FindingKind as DebugFidelityFindingKind,
    FindingSeverity as DebugFidelityFindingSeverity,
    InspectorStateClass as DebugFidelityInspectorStateClass,
    InspectorSurfaceClass as DebugFidelityInspectorSurfaceClass,
    KnownLimitClass as DebugFidelityKnownLimitClass,
    MappingFidelityBadgeClass as DebugFidelityMappingFidelityBadgeClass,
    PromotionState as DebugFidelityPromotionState, SupportClass as DebugFidelitySupportClass,
    ValidationFinding as DebugFidelityValidationFinding, WedgeClass as DebugFidelityWedgeClass,
    DEBUG_FIDELITY_TRUTH_ARTIFACT_DOC_REF, DEBUG_FIDELITY_TRUTH_DOC_REF,
    DEBUG_FIDELITY_TRUTH_FIXTURE_DIR, DEBUG_FIDELITY_TRUTH_PACKET_ARTIFACT_REF,
    DEBUG_FIDELITY_TRUTH_PACKET_RECORD_KIND, DEBUG_FIDELITY_TRUTH_SCHEMA_REF,
    DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION, DEBUG_FIDELITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_build_target_discovery_adapter_confidence_labels_and::{
    current_stable_build_target_hardening_truth_packet,
    AdapterConfidenceLabelClass as BuildTargetHardeningAdapterConfidenceLabelClass,
    BuildTargetHardeningLaneClass, BuildTargetHardeningRow, BuildTargetHardeningRowClass,
    BuildTargetHardeningSupportClass, BuildTargetHardeningTruthArtifactError,
    BuildTargetHardeningTruthPacket, BuildTargetHardeningTruthPacketInput,
    BuildTargetHardeningTruthSupportExport, ConfidenceClass as BuildTargetHardeningConfidenceClass,
    ConsumerProjection as BuildTargetHardeningConsumerProjection,
    ConsumerProjectionSurface as BuildTargetHardeningConsumerProjectionSurface,
    ConsumerSurfaceClass as BuildTargetHardeningConsumerSurfaceClass,
    DiscoveryFreshnessClass as BuildTargetHardeningDiscoveryFreshnessClass,
    DiscoverySourceClass as BuildTargetHardeningDiscoverySourceClass,
    DowngradeAutomationClass as BuildTargetHardeningDowngradeAutomationClass,
    EvidenceClass as BuildTargetHardeningEvidenceClass,
    FindingKind as BuildTargetHardeningFindingKind,
    FindingSeverity as BuildTargetHardeningFindingSeverity,
    KnownLimitClass as BuildTargetHardeningKnownLimitClass,
    PromotionState as BuildTargetHardeningPromotionState,
    TargetGraphSnapshotClass as BuildTargetHardeningTargetGraphSnapshotClass,
    ValidationFinding as BuildTargetHardeningValidationFinding,
    WedgeClass as BuildTargetHardeningWedgeClass, BUILD_TARGET_HARDENING_TRUTH_ARTIFACT_DOC_REF,
    BUILD_TARGET_HARDENING_TRUTH_DOC_REF, BUILD_TARGET_HARDENING_TRUTH_FIXTURE_DIR,
    BUILD_TARGET_HARDENING_TRUTH_PACKET_ARTIFACT_REF,
    BUILD_TARGET_HARDENING_TRUTH_PACKET_RECORD_KIND, BUILD_TARGET_HARDENING_TRUTH_SCHEMA_REF,
    BUILD_TARGET_HARDENING_TRUTH_SCHEMA_VERSION,
    BUILD_TARGET_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_coverage_flaky_test_snapshot_golden_and_baseline::{
    current_stable_coverage_quality_truth_packet,
    CandidateLineageClass as CoverageQualityCandidateLineageClass,
    ConsumerSurface as CoverageQualityConsumerSurface,
    ConsumerSurfaceBindingClass as CoverageQualityConsumerSurfaceBindingClass,
    CoverageImpactClass as CoverageQualityCoverageImpactClass, CoverageQualityConfidenceClass,
    CoverageQualityConsumerProjection, CoverageQualityLaneClass, CoverageQualityRow,
    CoverageQualityRowClass, CoverageQualityTruthArtifactError, CoverageQualityTruthPacket,
    CoverageQualityTruthPacketInput, CoverageQualityTruthSupportExport,
    DowngradeAutomationClass as CoverageQualityDowngradeAutomationClass,
    EvidenceClass as CoverageQualityEvidenceClass, FindingKind as CoverageQualityFindingKind,
    FindingSeverity as CoverageQualityFindingSeverity,
    KnownLimitClass as CoverageQualityKnownLimitClass,
    PromotionState as CoverageQualityPromotionState,
    QuarantineMuteStateClass as CoverageQualityQuarantineMuteStateClass,
    StabilityVerdictClass as CoverageQualityStabilityVerdictClass,
    SupportClass as CoverageQualitySupportClass, TestSourceClass as CoverageQualityTestSourceClass,
    ValidationFinding as CoverageQualityValidationFinding, WedgeClass as CoverageQualityWedgeClass,
    COVERAGE_QUALITY_TRUTH_ARTIFACT_DOC_REF, COVERAGE_QUALITY_TRUTH_DOC_REF,
    COVERAGE_QUALITY_TRUTH_FIXTURE_DIR, COVERAGE_QUALITY_TRUTH_PACKET_ARTIFACT_REF,
    COVERAGE_QUALITY_TRUTH_PACKET_RECORD_KIND, COVERAGE_QUALITY_TRUTH_SCHEMA_REF,
    COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION, COVERAGE_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_environment_capsule_resolution::{
    current_stable_capsule_resolution_truth_packet, CapsuleFieldClass,
    CapsuleResolutionConfidenceClass, CapsuleResolutionConsumerProjection,
    CapsuleResolutionConsumerSurface, CapsuleResolutionDowngradeAutomationClass,
    CapsuleResolutionEvidenceClass, CapsuleResolutionFindingKind, CapsuleResolutionFindingSeverity,
    CapsuleResolutionKnownLimitClass, CapsuleResolutionLaneClass, CapsuleResolutionPromotionState,
    CapsuleResolutionRow, CapsuleResolutionRowClass, CapsuleResolutionSupportClass,
    CapsuleResolutionTruthArtifactError, CapsuleResolutionTruthPacket,
    CapsuleResolutionTruthPacketInput, CapsuleResolutionTruthSupportExport,
    CapsuleResolutionValidationFinding, InvalidationReasonClass, PrebuildFingerprintComponentClass,
    ProjectDoctorFindingClass, CAPSULE_RESOLUTION_TRUTH_ARTIFACT_DOC_REF,
    CAPSULE_RESOLUTION_TRUTH_DOC_REF, CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR,
    CAPSULE_RESOLUTION_TRUTH_PACKET_ARTIFACT_REF, CAPSULE_RESOLUTION_TRUTH_PACKET_RECORD_KIND,
    CAPSULE_RESOLUTION_TRUTH_SCHEMA_REF, CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION,
    CAPSULE_RESOLUTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_the_stable_profiler_and_tracing_hooks_needed::{
    current_stable_profiler_truth_packet, BuildModeClass as ProfilerBuildModeClass,
    CaptureStateClass as ProfilerCaptureStateClass, ConfidenceClass as ProfilerConfidenceClass,
    ConfounderClass as ProfilerConfounderClass,
    ConsumerProjectionSurface as ProfilerConsumerProjectionSurface,
    DowngradeAutomationClass as ProfilerDowngradeAutomationClass,
    EvidenceClass as ProfilerEvidenceClass, FindingKind as ProfilerFindingKind,
    FindingSeverity as ProfilerFindingSeverity, KnownLimitClass as ProfilerKnownLimitClass,
    OriginClass as ProfilerOriginClass, ProfilerConsumerProjection, ProfilerLaneClass, ProfilerRow,
    ProfilerRowClass, ProfilerSurfaceClass, ProfilerTruthArtifactError, ProfilerTruthPacket,
    ProfilerTruthPacketInput, ProfilerTruthSupportExport, PromotionState as ProfilerPromotionState,
    ReplayStateClass as ProfilerReplayStateClass, RunClassClass as ProfilerRunClassClass,
    SupportClass as ProfilerSupportClass, ValidationFinding as ProfilerValidationFinding,
    WedgeClass as ProfilerWedgeClass, PROFILER_TRUTH_ARTIFACT_DOC_REF, PROFILER_TRUTH_DOC_REF,
    PROFILER_TRUTH_FIXTURE_DIR, PROFILER_TRUTH_PACKET_ARTIFACT_REF,
    PROFILER_TRUTH_PACKET_RECORD_KIND, PROFILER_TRUTH_SCHEMA_REF, PROFILER_TRUTH_SCHEMA_VERSION,
    PROFILER_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use host_boundary::{
    evaluate_host_boundary_reapproval, ActionExposureClass, ActionOriginClass, ActionRouteClass,
    ActionTargetClass, AdapterConfidenceClass, AdapterConfidencePlaceholder, AdapterKind,
    AuthorityLinkageClass, BoundaryFreshnessClass, BoundaryManagedLifecycleState,
    BoundaryReachabilityClass, BoundaryRedactionClass, DiscoveryAuthorityBlock, ExpiryReasonClass,
    ExportInclusionPosture, HostBoundaryDriftField, HostBoundaryDriftRow,
    HostBoundaryIdentityChips, HostBoundaryReapprovalEvaluation, HostBoundaryReapprovalOutcome,
    HostBoundaryReviewBinding, HostBoundarySupportExport, HostBoundarySurfaceClass,
    HostBoundarySurfaceProjection, HostBoundaryTruthOptions, HostBoundaryTruthRecord,
    HostBoundaryTruthViolation, LocalOnlyContinuationReasonClass, ManagedLifecycleTruth,
    ManagedWorkspaceReviewerLabel, ReapprovalRequirementClass, RouteChangeReasonCode,
    WrongTargetCorrectionClass, HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION,
    HOST_BOUNDARY_REAPPROVAL_EVALUATION_RECORD_KIND, HOST_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND,
    HOST_BOUNDARY_SURFACE_PROJECTION_RECORD_KIND, HOST_BOUNDARY_TRUTH_RECORD_KIND,
};
pub use language_hosts::{
    LanguageHostEventClass, LanguageHostExitReasonClass, LanguageHostIdentity,
    LanguageHostLaunchSpec, LanguageHostRuntimeStateClass, LanguageHostScopeKey,
    LanguageHostSnapshot, LanguageHostSupervisor, LanguageHostSupervisorConfig,
    LanguageHostSupervisorError, LanguageHostSupervisorEvent, LanguageHostSupportPacket,
    LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION,
};
pub use launch_profiles::{
    LaunchProfile, LaunchProfileAdapterBinding, LaunchProfileArguments, LaunchProfileCreateRequest,
    LaunchProfileDisclosureRow, LaunchProfileEdit, LaunchProfileEditClass,
    LaunchProfileEnvironmentBinding, LaunchProfileInvalidReason, LaunchProfileKind,
    LaunchProfileMode, LaunchProfileMutable, LaunchProfilePreview, LaunchProfilePreviewState,
    LaunchProfileRevision, LaunchProfileSideEffectClass, LaunchProfileStore,
    LaunchProfileStoreError, LaunchProfileSupportExport, LaunchProfileSupportRow,
    LaunchProfileTargetBinding, LAUNCH_PROFILE_EDIT_RECORD_KIND,
    LAUNCH_PROFILE_PREVIEW_RECORD_KIND, LAUNCH_PROFILE_RECORD_KIND,
    LAUNCH_PROFILE_REVISION_RECORD_KIND, LAUNCH_PROFILE_SCHEMA_VERSION,
    LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use log_metric_slice_and_incident_timeline_contract::{
    current_stable_operational_evidence_contract_input,
    current_stable_operational_evidence_contract_json,
    current_stable_operational_evidence_contract_packet, ActorLineage, EvidenceFreshnessState,
    EvidenceTimeWindow, ExportRedactionClass, IncidentTimelineEntry, OperationalEvidenceBundle,
    OperationalEvidenceConsumerProjection, OperationalEvidenceConsumerSurface,
    OperationalEvidenceContractArtifactError, OperationalEvidenceContractPacket,
    OperationalEvidenceContractPacketInput, OperationalEvidenceFindingKind,
    OperationalEvidenceFindingSeverity, OperationalEvidencePromotionState,
    OperationalEvidenceSupportClass, OperationalEvidenceSupportExport,
    OperationalEvidenceValidationFinding, RunbookActionClass, RunbookPacket, RunbookStepExecution,
    RunbookStepStatus, SamplePosture, SignalKind, SignalSlice, SignalSourceIdentity, TargetScope,
    TimelineLink, TimelineLinkClass, OPERATIONAL_EVIDENCE_CONTRACT_ARTIFACT_DOC_REF,
    OPERATIONAL_EVIDENCE_CONTRACT_DOC_REF, OPERATIONAL_EVIDENCE_CONTRACT_FIXTURE_DIR,
    OPERATIONAL_EVIDENCE_CONTRACT_RECORD_KIND, OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_REF,
    OPERATIONAL_EVIDENCE_CONTRACT_SCHEMA_VERSION, OPERATIONAL_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_adapter_confidence_labels::{
    current_stable_adapter_confidence_audit_input, seeded_adapter_confidence_audit,
    validate_adapter_confidence_audit, AdapterConfidenceAiEvidenceView, AdapterConfidenceAudit,
    AdapterConfidenceAuditInput, AdapterConfidenceAuditSupportExport,
    AdapterConfidenceCliHeadlessRow, AdapterConfidenceCliHeadlessView, AiEvidenceClaimRow,
    AiEvidenceSubjectRow, ClaimSubject, ClaimSubjectKind, ClaimSubjectResolution,
    ClaimSubjectResolutionInput, ConfidenceAuditFindingKind, ConfidenceAuditValidationFinding,
    ConfidenceClaim, ConfidenceLabel, ConfidenceLabelSurface, OverwriteDecision,
    OverwriteDecisionRow, OverwriteReason, SourceQualityChange, SurfaceLabelBinding,
    ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_ID, ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_RECORD_KIND,
    ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_ID, ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_RECORD_KIND,
    ADAPTER_CONFIDENCE_AUDIT_DOC_REF, ADAPTER_CONFIDENCE_AUDIT_ENVELOPE_SCHEMA_REF,
    ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR, ADAPTER_CONFIDENCE_AUDIT_ID,
    ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF, ADAPTER_CONFIDENCE_AUDIT_POLICY_BASELINE_REF,
    ADAPTER_CONFIDENCE_AUDIT_RECORD_KIND, ADAPTER_CONFIDENCE_AUDIT_SCHEMA_REF,
    ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION, ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_ID,
    ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_adapter_hierarchy_negotiation::{
    current_stable_adapter_hierarchy_negotiation_input, downgrade_reason_for_fallback,
    fallback_class_for, seeded_adapter_hierarchy_negotiation_baseline,
    validate_adapter_hierarchy_negotiation_baseline, AdapterCandidate, AdapterNegotiationBaseline,
    AdapterNegotiationBaselineInput, AdapterNegotiationSupportExport, CapabilityDriftSignal,
    CapabilityNegotiation, DisclosureSurface, DriftClass, Ecosystem, EcosystemAdapterResolution,
    FallbackClass, NegotiatedCapability, NegotiationDisclosureBinding, NegotiationFindingKind,
    NegotiationValidationFinding, SkipReason, SkippedAdapterReason,
    ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF, ADAPTER_NEGOTIATION_BASELINE_ID,
    ADAPTER_NEGOTIATION_DOC_REF, ADAPTER_NEGOTIATION_ENVELOPE_SCHEMA_REF,
    ADAPTER_NEGOTIATION_FIXTURE_DIR, ADAPTER_NEGOTIATION_POLICY_SCHEMA_REF,
    ADAPTER_NEGOTIATION_RECORD_KIND, ADAPTER_NEGOTIATION_SCHEMA_REF,
    ADAPTER_NEGOTIATION_SCHEMA_VERSION, ADAPTER_NEGOTIATION_SUPPORT_EXPORT_ID,
    ADAPTER_NEGOTIATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_automation_certification::{
    current_stable_automation_certification_input, seeded_automation_certification_packet,
    validate_automation_certification_packet, AutomationAuthoringPath,
    AutomationCertificationCliHeadlessView, AutomationCertificationDimension,
    AutomationCertificationEvidenceJoinView, AutomationCertificationIndex,
    AutomationCertificationPacket, AutomationCertificationPacketInput,
    AutomationCertificationSupportExport,
    AutomationDimensionOutcome as AutomationCertificationDimensionOutcome, AutomationSurface,
    AutomationSurfaceCertification,
    CertificationEvidenceSurface as AutomationCertificationEvidenceSurface,
    CertificationFindingKind as AutomationCertificationFindingKind,
    CertificationFreshnessState as AutomationCertificationFreshnessState,
    CertificationValidationFinding as AutomationCertificationValidationFinding,
    SurfaceCertificationRow, SurfaceClaimState, AUTOMATION_CERTIFICATION_AI_EVIDENCE_ID,
    AUTOMATION_CERTIFICATION_CLI_HEADLESS_ID, AUTOMATION_CERTIFICATION_CLI_HEADLESS_RECORD_KIND,
    AUTOMATION_CERTIFICATION_CONTRACT_BASELINE_SCHEMA_REF, AUTOMATION_CERTIFICATION_DOC_REF,
    AUTOMATION_CERTIFICATION_EVIDENCE_JOIN_RECORD_KIND, AUTOMATION_CERTIFICATION_EVIDENCE_REFS,
    AUTOMATION_CERTIFICATION_FIXTURE_DIR, AUTOMATION_CERTIFICATION_ID,
    AUTOMATION_CERTIFICATION_INCIDENT_PACKET_ID, AUTOMATION_CERTIFICATION_INDEX_REF,
    AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF, AUTOMATION_CERTIFICATION_RECORD_KIND,
    AUTOMATION_CERTIFICATION_SCHEMA_REF, AUTOMATION_CERTIFICATION_SCHEMA_VERSION,
    AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_ID,
    AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_automation_contract_baseline::{
    canonical_safety_labels, current_automation_contract_baseline_input,
    seeded_automation_contract_baseline_packet, seeded_dry_run_explain_packet,
    seeded_macro_session_discarded, seeded_macro_session_stopped_promotable,
    seeded_parameter_review_sheet, seeded_recipe_builder_session_blocked,
    seeded_recipe_builder_session_preview_ready, validate_automation_contract_baseline_packet,
    ArgumentInspectionKind, AutomationBaselinePromotionState,
    AutomationContractBaselineCliHeadlessView, AutomationContractBaselineInput,
    AutomationContractBaselinePacket, AutomationContractBaselineSupportExport,
    AutomationObjectFamily, AutomationSafetyLabel, AutomationSafetyLabelId,
    AutomationSafetyLabelManifest, BaselineFindingKind, BaselineFindingSeverity,
    BaselineInvariantsBlock, BaselineValidationFinding, BuilderValidationFinding, ContentAddress,
    DryRunExplainPacket, DryRunOutcomeClass, DryRunStepExplanation, MacroCaptureStep,
    MacroPromotionAffordanceClass, MacroRecorderStateClass, MacroSession, ObjectFamilyBinding,
    ParameterReviewRow, ParameterReviewSheet, ParameterReviewVerdictClass, RecipeBuilderSession,
    RecipeBuilderStateClass, RecipeBuilderStepDraft, SafetyLabelKind, SupportExportFamilyRow,
    AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_ID,
    AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_RECORD_KIND, AUTOMATION_CONTRACT_BASELINE_DOC_REF,
    AUTOMATION_CONTRACT_BASELINE_FIXTURE_ROOT, AUTOMATION_CONTRACT_BASELINE_ID,
    AUTOMATION_CONTRACT_BASELINE_PACKET_ARTIFACT_REF, AUTOMATION_CONTRACT_BASELINE_RECORD_KIND,
    AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF, AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
    AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_ID,
    AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_RECORD_KIND, AUTOMATION_SAFETY_LABEL_MANIFEST_ID,
    AUTOMATION_SAFETY_LABEL_MANIFEST_RECORD_KIND, CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
    MACRO_SESSION_SCHEMA_REF, RECIPE_BUILDER_SCHEMA_REF, RECIPE_MANIFEST_SCHEMA_REF,
    RUN_HISTORY_ROW_SCHEMA_REF, RUN_RECORD_SCHEMA_REF, RUN_SUMMARY_EXPORT_SCHEMA_REF,
};
pub use m5_cross_surface_event_reuse::{
    current_stable_cross_surface_event_reuse_input, seeded_cross_surface_event_reuse_packet,
    validate_cross_surface_event_reuse_packet, ConsumerBinding, ConsumerBindingRow,
    ConsumerSurface, CrossSurfaceCliHeadlessView, CrossSurfaceEventReusePacket,
    CrossSurfaceEventReusePacketInput, CrossSurfaceEventReuseSupportExport,
    CrossSurfaceEvidenceJoinView, CrossSurfaceFindingKind, CrossSurfaceFlow, CrossSurfaceFlowKind,
    CrossSurfaceFlowRow, CrossSurfaceValidationFinding, ReuseEvidenceSurface, SharedEventRow,
    CROSS_SURFACE_EVENT_REUSE_AI_EVIDENCE_ID, CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_ID,
    CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_RECORD_KIND, CROSS_SURFACE_EVENT_REUSE_DOC_REF,
    CROSS_SURFACE_EVENT_REUSE_ENVELOPE_SCHEMA_REF,
    CROSS_SURFACE_EVENT_REUSE_EVIDENCE_JOIN_RECORD_KIND,
    CROSS_SURFACE_EVENT_REUSE_FIRST_CONSUMERS_PACKET_REF, CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR,
    CROSS_SURFACE_EVENT_REUSE_ID, CROSS_SURFACE_EVENT_REUSE_INCIDENT_PACKET_ID,
    CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF, CROSS_SURFACE_EVENT_REUSE_POLICY_BASELINE_REF,
    CROSS_SURFACE_EVENT_REUSE_RECORD_KIND, CROSS_SURFACE_EVENT_REUSE_SCHEMA_REF,
    CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION, CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_ID,
    CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_environment_status_strips::{
    current_m5_environment_status_strips, ContextFacet, ContextFreshness, ContextResolutionPath,
    ContextStatusClass, EnvironmentStatusStrip, M5EnvironmentStatusStripExportProjection,
    M5EnvironmentStatusStripExportRow, M5EnvironmentStatusStripSummary,
    M5EnvironmentStatusStripSupportExport, M5EnvironmentStatusStripViolation,
    M5EnvironmentStatusStrips, RunSurface, StripConsumerBinding, StripConsumerSurface,
    StripDowngradeReason, StripFacet, StripPresentation,
    M5_ENVIRONMENT_STATUS_STRIP_ARTIFACT_DOC_REF, M5_ENVIRONMENT_STATUS_STRIP_DOC_REF,
    M5_ENVIRONMENT_STATUS_STRIP_FIXTURE_DIR, M5_ENVIRONMENT_STATUS_STRIP_JSON,
    M5_ENVIRONMENT_STATUS_STRIP_PATH, M5_ENVIRONMENT_STATUS_STRIP_RECORD_KIND,
    M5_ENVIRONMENT_STATUS_STRIP_REVIEW_PACKET_REF, M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_REF,
    M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_VERSION,
    M5_ENVIRONMENT_STATUS_STRIP_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_event_interop_certification::{
    current_stable_event_interop_certification_input, seeded_event_interop_certification_packet,
    validate_event_interop_certification_packet, CertificationDimension,
    CertificationEvidenceSurface, CertificationFindingKind, CertificationIndex,
    CertificationValidationFinding, ConsumerTruthSource,
    DimensionOutcome as CertificationDimensionOutcome, EventInteropCertificationCliHeadlessView,
    EventInteropCertificationEvidenceJoinView, EventInteropCertificationPacket,
    EventInteropCertificationPacketInput, EventInteropCertificationSupportExport,
    EvidenceFreshnessState as CertificationEvidenceFreshnessState, ProfileCertificationRow,
    ProfileClaimState, ToolingProfile, ToolingProfileCertification,
    EVENT_INTEROP_CERTIFICATION_AI_EVIDENCE_ID, EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_ID,
    EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_RECORD_KIND,
    EVENT_INTEROP_CERTIFICATION_CONFORMANCE_PACKET_REF, EVENT_INTEROP_CERTIFICATION_DOC_REF,
    EVENT_INTEROP_CERTIFICATION_ENVELOPE_SCHEMA_REF,
    EVENT_INTEROP_CERTIFICATION_EVIDENCE_JOIN_RECORD_KIND,
    EVENT_INTEROP_CERTIFICATION_EVIDENCE_REFS, EVENT_INTEROP_CERTIFICATION_ID,
    EVENT_INTEROP_CERTIFICATION_INCIDENT_PACKET_ID, EVENT_INTEROP_CERTIFICATION_INDEX_REF,
    EVENT_INTEROP_CERTIFICATION_INTEROP_PACKET_REF,
    EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF,
    EVENT_INTEROP_CERTIFICATION_POLICY_BASELINE_REF, EVENT_INTEROP_CERTIFICATION_RECORD_KIND,
    EVENT_INTEROP_CERTIFICATION_SCHEMA_REF, EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION,
    EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_ID,
    EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_execution_evidence_causality_matrix::{
    current_m5_execution_evidence_causality_matrix, CausalChain as ExecEvidenceCausalChain,
    CausalClaim as ExecEvidenceCausalClaim, CausalityLaneRow,
    ClaimDistribution as ExecEvidenceClaimDistribution, ClaimPosture as ExecEvidenceClaimPosture,
    ConfidenceTier as ExecEvidenceConfidenceTier, ExportPacket as ExecEvidenceExportPacket,
    FreshnessState as ExecEvidenceFreshnessState, LaneCausalDecision,
    LaneIdentity as ExecEvidenceLaneIdentity, LaneVerification as ExecEvidenceLaneVerification,
    M5ExecutionEvidenceCausalityArtifactError, M5ExecutionEvidenceCausalityMatrixInput,
    M5ExecutionEvidenceCausalityMatrixPacket, M5ExecutionEvidenceCausalityViolation,
    NarrowingReason as ExecEvidenceNarrowingReason, OriginClass as ExecEvidenceOriginClass,
    OutputChannelClass as ExecEvidenceOutputChannelClass,
    ProblemSourceKind as ExecEvidenceProblemSourceKind, ProofCurrency as ExecEvidenceProofCurrency,
    ReopenTarget as ExecEvidenceReopenTarget, SurfaceFamily as ExecEvidenceSurfaceFamily,
    VerificationFreshness as ExecEvidenceVerificationFreshness,
    M5_EXECUTION_EVIDENCE_CAUSALITY_DOC_REF, M5_EXECUTION_EVIDENCE_CAUSALITY_FIXTURE_DIR,
    M5_EXECUTION_EVIDENCE_CAUSALITY_MATRIX_REF, M5_EXECUTION_EVIDENCE_CAUSALITY_RECORD_KIND,
    M5_EXECUTION_EVIDENCE_CAUSALITY_REPORT_REF, M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_REF,
    M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_VERSION,
    M5_EXECUTION_EVIDENCE_CAUSALITY_SUPPORT_EXPORT_REF,
    M5_EXECUTION_EVIDENCE_CAUSALITY_TAXONOMY_VERSION,
};
pub use m5_execution_evidence_projection_overlays::{
    current_m5_execution_evidence_projection_set, seeded_execution_evidence_projection_set,
    ExecutionEvidenceProjection, M5ExecutionEvidenceProjectionArtifactError,
    M5ExecutionEvidenceProjectionSetInput, M5ExecutionEvidenceProjectionSetPacket,
    M5ExecutionEvidenceProjectionViolation, ProjectionClaim, ProjectionClaimDistribution,
    ProjectionConfidenceTier, ProjectionDecision, ProjectionIntegrity, ProjectionKind,
    ProjectionLineage, ProjectionNarrowingReason, ProjectionSurface, ProjectionVerification,
    RemapQuality, RevisionRemap, SurfaceRendering, M5_EXECUTION_EVIDENCE_PROJECTIONS_DOC_REF,
    M5_EXECUTION_EVIDENCE_PROJECTIONS_FIXTURE_DIR, M5_EXECUTION_EVIDENCE_PROJECTIONS_PACKET_ID,
    M5_EXECUTION_EVIDENCE_PROJECTIONS_RECORD_KIND, M5_EXECUTION_EVIDENCE_PROJECTIONS_REPORT_REF,
    M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_REF, M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_VERSION,
    M5_EXECUTION_EVIDENCE_PROJECTIONS_SUPPORT_EXPORT_REF,
    M5_EXECUTION_EVIDENCE_PROJECTIONS_TAXONOMY_VERSION,
};
pub use m5_interop_conformance::{
    archetypes_for_family, current_stable_interop_conformance_input,
    seeded_interop_conformance_packet, validate_interop_conformance_packet, ConformanceCase,
    ConformanceCaseRow, ConformanceDimension, ConformanceEvidenceSurface, CorpusFamily,
    DimensionOutcome, EvidenceFreshnessState as InteropEvidenceFreshnessState, InteropArchetype,
    InteropConformanceCliHeadlessView, InteropConformanceEvidenceJoinView,
    InteropConformanceFindingKind, InteropConformancePacket, InteropConformancePacketInput,
    InteropConformanceSupportExport, InteropConformanceValidationFinding, InteropCorpus,
    InteropCorpusRow, ReleaseEvidenceBinding, INTEROP_CONFORMANCE_AI_EVIDENCE_ID,
    INTEROP_CONFORMANCE_CLI_HEADLESS_ID, INTEROP_CONFORMANCE_CLI_HEADLESS_RECORD_KIND,
    INTEROP_CONFORMANCE_DOC_REF, INTEROP_CONFORMANCE_ENVELOPE_SCHEMA_REF,
    INTEROP_CONFORMANCE_EVIDENCE_JOIN_RECORD_KIND, INTEROP_CONFORMANCE_ID,
    INTEROP_CONFORMANCE_INCIDENT_PACKET_ID, INTEROP_CONFORMANCE_INTEROP_PACKET_REF,
    INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF, INTEROP_CONFORMANCE_POLICY_BASELINE_REF,
    INTEROP_CONFORMANCE_RECORD_KIND, INTEROP_CONFORMANCE_RELEASE_EVIDENCE_REF,
    INTEROP_CONFORMANCE_SCHEMA_REF, INTEROP_CONFORMANCE_SCHEMA_VERSION,
    INTEROP_CONFORMANCE_SUPPORT_EXPORT_ID, INTEROP_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_label_parity::{
    canonical_reused_contract_refs as label_parity_reused_contract_refs,
    current_label_parity_input, seeded_command_rows as label_parity_seeded_command_rows,
    seeded_label_parity_packet, validate_label_parity_packet, CommandLabelParityRow,
    LabelParityCliHeadlessView, LabelParityFinding, LabelParityFindingKind,
    LabelParityFindingSeverity, LabelParityInput, LabelParityInvariantsBlock, LabelParityPacket,
    LabelParitySupportExport, LabelSurfaceClass, ProjectedLabel, SupportCommandRow,
    SurfaceLabelProjection, LABEL_PARITY_CLI_HEADLESS_ID, LABEL_PARITY_CLI_HEADLESS_RECORD_KIND,
    LABEL_PARITY_DOC_REF, LABEL_PARITY_FIXTURE_DIR, LABEL_PARITY_ID,
    LABEL_PARITY_PACKET_ARTIFACT_REF, LABEL_PARITY_RECORD_KIND, LABEL_PARITY_SCHEMA_REF,
    LABEL_PARITY_SCHEMA_VERSION, LABEL_PARITY_SUPPORT_EXPORT_ID,
    LABEL_PARITY_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_output_channel_virtualization_trust_and_freshness::{
    current_m5_output_channel_set, seeded_m5_output_channel_set, ChannelAccessControls,
    ChannelClaim, ChannelClaimDistribution, ChannelDecision, ChannelFreshness, ChannelIntegrity,
    ChannelLineage, ChannelNarrowingReason, ChannelPayloadKind, ChannelRendering, ChannelSurface,
    ChannelVerification, ContentTrustClass, M5OutputChannelArtifactError, M5OutputChannelSetInput,
    M5OutputChannelSetPacket, M5OutputChannelViolation, OutputChannelRecord, VirtualizationProfile,
    LARGE_CHANNEL_CHUNK_THRESHOLD, M5_OUTPUT_CHANNELS_DOC_REF, M5_OUTPUT_CHANNELS_FIXTURE_DIR,
    M5_OUTPUT_CHANNELS_PACKET_ID, M5_OUTPUT_CHANNELS_RECORD_KIND, M5_OUTPUT_CHANNELS_REPORT_REF,
    M5_OUTPUT_CHANNELS_SCHEMA_REF, M5_OUTPUT_CHANNELS_SCHEMA_VERSION,
    M5_OUTPUT_CHANNELS_SUPPORT_EXPORT_REF, M5_OUTPUT_CHANNELS_TAXONOMY_VERSION,
};
pub use m5_problem_records_source_task_correlation_and_rerun_jump_parity::{
    current_m5_problem_record_set, seeded_problem_record_set, ActionAvailability,
    ActionAvailabilitySet, FileSpanAnchor, M5ProblemRecordSetInput, M5ProblemRecordSetPacket,
    M5ProblemRecordsArtifactError, M5ProblemRecordsViolation, ProblemAction, ProblemCorrelations,
    ProblemDowngradeReason, ProblemEvidence, ProblemRecord, ProblemRecordDecision,
    ProblemRecordStatus, ProblemSeverity as M5ProblemSeverity, ProblemSourceRefs, RerunAuthority,
    StatusDistribution as ProblemRecordStatusDistribution, M5_PROBLEM_RECORDS_DOC_REF,
    M5_PROBLEM_RECORDS_FIXTURE_DIR, M5_PROBLEM_RECORDS_PACKET_ID, M5_PROBLEM_RECORDS_RECORD_KIND,
    M5_PROBLEM_RECORDS_REPORT_REF, M5_PROBLEM_RECORDS_SCHEMA_REF,
    M5_PROBLEM_RECORDS_SCHEMA_VERSION, M5_PROBLEM_RECORDS_SUPPORT_EXPORT_REF,
    M5_PROBLEM_RECORDS_TAXONOMY_VERSION,
};
pub use m5_quality_action_proposals_and_sessions::{
    current_m5_quality_session_ledger_export, QualityActionConsumerProjection,
    QualityActionCoverage, QualityActionGuardrails, QualityActionViolation,
    QualitySessionExportRow, QualitySessionLedgerArtifactError, QualitySessionLedgerPacket,
    QualitySessionLedgerPacketInput, QualitySessionLedgerSupportExport,
    QualitySessionSurfaceProjection, M5_QUALITY_SESSION_LEDGER_ARTIFACT_REF,
    M5_QUALITY_SESSION_LEDGER_DOC_REF, M5_QUALITY_SESSION_LEDGER_RECORD_KIND,
    M5_QUALITY_SESSION_LEDGER_SCHEMA_REF, M5_QUALITY_SESSION_LEDGER_SCHEMA_VERSION,
    M5_QUALITY_SESSION_LEDGER_SUMMARY_REF, M5_QUALITY_SESSION_LEDGER_SUPPORT_EXPORT_RECORD_KIND,
    M5_QUALITY_SESSION_SURFACE_PROJECTION_RECORD_KIND, QUALITY_ACTION_PROPOSAL_SCHEMA_REF,
    QUALITY_SESSION_SCHEMA_REF, REQUIRED_ACTION_CLASSES, REQUIRED_TRIGGER_PATHS,
};
pub use m5_replay_bundles::{
    current_stable_replay_bundle_input, retention_ai_evidence_safe, retention_byte_bound,
    retention_replay_safe, retention_support_export_safe, seeded_replay_bundle,
    validate_replay_bundle, LineageJoinProjection, NormalizedReplayRow, RawLineageEvidenceRow,
    RawPayloadLineageEntry, ReplayBundle, ReplayBundleCliHeadlessRow, ReplayBundleCliHeadlessView,
    ReplayBundleFindingKind, ReplayBundleInput, ReplayBundleSupportExport,
    ReplayBundleValidationFinding, ReplayEvidenceJoinView, ReplayFailureMode, ReplayJoinSurface,
    ReplayRecoveryPosture, ReplayRobustnessCase, REPLAY_BUNDLE_AI_EVIDENCE_ID,
    REPLAY_BUNDLE_CLI_HEADLESS_ID, REPLAY_BUNDLE_CLI_HEADLESS_RECORD_KIND, REPLAY_BUNDLE_DOC_REF,
    REPLAY_BUNDLE_ENVELOPE_SCHEMA_REF, REPLAY_BUNDLE_EVIDENCE_JOIN_RECORD_KIND,
    REPLAY_BUNDLE_FIRST_CONSUMERS_PACKET_REF, REPLAY_BUNDLE_FIXTURE_DIR, REPLAY_BUNDLE_ID,
    REPLAY_BUNDLE_INCIDENT_PACKET_ID, REPLAY_BUNDLE_PACKET_ARTIFACT_REF,
    REPLAY_BUNDLE_POLICY_BASELINE_REF, REPLAY_BUNDLE_RECORD_KIND, REPLAY_BUNDLE_SCHEMA_REF,
    REPLAY_BUNDLE_SCHEMA_VERSION, REPLAY_BUNDLE_SUPPORT_EXPORT_ID,
    REPLAY_BUNDLE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_structured_versus_heuristic_fallback_drills::{
    current_m5_fallback_evidence_drill_set, seeded_fallback_evidence_drill_set,
    ChannelVirtualization, FallbackClaim, FallbackClaimDistribution, FallbackDecision,
    FallbackDrillCase, FallbackDrillKind, FallbackIntegrity, FallbackLinks,
    FallbackNarrowingReason, FallbackVerification, M5FallbackEvidenceDrillArtifactError,
    M5FallbackEvidenceDrillSetInput, M5FallbackEvidenceDrillSetPacket,
    M5FallbackEvidenceDrillViolation, ProfileBinding, ToolingProfile as FallbackToolingProfile,
    M5_FALLBACK_EVIDENCE_DRILL_DOC_REF, M5_FALLBACK_EVIDENCE_DRILL_FIXTURE_DIR,
    M5_FALLBACK_EVIDENCE_DRILL_PACKET_ID, M5_FALLBACK_EVIDENCE_DRILL_RECORD_KIND,
    M5_FALLBACK_EVIDENCE_DRILL_REPORT_REF, M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_REF,
    M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_VERSION, M5_FALLBACK_EVIDENCE_DRILL_SUPPORT_EXPORT_REF,
    M5_FALLBACK_EVIDENCE_DRILL_TAXONOMY_VERSION,
};
pub use m5_task_event_adapter_policy::{
    canonical_confidence_ceiling, canonical_priority_rank,
    current_stable_task_event_adapter_policy_input, seeded_task_event_adapter_policy_baseline,
    source_is_authoritative, validate_task_event_adapter_policy_baseline, AdapterArbitrationRow,
    AdapterPriorityRung, DowngradeReason, DowngradeVocabularyEntry, PolicyFindingKind,
    PolicyValidationFinding, RawPayloadRetentionCell, TaskEventAdapterPolicyBaseline,
    TaskEventAdapterPolicyBaselineInput, TaskEventAdapterPolicySupportExport, TaskEventConsumer,
    TaskEventConsumerBinding, TaskEventEnvelope, TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF,
    TASK_EVENT_ADAPTER_POLICY_BASELINE_ID, TASK_EVENT_ADAPTER_POLICY_CAPABILITY_SCHEMA_REF,
    TASK_EVENT_ADAPTER_POLICY_DOC_REF, TASK_EVENT_ADAPTER_POLICY_ENVELOPE_SCHEMA_REF,
    TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR, TASK_EVENT_ADAPTER_POLICY_RECORD_KIND,
    TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION, TASK_EVENT_ADAPTER_POLICY_SEED_CONTRACT_REF,
    TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_ID,
    TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_task_event_envelope_bus::{
    canonical_payload_kind, current_stable_task_event_first_consumers_input,
    seeded_task_event_first_consumers_packet, validate_task_event_first_consumers_packet,
    EventBusFindingKind, EventBusValidationFinding, TaskEventCliHeadlessRow,
    TaskEventCliHeadlessView, TaskEventFirstConsumersPacket, TaskEventFirstConsumersPacketInput,
    TaskEventFirstConsumersSupportExport, TaskEventRecord, TaskEventSurface,
    TaskEventSurfaceProjection, TaskEventTraceSummary, TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND, TASK_EVENT_FIRST_CONSUMERS_DOC_REF,
    TASK_EVENT_FIRST_CONSUMERS_ENVELOPE_SCHEMA_REF, TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR,
    TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF, TASK_EVENT_FIRST_CONSUMERS_PACKET_ID,
    TASK_EVENT_FIRST_CONSUMERS_POLICY_BASELINE_REF, TASK_EVENT_FIRST_CONSUMERS_RECORD_KIND,
    TASK_EVENT_FIRST_CONSUMERS_SCHEMA_REF, TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION,
    TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
    TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_task_problem_output_chronology_reuse::{
    current_m5_chronology_reuse_set, seeded_chronology_reuse_set, ChronologyActorAction,
    ChronologyActorKind, ChronologyClaim, ChronologyClaimDistribution, ChronologyDecision,
    ChronologyEntry, ChronologyIntegrity, ChronologyLinks, ChronologyNarrowingReason,
    ChronologyObjectKind, ChronologyOutcome, ChronologyPhase, ChronologyScope, ChronologySurface,
    ChronologySurfaceBinding, ChronologyVerification, M5ChronologyReuseArtifactError,
    M5ChronologyReuseSetInput, M5ChronologyReuseSetPacket, M5ChronologyReuseViolation,
    RetryLineage, M5_CHRONOLOGY_REUSE_DOC_REF, M5_CHRONOLOGY_REUSE_FIXTURE_DIR,
    M5_CHRONOLOGY_REUSE_PACKET_ID, M5_CHRONOLOGY_REUSE_RECORD_KIND, M5_CHRONOLOGY_REUSE_REPORT_REF,
    M5_CHRONOLOGY_REUSE_SCHEMA_REF, M5_CHRONOLOGY_REUSE_SCHEMA_VERSION,
    M5_CHRONOLOGY_REUSE_SUPPORT_EXPORT_REF, M5_CHRONOLOGY_REUSE_TAXONOMY_VERSION,
};
pub use macro_sessions::{
    canonical_reused_contract_refs as macro_recorder_reused_contract_refs,
    current_macro_recorder_first_consumers_input,
    seeded_consumer_panel as seeded_macro_recorder_panel, seeded_cross_scope_promotion_session,
    seeded_macro_recorder_first_consumers_packet, seeded_macro_recorder_session,
    seeded_macro_session_export_roundtrip, seeded_unsupported_command_session,
    validate_macro_recorder_first_consumers_packet, ActiveRecordingStrip, CapturedCommand,
    CapturedCommandReview, CapturedCommandReviewRow, CapturedCommandSupportClass,
    MacroRecorderConsumerBinding, MacroRecorderFinding, MacroRecorderFindingKind,
    MacroRecorderFindingSeverity, MacroRecorderFirstConsumersCliHeadlessView,
    MacroRecorderFirstConsumersInput, MacroRecorderFirstConsumersPacket,
    MacroRecorderFirstConsumersSupportExport, MacroRecorderInvariantsBlock, MacroRecorderSession,
    MacroRecorderSupportConsumerRow, MacroRecorderSupportSessionRow, MacroRedactionClass,
    MacroReplayActionClass, MacroReplayBlocker, MacroReplayDisposition, MacroReplayResolution,
    MacroSessionExport, MacroStorageScopeClass, RecordedSurfaceClass, ReplayPostureClass,
    SessionDispositionClass, TargetScopeClass, UnsupportedCommandWarning, MACRO_RECORDER_DOC_REF,
    MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND, MACRO_RECORDER_FIRST_CONSUMERS_ID,
    MACRO_RECORDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF, MACRO_RECORDER_FIRST_CONSUMERS_RECORD_KIND,
    MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_REF, MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION,
    MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
    MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND, MACRO_RECORDER_FIXTURE_DIR,
    MACRO_REPLAY_RESOLUTION_RECORD_KIND, MACRO_SESSION_EXPORT_RECORD_KIND,
};
pub use managed_alpha::{
    ManagedReachabilityClass, ManagedReapprovalRequirementClass, ManagedRerunPostureClass,
    ManagedRuntimeInspectionLabel, ManagedRuntimePlacementClass, ManagedTargetFreshnessClass,
    ManagedWorkspaceAlphaRecord, ManagedWorkspaceAlphaViolation, ManagedWorkspaceBoundary,
    ManagedWorkspaceContinuity, ManagedWorkspaceInspectionSurface, ManagedWorkspaceLaneScope,
    ManagedWorkspaceLifecycleState, ManagedWorkspaceRuntimeInspection, ManagedWorkspaceStateClass,
    ManagedWorkspaceSupportExport, ManagedWorkspaceTransition, ManagedWorkspaceTransitionReason,
    MANAGED_WORKSPACE_ALPHA_LANE_ID, MANAGED_WORKSPACE_ALPHA_RECORD_KIND,
    MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION, MANAGED_WORKSPACE_RUNTIME_INSPECTION_RECORD_KIND,
    MANAGED_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use managed_workspace_lifecycle_beta::{
    ManagedLifecycleLineageEntry, ManagedLifecyclePhaseClass, ManagedLifecycleStateClass,
    ManagedLocalEditingContinuityClass, ManagedSurfaceClass, ManagedWorkspaceLifecycleBetaRecord,
    ManagedWorkspaceLifecycleBetaSupportExport, ManagedWorkspaceLifecycleBetaSurfaceProjection,
    ManagedWorkspaceLifecycleBetaViolation, MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SUPPORT_EXPORT_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SURFACE_PROJECTION_RECORD_KIND,
};
pub use packages::{
    DependencySection, LockfileAlphaRef, LockfileCouplingClass, LockfileImpactAlphaRecord,
    LockfileImpactClass, LockfileMutationMode, ManifestDeltaClass, ManifestRequirementState,
    ManifestScopeAlphaDescriptor, ManifestScopeClass, MirrorOrOfflineStateClass,
    NodePackageMutationReviewRequest, NodePackageMutationReviewer,
    NodePackageMutationReviewerConfig, PackageAuditResultClass, PackageManagerFamily,
    PackageOperationAlphaPacket, PackageOperationAlphaViolation, PackageOperationAuditLineage,
    PackageOperationAuditPacket, PackageOperationClass, PackageOperationNoHiddenMutationGuards,
    PackageOperationSupportExport, PackageOperationSupportExportRow, PackageRedactionClass,
    PackageResolverIdentity, PackageReviewOutcomeClass, RegistryAuthModeClass,
    RegistryFreshnessClass, RegistryRevocationStateClass, RegistrySourceAlphaDescriptor,
    RegistrySourceClass, RollbackCheckpointAlphaSummary, RollbackPostureClass,
    ScriptRiskAlphaDescriptor, ScriptRiskClass, TransitiveImpactClass, ValidationTaskClass,
    LOCKFILE_IMPACT_ALPHA_RECORD_KIND, MANIFEST_SCOPE_ALPHA_RECORD_KIND,
    PACKAGE_MUTATION_REVIEWER_VERSION, PACKAGE_OPERATION_ALPHA_RECORD_KIND,
    PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION, PACKAGE_OPERATION_AUDIT_RECORD_KIND,
    PACKAGE_OPERATION_SUPPORT_EXPORT_RECORD_KIND, REGISTRY_SOURCE_ALPHA_RECORD_KIND,
};
pub use parameter_review::{
    canonical_reused_contract_refs as parameter_review_reused_contract_refs,
    current_parameter_review_first_consumers_input, seeded_consumer_sheet,
    seeded_parameter_review_export_roundtrip, seeded_parameter_review_first_consumers_packet,
    seeded_secret_reference_sheet, validate_parameter_review_first_consumers_packet,
    ParameterConstraintKind, ParameterFieldType, ParameterReviewBuilder,
    ParameterReviewConsumerBinding, ParameterReviewError, ParameterReviewExport,
    ParameterReviewFinding, ParameterReviewFindingKind, ParameterReviewFindingSeverity,
    ParameterReviewFirstConsumersCliHeadlessView, ParameterReviewFirstConsumersInput,
    ParameterReviewFirstConsumersPacket, ParameterReviewFirstConsumersSupportExport,
    ParameterReviewInvariantsBlock, ParameterReviewSupportConsumerRow,
    ParameterReviewSupportParameterRow, ParameterSourceLayer, ParameterValidation,
    ParameterValueState, ReviewedParameter, SaveToScope, SecretReference, PARAMETER_REVIEW_DOC_REF,
    PARAMETER_REVIEW_EXPORT_RECORD_KIND, PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND, PARAMETER_REVIEW_FIRST_CONSUMERS_ID,
    PARAMETER_REVIEW_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    PARAMETER_REVIEW_FIRST_CONSUMERS_RECORD_KIND, PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_REF,
    PARAMETER_REVIEW_FIRST_CONSUMERS_SCHEMA_VERSION,
    PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
    PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND, PARAMETER_REVIEW_FIXTURE_DIR,
    PARAMETER_REVIEW_SHEET_RECORD_KIND, PARAMETER_REVIEW_SHEET_SCHEMA_REF,
};
pub use preview_drift::{
    evaluate_preview_commit_guard, seeded_preview_commit_guard_scenario, ApprovalTicketBinding,
    GuardedActionClass, PolicySnapshotBinding, PreviewApprovalState,
    PreviewCommitAdmissionDecision, PreviewCommitAuditEventClass, PreviewCommitBasis,
    PreviewCommitCliOutput, PreviewCommitContext, PreviewCommitGuard, PreviewCommitGuardAuditEvent,
    PreviewCommitGuardEvaluation, PreviewCommitGuardScenario, PreviewCommitGuardSupportExport,
    PreviewCommitSurfaceProjection, PreviewInvalidationReason, PreviewInvalidationRow,
    PreviewLifecycleState, PreviewRepresentationClass, PreviewScalarBinding, PreviewTargetBinding,
    PREVIEW_COMMIT_GUARD_AUDIT_EVENT_RECORD_KIND, PREVIEW_COMMIT_GUARD_EVALUATION_RECORD_KIND,
    PREVIEW_COMMIT_GUARD_RECORD_KIND, PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
    PREVIEW_COMMIT_GUARD_SUPPORT_EXPORT_RECORD_KIND, PREVIEW_COMMIT_SURFACE_PROJECTION_RECORD_KIND,
};
pub use profiler_trace_replay_regression_qualification::{
    performance_qualification_packet_from_json,
    BuildRuntimeIdentity as PerformanceBuildRuntimeIdentity, CaptureMode as PerformanceCaptureMode,
    CaptureSourceClass as PerformanceCaptureSourceClass, CaptureWindow as PerformanceCaptureWindow,
    EvidenceState as PerformanceEvidenceState,
    ExportReviewPosture as PerformanceExportReviewPosture, FindingKind as PerformanceFindingKind,
    FindingSeverity as PerformanceFindingSeverity,
    MappingQualityState as PerformanceMappingQualityState,
    MappingReferenceSet as PerformanceMappingReferenceSet, MetricFamily as PerformanceMetricFamily,
    PerformanceClaimLabel, PerformanceQualificationArtifactError, PerformanceQualificationFinding,
    PerformanceQualificationPacket, PerformanceQualificationPacketInput,
    PerformanceQualificationRow, PerformanceQualificationSupportExport, PerformanceSurfaceKind,
    ProfileSessionDescriptor, ProjectionSurface as PerformanceProjectionSurface,
    RedactionMode as PerformanceRedactionMode, RegressionComparisonPacket, RegressionConfounder,
    RegressionConfounderKind, ReplayCapabilityDescriptor as PerformanceReplayCapabilityDescriptor,
    ReplayDegradationState as PerformanceReplayDegradationState,
    ReplayFeatureState as PerformanceReplayFeatureState,
    ReplaySupportMatrix as PerformanceReplaySupportMatrix,
    RetentionClass as PerformanceRetentionClass, ReverseReplayChrome,
    SessionStrip as PerformanceSessionStrip, TargetIdentity as PerformanceTargetIdentity,
    ThresholdWaiverState as PerformanceThresholdWaiverState, TraceBundleManifest,
    PERFORMANCE_QUALIFICATION_ARTIFACT_DOC_REF, PERFORMANCE_QUALIFICATION_DOC_REF,
    PERFORMANCE_QUALIFICATION_FIXTURE_DIR, PERFORMANCE_QUALIFICATION_HELP_DOC_REF,
    PERFORMANCE_QUALIFICATION_RECORD_KIND, PERFORMANCE_QUALIFICATION_SCHEMA_REF,
    PERFORMANCE_QUALIFICATION_SCHEMA_VERSION, PERFORMANCE_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use provenance::evidence_packet::{
    seeded_runtime_evidence_packet, seeded_runtime_evidence_packet_support_export,
    ReplayCompatibilityClass, ReplayIncompatibilityReason, RuntimeEvidenceKind,
    RuntimeEvidenceLane, RuntimeEvidencePacket, RuntimeEvidencePacketSeededScenario,
    RuntimeEvidencePacketSupportExport, RuntimeEvidenceReplayComparison,
    RUNTIME_EVIDENCE_PACKET_RECORD_KIND, RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION,
    RUNTIME_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND,
    RUNTIME_EVIDENCE_REPLAY_COMPARISON_RECORD_KIND,
};
pub use provenance::{
    dedupe_context_provenance, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass, ExecutionProvenanceInputDecision,
    ExecutionProvenanceRedactionClass, EXECUTION_EVENT_PROVENANCE_RECORD_KIND,
    EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION, EXECUTION_PROVENANCE_EVENT_RECORD_KIND,
};
pub use publish_execution_plane_certification_packets_for_local_remote::{
    current_stable_execution_plane_truth_packet,
    ArtifactProvenanceStateClass as ExecutionPlaneArtifactProvenanceStateClass,
    ConfidenceClass as ExecutionPlaneConfidenceClass,
    ConsumerSurface as ExecutionPlaneConsumerSurface,
    DegradedHelperStateClass as ExecutionPlaneDegradedHelperStateClass,
    DowngradeAutomationClass as ExecutionPlaneDowngradeAutomationClass,
    EvidenceClass as ExecutionPlaneEvidenceClass, ExecutionPlaneCertificationRow,
    ExecutionPlaneConsumerProjection, ExecutionPlaneLaneClass, ExecutionPlaneRowClass,
    ExecutionPlaneTruthArtifactError, ExecutionPlaneTruthPacket, ExecutionPlaneTruthPacketInput,
    ExecutionPlaneTruthSupportExport, FindingKind as ExecutionPlaneFindingKind,
    FindingSeverity as ExecutionPlaneFindingSeverity,
    KnownLimitClass as ExecutionPlaneKnownLimitClass,
    PromotionState as ExecutionPlanePromotionState,
    ReconnectStateClass as ExecutionPlaneReconnectStateClass,
    RouteStateClass as ExecutionPlaneRouteStateClass, SupportClass as ExecutionPlaneSupportClass,
    SurfaceBindingClass as ExecutionPlaneSurfaceBindingClass,
    ValidationFinding as ExecutionPlaneValidationFinding, EXECUTION_PLANE_TRUTH_ARTIFACT_DOC_REF,
    EXECUTION_PLANE_TRUTH_DOC_REF, EXECUTION_PLANE_TRUTH_FIXTURE_DIR,
    EXECUTION_PLANE_TRUTH_PACKET_ARTIFACT_REF, EXECUTION_PLANE_TRUTH_PACKET_RECORD_KIND,
    EXECUTION_PLANE_TRUTH_SCHEMA_REF, EXECUTION_PLANE_TRUTH_SCHEMA_VERSION,
    EXECUTION_PLANE_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use quality::{
    BaselineCompatibilityStateClass, BaselineRecord, BaselineRecordRequest,
    EffectiveQualityProfile, QualityActionClass, QualityActionDisclosureClass,
    QualityActionProposal, QualityActionProposalRequest, QualityActorClass,
    QualityApplyPostureClass, QualityDebtReopenStateClass, QualityFixSafetyClass,
    QualityGovernanceError, QualityGovernanceSupportExport, QualityLockReasonClass,
    QualityLockStateClass, QualityMutationScopeClass, QualityOwnerClass,
    QualityPolicyLockStateClass, QualityPreviewRequirementClass, QualityProfileResolutionRequest,
    QualityProfileResolver, QualityProfileSourceCandidate, QualityProfileSourceLayer,
    QualityProfileSourceRow, QualityProfileSourceStateClass, QualityProfileSurfaceProjection,
    QualityReleaseDebtCounts, QualityReleaseDebtPacket, QualityReleaseDebtRow,
    QualityReleaseDebtStateClass, QualityReopenRuleClass, QualityRollbackBoundaryClass,
    QualitySafetyClass, QualitySaveParticipantRow, QualitySession, QualitySessionOutcomeClass,
    QualitySessionRequest, QualitySessionTriggerClass, QualitySurfaceClass,
    QualityTargetScopeClass, QualityToolFamilyClass, QualityTruthMutationClass,
    SaveParticipantPhaseClass, SuppressionRecord, SuppressionRecordRequest, BASELINE_RECORD_KIND,
    EFFECTIVE_QUALITY_PROFILE_RECORD_KIND, QUALITY_ACTION_PROPOSAL_RECORD_KIND,
    QUALITY_GOVERNANCE_SCHEMA_VERSION, QUALITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND,
    QUALITY_RELEASE_DEBT_PACKET_RECORD_KIND, QUALITY_SESSION_RECORD_KIND, SUPPRESSION_RECORD_KIND,
};
pub use queue_governor_and_admission_control::{
    current_stable_queue_governor_packet, BackgroundJobKind, CancellationContract,
    CheckpointPolicy, CollapsePolicy, InitiatingSource, QueueGovernorConsumerProjection,
    QueueGovernorLab, QueueGovernorStablePacket, QueueGovernorSupportExport, QueueJobScope,
    QueueLaneRule, QueueLaneSummary, RuntimeHealthProjection, StableBackgroundJob, StalenessInputs,
    StalenessPolicy, QUEUE_GOVERNOR_ARTIFACT_DOC_REF, QUEUE_GOVERNOR_DOC_REF,
    QUEUE_GOVERNOR_FIXTURE_DIR, QUEUE_GOVERNOR_PACKET_ARTIFACT_REF, QUEUE_GOVERNOR_SCHEMA_REF,
    QUEUE_GOVERNOR_SCHEMA_VERSION, QUEUE_GOVERNOR_STABLE_PACKET_RECORD_KIND,
    QUEUE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND,
};
pub use queue_session_terminal_governance::{
    current_queue_session_terminal_governance_packet, ActivityJobStateClass,
    ActivityNextActionClass, AuthorityStatusClass as GovernanceAuthorityStatusClass,
    BoundaryDisclosureClass as GovernanceBoundaryDisclosureClass,
    BudgetDomainClass as GovernanceBudgetDomainClass,
    CancellationClass as GovernanceCancellationClass,
    CheckpointPolicyClass as GovernanceCheckpointPolicyClass,
    ClipboardPostureClass as GovernanceClipboardPostureClass,
    CollapseKeyClass as GovernanceCollapseKeyClass, ConfidenceClass as GovernanceConfidenceClass,
    ConsumerSurface as GovernanceConsumerSurface,
    DowngradeRuleClass as GovernanceDowngradeRuleClass, EvidenceClass as GovernanceEvidenceClass,
    FairnessOutcomeClass as GovernanceFairnessOutcomeClass, FindingKind as GovernanceFindingKind,
    FindingSeverity as GovernanceFindingSeverity,
    GovernanceRowClass as QueueSessionTerminalGovernanceRowClass, GovernedJobIdentity,
    GovernedJobKind, GovernedWorkloadClass, KnownLimitClass as GovernanceKnownLimitClass,
    NoHiddenRerunClass, PowerThermalStateClass as GovernancePowerThermalStateClass,
    PromotionState as GovernancePromotionState,
    ProtectedPathBudgetOutcomeClass as GovernanceProtectedPathBudgetOutcomeClass,
    ProtectedPathClass as GovernanceProtectedPathClass, QueueLaneClass as GovernanceQueueLaneClass,
    QueueSessionActivityJobRow, QueueSessionFairnessLaneRow, QueueSessionPowerThermalTransition,
    QueueSessionProtectedPathFitnessRow, QueueSessionSchedulerLaneRow,
    QueueSessionTerminalGovernanceConsumerProjection, QueueSessionTerminalGovernancePacket,
    QueueSessionTerminalGovernancePacketInput, QueueSessionTerminalGovernanceRow,
    QueueSessionTerminalGovernanceSupportExport, QueueSessionTerminalLinkificationRow,
    QueueSessionTerminalOutputConsumerRow, QueueSessionTerminalProtocolSurfaceRow,
    QueueSessionTerminalSessionContinuityRow, QueueSessionTerminalSharedControlAuditRow,
    QueueSessionTerminalSharedControlRow, QueueSessionTerminalTranscriptExportRow,
    RestoreFidelityClass as GovernanceRestoreFidelityClass,
    ResumeRequirementClass as GovernanceResumeRequirementClass, RetryClass as GovernanceRetryClass,
    SchedulerLaneRetryStateClass, SessionContinuityClass as GovernanceSessionContinuityClass,
    SharedControlAuditEventClass as GovernanceSharedControlAuditEventClass,
    SharedControlGrantStateClass as GovernanceSharedControlGrantStateClass,
    SharedSessionRoleClass as GovernanceSharedSessionRoleClass,
    SheddingReasonClass as GovernanceSheddingReasonClass,
    ShellIntegrationSignalClass as GovernanceShellIntegrationSignalClass,
    SupportClass as GovernanceSupportClass,
    TerminalBoundaryClass as GovernanceTerminalBoundaryClass,
    TerminalLinkConfidenceClass as GovernanceTerminalLinkConfidenceClass,
    TerminalLinkTargetClass as GovernanceTerminalLinkTargetClass,
    TerminalOutputConsumerClass as GovernanceTerminalOutputConsumerClass,
    TerminalOutputProvenanceClass as GovernanceTerminalOutputProvenanceClass,
    TerminalOutputTaintClass as GovernanceTerminalOutputTaintClass,
    TerminalProtocolCapabilityClass as GovernanceTerminalProtocolCapabilityClass,
    TerminalProtocolSurfaceClass as GovernanceTerminalProtocolSurfaceClass,
    TranscriptExportRedactionClass as GovernanceTranscriptExportRedactionClass,
    ValidationFinding as GovernanceValidationFinding, BACKGROUND_QUEUE_CONTRACT_DOC_REF,
    CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF, FOREGROUND_TASK_BUDGET_DOMAIN_REF,
    HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF, KNOWLEDGE_REFRESH_BUDGET_DOMAIN_REF,
    MAINTENANCE_BUDGET_DOMAIN_REF, PROVIDER_OVERLAY_BUDGET_DOMAIN_REF,
    QUEUE_SESSION_TERMINAL_ACTIVITY_JOB_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_FAIRNESS_LANE_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF, QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_FIXTURE_DIR, QUEUE_SESSION_TERMINAL_GOVERNANCE_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF, QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_VERSION,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_LINKIFICATION_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_OUTPUT_CONSUMER_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_PROTECTED_PATH_FITNESS_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_PROTOCOL_SURFACE_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_SCHEDULER_LANE_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_SESSION_CONTINUITY_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_SHARED_CONTROL_AUDIT_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_SHARED_CONTROL_ROW_RECORD_KIND,
    QUEUE_SESSION_TERMINAL_TRANSCRIPT_EXPORT_ROW_RECORD_KIND, REPLICATION_BUDGET_DOMAIN_REF,
};
pub use recipe_builder::{
    canonical_reused_contract_refs as recipe_builder_reused_contract_refs, copy_cli_for_verb,
    current_recipe_builder_first_consumers_input, open_docs_for_verb,
    seeded_blocked_recipe_builder, seeded_consumer_builder, seeded_recipe_builder_export_roundtrip,
    seeded_recipe_builder_first_consumers_packet, slugify_canonical_verb, step_parity_holds,
    validate_recipe_builder_first_consumers_packet, FirstConsumersFinding,
    FirstConsumersFindingKind, FirstConsumersFindingSeverity, RecipeBuilder,
    RecipeBuilderConsumerBinding, RecipeBuilderEntrypoint, RecipeBuilderError, RecipeBuilderExport,
    RecipeBuilderFirstConsumersCliHeadlessView, RecipeBuilderFirstConsumersInput,
    RecipeBuilderFirstConsumersPacket, RecipeBuilderFirstConsumersSupportExport,
    RecipeBuilderInvariantsBlock, RecipeBuilderStep, ReorderEvent, ReorderGesture,
    ReorderGestureKind, StepBlockReason, SupportExportConsumerRow, RECIPE_BUILDER_CLI_BINARY,
    RECIPE_BUILDER_COMMAND_DOCS_BASE, RECIPE_BUILDER_DEFAULT_AUTHORING_LANGUAGE,
    RECIPE_BUILDER_EXPORT_RECORD_KIND, RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND,
    RECIPE_BUILDER_FIRST_CONSUMERS_DOC_REF, RECIPE_BUILDER_FIRST_CONSUMERS_FIXTURE_DIR,
    RECIPE_BUILDER_FIRST_CONSUMERS_ID, RECIPE_BUILDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    RECIPE_BUILDER_FIRST_CONSUMERS_RECORD_KIND, RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_REF,
    RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION,
    RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
    RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND, RECIPE_BUILDER_SESSION_RECORD_KIND,
};
pub use recipes::{
    RecipeAlphaContractRefs, RecipeAlphaCoverage, RecipeAlphaFinding, RecipeAlphaFindingSeverity,
    RecipeAlphaFixtureMetadata, RecipeAlphaPage, RecipeAlphaSupportExport,
    RecipeAlphaValidationReport, RecipeApprovalClass, RecipeAttribution, RecipeAttributionSummary,
    RecipeAttributionSurfaceClass, RecipeAuditEvent, RecipeAuditEventClass,
    RecipeAuditEventSummary, RecipeDefinition, RecipeDefinitionSummary, RecipeDenialReasonClass,
    RecipePreviewRequirementClass, RecipeRun, RecipeRunDispositionClass, RecipeRunSummary,
    RecipeStep, RecipeStepDisposition, RecipeStepDispositionClass, RecipeTrustGateClass,
    RecipeWriteClass, StepCommandLineageClass, StepModeRequirementClass,
    RECIPE_ALPHA_ATTRIBUTION_RECORD_KIND, RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND,
    RECIPE_ALPHA_DEFINITION_RECORD_KIND, RECIPE_ALPHA_PAGE_RECORD_KIND,
    RECIPE_ALPHA_RUN_RECORD_KIND, RECIPE_ALPHA_SCHEMA_VERSION, RECIPE_ALPHA_SHARED_CONTRACT_REF,
    RECIPE_ALPHA_SUPPORT_EXPORT_RECORD_KIND, RECIPE_ALPHA_VALIDATION_REPORT_RECORD_KIND,
};
pub use remote_helper_skew_beta::{
    RemoteHelperBetaCompatibilityRow, RemoteHelperBetaRecord, RemoteHelperBetaSupportExport,
    RemoteHelperLifecyclePhaseClass, RemoteHelperRepairPathClass, RemoteHelperSkewVisibilityClass,
    RemoteHelperVisibleVersionState, REMOTE_HELPER_SKEW_BETA_COMPATIBILITY_ROW_RECORD_KIND,
    REMOTE_HELPER_SKEW_BETA_RECORD_KIND, REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION,
    REMOTE_HELPER_SKEW_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use request_workspace::{
    seeded_request_workspace_record, seeded_request_workspace_support_export,
    seeded_send_inspector_report, AssertionDescriptor, AssertionKind, AssertionOutcomeClass,
    AssertionResultRow, AuthProfile, AuthStrategyKind, CredentialClass, EnvironmentLayerKind,
    EnvironmentSet, EnvironmentVariableLayer, ExpectedSideEffectRow, LatencyBandClass,
    RequestDocument, RequestMethodClass, RequestWorkspaceAlphaRecord,
    RequestWorkspaceAlphaViolation, RequestWorkspaceSeededScenario, RequestWorkspaceSupportExport,
    ResponseArtifact, ResponsePreviewClass, ResponseRedactionClass, SchemaSnapshot,
    SchemaSnapshotFreshness, SchemaSnapshotKind, SchemaSnapshotSourceClass, SendInspectorBanner,
    SendInspectorReadiness, SendInspectorReport, SideEffectClass, REQUEST_WORKSPACE_ALPHA_LANE_ID,
    REQUEST_WORKSPACE_ALPHA_RECORD_KIND, REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
    REQUEST_WORKSPACE_SEND_INSPECTOR_RECORD_KIND, REQUEST_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use request_workspace_contracts::{
    AssertionEvidenceState, AssertionSuite, AssertionSuiteLineageClass, AuthSourceClass,
    EndpointIdentity, EndpointSourceClass, EnvironmentFingerprintState, FingerprintDigestClass,
    PortableExportClass, PortableExportContract, RequestEnvironmentFingerprint,
    RequestHistoryPosture, RequestHistoryRetentionClass, ResponseCopyExportClass,
    ResponsePayloadSizeClass, ResponsePreviewComponentClass, ResponsePreviewRule,
    ResponseSafePreviewClass, REQUEST_ASSERTION_SUITE_SCHEMA_ID,
    REQUEST_ENVIRONMENT_FINGERPRINT_SCHEMA_ID, REQUEST_RESPONSE_PREVIEW_SCHEMA_ID,
};
pub use rerun::{
    built_in_rerun_command_bindings, RerunAttemptSummary, RerunCommandBinding, RerunContractKind,
    RerunDiffClass, RerunDiffRow, RerunDispatchState, RerunKeyboardRoute, RerunLane,
    RerunLastLaunch, RerunLastLoop, RerunPreparedAttempt, RerunRunContract, RerunSupportExport,
    RerunTargetComparison, RerunTargetMode, RerunTargetSnapshot, RerunUnavailableReason,
    RERUN_COMMAND_BINDING_RECORD_KIND, RERUN_LAST_LAUNCH_RECORD_KIND, RERUN_LAST_TASK_COMMAND_ID,
    RERUN_LAST_TEST_COMMAND_ID, RERUN_LOOP_SCHEMA_VERSION, RERUN_PREPARED_ATTEMPT_RECORD_KIND,
    RERUN_SUPPORT_EXPORT_RECORD_KIND, RERUN_TARGET_COMPARISON_RECORD_KIND,
};
pub use resource_governor::{
    seeded_resource_governor_snapshot, seeded_resource_governor_support_export,
    AdmissionControlDecision, AdmissionDecisionClass, CheckpointMetadata, GovernorHealthState,
    GovernorTransition, GovernorWorkClass, OverrideDecisionClass, OverrideScope, OverrideSheet,
    PressureDimension, PressureInput, ProtectedForegroundAction, QueueLane, QueueLaneState,
    QueueLaneStateFlag, ResourceGovernorSnapshot, ResourceGovernorSupportExport,
    ResourceGovernorValidationReport, ResourceGovernorValidationViolation, VisibleHealthState,
    QUEUE_LANE_STATE_RECORD_KIND, RESOURCE_GOVERNOR_SCHEMA_VERSION,
    RESOURCE_GOVERNOR_SNAPSHOT_RECORD_KIND, RESOURCE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND,
};
pub use run_history::{
    canonical_reused_contract_refs as run_history_reused_contract_refs,
    current_run_history_first_consumers_input, seeded_consumer_entry as seeded_run_history_entry,
    seeded_consumer_panel as seeded_run_history_panel, seeded_imported_entry,
    seeded_run_history_export_roundtrip, seeded_run_history_first_consumers_packet,
    validate_run_history_first_consumers_packet, ArtifactBundleStateClass, ArtifactLink,
    ArtifactLinkClass, AutomationLayerClass, ContextSummary, CurrentPolicyBlocker,
    ExecutionModeClass, KillSwitchObservationClass, OpenAsRecipeActionClass,
    PolicyObservationClass, RedactionModeClass, RerunActionClass, RerunDisposition,
    RerunResolution, RetentionClass, RunHistoryConsumerBinding, RunHistoryEntry, RunHistoryError,
    RunHistoryEvidenceExport, RunHistoryEvidenceRow, RunHistoryFinding, RunHistoryFindingKind,
    RunHistoryFindingSeverity, RunHistoryFirstConsumersCliHeadlessView,
    RunHistoryFirstConsumersInput, RunHistoryFirstConsumersPacket,
    RunHistoryFirstConsumersSupportExport, RunHistoryInvariantsBlock, RunHistorySupportConsumerRow,
    RunHistorySupportEntryRow, RunIdentity, RunResultClass, TrustStateClass,
    RERUN_RESOLUTION_RECORD_KIND, RUN_HISTORY_DOC_REF, RUN_HISTORY_EVIDENCE_EXPORT_RECORD_KIND,
    RUN_HISTORY_EVIDENCE_ROW_RECORD_KIND, RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND, RUN_HISTORY_FIRST_CONSUMERS_ID,
    RUN_HISTORY_FIRST_CONSUMERS_PACKET_ARTIFACT_REF, RUN_HISTORY_FIRST_CONSUMERS_RECORD_KIND,
    RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_REF, RUN_HISTORY_FIRST_CONSUMERS_SCHEMA_VERSION,
    RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
    RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND, RUN_HISTORY_FIXTURE_DIR,
};
pub use run_lineage::{
    seeded_run_history_support_export, DurableJobRow, RerunReviewDriftField, RerunReviewDriftRow,
    RerunReviewMode, RerunReviewModeOption, RerunReviewSheet, RunActionRef, RunArtifactActionClass,
    RunArtifactDetailSheet, RunArtifactKind, RunArtifactRetentionClass, RunArtifactViewerClass,
    RunBoundaryClass, RunBuildIdentity, RunContextSummary, RunContinuityMarker,
    RunCurrentRelationshipClass, RunFreshnessClass, RunHistorySupportExport, RunInterruptionKind,
    RunLifecycleState, RunLineageSeededScenario, RunSummaryCard, DURABLE_JOB_ROW_RECORD_KIND,
    RERUN_REVIEW_SHEET_RECORD_KIND, RUN_ARTIFACT_DETAIL_SHEET_RECORD_KIND,
    RUN_HISTORY_SUPPORT_EXPORT_RECORD_KIND, RUN_LINEAGE_SCHEMA_VERSION,
    RUN_SUMMARY_CARD_RECORD_KIND,
};
pub use runtime_continuity_surface_qualification::{
    current_runtime_continuity_surface_qualification_export,
    seeded_runtime_continuity_surface_qualification_packet,
    EvidenceCurrency as RuntimeContinuityEvidenceCurrency, RuntimeContinuityEvidenceConsumer,
    RuntimeContinuityEvidenceConsumerBinding, RuntimeContinuityEvidenceIndexEntry,
    RuntimeContinuityLabel, RuntimeContinuityNarrowReason, RuntimeContinuityProfile,
    RuntimeContinuityProofClass, RuntimeContinuitySurfaceQualificationArtifactError,
    RuntimeContinuitySurfaceQualificationPacket, RuntimeContinuitySurfaceQualificationPacketInput,
    RuntimeContinuitySurfaceQualificationViolation,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_ARTIFACT_REF,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_DOC_REF,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_FIXTURE_DIR,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_HELP_REF,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_REF,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SUMMARY_REF,
};
pub use sandbox::{
    current_stable_sandbox_profile_packet, current_stable_sandbox_profile_packet_input,
    ApprovalActionClass as SandboxApprovalActionClass,
    ApprovalEnvelopeBinding as SandboxApprovalEnvelopeBinding,
    ApprovalRevocationState as SandboxApprovalRevocationState,
    BackendEnforcementClass as SandboxBackendEnforcementClass,
    BackendPlatformClass as SandboxBackendPlatformClass, ChildProcessPosture,
    EnforcementPosture as SandboxEnforcementPosture, FilesystemPosture,
    NetworkPosture as SandboxNetworkPosture, RevalidationTrigger as SandboxRevalidationTrigger,
    SandboxBackendRow, SandboxConsumerProjection, SandboxConsumerSurface, SandboxFindingKind,
    SandboxFindingSeverity, SandboxProfile, SandboxProfileId, SandboxProfilePacket,
    SandboxProfilePacketInput, SandboxProfileSupportExport, SandboxPromotionState,
    SandboxValidationFinding, SecretPosture as SandboxSecretPosture, TrustRequirement,
    SANDBOX_BACKEND_CROSSWALK_REF, SANDBOX_PROFILE_DOC_REF, SANDBOX_PROFILE_HELP_DOC_REF,
    SANDBOX_PROFILE_PACKET_ARTIFACT_REF, SANDBOX_PROFILE_PACKET_RECORD_KIND,
    SANDBOX_PROFILE_SCHEMA_REF, SANDBOX_PROFILE_SCHEMA_VERSION,
    SANDBOX_PROFILE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use shared_debug_alpha::{
    LocalDebugContinuityClass, LocalDebugContinuityObservation,
    LocalDebugContinuityObservationSummary, SharedDebugAlphaContractRefs, SharedDebugAlphaCoverage,
    SharedDebugAlphaFinding, SharedDebugAlphaFindingSeverity, SharedDebugAlphaFixtureMetadata,
    SharedDebugAlphaPage, SharedDebugAlphaSupportExport, SharedDebugAlphaValidationReport,
    SharedDebugAuditEvent, SharedDebugAuditEventClass, SharedDebugAuditEventSummary,
    SharedDebugBinding, SharedDebugControlState, SharedDebugControlStateClass,
    SharedDebugControlStateSummary, SHARED_DEBUG_ALPHA_AUDIT_EVENT_RECORD_KIND,
    SHARED_DEBUG_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
    SHARED_DEBUG_ALPHA_CONTROL_STATE_RECORD_KIND, SHARED_DEBUG_ALPHA_PAGE_RECORD_KIND,
    SHARED_DEBUG_ALPHA_PRESENTER_HANDOFF_RECORD_KIND, SHARED_DEBUG_ALPHA_SCHEMA_VERSION,
    SHARED_DEBUG_ALPHA_SHARED_CONTRACT_REF, SHARED_DEBUG_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
    SHARED_DEBUG_ALPHA_VALIDATION_REPORT_RECORD_KIND,
};
pub use shared_terminal_alpha::{
    ControlRevocationCauseClass, LocalContinuityClass, LocalTerminalContinuityObservation,
    LocalTerminalContinuityObservationSummary, ParticipantRoleClass, PresenterHandoffEvent,
    PresenterHandoffOutcomeClass, PresenterHandoffSummary, SharedTerminalAlphaContractRefs,
    SharedTerminalAlphaCoverage, SharedTerminalAlphaFinding, SharedTerminalAlphaFindingSeverity,
    SharedTerminalAlphaFixtureMetadata, SharedTerminalAlphaPage, SharedTerminalAlphaSupportExport,
    SharedTerminalAlphaValidationReport, SharedTerminalAuditEvent, SharedTerminalAuditEventClass,
    SharedTerminalAuditEventSummary, SharedTerminalBinding, SharedTerminalControlState,
    SharedTerminalControlStateClass, SharedTerminalControlStateSummary,
    SHARED_TERMINAL_ALPHA_AUDIT_EVENT_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_CONTROL_STATE_RECORD_KIND, SHARED_TERMINAL_ALPHA_PAGE_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_PRESENTER_HANDOFF_RECORD_KIND, SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
    SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF, SHARED_TERMINAL_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_VALIDATION_REPORT_RECORD_KIND,
};
pub use stabilize_debugger_host_and_adapter_negotiation::{
    current_stable_debugger_stabilization_truth_packet,
    AdapterDescriptorFieldClass as DebuggerStabilizationAdapterDescriptorFieldClass,
    AttachLaunchParitySurfaceClass as DebuggerStabilizationAttachLaunchParitySurfaceClass,
    AttachLaunchPostureClass as DebuggerStabilizationAttachLaunchPostureClass,
    ConsumerSurface as DebuggerStabilizationConsumerSurface,
    CrashIsolationAssertionClass as DebuggerStabilizationCrashIsolationAssertionClass,
    DebuggerStabilizationConfidenceClass, DebuggerStabilizationConsumerProjection,
    DebuggerStabilizationLaneClass, DebuggerStabilizationRow, DebuggerStabilizationRowClass,
    DebuggerStabilizationTruthArtifactError, DebuggerStabilizationTruthPacket,
    DebuggerStabilizationTruthPacketInput, DebuggerStabilizationTruthSupportExport,
    DowngradeAutomationClass as DebuggerStabilizationDowngradeAutomationClass,
    EvidenceClass as DebuggerStabilizationEvidenceClass,
    FindingKind as DebuggerStabilizationFindingKind,
    FindingSeverity as DebuggerStabilizationFindingSeverity,
    KnownLimitClass as DebuggerStabilizationKnownLimitClass,
    PromotionState as DebuggerStabilizationPromotionState,
    SupportClass as DebuggerStabilizationSupportClass,
    ValidationFinding as DebuggerStabilizationValidationFinding,
    WedgeClass as DebuggerStabilizationWedgeClass, DEBUGGER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF,
    DEBUGGER_STABILIZATION_TRUTH_DOC_REF, DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR,
    DEBUGGER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF,
    DEBUGGER_STABILIZATION_TRUTH_PACKET_RECORD_KIND, DEBUGGER_STABILIZATION_TRUTH_SCHEMA_REF,
    DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION,
    DEBUGGER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_execution_context_resolver::{
    current_stable_stabilize_execution_context_resolver_truth_packet,
    ConsumerSurface as StabilizeExecutionContextResolverConsumerSurface,
    DowngradeAutomationClass as StabilizeExecutionContextResolverDowngradeAutomationClass,
    EvidenceClass as StabilizeExecutionContextResolverEvidenceClass,
    ExecutionContextConfidenceClass as StabilizeExecutionContextResolverConfidenceClass,
    ExecutionContextRowClass as StabilizeExecutionContextResolverRowClass,
    ExecutionLaneClass as StabilizeExecutionContextResolverLaneClass,
    FindingKind as StabilizeExecutionContextResolverFindingKind,
    FindingSeverity as StabilizeExecutionContextResolverFindingSeverity,
    KnownLimitClass as StabilizeExecutionContextResolverKnownLimitClass,
    PromotionState as StabilizeExecutionContextResolverPromotionState,
    ResolverStateClass as StabilizeExecutionContextResolverStateClass,
    StabilizeExecutionContextResolverConsumerProjection, StabilizeExecutionContextResolverRow,
    StabilizeExecutionContextResolverTruthArtifactError,
    StabilizeExecutionContextResolverTruthPacket,
    StabilizeExecutionContextResolverTruthPacketInput,
    StabilizeExecutionContextResolverTruthSupportExport,
    SupportClass as StabilizeExecutionContextResolverSupportClass,
    SurfaceBindingClass as StabilizeExecutionContextResolverSurfaceBindingClass,
    ValidationFinding as StabilizeExecutionContextResolverValidationFinding,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_ARTIFACT_DOC_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_DOC_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_FIXTURE_DIR,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_PACKET_ARTIFACT_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_PACKET_RECORD_KIND,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_SCHEMA_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_SCHEMA_VERSION,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_problem_records_output_channels_and_execution_evidence::{
    current_stable_execution_evidence_bundle, current_stable_execution_evidence_bundle_input,
    CanonicalOutputChannelName as ExecutionEvidenceOutputChannelName,
    EvidenceConfidenceClass as ExecutionEvidenceConfidenceClass,
    EvidenceConsumerProjection as ExecutionEvidenceConsumerProjection,
    EvidenceConsumerSurface as ExecutionEvidenceConsumerSurface,
    EvidenceFindingKind as ExecutionEvidenceFindingKind,
    EvidenceFindingSeverity as ExecutionEvidenceFindingSeverity,
    EvidenceFreshnessState as ExecutionEvidenceFreshnessState,
    EvidencePromotionState as ExecutionEvidencePromotionState, EvidenceQualifier,
    EvidenceTaskEventKind as ExecutionEvidenceTaskEventKind,
    EvidenceValidationFinding as ExecutionEvidenceValidationFinding, ExecutionEvidenceBundle,
    ExecutionEvidenceBundleInput, ExecutionEvidenceKind, ExecutionEvidenceSupportExport,
    ExecutionSourceKind, OutputChunkRenderClass as ExecutionEvidenceOutputChunkRenderClass,
    OutputRetentionClass as ExecutionEvidenceOutputRetentionClass,
    OutputTrustState as ExecutionEvidenceOutputTrustState, ProblemLocation, ProblemSeverity,
    StableExecutionEvidenceObject, StableOutputChannelDescriptor, StableOutputChunk,
    StableProblemRecord, StableTaskEventEnvelope, EXECUTION_EVIDENCE_BUNDLE_ARTIFACT_DOC_REF,
    EXECUTION_EVIDENCE_BUNDLE_DOC_REF, EXECUTION_EVIDENCE_BUNDLE_FIXTURE_DIR,
    EXECUTION_EVIDENCE_BUNDLE_PACKET_ARTIFACT_REF, EXECUTION_EVIDENCE_BUNDLE_RECORD_KIND,
    EXECUTION_EVIDENCE_BUNDLE_SCHEMA_REF, EXECUTION_EVIDENCE_BUNDLE_SCHEMA_VERSION,
    EXECUTION_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_task_discovery_launch_profiles_rerun_last_behavior::{
    current_stable_task_event_truth_packet, ConsumerSurface as TaskEventTruthConsumerSurface,
    DowngradeAutomationClass as TaskEventTruthDowngradeAutomationClass,
    DownstreamSurfaceClass as TaskEventTruthDownstreamSurfaceClass,
    EnvelopeFieldClass as TaskEventTruthEnvelopeFieldClass,
    EvidenceClass as TaskEventTruthEvidenceClass, FindingKind as TaskEventTruthFindingKind,
    FindingSeverity as TaskEventTruthFindingSeverity,
    KnownLimitClass as TaskEventTruthKnownLimitClass,
    PromotionState as TaskEventTruthPromotionState, SupportClass as TaskEventTruthSupportClass,
    TaskEventTruthArtifactError, TaskEventTruthConfidenceClass, TaskEventTruthConsumerProjection,
    TaskEventTruthLaneClass, TaskEventTruthPacket, TaskEventTruthPacketInput, TaskEventTruthRow,
    TaskEventTruthRowClass, TaskEventTruthSupportExport,
    ValidationFinding as TaskEventTruthValidationFinding, WedgeClass as TaskEventTruthWedgeClass,
    TASK_EVENT_TRUTH_ARTIFACT_DOC_REF, TASK_EVENT_TRUTH_DOC_REF, TASK_EVENT_TRUTH_FIXTURE_DIR,
    TASK_EVENT_TRUTH_PACKET_ARTIFACT_REF, TASK_EVENT_TRUTH_PACKET_RECORD_KIND,
    TASK_EVENT_TRUTH_SCHEMA_REF, TASK_EVENT_TRUTH_SCHEMA_VERSION,
    TASK_EVENT_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_the_artifact_manager_preview_runtime_inspectors_and::{
    current_stable_evidence_export_truth_packet, ConfidenceClass as EvidenceExportConfidenceClass,
    ConsumerProjection as EvidenceExportConsumerProjection,
    ConsumerProjectionSurface as EvidenceExportConsumerProjectionSurface,
    ConsumerSurfaceClass as EvidenceExportConsumerSurfaceClass,
    DowngradeAutomationClass as EvidenceExportDowngradeAutomationClass,
    EvidenceClass as EvidenceExportEvidenceClass, EvidenceExportLaneClass, EvidenceExportRow,
    EvidenceExportRowClass, EvidenceExportTruthArtifactError, EvidenceExportTruthPacket,
    EvidenceExportTruthPacketInput, EvidenceExportTruthSupportExport,
    FindingKind as EvidenceExportFindingKind, FindingSeverity as EvidenceExportFindingSeverity,
    KnownLimitClass as EvidenceExportKnownLimitClass,
    PromotionState as EvidenceExportPromotionState,
    ReplayChronologyStateClass as EvidenceExportReplayChronologyStateClass,
    RetentionClass as EvidenceExportRetentionClass,
    SignalSliceKindClass as EvidenceExportSignalSliceKindClass,
    SliceFreshnessClass as EvidenceExportSliceFreshnessClass,
    SupportClass as EvidenceExportSupportClass,
    ValidationFinding as EvidenceExportValidationFinding, WedgeClass as EvidenceExportWedgeClass,
    EVIDENCE_EXPORT_TRUTH_ARTIFACT_DOC_REF, EVIDENCE_EXPORT_TRUTH_DOC_REF,
    EVIDENCE_EXPORT_TRUTH_FIXTURE_DIR, EVIDENCE_EXPORT_TRUTH_PACKET_ARTIFACT_REF,
    EVIDENCE_EXPORT_TRUTH_PACKET_RECORD_KIND, EVIDENCE_EXPORT_TRUTH_SCHEMA_REF,
    EVIDENCE_EXPORT_TRUTH_SCHEMA_VERSION, EVIDENCE_EXPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_the_test_explorer_inline_results_watch_mode::{
    current_stable_test_explorer_stabilization_truth_packet,
    ConsumerSurface as TestExplorerStabilizationConsumerSurface,
    ConsumerSurfaceBindingClass as TestExplorerStabilizationConsumerSurfaceBindingClass,
    DiscoveryPostureClass as TestExplorerStabilizationDiscoveryPostureClass,
    DowngradeAutomationClass as TestExplorerStabilizationDowngradeAutomationClass,
    EvidenceClass as TestExplorerStabilizationEvidenceClass,
    FindingKind as TestExplorerStabilizationFindingKind,
    FindingSeverity as TestExplorerStabilizationFindingSeverity,
    KnownLimitClass as TestExplorerStabilizationKnownLimitClass,
    PromotionState as TestExplorerStabilizationPromotionState,
    SelectorDurabilityClass as TestExplorerStabilizationSelectorDurabilityClass,
    SupportClass as TestExplorerStabilizationSupportClass,
    TestExplorerConfidenceClass as TestExplorerStabilizationConfidenceClass,
    TestExplorerConsumerProjection as TestExplorerStabilizationConsumerProjection,
    TestExplorerLaneClass as TestExplorerStabilizationLaneClass,
    TestExplorerRow as TestExplorerStabilizationRow,
    TestExplorerRowClass as TestExplorerStabilizationRowClass,
    TestExplorerStabilizationTruthArtifactError, TestExplorerStabilizationTruthPacket,
    TestExplorerStabilizationTruthPacketInput, TestExplorerStabilizationTruthSupportExport,
    TestIdentityClass as TestExplorerStabilizationTestIdentityClass,
    ValidationFinding as TestExplorerStabilizationValidationFinding,
    WatchModeSupportClass as TestExplorerStabilizationWatchModeSupportClass,
    WedgeClass as TestExplorerStabilizationWedgeClass,
    TEST_EXPLORER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF, TEST_EXPLORER_STABILIZATION_TRUTH_DOC_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR,
    TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_RECORD_KIND,
    TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_REF, TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION,
    TEST_EXPLORER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use support_matrix_beta::{
    SupportMatrixAttachSupport, SupportMatrixBetaManifest, SupportMatrixBetaSupportExport,
    SupportMatrixClass, SupportMatrixContextLane, SupportMatrixContextLaneExpectation,
    SupportMatrixContextLaneSupport, SupportMatrixContextSupport, SupportMatrixDowngradeRule,
    SupportMatrixInputMismatch, SupportMatrixLaunchSupport, SupportMatrixTestSupport,
    SupportMatrixWedgeId, SupportMatrixWedgeInput, SupportMatrixWedgeRow,
    SUPPORT_MATRIX_BETA_MANIFEST_RECORD_KIND, SUPPORT_MATRIX_BETA_SCHEMA_VERSION,
    SUPPORT_MATRIX_BETA_SUPPORT_EXPORT_RECORD_KIND, SUPPORT_MATRIX_BETA_WEDGE_INPUT_RECORD_KIND,
    SUPPORT_MATRIX_BETA_WEDGE_ROW_RECORD_KIND,
};
pub use target_discovery::{
    DiscoveryFreshnessClass, DiscoverySourceClass, ProtectedActionClass,
    ProtectedActionDecisionClass, ProtectedActionDecisionRow, SupportedCapabilityClass,
    TargetDiscoveryBetaCoverageManifest, TargetDiscoveryBetaCoverageRow,
    TargetDiscoveryBetaProjection, TargetDiscoveryBetaRow, TargetDiscoveryBetaSupportExport,
    TARGET_DISCOVERY_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TARGET_DISCOVERY_BETA_PROJECTION_RECORD_KIND, TARGET_DISCOVERY_BETA_ROW_RECORD_KIND,
    TARGET_DISCOVERY_BETA_SCHEMA_VERSION, TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use targets::{
    HostBoundaryCueClass, TargetConfidenceCard, TargetConfidenceExplanationRow,
    TargetConfidenceLaneClass, TargetConfidenceReviewPacket, TargetConfidenceReviewRow,
    TargetConfidenceSupportExport, TargetDiscoveryConfidenceClass, TargetHostBoundaryRow,
    TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION, TARGET_CONFIDENCE_CARD_RECORD_KIND,
    TARGET_CONFIDENCE_REVIEW_PACKET_RECORD_KIND, TARGET_CONFIDENCE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use task_events::{
    lane_for_event, lane_for_wedge, TaskEventBetaCoverageManifest, TaskEventBetaLane,
    TaskEventBetaLaneCoverageRow, TASK_EVENT_BETA_COVERAGE_MANIFEST_RECORD_KIND,
};
pub use tasks::{
    RawEnvelopeRetentionState, RawTaskEventEnvelope, TaskActivityProjection, TaskArtifactKind,
    TaskBlockReason, TaskConsumerSurfaceClass, TaskDegradationReason, TaskDiagnosticSeverity,
    TaskEvent, TaskEventConfidence, TaskEventIdentity, TaskEventKind, TaskEventPayload,
    TaskEventProvenance, TaskEventRedactionClass, TaskEventSourceKind, TaskEventStream,
    TaskEventStreamError, TaskExitStatus, TaskFailureClass, TaskInputClass, TaskInputRequest,
    TaskOutputStreamClass, TaskProgress, TaskShellProjection, TaskState, TaskStateClass,
    TaskSupportEventRow, TaskSupportExport, TaskWedgeClass, RAW_TASK_EVENT_ENVELOPE_RECORD_KIND,
    TASK_ACTIVITY_PROJECTION_RECORD_KIND, TASK_EVENT_RECORD_KIND, TASK_EVENT_SCHEMA_VERSION,
    TASK_EVENT_STREAM_RECORD_KIND, TASK_SHELL_PROJECTION_RECORD_KIND, TASK_STATE_RECORD_KIND,
    TASK_SUPPORT_EXPORT_RECORD_KIND,
};
pub use testing::{
    InlineTestResultProjection, InlineTestResultRow, TestArtifactIdentity, TestArtifactKind,
    TestRunnerBetaCoverageManifest, TestRunnerBetaCoverageRow, TestRunnerBetaFramework,
    TestRunnerBetaParityState, TestRunnerBetaProjection, TestRunnerBetaRerunParity,
    TestRunnerBetaSupportExport, TestTreeProjection, TestTreeRow, TestTreeRowKind,
    TEST_RUNNER_BETA_ARTIFACT_IDENTITY_RECORD_KIND, TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TEST_RUNNER_BETA_INLINE_PROJECTION_RECORD_KIND, TEST_RUNNER_BETA_INLINE_ROW_RECORD_KIND,
    TEST_RUNNER_BETA_RERUN_PARITY_RECORD_KIND, TEST_RUNNER_BETA_SCHEMA_VERSION,
    TEST_RUNNER_BETA_SUPPORT_EXPORT_RECORD_KIND, TEST_RUNNER_BETA_TREE_PROJECTION_RECORD_KIND,
    TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND,
};
pub use testing_identity::{
    CanonicalTestAttempt, CanonicalTestItem, CanonicalTestItemKind, CanonicalTestSession,
    ImportedCiTruthClass, ImportedCiTruthOverlay, TestAdapterKind, TestAttemptLineageClass,
    TestEvidenceClass, TestIdentityBetaBundle, TestIdentityLedgerError, TestIdentitySupportExport,
    TestIdentitySurface, TestItemIdentityClass, TestResultFreshnessClass, TestSelectionOrigin,
    TestSelectorBinding, TestSurfaceIdentityBinding, TestTargetEnvironmentClass,
    TestTargetEnvironmentIdentity, CANONICAL_TEST_ATTEMPT_RECORD_KIND,
    CANONICAL_TEST_ITEM_RECORD_KIND, CANONICAL_TEST_SESSION_RECORD_KIND,
    IMPORTED_CI_TRUTH_OVERLAY_RECORD_KIND, TEST_IDENTITY_BETA_BUNDLE_RECORD_KIND,
    TEST_IDENTITY_BETA_SCHEMA_VERSION, TEST_IDENTITY_SUPPORT_EXPORT_RECORD_KIND,
    TEST_SELECTOR_BINDING_RECORD_KIND, TEST_SURFACE_IDENTITY_BINDING_RECORD_KIND,
};
pub use testing_quality::{
    BaselineTruthPacket, CoverageTruthPacket, FlakyTruthPacket, SnapshotTruthPacket,
    TestQualityBetaCoverageManifest, TestQualityBetaCoverageRow, TestQualityBetaSupportExport,
    TestQualityFreshness, TestQualityKind, TestQualityPacketIdentity, TestQualityProjection,
    TestQualityProvenanceSource, TestQualityRowTruth, TestQualitySupportClass,
    TEST_QUALITY_BASELINE_PACKET_RECORD_KIND, TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TEST_QUALITY_BETA_PROJECTION_RECORD_KIND, TEST_QUALITY_BETA_SUPPORT_EXPORT_RECORD_KIND,
    TEST_QUALITY_COVERAGE_PACKET_RECORD_KIND, TEST_QUALITY_FLAKY_PACKET_RECORD_KIND,
    TEST_QUALITY_ROW_TRUTH_RECORD_KIND, TEST_QUALITY_SNAPSHOT_PACKET_RECORD_KIND,
    TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
};
pub use testing_triage::{
    FlakyVerdictAttemptInput, FlakyVerdictPacket, SnapshotFileChangePreview,
    SnapshotMutationReview, SnapshotMutationReviewState, TestEvidenceTrustClass,
    TestQuarantineReason, TestQuarantineRecord, TestQuarantineReopenBehavior,
    TestQuarantineScopeClass, TestQuarantineStatus, TestQuarantineTreatmentKind,
    TestReleaseDebtClass, TestTriageIdentity, TestTrustPacket, TestTrustRowSummary,
    WatchModeDowngradeReason, WatchModeState, WatchStatePacket, FLAKY_VERDICT_PACKET_RECORD_KIND,
    SNAPSHOT_MUTATION_REVIEW_RECORD_KIND, TEST_QUARANTINE_RECORD_KIND,
    TEST_TRIAGE_TRUST_SCHEMA_VERSION, TEST_TRUST_PACKET_RECORD_KIND,
    WATCH_STATE_PACKET_RECORD_KIND,
};
pub use tests::{
    AiTestGenerationGateState, CoverageMergeClass, FlakyVerdictState, ImportedCiProjection,
    ImportedCiProjectionClass, ImportedSignalAuthority, TestAttemptAlphaOptions,
    TestAttemptAlphaPacket, TestAttemptKind, TestAttemptRecord, TestAttemptResultState,
    TestAttemptSupportExport, TestConsumerSurface, TestIdentityStability,
    TestItemIdentityProjection, TestLaunchWedgeProjection, TestSessionMode, TestSessionPlan,
    TestSourceDriftState, TestStabilityVerdict, TestWatchController, TestWatchDegradationReason,
    TestWatchState, IMPORTED_CI_PROJECTION_RECORD_KIND, TEST_ATTEMPT_ALPHA_PACKET_RECORD_KIND,
    TEST_ATTEMPT_ALPHA_SCHEMA_VERSION, TEST_ATTEMPT_RECORD_KIND,
    TEST_ATTEMPT_SUPPORT_EXPORT_RECORD_KIND, TEST_ITEM_IDENTITY_PROJECTION_RECORD_KIND,
    TEST_LAUNCH_WEDGE_PROJECTION_RECORD_KIND, TEST_SESSION_PLAN_RECORD_KIND,
    TEST_STABILITY_VERDICT_RECORD_KIND, TEST_WATCH_CONTROLLER_RECORD_KIND,
};
pub use topology_inspector::{
    seeded_host_lanes, seeded_host_topology_inspector, seeded_lane_filtered_event_viewer,
    seeded_reattach_review_sheet, CrashLoopQuarantineBanner, FaultDomainClass,
    FaultDomainNextSafeActionClass, FaultDomainRestartCard, HostBadgeGroup, HostBoundaryBadge,
    HostBoundaryBadgeClass, HostDetailAction, HostDetailOpenTarget, HostLaneFamily,
    HostLaneHealthClass, HostLaneRecord, HostLaneSeed, HostResultFreshnessClass, LaneEventRow,
    LaneFilteredEventViewer, ReattachDriftFieldClass, ReattachDriftRow, ReattachReplayRiskClass,
    ReattachReviewDecisionClass, ReattachReviewInput, ReattachReviewSheet, RerunRequirementClass,
    RestartBudgetStateClass, RestartMarkerClass, RuntimeResultSeed, RuntimeSurfaceClass,
    RuntimeSurfaceResult, TopologyInspectorRecord, TopologyInspectorViolation,
    VisibleTruthLabelClass, CRASH_LOOP_QUARANTINE_BANNER_RECORD_KIND,
    FAULT_DOMAIN_RESTART_CARD_RECORD_KIND, HOST_BADGE_GROUP_RECORD_KIND, HOST_LANE_RECORD_KIND,
    HOST_TOPOLOGY_SCHEMA_VERSION, LANE_FILTERED_EVENT_VIEWER_RECORD_KIND,
    REATTACH_REVIEW_SHEET_RECORD_KIND, TOPOLOGY_INSPECTOR_RECORD_KIND,
};
pub use trace_replay_alpha::{
    BuildRuntimeIdentity, CaptureMode, CaptureSource, CaptureWindow, ComparisonClass,
    ComparisonClassAlphaPacket, ComparisonRuntimeToolchain, ComparisonSourceClass,
    DerivedTraceView, DerivedViewKind, DigestAlgorithm, DigestEntry, HardwarePowerProfile,
    MappingQualityState, MappingQualitySummary, OverheadClass, ProfileCaptureDescriptor,
    ProfileExportPolicy, ProfileSessionAlpha, ProfileTargetIdentity, RawTraceBundle,
    ReplayBackendIdentity, ReplayCapabilityAlphaDescriptor, ReplayExportPosture,
    ReplayFeatureState, ReplayFeatureSupport, ReplayLaneState, ReplayOverheadStorageBand,
    ReplayRuntimeToolchainRange, ReplaySupportMatrix, RuntimeEvidenceAlphaPacket,
    RuntimeEvidenceDataClass, RuntimeEvidenceDataPosture, RuntimeEvidenceSupportExport,
    TraceBundleAlphaManifest, TraceBundleImmutability, TraceBundleRedaction, TraceBundleRetention,
    TraceMetricFamily, TraceRedactionMode, TraceRetentionClass, VarianceWindow,
    COMPARISON_CLASS_ALPHA_RECORD_KIND, PROFILE_SESSION_ALPHA_RECORD_KIND,
    REPLAY_CAPABILITY_ALPHA_RECORD_KIND, RUNTIME_EVIDENCE_ALPHA_PACKET_RECORD_KIND,
    RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION, RUNTIME_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND,
    SUPPORT_ITEM_RUNTIME_TRACES, TRACE_BUNDLE_ALPHA_RECORD_KIND,
};
