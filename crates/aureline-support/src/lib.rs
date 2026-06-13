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
//! - The [`records_policy_governance`] support export — joins the checked-in
//!   records-governance matrix to the stable policy snapshot so support packets
//!   can prove that durable managed/provider/support artifacts share one
//!   record-class, chronology, deletion, export, and policy-governance source.
//! - The [`records_export_delete_governance`] support export — consumes the
//!   canonical export-job, request-case, delete-case, manifest, and receipt
//!   packet so support exports can inspect partial, blocked, policy-retained,
//!   redacted, and outside-scope outcomes directly instead of reading prose.
//! - The [`m5_records_policy_governance`] support export — joins the canonical
//!   M5 legal-hold/retention packet to the policy exception/expiry packet so
//!   support exports can inspect hold notices, hold selector scopes, retention
//!   owners, archive state, and the pre-action delete/export truth, and prove
//!   every hold/retention claim is gated by a live, bounded, actor-scoped
//!   exception instead of indefinite waiver prose.
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
//! - The [`m5_fault_crash_governance`] packet — freezes the canonical M5
//!   fault-domain classes, restart classes, crash-artifact vocabulary,
//!   diagnostics-schema governance rows, and downgrade rules used by notebook,
//!   data/API, preview, provider, profiler/replay, pipeline, docs/browser,
//!   and infrastructure helper hosts.
//! - The [`m5_forensic_packet`] packet — projects runtime forensic packets into
//!   one support-side export-safe object that keeps locality (`local_only`,
//!   `imported`, `mirrored`, `uploaded`), checkpoint lineage, exact-build
//!   identity, redaction posture, and reviewed share actions explicit.
//! - The [`m5_host_failure_drills`] packet — freezes the seeded M5 host-failure
//!   drill corpus and proves restart-budget enforcement, scoped failure,
//!   checkpoint preservation, no-hidden-rerun behavior, and no-silent-upload
//!   posture across every claimed host family.
//! - The [`m5_fault_crash_certification`] packet — certifies the claimed M5
//!   host families on each claimed profile by binding one shared row to restart,
//!   crash, symbolication, schema-governance, and field-readiness proof, then
//!   exposing the same certification index to Help/About, service-health,
//!   support-export, and release-manifest consumers.
//! - The [`m5_fs_mutation_lineage_certification`] packet — certifies the
//!   claimed M5 filesystem-identity, watch/save, mutation-lineage,
//!   state-class-recovery, and deferred-intent rows so Help, diagnostics,
//!   support-bundle, and release-center surfaces read one narrowing packet.
//! - The [`schema_registry`] packet — freezes the M5 depth-surface telemetry,
//!   diagnostics, consent-ledger inheritance, endpoint truth, and
//!   redaction-default packet classes used by notebook, provider, profiler,
//!   pipeline, preview, and data surfaces so support export stays explicit and
//!   local-first.
//! - The [`crash_store`] packet — binds crash-envelope, dump/core metadata,
//!   exact-build identity, redaction posture, and local preview/export/upload
//!   actions into one local-first crash-store viewer for M5 host families.
//! - The [`recovery_review`] packet — composes crash-loop center decisions,
//!   scoped reset / reattach comparisons, quarantine and rollback reviews, and
//!   bounded continuity rows into one metadata-only recovery-review export for
//!   M5 host families.
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
pub mod crash_store;
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
pub mod m5_fault_crash_certification;
pub mod m5_fault_crash_governance;
pub mod m5_forensic_packet;
pub mod m5_fs_mutation_lineage_certification;
pub mod m5_host_failure_drills;
pub mod m5_mutation_lineage;
pub mod m5_records_policy_governance;
pub mod mutation_journal;
pub mod policy_simulation;
pub mod portable_bundle_handoff;
pub mod project_doctor;
pub mod publication_dry_run;
pub mod publish_supportability_runbooks_field_playbooks_and_incident_advisory;
pub mod records_export_delete_governance;
pub mod records_policy_governance;
pub mod recovery_ladder;
pub mod recovery_review;
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
pub mod schema_registry;
pub mod stabilize_dashboard_queue_and_followup_bundle_truth;
pub mod stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded;
pub mod stabilize_runbook_source_step_envelope_and_handoff_truth;
pub mod stabilize_support_bundle_generation_with_redaction_default_manifests;
pub mod stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows;
pub mod state_class_recovery;
pub mod storage_inspector;
pub mod supervised_restart_evidence_pipeline;

pub use crash_store::{
    seeded_crash_store_viewer_packet, seeded_expired_dump_crash_store_viewer_packet,
    CrashPreservationClass, CrashStoreActionClass, CrashStoreActionRow,
    CrashStoreRedactionPostureClass, CrashStoreViewerPacket, CrashStoreViewerRow,
    CrashStoreViewerViolation, CRASH_STORE_VIEWER_ARTIFACT_REF, CRASH_STORE_VIEWER_DOC_REF,
    CRASH_STORE_VIEWER_FIXTURE_DIR, CRASH_STORE_VIEWER_PACKET_RECORD_KIND,
    CRASH_STORE_VIEWER_ROW_RECORD_KIND, CRASH_STORE_VIEWER_SCHEMA_REF,
    CRASH_STORE_VIEWER_SCHEMA_VERSION,
};
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
    seeded_fault_domain_view_packet, FaultDomainTopologyResultHostSummary,
    FaultDomainTopologyResultRow, FaultDomainViewPacket, FaultDomainViewRow,
    FaultDomainViewViolation, VisibleTruthResultRow, FAULT_DOMAIN_TOPOLOGY_RESULT_RECORD_KIND,
    FAULT_DOMAIN_VIEW_ARTIFACT_REF, FAULT_DOMAIN_VIEW_DOC_REF,
    FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND, FAULT_DOMAIN_VIEW_ROW_RECORD_KIND,
    FAULT_DOMAIN_VIEW_SCHEMA_REF,
};
pub use locale_beta::current_locale_pack_support_export;
pub use m5_fault_crash_certification::{
    seeded_m5_fault_crash_certification_packet,
    seeded_stale_schema_m5_fault_crash_certification_packet,
    seeded_stale_symbolication_m5_fault_crash_certification_packet, CertificationDowngradeRuleRow,
    CertificationDowngradeTriggerClass, CertificationStateClass, CertificationSurfaceBinding,
    CertificationSurfaceClass, ClaimedM5Profile, DiagnosticsGovernancePostureClass,
    HostProfileCertificationRow, M5FaultCrashCertificationPacket,
    M5FaultCrashCertificationViolation, SymbolicationPostureClass,
    M5_FAULT_CRASH_CERTIFICATION_ARTIFACT_REF, M5_FAULT_CRASH_CERTIFICATION_DOC_REF,
    M5_FAULT_CRASH_CERTIFICATION_FIXTURE_DIR, M5_FAULT_CRASH_CERTIFICATION_PACKET_RECORD_KIND,
    M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF, M5_FAULT_CRASH_CERTIFICATION_SCHEMA_VERSION,
};
pub use m5_fault_crash_governance::{
    seeded_m5_fault_crash_governance_packet, CheckpointSourceClass, ClaimStateClass,
    CrashArtifactClass, CrashArtifactGovernanceRow, DiagnosticDataClass, DiagnosticOptInScope,
    DiagnosticSchemaGovernanceRow, DiagnosticSignalClass, DowngradeRuleRow, DowngradeTriggerClass,
    FaultDomainClass, FaultDomainMatrixRow, HostFamilyGovernanceRow, M5FaultCrashGovernancePacket,
    M5FaultCrashGovernanceViolation, RedactionProfileClass, RestartClass, RestartClassRow,
    RetentionClass, M5_FAULT_CRASH_GOVERNANCE_ARTIFACT_REF, M5_FAULT_CRASH_GOVERNANCE_DOC_REF,
    M5_FAULT_CRASH_GOVERNANCE_FIXTURE_DIR, M5_FAULT_CRASH_GOVERNANCE_PACKET_RECORD_KIND,
    M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF, M5_FAULT_CRASH_GOVERNANCE_SCHEMA_VERSION,
};
pub use m5_forensic_packet::{
    seeded_m5_forensic_packet, ForensicArtifactClass, ForensicArtifactStateClass,
    ForensicArtifactStateRow, ForensicShareActionRow, ForensicShareDestinationClass,
    ForensicTriggerClass, M5ForensicPacket, M5ForensicPacketViolation, M5ForensicRow,
    M5_FORENSIC_PACKET_ARTIFACT_REF, M5_FORENSIC_PACKET_DOC_REF, M5_FORENSIC_PACKET_FIXTURE_DIR,
    M5_FORENSIC_PACKET_RECORD_KIND, M5_FORENSIC_PACKET_SCHEMA_REF,
    M5_FORENSIC_PACKET_SCHEMA_VERSION,
};
pub use m5_fs_mutation_lineage_certification::{
    seeded_m5_fs_mutation_lineage_certification_packet,
    seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet,
    CertificationDowngradeRuleRow as FsMutationLineageCertificationDowngradeRuleRow,
    CertificationDowngradeTriggerClass as FsMutationLineageCertificationDowngradeTriggerClass,
    CertificationStateClass as FsMutationLineageCertificationStateClass,
    CertificationSurfaceBinding as FsMutationLineageCertificationSurfaceBinding,
    CertificationSurfaceClass as FsMutationLineageCertificationSurfaceClass,
    DiagnosticsProjectionRow as FsMutationLineageDiagnosticsProjectionRow,
    FixtureVariantClass as FsMutationLineageCertificationFixtureVariantClass,
    HelpProjectionRow as FsMutationLineageHelpProjectionRow, M5FsMutationLineageCertificationError,
    M5FsMutationLineageCertificationPacket, M5FsMutationLineageCertificationViolation,
    M5FsMutationLineageDiagnosticsExportProjection, M5FsMutationLineageHelpSurfaceProjection,
    M5FsMutationLineageReleaseCenterProjection, M5FsMutationLineageSupportBundleProjection,
    ReleaseCenterProjectionRow as FsMutationLineageReleaseCenterProjectionRow,
    SupportBundleProjectionRow as FsMutationLineageSupportBundleProjectionRow,
    SurfaceCertificationRow as FsMutationLineageSurfaceCertificationRow,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF, M5_FS_MUTATION_LINEAGE_CERTIFICATION_FIXTURE_DIR,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_PACKET_RECORD_KIND,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_VERSION,
    M5_FS_MUTATION_LINEAGE_CERTIFICATION_SUMMARY_REF,
};
pub use m5_host_failure_drills::{
    seeded_forensic_row, seeded_m5_host_failure_drill_packet, HostFailureDrillRow,
    HostFailureScenarioClass, M5HostFailureDrillPacket, M5HostFailureDrillViolation,
    M5_HOST_FAILURE_DRILL_ARTIFACT_REF, M5_HOST_FAILURE_DRILL_DOC_REF,
    M5_HOST_FAILURE_DRILL_FIXTURE_DIR, M5_HOST_FAILURE_DRILL_PACKET_RECORD_KIND,
    M5_HOST_FAILURE_DRILL_SCHEMA_REF, M5_HOST_FAILURE_DRILL_SCHEMA_VERSION,
};
pub use m5_mutation_lineage::{
    compile_support_export_envelope as compile_m5_mutation_lineage_support_export_envelope,
    M5MutationLineageSupportExportEnvelope, M5MutationLineageSupportExportError,
    M5MutationLineageSupportExportRow, M5_MUTATION_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND,
    M5_MUTATION_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND,
};
pub use m5_records_policy_governance::{
    M5RecordsPolicyGovernanceSupportExport, M5RecordsPolicyGovernanceViolation,
    M5_RECORDS_POLICY_GOVERNANCE_RECORD_KIND, M5_RECORDS_POLICY_GOVERNANCE_SCHEMA_VERSION,
};
pub use recovery_review::{
    seeded_recovery_review_packet, CrashLoopReviewRow, QuarantineReviewRow, RecoveryContinuityRow,
    RecoveryReviewPacket, RecoveryReviewViolation, ScopedResetReviewRow,
    CRASH_LOOP_REVIEW_ROW_RECORD_KIND, QUARANTINE_REVIEW_ROW_RECORD_KIND,
    RECOVERY_CONTINUITY_ROW_RECORD_KIND, RECOVERY_REVIEW_ARTIFACT_REF, RECOVERY_REVIEW_DOC_REF,
    RECOVERY_REVIEW_FIXTURE_DIR, RECOVERY_REVIEW_PACKET_RECORD_KIND, RECOVERY_REVIEW_SCHEMA_REF,
    RECOVERY_REVIEW_SCHEMA_VERSION, SCOPED_RESET_REVIEW_ROW_RECORD_KIND,
};
pub use route_exposure_beta::{
    audit_route_exposure_matrix, current_route_exposure_matrix, validate_route_exposure_matrix,
    RouteExposureFinding, RouteExposureMatrix, RouteExposureSupportExport,
    ROUTE_EXPOSURE_MATRIX_PATH, ROUTE_EXPOSURE_MATRIX_RECORD_KIND,
    ROUTE_EXPOSURE_MATRIX_SCHEMA_PATH, ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION,
    ROUTE_EXPOSURE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use schema_registry::{
    seeded_depth_surface_schema_registry_packet, ConsentLedgerBindingRow, ConsentStateClass,
    DepthSignalClass, DepthSurfaceClass, DepthSurfaceSchemaRegistryPacket,
    DepthSurfaceSchemaRegistryViolation, EndpointStateClass, PacketClassManifestRow,
    SchemaDeclarationRow, SurfaceInspectionRow, DEPTH_SCHEMA_REGISTRY_ARTIFACT_REF,
    DEPTH_SCHEMA_REGISTRY_DOC_REF, DEPTH_SCHEMA_REGISTRY_FIXTURE_DIR,
    DEPTH_SCHEMA_REGISTRY_PACKET_RECORD_KIND, DEPTH_SCHEMA_REGISTRY_SCHEMA_REF,
    DEPTH_SCHEMA_REGISTRY_SCHEMA_VERSION,
};
pub use state_class_recovery::{
    compile_support_export_envelope as compile_state_class_recovery_support_export_envelope,
    StateClassRecoverySupportExportEnvelope, StateClassRecoverySupportExportError,
    StateClassRecoverySupportExportRow, STATE_CLASS_RECOVERY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND,
    STATE_CLASS_RECOVERY_SUPPORT_EXPORT_ROW_RECORD_KIND,
};
pub use supervised_restart_evidence_pipeline::{
    seeded_supervised_restart_evidence_packet, FaultDomainRestartSummary, HostLaneIdentityRecord,
    NoRerunPolicyClass, NoRerunPolicyRecord, RestartBudgetToken, RestartDomainClass,
    RestartLineageEntry, RestartTriggerClass, SupervisedRestartDecisionClass,
    SupervisedRestartEvidencePacket, SupervisedRestartReviewDecision, SupervisedRestartViolation,
    FAULT_DOMAIN_RESTART_SUMMARY_RECORD_KIND, HOST_LANE_IDENTITY_RECORD_KIND,
    NO_RERUN_POLICY_RECORD_KIND, RESTART_LINEAGE_ENTRY_RECORD_KIND,
    SUPERVISED_RESTART_EVIDENCE_PACKET_RECORD_KIND, SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF,
    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF,
    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
    SUPERVISED_RESTART_REVIEW_DECISION_RECORD_KIND,
};
