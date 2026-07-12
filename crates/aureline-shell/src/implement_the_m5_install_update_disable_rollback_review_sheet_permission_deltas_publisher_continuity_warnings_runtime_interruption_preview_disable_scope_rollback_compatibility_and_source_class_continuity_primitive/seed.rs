//! Canonical seed builders for the M5 install / update / disable / rollback review-sheet controls
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolver so the packet can only carry projections the resolver actually produces. Clean sheets
//! carry the reviewed transaction grammar for every mutation flow, name the disable scope on a
//! disable and the rollback compatibility on an update / rollback, and keep the registry source
//! class and publisher continuity explicit, so a lifecycle mutation is never one-click opaque.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_PACKET_ID: &str =
    "m5-install-update-disable-rollback-review-sheet-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn sheet(input: M5InstallReviewSheetResolutionInput) -> M5ResolvedInstallReviewSheet {
    resolve_install_review_sheet(input).expect("seed install-review sheet resolves")
}

/// The full reviewed transaction grammar plus every inspect action.
fn full_actions() -> Vec<M5InstallReviewAction> {
    M5InstallReviewAction::ALL.to_vec()
}

// -- Clean review-sheet examples ----------------------------------------------------------------

/// Clean install sheet: public source, compatible, no permission change, verified publisher.
fn sheet_install_clean() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:install-acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Clean update sheet: enterprise source, transitive widening named, transferred publisher warned,
/// exact rollback disclosed.
fn sheet_update_clean() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:update-acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Update,
        registry_source: M5RegistrySourceClass::EnterpriseRegistry,
        compatibility: M5CompatibilityState::CompatibleWithWarnings,
        permission_delta: M5InstallReviewPermissionDelta::WidenedTransitive,
        publisher_continuity: M5PublisherContinuityState::Transferred,
        runtime_interruption: M5InstallReviewRuntimeInterruption::RestartRequired,
        disable_scope: None,
        rollback_compatibility: Some(M5RollbackCompatibilityState::RollbackExact),
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Clean disable sheet: mirrored source, workspace-scoped disable, active sessions ended.
fn sheet_disable_clean() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:disable-acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Disable,
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ActiveSessionsEnded,
        disable_scope: Some(M5DisableScopeClass::DisableWorkspace),
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Clean rollback sheet: public source, data-loss rollback honestly disclosed before commit.
fn sheet_rollback_clean() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:rollback-acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Rollback,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::RestartRequired,
        disable_scope: None,
        rollback_compatibility: Some(M5RollbackCompatibilityState::RollbackDataLoss),
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

// -- Degraded review-sheet examples -------------------------------------------------------------

/// Degraded: the artifact identity is unstated.
fn sheet_identity_unstated() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: the registry source class cannot be resolved.
fn sheet_source_unresolved() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:source-unresolved".to_owned(),
        artifact_identity: "sourceless-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::SourceUnknown,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: the registry source class is collapsed across public / mirrored / enterprise.
fn sheet_source_collapsed() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:source-collapsed".to_owned(),
        artifact_identity: "collapsed-source-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Update,
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: Some(M5RollbackCompatibilityState::RollbackCompatible),
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: true,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: the reviewed transaction grammar is incomplete (no cancel action).
fn sheet_grammar_incomplete() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:grammar-incomplete".to_owned(),
        artifact_identity: "one-click-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: vec![
            M5InstallReviewAction::ReviewTransaction,
            M5InstallReviewAction::ConfirmMutation,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: the permission delta cannot be verified.
fn sheet_permission_delta_unverified() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:permission-unverified".to_owned(),
        artifact_identity: "permissionless-delta-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Update,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::DeltaUnknown,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: Some(M5RollbackCompatibilityState::RollbackExact),
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: an incompatible artifact reads as ready to mutate.
fn sheet_incompatible_shown_ready() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:incompatible-ready".to_owned(),
        artifact_identity: "incompatible-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Incompatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: true,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: a transferred publisher reads as continuous with no warning.
fn sheet_publisher_transfer_hidden() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:transfer-hidden".to_owned(),
        artifact_identity: "silently-transferred-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Update,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::Transferred,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: Some(M5RollbackCompatibilityState::RollbackExact),
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: true,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: the runtime-interruption preview cannot be resolved before commit.
fn sheet_runtime_interruption_unresolved() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:interruption-unresolved".to_owned(),
        artifact_identity: "interruption-unknown-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::InterruptionUnknown,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: a disable flow leaves its disable scope unstated.
fn sheet_disable_scope_unstated() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:disable-scope-unstated".to_owned(),
        artifact_identity: "scopeless-disable-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Disable,
        registry_source: M5RegistrySourceClass::EnterpriseRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ActiveSessionsEnded,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: a rollback flow leaves its rollback compatibility unresolved.
fn sheet_rollback_compat_unresolved() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:rollback-unresolved".to_owned(),
        artifact_identity: "rollbackless-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Rollback,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::RestartRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

/// Degraded: a data-loss rollback reads as a clean revert.
fn sheet_rollback_incompatibility_hidden() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:rollback-hidden".to_owned(),
        artifact_identity: "silent-data-loss-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Rollback,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::RestartRequired,
        disable_scope: None,
        rollback_compatibility: Some(M5RollbackCompatibilityState::RollbackIncompatible),
        review_actions: full_actions(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: true,
        proof_fresh: true,
    })
}

/// Degraded: Certified language is left in place on stale evidence.
fn sheet_stale_certified() -> M5ResolvedInstallReviewSheet {
    sheet(M5InstallReviewSheetResolutionInput {
        sheet_id: "review-sheet:stale-certified".to_owned(),
        artifact_identity: "stale-certified-artifact".to_owned(),
        mutation_flow: M5InstallReviewMutationFlow::Install,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        permission_delta: M5InstallReviewPermissionDelta::NoChange,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        runtime_interruption: M5InstallReviewRuntimeInterruption::ReloadRequired,
        disable_scope: None,
        rollback_compatibility: None,
        review_actions: full_actions(),
        certified_or_supported_claimed: true,
        evidence_fresh: false,
        reads_incompatible_as_ready: false,
        reads_transferred_as_continuous: false,
        collapses_source_class: false,
        reads_rollback_as_clean: false,
        proof_fresh: true,
    })
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5InstallReviewSheetConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    review_sheet_examples: Vec<M5ResolvedInstallReviewSheet>,
) -> M5InstallReviewSheetControlsRow {
    M5InstallReviewSheetControlsRow {
        consumer_surface,
        qualification: M5MarketplaceInstallQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5MarketplaceInstallDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5MarketplaceInstallRequiredLabel::Identity,
            M5MarketplaceInstallRequiredLabel::State,
            M5MarketplaceInstallRequiredLabel::KeyboardRoute,
            M5MarketplaceInstallRequiredLabel::CompatibilityAndHost,
            M5MarketplaceInstallRequiredLabel::PermissionAndBudget,
            M5MarketplaceInstallRequiredLabel::PublisherAndSourceClass,
        ],
        accessibility_routes: M5MarketplaceInstallAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5InstallReviewAnatomyPart::ALL.to_vec(),
        export_fields: M5InstallReviewExportField::ALL.to_vec(),
        downgrade_triggers,
        review_sheet_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_REF,
            M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
        ]),
        hides_permission_delta_or_runtime_interruption: false,
        hides_publisher_transfer_disable_scope_or_rollback_incompatibility: false,
        collapses_registry_source_class_across_public_mirrored_enterprise: false,
        presents_incompatible_or_over_budget_as_ready: false,
    }
}

fn controls_rows() -> Vec<M5InstallReviewSheetControlsRow> {
    use M5MarketplaceInstallConsumerSurface as C;
    use M5MarketplaceInstallDowngradeTrigger as D;

    vec![
        base_row(
            C::MarketplaceUi,
            "Marketplace catalog owner",
            "The marketplace listing opens the same reviewed install transaction as every other surface, naming the public / mirrored / enterprise source class, the permission delta, the runtime-interruption preview, and the publisher continuity before install, and degrades honestly when an incompatible artifact reads as ready",
            "evidence:m5-install-review-sheet-marketplace-ui:001",
            vec![
                D::CompatibilityRangeUnstated,
                D::RegistrySourceClassCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![sheet_install_clean(), sheet_incompatible_shown_ready()],
        ),
        base_row(
            C::ExtensionsUi,
            "Extensions manager owner",
            "The extensions manager reuses the same reviewed transaction grammar for updates, names the transitive permission widening and the exact rollback path before commit, and degrades honestly when the review / confirm / cancel grammar is incomplete",
            "evidence:m5-install-review-sheet-extensions-ui:001",
            vec![
                D::PermissionWideningHidden,
                D::RollbackIncompatibilityHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![sheet_update_clean(), sheet_grammar_incomplete()],
        ),
        base_row(
            C::InstallReviewUi,
            "Install-review owner",
            "The install-review sheet is the canonical mutation surface: it names the disable scope on a disable and the rollback compatibility on a rollback before commit, keeps a data-loss rollback disclosed, and degrades honestly when a disable leaves its scope unstated or a rollback leaves its compatibility unresolved",
            "evidence:m5-install-review-sheet-install-review-ui:001",
            vec![
                D::DisableScopeUnstated,
                D::RollbackIncompatibilityHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                sheet_disable_clean(),
                sheet_rollback_clean(),
                sheet_disable_scope_unstated(),
                sheet_rollback_compat_unresolved(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved review-sheet truth, so a collapsed source class, a hidden publisher transfer, a hidden data-loss rollback, an unverified permission delta, or a stale Certified overclaim is visible in evidence rather than hidden behind compact chrome from review through help / support / export handoff",
            "evidence:m5-install-review-sheet-support-export:001",
            vec![
                D::RegistrySourceClassCollapsed,
                D::PublisherTransferHidden,
                D::PermissionWideningHidden,
                D::RollbackIncompatibilityHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                sheet_source_collapsed(),
                sheet_publisher_transfer_hidden(),
                sheet_rollback_incompatibility_hidden(),
                sheet_permission_delta_unverified(),
                sheet_stale_certified(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product lifecycle owner",
            "In-product install / update / disable / rollback surfaces reuse the same reviewed transaction grammar and keep the registry source class explicit, and degrade honestly when the source class is unresolved, the runtime-interruption preview is unavailable, or the artifact identity is unstated so no opaque mutation is quietly carried forward",
            "evidence:m5-install-review-sheet-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::RegistrySourceClassCollapsed,
                D::ProofStale,
            ],
            vec![
                sheet_install_clean(),
                sheet_runtime_interruption_unresolved(),
                sheet_source_unresolved(),
                sheet_identity_unstated(),
            ],
        ),
    ]
}

fn governance_review() -> M5InstallReviewSheetGovernanceReview {
    M5InstallReviewSheetGovernanceReview {
        one_reviewed_transaction_grammar_across_flows: true,
        names_permission_delta: true,
        warns_on_publisher_continuity_change: true,
        previews_runtime_interruption_before_commit: true,
        disable_scope_always_explicit: true,
        rollback_compatibility_always_explicit: true,
        source_class_always_explicit: true,
        incompatible_never_shown_ready: true,
        stale_evidence_never_leaves_certified_language: true,
        source_continuity_visible_through_handoff: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5InstallReviewSheetConsumerProjection {
    M5InstallReviewSheetConsumerProjection {
        install_surfaces_consume_review_vocabulary: true,
        disable_rollback_traces_to_single_contract: true,
        source_continuity_carried_into_handoff: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5InstallReviewSheetProofFreshness {
    M5InstallReviewSheetProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5InstallReviewSheetReleasePosture {
    M5InstallReviewSheetReleasePosture {
        proof_packet_ref: M5_INSTALL_REVIEW_SHEET_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_INSTALL_REVIEW_SHEET_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_REF,
        M5_INSTALL_REVIEW_SHEET_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 install / update / disable / rollback review-sheet controls packet.
pub fn seeded_m5_install_review_sheet_controls() -> M5InstallReviewSheetControlsPacket {
    M5InstallReviewSheetControlsPacket::new(M5InstallReviewSheetControlsPacketInput {
        packet_id: M5_INSTALL_REVIEW_SHEET_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 install / update / disable / rollback review-sheet controls with one reviewed transaction grammar, permission deltas, publisher-continuity warnings, runtime-interruption preview, disable-scope clarity, rollback-compatibility truth, and public / mirror / enterprise source-class continuity across marketplace, install, help, and export"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5InstallReviewSheetVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the install-review row is held at Beta pending disable-scope / rollback parity
/// on every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_install_review_sheet_controls_install_review_ui_beta_narrowed(
) -> M5InstallReviewSheetControlsPacket {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.packet_id =
        "m5-install-update-disable-rollback-review-sheet-controls:install-review-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .expect("install-review row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Beta;
    packet
}

/// Narrowed variant: the marketplace row is narrowed to Preview pending source-class parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_install_review_sheet_controls_marketplace_ui_preview_narrowed(
) -> M5InstallReviewSheetControlsPacket {
    let mut packet = seeded_m5_install_review_sheet_controls();
    packet.packet_id =
        "m5-install-update-disable-rollback-review-sheet-controls:marketplace-ui-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .expect("marketplace-ui row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Preview;
    packet
}
