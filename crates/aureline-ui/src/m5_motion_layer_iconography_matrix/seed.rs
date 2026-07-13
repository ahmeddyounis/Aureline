//! Canonical seed builders for the frozen M5 motion / layer / iconography matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical visual-interaction matrix.
pub const M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_PACKET_ID: &str =
    "m5-motion-layer-iconography:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5VisualInteractionRequiredLabel> {
    M5VisualInteractionRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(
    extra: &[M5VisualInteractionRequiredLabel],
) -> Vec<M5VisualInteractionRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    interaction_family: M5VisualInteractionFamily,
    qualification: M5VisualInteractionQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5VisualInteractionRow {
    M5VisualInteractionRow {
        interaction_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5VisualInteractionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5VisualInteractionDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        motion_roles: vec![],
        reduced_motion_roles: vec![],
        opacity_scrim_roles: vec![],
        layer_order_roles: vec![],
        portal_ownership_roles: vec![],
        iconography_roles: vec![],
        illustration_roles: vec![],
        degraded_reasons: M5VisualInteractionDegradedReason::ALL.to_vec(),
        accessibility_routes: M5VisualInteractionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5VisualInteractionConsumerSurface::SupportExport,
            M5VisualInteractionConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5VisualInteractionDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        delays_protected_input_with_motion: false,
        scrim_erases_orientation_or_contrast: false,
        overlay_bypasses_shared_z_order: false,
        uses_unlabeled_icon_for_uncommon_or_destructive_action: false,
        lets_illustration_impersonate_operational_or_security_truth: false,
    }
}

fn interaction_rows() -> Vec<M5VisualInteractionRow> {
    use M5IconographyRole as IC;
    use M5IllustrationRole as IL;
    use M5LayerOrderRole as LA;
    use M5MotionTokenRole as MO;
    use M5OpacityScrimRole as OP;
    use M5PortalOwnershipRole as PO;
    use M5ReducedMotionRole as RM;
    use M5VisualInteractionConsumerSurface as C;
    use M5VisualInteractionDowngradeTrigger as D;
    use M5VisualInteractionFamily as F;
    use M5VisualInteractionQualificationClass as Q;
    use M5VisualInteractionRequiredLabel as L;
    use M5VisualInteractionRole as R;

    let mut rows = Vec::new();

    // 1. Motion tokens.
    let mut row = base_row(
        F::MotionToken,
        Q::Stable,
        "Design-system foundations owner",
        "One motion-token system naming duration and easing families that clarify origin, continuity, and completion without ever delaying input on protected paths like the command palette or typing-critical surfaces",
        "evidence:m5-motion-token-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF],
    );
    row.motion_roles = MO::ALL.to_vec();
    row.semantic_roles = vec![R::Motion, R::Attention];
    row.required_labels = labels_with(&[L::MotionProfile]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::OnboardingUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::MotionDelayedProtectedInput,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Reduced motion.
    let mut row = base_row(
        F::ReducedMotion,
        Q::Stable,
        "Accessibility foundations owner",
        "One reduced-motion contract naming the reduced-motion, power-saver, and thermal clamps and the static fallback that preserves the same meaning so no motion ever carries the only cue",
        "evidence:m5-reduced-motion-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF],
    );
    row.reduced_motion_roles = RM::ALL.to_vec();
    row.semantic_roles = vec![R::Motion, R::Attention];
    row.required_labels = labels_with(&[L::MotionProfile]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::HelpUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::MotionMeaningLostUnderReducedMotion,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Opacity / scrim.
    let mut row = base_row(
        F::OpacityScrim,
        Q::Stable,
        "Shell surface owner",
        "One opacity / scrim class set naming scrim layers and opacity levels that preserve workspace orientation and text contrast and always offer a dismiss affordance so an overlay never erases context",
        "evidence:m5-opacity-scrim-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_OPACITY_SCRIM_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF],
    );
    row.opacity_scrim_roles = OP::ALL.to_vec();
    row.semantic_roles = vec![R::Overlay, R::Attention];
    row.required_labels = labels_with(&[L::AccessibleFallback]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::OnboardingUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ScrimErasedOrientationOrContrast,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Layer order.
    let mut row = base_row(
        F::LayerOrder,
        Q::Stable,
        "Shell surface owner",
        "One z-order model naming base-content, overlay, dialog, and notification tiers so every menu, popover, dialog, and toast stacks under a single shared order no private overlay can bypass",
        "evidence:m5-layer-order-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF],
    );
    row.layer_order_roles = LA::ALL.to_vec();
    row.semantic_roles = vec![R::Layer, R::Overlay];
    row.required_labels = labels_with(&[L::LayerTier]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::MarketplaceUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OverlayBypassedSharedZOrder,
        D::LayerTierUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Portal ownership.
    let mut row = base_row(
        F::PortalOwnership,
        Q::Stable,
        "Shell surface owner",
        "One portal-ownership contract so every portal attaches to its owning surface, contains focus, dismisses with its owner, and stacks under the shared z-order — including extension and embedded overlays",
        "evidence:m5-portal-ownership-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF],
    );
    row.portal_ownership_roles = PO::ALL.to_vec();
    row.semantic_roles = vec![R::Portal, R::Layer];
    row.required_labels = labels_with(&[L::LayerTier]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::MarketplaceUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PortalDetachedFromOwningSurface,
        D::LayerTierUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Iconography.
    let mut row = base_row(
        F::Iconography,
        Q::Stable,
        "Design-system foundations owner",
        "One iconography system naming status, action, and navigation icon categories that stay semantic and always carry a text label for uncommon or destructive actions",
        "evidence:m5-iconography-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF],
    );
    row.iconography_roles = IC::ALL.to_vec();
    row.semantic_roles = vec![R::Icon, R::Attention];
    row.required_labels = labels_with(&[L::AccessibleFallback]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::HelpUi,
        C::MarketplaceUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::UnlabeledIconForUncommonOrDestructiveAction,
        D::IconSemanticsAmbiguous,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Illustration.
    let mut row = base_row(
        F::Illustration,
        Q::Stable,
        "Design-system foundations owner",
        "One illustration-boundary contract keeping empty-state, onboarding, and decorative illustrations secondary to content so an illustration never impersonates operational state, safety approval, or security messaging",
        "evidence:m5-illustration-boundary-parity:001",
        &[M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF, M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF],
    );
    row.illustration_roles = IL::ALL.to_vec();
    row.semantic_roles = vec![R::Illustration, R::Attention];
    row.required_labels = labels_with(&[L::AccessibleFallback]);
    row.consumer_surfaces = vec![
        C::OnboardingUi,
        C::HelpUi,
        C::MarketplaceUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::IllustrationImpersonatedOperationalState,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5VisualInteractionGovernanceReview {
    M5VisualInteractionGovernanceReview {
        motion_clarifies_origin_continuity_completion: true,
        motion_never_delays_protected_input: true,
        reduced_motion_power_saver_thermal_clamps_respected: true,
        scrims_preserve_orientation_and_contrast: true,
        layers_follow_single_z_order_model: true,
        portals_attach_to_owning_surface: true,
        extension_overlays_cannot_bypass_shared_z_order: true,
        icons_stay_semantic_and_labeled: true,
        uncommon_or_destructive_icons_are_labeled: true,
        illustrations_remain_secondary: true,
        illustrations_never_impersonate_trust_or_severity: true,
        motion_tokens_bind_to_appearance_session: true,
        every_family_declares_deployment_lines: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_visual_interaction_source: true,
        later_rows_cannot_invent_parallel_motion_layer_icon_vocabulary: true,
    }
}

fn consumer_projection() -> M5VisualInteractionConsumerProjection {
    M5VisualInteractionConsumerProjection {
        shell_and_dialog_consume_shared_motion_and_layer_grammar: true,
        onboarding_and_notification_consume_shared_icon_and_illustration_language: true,
        embedded_surfaces_consume_shared_z_order_and_portal_model: true,
        motion_layer_icon_consumers_read_single_token_source: true,
        appearance_session_binds_to_shared_motion_tokens: true,
        support_export_reads_single_visual_interaction_source: true,
    }
}

fn proof_freshness() -> M5VisualInteractionProofFreshness {
    M5VisualInteractionProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5VisualInteractionReleasePosture {
    M5VisualInteractionReleasePosture {
        proof_packet_ref: M5_MOTION_LAYER_ICONOGRAPHY_ARTIFACT_REF.to_owned(),
        interaction_audit_ref: M5_MOTION_LAYER_ICONOGRAPHY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
        M5_OPACITY_SCRIM_SCHEMA_REF,
        M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
        M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 motion / layer / iconography matrix packet.
pub fn seeded_m5_motion_layer_iconography_matrix() -> M5MotionLayerIconographyMatrixPacket {
    M5MotionLayerIconographyMatrixPacket::new(M5MotionLayerIconographyMatrixPacketInput {
        packet_id: M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 motion-token, reduced-motion, opacity / scrim, layer-order, portal-ownership, iconography, and illustration-boundary visual-interaction matrix"
                .to_owned(),
        interaction_rows: interaction_rows(),
        vocabulary_set: M5VisualInteractionVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: reduced motion is held at Beta because power-saver and thermal clamp parity is not
/// yet proven across every deployment line; every family stays visible.
pub fn seeded_m5_motion_layer_iconography_matrix_reduced_motion_beta_narrowed(
) -> M5MotionLayerIconographyMatrixPacket {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.packet_id = "m5-motion-layer-iconography:reduced-motion-beta:0001".to_owned();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::ReducedMotion)
        .expect("reduced-motion row present");
    row.qualification = M5VisualInteractionQualificationClass::Beta;
    packet
}

/// Narrowed variant: illustration is narrowed to Preview pending illustration-boundary parity across every
/// deployment line; every family stays visible.
pub fn seeded_m5_motion_layer_iconography_matrix_illustration_preview_narrowed(
) -> M5MotionLayerIconographyMatrixPacket {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.packet_id = "m5-motion-layer-iconography:illustration-preview:0001".to_owned();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::Illustration)
        .expect("illustration row present");
    row.qualification = M5VisualInteractionQualificationClass::Preview;
    packet
}
