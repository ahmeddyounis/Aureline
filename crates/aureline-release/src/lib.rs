//! Release engineering contracts shared by UI, headless, support, and audit flows.
//!
//! This crate owns release-object types that must stay independent of any
//! single renderer, CI script, or support export. The first module is the
//! release-center object model: release candidates, version-bump proposals,
//! publish targets, artifact bundles, promotion steps, and scoped
//! rollback/revocation records. The correction-train module formalizes the
//! shared correction-train, hotfix, and backport packet form on top of the
//! same rollback and release-candidate refs. The stable-claim-matrix module
//! freezes the stable claim matrix, launch cutline, qualification rows, and
//! shiproom stop rules that decide which surfaces may publish as Stable. The
//! support-class-ledger module is the publication layer on top of that matrix:
//! it publishes the v1.0 support-class assignments, the certified-archetype
//! manifest, and the downgrade automation that narrows a published support
//! class when its backing thins out. The stable-qualification-matrix module
//! finalizes the per-lane qualification rows (desktop, remote/helper,
//! ecosystem, state/schema, provider, accessibility) that ground those claims
//! and, for every cross-binary or cross-service boundary, publishes the
//! mixed-version section — negotiated fields, supported skew window, upgrade and
//! rollback order, and unsupported-state behavior — that decides whether the
//! boundary may inherit a Stable mixed-version claim or is coordinated-upgrade-only.
//! The stable-claim-manifest module is the publication layer that binds all
//! three of those records together: it assigns each published subject one
//! canonical lifecycle label, names the backing claim row, qualification rows, and
//! support-class entry that label depends on, and attaches a packet-freshness SLO
//! so a subject whose proof packet has breached its SLO narrows below the launch
//! cutline automatically before publication. The stable-boundary-manifest module
//! is the deployment-boundary layer on top of that manifest: for every published
//! subject it records, across the local-OSS, self-hosted, managed, and air-gapped
//! value lines, the lifecycle label each line can carry — never wider than the
//! subject's canonical manifest label — so a value line that lacks a capability,
//! whose line evidence is incomplete, or whose proof packet aged out narrows below
//! the cutline before publication while still ingesting the one canonical label.
//! The stable-proof-index module is the requirement-facing layer that closes the
//! loop: for every launch-blocking requirement it records one row binding the
//! requirement to the proof packet that proves it, the waiver (if any) holding it
//! provisionally, and the public claim (a stable-claim-manifest entry) whose
//! lifecycle label that proof backs — never wider than the claim's canonical label
//! — so a requirement whose proof packet aged out or is missing, whose waiver
//! expired, whose requirement evidence is incomplete, or whose backing public claim
//! is itself below the cutline narrows below the launch cutline and holds
//! publication, while the launch-blocking requirement set stays fully covered.
//! The stable-version-windows module is the interface-freeze layer alongside that
//! index: for every public interface surface — a CLI command surface, a wire/state
//! schema, an API, or a manifest format — it freezes the stable version window
//! (floor, current, ceiling, compatibility posture) and the deprecation packet that
//! governs how older versions leave the window, backs each surface against a public
//! claim whose canonical label is a hard ceiling, and narrows below the cutline any
//! surface whose freeze packet aged out or is missing, whose deprecation packet is
//! incomplete or carries an overdue removal, whose waiver expired, whose surface
//! evidence is incomplete, or whose backing public claim is itself below the cutline
//! — while the CLI/schema/API/manifest surface kinds and the release-line surface
//! set both stay fully covered.
//! The maintenance-control-packet module is the post-release maintenance layer that
//! sits alongside those freezes: for every maintenance lane — an emergency hotfix lane,
//! a supported-line backport lane, a planned correction-train lane, or a support-window
//! commitment — it records one row binding the lane to the control packet that proves
//! it is staffed, the support window it commits to, and the shared correction-train
//! packet form it rides, backs each lane against a public claim whose canonical label
//! is a hard ceiling, and narrows below the cutline any lane whose control packet aged
//! out or is missing, whose support window is incomplete or has passed its
//! end-of-support date, whose waiver expired, whose lane evidence is incomplete, or
//! whose backing public claim is itself below the cutline — while the
//! hotfix/backport/correction-train/support-window lane kinds and the release-line lane
//! set both stay fully covered.
//! The shiproom-dashboard module is the consuming dashboard layer over all of the above:
//! for every shiproom panel — a claim-truth, qualification, public-proof, or maintenance
//! panel — it records one row binding the panel to the upstream source it ingests, the
//! qualification rows it watches, the freshness packet that proves it is current, and the
//! measurable fitness functions it must clear, backs each panel against a public claim
//! whose canonical label is a hard ceiling, and narrows below the cutline any panel whose
//! freshness packet aged out or is missing, whose fitness function failed or is
//! unmeasured, whose watched qualification row regressed, whose waiver expired, whose
//! panel evidence is incomplete, or whose backing public claim is itself below the cutline
//! — while the claim-truth/qualification/public-proof/maintenance panel kinds and the
//! release-line panel set both stay fully covered, so shiproom and release tooling can
//! fail promotion directly from the dashboard.
//! The optional-surface-qualification module is the claim-narrowing automation alongside all
//! of the above: where the manifest, qualification matrix, and proof index speak for surfaces
//! meant to ship at the cutline, this register governs the *optional* surfaces — opt-in
//! capabilities, optional integrations, secondary platforms, and shipped-but-experimental
//! previews — whose default is *narrowed*. For every optional surface it records one row
//! binding the surface to the public claim it backs and to its qualification packet as an
//! optional value, so a surface that lacks a stable qualification packet entirely, whose
//! packet breached its freshness SLO, whose surface evidence or capability is incomplete,
//! whose waiver expired, or whose backing public claim is itself below the cutline narrows
//! below the launch cutline and never inherits an adjacent qualified surface — while the
//! opt-in/integration/platform/preview surface kinds and the release-relevant surface set
//! both stay fully covered, so shiproom and release tooling can fail promotion directly from
//! the register.
//! The finalize-qualification-packets-for-optional-surfaces module is the M4-stable-line
//! finalization layer on top of that register: it enumerates every optional surface required
//! for M4 stable promotion — notebook/data-rich, voice/dictation, browser/mobile companion,
//! preview/designer/publish, AI-adjacent, browser-runtime inspectors, package/dependency
//! mutation, infrastructure/cluster live-state, pipeline/run-control overlays, collaboration
//! session admission, observer/follow modes, shared terminal/debug control, consent/retention
//! envelopes, and session export/delete — and records per-deployment-target access modes
//! (desktop local, remote/helper, managed, self-hosted, air-gapped) so a missing packet
//! forces automatic downgrade on every target rather than inheriting an adjacent green row.
//! The benchmark-lab-governance module is the performance-evidence layer beside those
//! gates: where the hot-path-performance-budgets register protects the published p50/p95
//! numbers for each individual hot path, this register governs the benchmark-lab
//! automation lanes, corpus governance assets, and public benchmark publication packs that
//! *produce* those numbers. For every such asset it records one row binding the asset to
//! the public claim it backs and to the proof packet that grounds it (a CI lane health
//! record, a corpus manifest, a protected-metrics revision, or a publication pack), protects
//! each benchmark publication's published p50/p95 budget against the measured numbers (with
//! corpus metadata, lab trace, and a waiver hook for intentionally tightened thresholds),
//! and narrows below the cutline any asset whose proof packet aged out or is missing, whose
//! corpus metadata or benchmark-lab trace is missing, whose waiver expired, whose evidence
//! is incomplete, or whose backing public claim is itself below the cutline — while the
//! nightly-ci/self-capture/corpus/metrics/hardware/image/ledger/publication-pack asset kinds
//! and the release-blocking asset set both stay fully covered, so shiproom and release
//! tooling can fail qualification directly from the register.
//! The cohort-scoreboards module is the signoff-loop layer beside those gates: it
//! finalizes the design-partner, certified-archetype, and stable-cohort
//! scoreboards as one canonical packet, binds every scoreboard row to a public
//! claim ceiling and proof packet, and narrows any row whose packet is stale,
//! metric fails, waiver expires, or required signoff loop is incomplete before the
//! row can widen release, docs, Help/About, or support-export language.
//! The certified-reference-workspaces module is the certification-evidence layer
//! that hardens every marketed Certified archetype: it publishes one current
//! reference-workspace report per archetype, binds each report to the archetype
//! pass-matrix row that carries it, and automates the downgrade that narrows a
//! Certified claim when its report goes stale, missing, or manually edited.
//! The stable-publication-pack module is the outward-facing publication layer over all of
//! the above: where the manifest, proof index, version windows, and maintenance packet
//! govern what the release line *is*, this pack governs what the release line *says about
//! itself* — its known-limits publications, its public benchmark publications, its
//! compatibility publications, and its migration publications. For every such publication
//! it records one row binding the publication to the public claim it backs and to the
//! proof packet that grounds it (a known-limits register, a benchmark-lab trace, a
//! compatibility report, or a migration guide), protects each benchmark publication's
//! published p50/p95 budget against the measured numbers (with corpus metadata, lab
//! trace, and a waiver hook for intentionally tightened thresholds), and narrows below
//! the cutline any publication whose proof packet aged out or is missing, whose measured
//! numbers regressed beyond the published budget, whose corpus metadata or trace is
//! missing, whose waiver expired, whose evidence is incomplete, or whose backing public
//! claim is itself below the cutline — while the known-limit/benchmark/compatibility/
//! migration publication kinds and the release-line publication set both stay fully
//! covered, so shiproom and release tooling can fail publication directly from the pack.
//! The claim-publication-manifest module is the joined publication source consumed by docs,
//! Help/About, service-health, CLI inspection, release notes, public proof, support export,
//! and enterprise evaluation surfaces: it links every rendered claim to current
//! reference-workspace, compatibility, and evaluation report refs, then narrows every
//! destination automatically when backing evidence is stale, missing, dropped, or unsigned.
//! The open-paid-boundary-audit module is the governance-fact layer beside those gates:
//! where the manifest, proof index, and version windows speak for product capabilities and
//! interface surfaces, this audit governs the governance facts the stable launch rests on —
//! where the open-source core ends and the paid/managed tier begins, the licensing posture,
//! the build provenance, and the contribution policy. For every audited subject it records
//! one row binding the subject to the public claim it backs and to its attestation packet,
//! its required audit controls, and an owner sign-off, so a subject whose attestation packet
//! aged out or is missing, whose required audit control is unsatisfied, whose evidence is
//! incomplete, whose owner sign-off is missing, whose waiver expired, or whose backing public
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent attested row — while the open-paid-boundary/licensing/provenance/contribution-
//! policy domains and the release-line audit set both stay fully covered, so shiproom and
//! release tooling can fail promotion directly from the audit.
//! The go-no-go-rehearsal module is the launch-rehearsal layer that closes the loop over all
//! of the above: where the manifest, proof index, version windows, and audit govern what the
//! release line *is*, this rehearsal governs whether the release train was actually
//! *exercised* before the go/no-go — the explicit launch cutline signed off, the promotion
//! publish step dry-run, each rollback checkpoint verified to a restore point, and every open
//! exception packet reviewed. For every rehearsal stage it records one row binding the stage
//! to the public claim it backs and to its rehearsal packet, its required rollback
//! checkpoints, an exception packet (if any) holding it provisionally, and an owner sign-off,
//! so a stage whose rehearsal packet aged out or is missing, whose rollback checkpoint is
//! unverified, whose evidence is incomplete, whose owner sign-off is missing, whose exception
//! packet expired, or whose backing public claim is itself below the cutline narrows to a
//! No-Go below the launch cutline and never inherits an adjacent rehearsed stage — while the
//! cutline-review/promotion-step/rollback-checkpoint/exception-review stage kinds and the
//! release-line rehearsal set both stay fully covered, so shiproom and release tooling can
//! fail the go/no-go directly from the rehearsal.
//! The hot-path-performance-budgets module is the performance-layer register beside those
//! gates: for every hot path — startup, restore, quick open, typing, scrolling, search, and
//! Git status — it records one row binding the path to the stable claim manifest entry whose
//! lifecycle label it backs, the benchmark budget that protects the published p50/p95 numbers,
//! the proof packet that grounds them, and the waiver (if any) holding a tightened threshold
//! provisionally, so a path whose measured numbers regressed beyond the published budget,
//! whose proof packet aged out or is missing, whose corpus metadata or benchmark-lab trace is
//! absent, whose waiver expired, whose owner sign-off is missing, or whose backing public
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent backed budget — while the seven hot path kinds and the release-blocking path set
//! both stay fully covered, so shiproom and release tooling can fail promotion directly from
//! the register.
//! The accessibility-surface-signoffs module is the accessibility-layer register beside those
//! gates: for every touched surface — shell, tree, palette, diff, terminal, debugger, settings,
//! auth, and recovery — it records one row binding the surface to the stable claim manifest
//! entry whose lifecycle label it backs, the per-dimension checks that validate keyboard,
//! screen-reader, IME/grapheme/bidi, zoom, high-contrast, and reduced-motion behavior, the
//! proof packet that grounds them, and the waiver (if any) holding a provisional signoff, so
//! a surface whose dimension checks are blocked or pending, whose proof packet aged out or is
//! missing, whose owner sign-off is absent, or whose backing public claim is itself below the
//! cutline narrows below the launch cutline and never inherits an adjacent qualified surface —
//! while the nine surface kinds and the release-blocking surface set both stay fully covered,
//! so shiproom and release tooling can fail promotion directly from the register.
//! The clean-room-rebuild proof module is the exact-build supportability lane beside those
//! gates: for every marketed package-channel row, exact-build symbolication row, and release
//! truth parity surface, it records whether a fresh packet, verified rebuild evidence, and
//! exact-build symbol linkage still support the published claim or have already narrowed below
//! it — while mirror/offline publication coherence stays explicitly governed instead of being
//! inferred from the primary package rows alone.
//! The notebook-and-data-rich-surface-qualification module is the family-specific release
//! guard for notebook and data-heavy promoted surfaces. It keeps document trust,
//! kernel/runtime trust, and output trust as separate packet truths; binds notebook headers,
//! kernel bars, cells, output panes, variable explorers, data tables, result grids, chart
//! summaries, and experiment handoff cards to replay/export, snapshot/golden review,
//! accessibility, support-export, and downgrade-label evidence; and prevents notebook,
//! database/result-grid, or profiler-style language from widening unless that family row has
//! its own current proof.
//! The voice-and-dictation-surface-qualification module is the family-specific release
//! guard for speech input. It requires explicit command-vs-dictation mode truth,
//! push-to-talk or explicit activation defaults, provider/privacy disclosure, bounded
//! transcript handling, unavailable-state fallbacks, accessibility evidence, and command
//! graph parity before any voice or dictation row can render as Stable.
//! The publish-feature-scorecard-and-compatibility-packet-templates module is the
//! template-governance layer for every M5 feature family: it publishes the canonical
//! scorecard-template and compatibility-packet-template definitions that downstream
//! scorecards and compatibility reports must follow, binds each family to its templates,
//! tracks template-section publication state, and narrows any family whose templates are
//! incomplete, stale, missing required sections, or lack owner sign-off — while the
//! notebook/data-rich/ai-adjacent/framework/review/companion/managed-depth family kinds
//! and the release-blocking family set both stay fully covered, so shiproom and release
//! tooling can fail promotion directly from the register.
//! The freeze-the-m5-depth-claim-manifest module is the depth-claim freeze that closes the
//! M5 qualification loop: for every M5 feature family it records one feature-family packet
//! binding the family to the stable depth claim it backs and to a qualification matrix of one
//! cell per dimension — scorecard, compatibility, proof freshness, generated-artifact lineage,
//! locale parity, support-packet currency, accessibility, and downgrade automation — so a
//! family whose proof packet aged out or is missing, whose lineage is absent, whose locale
//! parity drifted, whose support packet lags shipped behavior, whose accessibility is unsigned,
//! whose downgrade automation is undefined, whose waiver expired, whose owner sign-off is
//! missing, or whose backing depth claim is itself below the cutline narrows below the launch
//! cutline and never inherits an adjacent qualified family — while the seven family kinds, the
//! eight qualification dimensions, and the release-blocking family set all stay fully covered,
//! so shiproom and release tooling can fail promotion directly from the manifest.
//! The implement-per-feature-scorecards module is the per-train qualification layer that
//! sits beside the depth-claim manifest: where the manifest speaks for the depth claim each
//! M5 feature *family* publishes, this register speaks for the per-feature *scorecard*, the
//! *owner manifest*, and the explicit *rollback/downgrade automation* every M5 feature train
//! carries. For every M5 train it records one scorecard binding the train to the stable claim
//! it backs, a scorecard of one cell per axis (functionality, performance, accessibility,
//! compatibility, localization, support readiness), an owner-manifest sign-off, and a
//! rollback/downgrade automation record bound to a verified rollback plan and the trigger and
//! floor it narrows to, so a train whose scorecard axis failed or is missing, whose proof
//! packet aged out or is missing, whose owner manifest is unsigned, whose rollback plan is
//! unverified, whose downgrade automation is undefined, whose waiver expired, or whose backing
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent qualified train — while the seven train kinds, the six scorecard axes, and the
//! release-blocking train set all stay fully covered, so shiproom and release tooling can fail
//! promotion directly from the register.
//! The ship-generated-artifact-lineage module is the lineage-truth layer for generated
//! outputs: where the train scorecard register speaks for each feature train, this register
//! speaks for the *lineage surface* every generated-artifact family exposes — scaffolded,
//! AI-generated, notebook-derived, and preview-derived outputs. For every family it records one
//! surface binding the family to the stable claim it backs, a lineage scorecard of one cell per
//! dimension (provenance, inputs, generator identity, transform, reproducibility, disclosure),
//! the disclosed artifact provenance and trust tier, an owner-manifest sign-off, and a
//! rollback/downgrade automation record bound to a verified rollback plan, so a surface whose
//! lineage dimension failed or is missing, whose artifact is not labeled as generated, whose
//! proof packet aged out or is missing, whose owner manifest is unsigned, whose rollback plan is
//! unverified, whose downgrade automation is undefined, whose waiver expired, or whose backing
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent traced surface — while the four generator kinds, the six lineage dimensions, and the
//! release-blocking surface set all stay fully covered, so shiproom and release tooling can fail
//! promotion directly from the register.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces;
pub mod browser_mobile_companion_surface_qualification;
pub mod claim_publication_manifest;
pub mod correction_train;
pub mod finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack;
pub mod finalize_compatibility_reports_deprecation_packets_schema_version_windows;
pub mod finalize_design_partner_certified_archetype_and_stable_cohort;
pub mod finalize_experiments_labs_inventory;
pub mod finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance;
pub mod finalize_qualification_packets_for_optional_surfaces_and_enforce;
pub mod finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support;
pub mod finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills;
pub mod freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix;
pub mod freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph;
pub mod freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules;
pub mod generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains;
pub mod generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows;
pub mod generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes;
pub mod go_no_go_rehearsal;
pub mod harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation;
pub mod harden_docs_help_about_and_service_health_truth;
pub mod harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage;
pub mod harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity;
pub mod implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance;
pub mod implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains;
pub mod maintenance_control_packet;
pub mod mixed_version_compatibility_and_skew_governance;
pub mod notebook_and_data_rich_surface_qualification;
pub mod open_paid_boundary_audit;
pub mod optional_surface_qualification;
pub mod preview_designer_publish_surface_qualification;
pub mod prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence;
pub mod publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family;
pub mod publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes;
pub mod publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan;
pub mod publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet;
pub mod publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts;
pub mod publish_the_signed_m4_stable_evidence_pack_plus;
pub mod release_center_model;
pub mod seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails;
pub mod ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards;
pub mod ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs;
pub mod shiproom_dashboard;
pub mod stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery;
pub mod stabilize_embedded_surface_boundary_truth;
pub mod stabilize_hot_path_performance_against_published_budgets_for;
pub mod stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication;
pub mod stabilize_the_release_center_promotion_evidence_canary_pilot;
pub mod stable_boundary_manifest;
pub mod stable_claim_manifest;
pub mod stable_claim_matrix;
pub mod stable_proof_index;
pub mod stable_publication_pack;
pub mod stable_qualification_matrix;
pub mod stable_version_windows;
pub mod support_class_ledger;
pub mod voice_and_dictation_surface_qualification;

pub use freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix::{
    current_m5_depth_claim_manifest, DepthClaimExportProjection, DepthClaimExportRow,
    DepthClaimManifest, DepthClaimManifestSummary, DepthClaimManifestViolation, DepthStopAction,
    DepthStopRule, FamilyKind, FamilyPacket, NarrowingReason as DepthClaimNarrowingReason,
    PacketState, QualificationCell, QualificationDimension,
    QualificationState as DepthClaimQualificationState, FREEZE_M5_DEPTH_CLAIM_MANIFEST_JSON,
    FREEZE_M5_DEPTH_CLAIM_MANIFEST_PATH, FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND,
    FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION,
};

pub use implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains::{
    current_m5_train_scorecard_register, AutomationState as TrainAutomationState,
    DowngradeAutomation, DowngradeTrigger, NarrowingReason as TrainScorecardNarrowingReason,
    ScoreGrade, ScorecardAxis, ScorecardCell, StopAction as TrainScorecardStopAction, TrainKind,
    TrainScorecard, TrainScorecardExportProjection,
    TrainScorecardExportRow, TrainScorecardRegister, TrainScorecardRegisterSummary,
    TrainScorecardRegisterViolation, TrainState, TrainStopRule,
    IMPLEMENT_M5_TRAIN_SCORECARDS_JSON, IMPLEMENT_M5_TRAIN_SCORECARDS_PATH,
    IMPLEMENT_M5_TRAIN_SCORECARDS_RECORD_KIND, IMPLEMENT_M5_TRAIN_SCORECARDS_SCHEMA_VERSION,
};

pub use implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance::{
    current_feature_train_compatibility_register, ChangeFreezeGuidance, CompatibilityCell,
    CompatibilityDimension, DimensionGrade as FeatureTrainDimensionGrade, FeatureTrainCompatibilityRegister,
    FeatureTrainCompatibilitySummary, FeatureTrainExportProjection, FeatureTrainExportRow,
    FeatureTrainLane, FeatureTrainRegisterViolation, FeatureTrainStopRule,
    FreezeState, FreezeTrigger, NarrowingReason as FeatureTrainNarrowingReason, ProviderSupportWindow,
    StopAction as FeatureTrainStopAction, TrainChannel, TrainState as FeatureTrainState,
    TrustTier as FeatureTrainTrustTier,
    FEATURE_TRAIN_COMPATIBILITY_JSON, FEATURE_TRAIN_COMPATIBILITY_PATH,
    FEATURE_TRAIN_COMPATIBILITY_RECORD_KIND, FEATURE_TRAIN_COMPATIBILITY_SCHEMA_VERSION,
};

pub use add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces::{
    current_locale_pack_governance_register, AutomationState as LocalePackAutomationState,
    DimensionGrade as LocalePackDimensionGrade, DowngradeAutomation as LocalePackDowngradeAutomation,
    DowngradeTrigger as LocalePackDowngradeTrigger, GovernanceCell, GovernanceDimension,
    LocalePackExportProjection, LocalePackExportRow, LocalePackGovernanceRegister,
    LocalePackGovernanceSummary, LocalePackLane, LocalePackRegisterViolation, LocalePackStopRule,
    NarrowingReason as LocalePackNarrowingReason, PackChannel, PackState,
    StopAction as LocalePackStopAction, TranslationGovernance, TrustTier as LocalePackTrustTier,
    LOCALE_PACK_GOVERNANCE_JSON, LOCALE_PACK_GOVERNANCE_PATH, LOCALE_PACK_GOVERNANCE_RECORD_KIND,
    LOCALE_PACK_GOVERNANCE_SCHEMA_VERSION,
};

pub use ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs::{
    current_generated_artifact_lineage_register, AutomationState as LineageAutomationState,
    DimensionGrade, DowngradeAutomation as LineageDowngradeAutomation,
    DowngradeTrigger as LineageDowngradeTrigger, GeneratedArtifactLineageRegister,
    GeneratedArtifactLineageSummary, GeneratorKind, LineageCell, LineageDimension,
    LineageExportProjection, LineageExportRow, LineageProvenance, LineageRegisterViolation,
    LineageState, LineageStopRule, LineageSurface, NarrowingReason as LineageNarrowingReason,
    StopAction as LineageStopAction, TrustTier, GENERATED_ARTIFACT_LINEAGE_JSON,
    GENERATED_ARTIFACT_LINEAGE_PATH, GENERATED_ARTIFACT_LINEAGE_RECORD_KIND,
    GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION,
};

pub use ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards::{
    current_fitness_surface_register, AutomationState as FitnessAutomationState,
    AutomationTrigger as FitnessAutomationTrigger, CorpusProvenance,
    DimensionGrade as FitnessDimensionGrade, DowngradeAutomation as FitnessDowngradeAutomation,
    FitnessCell, FitnessDimension, FitnessSurfaceExportProjection, FitnessSurfaceExportRow,
    FitnessSurfaceLane, FitnessSurfaceRegister, FitnessSurfaceStopRule, FitnessSurfaceSummary,
    FitnessSurfaceViolation, NarrowingReason as FitnessSurfaceNarrowingReason,
    StopAction as FitnessSurfaceStopAction, SurfaceKind as FitnessSurfaceKind,
    SurfaceState as FitnessSurfaceState,
    TrustTier as FitnessSurfaceTrustTier, FITNESS_SURFACE_JSON, FITNESS_SURFACE_PATH,
    FITNESS_SURFACE_RECORD_KIND, FITNESS_SURFACE_SCHEMA_VERSION,
};

pub use claim_publication_manifest::{
    current_claim_publication_manifest, ClaimDowngradeRule, ClaimNarrowingReason,
    ClaimPublicationDecision, ClaimPublicationEntry, ClaimPublicationManifest,
    ClaimPublicationRecord, ClaimPublicationSummary, ClaimPublicationSurfaceEntry,
    ClaimPublicationSurfaceExport, ClaimPublicationViolation, ClaimReportRef, ClaimSurface,
    ClaimValidityWindow, EffectiveClaim, EvaluationFilter, EvidenceState,
    PublicationAction as ClaimPublicationAction, ReportFamily, SupportClass as ClaimSupportClass,
    SurfaceProjection, CLAIM_PUBLICATION_MANIFEST_JSON, CLAIM_PUBLICATION_MANIFEST_PATH,
    CLAIM_PUBLICATION_MANIFEST_RECORD_KIND, CLAIM_PUBLICATION_MANIFEST_SCHEMA_VERSION,
};
pub use correction_train::{
    BackportDecision, BackportMatrixRow, CorrectionEvidence, CorrectionItem, CorrectionRisk,
    CorrectionScope, CorrectionTrainPacket, CorrectionTrainViolation, CorrectionTriage,
    PacketTemplates, ReleaseNotesRefs, SupportProjection, TargetChannelUpdate, TriageLane,
    CORRECTION_TRAIN_PACKET_RECORD_KIND, CORRECTION_TRAIN_PACKET_SCHEMA_VERSION,
    SECURITY_OR_TRUST_ISSUE_CLASSES, SHARED_PACKET_FORM_TERMS, SUPPORTED_LINE_CLASSES,
};

pub use finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack::{
    current_benchmark_lab_governance, AssetAction, AssetState, BenchmarkLabGovernance,
    BenchmarkLabGovernanceExportProjection, BenchmarkLabGovernanceExportRow,
    BenchmarkLabGovernanceSummary, BenchmarkLabGovernanceViolation, GovernanceAssetKind,
    GovernanceAssetRow, GovernanceRule, GapReason as BenchmarkLabGapReason, QualificationRecord,
    BENCHMARK_LAB_GOVERNANCE_JSON, BENCHMARK_LAB_GOVERNANCE_PATH,
    BENCHMARK_LAB_GOVERNANCE_RECORD_KIND, BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION,
};

pub use finalize_design_partner_certified_archetype_and_stable_cohort::{
    current_cohort_scoreboards, CohortScoreboardRow, CohortScoreboards,
    CohortScoreboardsExportProjection, CohortScoreboardsExportRow, CohortScoreboardsSummary,
    CohortScoreboardsViolation, RequiredSignoff, ScoreboardAction, ScoreboardGapReason,
    ScoreboardLane, ScoreboardMetric, ScoreboardPublicationRecord, ScoreboardRule, ScoreboardState,
    SignoffLoop, COHORT_SCOREBOARDS_JSON, COHORT_SCOREBOARDS_PATH, COHORT_SCOREBOARDS_RECORD_KIND,
    COHORT_SCOREBOARDS_SCHEMA_VERSION,
};
pub use finalize_experiments_labs_inventory::{
    audit_finalize_experiments_labs_inventory_page, build_page_from_inventory,
    seeded_finalize_experiments_labs_inventory_page,
    validate_finalize_experiments_labs_inventory_page,
    FinalizeExperimentsLabsInventoryCliProjection, FinalizeExperimentsLabsInventoryCliRow,
    FinalizeExperimentsLabsInventoryDefect, FinalizeExperimentsLabsInventoryError,
    FinalizeExperimentsLabsInventoryPage, FinalizeExperimentsLabsInventoryRow,
    FinalizeExperimentsLabsInventorySummary, FinalizeExperimentsLabsInventorySupportExport,
    InventoryDependencyMarker, InventoryNarrowReasonClass, InventoryQualificationClass,
    InventorySurfaceClass, KillSwitchVisibilityRow,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_ARTIFACT_REF,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_DOC_REF,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_PAGE_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_ROW_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUMMARY_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUPPORT_EXPORT_RECORD_KIND,
};

pub use finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance::{
    current_desktop_platform_conformance, CheckKind, CheckState, ConformanceAction, ConformanceDomain,
    ConformanceState, DesktopPlatformConformance, DesktopPlatformConformanceRule,
    DesktopPlatformConformanceRow, DesktopPlatformConformanceSummary,
    DesktopPlatformConformanceViolation, GapReason as ConformanceGapReason,
    DESKTOP_PLATFORM_CONFORMANCE_JSON, DESKTOP_PLATFORM_CONFORMANCE_PATH,
    DESKTOP_PLATFORM_CONFORMANCE_RECORD_KIND, DESKTOP_PLATFORM_CONFORMANCE_SCHEMA_VERSION,
};

pub use finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills::{
    current_security_response_packet, EmergencyControl, GapReason as ResponseGapReason,
    MirrorDrillCheckpoint, ResponseAction, ResponseExportProjection, ResponseExportRow,
    ResponseKind, ResponsePublicationRecord, ResponseRule, ResponseRow, ResponseState,
    SecurityResponsePacket, SecurityResponsePacketSummary, SecurityResponsePacketViolation,
    SECURITY_RESPONSE_PACKET_JSON, SECURITY_RESPONSE_PACKET_PATH,
    SECURITY_RESPONSE_PACKET_RECORD_KIND, SECURITY_RESPONSE_PACKET_SCHEMA_VERSION,
};

pub use finalize_compatibility_reports_deprecation_packets_schema_version_windows::{
    current_finalize_compatibility_reports_deprecation_packets_schema_version_windows,
    CompatibilityOutcome, CompatibilityReportPacket, DeprecationDetail, FinalizeAction,
    FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows, FinalizeExportProjection,
    FinalizeExportRow, FinalizeKind, FinalizePublicationRecord, FinalizeRow, FinalizeRule,
    FinalizeState, FinalizeSummary, FinalizeViolation, GapReason as FinalizeGapReason,
    MigrationDetail, Scorecard, ValidityWindow as FinalizeValidityWindow,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_JSON,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_PATH,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_RECORD_KIND,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_SCHEMA_VERSION,
};

pub use finalize_qualification_packets_for_optional_surfaces_and_enforce::{
    current_finalize_qualification_packets_for_optional_surfaces_and_enforce, DeploymentAccessMode,
    DeploymentQualification, DeploymentTarget, FinalizeNarrowAction as OptionalSurfaceNarrowAction,
    FinalizeNarrowReason as OptionalSurfaceNarrowReason, FinalizeOptionalSurface,
    FinalizeOptionalSurfaceKind, FinalizeQualificationPacketsForOptionalSurfacesAndEnforce,
    FinalizeQualificationSummary, FinalizeQualificationViolation, FinalizeSurfaceExportProjection,
    FinalizeSurfaceExportRow, FinalizeSurfacePublicationRecord, FinalizeSurfaceState,
    FinalizeSurfaceStopRule, FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_JSON,
    FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_PATH,
    FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_RECORD_KIND,
    FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_SCHEMA_VERSION,
};

pub use finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support::{
    current_finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support,
    ConsumingSurface, DowngradePropagationStatus, FreshnessObjectAction, FreshnessObjectExportProjection,
    FreshnessObjectExportRow, FreshnessObjectGapReason, FreshnessObjectKind, FreshnessObjectPublicationRecord,
    FreshnessObjectRule, FreshnessObjectRow, FreshnessObjectState, FreshnessObjectSummary,
    FreshnessObjectViolation,
    FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport,
    ValidityWindow as FreshnessValidityWindow,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_JSON,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_PATH,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_RECORD_KIND,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_SCHEMA_VERSION,
};

pub use go_no_go_rehearsal::{
    current_go_no_go_rehearsal, GoNoGoRehearsal, GoNoGoRehearsalSummary, GoNoGoRehearsalViolation,
    RehearsalAction, RehearsalExportProjection, RehearsalExportRow, RehearsalGapReason,
    RehearsalPublicationRecord, RehearsalRule, RehearsalStageRow, RehearsalState,
    RollbackCheckpoint, StageKind, GO_NO_GO_REHEARSAL_JSON, GO_NO_GO_REHEARSAL_PATH,
    GO_NO_GO_REHEARSAL_RECORD_KIND, GO_NO_GO_REHEARSAL_SCHEMA_VERSION,
};

pub use harden_docs_help_about_and_service_health_truth::{
    current_docs_help_about_service_health_truth, AboutProvenanceCard, DestinationTrustClass,
    DocsHelpAboutServiceHealthTruth, DocsHelpAboutServiceHealthTruthViolation, HelpDestination,
    PackageSafetyDisclosure, ServiceContractState, TruthAction, TruthExportProjection,
    TruthExportRow, TruthPublicationRecord, TruthRow, TruthRule, TruthState, TruthSummary,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_JSON, DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_PATH,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_RECORD_KIND,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_SCHEMA_VERSION,
};

pub use stabilize_embedded_surface_boundary_truth::{
    current_embedded_surface_boundary_truth, AuthHandoffSnapshot,
    BoundaryState as EmbeddedBoundaryState, BrowserFallbackSnapshot, EmbeddedSurfaceBoundaryTruth,
    EmbeddedSurfaceBoundaryTruthViolation, GapReason as EmbeddedSurfaceGapReason,
    NativeApprovalSnapshot, SourceTruthSnapshot, SurfaceKind as EmbeddedSurfaceKind,
    TruthAction as EmbeddedSurfaceTruthAction,
    TruthExportProjection as EmbeddedSurfaceTruthExportProjection,
    TruthExportRow as EmbeddedSurfaceTruthExportRow,
    TruthPublicationRecord as EmbeddedSurfaceTruthPublicationRecord,
    TruthRow as EmbeddedSurfaceTruthRow, TruthRule as EmbeddedSurfaceTruthRule,
    TruthState as EmbeddedSurfaceTruthState, TruthSummary as EmbeddedSurfaceTruthSummary,
    EMBEDDED_SURFACE_BOUNDARY_TRUTH_JSON, EMBEDDED_SURFACE_BOUNDARY_TRUTH_PATH,
    EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND, EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION,
};

pub use harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation::{
    current_certified_reference_workspaces, ArchetypePassMatrixExportRow, ArchetypePassMatrixRow,
    CertifiedReferenceWorkspaces, CertifiedReferenceWorkspacesExportProjection,
    CertifiedReferenceWorkspacesSummary, CertifiedReferenceWorkspacesViolation,
    DowngradeReason as ReferenceWorkspaceDowngradeReason,
    DowngradeRule as ReferenceWorkspaceDowngradeRule, MatrixAction, MatrixRowState,
    PublicationDecision as ReferenceWorkspacePublicationDecision,
    PublicationDecisionRecord as ReferenceWorkspacePublicationDecisionRecord,
    ReferenceWorkspaceExportRow, ReferenceWorkspaceReport, ReportState, ValidityWindow,
    CERTIFIED_REFERENCE_WORKSPACES_JSON, CERTIFIED_REFERENCE_WORKSPACES_PATH,
    CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND, CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION,
};

pub use harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity::{
    current_harden_release_artifact_graph, ArtifactFamilyAction, ArtifactFamilyExportRow,
    ArtifactFamilyGapReason, ArtifactFamilyKind, ArtifactFamilyRow, ArtifactFamilyRule,
    ArtifactFamilyState, HardenReleaseArtifactGraph, HardenReleaseArtifactGraphExportProjection,
    HardenReleaseArtifactGraphSummary, HardenReleaseArtifactGraphViolation,
    PublicationDecision as ArtifactGraphPublicationDecision,
    PublicationDecisionRecord as ArtifactGraphPublicationDecisionRecord,
    HARDEN_RELEASE_ARTIFACT_GRAPH_JSON, HARDEN_RELEASE_ARTIFACT_GRAPH_PATH,
    HARDEN_RELEASE_ARTIFACT_GRAPH_RECORD_KIND, HARDEN_RELEASE_ARTIFACT_GRAPH_SCHEMA_VERSION,
};

pub use harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage::{
    current_harden_critical_dependency_register, HardenCriticalDependencyRegister,
    HardenCriticalDependencyRegisterExportProjection, HardenCriticalDependencyRegisterSummary,
    HardenCriticalDependencyRegisterViolation, LaneAction, LaneExportRow, LaneGapReason,
    LaneKind as DependencyLaneKind, LaneRow, LaneRule, LaneState, PublicationDecision as DependencyRegisterPublicationDecision,
    PublicationDecisionRecord as DependencyRegisterPublicationDecisionRecord,
    HARDEN_CRITICAL_DEPENDENCY_REGISTER_JSON, HARDEN_CRITICAL_DEPENDENCY_REGISTER_PATH,
    HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND, HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION,
};

pub use maintenance_control_packet::{
    current_maintenance_control_packet, ControlAction, ControlPublicationRecord, ControlRule,
    ControlState, GapReason as MaintenanceGapReason, LaneKind, MaintenanceControlPacket,
    MaintenanceControlPacketSummary, MaintenanceControlPacketViolation,
    MaintenanceExportProjection, MaintenanceExportRow, MaintenanceRow, SupportPosture,
    SupportWindow, MAINTENANCE_CONTROL_PACKET_JSON, MAINTENANCE_CONTROL_PACKET_PATH,
    MAINTENANCE_CONTROL_PACKET_RECORD_KIND, MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

pub use open_paid_boundary_audit::{
    current_open_paid_boundary_audit, AuditAction, AuditControl, AuditDomain,
    AuditExportProjection, AuditExportRow, AuditGapReason, AuditPublicationRecord, AuditRow,
    AuditRule, AuditState, OpenPaidBoundaryAudit, OpenPaidBoundaryAuditSummary,
    OpenPaidBoundaryAuditViolation, OPEN_PAID_BOUNDARY_AUDIT_JSON, OPEN_PAID_BOUNDARY_AUDIT_PATH,
    OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND, OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION,
};

pub use publish_the_signed_m4_stable_evidence_pack_plus::{
    current_signed_m4_stable_evidence_pack, BundleAction, BundleExportProjection, BundleExportRow,
    BundleGapReason, BundleRule, BundleState, EvidenceBundleKind, EvidenceBundleRow,
    SignedM4StableEvidencePack, SignedM4StableEvidencePackViolation,
    SIGNED_M4_STABLE_EVIDENCE_PACK_JSON, SIGNED_M4_STABLE_EVIDENCE_PACK_PATH,
    SIGNED_M4_STABLE_EVIDENCE_PACK_RECORD_KIND, SIGNED_M4_STABLE_EVIDENCE_PACK_SCHEMA_VERSION,
};

pub use prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence::{
    current_clean_room_rebuild_proof, ChannelFamilyAction, ChannelFamilyCategory, ChannelFamilyExportRow,
    ChannelFamilyGapReason, ChannelFamilyKind, ChannelFamilyRow, ChannelFamilyRule, ChannelFamilyState,
    CleanRoomRebuildProof, CleanRoomRebuildProofExportProjection, CleanRoomRebuildProofSummary,
    CleanRoomRebuildProofViolation, PublicationDecision as CleanRoomRebuildPublicationDecision,
    PublicationDecisionRecord as CleanRoomRebuildPublicationDecisionRecord, RebuildState,
    SymbolicationState, CLEAN_ROOM_REBUILD_PROOF_JSON, CLEAN_ROOM_REBUILD_PROOF_PATH,
    CLEAN_ROOM_REBUILD_PROOF_RECORD_KIND, CLEAN_ROOM_REBUILD_PROOF_SCHEMA_VERSION,
};

pub use optional_surface_qualification::{
    current_optional_surface_qualification, NarrowAction, NarrowReason, OptionalSurface,
    OptionalSurfaceKind, OptionalSurfaceQualification, OptionalSurfaceQualificationSummary,
    OptionalSurfaceQualificationViolation, SurfaceExportProjection, SurfaceExportRow,
    SurfacePublicationRecord, SurfaceState, SurfaceStopRule, OPTIONAL_SURFACE_QUALIFICATION_JSON,
    OPTIONAL_SURFACE_QUALIFICATION_PATH, OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND,
    OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use voice_and_dictation_surface_qualification::{
    current_voice_and_dictation_surface_qualification, ActivationDefault, CommandParityContract,
    ProcessingClass, TranscriptPrivacyControls, TranscriptRetention,
    VoiceAndDictationSurfaceQualification, VoiceFallbackState, VoiceMode, VoiceProjection,
    VoiceQualificationSummary, VoiceQualificationViolation, VoiceSurfaceKind, VoiceSurfaceRow,
    VoiceUiPrimitives, VOICE_DICTATION_SURFACE_QUALIFICATION_JSON,
    VOICE_DICTATION_SURFACE_QUALIFICATION_PATH, VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND,
    VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use preview_designer_publish_surface_qualification::{
    current_preview_designer_publish_surface_qualification, ActionSafetyLineage,
    BrowserInspectionBoundary, ExportedArtifactTruth, FallbackPaths, GeneratedSourceTruth,
    PreviewDesignerPublishExportProjection, PreviewDesignerPublishExportRow,
    PreviewDesignerPublishQualificationSummary, PreviewDesignerPublishQualificationViolation,
    PreviewDesignerPublishSurfaceKind, PreviewDesignerPublishSurfaceQualification,
    PreviewDesignerPublishSurfaceRow, QualificationProjection as PreviewDesignerPublishProjection,
    SafePreviewPosture, SourceMappingQuality, SourceSyncState,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_JSON,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_PATH,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use release_center_model::{
    ArtifactBundleCard, ArtifactFamilyClass, ArtifactGraphConsistency, ArtifactPayloadRefs,
    AuthSourceClass, BlastRadiusClass, BreakGlassDisclosure, BreakGlassStateClass,
    CompatibilityImpactClass, CompatibilityNote, ContinuityClass, ContinuityNote,
    DryRunAvailabilityClass, DryRunDisclosure, EvidenceFreshnessClass, EvidenceRef,
    ImmutableDigest, PromotionEventClass, PromotionReadiness, PromotionStage,
    PromotionTimelineStep, PublishTargetClass, PublishTargetDescriptor, ReleaseCandidate,
    ReleaseCenterHeadlessPlan, ReleaseCenterModelValidationReport, ReleaseCenterModelViolation,
    ReleaseCenterObjectIdentityIndex, ReleaseCenterObjectModel, ReleaseCenterSupportAuditExport,
    ReleaseCenterUiState, RollbackOrRevocationKind, RollbackOrRevocationRecord, RolloutRing,
    SemanticChangeClass, SignatureStateClass, TargetMutabilityClass, TargetVisibilityClass,
    VersionBumpProposal, RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND,
    RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION,
};

pub use shiproom_dashboard::{
    current_shiproom_dashboard, Comparator, DashboardExportProjection, DashboardExportRow,
    DashboardPanel, DashboardPublicationRecord, FitnessFunction, FitnessStatus, PanelKind,
    PanelState, QualificationStopRule, ShiproomDashboard, ShiproomDashboardSummary,
    ShiproomDashboardViolation, StopAction as DashboardStopAction, StopReason,
    SHIPROOM_DASHBOARD_JSON, SHIPROOM_DASHBOARD_PATH, SHIPROOM_DASHBOARD_RECORD_KIND,
    SHIPROOM_DASHBOARD_SCHEMA_VERSION,
};

pub use stable_boundary_manifest::{
    current_stable_boundary_manifest, BoundaryAction, BoundaryExportProjection, BoundaryExportRow,
    BoundaryPublicationRecord, BoundaryRow, BoundaryRule, BoundaryState,
    NarrowingReason as BoundaryNarrowingReason, StableBoundaryManifest,
    StableBoundaryManifestSummary, StableBoundaryManifestViolation, ValueLine, ValueLineProfile,
    ValueLineRollup, STABLE_BOUNDARY_MANIFEST_JSON, STABLE_BOUNDARY_MANIFEST_PATH,
    STABLE_BOUNDARY_MANIFEST_RECORD_KIND, STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION,
};

pub use stable_claim_manifest::{
    current_stable_claim_manifest, FreshnessSlo, FreshnessSloState, ManifestEntry,
    ManifestExportProjection, ManifestExportRow, ManifestPublicationRecord, ManifestState,
    NarrowingReason, ProofPacket, PublicationAction, PublicationRule, StableClaimManifest,
    StableClaimManifestSummary, StableClaimManifestViolation, STABLE_CLAIM_MANIFEST_JSON,
    STABLE_CLAIM_MANIFEST_PATH, STABLE_CLAIM_MANIFEST_RECORD_KIND,
    STABLE_CLAIM_MANIFEST_SCHEMA_VERSION,
};

pub use stable_proof_index::{
    current_stable_proof_index, GapReason, IndexAction, ProofIndexExportProjection,
    ProofIndexExportRow, ProofPublicationRecord, ProofRow, ProofRule, ProofState, StableProofIndex,
    StableProofIndexSummary, StableProofIndexViolation, STABLE_PROOF_INDEX_JSON,
    STABLE_PROOF_INDEX_PATH, STABLE_PROOF_INDEX_RECORD_KIND, STABLE_PROOF_INDEX_SCHEMA_VERSION,
};

pub use stable_publication_pack::{
    current_stable_publication_pack, BenchmarkBudget, GapReason as PublicationGapReason,
    PackPublicationRecord, PublicationAction as PackPublicationAction, PublicationKind,
    PublicationPackExportProjection, PublicationPackExportRow, PublicationRow,
    PublicationRule as PackPublicationRule, PublicationState, StablePublicationPack,
    StablePublicationPackSummary, StablePublicationPackViolation, STABLE_PUBLICATION_PACK_JSON,
    STABLE_PUBLICATION_PACK_PATH, STABLE_PUBLICATION_PACK_RECORD_KIND,
    STABLE_PUBLICATION_PACK_SCHEMA_VERSION,
};

pub use stable_claim_matrix::{
    current_stable_claim_matrix, DowngradeReason, LaunchCutline, OwnerSignoff, PromotionDecision,
    PromotionDecisionRecord, QualificationEvidence, QualificationState, QualificationWaiver,
    ShiproomStopRule, StableClaimExportProjection, StableClaimExportRow, StableClaimLevel,
    StableClaimMatrix, StableClaimMatrixSummary, StableClaimMatrixViolation, StableClaimRow,
    StopAction, STABLE_CLAIM_MATRIX_JSON, STABLE_CLAIM_MATRIX_PATH,
    STABLE_CLAIM_MATRIX_RECORD_KIND, STABLE_CLAIM_MATRIX_SCHEMA_VERSION,
};

pub use stable_qualification_matrix::{
    current_stable_qualification_matrix, BoundaryFamily,
    DowngradeReason as QualificationDowngradeReason, DowngradeRule as QualificationDowngradeRule,
    MixedVersionPosture, MixedVersionSection, OrderRecord, OutOfWindowPosture,
    PromotionDecisionRecord as QualificationPromotionDecisionRecord, QualificationAction,
    QualificationExportProjection, QualificationExportRow, QualificationRow, QualificationRowScope,
    SkewWindow, StableQualificationMatrix, StableQualificationMatrixSummary,
    StableQualificationMatrixViolation, UnsupportedStateBehavior, STABLE_QUALIFICATION_MATRIX_JSON,
    STABLE_QUALIFICATION_MATRIX_PATH, STABLE_QUALIFICATION_MATRIX_RECORD_KIND,
    STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
};

pub use stable_version_windows::{
    current_stable_version_windows, CompatibilityPosture, DeprecationNotice, DeprecationPacket,
    DeprecationStatus, FreezePublicationRecord, FreezeRule, GapReason as VersionWindowGapReason,
    StableVersionWindows, StableVersionWindowsSummary, StableVersionWindowsViolation, SurfaceKind,
    VersionWindow, VersionWindowExportProjection, VersionWindowExportRow, WindowAction, WindowRow,
    WindowState, STABLE_VERSION_WINDOWS_JSON, STABLE_VERSION_WINDOWS_PATH,
    STABLE_VERSION_WINDOWS_RECORD_KIND, STABLE_VERSION_WINDOWS_SCHEMA_VERSION,
};

pub use support_class_ledger::{
    current_support_class_ledger, ArchetypeCertification, CertificationStatus, CertifiedArchetype,
    CertifiedCutline, DowngradeAction, DowngradeReason as LedgerDowngradeReason, DowngradeRule,
    EvidencePathClass, LedgerOwnerSignoff, LedgerState, LedgerWaiver, PublicationDecision,
    PublicationDecisionRecord as SupportPublicationDecisionRecord, SupportClass, SupportClassEntry,
    SupportClassExportProjection, SupportClassExportRow, SupportClassLedger,
    SupportClassLedgerSummary, SupportClassLedgerViolation, SupportEvidence,
    SUPPORT_CLASS_LEDGER_JSON, SUPPORT_CLASS_LEDGER_PATH, SUPPORT_CLASS_LEDGER_RECORD_KIND,
    SUPPORT_CLASS_LEDGER_SCHEMA_VERSION,
};

pub use browser_mobile_companion_surface_qualification::{
    current_browser_mobile_companion_surface_qualification,
    BrowserMobileCompanionSurfaceQualification, CompanionAuthority, CompanionClientKind,
    CompanionFreshness, CompanionProjection, CompanionQualificationSummary,
    CompanionQualificationViolation, CompanionScope, CompanionSurfaceRow, CompanionVisibleLabel,
    DesktopHandoffTruth, BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_JSON,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_PATH,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery::{
    current_accessibility_surface_signoffs, AccessibilitySurfaceSignoffExportProjection,
    AccessibilitySurfaceSignoffExportRow, AccessibilitySurfaceSignoffRule,
    AccessibilitySurfaceSignoffRow, AccessibilitySurfaceSignoffs,
    AccessibilitySurfaceSignoffsSummary, AccessibilitySurfaceSignoffsViolation,
    DimensionCheck, DimensionKind, DimensionState, GapReason as AccessibilityGapReason,
    SignoffAction, SignoffState, SurfaceKind as AccessibilitySurfaceKind,
    ACCESSIBILITY_SURFACE_SIGNOFFS_JSON, ACCESSIBILITY_SURFACE_SIGNOFFS_PATH,
    ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND, ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION,
};

pub use stabilize_hot_path_performance_against_published_budgets_for::{
    current_hot_path_performance_budgets, BudgetAction, BudgetState, GapReason as HotPathGapReason,
    HotPathBudget, HotPathBudgetRow, HotPathBudgetRule, HotPathExportProjection, HotPathExportRow,
    HotPathKind, HotPathPerformanceBudgets, HotPathPerformanceBudgetsSummary,
    HotPathPerformanceBudgetsViolation, PromotionRecord, HOT_PATH_PERFORMANCE_BUDGETS_JSON,
    HOT_PATH_PERFORMANCE_BUDGETS_PATH, HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND,
    HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION,
};

pub use stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication::{
    current_stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication,
    StabilizeAction, StabilizeExportProjection, StabilizeExportRow, StabilizeGapReason,
    StabilizeKind, StabilizePublicationRecord, StabilizeRow, StabilizeRule, StabilizeState,
    StabilizeSummary, StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication,
    StabilizeViolation, STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_JSON,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_PATH,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_RECORD_KIND,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_SCHEMA_VERSION,
};

pub use freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph::{
    current_m5_feature_train_matrix, M5Action, M5DependencyEdge, M5DependencyKind,
    M5FeatureTrainExportProjection, M5FeatureTrainExportRow, M5FeatureTrainMatrix,
    M5FeatureTrainMatrixSummary, M5FeatureTrainMatrixViolation, M5GapReason, M5LaneKind, M5LaneRow,
    M5Scorecard, M5ScorecardState, M5StopRule,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_JSON,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_PATH,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_RECORD_KIND,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_SCHEMA_VERSION,
};
pub use freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules::{
    current_m5_rollback_downgrade_register, DowngradeKind, M5ClaimNarrowingRule, M5DowngradeRule,
    M5PromotionStage, M5RollbackAction, M5RollbackDowngradeExportProjection,
    M5RollbackDowngradeExportRow, M5RollbackDowngradeRegister, M5RollbackDowngradeRow,
    M5RollbackDowngradeState, M5RollbackDowngradeSummary, M5RollbackDowngradeViolation,
    M5RollbackGapReason, M5RollbackStopRule, PromotionStageKind, RollbackPathState, StageState,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_JSON,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_PATH,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_RECORD_KIND,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_SCHEMA_VERSION,
};
pub use generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains::{
    current_m5_depth_train_automation_register, AutomationAction, AutomationGapReason,
    AutomationState, AutomationStopRule, BackportEligibility, BackportKind, EvidenceExpiryRecord,
    EvidenceKind, M5DepthTrainAutomationExportProjection, M5DepthTrainAutomationExportRow,
    M5DepthTrainAutomationRegister, M5DepthTrainAutomationSummary, M5DepthTrainAutomationViolation,
    M5DepthTrainRow,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_JSON,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_PATH,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION,
};

pub use publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family::{
    current_m5_template_register, CompatibilityPacketSectionKind, CompatibilityPacketTemplate,
    CompatibilityPacketTemplateSection, M5FamilyKind, M5FamilyTemplateRow, M5TemplateRegister,
    M5TemplateRegisterExportProjection, M5TemplateRegisterExportRow, M5TemplateRegisterSummary,
    M5TemplateRegisterViolation, ScorecardSectionKind, ScorecardTemplate, ScorecardTemplateSection,
    TemplateAction, TemplateGapReason, TemplateRegisterState, TemplateSectionState,
    TemplateStopRule,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_JSON,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_PATH,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_RECORD_KIND,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_SCHEMA_VERSION,
};

pub use publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes::{
    current_m5_admin_policy_story_register, AdminPolicyAction, AdminPolicyGapReason,
    AdminPolicyLaneState, AdminPolicyStory, AdminPolicyStoryItem, AdminPolicyStoryItemKind,
    AdminPolicyStoryItemState, AdminPolicyStopRule, M5AdminPolicyLaneKind, M5AdminPolicyLaneRow,
    M5AdminPolicyRegisterExportProjection, M5AdminPolicyRegisterExportRow,
    M5AdminPolicyRegisterSummary, M5AdminPolicyRegisterViolation, M5AdminPolicyStoryRegister,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_JSON,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_PATH,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION,
};

pub use publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts::{
    current_m5_storage_retention_matrix, ArtifactRetentionAction, ArtifactRetentionGapReason,
    ArtifactRetentionPosture, ArtifactRetentionState, ArtifactRetentionStopRule,
    M5ArtifactRetentionExportProjection, M5ArtifactRetentionExportRow, M5ArtifactRetentionRow,
    M5ArtifactRetentionSummary, M5ArtifactRetentionViolation, M5DurableArtifactKind,
    M5StorageRetentionMatrix, RetentionPostureIndicator, RetentionPostureIndicatorKind,
    RetentionPostureIndicatorState,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_JSON,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_PATH,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_RECORD_KIND,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_SCHEMA_VERSION,
};

pub use publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan::{
    current_m5_feature_family_register, M5FeatureFamilyAction, M5FeatureFamilyGapReason,
    M5FeatureFamilyKind, M5FeatureFamilyRegister, M5FeatureFamilyRegisterExportProjection,
    M5FeatureFamilyRegisterExportRow, M5FeatureFamilyRegisterSummary,
    M5FeatureFamilyRegisterViolation, M5FeatureFamilyRow, M5FeatureFamilyState,
    M5FeatureFamilyStopRule, ProofCorpusItemKind, ProofCorpusItemState, ProofCorpusPlan,
    ProofCorpusPlanEntry,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_JSON,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_PATH,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_RECORD_KIND,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_SCHEMA_VERSION,
};

pub use publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet::{
    current_m5_control_packet_register, ControlPacketAction, ControlPacketGapReason,
    ControlPacketItem, ControlPacketItemKind, ControlPacketItemState, ControlPacketLaneState,
    ControlPacketStopRule, ControlPacketStory, M5ControlPacketLaneKind, M5ControlPacketLaneRow,
    M5ControlPacketRegister, M5ControlPacketRegisterExportProjection,
    M5ControlPacketRegisterExportRow, M5ControlPacketRegisterSummary,
    M5ControlPacketRegisterViolation,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_JSON,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_PATH,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_RECORD_KIND,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

pub use seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails::{
    current_m5_health_bundle_matrix, CertifiedArchetypeKind, HealthBundle, HealthBundleAction,
    HealthBundleGapReason, HealthBundleKind, HealthBundleRow, HealthBundleRowState,
    HealthIndicator, HealthIndicatorKind, HealthIndicatorState, M5HealthBundleMatrix,
    M5HealthBundleMatrixExportProjection, M5HealthBundleMatrixExportRow,
    M5HealthBundleMatrixSummary, M5HealthBundleMatrixViolation, RegressionGuardrailRule,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_JSON,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_PATH,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_RECORD_KIND,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_SCHEMA_VERSION,
};

pub use stabilize_the_release_center_promotion_evidence_canary_pilot::{
    current_ring_promotion_control, Action as PromotionAction, GapReason as PromotionGapReason,
    KillSwitchPosture, PromotionDecision as RingPromotionDecision, PromotionPublicationRecord,
    PromotionRule, PromotionState, PromotionSubjectExportRow, PromotionSubjectKind,
    PromotionSubjectRow, Ring, RingPromotionControl, RingPromotionControlExportProjection,
    RingPromotionControlSummary, RingPromotionControlViolation, RollbackStopTrigger,
    RollbackTriggerKind, SoakWindow, RING_PROMOTION_CONTROL_JSON, RING_PROMOTION_CONTROL_PATH,
    RING_PROMOTION_CONTROL_RECORD_KIND, RING_PROMOTION_CONTROL_SCHEMA_VERSION,
};
