//! Canonical seed builders for the M5 adapter-confidence-chip / discovery-diff-card controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_PACKET_ID: &str =
    "m5-adapter-confidence-chip-discovery-diff-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn chip(input: M5AdapterConfidenceChipResolutionInput) -> M5ResolvedAdapterConfidenceChip {
    resolve_adapter_confidence_chip(input).expect("seed adapter-confidence chip input resolves")
}

fn card(input: M5DiscoveryDiffCardResolutionInput) -> M5ResolvedDiscoveryDiffCard {
    resolve_discovery_diff_card(input).expect("seed discovery-diff card input resolves")
}

// -- Canonical adapter-confidence chip examples ------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn chip_input(
    chip_id: &str,
    adapter_source_class: TargetDiscoveryClass,
    source_class_disclosed: bool,
    adapter_confidence: AdapterConfidence,
    confidence_band_disclosed: bool,
    discovery_mode: DiscoveryConfidence,
    discovery_mode_disclosed: bool,
    stale: bool,
    current_downgrade_reason: Option<NarrowingReason>,
) -> M5AdapterConfidenceChipResolutionInput {
    M5AdapterConfidenceChipResolutionInput {
        chip_id: chip_id.to_owned(),
        adapter_source_class,
        source_class_disclosed,
        adapter_confidence,
        confidence_band_disclosed,
        discovery_mode,
        discovery_mode_disclosed,
        stale,
        current_downgrade_reason,
        proof_fresh: true,
    }
}

/// Clean chip: an exact, verified, declared-manifest target.
fn chip_exact() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:exact",
        TargetDiscoveryClass::DeclaredManifest,
        true,
        AdapterConfidence::Verified,
        true,
        DiscoveryConfidence::Exact,
        true,
        false,
        None,
    ))
}

/// Clean chip: a compatible, structured-signal target.
fn chip_compatible() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:compatible",
        TargetDiscoveryClass::WorkspaceProbe,
        true,
        AdapterConfidence::High,
        true,
        DiscoveryConfidence::Structured,
        true,
        false,
        None,
    ))
}

/// Clean chip: a heuristic-only target.
fn chip_heuristic() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:heuristic",
        TargetDiscoveryClass::AdapterInferred,
        true,
        AdapterConfidence::Heuristic,
        true,
        DiscoveryConfidence::Heuristic,
        true,
        false,
        None,
    ))
}

/// Clean chip: an imported target reconstructed from a structured import.
fn chip_imported() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:imported",
        TargetDiscoveryClass::UserSupplied,
        true,
        AdapterConfidence::High,
        true,
        DiscoveryConfidence::Imported,
        true,
        false,
        None,
    ))
}

/// Clean chip: a downgraded target that attributes its current downgrade reason.
fn chip_downgraded() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:downgraded",
        TargetDiscoveryClass::AdapterInferred,
        true,
        AdapterConfidence::Unverified,
        true,
        DiscoveryConfidence::Heuristic,
        true,
        false,
        Some(NarrowingReason::AdapterConfidenceLow),
    ))
}

/// Clean chip: a stale target that attributes its current downgrade reason.
fn chip_stale() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:stale",
        TargetDiscoveryClass::WorkspaceProbe,
        true,
        AdapterConfidence::High,
        true,
        DiscoveryConfidence::Structured,
        true,
        true,
        Some(NarrowingReason::EvidenceStale),
    ))
}

/// Degraded chip: the adapter / source class is undisclosed — proves AC1's source-class half.
fn chip_source_class_unstated() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:source-hidden",
        TargetDiscoveryClass::AdapterInferred,
        false,
        AdapterConfidence::High,
        true,
        DiscoveryConfidence::Structured,
        true,
        false,
        None,
    ))
}

/// Degraded chip: the confidence band is undisclosed — proves AC1's confidence-band half.
fn chip_confidence_band_unstated() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:confidence-hidden",
        TargetDiscoveryClass::WorkspaceProbe,
        true,
        AdapterConfidence::High,
        false,
        DiscoveryConfidence::Structured,
        true,
        false,
        None,
    ))
}

/// Degraded chip: the discovery mode is undisclosed — proves AC1's discovery-mode half.
fn chip_discovery_mode_unstated() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:mode-hidden",
        TargetDiscoveryClass::AdapterInferred,
        true,
        AdapterConfidence::Heuristic,
        true,
        DiscoveryConfidence::Heuristic,
        false,
        false,
        None,
    ))
}

/// Degraded chip: a downgraded target carries no attributed current downgrade reason.
fn chip_downgrade_unattributed() -> M5ResolvedAdapterConfidenceChip {
    chip(chip_input(
        "adapter-chip:unattributed",
        TargetDiscoveryClass::AdapterInferred,
        true,
        AdapterConfidence::Unverified,
        true,
        DiscoveryConfidence::Heuristic,
        true,
        false,
        None,
    ))
}

// -- Canonical discovery-diff card examples ----------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn card_input(
    card_id: &str,
    previous_target_identity: &str,
    current_target_identity: &str,
    previous_confidence: DiscoveryConfidence,
    current_confidence: DiscoveryConfidence,
    target_identity_disclosed: bool,
    material_change: bool,
    changed_certainty_disclosed: bool,
    review_before_switch_available: bool,
    attributed_review_state: bool,
) -> M5DiscoveryDiffCardResolutionInput {
    M5DiscoveryDiffCardResolutionInput {
        card_id: card_id.to_owned(),
        previous_target_identity: previous_target_identity.to_owned(),
        current_target_identity: current_target_identity.to_owned(),
        previous_confidence,
        current_confidence,
        target_identity_disclosed,
        material_change,
        changed_certainty_disclosed,
        review_before_switch_available,
        attributed_review_state,
        proof_fresh: true,
    }
}

/// Clean card: a material change presented with an attributable review state and a
/// review-before-switch affordance.
fn card_reviewed() -> M5ResolvedDiscoveryDiffCard {
    card(card_input(
        "discovery-diff:reviewed",
        "web-frontend@service-alpha",
        "web-frontend@service-beta",
        DiscoveryConfidence::Structured,
        DiscoveryConfidence::Structured,
        true,
        true,
        true,
        true,
        true,
    ))
}

/// Clean card: no material change; the resolved target is unchanged.
fn card_no_change() -> M5ResolvedDiscoveryDiffCard {
    card(card_input(
        "discovery-diff:no-change",
        "api-server@main",
        "api-server@main",
        DiscoveryConfidence::Exact,
        DiscoveryConfidence::Exact,
        true,
        false,
        true,
        true,
        true,
    ))
}

/// Degraded card: a material change presented as a silent relabel — proves AC2.
fn card_silent_relabel() -> M5ResolvedDiscoveryDiffCard {
    card(card_input(
        "discovery-diff:silent-relabel",
        "web-frontend@service-alpha",
        "web-frontend@service-gamma",
        DiscoveryConfidence::Structured,
        DiscoveryConfidence::Structured,
        true,
        true,
        true,
        true,
        false,
    ))
}

/// Degraded card: a material change carries no changed-certainty label.
fn card_changed_certainty_unstated() -> M5ResolvedDiscoveryDiffCard {
    card(card_input(
        "discovery-diff:certainty-hidden",
        "worker@queue-a",
        "worker@queue-b",
        DiscoveryConfidence::Structured,
        DiscoveryConfidence::Imported,
        true,
        true,
        false,
        true,
        true,
    ))
}

/// Degraded card: a weaker discovery result would overwrite a stronger resolved target without
/// review — proves the no-higher-confidence-overwrite guardrail.
fn card_overwrite() -> M5ResolvedDiscoveryDiffCard {
    card(card_input(
        "discovery-diff:overwrite",
        "db-migrator@exact",
        "db-migrator@guess",
        DiscoveryConfidence::Exact,
        DiscoveryConfidence::Heuristic,
        true,
        true,
        true,
        false,
        true,
    ))
}

/// Degraded card: the target identity is undisclosed.
fn card_identity_unstated() -> M5ResolvedDiscoveryDiffCard {
    card(card_input(
        "discovery-diff:identity-hidden",
        "",
        "web-frontend@service-beta",
        DiscoveryConfidence::Structured,
        DiscoveryConfidence::Structured,
        false,
        true,
        true,
        true,
        true,
    ))
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5AdapterDiscoveryConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    adapter_confidence_chip_examples: Vec<M5ResolvedAdapterConfidenceChip>,
    discovery_diff_card_examples: Vec<M5ResolvedDiscoveryDiffCard>,
) -> M5AdapterDiscoveryControlsRow {
    M5AdapterDiscoveryControlsRow {
        consumer_surface,
        qualification: M5BuildRemoteQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5BuildRemoteDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5BuildRemoteRequiredLabel::Identity,
            M5BuildRemoteRequiredLabel::State,
            M5BuildRemoteRequiredLabel::KeyboardRoute,
            M5BuildRemoteRequiredLabel::ConfidenceAndDiscovery,
        ],
        accessibility_routes: M5BuildRemoteAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5AdapterDiscoveryAnatomyPart::ALL.to_vec(),
        export_fields: M5AdapterDiscoveryExportField::ALL.to_vec(),
        downgrade_triggers,
        adapter_confidence_chip_examples,
        discovery_diff_card_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_REF,
            M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
            M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
        ]),
        relabels_target_without_attributable_review: false,
        lower_confidence_overwrites_resolved_without_review: false,
        hides_adapter_confidence_or_discovery_mode: false,
        conceals_downgrade_or_drift_in_generic_status_wording: false,
    }
}

fn controls_rows() -> Vec<M5AdapterDiscoveryControlsRow> {
    use M5BuildRemoteConsumerSurface as C;
    use M5BuildRemoteDowngradeTrigger as D;

    vec![
        base_row(
            C::RunTestDebugUi,
            "Run/test/debug surface owner",
            "Every run, test, and debug target renders an adapter-confidence chip naming the adapter/source class, confidence band, and heuristic-vs-structured-vs-imported discovery mode before the user invokes; a discovery-diff card names the previous and current target with an attributable review state and never silently relabels a material change",
            "evidence:m5-adapter-discovery-run-test-debug:001",
            vec![
                D::AdapterConfidenceUnstated,
                D::DiscoveryDriftHidden,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![chip_exact(), chip_heuristic(), chip_source_class_unstated()],
            vec![card_reviewed(), card_silent_relabel()],
        ),
        base_row(
            C::PreviewUi,
            "Preview surface owner",
            "Preview targets reuse the same confidence chip vocabulary, naming the compatible or imported discovery mode and degrading honestly when the confidence band is unstated; the discovery-diff card names the changed certainty rather than hiding a material change",
            "evidence:m5-adapter-discovery-preview:001",
            vec![
                D::AdapterConfidenceUnstated,
                D::DiscoveryDriftHidden,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![
                chip_compatible(),
                chip_imported(),
                chip_confidence_band_unstated(),
            ],
            vec![card_no_change(), card_changed_certainty_unstated()],
        ),
        base_row(
            C::CompanionUi,
            "AI tool-routing owner",
            "AI tool routing reads the same adapter-confidence chip so a downgraded target attributes its current downgrade reason before the model runs, debugs, or hands off work; the discovery-diff card keeps a higher-confidence resolved target instead of letting a weaker heuristic overwrite it without review",
            "evidence:m5-adapter-discovery-ai-tool-routing:001",
            vec![
                D::DiscoveryDriftHidden,
                D::LowerConfidenceOverwroteResolvedTarget,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![chip_downgraded(), chip_discovery_mode_unstated()],
            vec![card_overwrite()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved chip and card truth, so a stale target's attributed downgrade reason, an unattributed downgrade, or an undisclosed target identity is visible in evidence rather than hidden behind feature-local prose",
            "evidence:m5-adapter-discovery-support-export:001",
            vec![
                D::AdapterConfidenceUnstated,
                D::DiscoveryDriftHidden,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![chip_stale(), chip_downgrade_unattributed()],
            vec![card_identity_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product surface owner",
            "In-product surfaces reuse the same confidence and discovery-drift vocabulary the run/test/debug surface shows, keeping the language consistent across shell, notebook, and companion so an exact target reads as exact and a reviewed switch reads as reviewed everywhere",
            "evidence:m5-adapter-discovery-product-ui:001",
            vec![
                D::AdapterConfidenceUnstated,
                D::DiscoveryDriftHidden,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![chip_exact()],
            vec![card_reviewed()],
        ),
    ]
}

fn governance_review() -> M5AdapterDiscoveryGovernanceReview {
    M5AdapterDiscoveryGovernanceReview {
        chip_names_source_class_and_confidence_band: true,
        chip_names_discovery_mode_and_downgrade_reason: true,
        confidence_basis_always_explicit: true,
        card_shows_previous_and_current_target: true,
        card_shows_changed_certainty_and_review_state: true,
        material_drift_never_silently_relabeled: true,
        lower_confidence_never_overwrites_resolved_without_review: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AdapterDiscoveryConsumerProjection {
    M5AdapterDiscoveryConsumerProjection {
        run_test_debug_surfaces_consume_confidence_vocabulary: true,
        preview_surfaces_consume_confidence_vocabulary: true,
        ai_tool_routing_consumes_confidence_vocabulary: true,
        support_export_reads_single_confidence_source: true,
        discovery_language_consistent_across_surfaces: true,
    }
}

fn proof_freshness() -> M5AdapterDiscoveryProofFreshness {
    M5AdapterDiscoveryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AdapterDiscoveryReleasePosture {
    M5AdapterDiscoveryReleasePosture {
        proof_packet_ref: M5_ADAPTER_DISCOVERY_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_ADAPTER_DISCOVERY_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_REF,
        M5_ADAPTER_DISCOVERY_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
        M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
        M5_ADAPTER_DISCOVERY_BUILD_GOVERNANCE_PATH,
        M5_ADAPTER_DISCOVERY_TARGET_DISCOVERY_PATH,
    ])
}

/// Builds the canonical M5 adapter-confidence-chip / discovery-diff-card controls packet.
pub fn seeded_m5_adapter_discovery_controls() -> M5AdapterDiscoveryControlsPacket {
    M5AdapterDiscoveryControlsPacket::new(M5AdapterDiscoveryControlsPacketInput {
        packet_id: M5_ADAPTER_DISCOVERY_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 adapter-confidence-chip and discovery-diff-card controls with adapter/source class, confidence band, heuristic-vs-structured-vs-imported discovery mode, current downgrade reason, previous-vs-current target identity, changed certainty, review-before-switch, and no-higher-confidence-overwrite truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5AdapterDiscoveryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the run/test/debug row is held at Beta pending confidence-chip parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed(
) -> M5AdapterDiscoveryControlsPacket {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.packet_id =
        "m5-adapter-confidence-chip-discovery-diff-card-controls:run-test-debug-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::RunTestDebugUi)
        .expect("run/test/debug row present");
    row.qualification = M5BuildRemoteQualificationClass::Beta;
    packet
}

/// Narrowed variant: the preview row is narrowed to Preview pending discovery-diff-card parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed(
) -> M5AdapterDiscoveryControlsPacket {
    let mut packet = seeded_m5_adapter_discovery_controls();
    packet.packet_id =
        "m5-adapter-confidence-chip-discovery-diff-card-controls:preview-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .expect("preview row present");
    row.qualification = M5BuildRemoteQualificationClass::Preview;
    packet
}
