//! Canonical seed builders for the frozen M5 dynamic-surface accessibility matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical dynamic-surface accessibility matrix.
pub const M5_DYNAMIC_A11Y_MATRIX_PACKET_ID: &str = "m5-dynamic-surface-a11y-matrix:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

#[derive(Default)]
struct A11yTokenVecs {
    announcement_politeness: Vec<A11yAnnouncementPoliteness>,
    coalescing_strategies: Vec<A11yCoalescingStrategy>,
    fallback_durabilities: Vec<A11yFallbackDurability>,
    non_visual_fidelities: Vec<A11yNonVisualFidelity>,
    bridge_states: Vec<A11yBridgeState>,
    focus_return_dispositions: Vec<A11yFocusReturnDisposition>,
    semantic_role_classes: Vec<A11ySemanticRoleClass>,
}

impl A11yTokenVecs {
    fn merge(&mut self, other: A11yTokenVecs) {
        self.announcement_politeness.extend(other.announcement_politeness);
        self.coalescing_strategies.extend(other.coalescing_strategies);
        self.fallback_durabilities.extend(other.fallback_durabilities);
        self.non_visual_fidelities.extend(other.non_visual_fidelities);
        self.bridge_states.extend(other.bridge_states);
        self.focus_return_dispositions
            .extend(other.focus_return_dispositions);
        self.semantic_role_classes.extend(other.semantic_role_classes);
    }
}

fn tokens_for(vocab: M5DynamicSurfaceA11yStateVocabulary) -> A11yTokenVecs {
    use M5DynamicSurfaceA11yStateVocabulary as V;
    let mut vecs = A11yTokenVecs::default();
    match vocab {
        V::AnnouncementPoliteness => {
            vecs.announcement_politeness = A11yAnnouncementPoliteness::ALL.to_vec()
        }
        V::CoalescingStrategy => vecs.coalescing_strategies = A11yCoalescingStrategy::ALL.to_vec(),
        V::FallbackDurability => vecs.fallback_durabilities = A11yFallbackDurability::ALL.to_vec(),
        V::NonVisualFidelity => vecs.non_visual_fidelities = A11yNonVisualFidelity::ALL.to_vec(),
        V::BridgeState => vecs.bridge_states = A11yBridgeState::ALL.to_vec(),
        V::FocusReturnDisposition => {
            vecs.focus_return_dispositions = A11yFocusReturnDisposition::ALL.to_vec()
        }
        V::SemanticRoleClass => vecs.semantic_role_classes = A11ySemanticRoleClass::ALL.to_vec(),
    }
    vecs
}

#[allow(clippy::too_many_arguments)]
fn row(
    object_kind: M5DynamicSurfaceA11yObjectKind,
    qualification: M5DynamicSurfaceA11yQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    required_fields: &[&str],
    evidence_requirement: M5DynamicSurfaceA11yEvidenceRequirement,
    required_proof_packet_refs: &[&str],
    downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    rollback_posture: M5DynamicSurfaceA11yRollbackPosture,
    source_contract_refs: &[&str],
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5DynamicSurfaceA11yObjectRow {
    // Declared vocabularies come straight from the object kind so the row's token
    // vecs and declared list cannot disagree.
    let state_vocabularies = object_kind.required_state_vocabularies().to_vec();
    let mut tokens = A11yTokenVecs::default();
    for vocab in &state_vocabularies {
        tokens.merge(tokens_for(*vocab));
    }

    M5DynamicSurfaceA11yObjectRow {
        object_kind,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_fields: strings(required_fields),
        state_vocabularies,
        announcement_politeness: tokens.announcement_politeness,
        coalescing_strategies: tokens.coalescing_strategies,
        fallback_durabilities: tokens.fallback_durabilities,
        non_visual_fidelities: tokens.non_visual_fidelities,
        bridge_states: tokens.bridge_states,
        focus_return_dispositions: tokens.focus_return_dispositions,
        semantic_role_classes: tokens.semantic_role_classes,
        evidence_requirement,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        downgrade_triggers,
        rollback_posture,
        source_contract_refs: strings(source_contract_refs),
        consumer_surfaces,
    }
}

fn object_rows() -> Vec<M5DynamicSurfaceA11yObjectRow> {
    use M5DynamicSurfaceA11yConsumerSurface as S;
    use M5DynamicSurfaceA11yDowngradeTrigger as D;
    vec![
        row(
            M5DynamicSurfaceA11yObjectKind::AccessibilitySurfaceDescriptor,
            M5DynamicSurfaceA11yQualificationClass::Stable,
            "Accessibility owner",
            "Semantic-structure descriptor for one custom-rendered dynamic surface; exposes role, name, value, and state from the renderer's semantic model so the surface is never visual-only, and discloses its non-visual fidelity and bridge state instead of implying full parity",
            &[
                "surface_id",
                "surface_family",
                "semantic_role_class",
                "non_visual_fidelity",
                "bridge_state",
                "name_source",
            ],
            M5DynamicSurfaceA11yEvidenceRequirement::Required,
            &["evidence:accessibility-surface-descriptor-conformance:m5"],
            vec![
                D::NonVisualFidelityLost,
                D::PointerOrHoverDependence,
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::ProofStale,
            ],
            M5DynamicSurfaceA11yRollbackPosture::SemanticStructurePreserved,
            &[
                M5_DYNAMIC_A11Y_TREE_CONTRACT_REF,
                M5_DYNAMIC_A11Y_SHELL_BRIDGE_CONTRACT_REF,
            ],
            vec![
                S::Shell,
                S::Editor,
                S::Terminal,
                S::Notebook,
                S::DataGrid,
                S::Review,
                S::SupportExport,
            ],
        ),
        row(
            M5DynamicSurfaceA11yObjectKind::ScreenReaderLabelModel,
            M5DynamicSurfaceA11yQualificationClass::Stable,
            "Accessibility owner",
            "Screen-reader name / role / value / state label model for a surface; labels resolve from controlled message-id sources, never drift from the semantic role, and carry a durable fallback so a transient live region is never the only carrier of meaning",
            &[
                "label_model_id",
                "semantic_role_class",
                "name_source",
                "state_label_class",
                "fallback_durability",
                "bridge_state",
            ],
            M5DynamicSurfaceA11yEvidenceRequirement::Required,
            &["evidence:screen-reader-label-model-conformance:m5"],
            vec![
                D::LabelOrRoleDrift,
                D::NonVisualFidelityLost,
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::ProofStale,
            ],
            M5DynamicSurfaceA11yRollbackPosture::SemanticStructurePreserved,
            &[
                M5_DYNAMIC_A11Y_TREE_CONTRACT_REF,
                M5_DYNAMIC_A11Y_SCREEN_READER_CONTRACT_REF,
            ],
            vec![
                S::Shell,
                S::Editor,
                S::Terminal,
                S::Review,
                S::Help,
                S::SupportExport,
            ],
        ),
        row(
            M5DynamicSurfaceA11yObjectKind::LiveAnnouncementClass,
            M5DynamicSurfaceA11yQualificationClass::Stable,
            "Accessibility owner",
            "Live-announcement class governing politeness, coalescing, and durable fallback; assertive is reserved for safety-critical state, polite is queued, bursts coalesce instead of spamming, and every announced state change carries a durable fallback so meaning survives a missed utterance",
            &[
                "announcement_class_id",
                "announcement_politeness",
                "coalescing_strategy",
                "fallback_durability",
                "meaning_hash_ref",
                "coalesce_window_ms",
            ],
            M5DynamicSurfaceA11yEvidenceRequirement::Required,
            &[
                "evidence:live-announcement-class-conformance:m5",
                "evidence:live-region-coalescing-corpus:m5",
            ],
            vec![
                D::LiveRegionSpam,
                D::AnnouncementMeaningLost,
                D::BridgePartialOrStale,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5DynamicSurfaceA11yRollbackPosture::AnnouncementCoalescedNotSpammed,
            &[
                M5_DYNAMIC_A11Y_SCREEN_READER_CONTRACT_REF,
                M5_DYNAMIC_A11Y_OPERATIONAL_PARITY_CONTRACT_REF,
            ],
            vec![
                S::Shell,
                S::Editor,
                S::Terminal,
                S::Notebook,
                S::Review,
                S::Presentation,
                S::AiSurfaces,
                S::SupportExport,
            ],
        ),
        row(
            M5DynamicSurfaceA11yObjectKind::FocusReturnContract,
            M5DynamicSurfaceA11yQualificationClass::Stable,
            "Accessibility owner",
            "Focus-return contract for asynchronous updates and overlay teardown; focus returns to a real owner — exact, nearest safe ancestor, current batch/detail owner, or an announced placeholder — and never teleports to an unrelated surface or vanishes, with a durable re-entry fallback when the prior owner is destroyed",
            &[
                "focus_contract_id",
                "focus_return_disposition",
                "return_target_role",
                "fallback_durability",
                "overlay_kind",
            ],
            M5DynamicSurfaceA11yEvidenceRequirement::Required,
            &["evidence:focus-return-contract-conformance:m5"],
            vec![
                D::FocusTeleported,
                D::FocusLost,
                D::BridgePartialOrStale,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5DynamicSurfaceA11yRollbackPosture::FocusAnchorPreserved,
            &[
                M5_DYNAMIC_A11Y_FOCUS_CONTRACT_REF,
                M5_DYNAMIC_A11Y_OPERATIONAL_PARITY_CONTRACT_REF,
            ],
            vec![
                S::Shell,
                S::Editor,
                S::Terminal,
                S::Notebook,
                S::DataGrid,
                S::Review,
                S::Presentation,
                S::SupportExport,
            ],
        ),
        row(
            M5DynamicSurfaceA11yObjectKind::DenseSurfaceNonVisualSummary,
            M5DynamicSurfaceA11yQualificationClass::Stable,
            "Accessibility owner",
            "Dense-surface non-visual summary for lists, trees, grids, and logs; exposes position, selection scope, hidden-selected and blocked counts, and virtualization truth as a coalesced non-visual summary so a screen-reader user knows the real scope before acting, never just the visible rows",
            &[
                "summary_id",
                "semantic_role_class",
                "non_visual_fidelity",
                "coalescing_strategy",
                "count_scope_terms",
                "virtualization_state",
            ],
            M5DynamicSurfaceA11yEvidenceRequirement::Required,
            &["evidence:dense-surface-non-visual-summary-conformance:m5"],
            vec![
                D::NonVisualFidelityLost,
                D::LiveRegionSpam,
                D::PointerOrHoverDependence,
                D::BridgePartialOrStale,
                D::ProofStale,
            ],
            M5DynamicSurfaceA11yRollbackPosture::SemanticStructurePreserved,
            &[
                M5_DYNAMIC_A11Y_COLLECTION_CONTRACT_REF,
                M5_DYNAMIC_A11Y_TREE_CONTRACT_REF,
            ],
            vec![
                S::DataGrid,
                S::Review,
                S::Notebook,
                S::Terminal,
                S::SupportExport,
            ],
        ),
        row(
            M5DynamicSurfaceA11yObjectKind::BridgeDiagnosticsPacket,
            M5DynamicSurfaceA11yQualificationClass::Beta,
            "Accessibility platform owner",
            "OS accessibility-bridge diagnostics packet that names the active platform bridge and its connection state; when the bridge is partial, stale, or unavailable the packet discloses the degradation and the affected surfaces auto-narrow rather than claiming silent screen-reader completeness",
            &[
                "diagnostics_id",
                "bridge_kind",
                "bridge_state",
                "non_visual_fidelity",
                "degradation_reason_class",
                "affected_surface_refs",
            ],
            M5DynamicSurfaceA11yEvidenceRequirement::Required,
            &[
                "evidence:bridge-diagnostics-packet-conformance:m5",
                "evidence:platform-bridge-coverage-matrix:m5",
            ],
            vec![
                D::BridgeUnavailable,
                D::BridgePartialOrStale,
                D::NonVisualFidelityLost,
                D::UpstreamDependencyNarrowed,
                D::ProofStale,
            ],
            M5DynamicSurfaceA11yRollbackPosture::BridgeDegradationDisclosed,
            &[
                M5_DYNAMIC_A11Y_SHELL_BRIDGE_CONTRACT_REF,
                M5_DYNAMIC_A11Y_TREE_CONTRACT_REF,
            ],
            vec![
                S::Shell,
                S::Help,
                S::SupportExport,
                S::Review,
                S::Presentation,
            ],
        ),
    ]
}

fn conformance_review() -> M5DynamicSurfaceA11yConformanceReview {
    M5DynamicSurfaceA11yConformanceReview {
        custom_surfaces_expose_semantic_structure: true,
        focus_never_teleports_or_vanishes_on_async_update: true,
        live_regions_coalesce_rather_than_spam: true,
        dynamic_state_changes_announce_meaning_not_repaint_noise: true,
        no_visual_only_state_or_pointer_hover_dependence: true,
        dense_surfaces_expose_non_visual_summaries: true,
        durable_fallbacks_present_for_blocking_states: true,
        bridge_degradation_disclosed_not_hidden: true,
        one_bridge_aware_contract_not_per_surface_adhoc: true,
        claimed_rows_auto_narrow_when_bridge_or_proof_stale: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5DynamicSurfaceA11yConsumerProjection {
    M5DynamicSurfaceA11yConsumerProjection {
        shell_consumes_object_model: true,
        editor_exposes_semantic_structure: true,
        terminal_announces_via_live_region: true,
        notebook_returns_focus_on_async_update: true,
        data_grid_exposes_non_visual_summary: true,
        review_exposes_semantic_structure: true,
        help_documents_bridge_diagnostics: true,
        presentation_announces_meaning_not_repaint: true,
        support_export_shows_object_model: true,
        unqualified_surfaces_labeled_when_uncovered: true,
    }
}

fn proof_freshness() -> M5DynamicSurfaceA11yProofFreshness {
    M5DynamicSurfaceA11yProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DynamicSurfaceA11yReleasePosture {
    M5DynamicSurfaceA11yReleasePosture {
        release_packet_ref: "evidence:dynamic-surface-a11y-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:dynamic-surface-a11y-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DYNAMIC_A11Y_MATRIX_SCHEMA_REF,
        M5_DYNAMIC_A11Y_MATRIX_DOC_REF,
        M5_DYNAMIC_A11Y_SCREEN_READER_CONTRACT_REF,
        M5_DYNAMIC_A11Y_TREE_CONTRACT_REF,
        M5_DYNAMIC_A11Y_FOCUS_CONTRACT_REF,
        M5_DYNAMIC_A11Y_COLLECTION_CONTRACT_REF,
        M5_DYNAMIC_A11Y_SHELL_BRIDGE_CONTRACT_REF,
        M5_DYNAMIC_A11Y_OPERATIONAL_PARITY_CONTRACT_REF,
    ])
}

fn base_input() -> M5DynamicSurfaceA11yMatrixPacketInput {
    M5DynamicSurfaceA11yMatrixPacketInput {
        packet_id: M5_DYNAMIC_A11Y_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 Accessibility-Bridge, Live-Announcement, Focus-Return, and Non-Visual Dynamic-Surface Matrix"
                .to_owned(),
        object_rows: object_rows(),
        vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable M5 dynamic-surface accessibility matrix packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_dynamic_surface_a11y_matrix() -> M5DynamicSurfaceA11yMatrixPacket {
    M5DynamicSurfaceA11yMatrixPacket::new(base_input())
}

/// Builds a narrowed variant where the bridge-diagnostics packet is held after the
/// OS accessibility bridge goes unavailable, proving downgrade narrows the claim
/// rather than hiding the object.
pub fn seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable() -> M5DynamicSurfaceA11yMatrixPacket
{
    let mut input = base_input();
    input.packet_id = "m5-dynamic-surface-a11y-matrix:bridge-unavailable:0001".to_owned();
    for row in &mut input.object_rows {
        if row.object_kind == M5DynamicSurfaceA11yObjectKind::BridgeDiagnosticsPacket {
            row.qualification = M5DynamicSurfaceA11yQualificationClass::Held;
            // A held object no longer carries a public claim, so proof becomes
            // recommended rather than required; the object stays visible.
            row.evidence_requirement = M5DynamicSurfaceA11yEvidenceRequirement::Recommended;
        }
    }
    M5DynamicSurfaceA11yMatrixPacket::new(input)
}

/// Builds a narrowed variant where the dense-surface non-visual summary is pulled to
/// preview after a non-visual-fidelity finding, proving auto-narrowing keeps the
/// object visible.
pub fn seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed(
) -> M5DynamicSurfaceA11yMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-dynamic-surface-a11y-matrix:dense-summary-narrowed:0001".to_owned();
    for row in &mut input.object_rows {
        if row.object_kind == M5DynamicSurfaceA11yObjectKind::DenseSurfaceNonVisualSummary {
            row.qualification = M5DynamicSurfaceA11yQualificationClass::Preview;
        }
    }
    M5DynamicSurfaceA11yMatrixPacket::new(input)
}
