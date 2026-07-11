//! Canonical seed builders for the M5 compatibility-label-strip / publisher-continuity-row controls
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean strips
//! and rows that describe a deprecated or transferred artifact are built carrying the replacement /
//! continuity language, so trust is never quietly carried forward.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_PACKET_ID: &str =
    "m5-compatibility-label-strip-publisher-continuity-row-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn strip(input: M5CompatibilityLabelStripResolutionInput) -> M5ResolvedCompatibilityLabelStrip {
    resolve_compatibility_label_strip(input).expect("seed compatibility label strip resolves")
}

fn prow(input: M5PublisherContinuityRowResolutionInput) -> M5ResolvedPublisherContinuityRow {
    resolve_publisher_continuity_row(input).expect("seed publisher continuity row resolves")
}

// -- Clean compatibility-label strip examples --------------------------------------------------

/// Clean strip: compatible, sandboxed, active, certified on fresh evidence.
fn strip_acme_active_clean() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: ">=1.4.0, <2.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Clean strip: deprecated but carrying a visible replacement path on fresh evidence.
fn strip_deprecated_with_replacement_clean() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:legacy-fmt".to_owned(),
        artifact_identity: "legacy-fmt".to_owned(),
        compatibility: M5CompatibilityState::CompatibleWithWarnings,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        host_version_range: ">=0.8.0, <1.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v2".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Deprecated,
        replacement_path: "replaced by modern-fmt >=1.0".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Clean strip: preview, remote host, not certified.
fn strip_preview_clean() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:corp-preview".to_owned(),
        artifact_identity: "corp-preview".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::RemoteHost,
        host_version_range: ">=3.0.0, <4.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v4".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Preview,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

// -- Degraded compatibility-label strip examples -----------------------------------------------

/// Degraded strip: the artifact identity is unstated.
fn strip_identity_unstated() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: ">=1.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded strip: the host-version range is unstated.
fn strip_host_version_unstated() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:no-host-range".to_owned(),
        artifact_identity: "rangeless-artifact".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: "".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded strip: the manifest-schema version is unstated.
fn strip_manifest_unstated() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:no-manifest-schema".to_owned(),
        artifact_identity: "schemaless-artifact".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        host_version_range: ">=1.0.0".to_owned(),
        manifest_schema_version: "".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded strip: an incompatible artifact reads as ready to install.
fn strip_incompatible_ready() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:incompatible-ready".to_owned(),
        artifact_identity: "incompatible-artifact".to_owned(),
        compatibility: M5CompatibilityState::Incompatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: ">=9.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: true,
        proof_fresh: true,
    })
}

/// Degraded strip: the lifecycle state is unstated.
fn strip_lifecycle_unstated() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:lifecycle-unstated".to_owned(),
        artifact_identity: "lifecycle-less-artifact".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: ">=1.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::LifecycleUnknown,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded strip: a deprecated artifact carries no replacement path.
fn strip_replacement_missing() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:replacement-missing".to_owned(),
        artifact_identity: "orphaned-eol-artifact".to_owned(),
        compatibility: M5CompatibilityState::CompatibleWithWarnings,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        host_version_range: ">=0.5.0".to_owned(),
        manifest_schema_version: "manifest-schema v1".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::EndOfLife,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded strip: Certified language is left in place on stale evidence.
fn strip_stale_certified() -> M5ResolvedCompatibilityLabelStrip {
    strip(M5CompatibilityLabelStripResolutionInput {
        strip_id: "compat-strip:stale-certified".to_owned(),
        artifact_identity: "stale-certified-artifact".to_owned(),
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        host_version_range: ">=2.0.0".to_owned(),
        manifest_schema_version: "manifest-schema v3".to_owned(),
        lifecycle: M5CompatibilityLifecycleState::Active,
        replacement_path: "".to_owned(),
        certified_or_supported_claimed: true,
        evidence_fresh: false,
        reads_incompatible_as_ready: false,
        proof_fresh: true,
    })
}

// -- Clean publisher-continuity row examples ---------------------------------------------------

/// Clean row: a verified publisher from an enterprise registry, certified on fresh evidence.
fn row_verified_enterprise_clean() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:corp-tool".to_owned(),
        artifact_identity: "corp-tool".to_owned(),
        continuity: M5PublisherContinuityState::VerifiedPublisher,
        registry_source: M5RegistrySourceClass::EnterpriseRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Clean row: a transferred publisher from the public registry carrying continuity language and
/// stated transfer history.
fn row_transferred_with_continuity_clean() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        continuity: M5PublisherContinuityState::Transferred,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "maintainership transferred to acme-foundation".to_owned(),
        transfer_history_available: true,
        transfer_history_stated: true,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Clean row: a continuous publisher preserved through a mirror registry.
fn row_mirror_continuous_clean() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:mirror-fmt".to_owned(),
        artifact_identity: "mirror-fmt".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Clean row: a continuous publisher on the public registry, certified on fresh evidence.
fn row_continuous_public_clean() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:steady-tool".to_owned(),
        artifact_identity: "steady-tool".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

// -- Degraded publisher-continuity row examples ------------------------------------------------

/// Degraded row: the registry source cannot be resolved.
fn row_source_unknown() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:source-unknown".to_owned(),
        artifact_identity: "pending-artifact".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::SourceUnknown,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Degraded row: the source class is collapsed into one ambiguous origin.
fn row_source_collapsed() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:source-collapsed".to_owned(),
        artifact_identity: "collapsed-origin-artifact".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: true,
        proof_fresh: true,
    })
}

/// Degraded row: a transferred publisher hides its continuity language.
fn row_continuity_hidden() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:continuity-hidden".to_owned(),
        artifact_identity: "transferred-artifact".to_owned(),
        continuity: M5PublisherContinuityState::Transferred,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: true,
        transfer_history_stated: true,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Degraded row: a deprecated publisher hides its continuity language.
fn row_deprecated_hidden() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:deprecated-hidden".to_owned(),
        artifact_identity: "deprecated-artifact".to_owned(),
        continuity: M5PublisherContinuityState::Deprecated,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Degraded row: available transfer history is hidden.
fn row_history_hidden() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:history-hidden".to_owned(),
        artifact_identity: "history-hidden-artifact".to_owned(),
        continuity: M5PublisherContinuityState::Transferred,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "maintainership transferred to new-owner".to_owned(),
        transfer_history_available: true,
        transfer_history_stated: false,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Degraded row: Certified language is left in place on stale evidence.
fn row_stale_certified() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:stale-certified".to_owned(),
        artifact_identity: "stale-certified-publisher".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: true,
        evidence_fresh: false,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Degraded row: Certified language is left in place on an unverifiable continuity.
fn row_unverifiable_certified() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:unverifiable-certified".to_owned(),
        artifact_identity: "unverifiable-publisher".to_owned(),
        continuity: M5PublisherContinuityState::ContinuityUnknown,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

/// Degraded row: the artifact identity is unstated.
fn row_identity_unstated() -> M5ResolvedPublisherContinuityRow {
    prow(M5PublisherContinuityRowResolutionInput {
        row_id: "publisher-row:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        continuity: M5PublisherContinuityState::Continuous,
        registry_source: M5RegistrySourceClass::PublicRegistry,
        continuity_language: "".to_owned(),
        transfer_history_available: false,
        transfer_history_stated: false,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        collapses_source_class: false,
        proof_fresh: true,
    })
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5CompatibilityContinuityConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    compatibility_label_strip_examples: Vec<M5ResolvedCompatibilityLabelStrip>,
    publisher_continuity_row_examples: Vec<M5ResolvedPublisherContinuityRow>,
) -> M5CompatibilityContinuityControlsRow {
    M5CompatibilityContinuityControlsRow {
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
        anatomy_parts: M5CompatibilityContinuityAnatomyPart::ALL.to_vec(),
        export_fields: M5CompatibilityContinuityExportField::ALL.to_vec(),
        downgrade_triggers,
        compatibility_label_strip_examples,
        publisher_continuity_row_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_REF,
            M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
            M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
        ]),
        collapses_registry_source_class_across_public_mirrored_enterprise: false,
        hides_replacement_path_or_lifecycle_state: false,
        hides_publisher_transfer_or_continuity_language: false,
        leaves_stale_evidence_certified_or_supported: false,
    }
}

fn controls_rows() -> Vec<M5CompatibilityContinuityControlsRow> {
    use M5MarketplaceInstallConsumerSurface as C;
    use M5MarketplaceInstallDowngradeTrigger as D;

    vec![
        base_row(
            C::MarketplaceUi,
            "Marketplace catalog owner",
            "The marketplace listing renders one compatibility-label strip per artifact naming the compatibility range, host / runtime model, host-version and manifest-schema range, lifecycle state, and replacement path, and one publisher-continuity row naming verified, transferred, lost, mirrored, or unverifiable continuity so a compare decision needs no disconnected page",
            "evidence:m5-compatibility-continuity-marketplace-ui:001",
            vec![
                D::CompatibilityRangeUnstated,
                D::HostModelUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![strip_acme_active_clean(), strip_host_version_unstated()],
            vec![row_transferred_with_continuity_clean(), row_source_unknown()],
        ),
        base_row(
            C::ExtensionsUi,
            "Extensions manager owner",
            "The extensions detail surface reuses the same lifecycle grammar, shows a deprecated artifact carrying its replacement path, names a transferred publisher's continuity language, and degrades honestly when the replacement path or continuity language is hidden",
            "evidence:m5-compatibility-continuity-extensions-ui:001",
            vec![
                D::PublisherTransferHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                strip_deprecated_with_replacement_clean(),
                strip_replacement_missing(),
            ],
            vec![row_verified_enterprise_clean(), row_continuity_hidden()],
        ),
        base_row(
            C::InstallReviewUi,
            "Install-review owner",
            "The install-review sheet keeps compatibility and continuity explicit before install trust silently continues, degrading honestly when the manifest-schema version or lifecycle state cannot be resolved or available transfer history is hidden",
            "evidence:m5-compatibility-continuity-install-review-ui:001",
            vec![
                D::CompatibilityRangeUnstated,
                D::PublisherTransferHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![strip_preview_clean(), strip_manifest_unstated()],
            vec![row_mirror_continuous_clean(), row_history_hidden()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved strip and row truth, so an incompatible-shown-ready strip, a missing replacement path, a hidden continuity language, or a stale or unverifiable Certified overclaim is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-compatibility-continuity-support-export:001",
            vec![
                D::CompatibilityRangeUnstated,
                D::PublisherTransferHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                strip_incompatible_ready(),
                strip_stale_certified(),
                strip_lifecycle_unstated(),
            ],
            vec![
                row_source_collapsed(),
                row_stale_certified(),
                row_unverifiable_certified(),
                row_deprecated_hidden(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product diagnostics owner",
            "In-product listing and diagnostics surfaces reuse the same fact grammar, keep continuous and verified publishers explicit, and degrade honestly when the artifact identity is missing so no stale trust is quietly carried forward into installed-state diagnostics",
            "evidence:m5-compatibility-continuity-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::CompatibilityRangeUnstated,
                D::ProofStale,
            ],
            vec![strip_acme_active_clean(), strip_identity_unstated()],
            vec![row_continuous_public_clean(), row_identity_unstated()],
        ),
    ]
}

fn governance_review() -> M5CompatibilityContinuityGovernanceReview {
    M5CompatibilityContinuityGovernanceReview {
        strip_names_compatibility_host_and_ranges: true,
        strip_names_lifecycle_and_replacement: true,
        row_names_continuity_and_source_class: true,
        row_names_transfer_history_where_available: true,
        deprecated_or_transferred_carry_replacement_language: true,
        source_class_always_explicit_never_collapsed: true,
        incompatible_never_ready: true,
        stale_evidence_never_leaves_certified_language: true,
        states_explicit_across_all_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5CompatibilityContinuityConsumerProjection {
    M5CompatibilityContinuityConsumerProjection {
        marketplace_surfaces_consume_compatibility_and_lifecycle_vocabulary: true,
        registry_surfaces_consume_publisher_and_source_vocabulary: true,
        facts_trace_to_single_component_contract: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CompatibilityContinuityProofFreshness {
    M5CompatibilityContinuityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CompatibilityContinuityReleasePosture {
    M5CompatibilityContinuityReleasePosture {
        proof_packet_ref: M5_COMPATIBILITY_CONTINUITY_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_COMPATIBILITY_CONTINUITY_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_REF,
        M5_COMPATIBILITY_CONTINUITY_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
        M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 compatibility-label-strip / publisher-continuity-row controls packet.
pub fn seeded_m5_compatibility_continuity_controls() -> M5CompatibilityContinuityControlsPacket {
    M5CompatibilityContinuityControlsPacket::new(M5CompatibilityContinuityControlsPacketInput {
        packet_id: M5_COMPATIBILITY_CONTINUITY_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 compatibility-label-strip and publisher-continuity-row controls with host/version range, manifest-schema, lifecycle and replacement path, publisher continuity and transfer history, and no-stale-certified-overclaim across listing, detail, install, diagnostics, and export"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5CompatibilityContinuityVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the marketplace-UI row is held at Beta pending compatibility / continuity
/// parity on every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed(
) -> M5CompatibilityContinuityControlsPacket {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.packet_id =
        "m5-compatibility-label-strip-publisher-continuity-row-controls:marketplace-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .expect("marketplace-ui row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Beta;
    packet
}

/// Narrowed variant: the install-review row is narrowed to Preview pending publisher-continuity
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed(
) -> M5CompatibilityContinuityControlsPacket {
    let mut packet = seeded_m5_compatibility_continuity_controls();
    packet.packet_id =
        "m5-compatibility-label-strip-publisher-continuity-row-controls:install-review-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .expect("install-review row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Preview;
    packet
}
