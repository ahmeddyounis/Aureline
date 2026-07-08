//! Canonical seed builders for the M5 compatibility-state badge primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical compatibility-state badge primitive packet.
pub const M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_PACKET_ID: &str =
    "m5-compatibility-state-badge-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full compatibility-state input.
fn case(
    subject_label: &str,
    state: M5CompatibilityStateBadgeValue,
    reconciliation_detail_repr: Option<&str>,
    last_evaluated_repr: &str,
) -> M5CompatibilityStateResolutionCase {
    M5CompatibilityStateResolutionCase::resolved(M5CompatibilityStateBadgeInput {
        subject_label: subject_label.to_owned(),
        state,
        reconciliation_detail_repr: reconciliation_detail_repr.map(str::to_owned),
        last_evaluated_repr: last_evaluated_repr.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, state, posture, gap,
/// residual-capability, repair-action, explanation-field, export-field, and accessibility
/// parity every consumer carries.
fn base_row(
    consumer_surface: M5CompatibilityStateConsumerSurface,
    qualification: M5BadgeQualificationClass,
    owner_role: &str,
    state_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5CompatibilityStateResolutionCase>,
) -> M5CompatibilityStateRow {
    M5CompatibilityStateRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        state_summary: state_summary.to_owned(),
        surface_families: M5BadgeSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5CompatibilityStateAnatomyPart::ALL.to_vec(),
        state_values: M5CompatibilityStateBadgeValue::ALL.to_vec(),
        compatibility_postures: M5CompatibilityPosture::ALL.to_vec(),
        gap_classes: M5CompatibilityGapClass::ALL.to_vec(),
        residual_capabilities: M5CompatibilityResidualCapability::ALL.to_vec(),
        repair_actions: M5CompatibilityRepairAction::ALL.to_vec(),
        explanation_fields: M5BadgeExplanationField::ALL.to_vec(),
        export_fields: M5CompatibilityStateExportField::ALL.to_vec(),
        accessibility_routes: M5BadgeAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5BadgeConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5BadgeDowngradeTrigger::CompatibilityStateUnstated,
            M5BadgeDowngradeTrigger::ExplanationDrawerMissing,
            M5BadgeDowngradeTrigger::AxisMergedIntoAnother,
            M5BadgeDowngradeTrigger::FilterKeyDropped,
            M5BadgeDowngradeTrigger::ExportLostBadgeMeaning,
            M5BadgeDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF,
            M5_COMPATIBILITY_STATE_BADGE_FAMILY_MATRIX_REF,
            M5_COMPATIBILITY_STATE_BADGE_REPAIR_REF,
            M5_COMPATIBILITY_STATE_BADGE_COMPARE_REF,
        ]),
        example_resolutions,
        collapses_state_into_support_lifecycle_or_channel: false,
        implies_support_class_from_compatibility_state: false,
        drops_reconciliation_detail_on_mismatch: false,
        drops_badge_meaning_in_export: false,
    }
}

fn badge_rows() -> Vec<M5CompatibilityStateRow> {
    use M5CompatibilityStateBadgeValue as State;

    vec![
    // 1. Workspace reopen card — an exact-match portable-state artifact that is full parity
    //    and safe to reopen, alongside a mismatched artifact whose reconciliation note
    //    names the version/schema gap and repair action (the preflight-posture-disclosure
    //    proof: the posture spans full parity through mismatch, presented before reopen).
    base_row(
        M5CompatibilityStateConsumerSurface::WorkspaceReopenCard,
        M5BadgeQualificationClass::Stable,
        "Workspace reopen compatibility badge owner",
        "The workspace reopen card renders the shared compatibility-state badge so an exact-match portable-state artifact reads as full parity and safe to reopen, and a mismatched artifact reads as incompatible-as-claimed with its version/schema gap, residual capability, and repair action preserved — proving the compatibility state is its own axis, presented before the reopen proceeds, and never collapses into support class, lifecycle, or channel",
        "evidence:m5-compatibility-state-parity:001",
        vec![
            case(
                "aureline workspace: portable session snapshot",
                State::ExactMatch,
                None,
                "2026-07-01T00:00:00Z",
            ),
            case(
                "aureline workspace: stale layout artifact",
                State::Mismatch,
                Some("schema:layout-v3-vs-v4/removed-panes"),
                "2026-06-14T00:00:00Z",
            ),
        ],
    ),

    // 2. Toolchain install row — a compatible toolchain that installs without reconciliation,
    //    alongside a limited toolchain whose reduced-capability subset is disclosed before
    //    install.
    base_row(
        M5CompatibilityStateConsumerSurface::ToolchainInstallRow,
        M5BadgeQualificationClass::Stable,
        "Toolchain install compatibility badge owner",
        "The toolchain install row renders the shared compatibility-state badge so a compatible toolchain reads as compatible-within-range and installs without reconciliation, and a limited toolchain reads as reduced-capability with the exact reduced subset and compare/review action disclosed before install — the same compatibility vocabulary an install reviewer reads elsewhere",
        "evidence:m5-compatibility-state-parity:002",
        vec![
            case(
                "aureline toolchain: pinned formatter within range",
                State::Compatible,
                None,
                "2026-07-02T00:00:00Z",
            ),
            case(
                "aureline toolchain: analyzer with narrowed feature set",
                State::Limited,
                Some("toolchain:pin-1.79-vs-1.82/abi-subset"),
                "2026-06-20T00:00:00Z",
            ),
        ],
    ),

    // 3. Extension import row — a limited extension whose reduced capability subset is
    //    disclosed before import, alongside an exact-match extension.
    base_row(
        M5CompatibilityStateConsumerSurface::ExtensionImportRow,
        M5BadgeQualificationClass::Stable,
        "Extension import compatibility badge owner",
        "The extension import row renders the shared compatibility-state badge so a limited extension reads as reduced-capability and continues with a reduced scope — disclosing exactly which capabilities are narrowed before import — and an exact-match extension reads as full parity, so a Limited reading is a reviewable narrowing rather than a silent exclusion",
        "evidence:m5-compatibility-state-parity:003",
        vec![
            case(
                "aureline extension: linter with partial ruleset",
                State::Limited,
                Some("capability:subset-4of6/skips-remote-eval"),
                "2026-05-30T00:00:00Z",
            ),
            case(
                "aureline extension: theme pack exact target",
                State::ExactMatch,
                None,
                "2026-06-30T00:00:00Z",
            ),
        ],
    ),

    // 4. Workflow-bundle apply card — a mismatched bundle whose repair-before-apply action
    //    and gap detail are preserved, alongside a compatible bundle, so the apply flow
    //    surfaces a repair entrypoint before proceeding.
    base_row(
        M5CompatibilityStateConsumerSurface::WorkflowBundleApplyCard,
        M5BadgeQualificationClass::Stable,
        "Workflow bundle apply compatibility badge owner",
        "The workflow-bundle apply card renders the shared compatibility-state badge so a mismatched bundle reads as incompatible-as-claimed and is blocked-until-reconciled — preserving the version/schema gap and a repair-before-apply entrypoint — and a compatible bundle reads as compatible-within-range, so a risky apply is gated on an explicit posture instead of a generic warning",
        "evidence:m5-compatibility-state-parity:004",
        vec![
            case(
                "aureline bundle: migration bundle against newer schema",
                State::Mismatch,
                Some("schema:bundle-manifest-v2-vs-v5/breaking-steps"),
                "2026-06-05T00:00:00Z",
            ),
            case(
                "aureline bundle: review bundle within range",
                State::Compatible,
                None,
                "2026-07-03T00:00:00Z",
            ),
        ],
    ),

    // 5. Compare / review panel — a limited artifact and a mismatched artifact side by side,
    //    each keeping its distinct gap, residual capability, and repair/compare action (the
    //    limited-and-mismatch distinctness proof: two non-parity readings never collapse
    //    into one generic warning).
    base_row(
        M5CompatibilityStateConsumerSurface::CompareReviewPanel,
        M5BadgeQualificationClass::Stable,
        "Compare review compatibility badge owner",
        "The compare / review panel renders the shared compatibility-state badge so a limited artifact reads as reduced-capability with a compare-and-review action and a mismatched artifact reads as incompatible-as-claimed with a repair-before-apply action — the two non-parity readings stay distinct, detail-preserving cues a reviewer can compare directly rather than one collapsed warning",
        "evidence:m5-compatibility-state-parity:005",
        vec![
            case(
                "aureline compare: locale pack with partial coverage",
                State::Limited,
                Some("capability:locale-subset-82pct/missing-plurals"),
                "2026-06-18T00:00:00Z",
            ),
            case(
                "aureline compare: imported settings against renamed keys",
                State::Mismatch,
                Some("schema:settings-v7-vs-v9/renamed-keys"),
                "2026-06-01T00:00:00Z",
            ),
        ],
    ),

    // 6. Support-export row — a mismatched artifact whose full reconciliation detail survives
    //    into exported evidence, alongside an exact-match artifact, so exported evidence
    //    never loses the badge's meaning.
    base_row(
        M5CompatibilityStateConsumerSurface::SupportExportRow,
        M5BadgeQualificationClass::Stable,
        "Support export compatibility badge owner",
        "The support-export row renders the shared compatibility-state badge so a mismatched artifact carries its state, posture, gap class, reconciliation detail, and residual capability as separate fields into exported evidence — enough to repair, compare, and narrow the claim later — and an exact-match artifact reads as full parity, so exported evidence never loses the state's meaning",
        "evidence:m5-compatibility-state-parity:006",
        vec![
            case(
                "aureline support: portable artifact against pinned schema",
                State::Mismatch,
                Some("schema:artifact-v4-vs-v6/dropped-fields"),
                "2026-03-18T00:00:00Z",
            ),
            case(
                "aureline support: matched diagnostic bundle",
                State::ExactMatch,
                None,
                "2026-07-05T00:00:00Z",
            ),
        ],
    ),
    ]
}

fn governance_review() -> M5CompatibilityStateGovernanceReview {
    M5CompatibilityStateGovernanceReview {
        compatibility_state_shown_as_distinct_cue: true,
        state_never_collapsed_into_support_lifecycle_or_channel: true,
        compatibility_state_never_implies_support_class: true,
        compatibility_state_never_implies_lifecycle: true,
        posture_presented_before_install_import_apply_reopen: true,
        mismatch_auto_discloses_reconciliation_detail: true,
        reconciliation_note_preserves_state_context: true,
        limited_and_mismatch_preserve_repair_and_compare_detail: true,
        downgrade_behavior_is_visible_not_silent: true,
        every_badge_opens_explanation_drawer: true,
        every_badge_is_separately_filterable: true,
        exported_evidence_keeps_state_meaning: true,
        every_row_declares_accessibility_route: true,
    }
}

fn consumer_projection() -> M5CompatibilityStateConsumerProjection {
    M5CompatibilityStateConsumerProjection {
        install_import_apply_reopen_surfaces_consume_shared_state_badge: true,
        compare_review_and_export_surfaces_consume_shared_state_badge: true,
        state_filter_reads_single_source: true,
        compatibility_posture_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CompatibilityStateProofFreshness {
    M5CompatibilityStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CompatibilityStateReleasePosture {
    M5CompatibilityStateReleasePosture {
        release_packet_ref: M5_COMPATIBILITY_STATE_BADGE_ARTIFACT_REF.to_owned(),
        badge_audit_ref: M5_COMPATIBILITY_STATE_BADGE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF,
        M5_COMPATIBILITY_STATE_BADGE_DOC_REF,
        M5_COMPATIBILITY_STATE_BADGE_FAMILY_MATRIX_REF,
        M5_COMPATIBILITY_STATE_BADGE_REPAIR_REF,
        M5_COMPATIBILITY_STATE_BADGE_COMPARE_REF,
    ])
}

/// Builds the canonical M5 compatibility-state badge primitive packet.
pub fn seeded_m5_compatibility_state_badge_primitive_packet(
) -> M5CompatibilityStateBadgePrimitivePacket {
    M5CompatibilityStateBadgePrimitivePacket::new(M5CompatibilityStateBadgePrimitivePacketInput {
        packet_id: M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 compatibility-state badge primitive: exact-match/compatible/limited/mismatch parity as one distinct, composable cue with reconciliation, repair, and compare detail preserved before install/import/apply/reopen"
                .to_owned(),
        badge_rows: badge_rows(),
        vocabulary_set: M5CompatibilityStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the compare / review panel is held at Beta because a slice of
/// compare surfaces do not yet render the reconciliation drawer on every profile; every
/// badge consumer stays visible.
pub fn seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed(
) -> M5CompatibilityStateBadgePrimitivePacket {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.packet_id = "m5-compatibility-state-badge-primitive:compare-beta:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CompatibilityStateConsumerSurface::CompareReviewPanel)
        .expect("compare review panel present");
    row.qualification = M5BadgeQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support-export row is narrowed to Preview pending reconciliation
/// detail parity proof across every export path; every badge consumer stays visible.
pub fn seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed(
) -> M5CompatibilityStateBadgePrimitivePacket {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.packet_id =
        "m5-compatibility-state-badge-primitive:support-export-preview:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CompatibilityStateConsumerSurface::SupportExportRow)
        .expect("support export row present");
    row.qualification = M5BadgeQualificationClass::Preview;
    packet
}
