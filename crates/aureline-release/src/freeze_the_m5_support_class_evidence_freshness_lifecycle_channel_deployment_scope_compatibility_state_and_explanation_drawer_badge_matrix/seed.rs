//! Canonical seed builders for the frozen M5 support-class, evidence-freshness,
//! lifecycle, channel, deployment-scope, compatibility-state, and
//! explanation-drawer badge matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical badge-family matrix.
pub const M5_BADGE_FAMILY_MATRIX_PACKET_ID: &str = "m5-badge-family:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every badge must be able to show.
fn mandatory_labels() -> Vec<M5BadgeRequiredLabel> {
    M5BadgeRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a badge carries.
fn labels_with(extra: &[M5BadgeRequiredLabel]) -> Vec<M5BadgeRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every badge filled in and every
/// family-specific value vocabulary left empty for the caller to populate.
fn base_row(
    badge_family: M5BadgeFamily,
    qualification: M5BadgeQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
) -> M5BadgeFamilyRow {
    M5BadgeFamilyRow {
        badge_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BadgeSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        required_labels: labels_with(&[
            M5BadgeRequiredLabel::ExplanationDrawer,
            M5BadgeRequiredLabel::EvidenceSource,
            M5BadgeRequiredLabel::FilterKey,
        ]),
        explanation_fields: M5BadgeExplanationField::ALL.to_vec(),
        support_class_values: vec![],
        evidence_freshness_values: vec![],
        lifecycle_values: vec![],
        channel_values: vec![],
        deployment_scope_values: vec![],
        compatibility_state_values: vec![],
        accessibility_routes: M5BadgeAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5BadgeConsumerSurface::MarketplaceUi,
            M5BadgeConsumerSurface::SupportExport,
            M5BadgeConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5BadgeDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BADGE_FAMILY_SCHEMA_REF,
            M5_BADGE_FAMILY_BADGE_VOCABULARY_REF,
        ]),
        collapses_multiple_axes_into_one_pill: false,
        implies_freshness_from_support_class: false,
        implies_lifecycle_from_deployment_scope: false,
        drops_badge_meaning_in_export: false,
    }
}

fn badge_rows() -> Vec<M5BadgeFamilyRow> {
    use M5BadgeConsumerSurface as C;
    use M5BadgeDowngradeTrigger as D;
    use M5BadgeFamily as F;
    use M5BadgeQualificationClass as Q;
    use M5ChannelBadge as CH;
    use M5CompatibilityStateBadge as CO;
    use M5DeploymentScopeBadge as DS;
    use M5EvidenceFreshnessBadge as EF;
    use M5LifecycleBadge as LC;
    use M5SupportClassBadge as SU;

    let mut rows = Vec::new();

    // 1. Support-class badge.
    let mut row = base_row(
        F::SupportClass,
        Q::Stable,
        "Support-class badge owner",
        "One support-class badge naming how supported a thing is — certified, fully supported, community supported, best effort, deprecated, or unsupported — so a support posture is always explicit and never implies anything about evidence freshness",
        "evidence:m5-support-class-badge-parity:001",
    );
    row.support_class_values = SU::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::HelpAbout,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SupportClassValueUnstated,
        D::AxisMergedIntoAnother,
        D::FreshnessImpliedFromSupportClass,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Evidence-freshness badge.
    let mut row = base_row(
        F::EvidenceFreshness,
        Q::Stable,
        "Evidence-freshness badge owner",
        "One evidence-freshness badge naming how fresh the proof behind a claim is — fresh, recent, aging, stale, expired, or unverified — so stale or unverified evidence is never presented as fresh and freshness stays independent of support class",
        "evidence:m5-evidence-freshness-badge-parity:001",
    );
    row.evidence_freshness_values = EF::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::DiagnosticsSurface,
        C::EvaluationPack,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EvidenceFreshnessHidden,
        D::AxisMergedIntoAnother,
        D::ExportLostBadgeMeaning,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Lifecycle badge.
    let mut row = base_row(
        F::Lifecycle,
        Q::Stable,
        "Lifecycle badge owner",
        "One lifecycle badge naming the lifecycle stage of a thing — stable, beta, preview, experimental, maintenance, or end-of-life — so the stage is always explicit and never stands in for a channel or a support class",
        "evidence:m5-lifecycle-badge-parity:001",
    );
    row.lifecycle_values = LC::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::OnboardingFlow,
        C::DocsPortal,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LifecycleValueUnstated,
        D::AxisMergedIntoAnother,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Channel badge.
    let mut row = base_row(
        F::Channel,
        Q::Stable,
        "Channel badge owner",
        "One channel badge naming which release channel a thing rides — stable, beta, nightly, edge, LTS, or custom — so the channel is always explicit and never implies a support class",
        "evidence:m5-channel-badge-parity:001",
    );
    row.channel_values = CH::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::SettingsUi,
        C::DiagnosticsSurface,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ChannelValueUnstated,
        D::AxisMergedIntoAnother,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Deployment-scope badge.
    let mut row = base_row(
        F::DeploymentScope,
        Q::Stable,
        "Deployment-scope badge owner",
        "One deployment-scope badge naming where a thing runs — desktop-only, local OSS, self-hosted, managed, air-gapped, or mirror-offline — so the scope is always explicit and never implies an experimental or lower lifecycle stage",
        "evidence:m5-deployment-scope-badge-parity:001",
    );
    row.deployment_scope_values = DS::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::SettingsUi,
        C::HelpAbout,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DeploymentScopeUnstated,
        D::AxisMergedIntoAnother,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Compatibility-state badge.
    let mut row = base_row(
        F::CompatibilityState,
        Q::Stable,
        "Compatibility-state badge owner",
        "One compatibility-state badge naming how compatible a thing is with the host — compatible, minor skew, major skew, incompatible, migration required, or unknown — so skew and required migrations are never hidden",
        "evidence:m5-compatibility-state-badge-parity:001",
    );
    row.compatibility_state_values = CO::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::DiagnosticsSurface,
        C::EvaluationPack,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CompatibilityStateUnstated,
        D::AxisMergedIntoAnother,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn axis_separation_rules() -> Vec<String> {
    M5BadgeAxisSeparationRule::ALL
        .iter()
        .map(|rule| rule.as_str().to_owned())
        .collect()
}

fn governance_review() -> M5BadgeGovernanceReview {
    M5BadgeGovernanceReview {
        each_family_shows_its_own_value: true,
        every_badge_opens_explanation_drawer: true,
        support_class_never_implies_freshness: true,
        deployment_scope_never_implies_lifecycle: true,
        no_badge_collapses_two_axes: true,
        every_badge_is_separately_filterable: true,
        exported_evidence_keeps_badge_meaning: true,
        no_badge_invents_second_grammar: true,
        every_badge_declares_deployment_lines: true,
        every_badge_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5BadgeConsumerProjection {
    M5BadgeConsumerProjection {
        marketplace_and_help_surfaces_consume_matrix: true,
        settings_and_onboarding_surfaces_consume_matrix: true,
        diagnostics_and_runtime_surfaces_consume_matrix: true,
        filters_read_single_source_per_axis: true,
        support_export_reads_single_source: true,
        docs_help_read_single_source: true,
    }
}

fn proof_freshness() -> M5BadgeProofFreshness {
    M5BadgeProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BadgeReleasePosture {
    M5BadgeReleasePosture {
        release_packet_ref: "artifacts/release/m5-badge-family-proof/support_export.json"
            .to_owned(),
        badge_family_audit_ref: "artifacts/components/m5-badge-family-components.md".to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BADGE_FAMILY_SCHEMA_REF,
        M5_BADGE_FAMILY_DOC_REF,
        M5_BADGE_FAMILY_SUPPORT_CLASS_REF,
        M5_BADGE_FAMILY_FRESHNESS_REF,
        M5_BADGE_FAMILY_LIFECYCLE_REF,
        M5_BADGE_FAMILY_COMPATIBILITY_REF,
        M5_BADGE_FAMILY_BADGE_VOCABULARY_REF,
    ])
}

/// Builds the canonical frozen M5 badge-family matrix packet.
pub fn seeded_m5_badge_family_matrix() -> M5BadgeFamilyMatrixPacket {
    M5BadgeFamilyMatrixPacket::new(M5BadgeFamilyMatrixPacketInput {
        packet_id: M5_BADGE_FAMILY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 support-class, evidence-freshness, lifecycle, channel, deployment-scope, compatibility-state, and explanation-drawer badge matrix"
                .to_owned(),
        badge_rows: badge_rows(),
        vocabulary_set: M5BadgeVocabularySet::canonical(),
        axis_separation_rules: axis_separation_rules(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the channel badge is held at Beta because a slice of channel
/// values do not yet round-trip across every export path; every badge family stays
/// visible.
pub fn seeded_m5_badge_family_matrix_channel_badge_beta_narrowed() -> M5BadgeFamilyMatrixPacket {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.packet_id = "m5-badge-family:channel-badge-beta:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::Channel)
        .expect("channel badge row present");
    row.qualification = M5BadgeQualificationClass::Beta;
    packet
}

/// Narrowed variant: the compatibility-state badge is narrowed to Preview pending
/// compatibility-forecast parity proof across every host; every badge family stays
/// visible.
pub fn seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed(
) -> M5BadgeFamilyMatrixPacket {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.packet_id = "m5-badge-family:compatibility-state-badge-preview:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::CompatibilityState)
        .expect("compatibility-state badge row present");
    row.qualification = M5BadgeQualificationClass::Preview;
    packet
}
