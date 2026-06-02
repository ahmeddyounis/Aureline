//! Support-bundle manifest, redaction defaults, local preview, and exact-build capture.
//!
//! This crate is the live shell's first trustworthy support-export path. It
//! mints a structured support-bundle manifest with redaction defaults, exact-
//! build identity, and a local preview that the chrome can show before any
//! share or upload step.
//!
//! ## What this seed owns
//!
//! - The [`bundle::SupportBundleManifest`] record — the canonical truth
//!   model for what a bundle would contain on export. It mirrors the
//!   boundary schema at
//!   `/schemas/support/support_bundle_manifest.schema.json` so the chrome
//!   and the export writer never invent divergent shapes.
//! - The [`bundle::SupportBundlePreviewItem`] row — one inspectable line in
//!   the local preview. Mirrors
//!   `/schemas/support/support_bundle_preview_item.schema.json`.
//! - The [`bundle::redaction::LocalFirstDefaults`] profile — the default
//!   redaction posture for a local-first export. Mirrors the seed profile
//!   at `/fixtures/support/redaction_profiles/local_first_default.yaml`.
//! - The [`bundle::ExactBuildCapture`] join — quotes
//!   [`aureline_build_info::build_identity`] and
//!   [`aureline_build_info::release_channel_class`] verbatim so the
//!   manifest carries the same exact-build identity as the running binary.
//! - The [`bundle::SupportBundlePreview`] projection — the read-only view
//!   the shell renders before letting the user export.
//!
//! ## What this seed does NOT own
//!
//! - Live byte-level redaction implementation, upload transport, hosted
//!   intake, or ticket routing. Those belong to later milestones.
//! - The full diagnostic_artifact_matrix item set. The seed surfaces the
//!   minimum row classes needed to prove the protected walk and the
//!   failure drill (metadata + secret-bearing prohibited).
//! - Live Project Doctor probe ownership. The [`project_doctor`] module
//!   consumes the checked-in read-only alpha probe pack and the runtime
//!   support projection from `aureline-doctor`, but it does not apply repairs.
//! - The [`recovery_ladder`] alpha evaluator — consumes typed recovery
//!   evidence and emits bounded safe-mode, quarantine, open-without-restore,
//!   and cache/index repair decisions plus support/release projections.
//! - The [`safe_mode`] beta evaluator — mints a typed
//!   `safe_mode_profile_record` that declares which hosts, services, and
//!   surfaces are disabled or narrowed and why, plus typed
//!   `safe_mode_transition_record` rows for entry and exit that preserve
//!   user-owned state. Bound to the boundary schema at
//!   `/schemas/support/safe_mode_profile.schema.json` and the protected
//!   fixture corpus at `/fixtures/recovery/m3/safe_mode/`.
//! - The [`extension_bisect`] beta evaluator — mints typed
//!   `extension_bisect_session_record`, `extension_bisect_step_record`,
//!   `extension_bisect_finding_record`, and `extension_bisect_restore_record`
//!   rows that log tested states, suspected extension sets, user-visible
//!   findings, and the restore of the prior extension state plus a
//!   metadata-safe support packet. Bound to the boundary schema at
//!   `/schemas/support/extension_bisect.schema.json` and the protected
//!   fixture corpus at `/fixtures/recovery/m3/extension_bisect/`.
//! - The [`crash_loop_center`] beta evaluator — consumes a typed
//!   `crash_loop_signal_record` describing a restart-budget breach and
//!   synthesizes a `crash_loop_recovery_center_record` that routes the
//!   blocked user into bounded, command-backed recovery choices (Safe mode,
//!   Open without restore, Disable recently changed extension, Disable
//!   recently changed profile/layout, Open logs, Export crash manifest,
//!   Report issue) plus distinct evidence-only and checkpoint/diff entry
//!   points. Crash id, build id, restore class, and suspected fault domain
//!   stay visible; Safe mode and Open without restore honor no-silent-rerun
//!   semantics for privileged or mutating sessions; and no choice deletes
//!   user-owned state. Bound to the boundary schema at
//!   `/schemas/support/crash_loop_recovery.schema.json` and the protected
//!   fixture corpus at `/fixtures/recovery/m3/crash_loop_center/`.
//! - The [`repair`] alpha preview compiler — consumes checked-in repair seed
//!   cases and emits typed transaction, preview, outcome, and journal records
//!   before any guided repair can apply.
//! - The [`repair_transactions`] beta preview-skeleton evaluator — mints
//!   typed `repair_preview_skeleton_record` and
//!   `repair_preview_comparison_record` rows that declare blast-radius
//!   class, compensation class, affected object classes, checkpoint
//!   disposition, and a cancellable comparison disposition bound to the
//!   alpha transaction id and reversal class, plus a metadata-safe support
//!   packet. Bound to the boundary schema at
//!   `/schemas/support/repair_transaction_preview_skeleton.schema.json` and
//!   the protected fixture corpus at
//!   `/fixtures/recovery/m3/repair_transaction_preview/`.
//! - The [`advisory_baseline`] support projection — consumes the checked-in
//!   affected-build scope example and projects advisory, exact-build,
//!   rollback, known-limit, and support refs into metadata-only support rows.
//! - The [`release_evidence`] alpha projection — consumes the checked-in
//!   artifact graph and reconstructs release-center candidate, target,
//!   digest-set, rollout, auth-source, rollback, and trust-domain fields for
//!   metadata-only support/export review.
//! - The [`publication_dry_run`] alpha projection — consumes the checked-in
//!   publication manifest and exposes clean-room, mirror-only, deny-all,
//!   offline, notice, SBOM, provenance, blocker, and live-truth degradation
//!   state for metadata-only support/export review.
//! - The [`distributed_compatibility`] beta projection — consumes the generated
//!   distributed compatibility support export so support packets quote the same
//!   manifest rows, skew cases, unsupported postures, and repair hints as the
//!   release packet.
//! - The [`reproducible_rc`] beta projection — consumes the generated
//!   reproducible release-candidate support export so release, security,
//!   partner proof, and support packets quote the same clean-room rebuild
//!   state, exact-build identity, rebuilt artifact graph checks, and blocking
//!   publication checks.
//! - The [`bundle::notice_digest_preview_item_seed`] projection — consumes the
//!   typed `aureline-notices` bundle and inserts the dependency notice digest
//!   into support-bundle previews as metadata-only evidence.
//! - The [`route_origin_alpha`] projection — consumes the checked-in
//!   route-origin matrix, transport-decision fixtures, and reconstruction
//!   packet so support/export previews can rebuild command, target, route,
//!   traffic-origin, policy, outcome, and fallback truth.
//! - The [`route_exposure_beta`] projection — consumes the checked-in
//!   route/exposure matrix so Help/About, service health, diagnostics,
//!   release evidence, and support exports quote the same origin, target,
//!   route, exposure, approval-reuse, reapproval-trigger, privacy-consequence,
//!   and browser/system handoff vocabulary.
//! - The [`bundle::records`] records-governance projection — consumes the
//!   record-class registry alongside typed governance inputs and emits one
//!   typed records-governance packet per artifact so support exports
//!   carry artifact-class, hold-state, retention-owner, chain-of-custody,
//!   and destruction-caveat truth instead of implying it.
//! - The [`bundle::deletion_and_hold`] projection — adds held-record
//!   selectors, stable deletion-honesty labels, and metadata-only
//!   destruction receipt rows to the same support-bundle preview path.
//! - The [`bundle::evidence_timeline`] projection — exports delete-request,
//!   queue, hold, retained-evidence, and completion chronology with source
//!   timezone and actor ordering preserved for support/operator packets.
//! - The [`local_history_timeline`] support projection — consumes the
//!   checked-in local-history timeline corpus and emits metadata-only support
//!   rows that quote the same exact, compatible, layout-only, and evidence-only
//!   fidelity labels as the timeline and restore-preview surfaces.
//! - The [`refactor_preview`] support projection — consumes the checked-in
//!   launch-language refactor preview corpus and emits metadata-only support
//!   rows for green, downgraded, and unsupported semantic-change claims,
//!   including fallback labels and grouped rollback refs.
//! - The [`portable_bundle_handoff`] support projection — folds portable
//!   change bundle and shelf fixtures into one metadata-safe handoff envelope
//!   with target identity, stale-validation labels, reopen modes, redaction
//!   posture, and support-export lineage preserved.
//! - The [`export_review`] default-redacted profile and reopen manifest —
//!   consume the checked-in profile corpus and reopen-manifest corpus and
//!   project the inspectable [`export_review::EscalationPacketReview`] surface
//!   so support-bundle and incident-export emitters can prove exact-build
//!   identity, scenario family, doctor finding codes, repair history,
//!   crash manifest refs, and symbolication report refs stay carried by
//!   reference; raw dumps, raw transcripts, code-adjacent attachments, and
//!   secret-bearing material never embed by default; the local-only path
//!   stays at equal prominence; and the reopen manifest preserves the
//!   included/excluded class lists, build identity, and destination class.
//!
//! ## Failure-drill posture
//!
//! When a caller queues a row whose contents would carry secret-bearing
//! material, the manifest forces the row's redaction state to `prohibited`,
//! emits an [`bundle::ExcludedClass`] entry with an explicit reason, and
//! adds the support-pack item id to the manifest's
//! `prohibited_items_confirmed_absent` list. The protected-walk preview
//! never exports raw secret bytes even if a caller mis-classifies them.

#![doc(html_root_url = "https://docs.rs/aureline-support/0.0.0")]

pub mod advisory_baseline;
pub mod bundle;
pub mod capabilities;
pub mod crash_loop_and_restore_fidelity;
pub mod crash_loop_center;
pub mod distributed_compatibility;
pub mod export_review;
pub mod extension_bisect;
pub mod fault_domain_views;
pub mod field_readiness;
pub mod finalize_support_center_surfaces_performance_inspector_language_service;
pub mod finalize_typed_repair_transaction_preview_checkpoint_rollback_and;
pub mod fitness;
pub mod generated_lineage;
pub mod graph_drift;
pub mod harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets;
pub mod harden_the_safe_mode_runtime_profile_retained_capabilities;
pub mod incident_workspace;
pub mod incident_workspace_beta;
pub mod local_history_timeline;
pub mod locale_beta;
pub mod m3_scenario_corpus;
pub mod mutation_journal;
pub mod policy_simulation;
pub mod portable_bundle_handoff;
pub mod project_doctor;
pub mod publication_dry_run;
pub mod recovery_ladder;
pub mod refactor_preview;
pub mod release_evidence;
pub mod repair;
pub mod repair_transactions;
pub mod reproducible_rc;
pub mod route_exposure_beta;
pub mod route_origin_alpha;
pub mod runtime_evidence;
pub mod runtime_health_alpha;
pub mod safe_mode;
pub mod scenario_scorecard;
pub mod stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded;
pub mod stabilize_support_bundle_generation_with_redaction_default_manifests;
pub mod stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows;
pub mod storage_inspector;

pub use export_review::{
    current_escalation_packet_reviews, load_profile_corpus, load_reopen_corpus,
    BroadenEvidenceReviewClass, BuildIdentityBlock, CrashLinkageBlock, DataClassBoundaryClass,
    DefaultRequiredEvidenceClass, DestinationClass, DestinationPostureBlock,
    EscalationPacketReview, EvidenceClass, EvidenceClassRuleRow, EvidenceInclusionClass,
    GovernanceBindingsBlock, PlatformClass, ReleaseChannelClass as ExportReleaseChannelClass,
    ReopenPathBlock, ScenarioFamilyClass, SupportExportRedactionError,
    SupportExportRedactionProfile, SupportExportReopenManifest, REQUIRED_EVIDENCE_CLASSES,
    SUPPORT_EXPORT_REDACTION_PROFILE_CORPUS_DIR, SUPPORT_EXPORT_REDACTION_PROFILE_DOC_REF,
    SUPPORT_EXPORT_REDACTION_PROFILE_RECORD_KIND, SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_REF,
    SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_VERSION,
    SUPPORT_EXPORT_REDACTION_PROFILE_SEED_CASE_RECORD_KIND,
    SUPPORT_EXPORT_REOPEN_MANIFEST_RECORD_KIND,
    SUPPORT_EXPORT_REOPEN_MANIFEST_SEED_CASE_RECORD_KIND,
};
pub use fault_domain_views::{
    seeded_fault_domain_view_packet, FaultDomainViewPacket, FaultDomainViewRow,
    FaultDomainViewViolation, FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND,
    FAULT_DOMAIN_VIEW_ROW_RECORD_KIND,
};
pub use locale_beta::current_locale_pack_support_export;
pub use route_exposure_beta::{
    audit_route_exposure_matrix, current_route_exposure_matrix, validate_route_exposure_matrix,
    RouteExposureFinding, RouteExposureMatrix, RouteExposureSupportExport,
    ROUTE_EXPOSURE_MATRIX_PATH, ROUTE_EXPOSURE_MATRIX_RECORD_KIND,
    ROUTE_EXPOSURE_MATRIX_SCHEMA_PATH, ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION,
    ROUTE_EXPOSURE_SUPPORT_EXPORT_RECORD_KIND,
};
