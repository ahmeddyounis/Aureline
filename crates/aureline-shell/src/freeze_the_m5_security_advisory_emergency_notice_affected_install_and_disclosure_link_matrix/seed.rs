//! Canonical seed builders for the frozen M5 security-advisory, emergency-notice,
//! affected-install, and disclosure-link component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical advisory-component matrix.
pub const M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID: &str = "m5-advisory-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-01T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5AdvisoryRequiredLabel> {
    M5AdvisoryRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5AdvisoryRequiredLabel]) -> Vec<M5AdvisoryRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5AdvisoryComponentFamily,
    qualification: M5AdvisoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
) -> M5AdvisoryComponentRow {
    M5AdvisoryComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        surface_families: M5ShellSurfaceFamily::ALL.to_vec(),
        required_labels: mandatory_labels(),
        severity_classes: M5AdvisorySeverityClass::ALL.to_vec(),
        projection_surfaces: M5AdvisoryProjectionSurface::ALL.to_vec(),
        anatomy_fields: vec![],
        action_states: vec![],
        required_actions: vec![],
        dismissal_states: vec![],
        continuity_claims: vec![],
        delivery_profiles: vec![],
        freshness_states: vec![],
        disclosure_fields: vec![],
        notification_behaviors: vec![],
        export_fields: vec![],
        accessibility_routes: M5AdvisoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5AdvisoryDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ADVISORY_COMPONENTS_SCHEMA_REF,
            M5_ADVISORY_COMPONENTS_ADVISORY_CARD_CONTRACT_REF,
            M5_ADVISORY_COMPONENTS_SHELL_ZONE_REF,
        ]),
        hides_affected_scope: false,
        hides_local_continuity: false,
        invents_generic_advisory_language: false,
        stays_silent_on_stale_or_unsigned: false,
    }
}

fn component_rows() -> Vec<M5AdvisoryComponentRow> {
    use M5AdvisoryActionState as A;
    use M5AdvisoryAnatomyField as N;
    use M5AdvisoryComponentFamily as F;
    use M5AdvisoryContinuityClaim as CN;
    use M5AdvisoryDeliveryProfile as DP;
    use M5AdvisoryDisclosureField as DF;
    use M5AdvisoryDismissalState as DS;
    use M5AdvisoryDowngradeTrigger as D;
    use M5AdvisoryExportField as EF;
    use M5AdvisoryFreshnessState as FR;
    use M5AdvisoryNotificationBehavior as NB;
    use M5AdvisoryProjectionSurface as P;
    use M5AdvisoryQualificationClass as Q;
    use M5AdvisoryRequiredAction as RA;
    use M5AdvisoryRequiredLabel as L;
    use M5AdvisorySeverityClass as S;
    use M5ShellConsumerSurface as C;
    use M5ShellZoneSlot as Z;

    let mut rows = Vec::new();

    // 1. Security-advisory card.
    let mut row = base_row(
        F::AdvisoryCard,
        Q::Stable,
        "Security advisory component owner",
        "One security-advisory card model that names the affected object, its severity, current exposure, the fixed version or mitigation, the signer/source continuity state, and the primary actions, and always states what still works locally — never a generic update banner",
        Z::MainWorkspace,
        "evidence:m5-advisory-card-parity:001",
    );
    row.anatomy_fields = N::ALL.to_vec();
    row.action_states = A::ALL.to_vec();
    row.required_actions = RA::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::PrimaryAction, L::ContinuityNote]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AffectedScopeHidden,
        D::ExposureHiddenBehindGenericBanner,
        D::FixedVersionOrMitigationMissing,
        D::SignerSourceStateHidden,
        D::LocalContinuityHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Emergency notice.
    let mut row = base_row(
        F::EmergencyNotice,
        Q::Stable,
        "Emergency response component owner",
        "One emergency-notice model that stays explicit about blast radius, the acknowledge/snooze/dismiss rules, and the forced-disable scope, and that cannot be silently dismissed while an exposure is unremediated",
        Z::TitleContextBar,
        "evidence:m5-emergency-notice-parity:001",
    );
    row.severity_classes = vec![S::High, S::Critical, S::OperationalEmergency];
    row.action_states = vec![
        A::ActionRequired,
        A::Blocking,
        A::ImmediateRemediation,
        A::MitigationComplete,
    ];
    row.required_actions = vec![
        RA::UpdateToFixedVersion,
        RA::RollbackOrRepin,
        RA::DisableOrRemove,
        RA::RotateTrustRoot,
        RA::ContactAdmin,
        RA::WaitForSupersedingAction,
    ];
    row.dismissal_states = DS::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::PrimaryAction, L::ContinuityNote]);
    row.projection_surfaces = vec![
        P::UpdateCenter,
        P::HelpAbout,
        P::SupportBundle,
        P::NativeNotification,
        P::ActivityCenter,
        P::ReleasePacket,
    ];
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AffectedScopeHidden,
        D::DismissalRuleViolated,
        D::ForcedDisableScopeHidden,
        D::LocalContinuityHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Affected-install panel.
    let mut row = base_row(
        F::AffectedInstallPanel,
        Q::Stable,
        "Install/update component owner",
        "One affected-install panel model that assesses which install lanes are affected from the same install-profile, exact-build, delivery-profile, and mirror-freshness vocabulary, and that discloses mirror lag and the local-continuity claim instead of staying green",
        Z::RightInspector,
        "evidence:m5-affected-install-parity:001",
    );
    row.continuity_claims = CN::ALL.to_vec();
    row.delivery_profiles = DP::ALL.to_vec();
    row.freshness_states = FR::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::PrimaryAction, L::ContinuityNote]);
    row.source_contract_refs = strings(&[
        M5_ADVISORY_COMPONENTS_SCHEMA_REF,
        M5_ADVISORY_COMPONENTS_AFFECTED_INSTALL_CONTRACT_REF,
        M5_ADVISORY_COMPONENTS_SHELL_ZONE_REF,
    ]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AffectedScopeHidden,
        D::LocalContinuityHidden,
        D::MirrorLagUndisclosed,
        D::UnsignedDistributionUndisclosed,
        D::StaleNoticeStateSilent,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Disclosure / history block.
    let mut row = base_row(
        F::DisclosureBlock,
        Q::Stable,
        "Disclosure/history component owner",
        "One disclosure/history block model that carries the copy-safe Aureline advisory id with CVE and GHSA aliases, the disclosure timing and visibility posture, and the resolved-versus-active history — so disclosure lives in the product and is never flattened into a single link to an external page",
        Z::BottomPanel,
        "evidence:m5-disclosure-block-parity:001",
    );
    row.disclosure_fields = DF::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::PrimaryAction]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::DocsHelp,
        C::ReleaseProof,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AffectedScopeHidden,
        D::ExternalDisclosureOnly,
        D::StaleNoticeStateSilent,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Advisory activity row.
    let mut row = base_row(
        F::AdvisoryActivityRow,
        Q::Stable,
        "Activity/history component owner",
        "One advisory activity-row model that projects each advisory event into the activity center and the support export with the advisory id, severity, action state, affected surface, mitigation state, delivery profile, freshness state, continuity note, disclosure visibility, and history state — so a support bundle reconstructs advisory truth without a screenshot",
        Z::BottomPanel,
        "evidence:m5-advisory-activity-row-parity:001",
    );
    row.export_fields = EF::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::ContinuityNote]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::ReleaseProof,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AffectedScopeHidden,
        D::StaleNoticeStateSilent,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Native notification handoff.
    let mut row = base_row(
        F::NativeNotificationHandoff,
        Q::Stable,
        "Notification routing component owner",
        "One native-notification handoff model that surfaces a compact OS summary with no sensitive body, clicks through to the in-product advisory, respects quiet hours for non-emergency severities while letting an emergency bypass them, and syncs OS dismissal to the in-app dismissal state",
        Z::TransientOverlay,
        "evidence:m5-native-notification-parity:001",
    );
    row.notification_behaviors = NB::ALL.to_vec();
    row.severity_classes = vec![S::Moderate, S::High, S::Critical, S::OperationalEmergency];
    row.required_labels = labels_with(&[L::Provenance, L::PrimaryAction]);
    row.projection_surfaces = vec![P::NativeNotification, P::ActivityCenter, P::SupportBundle];
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::NotificationEnvelope,
        C::AttentionRouter,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AffectedScopeHidden,
        D::DismissalRuleViolated,
        D::StaleNoticeStateSilent,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5AdvisoryComponentGovernanceReview {
    M5AdvisoryComponentGovernanceReview {
        advisory_card_names_required_anatomy: true,
        severity_vocabulary_is_closed: true,
        emergency_notice_declares_blast_radius_and_dismissal: true,
        affected_install_panel_preserves_local_continuity: true,
        disclosure_block_keeps_ids_and_history: true,
        activity_row_reconstructable_from_support_export: true,
        native_notification_carries_no_sensitive_body: true,
        mirror_lag_or_unsigned_auto_narrows: true,
        no_component_invents_generic_advisory_language: true,
        every_component_bound_to_shell_zone: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5AdvisoryComponentConsumerProjection {
    M5AdvisoryComponentConsumerProjection {
        update_center_reads_advisory_matrix: true,
        marketplace_reads_advisory_matrix: true,
        help_about_reads_advisory_matrix: true,
        support_bundle_reads_single_source: true,
        native_notifications_read_single_source: true,
        mirror_offline_drills_read_single_source: true,
    }
}

fn proof_freshness() -> M5AdvisoryComponentProofFreshness {
    M5AdvisoryComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AdvisoryComponentReleasePosture {
    M5AdvisoryComponentReleasePosture {
        release_packet_ref: "artifacts/release/m5-advisory-proof/support_export.json".to_owned(),
        advisory_component_audit_ref: "artifacts/security/m5-advisory-component-matrix.md"
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ADVISORY_COMPONENTS_SCHEMA_REF,
        M5_ADVISORY_COMPONENTS_DOC_REF,
        M5_ADVISORY_COMPONENTS_SHELL_ZONE_REF,
        M5_ADVISORY_COMPONENTS_ADVISORY_CARD_CONTRACT_REF,
        M5_ADVISORY_COMPONENTS_AFFECTED_INSTALL_CONTRACT_REF,
        M5_ADVISORY_COMPONENTS_SEVERITY_MATRIX_REF,
    ])
}

/// Builds the canonical frozen M5 advisory-component matrix packet.
pub fn seeded_m5_advisory_component_matrix() -> M5AdvisoryComponentMatrixPacket {
    M5AdvisoryComponentMatrixPacket::new(M5AdvisoryComponentMatrixPacketInput {
        packet_id: M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 security-advisory, emergency-notice, affected-install, and disclosure-link component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5AdvisoryComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the emergency notice is held at Beta because a slice of
/// managed and offline-mirror lanes do not yet prove forced-disable-scope parity
/// across every drill; every component stays visible.
pub fn seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed(
) -> M5AdvisoryComponentMatrixPacket {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.packet_id = "m5-advisory-components:emergency-notice-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AdvisoryComponentFamily::EmergencyNotice)
        .expect("emergency-notice row present");
    row.qualification = M5AdvisoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the affected-install panel is narrowed to Preview pending
/// mirror-freshness parity proof across every offline drill; every component
/// stays visible.
pub fn seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed(
) -> M5AdvisoryComponentMatrixPacket {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.packet_id = "m5-advisory-components:affected-install-panel-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AdvisoryComponentFamily::AffectedInstallPanel)
        .expect("affected-install-panel row present");
    row.qualification = M5AdvisoryQualificationClass::Preview;
    packet
}
