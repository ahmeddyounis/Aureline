//! Deployment-profile continuity corpus: the checked-in matrix that proves
//! every marketed beta deployment row (individual-local, self-hosted /
//! sovereign, hybrid enterprise-online, mirror-only enterprise-online,
//! air-gapped, managed-cloud) has current evidence covering host posture,
//! residual hosted dependencies, control-plane vs. data-plane outage
//! separation, and the safest bounded next action.
//!
//! The packet returned by [`seeded_deployment_profile_corpus_packet`] is
//! the source of truth for:
//!
//! - `/fixtures/deployment/m3/profile_truth/` — one per-profile
//!   [`DeploymentProfilePage`] case per marketed row and surface lens, with
//!   a top-level packet that names the cases, the outage drills, and the
//!   residual-dependency matrix;
//! - `/fixtures/deployment/m3/control_plane_vs_data_plane/` — one
//!   [`DeploymentProfilePage`] per outage drill covering control-plane
//!   unavailable, data-plane blocked, mirror-only fallback, offline-cache
//!   only, sign-out / org-switch / seat-loss, region mismatch, and stale
//!   policy / catalog cache;
//! - `/artifacts/release/m3/deployment_profile_conformance_report.md` —
//!   the release-evidence excerpt rendered from this packet;
//! - `/artifacts/release/m3/residual_dependency_matrix.json` — the
//!   per-profile posture matrix rendered from this packet.
//!
//! Every case and drill composes one [`DeploymentProfilePage`] whose
//! `audit()` must return an empty defect set; the fixture-replay test
//! enforces that, so a regression that would let a sovereign claim
//! silently carry vendor-managed keys, an air-gapped claim route through a
//! companion surface, or a managed-cloud outage collapse into a generic
//! "service degraded" copy fails the suite.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{
    consumer_surfaces_present, individual_local_baseline_page, AbsenceImpactClass, ArtifactClass,
    ConsumerSurfaceClass, ContinuityFallbackClass, ControlPlaneServiceClass,
    ControlPlaneServiceStateClass, ControlPlaneSummary, DataPlaneCapabilityClass,
    DataPlaneCapabilityStateClass, DataPlaneSummary, DependencyClass, DeploymentProfileClass,
    DeploymentProfileDefect, DeploymentProfilePage, DigestStateClass, FreshnessSummary,
    InspectOnlyAction, KeyModeClass, MirrorFreshnessClass, MirrorOfflineArtifactRow,
    MirrorOfflineStateClass, MirrorSourceClass, OfflineCachePostureClass, PlaneStatusStrip,
    PostureClass, ProductFacingLabelClass, ProfileSummary, ProhibitedImpliedClaimClass,
    RedactionClass, RegionScopeClass, ResidualDependencyRow, RetentionClass, SafestNextAction,
    SafestNextActionClass, SignerStateClass, StalenessClass, TenantOrgScopeClass,
    VerifyOrOpenManifestAction, DEPLOYMENT_PROFILE_SCHEMA_VERSION,
    MIRROR_OFFLINE_ARTIFACT_ROW_RECORD_KIND, PLANE_STATUS_STRIP_RECORD_KIND,
    PROFILE_SUMMARY_RECORD_KIND, RESIDUAL_DEPENDENCY_ROW_RECORD_KIND,
};

/// Stable record-kind tag for [`DeploymentProfileCorpusPacket`].
pub const DEPLOYMENT_PROFILE_CORPUS_PACKET_RECORD_KIND: &str =
    "deployment_profile_corpus_packet_record";

/// Stable record-kind tag for [`DeploymentProfileCorpusCase`].
pub const DEPLOYMENT_PROFILE_CORPUS_CASE_RECORD_KIND: &str =
    "deployment_profile_corpus_case_record";

/// Stable record-kind tag for [`DeploymentProfileOutageDrill`].
pub const DEPLOYMENT_PROFILE_OUTAGE_DRILL_RECORD_KIND: &str =
    "deployment_profile_outage_drill_record";

/// Stable record-kind tag for [`ResidualDependencyMatrix`].
pub const RESIDUAL_DEPENDENCY_MATRIX_RECORD_KIND: &str = "residual_dependency_matrix_record";

/// Contract anchor: the beta operations document this corpus is bound to.
pub const DEPLOYMENT_PROFILE_CORPUS_SHARED_CONTRACT_REF: &str =
    "docs/ops/m3/deployment_profile_and_continuity_beta.md";

/// Companion qualification doc that consumes this corpus.
pub const DEPLOYMENT_PROFILE_CORPUS_QUALIFICATION_REF: &str =
    "docs/ops/m3/deployment_profile_claim_qualification.md";

/// Reviewer notice rendered on every packet so the corpus's scope is not
/// overstated.
pub const DEPLOYMENT_PROFILE_CORPUS_NOTICE: &str =
    "Deployment-profile continuity corpus: every case and drill exercises the same shell module \
     that About, diagnostics, support packets, admin-audit exports, release-evidence excerpts, \
     status-bar deployment cells, companion surfaces, and CLI text formatters read. The corpus \
     does not invent new runtime facts; it pins the matrix of marketed deployment rows and outage \
     drills that beta-exit reviewers MUST be able to replay.";

const EMITTED_AT: &str = "2026-05-19T00:00:00Z";

/// Surface lens — narrows the consumer surfaces a marketed deployment row
/// is rendered through. Every row in the matrix MUST be exercised under at
/// least one lens so the same vocabulary appears in every product, docs,
/// and support surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceLensClass {
    Desktop,
    CliHeadless,
    CompanionHandoff,
    SupportExport,
}

impl SurfaceLensClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::CompanionHandoff => "companion_handoff",
            Self::SupportExport => "support_export",
        }
    }

    pub const fn all() -> [Self; 4] {
        [
            Self::Desktop,
            Self::CliHeadless,
            Self::CompanionHandoff,
            Self::SupportExport,
        ]
    }
}

/// Closed outage-drill vocabulary covering the impairment scenarios the
/// spec demands proof for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageDrillClass {
    ControlPlaneUnavailable,
    DataPlaneBlockedPendingReconnect,
    MirrorOnlyFallback,
    OfflineCacheOnly,
    SignOutToLocalOnly,
    OrgSwitchBoundaryRecheck,
    SeatLossContinueLocal,
    RegionMismatchBoundaryRecheck,
    StalePolicyCache,
    StaleCatalogCache,
}

impl OutageDrillClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlPlaneUnavailable => "control_plane_unavailable",
            Self::DataPlaneBlockedPendingReconnect => "data_plane_blocked_pending_reconnect",
            Self::MirrorOnlyFallback => "mirror_only_fallback",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::SignOutToLocalOnly => "sign_out_to_local_only",
            Self::OrgSwitchBoundaryRecheck => "org_switch_boundary_recheck",
            Self::SeatLossContinueLocal => "seat_loss_continue_local",
            Self::RegionMismatchBoundaryRecheck => "region_mismatch_boundary_recheck",
            Self::StalePolicyCache => "stale_policy_cache",
            Self::StaleCatalogCache => "stale_catalog_cache",
        }
    }

    pub const fn all() -> [Self; 10] {
        [
            Self::ControlPlaneUnavailable,
            Self::DataPlaneBlockedPendingReconnect,
            Self::MirrorOnlyFallback,
            Self::OfflineCacheOnly,
            Self::SignOutToLocalOnly,
            Self::OrgSwitchBoundaryRecheck,
            Self::SeatLossContinueLocal,
            Self::RegionMismatchBoundaryRecheck,
            Self::StalePolicyCache,
            Self::StaleCatalogCache,
        ]
    }
}

/// One marketed deployment-row case in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileCorpusCase {
    pub record_kind: String,
    pub case_id: String,
    pub deployment_profile: DeploymentProfileClass,
    pub product_facing_label_class: ProductFacingLabelClass,
    pub surface_lens_class: SurfaceLensClass,
    pub scenario_summary: String,
    pub fixture_path: String,
    pub page: DeploymentProfilePage,
}

/// One outage-drill case. The drill names the impairment scenario and the
/// continuity assertion the page MUST prove.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileOutageDrill {
    pub record_kind: String,
    pub drill_id: String,
    pub drill_class: OutageDrillClass,
    pub deployment_profile: DeploymentProfileClass,
    pub scenario_summary: String,
    pub continuity_assertion: String,
    pub local_safe_remains: bool,
    pub control_plane_impaired: bool,
    pub data_plane_impaired: bool,
    pub honesty_marker_present: bool,
    pub fixture_path: String,
    pub page: DeploymentProfilePage,
}

/// One row of the residual-dependency matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependencyMatrixRow {
    pub dependency_class: DependencyClass,
    pub title: String,
    pub per_profile_posture: Vec<PerProfilePostureEntry>,
    pub per_profile_absence_impact: Vec<PerProfileAbsenceImpactEntry>,
    pub per_profile_continuity_fallback: Vec<PerProfileContinuityFallbackEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerProfilePostureEntry {
    pub deployment_profile: DeploymentProfileClass,
    pub posture_class: PostureClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerProfileAbsenceImpactEntry {
    pub deployment_profile: DeploymentProfileClass,
    pub absence_impact_class: AbsenceImpactClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerProfileContinuityFallbackEntry {
    pub deployment_profile: DeploymentProfileClass,
    pub continuity_fallback_class: ContinuityFallbackClass,
}

/// The residual-dependency matrix that backs the
/// `/artifacts/release/m3/residual_dependency_matrix.json` projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependencyMatrix {
    pub record_kind: String,
    pub schema_version: u32,
    pub source_ledger_ref: String,
    pub overview: String,
    pub rows: Vec<ResidualDependencyMatrixRow>,
}

/// Coverage summary projected from the packet for reviewer-facing summary
/// rendering.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DeploymentProfileCoverageSummary {
    pub case_count: usize,
    pub drill_count: usize,
    pub residual_dependency_row_count: usize,
    pub deployment_profiles_present: Vec<DeploymentProfileClass>,
    pub surface_lenses_present: Vec<SurfaceLensClass>,
    pub outage_drill_classes_present: Vec<OutageDrillClass>,
    pub dependency_classes_present: Vec<DependencyClass>,
    pub consumer_surfaces_present: Vec<ConsumerSurfaceClass>,
    pub all_cases_passed_audit: bool,
    pub all_drills_passed_audit: bool,
}

/// Composite corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileCorpusPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub shared_contract_ref: String,
    pub qualification_doc_ref: String,
    pub corpus_cases: Vec<DeploymentProfileCorpusCase>,
    pub outage_drills: Vec<DeploymentProfileOutageDrill>,
    pub residual_dependency_matrix: ResidualDependencyMatrix,
    pub coverage_summary: DeploymentProfileCoverageSummary,
}

// =============================================================================
// Builders.
// =============================================================================

fn inspect_action(action_id: &str, route: &str) -> InspectOnlyAction {
    InspectOnlyAction::open_details(action_id, route)
}

fn verify_action(action_id: &str, label: &str, revalidation: &str) -> VerifyOrOpenManifestAction {
    VerifyOrOpenManifestAction {
        action_id: action_id.to_owned(),
        label: label.to_owned(),
        scope_class: "scope_local_only".to_owned(),
        authority_class: "user_local_authority".to_owned(),
        consent_class: "no_consent_required_safe_default".to_owned(),
        side_effects: vec!["no_side_effect_inspect_only".to_owned()],
        preserves_evidence_context: true,
        modal_prohibited: true,
        revalidation_on_open: revalidation.to_owned(),
        evidence_links: Vec::new(),
    }
}

fn residual_row(
    summary_id: &str,
    dependency: DependencyClass,
    posture: PostureClass,
    impact: AbsenceImpactClass,
    fallback: ContinuityFallbackClass,
    dependent_feature_label: &str,
    unreachable_impact_label: &str,
    freshness_label: Option<&str>,
) -> ResidualDependencyRow {
    let vendor_bound = dependency.is_vendor_bound_when_required() && posture == PostureClass::Required;
    ResidualDependencyRow {
        record_kind: RESIDUAL_DEPENDENCY_ROW_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        row_id: format!("row.dep.{summary_id}.{}", dependency.as_str()),
        profile_summary_ref: summary_id.to_owned(),
        dependency_class: dependency,
        posture_class: posture,
        vendor_or_public_dependence: vendor_bound,
        dependent_feature_label: dependent_feature_label.to_owned(),
        dependent_feature_refs: Vec::new(),
        unreachable_impact_class: impact,
        unreachable_impact_label: unreachable_impact_label.to_owned(),
        continuity_fallback_class: fallback,
        ledger_row_ref: format!("ledger.{}", dependency.as_str()),
        freshness_label: freshness_label.map(str::to_owned),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_outage_notice_ref: None,
        linked_continuity_packet_ref: None,
        evidence_links: Vec::new(),
        notes: None,
    }
}

fn artifact_row(
    summary_id: &str,
    artifact: ArtifactClass,
    signer: SignerStateClass,
    digest: DigestStateClass,
    freshness: MirrorFreshnessClass,
    cache: OfflineCachePostureClass,
    source: MirrorSourceClass,
    artifact_label: &str,
) -> MirrorOfflineArtifactRow {
    let verify_revalidation = if digest == DigestStateClass::DigestMismatch {
        "blocked_until_fresh"
    } else {
        "snapshot_open_read_only"
    };
    MirrorOfflineArtifactRow {
        record_kind: MIRROR_OFFLINE_ARTIFACT_ROW_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        row_id: format!("row.artifact.{summary_id}.{}", artifact.as_str()),
        profile_summary_ref: summary_id.to_owned(),
        artifact_class: artifact,
        artifact_label: artifact_label.to_owned(),
        signer_state_class: signer,
        signer_fingerprint_ref: if signer.requires_fingerprint() {
            Some(format!("fp:{}:{}", summary_id, artifact.as_str()))
        } else {
            None
        },
        digest_state_class: digest,
        digest_ref: if matches!(
            digest,
            DigestStateClass::DigestVerified | DigestStateClass::DigestPending
        ) {
            Some(format!("sha256:{}:{}", summary_id, artifact.as_str()))
        } else {
            None
        },
        mirror_freshness_class: freshness,
        last_refresh_at: if matches!(
            freshness,
            MirrorFreshnessClass::MirrorFreshWithinWindow
                | MirrorFreshnessClass::MirrorWithinExtendedWindow
                | MirrorFreshnessClass::MirrorPastExtendedWindow
        ) {
            Some(EMITTED_AT.to_owned())
        } else {
            None
        },
        offline_cache_posture_class: cache,
        mirror_source_class: source,
        verify_action: verify_action(
            &format!("action.verify.{summary_id}.{}", artifact.as_str()),
            &format!("Verify {artifact_label}"),
            verify_revalidation,
        ),
        open_manifest_action: verify_action(
            &format!("action.manifest.{summary_id}.{}", artifact.as_str()),
            &format!("Open {artifact_label} manifest"),
            "snapshot_open_read_only",
        ),
        evidence_links: Vec::new(),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        notes: None,
    }
}

fn local_core_data_plane(summary_label: &str) -> DataPlaneSummary {
    DataPlaneSummary {
        worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        summary_label: summary_label.to_owned(),
        impaired_capability_classes: Vec::new(),
        available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline()
            .to_vec(),
    }
}

fn lens_consumer_surfaces(lens: SurfaceLensClass) -> Vec<ConsumerSurfaceClass> {
    match lens {
        SurfaceLensClass::Desktop => vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        SurfaceLensClass::CliHeadless => vec![
            ConsumerSurfaceClass::CliTextFormatter,
            ConsumerSurfaceClass::DiagnosticsView,
        ],
        SurfaceLensClass::CompanionHandoff => vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::CompanionSurface,
            ConsumerSurfaceClass::StatusBarCell,
        ],
        SurfaceLensClass::SupportExport => vec![
            ConsumerSurfaceClass::SupportPacketExport,
            ConsumerSurfaceClass::AdminAuditExport,
            ConsumerSurfaceClass::ReleaseEvidenceExcerpt,
        ],
    }
}

// -----------------------------------------------------------------------------
// Per-profile page builders.
// -----------------------------------------------------------------------------

fn individual_local_page(lens: SurfaceLensClass) -> DeploymentProfilePage {
    let mut page = individual_local_baseline_page(EMITTED_AT);
    let surfaces = lens_consumer_surfaces(lens);
    page.profile_summary.consumer_surfaces = surfaces.clone();
    page.plane_status_strip.consumer_surfaces = surfaces;
    // Recompose so the summary picks up any consumer-surface narrowing.
    let (rebuilt, _) = DeploymentProfilePage::compose(
        page.page_id.clone(),
        page.profile_summary.clone(),
        page.plane_status_strip.clone(),
        page.residual_dependency_rows.clone(),
        page.mirror_offline_artifact_rows.clone(),
    );
    rebuilt
}

fn self_hosted_sovereign_page(lens: SurfaceLensClass) -> DeploymentProfilePage {
    let summary_id = "summary.deployment.self_hosted_sovereign_baseline".to_owned();
    let strip_id = "strip.deployment.self_hosted_sovereign_baseline".to_owned();
    let policy_bundle = artifact_row(
        &summary_id,
        ArtifactClass::PolicyBundle,
        SignerStateClass::SignedOrgCaPinned,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::MirrorSnapshotPresent,
        MirrorSourceClass::CustomerOperatedMirror,
        "Customer-signed policy bundle",
    );
    let docs_pack = artifact_row(
        &summary_id,
        ArtifactClass::DocsPack,
        SignerStateClass::SignedOrgCaPinned,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::MirrorSnapshotPresent,
        MirrorSourceClass::CustomerOperatedMirror,
        "Customer-mirrored docs pack",
    );

    let residual = vec![
        residual_row(
            &summary_id,
            DependencyClass::SignIn,
            PostureClass::Required,
            AbsenceImpactClass::BlockedPendingReconnect,
            ContinuityFallbackClass::ResumeAfterReconnect,
            "Customer IdP sign-in for managed flows",
            "Sign-in-gated flows wait for reconnect; local-core editing continues.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::PolicyBundle,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToCachedLastKnownGood,
            ContinuityFallbackClass::ReplayCachedSnapshot,
            "Customer-operated policy distribution",
            "Policy evaluation uses cached last-known-good under a stale label.",
            Some("Last refreshed within freshness window"),
        ),
        residual_row(
            &summary_id,
            DependencyClass::HostedControlPlaneReachability,
            PostureClass::Optional,
            AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
            ContinuityFallbackClass::ContinueLocalNoRestore,
            "Customer-operated control plane",
            "Local-core capabilities continue; remote-attach is paused.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::AiProvider,
            PostureClass::Optional,
            AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
            ContinuityFallbackClass::ContinueLocalNoRestore,
            "Optional managed AI broker",
            "AI surfaces narrow to local-only or off; local edit/search/Git/tasks continue.",
            None,
        ),
    ];

    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::SelfHosted,
        product_facing_label_class: ProductFacingLabelClass::SelfHostedSovereign,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Customer-operated control plane reachable; policy bundle fresh."
                .to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: None,
            freshness_floor_ref: Some("freshness.self_hosted.fresh".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Healthy,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: vec![policy_bundle.row_id.clone(), docs_pack.row_id.clone()],
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedAirGappedWhenEgressAllowed,
            ProhibitedImpliedClaimClass::ImpliedSovereignWhenVendorManaged,
        ],
        open_details_action: inspect_action(
            "action.summary.self_hosted_sovereign.open_details",
            "route.deployment.details.self_hosted",
        ),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: Some(
            "card.deployment.self_hosted_sovereign_baseline".to_owned(),
        ),
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/self_hosted_stale_policy_session.json".to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };

    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Healthy,
            summary_label: "Customer-operated sync, registry, auth, and policy services healthy."
                .to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::RegistryService,
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::PolicyService,
                ControlPlaneServiceClass::DocsPackService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core editing, save, search, Git, tasks, docs inspection, export, and diagnostics available on device.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.self_hosted_sovereign.continue_local",
        ),
        alternate_actions: Vec::new(),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };

    let (page, _defects) = DeploymentProfilePage::compose(
        "page.deployment.self_hosted_sovereign_baseline",
        profile_summary,
        plane_status_strip,
        residual,
        vec![policy_bundle, docs_pack],
    );
    page
}

fn enterprise_online_hybrid_page(lens: SurfaceLensClass) -> DeploymentProfilePage {
    let summary_id = "summary.deployment.enterprise_online_hybrid_baseline".to_owned();
    let strip_id = "strip.deployment.enterprise_online_hybrid_baseline".to_owned();

    let residual = vec![
        residual_row(
            &summary_id,
            DependencyClass::HostedControlPlaneReachability,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
            ContinuityFallbackClass::ContinueLocalNoRestore,
            "Customer-federated hybrid control plane",
            "Local-core capabilities continue; remote-attach and managed flows pause.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::AiProvider,
            PostureClass::Optional,
            AbsenceImpactClass::NarrowsToCachedLastKnownGood,
            ContinuityFallbackClass::ReplayCachedSnapshot,
            "Optional managed AI broker for hybrid tenants",
            "AI surfaces narrow to cached last-known-good with explicit freshness label.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::BrowserHandoff,
            PostureClass::Optional,
            AbsenceImpactClass::NarrowsToCachedLastKnownGood,
            ContinuityFallbackClass::ReplayCachedSnapshot,
            "Cross-device browser companion handoff",
            "Companion handoff narrows to cached read-only with freshness label.",
            None,
        ),
    ];

    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::EnterpriseOnline,
        product_facing_label_class: ProductFacingLabelClass::HybridRemoteAttach,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::VendorRetentionWindowWithCustomerPolicy,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Hybrid control plane reachable; companion handoff fresh.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: None,
            freshness_floor_ref: Some("freshness.enterprise_online.fresh".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Healthy,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedSovereignWhenVendorManaged,
            ProhibitedImpliedClaimClass::ImpliedSelfHostedWhenManagedCloud,
        ],
        open_details_action: inspect_action(
            "action.summary.enterprise_online_hybrid.open_details",
            "route.deployment.details.enterprise_online",
        ),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: Some(
            "card.deployment.enterprise_online_hybrid_baseline".to_owned(),
        ),
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/enterprise_failover_boundary_recheck.json"
                .to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };

    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Healthy,
            summary_label: "Hybrid sync, registry, auth, policy, catalog, and AI broker services healthy."
                .to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::RegistryService,
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::PolicyService,
                ControlPlaneServiceClass::AiBrokerService,
                ControlPlaneServiceClass::CatalogService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities available; remote-attach and companion handoff online.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.enterprise_online_hybrid.continue_local",
        ),
        alternate_actions: Vec::new(),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };

    let (page, _defects) = DeploymentProfilePage::compose(
        "page.deployment.enterprise_online_hybrid_baseline",
        profile_summary,
        plane_status_strip,
        residual,
        Vec::new(),
    );
    page
}

fn enterprise_online_mirrored_page(lens: SurfaceLensClass) -> DeploymentProfilePage {
    let summary_id = "summary.deployment.enterprise_online_mirrored_baseline".to_owned();
    let strip_id = "strip.deployment.enterprise_online_mirrored_baseline".to_owned();

    let updates = artifact_row(
        &summary_id,
        ArtifactClass::Updates,
        SignerStateClass::SignedOrgCaPinned,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::MirrorSnapshotPresent,
        MirrorSourceClass::CustomerOperatedMirror,
        "Customer-mirrored update channel",
    );
    let extensions = artifact_row(
        &summary_id,
        ArtifactClass::Extensions,
        SignerStateClass::SignedOrgCaPinned,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::MirrorSnapshotPresent,
        MirrorSourceClass::CustomerOperatedMirror,
        "Customer-mirrored extension catalog",
    );

    let residual = vec![
        residual_row(
            &summary_id,
            DependencyClass::HostedControlPlaneReachability,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToMirrorBackedReadOnly,
            ContinuityFallbackClass::MirrorSnapshotImport,
            "Customer-federated control plane behind mirror",
            "Mirror snapshot remains authoritative; live fetch is blocked.",
            Some("Mirror snapshot fresh within window"),
        ),
        residual_row(
            &summary_id,
            DependencyClass::PackageRegistry,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToMirrorBackedReadOnly,
            ContinuityFallbackClass::MirrorSnapshotImport,
            "Mirror-backed registry",
            "Registry resolves against mirror; live fetch is suppressed.",
            Some("Mirror snapshot fresh within window"),
        ),
    ];

    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::EnterpriseOnline,
        product_facing_label_class: ProductFacingLabelClass::HybridRemoteAttach,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineMirrorOnly,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Mirror-only routing engaged; live control-plane fetch suppressed."
                .to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: Some("Mirror within configured freshness window".to_owned()),
            freshness_floor_ref: Some("freshness.enterprise_online.mirror".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::MirrorOnly,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: vec![updates.row_id.clone(), extensions.row_id.clone()],
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedOfflineParityWhenMirrorOnly,
            ProhibitedImpliedClaimClass::ImpliedAirGappedWhenEgressAllowed,
            ProhibitedImpliedClaimClass::ImpliedAlwaysFreshWhenBoundedOrUnboundedStale,
        ],
        open_details_action: inspect_action(
            "action.summary.enterprise_online_mirrored.open_details",
            "route.deployment.details.enterprise_online_mirrored",
        ),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: Some(
            "card.deployment.enterprise_online_mirrored_baseline".to_owned(),
        ),
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/enterprise_failover_boundary_recheck.json"
                .to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };

    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::MirrorOnly,
            summary_label: "Control plane reached through mirror snapshot only; live fetch suppressed."
                .to_owned(),
            impaired_service_classes: vec![
                ControlPlaneServiceClass::RegistryService,
                ControlPlaneServiceClass::CatalogService,
            ],
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::PolicyService,
                ControlPlaneServiceClass::DocsPackService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities available; remote and managed reads resolve against mirror.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.enterprise_online_mirrored.continue_local",
        ),
        alternate_actions: vec![SafestNextAction {
            action_id: "action.strip.enterprise_online_mirrored.switch_mirror".to_owned(),
            action_class: SafestNextActionClass::SwitchMirror,
            label: SafestNextActionClass::SwitchMirror.label().to_owned(),
            scope_class: "scope_local_with_mirror_recovery".to_owned(),
            authority_class: "user_local_authority".to_owned(),
            consent_class: "explicit_consent_required_mirror_switch".to_owned(),
            side_effects: vec!["mirror_switch_attempt".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }],
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };

    let (page, _defects) = DeploymentProfilePage::compose(
        "page.deployment.enterprise_online_mirrored_baseline",
        profile_summary,
        plane_status_strip,
        residual,
        vec![updates, extensions],
    );
    page
}

fn air_gapped_page(lens: SurfaceLensClass) -> DeploymentProfilePage {
    // Air-gapped MUST NOT route through companion_surface; if the caller
    // asked for the companion lens, narrow back to the desktop lens so the
    // case stays conforming. The corpus's coverage summary still records
    // that air-gapped is not covered by the companion lens, which is the
    // intended invariant.
    let lens = if matches!(lens, SurfaceLensClass::CompanionHandoff) {
        SurfaceLensClass::Desktop
    } else {
        lens
    };
    let summary_id = "summary.deployment.air_gapped_mirror_only_baseline".to_owned();
    let strip_id = "strip.deployment.air_gapped_mirror_only_baseline".to_owned();

    let docs_pack = artifact_row(
        &summary_id,
        ArtifactClass::DocsPack,
        SignerStateClass::SignedOfflineTrustRoot,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::OfflineBundleDerivedMirror,
        "Offline-trust-root docs pack",
    );
    let policy_bundle = artifact_row(
        &summary_id,
        ArtifactClass::PolicyBundle,
        SignerStateClass::SignedOfflineTrustRoot,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::OfflineBundleDerivedMirror,
        "Offline-trust-root policy bundle",
    );
    let extensions = artifact_row(
        &summary_id,
        ArtifactClass::Extensions,
        SignerStateClass::SignedOfflineTrustRoot,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::OfflineBundleDerivedMirror,
        "Offline-bundle extension catalog",
    );

    let residual = vec![
        residual_row(
            &summary_id,
            DependencyClass::RemoteMirror,
            PostureClass::Mirrored,
            AbsenceImpactClass::BlockedPendingMirrorRefresh,
            ContinuityFallbackClass::MirrorSnapshotImport,
            "Offline bundle / signed mirror import",
            "All managed reads pause until the next mirror refresh.",
            Some("Within bundle freshness window"),
        ),
        residual_row(
            &summary_id,
            DependencyClass::AiProvider,
            PostureClass::Forbidden,
            AbsenceImpactClass::FailClosedForbiddenInProfile,
            ContinuityFallbackClass::FailClosedNoFallback,
            "Hosted AI broker",
            "Hosted AI is forbidden on this profile; attempts fail closed.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::BrowserHandoff,
            PostureClass::Forbidden,
            AbsenceImpactClass::FailClosedForbiddenInProfile,
            ContinuityFallbackClass::FailClosedNoFallback,
            "Browser companion handoff",
            "Browser companion handoff is forbidden on this profile.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::CompanionNotificationChannel,
            PostureClass::Forbidden,
            AbsenceImpactClass::FailClosedForbiddenInProfile,
            ContinuityFallbackClass::FailClosedNoFallback,
            "Companion notification channel",
            "Companion notification channel is forbidden on this profile.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::HostedControlPlaneReachability,
            PostureClass::Forbidden,
            AbsenceImpactClass::FailClosedForbiddenInProfile,
            ContinuityFallbackClass::FailClosedNoFallback,
            "Hosted control-plane reachability",
            "Hosted control-plane reachability is forbidden on this profile.",
            None,
        ),
    ];

    let artifact_refs = vec![
        docs_pack.row_id.clone(),
        policy_bundle.row_id.clone(),
        extensions.row_id.clone(),
    ];

    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::AirGapped,
        product_facing_label_class: ProductFacingLabelClass::AirGappedMirrorOnly,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::OfflineTrustRoot,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineAirGapped,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Offline bundle within freshness window; no public egress allowed."
                .to_owned(),
            last_control_plane_sync_at: None,
            cache_age_label: Some("Within bundle freshness window".to_owned()),
            freshness_floor_ref: Some("freshness.air_gapped.bounded".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: artifact_refs,
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedAirGappedWhenEgressAllowed,
            ProhibitedImpliedClaimClass::ImpliedAlwaysFreshWhenBoundedOrUnboundedStale,
        ],
        open_details_action: inspect_action(
            "action.summary.air_gapped_mirror_only.open_details",
            "route.deployment.details.air_gapped",
        ),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: Some(
            "card.deployment.air_gapped_mirror_only_baseline".to_owned(),
        ),
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/air_gapped_mirror_only_docs.json".to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };

    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
            summary_label: "Air-gapped install; no live control plane is in scope.".to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: Vec::new(),
            last_sync_at: None,
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities available on offline bundle.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.air_gapped_mirror_only.continue_local",
        ),
        alternate_actions: Vec::new(),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };

    let (page, _defects) = DeploymentProfilePage::compose(
        "page.deployment.air_gapped_mirror_only_baseline",
        profile_summary,
        plane_status_strip,
        residual,
        vec![docs_pack, policy_bundle, extensions],
    );
    page
}

fn managed_cloud_page(lens: SurfaceLensClass) -> DeploymentProfilePage {
    let summary_id = "summary.deployment.managed_cloud_baseline".to_owned();
    let strip_id = "strip.deployment.managed_cloud_baseline".to_owned();

    let residual = vec![
        residual_row(
            &summary_id,
            DependencyClass::AiProvider,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToCachedLastKnownGood,
            ContinuityFallbackClass::ReplayCachedSnapshot,
            "Vendor-managed AI broker",
            "AI surfaces narrow to cached last-known-good with explicit freshness label.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::BrowserHandoff,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToCachedLastKnownGood,
            ContinuityFallbackClass::ReplayCachedSnapshot,
            "Browser companion handoff (primary surface)",
            "Companion handoff narrows to cached read-only with freshness label.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::CompanionNotificationChannel,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToCachedLastKnownGood,
            ContinuityFallbackClass::ResumeAfterReconnect,
            "Companion notification channel",
            "Companion notifications pause until reconnect; cached posture remains.",
            None,
        ),
        residual_row(
            &summary_id,
            DependencyClass::HostedControlPlaneReachability,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
            ContinuityFallbackClass::ContinueLocalNoRestore,
            "Vendor-operated control plane",
            "Local-core capabilities continue; managed surfaces narrow under outage.",
            None,
        ),
    ];

    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::ManagedCloud,
        product_facing_label_class: ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::VendorRetentionWindowDefault,
        key_mode_class: KeyModeClass::VendorManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Vendor-operated control plane reachable.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: None,
            freshness_floor_ref: Some("freshness.managed_cloud.fresh".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Healthy,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedSelfHostedWhenManagedCloud,
            ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
            ProhibitedImpliedClaimClass::ImpliedNoResidualDependencyWhenRequiredPresent,
            ProhibitedImpliedClaimClass::ImpliedSovereignWhenVendorManaged,
        ],
        open_details_action: inspect_action(
            "action.summary.managed_cloud_baseline.open_details",
            "route.deployment.details.managed_cloud",
        ),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: Some(
            "card.deployment.managed_cloud_baseline".to_owned(),
        ),
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/managed_cloud_relay_disconnect.json".to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };

    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Healthy,
            summary_label: "Vendor-operated control plane reachable; all services healthy."
                .to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::RegistryService,
                ControlPlaneServiceClass::RelayService,
                ControlPlaneServiceClass::AiBrokerService,
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::PolicyService,
                ControlPlaneServiceClass::DocsPackService,
                ControlPlaneServiceClass::CatalogService,
                ControlPlaneServiceClass::TelemetrySinkService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities available; companion handoff is the primary surface.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.managed_cloud_baseline.continue_local",
        ),
        alternate_actions: Vec::new(),
        consumer_surfaces: lens_consumer_surfaces(lens),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };

    let (page, _defects) = DeploymentProfilePage::compose(
        "page.deployment.managed_cloud_baseline",
        profile_summary,
        plane_status_strip,
        residual,
        Vec::new(),
    );
    page
}

// -----------------------------------------------------------------------------
// Outage-drill page builders.
// -----------------------------------------------------------------------------

fn control_plane_unavailable_page() -> DeploymentProfilePage {
    let summary_id = "summary.drill.control_plane_unavailable".to_owned();
    let strip_id = "strip.drill.control_plane_unavailable".to_owned();
    let residual = vec![residual_row(
        &summary_id,
        DependencyClass::HostedControlPlaneReachability,
        PostureClass::Required,
        AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
        ContinuityFallbackClass::ContinueLocalNoRestore,
        "Vendor-operated control plane",
        "Vendor control plane unreachable; local-core remains available.",
        None,
    )];
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::ManagedCloud,
        product_facing_label_class: ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::VendorRetentionWindowDefault,
        key_mode_class: KeyModeClass::VendorManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineGracePreserved,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Last managed sync within grace window.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: Some("Within grace window".to_owned()),
            freshness_floor_ref: Some("freshness.managed_cloud.grace".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Unavailable,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedSelfHostedWhenManagedCloud,
            ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
        ],
        open_details_action: inspect_action(
            "action.summary.drill.control_plane_unavailable.open_details",
            "route.deployment.details.managed_cloud",
        ),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
            ConsumerSurfaceClass::CompanionSurface,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: Some("card.deployment.managed_cloud_baseline".to_owned()),
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/managed_cloud_relay_disconnect.json".to_owned(),
        ),
        linked_outage_notice_refs: vec![
            "fixtures/deployment/mode_change_cases/service_degradation_relay_unavailable.yaml"
                .to_owned(),
        ],
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Unavailable,
            summary_label: "Vendor control plane unreachable; sync, relay, and AI broker offline."
                .to_owned(),
            impaired_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::RelayService,
                ControlPlaneServiceClass::AiBrokerService,
            ],
            healthy_service_classes: Vec::new(),
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities remain available; remote attach is blocked.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.drill.control_plane_unavailable.continue_local",
        ),
        alternate_actions: vec![SafestNextAction {
            action_id: "action.strip.drill.control_plane_unavailable.reconnect_managed_session"
                .to_owned(),
            action_class: SafestNextActionClass::ReconnectManagedSession,
            label: SafestNextActionClass::ReconnectManagedSession.label().to_owned(),
            scope_class: "scope_local_with_managed_recovery".to_owned(),
            authority_class: "user_managed_authority".to_owned(),
            consent_class: "explicit_consent_required_managed_recovery".to_owned(),
            side_effects: vec!["session_reconnect_attempt".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }],
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, _) = DeploymentProfilePage::compose(
        "page.drill.control_plane_unavailable",
        profile_summary,
        plane_status_strip,
        residual,
        Vec::new(),
    );
    page
}

fn data_plane_blocked_page() -> DeploymentProfilePage {
    let summary_id = "summary.drill.data_plane_blocked_pending_reconnect".to_owned();
    let strip_id = "strip.drill.data_plane_blocked_pending_reconnect".to_owned();
    let residual = vec![residual_row(
        &summary_id,
        DependencyClass::RemoteAgent,
        PostureClass::Optional,
        AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
        ContinuityFallbackClass::ContinueLocalNoRestore,
        "Remote agent attach",
        "Remote attach pauses; local-core continues without becoming a thin client.",
        None,
    )];
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::EnterpriseOnline,
        product_facing_label_class: ProductFacingLabelClass::HybridRemoteAttach,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::VendorRetentionWindowWithCustomerPolicy,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Control plane healthy; remote attach pending reconnect.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: None,
            freshness_floor_ref: None,
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Healthy,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
        ],
        open_details_action: inspect_action(
            "action.summary.drill.data_plane_blocked.open_details",
            "route.deployment.details.enterprise_online",
        ),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/impairment_cases/remote_connector_loss_continue_local.json"
                .to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Healthy,
            summary_label: "Control plane healthy; remote agent transport degraded.".to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::PolicyService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: DataPlaneSummary {
            worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
            summary_label: "Remote attach is blocked pending reconnect; local-core remains available."
                .to_owned(),
            impaired_capability_classes: Vec::new(),
            available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline()
                .to_vec(),
        },
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.drill.data_plane_blocked.continue_local",
        ),
        alternate_actions: Vec::new(),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, _) = DeploymentProfilePage::compose(
        "page.drill.data_plane_blocked_pending_reconnect",
        profile_summary,
        plane_status_strip,
        residual,
        Vec::new(),
    );
    page
}

fn mirror_only_fallback_page() -> DeploymentProfilePage {
    // Use the enterprise_online_mirrored builder; the fallback is the
    // mirror-only baseline page itself but with a SwitchMirror alternate
    // emphasized.
    enterprise_online_mirrored_page(SurfaceLensClass::Desktop)
}

fn offline_cache_only_page() -> DeploymentProfilePage {
    let summary_id = "summary.drill.offline_cache_only".to_owned();
    let strip_id = "strip.drill.offline_cache_only".to_owned();
    let docs_pack = artifact_row(
        &summary_id,
        ArtifactClass::DocsPack,
        SignerStateClass::SignedOrgCaPinned,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorWithinExtendedWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::OfflineBundleDerivedMirror,
        "Cached docs pack",
    );
    let residual = vec![residual_row(
        &summary_id,
        DependencyClass::HostedControlPlaneReachability,
        PostureClass::Optional,
        AbsenceImpactClass::NarrowsToCachedLastKnownGood,
        ContinuityFallbackClass::ReplayCachedSnapshot,
        "Hosted control-plane reachability",
        "Cached last-known-good remains served under stale label; live fetch is paused.",
        Some("Within extended freshness window"),
    )];
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::EnterpriseOnline,
        product_facing_label_class: ProductFacingLabelClass::HybridRemoteAttach,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineGracePreserved,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Offline cache active; control plane unreachable inside grace window."
                .to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: Some("Within grace window".to_owned()),
            freshness_floor_ref: Some("freshness.enterprise_online.grace".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::StaleCache,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: vec![docs_pack.row_id.clone()],
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedAlwaysFreshWhenBoundedOrUnboundedStale,
        ],
        open_details_action: inspect_action(
            "action.summary.drill.offline_cache_only.open_details",
            "route.deployment.details.enterprise_online",
        ),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/impairment_cases/stale_policy_session_cached_local_safe.json"
                .to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::StaleCache,
            summary_label: "Control plane unreachable; cache serves last-known-good.".to_owned(),
            impaired_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::PolicyService,
                ControlPlaneServiceClass::DocsPackService,
            ],
            healthy_service_classes: Vec::new(),
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities available; managed reads served from cache under stale label.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.drill.offline_cache_only.continue_local",
        ),
        alternate_actions: vec![SafestNextAction {
            action_id: "action.strip.drill.offline_cache_only.retry_policy_sync".to_owned(),
            action_class: SafestNextActionClass::RetryPolicySync,
            label: SafestNextActionClass::RetryPolicySync.label().to_owned(),
            scope_class: "scope_local_with_managed_recovery".to_owned(),
            authority_class: "user_managed_authority".to_owned(),
            consent_class: "explicit_consent_required_managed_recovery".to_owned(),
            side_effects: vec!["policy_resync_attempt".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }],
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, _) = DeploymentProfilePage::compose(
        "page.drill.offline_cache_only",
        profile_summary,
        plane_status_strip,
        residual,
        vec![docs_pack],
    );
    page
}

fn sign_out_to_local_only_page() -> DeploymentProfilePage {
    let mut page = individual_local_baseline_page(EMITTED_AT);
    page.page_id = "page.drill.sign_out_to_local_only".to_owned();
    page.profile_summary.summary_id = "summary.drill.sign_out_to_local_only".to_owned();
    page.profile_summary.linked_outage_notice_refs = vec![
        "fixtures/deployment/mode_change_cases/sign_out_to_local_only.yaml".to_owned(),
    ];
    page.profile_summary.notes = Some(
        "After sign-out the install collapses to the local-only baseline; managed surfaces are not claimed."
            .to_owned(),
    );
    page.plane_status_strip.strip_id = "strip.drill.sign_out_to_local_only".to_owned();
    page.profile_summary.plane_status_strip_ref = page.plane_status_strip.strip_id.clone();
    page.plane_status_strip.linked_profile_summary_ref =
        Some(page.profile_summary.summary_id.clone());
    let (rebuilt, _) = DeploymentProfilePage::compose(
        page.page_id.clone(),
        page.profile_summary.clone(),
        page.plane_status_strip.clone(),
        page.residual_dependency_rows.clone(),
        page.mirror_offline_artifact_rows.clone(),
    );
    rebuilt
}

fn org_switch_boundary_recheck_page() -> DeploymentProfilePage {
    let summary_id = "summary.drill.org_switch_boundary_recheck".to_owned();
    let strip_id = "strip.drill.org_switch_boundary_recheck".to_owned();
    let residual = vec![residual_row(
        &summary_id,
        DependencyClass::SignIn,
        PostureClass::Required,
        AbsenceImpactClass::BlockedPendingBoundaryRecheck,
        ContinuityFallbackClass::ManualReconcileAfterBoundaryChange,
        "Org sign-in for new tenant",
        "Cross-tenant boundary recheck must complete before managed flows resume.",
        None,
    )];
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::EnterpriseOnline,
        product_facing_label_class: ProductFacingLabelClass::HybridRemoteAttach,
        tenant_org_scope_class: TenantOrgScopeClass::TenantBoundaryRecheckRequired,
        region_scope_class: RegionScopeClass::BoundaryRecheckRequired,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Org switched; boundary recheck pending.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: None,
            freshness_floor_ref: None,
            staleness_rationale: Some("Tenant and region must be rechecked.".to_owned()),
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::BoundaryRecheckRequired,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
        ],
        open_details_action: inspect_action(
            "action.summary.drill.org_switch_boundary_recheck.open_details",
            "route.deployment.details.enterprise_online",
        ),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: vec![
            "fixtures/deployment/mode_change_cases/org_switch_with_reauth.yaml".to_owned(),
        ],
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::BoundaryRecheckRequired,
            summary_label: "Org switch requires tenant and region boundary recheck.".to_owned(),
            impaired_service_classes: vec![ControlPlaneServiceClass::AuthIdentityService],
            healthy_service_classes: Vec::new(),
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities remain available while boundary recheck completes.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.drill.org_switch_boundary_recheck.continue_local",
        ),
        alternate_actions: vec![SafestNextAction {
            action_id: "action.strip.drill.org_switch_boundary_recheck.recheck_boundary".to_owned(),
            action_class: SafestNextActionClass::RecheckBoundary,
            label: SafestNextActionClass::RecheckBoundary.label().to_owned(),
            scope_class: "scope_local_with_managed_recovery".to_owned(),
            authority_class: "user_managed_authority".to_owned(),
            consent_class: "explicit_consent_required_boundary_recheck".to_owned(),
            side_effects: vec!["boundary_recheck_attempt".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }],
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, _) = DeploymentProfilePage::compose(
        "page.drill.org_switch_boundary_recheck",
        profile_summary,
        plane_status_strip,
        residual,
        Vec::new(),
    );
    page
}

fn seat_loss_continue_local_page() -> DeploymentProfilePage {
    let mut page = org_switch_boundary_recheck_page();
    page.page_id = "page.drill.seat_loss_continue_local".to_owned();
    page.profile_summary.summary_id = "summary.drill.seat_loss_continue_local".to_owned();
    page.plane_status_strip.strip_id = "strip.drill.seat_loss_continue_local".to_owned();
    page.profile_summary.plane_status_strip_ref = page.plane_status_strip.strip_id.clone();
    page.plane_status_strip.linked_profile_summary_ref =
        Some(page.profile_summary.summary_id.clone());
    page.profile_summary.linked_outage_notice_refs =
        vec!["fixtures/deployment/mode_change_cases/sign_out_to_local_only.yaml".to_owned()];
    page.plane_status_strip.control_plane_summary.summary_label =
        "Seat was revoked; managed surfaces narrow until seat is restored.".to_owned();
    page.profile_summary.notes = Some(
        "Seat loss does not strip local-core capability; cached workspace state is preserved."
            .to_owned(),
    );
    // Rebuild refs / summaries.
    for row in &mut page.residual_dependency_rows {
        row.profile_summary_ref = page.profile_summary.summary_id.clone();
        row.row_id = format!(
            "row.dep.{}.{}",
            page.profile_summary.summary_id,
            row.dependency_class.as_str()
        );
    }
    page.profile_summary.residual_dependency_row_refs = page
        .residual_dependency_rows
        .iter()
        .map(|r| r.row_id.clone())
        .collect();
    let (rebuilt, _) = DeploymentProfilePage::compose(
        page.page_id.clone(),
        page.profile_summary.clone(),
        page.plane_status_strip.clone(),
        page.residual_dependency_rows.clone(),
        page.mirror_offline_artifact_rows.clone(),
    );
    rebuilt
}

fn region_mismatch_boundary_recheck_page() -> DeploymentProfilePage {
    let mut page = org_switch_boundary_recheck_page();
    page.page_id = "page.drill.region_mismatch_boundary_recheck".to_owned();
    page.profile_summary.summary_id = "summary.drill.region_mismatch_boundary_recheck".to_owned();
    page.plane_status_strip.strip_id = "strip.drill.region_mismatch_boundary_recheck".to_owned();
    page.profile_summary.plane_status_strip_ref = page.plane_status_strip.strip_id.clone();
    page.plane_status_strip.linked_profile_summary_ref =
        Some(page.profile_summary.summary_id.clone());
    page.profile_summary.tenant_org_scope_class = TenantOrgScopeClass::CustomerTenant;
    page.profile_summary.region_scope_class = RegionScopeClass::BoundaryRecheckRequired;
    page.profile_summary.linked_outage_notice_refs = vec![
        "fixtures/deployment/mode_change_cases/profile_narrow_managed_to_local.yaml".to_owned(),
    ];
    page.plane_status_strip.control_plane_summary.summary_label =
        "Region mismatch detected; remote target region differs from customer-pinned region."
            .to_owned();
    page.profile_summary.notes = Some(
        "Region boundary recheck pauses cross-region writes until the customer-pinned region is reasserted."
            .to_owned(),
    );
    for row in &mut page.residual_dependency_rows {
        row.profile_summary_ref = page.profile_summary.summary_id.clone();
        row.row_id = format!(
            "row.dep.{}.{}",
            page.profile_summary.summary_id,
            row.dependency_class.as_str()
        );
    }
    page.profile_summary.residual_dependency_row_refs = page
        .residual_dependency_rows
        .iter()
        .map(|r| r.row_id.clone())
        .collect();
    let (rebuilt, _) = DeploymentProfilePage::compose(
        page.page_id.clone(),
        page.profile_summary.clone(),
        page.plane_status_strip.clone(),
        page.residual_dependency_rows.clone(),
        page.mirror_offline_artifact_rows.clone(),
    );
    rebuilt
}

fn stale_policy_cache_page() -> DeploymentProfilePage {
    let summary_id = "summary.drill.stale_policy_cache".to_owned();
    let strip_id = "strip.drill.stale_policy_cache".to_owned();
    let residual = vec![residual_row(
        &summary_id,
        DependencyClass::PolicyBundle,
        PostureClass::Required,
        AbsenceImpactClass::NarrowsToCachedLastKnownGood,
        ContinuityFallbackClass::ReplayCachedSnapshot,
        "Customer-operated policy bundle",
        "Policy evaluation uses cached last-known-good under a stale label.",
        Some("Last refreshed inside extended grace window"),
    )];
    let policy_bundle = artifact_row(
        &summary_id,
        ArtifactClass::PolicyBundle,
        SignerStateClass::SignedOrgCaPinned,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorWithinExtendedWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::CustomerOperatedMirror,
        "Customer-signed policy bundle (stale)",
    );
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::SelfHosted,
        product_facing_label_class: ProductFacingLabelClass::SelfHostedSovereign,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::CustomerManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineGracePreserved,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Policy and catalog cache served stale inside grace window.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: Some("Inside extended grace window".to_owned()),
            freshness_floor_ref: Some("freshness.self_hosted.grace".to_owned()),
            staleness_rationale: Some("Customer policy service offline.".to_owned()),
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::StaleCache,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: residual.iter().map(|r| r.row_id.clone()).collect(),
        mirror_offline_artifact_row_refs: vec![policy_bundle.row_id.clone()],
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedAlwaysFreshWhenBoundedOrUnboundedStale,
        ],
        open_details_action: inspect_action(
            "action.summary.drill.stale_policy_cache.open_details",
            "route.deployment.details.self_hosted",
        ),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: Some(
            "fixtures/deployment/continuity_cases/self_hosted_stale_policy_session.json".to_owned(),
        ),
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::StaleCache,
            summary_label: "Policy service stale; cache serves last-known-good.".to_owned(),
            impaired_service_classes: vec![ControlPlaneServiceClass::PolicyService],
            healthy_service_classes: vec![
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::DocsPackService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: local_core_data_plane(
            "Local-core capabilities remain available; policy evaluation uses stale cache.",
        ),
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.drill.stale_policy_cache.continue_local",
        ),
        alternate_actions: vec![SafestNextAction {
            action_id: "action.strip.drill.stale_policy_cache.retry_policy_sync".to_owned(),
            action_class: SafestNextActionClass::RetryPolicySync,
            label: SafestNextActionClass::RetryPolicySync.label().to_owned(),
            scope_class: "scope_local_with_managed_recovery".to_owned(),
            authority_class: "user_managed_authority".to_owned(),
            consent_class: "explicit_consent_required_managed_recovery".to_owned(),
            side_effects: vec!["policy_resync_attempt".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }],
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, _) = DeploymentProfilePage::compose(
        "page.drill.stale_policy_cache",
        profile_summary,
        plane_status_strip,
        residual,
        vec![policy_bundle],
    );
    page
}

fn stale_catalog_cache_page() -> DeploymentProfilePage {
    let mut page = stale_policy_cache_page();
    page.page_id = "page.drill.stale_catalog_cache".to_owned();
    page.profile_summary.summary_id = "summary.drill.stale_catalog_cache".to_owned();
    page.plane_status_strip.strip_id = "strip.drill.stale_catalog_cache".to_owned();
    page.profile_summary.plane_status_strip_ref = page.plane_status_strip.strip_id.clone();
    page.plane_status_strip.linked_profile_summary_ref =
        Some(page.profile_summary.summary_id.clone());
    page.plane_status_strip.control_plane_summary.impaired_service_classes =
        vec![ControlPlaneServiceClass::CatalogService];
    page.plane_status_strip.control_plane_summary.summary_label =
        "Catalog service stale; cache serves last-known-good.".to_owned();
    for row in &mut page.residual_dependency_rows {
        row.profile_summary_ref = page.profile_summary.summary_id.clone();
        row.row_id = format!(
            "row.dep.{}.{}",
            page.profile_summary.summary_id,
            row.dependency_class.as_str()
        );
        if row.dependency_class == DependencyClass::PolicyBundle {
            row.dependent_feature_label = "Customer-operated catalog feed".to_owned();
            row.unreachable_impact_label =
                "Catalog reads narrow to cached last-known-good under a stale label.".to_owned();
        }
    }
    page.profile_summary.residual_dependency_row_refs = page
        .residual_dependency_rows
        .iter()
        .map(|r| r.row_id.clone())
        .collect();
    for row in &mut page.mirror_offline_artifact_rows {
        row.profile_summary_ref = page.profile_summary.summary_id.clone();
        row.row_id = format!(
            "row.artifact.{}.{}",
            page.profile_summary.summary_id,
            row.artifact_class.as_str()
        );
    }
    page.profile_summary.mirror_offline_artifact_row_refs = page
        .mirror_offline_artifact_rows
        .iter()
        .map(|r| r.row_id.clone())
        .collect();
    let (rebuilt, _) = DeploymentProfilePage::compose(
        page.page_id.clone(),
        page.profile_summary.clone(),
        page.plane_status_strip.clone(),
        page.residual_dependency_rows.clone(),
        page.mirror_offline_artifact_rows.clone(),
    );
    rebuilt
}

// -----------------------------------------------------------------------------
// Residual dependency matrix (mirrors artifacts/governance/residual_dependencies.yaml).
// -----------------------------------------------------------------------------

fn dependency_matrix_row(
    dependency: DependencyClass,
    title: &str,
    individual_local: (PostureClass, AbsenceImpactClass, ContinuityFallbackClass),
    self_hosted: (PostureClass, AbsenceImpactClass, ContinuityFallbackClass),
    enterprise_online: (PostureClass, AbsenceImpactClass, ContinuityFallbackClass),
    air_gapped: (PostureClass, AbsenceImpactClass, ContinuityFallbackClass),
    managed_cloud: (PostureClass, AbsenceImpactClass, ContinuityFallbackClass),
) -> ResidualDependencyMatrixRow {
    use DeploymentProfileClass::*;
    let postures = [
        (IndividualLocal, individual_local),
        (SelfHosted, self_hosted),
        (EnterpriseOnline, enterprise_online),
        (AirGapped, air_gapped),
        (ManagedCloud, managed_cloud),
    ];
    let per_profile_posture = postures
        .iter()
        .map(|(p, (posture, _, _))| PerProfilePostureEntry {
            deployment_profile: *p,
            posture_class: *posture,
        })
        .collect();
    let per_profile_absence_impact = postures
        .iter()
        .map(|(p, (_, impact, _))| PerProfileAbsenceImpactEntry {
            deployment_profile: *p,
            absence_impact_class: *impact,
        })
        .collect();
    let per_profile_continuity_fallback = postures
        .iter()
        .map(|(p, (_, _, fallback))| PerProfileContinuityFallbackEntry {
            deployment_profile: *p,
            continuity_fallback_class: *fallback,
        })
        .collect();
    ResidualDependencyMatrixRow {
        dependency_class: dependency,
        title: title.to_owned(),
        per_profile_posture,
        per_profile_absence_impact,
        per_profile_continuity_fallback,
    }
}

fn seeded_residual_dependency_matrix() -> ResidualDependencyMatrix {
    use AbsenceImpactClass::*;
    use ContinuityFallbackClass as Fallback;
    use DependencyClass::*;
    use PostureClass as Posture;
    use Fallback::*;
    use Posture::*;

    let rows = vec![
        dependency_matrix_row(
            SignIn,
            "Sign-in / identity session",
            (Posture::NotApplicableStructural, NoImpactCapabilityNotClaimedForProfile, Fallback::NotApplicableStructural),
            (Required, BlockedPendingReconnect, ResumeAfterReconnect),
            (Required, BlockedPendingReconnect, ResumeAfterReconnect),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Required, BlockedPendingReconnect, ResumeAfterReconnect),
        ),
        dependency_matrix_row(
            PackageRegistry,
            "Package registry / extension catalog fetch",
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Mirrored, NarrowsToMirrorBackedReadOnly, MirrorSnapshotImport),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
        ),
        dependency_matrix_row(
            RemoteMirror,
            "Signed mirror / offline bundle",
            (Optional, NoImpactCapabilityNotClaimedForProfile, ContinueLocalNoRestore),
            (Optional, NoImpactCapabilityNotClaimedForProfile, ResumeAfterReconnect),
            (Optional, NoImpactCapabilityNotClaimedForProfile, ResumeAfterReconnect),
            (Required, BlockedPendingMirrorRefresh, MirrorSnapshotImport),
            (Optional, NoImpactCapabilityNotClaimedForProfile, ResumeAfterReconnect),
        ),
        dependency_matrix_row(
            RemoteAgent,
            "Remote agent / SSH-attached development host",
            (Posture::NotApplicableStructural, NoImpactCapabilityNotClaimedForProfile, Fallback::NotApplicableStructural),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Forbidden, FailClosedForbiddenInProfile, FailClosedNoFallback),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
        ),
        dependency_matrix_row(
            SymbolService,
            "Symbol / index service",
            (Posture::NotApplicableStructural, NoImpactCapabilityNotClaimedForProfile, Fallback::NotApplicableStructural),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Mirrored, NarrowsToMirrorBackedReadOnly, ReplayCachedSnapshot),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
        ),
        dependency_matrix_row(
            AiProvider,
            "Managed AI provider / broker",
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Optional, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Forbidden, FailClosedForbiddenInProfile, FailClosedNoFallback),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
        ),
        dependency_matrix_row(
            PolicyBundle,
            "Policy bundle / admin policy service",
            (Optional, NoImpactCapabilityNotClaimedForProfile, ContinueLocalNoRestore),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Mirrored, NarrowsToMirrorBackedReadOnly, MirrorSnapshotImport),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
        ),
        dependency_matrix_row(
            DocsPack,
            "Docs / help pack",
            (Cached, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Cached, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Cached, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Mirrored, NarrowsToMirrorBackedReadOnly, MirrorSnapshotImport),
            (Cached, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
        ),
        dependency_matrix_row(
            BrowserHandoff,
            "Browser companion / handoff channel",
            (Optional, NoImpactCapabilityNotClaimedForProfile, ContinueLocalNoRestore),
            (Optional, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Optional, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
            (Forbidden, FailClosedForbiddenInProfile, FailClosedNoFallback),
            (Required, NarrowsToCachedLastKnownGood, ReplayCachedSnapshot),
        ),
        dependency_matrix_row(
            CompanionNotificationChannel,
            "Companion notification channel",
            (Posture::NotApplicableStructural, NoImpactCapabilityNotClaimedForProfile, Fallback::NotApplicableStructural),
            (Optional, NarrowsToCachedLastKnownGood, ResumeAfterReconnect),
            (Optional, NarrowsToCachedLastKnownGood, ResumeAfterReconnect),
            (Forbidden, FailClosedForbiddenInProfile, FailClosedNoFallback),
            (Required, NarrowsToCachedLastKnownGood, ResumeAfterReconnect),
        ),
        dependency_matrix_row(
            HostedControlPlaneReachability,
            "Hosted control-plane reachability",
            (Posture::NotApplicableStructural, NoImpactCapabilityNotClaimedForProfile, Fallback::NotApplicableStructural),
            (Optional, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Required, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
            (Forbidden, FailClosedForbiddenInProfile, FailClosedNoFallback),
            (Required, NarrowsToLocalCoreCapabilities, ContinueLocalNoRestore),
        ),
    ];

    ResidualDependencyMatrix {
        record_kind: RESIDUAL_DEPENDENCY_MATRIX_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        source_ledger_ref: "artifacts/governance/residual_dependencies.yaml".to_owned(),
        overview: "Per-profile residual-dependency posture, projected from the governance ledger. \
                   Forbidden postures fail closed; required postures back vendor-bound classes; \
                   mirrored postures resolve against a signed mirror only."
            .to_owned(),
        rows,
    }
}

// -----------------------------------------------------------------------------
// Packet builder.
// -----------------------------------------------------------------------------

fn case(
    deployment_profile: DeploymentProfileClass,
    product_facing_label_class: ProductFacingLabelClass,
    surface_lens_class: SurfaceLensClass,
    case_id: &str,
    scenario_summary: &str,
    page: DeploymentProfilePage,
) -> DeploymentProfileCorpusCase {
    DeploymentProfileCorpusCase {
        record_kind: DEPLOYMENT_PROFILE_CORPUS_CASE_RECORD_KIND.to_owned(),
        case_id: case_id.to_owned(),
        deployment_profile,
        product_facing_label_class,
        surface_lens_class,
        scenario_summary: scenario_summary.to_owned(),
        fixture_path: format!("fixtures/deployment/m3/profile_truth/cases/{case_id}.json"),
        page,
    }
}

fn drill(
    drill_class: OutageDrillClass,
    deployment_profile: DeploymentProfileClass,
    drill_id: &str,
    scenario_summary: &str,
    continuity_assertion: &str,
    page: DeploymentProfilePage,
) -> DeploymentProfileOutageDrill {
    DeploymentProfileOutageDrill {
        record_kind: DEPLOYMENT_PROFILE_OUTAGE_DRILL_RECORD_KIND.to_owned(),
        drill_id: drill_id.to_owned(),
        drill_class,
        deployment_profile,
        scenario_summary: scenario_summary.to_owned(),
        continuity_assertion: continuity_assertion.to_owned(),
        local_safe_remains: page.summary.local_safe_remains,
        control_plane_impaired: page.summary.control_plane_impaired,
        data_plane_impaired: page.summary.data_plane_impaired,
        honesty_marker_present: page.summary.honesty_marker_present,
        fixture_path: format!(
            "fixtures/deployment/m3/control_plane_vs_data_plane/drills/{drill_id}.json"
        ),
        page,
    }
}

/// Build the canonical corpus packet. Every page passes `audit()` with an
/// empty defect set.
pub fn seeded_deployment_profile_corpus_packet() -> DeploymentProfileCorpusPacket {
    let corpus_cases = vec![
        case(
            DeploymentProfileClass::IndividualLocal,
            ProductFacingLabelClass::DesktopLocalFirst,
            SurfaceLensClass::Desktop,
            "individual_local_desktop",
            "Desktop-local baseline rendered on the desktop About/diagnostics/status-bar surfaces.",
            individual_local_page(SurfaceLensClass::Desktop),
        ),
        case(
            DeploymentProfileClass::IndividualLocal,
            ProductFacingLabelClass::DesktopLocalFirst,
            SurfaceLensClass::CliHeadless,
            "individual_local_cli_headless",
            "Desktop-local baseline rendered on the CLI/headless surface.",
            individual_local_page(SurfaceLensClass::CliHeadless),
        ),
        case(
            DeploymentProfileClass::IndividualLocal,
            ProductFacingLabelClass::DesktopLocalFirst,
            SurfaceLensClass::SupportExport,
            "individual_local_support_export",
            "Desktop-local baseline projected through the support-export lens.",
            individual_local_page(SurfaceLensClass::SupportExport),
        ),
        case(
            DeploymentProfileClass::SelfHosted,
            ProductFacingLabelClass::SelfHostedSovereign,
            SurfaceLensClass::Desktop,
            "self_hosted_sovereign_desktop",
            "Self-hosted / sovereign baseline rendered on desktop surfaces.",
            self_hosted_sovereign_page(SurfaceLensClass::Desktop),
        ),
        case(
            DeploymentProfileClass::SelfHosted,
            ProductFacingLabelClass::SelfHostedSovereign,
            SurfaceLensClass::CliHeadless,
            "self_hosted_sovereign_cli_headless",
            "Self-hosted / sovereign baseline rendered on the CLI/headless surface.",
            self_hosted_sovereign_page(SurfaceLensClass::CliHeadless),
        ),
        case(
            DeploymentProfileClass::SelfHosted,
            ProductFacingLabelClass::SelfHostedSovereign,
            SurfaceLensClass::SupportExport,
            "self_hosted_sovereign_support_export",
            "Self-hosted / sovereign baseline projected through the support-export lens.",
            self_hosted_sovereign_page(SurfaceLensClass::SupportExport),
        ),
        case(
            DeploymentProfileClass::EnterpriseOnline,
            ProductFacingLabelClass::HybridRemoteAttach,
            SurfaceLensClass::Desktop,
            "enterprise_online_hybrid_desktop",
            "Enterprise-online hybrid baseline rendered on desktop surfaces.",
            enterprise_online_hybrid_page(SurfaceLensClass::Desktop),
        ),
        case(
            DeploymentProfileClass::EnterpriseOnline,
            ProductFacingLabelClass::HybridRemoteAttach,
            SurfaceLensClass::CompanionHandoff,
            "enterprise_online_hybrid_companion_handoff",
            "Enterprise-online hybrid baseline rendered on the companion handoff surface.",
            enterprise_online_hybrid_page(SurfaceLensClass::CompanionHandoff),
        ),
        case(
            DeploymentProfileClass::EnterpriseOnline,
            ProductFacingLabelClass::HybridRemoteAttach,
            SurfaceLensClass::SupportExport,
            "enterprise_online_hybrid_support_export",
            "Enterprise-online hybrid baseline projected through the support-export lens.",
            enterprise_online_hybrid_page(SurfaceLensClass::SupportExport),
        ),
        case(
            DeploymentProfileClass::EnterpriseOnline,
            ProductFacingLabelClass::HybridRemoteAttach,
            SurfaceLensClass::Desktop,
            "enterprise_online_mirrored_desktop",
            "Enterprise-online mirror-only baseline rendered on desktop surfaces.",
            enterprise_online_mirrored_page(SurfaceLensClass::Desktop),
        ),
        case(
            DeploymentProfileClass::EnterpriseOnline,
            ProductFacingLabelClass::HybridRemoteAttach,
            SurfaceLensClass::SupportExport,
            "enterprise_online_mirrored_support_export",
            "Enterprise-online mirror-only baseline projected through the support-export lens.",
            enterprise_online_mirrored_page(SurfaceLensClass::SupportExport),
        ),
        case(
            DeploymentProfileClass::AirGapped,
            ProductFacingLabelClass::AirGappedMirrorOnly,
            SurfaceLensClass::Desktop,
            "air_gapped_mirror_only_desktop",
            "Air-gapped / mirror-only baseline rendered on desktop surfaces; companion lens is forbidden on this profile.",
            air_gapped_page(SurfaceLensClass::Desktop),
        ),
        case(
            DeploymentProfileClass::AirGapped,
            ProductFacingLabelClass::AirGappedMirrorOnly,
            SurfaceLensClass::CliHeadless,
            "air_gapped_mirror_only_cli_headless",
            "Air-gapped / mirror-only baseline rendered on the CLI/headless surface.",
            air_gapped_page(SurfaceLensClass::CliHeadless),
        ),
        case(
            DeploymentProfileClass::AirGapped,
            ProductFacingLabelClass::AirGappedMirrorOnly,
            SurfaceLensClass::SupportExport,
            "air_gapped_mirror_only_support_export",
            "Air-gapped / mirror-only baseline projected through the support-export lens.",
            air_gapped_page(SurfaceLensClass::SupportExport),
        ),
        case(
            DeploymentProfileClass::ManagedCloud,
            ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
            SurfaceLensClass::Desktop,
            "managed_cloud_desktop",
            "Managed-cloud baseline rendered on desktop surfaces.",
            managed_cloud_page(SurfaceLensClass::Desktop),
        ),
        case(
            DeploymentProfileClass::ManagedCloud,
            ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
            SurfaceLensClass::CompanionHandoff,
            "managed_cloud_companion_handoff",
            "Managed-cloud baseline rendered on the companion handoff (primary) surface.",
            managed_cloud_page(SurfaceLensClass::CompanionHandoff),
        ),
        case(
            DeploymentProfileClass::ManagedCloud,
            ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
            SurfaceLensClass::SupportExport,
            "managed_cloud_support_export",
            "Managed-cloud baseline projected through the support-export lens.",
            managed_cloud_page(SurfaceLensClass::SupportExport),
        ),
    ];

    let outage_drills = vec![
        drill(
            OutageDrillClass::ControlPlaneUnavailable,
            DeploymentProfileClass::ManagedCloud,
            "control_plane_unavailable",
            "Vendor control plane unreachable; relay, sync, and AI broker offline. Local data plane stays local-safe.",
            "Control-plane impairment never collapses into generic 'service degraded' copy while local-safe data-plane work remains; the safest next action stays continue_local.",
            control_plane_unavailable_page(),
        ),
        drill(
            OutageDrillClass::DataPlaneBlockedPendingReconnect,
            DeploymentProfileClass::EnterpriseOnline,
            "data_plane_blocked_pending_reconnect",
            "Remote attach transport drops while local-core capabilities remain available.",
            "Remote disconnect does not turn the desktop into a thin client; local-core capabilities continue independent of remote-agent reconnect.",
            data_plane_blocked_page(),
        ),
        drill(
            OutageDrillClass::MirrorOnlyFallback,
            DeploymentProfileClass::EnterpriseOnline,
            "mirror_only_fallback",
            "Live control-plane fetch suppressed; mirror-only routing engaged with offline-parity guardrail.",
            "Mirror-only fallback names the offline-parity guardrail and emits at least one mirror/offline artifact row; live fetch claim is suppressed.",
            mirror_only_fallback_page(),
        ),
        drill(
            OutageDrillClass::OfflineCacheOnly,
            DeploymentProfileClass::EnterpriseOnline,
            "offline_cache_only",
            "Offline cache active while control plane is unreachable inside the grace window.",
            "Cached last-known-good is served under an explicit stale label; control-plane recovery is offered as an alternate action.",
            offline_cache_only_page(),
        ),
        drill(
            OutageDrillClass::SignOutToLocalOnly,
            DeploymentProfileClass::IndividualLocal,
            "sign_out_to_local_only",
            "After sign-out the install narrows to the individual-local baseline.",
            "Sign-out preserves local-core continuity; managed surfaces are not silently retained as claimed.",
            sign_out_to_local_only_page(),
        ),
        drill(
            OutageDrillClass::OrgSwitchBoundaryRecheck,
            DeploymentProfileClass::EnterpriseOnline,
            "org_switch_boundary_recheck",
            "Org switch requires a tenant and region boundary recheck before managed flows resume.",
            "Org-switch keeps local-core continuity available and surfaces RecheckBoundary as the explicit recovery action; managed surfaces are not silently inherited.",
            org_switch_boundary_recheck_page(),
        ),
        drill(
            OutageDrillClass::SeatLossContinueLocal,
            DeploymentProfileClass::EnterpriseOnline,
            "seat_loss_continue_local",
            "Seat is revoked; managed surfaces narrow while local-core continuity is preserved.",
            "Seat loss does not strip local-core capability or evict cached workspace state; ContinueLocal remains the safest next action.",
            seat_loss_continue_local_page(),
        ),
        drill(
            OutageDrillClass::RegionMismatchBoundaryRecheck,
            DeploymentProfileClass::EnterpriseOnline,
            "region_mismatch_boundary_recheck",
            "Remote target region differs from the customer-pinned region; boundary recheck is required.",
            "Region mismatch pauses cross-region writes and offers an explicit RecheckBoundary path; local-core continuity is preserved.",
            region_mismatch_boundary_recheck_page(),
        ),
        drill(
            OutageDrillClass::StalePolicyCache,
            DeploymentProfileClass::SelfHosted,
            "stale_policy_cache",
            "Policy service offline; cache serves last-known-good inside the grace window.",
            "Stale policy cache is served under an explicit stale label; Retry policy sync is offered as an alternate action without blocking local-core work.",
            stale_policy_cache_page(),
        ),
        drill(
            OutageDrillClass::StaleCatalogCache,
            DeploymentProfileClass::SelfHosted,
            "stale_catalog_cache",
            "Catalog service offline; cache serves last-known-good inside the grace window.",
            "Stale catalog cache is served under an explicit stale label; local-core continuity continues without depending on the catalog.",
            stale_catalog_cache_page(),
        ),
    ];

    let residual_dependency_matrix = seeded_residual_dependency_matrix();
    let coverage_summary = DeploymentProfileCoverageSummary::compute(
        &corpus_cases,
        &outage_drills,
        &residual_dependency_matrix,
    );

    DeploymentProfileCorpusPacket {
        record_kind: DEPLOYMENT_PROFILE_CORPUS_PACKET_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        notice: DEPLOYMENT_PROFILE_CORPUS_NOTICE.to_owned(),
        shared_contract_ref: DEPLOYMENT_PROFILE_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        qualification_doc_ref: DEPLOYMENT_PROFILE_CORPUS_QUALIFICATION_REF.to_owned(),
        corpus_cases,
        outage_drills,
        residual_dependency_matrix,
        coverage_summary,
    }
}

impl DeploymentProfileCoverageSummary {
    fn compute(
        cases: &[DeploymentProfileCorpusCase],
        drills: &[DeploymentProfileOutageDrill],
        matrix: &ResidualDependencyMatrix,
    ) -> Self {
        let mut profiles = BTreeSet::new();
        let mut lenses = BTreeSet::new();
        let mut drill_classes = BTreeSet::new();
        let mut surfaces: BTreeSet<ConsumerSurfaceClass> = BTreeSet::new();
        let mut all_cases_passed = true;
        let mut all_drills_passed = true;

        for c in cases {
            profiles.insert(c.deployment_profile.as_str().to_owned());
            lenses.insert(c.surface_lens_class);
            for s in consumer_surfaces_present(&c.page) {
                surfaces.insert(s);
            }
            if !c.page.audit().is_empty() {
                all_cases_passed = false;
            }
        }
        for d in drills {
            drill_classes.insert(d.drill_class);
            profiles.insert(d.deployment_profile.as_str().to_owned());
            for s in consumer_surfaces_present(&d.page) {
                surfaces.insert(s);
            }
            if !d.page.audit().is_empty() {
                all_drills_passed = false;
            }
        }

        // Deterministic ordered output: re-derive from canonical orderings.
        let deployment_profiles_present = canonical_deployment_profiles()
            .into_iter()
            .filter(|p| profiles.contains(p.as_str()))
            .collect::<Vec<_>>();
        let surface_lenses_present = lenses.into_iter().collect::<Vec<_>>();
        let outage_drill_classes_present = drill_classes.into_iter().collect::<Vec<_>>();
        let consumer_surfaces_present_vec = surfaces.into_iter().collect::<Vec<_>>();
        let dependency_classes_present = matrix.rows.iter().map(|r| r.dependency_class).collect();

        Self {
            case_count: cases.len(),
            drill_count: drills.len(),
            residual_dependency_row_count: matrix.rows.len(),
            deployment_profiles_present,
            surface_lenses_present,
            outage_drill_classes_present,
            dependency_classes_present,
            consumer_surfaces_present: consumer_surfaces_present_vec,
            all_cases_passed_audit: all_cases_passed,
            all_drills_passed_audit: all_drills_passed,
        }
    }
}

fn canonical_deployment_profiles() -> [DeploymentProfileClass; 5] {
    [
        DeploymentProfileClass::IndividualLocal,
        DeploymentProfileClass::SelfHosted,
        DeploymentProfileClass::EnterpriseOnline,
        DeploymentProfileClass::AirGapped,
        DeploymentProfileClass::ManagedCloud,
    ]
}

// -----------------------------------------------------------------------------
// Renderers.
// -----------------------------------------------------------------------------

/// Render the residual-dependency matrix as deterministic pretty JSON.
pub fn render_residual_dependency_matrix_json(packet: &DeploymentProfileCorpusPacket) -> String {
    let mut json = serde_json::to_string_pretty(&packet.residual_dependency_matrix)
        .expect("serializing the residual-dependency matrix must not fail");
    json.push('\n');
    json
}

/// Render the deployment-profile conformance report as deterministic
/// markdown bound to the packet.
pub fn render_deployment_profile_conformance_report_markdown(
    packet: &DeploymentProfileCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Deployment-profile conformance report\n\n");
    out.push_str(
        "This report is the M3 beta-exit conformance excerpt for the deployment-profile \
         continuity corpus. It binds the matrix of marketed deployment rows (individual-local, \
         self-hosted / sovereign, enterprise-online hybrid, enterprise-online mirror-only, \
         air-gapped, managed-cloud) and the control-plane vs. data-plane outage drills to the \
         shell's `DeploymentProfilePage::audit()` invariant. Every case below resolves through \
         one inspectable `DeploymentProfilePage` whose audit MUST return an empty defect set.\n\n",
    );
    out.push_str("Bound contract: [`");
    out.push_str(&packet.shared_contract_ref);
    out.push_str("`](../../../");
    out.push_str(&packet.shared_contract_ref);
    out.push_str(").\n\n");
    out.push_str("Bound qualification doc: [`");
    out.push_str(&packet.qualification_doc_ref);
    out.push_str("`](../../../");
    out.push_str(&packet.qualification_doc_ref);
    out.push_str(").\n\n");

    out.push_str("## Coverage summary\n\n");
    out.push_str(&format!(
        "- Marketed-row cases: **{}**\n",
        packet.coverage_summary.case_count
    ));
    out.push_str(&format!(
        "- Outage drills: **{}**\n",
        packet.coverage_summary.drill_count
    ));
    out.push_str(&format!(
        "- Residual-dependency rows: **{}**\n",
        packet.coverage_summary.residual_dependency_row_count
    ));
    out.push_str(&format!(
        "- All cases pass `audit()`: **{}**\n",
        packet.coverage_summary.all_cases_passed_audit
    ));
    out.push_str(&format!(
        "- All drills pass `audit()`: **{}**\n",
        packet.coverage_summary.all_drills_passed_audit
    ));
    out.push_str("- Profiles present: ");
    out.push_str(
        &packet
            .coverage_summary
            .deployment_profiles_present
            .iter()
            .map(|p| format!("`{}`", p.as_str()))
            .collect::<Vec<_>>()
            .join(", "),
    );
    out.push_str("\n- Surface lenses present: ");
    out.push_str(
        &packet
            .coverage_summary
            .surface_lenses_present
            .iter()
            .map(|l| format!("`{}`", l.as_str()))
            .collect::<Vec<_>>()
            .join(", "),
    );
    out.push_str("\n- Outage drill classes present: ");
    out.push_str(
        &packet
            .coverage_summary
            .outage_drill_classes_present
            .iter()
            .map(|d| format!("`{}`", d.as_str()))
            .collect::<Vec<_>>()
            .join(", "),
    );
    out.push_str("\n\n");

    out.push_str("## Marketed-row matrix\n\n");
    out.push_str("| Case id | Profile | Product label | Surface lens | Control plane | Data plane | Safest next action |\n");
    out.push_str("|---|---|---|---|---|---|---|\n");
    for c in &packet.corpus_cases {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            c.case_id,
            c.deployment_profile.as_str(),
            c.product_facing_label_class.as_str(),
            c.surface_lens_class.as_str(),
            c.page
                .plane_status_strip
                .control_plane_summary
                .worst_state_class
                .as_str(),
            c.page
                .plane_status_strip
                .data_plane_summary
                .worst_state_class
                .as_str(),
            c.page.plane_status_strip.safest_next_action.action_class.as_str(),
        ));
    }
    out.push('\n');

    out.push_str("## Outage drills\n\n");
    out.push_str("| Drill id | Class | Profile | Control plane | Data plane | Safest next action | Local-safe remains |\n");
    out.push_str("|---|---|---|---|---|---|---|\n");
    for d in &packet.outage_drills {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
            d.drill_id,
            d.drill_class.as_str(),
            d.deployment_profile.as_str(),
            d.page
                .plane_status_strip
                .control_plane_summary
                .worst_state_class
                .as_str(),
            d.page
                .plane_status_strip
                .data_plane_summary
                .worst_state_class
                .as_str(),
            d.page.plane_status_strip.safest_next_action.action_class.as_str(),
            d.local_safe_remains,
        ));
    }
    out.push('\n');

    out.push_str("## Continuity assertions\n\n");
    for d in &packet.outage_drills {
        out.push_str(&format!("- `{}`: {}\n", d.drill_id, d.continuity_assertion));
    }
    out.push('\n');

    out.push_str("## Residual-dependency matrix\n\n");
    out.push_str("Per-profile posture, projected from `");
    out.push_str(&packet.residual_dependency_matrix.source_ledger_ref);
    out.push_str("`. The full JSON projection lives at `artifacts/release/m3/residual_dependency_matrix.json`.\n\n");
    out.push_str("| Dependency | Individual local | Self-hosted | Enterprise online | Air-gapped | Managed cloud |\n");
    out.push_str("|---|---|---|---|---|---|\n");
    for row in &packet.residual_dependency_matrix.rows {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            row.dependency_class.as_str(),
            posture_for(row, DeploymentProfileClass::IndividualLocal),
            posture_for(row, DeploymentProfileClass::SelfHosted),
            posture_for(row, DeploymentProfileClass::EnterpriseOnline),
            posture_for(row, DeploymentProfileClass::AirGapped),
            posture_for(row, DeploymentProfileClass::ManagedCloud),
        ));
    }
    out.push('\n');

    out.push_str("## Verification\n\n");
    out.push_str("```bash\ncargo test -p aureline-shell --test deployment_profile_corpus_fixtures\n```\n\n");
    out.push_str(
        "The test loads every fixture under `fixtures/deployment/m3/profile_truth/` and \
         `fixtures/deployment/m3/control_plane_vs_data_plane/`, deserializes each one through \
         the shared `DeploymentProfilePage` shape, and asserts that `DeploymentProfilePage::audit()` \
         returns an empty defect set. It also asserts the fixture corpus matches the seeded packet \
         and that the rendered conformance report and residual-dependency matrix match this packet \
         byte-for-byte.\n",
    );
    out
}

fn posture_for(row: &ResidualDependencyMatrixRow, profile: DeploymentProfileClass) -> &'static str {
    row.per_profile_posture
        .iter()
        .find(|entry| entry.deployment_profile == profile)
        .map(|entry| entry.posture_class.as_str())
        .unwrap_or("unknown")
}

// -----------------------------------------------------------------------------
// Validation.
// -----------------------------------------------------------------------------

/// Validate the packet: every page MUST pass `audit()`, every fixture id is
/// unique, and the coverage summary MUST be consistent with the case and
/// drill lists.
pub fn validate_deployment_profile_corpus_packet(
    packet: &DeploymentProfileCorpusPacket,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut seen_case_ids = BTreeSet::new();
    let mut seen_drill_ids = BTreeSet::new();

    for c in &packet.corpus_cases {
        if !seen_case_ids.insert(c.case_id.clone()) {
            errors.push(format!("duplicate case id: {}", c.case_id));
        }
        let defects: Vec<DeploymentProfileDefect> = c.page.audit();
        if !defects.is_empty() {
            errors.push(format!(
                "case {} failed audit: {} defect(s) ({:?})",
                c.case_id,
                defects.len(),
                defects,
            ));
        }
        if c.page.profile_summary.deployment_profile != c.deployment_profile {
            errors.push(format!(
                "case {} profile mismatch: case says {}, page says {}",
                c.case_id,
                c.deployment_profile.as_str(),
                c.page.profile_summary.deployment_profile.as_str(),
            ));
        }
    }
    for d in &packet.outage_drills {
        if !seen_drill_ids.insert(d.drill_id.clone()) {
            errors.push(format!("duplicate drill id: {}", d.drill_id));
        }
        let defects: Vec<DeploymentProfileDefect> = d.page.audit();
        if !defects.is_empty() {
            errors.push(format!(
                "drill {} failed audit: {} defect(s) ({:?})",
                d.drill_id,
                defects.len(),
                defects,
            ));
        }
    }

    if packet.coverage_summary.case_count != packet.corpus_cases.len() {
        errors.push(format!(
            "coverage_summary.case_count {} disagrees with corpus_cases length {}",
            packet.coverage_summary.case_count,
            packet.corpus_cases.len()
        ));
    }
    if packet.coverage_summary.drill_count != packet.outage_drills.len() {
        errors.push(format!(
            "coverage_summary.drill_count {} disagrees with outage_drills length {}",
            packet.coverage_summary.drill_count,
            packet.outage_drills.len()
        ));
    }
    if packet.coverage_summary.residual_dependency_row_count
        != packet.residual_dependency_matrix.rows.len()
    {
        errors.push(format!(
            "coverage_summary.residual_dependency_row_count {} disagrees with matrix rows length {}",
            packet.coverage_summary.residual_dependency_row_count,
            packet.residual_dependency_matrix.rows.len()
        ));
    }
    if !packet.coverage_summary.all_cases_passed_audit {
        errors.push("coverage_summary.all_cases_passed_audit is false".to_owned());
    }
    if !packet.coverage_summary.all_drills_passed_audit {
        errors.push("coverage_summary.all_drills_passed_audit is false".to_owned());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_deployment_profile_corpus_packet();
        validate_deployment_profile_corpus_packet(&packet)
            .expect("seeded corpus packet must validate");
    }

    #[test]
    fn seeded_packet_is_deterministic() {
        let a = seeded_deployment_profile_corpus_packet();
        let b = seeded_deployment_profile_corpus_packet();
        assert_eq!(a, b);
    }

    #[test]
    fn coverage_summary_lists_every_profile_and_every_drill_class() {
        let packet = seeded_deployment_profile_corpus_packet();
        for profile in canonical_deployment_profiles() {
            assert!(
                packet
                    .coverage_summary
                    .deployment_profiles_present
                    .contains(&profile),
                "missing profile {}",
                profile.as_str()
            );
        }
        for drill_class in OutageDrillClass::all() {
            assert!(
                packet
                    .coverage_summary
                    .outage_drill_classes_present
                    .contains(&drill_class),
                "missing drill class {}",
                drill_class.as_str()
            );
        }
    }

    #[test]
    fn residual_dependency_matrix_lists_every_dependency_class() {
        let packet = seeded_deployment_profile_corpus_packet();
        let present: Vec<&'static str> = packet
            .residual_dependency_matrix
            .rows
            .iter()
            .map(|r| r.dependency_class.as_str())
            .collect();
        for dep in [
            DependencyClass::SignIn,
            DependencyClass::PackageRegistry,
            DependencyClass::RemoteMirror,
            DependencyClass::RemoteAgent,
            DependencyClass::SymbolService,
            DependencyClass::AiProvider,
            DependencyClass::PolicyBundle,
            DependencyClass::DocsPack,
            DependencyClass::BrowserHandoff,
            DependencyClass::CompanionNotificationChannel,
            DependencyClass::HostedControlPlaneReachability,
        ] {
            assert!(
                present.contains(&dep.as_str()),
                "missing dependency class {}",
                dep.as_str()
            );
        }
    }

    #[test]
    fn render_residual_dependency_matrix_json_is_deterministic() {
        let packet = seeded_deployment_profile_corpus_packet();
        let a = render_residual_dependency_matrix_json(&packet);
        let b = render_residual_dependency_matrix_json(&packet);
        assert_eq!(a, b);
        assert!(a.starts_with("{\n"));
        assert!(a.contains("\"record_kind\": \"residual_dependency_matrix_record\""));
    }

    #[test]
    fn render_conformance_report_is_deterministic_and_self_referential() {
        let packet = seeded_deployment_profile_corpus_packet();
        let a = render_deployment_profile_conformance_report_markdown(&packet);
        let b = render_deployment_profile_conformance_report_markdown(&packet);
        assert_eq!(a, b);
        assert!(a.contains("Deployment-profile conformance report"));
        assert!(a.contains("docs/ops/m3/deployment_profile_and_continuity_beta.md"));
        assert!(a.contains("docs/ops/m3/deployment_profile_claim_qualification.md"));
    }

    #[test]
    fn every_required_residual_row_with_vendor_bound_class_flags_vendor_or_public_dependence() {
        let packet = seeded_deployment_profile_corpus_packet();
        for c in &packet.corpus_cases {
            for row in &c.page.residual_dependency_rows {
                if row.posture_class == PostureClass::Required
                    && row.dependency_class.is_vendor_bound_when_required()
                {
                    assert!(
                        row.vendor_or_public_dependence,
                        "case {}: required vendor-bound dependency {} missing vendor flag",
                        c.case_id,
                        row.dependency_class.as_str()
                    );
                }
            }
        }
    }

    #[test]
    fn air_gapped_cases_never_route_through_companion_surface() {
        let packet = seeded_deployment_profile_corpus_packet();
        for c in &packet.corpus_cases {
            if c.deployment_profile == DeploymentProfileClass::AirGapped {
                assert!(
                    !c.page
                        .profile_summary
                        .consumer_surfaces
                        .contains(&ConsumerSurfaceClass::CompanionSurface),
                    "case {} air-gapped row routed through companion_surface",
                    c.case_id
                );
            }
        }
    }
}
