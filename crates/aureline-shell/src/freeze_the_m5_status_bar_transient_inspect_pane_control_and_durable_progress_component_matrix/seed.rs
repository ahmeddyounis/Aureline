//! Canonical seed builders for the frozen M5 status-bar, transient-inspect,
//! pane-control, and durable-progress-component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical shell-primitives matrix.
pub const M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID: &str = "m5-shell-primitives:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every primitive must be able to show.
fn mandatory_labels() -> Vec<M5PrimitiveRequiredLabel> {
    M5PrimitiveRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a primitive carries.
fn labels_with(extra: &[M5PrimitiveRequiredLabel]) -> Vec<M5PrimitiveRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every primitive filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    primitive_family: M5ShellPrimitiveFamily,
    qualification: M5PrimitiveQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
) -> M5ShellPrimitiveRow {
    M5ShellPrimitiveRow {
        primitive_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        surface_families: M5ShellSurfaceFamily::ALL.to_vec(),
        required_labels: mandatory_labels(),
        status_item_classes: vec![],
        overflow_behaviors: vec![],
        source_freshness_labels: vec![],
        representation_classes: vec![],
        promotion_states: vec![],
        pane_resize_states: vec![],
        progress_states: vec![],
        accessibility_routes: M5AccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ShellPrimitiveDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SHELL_PRIMITIVES_SCHEMA_REF,
            M5_SHELL_PRIMITIVES_SHELL_ZONE_REF,
        ]),
        reflows_around_vanity_items: false,
        hides_source_or_freshness: false,
        keeps_critical_truth_hover_only: false,
        resizable_by_pointer_only: false,
    }
}

fn primitive_rows() -> Vec<M5ShellPrimitiveRow> {
    use M5OverflowBehavior as O;
    use M5PaneResizeState as PR;
    use M5PrimitiveQualificationClass as Q;
    use M5PrimitiveRequiredLabel as L;
    use M5ProgressState as PG;
    use M5PromotionState as P;
    use M5RepresentationClass as R;
    use M5ShellConsumerSurface as C;
    use M5ShellPrimitiveDowngradeTrigger as D;
    use M5ShellPrimitiveFamily as F;
    use M5ShellZoneSlot as Z;
    use M5SourceFreshnessLabel as S;
    use M5StatusItemClass as SI;

    let mut rows = Vec::new();

    // 1. Status-bar item.
    let mut row = base_row(
        F::StatusBarItem,
        Q::Stable,
        "Shell/status-bar owner",
        "A single status-bar item projecting one ambient truth (background work, connection target, deployment profile, sync freshness, problem count, mode, notification summary, or capacity) with its source and freshness; it never reflows around a spinner or a vanity item and stays keyboard-reachable when it overflows",
        Z::StatusBar,
        "evidence:m5-status-bar-parity:001",
    );
    row.status_item_classes = SI::ALL.to_vec();
    row.overflow_behaviors = O::ALL.to_vec();
    row.source_freshness_labels = vec![
        S::LiveCanonical,
        S::CachedSnapshot,
        S::StaleInvalidated,
        S::SampledApproximate,
        S::RefreshInFlight,
    ];
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::StatusBar,
        C::AttentionRouter,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::VanityItemReflow,
        D::SpinnerOnlyState,
        D::SourceFreshnessHidden,
        D::SevereStateDisplacedTruth,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Status overflow menu.
    let mut row = base_row(
        F::StatusOverflowMenu,
        Q::Stable,
        "Shell/status-bar owner",
        "The status-bar overflow menu that holds displaced or lower-priority status items behind one keyboard-reachable route; a severe state promotes ahead of vanity items, and every held item keeps its identity, state, and reopen path",
        Z::StatusBar,
        "evidence:m5-status-overflow-parity:001",
    );
    row.status_item_classes = SI::ALL.to_vec();
    row.overflow_behaviors = O::ALL.to_vec();
    row.source_freshness_labels = vec![S::LiveCanonical, S::CachedSnapshot, S::StaleInvalidated];
    row.required_labels = labels_with(&[L::Freshness, L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::StatusBar,
        C::AttentionRouter,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::VanityItemReflow,
        D::HoverOnlyCriticalTruth,
        D::SevereStateDisplacedTruth,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Tooltip.
    let mut row = base_row(
        F::Tooltip,
        Q::Stable,
        "Shell/transient-inspect owner",
        "A plain tooltip that shows a short label or shortcut hint on hover or focus; it never carries critical state that is not also reachable without hover, and truncated content keeps a keyboard-reachable reopen path",
        Z::TransientOverlay,
        "evidence:m5-tooltip-parity:001",
    );
    row.representation_classes = vec![R::PlainTooltip, R::TruncatedWithReopen];
    row.source_freshness_labels = vec![S::LiveCanonical];
    row.required_labels = mandatory_labels();
    row.consumer_surfaces = vec![C::ShellFrame, C::DocsHelp, C::SupportExport, C::ProductUi];
    row.downgrade_triggers = vec![
        D::HoverOnlyCriticalTruth,
        D::SpinnerOnlyState,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Hovercard.
    let mut row = base_row(
        F::Hovercard,
        Q::Stable,
        "Shell/transient-inspect owner",
        "A rich hovercard that shows attributed inspectable detail with a provenance strip naming the source, provider, and freshness of what it shows; a cached or stale value is labelled so it never reads as live canonical content",
        Z::TransientOverlay,
        "evidence:m5-hovercard-parity:001",
    );
    row.representation_classes = vec![R::RichHovercard, R::ProvenanceStrip, R::TruncatedWithReopen];
    row.source_freshness_labels = vec![
        S::LiveCanonical,
        S::CachedSnapshot,
        S::ProviderAttributed,
        S::StaleInvalidated,
    ];
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness]);
    row.consumer_surfaces = vec![C::ShellFrame, C::DocsHelp, C::SupportExport, C::ProductUi];
    row.downgrade_triggers = vec![
        D::HoverOnlyCriticalTruth,
        D::SourceFreshnessHidden,
        D::StalePreviewMistakenForLive,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Peek panel.
    let mut row = base_row(
        F::PeekPanel,
        Q::Stable,
        "Shell/transient-inspect owner",
        "An inline peek panel that previews a target's structure and can be pinned or promoted to a durable panel; pinning never drops its representation or provenance truth, and a stale preview is always labelled",
        Z::TransientOverlay,
        "evidence:m5-peek-panel-parity:001",
    );
    row.representation_classes = vec![
        R::StructuredPeek,
        R::PinnedPeek,
        R::ProvenanceStrip,
        R::TruncatedWithReopen,
    ];
    row.promotion_states = P::ALL.to_vec();
    row.source_freshness_labels = vec![
        S::LiveCanonical,
        S::CachedSnapshot,
        S::ProviderAttributed,
        S::StaleInvalidated,
        S::RefreshInFlight,
    ];
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness, L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StalePreviewMistakenForLive,
        D::PromotionDroppedTruth,
        D::SourceFreshnessHidden,
        D::HoverOnlyCriticalTruth,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Pinned-preview promotion.
    let mut row = base_row(
        F::PinnedPreviewPromotion,
        Q::Stable,
        "Shell/transient-inspect owner",
        "A pinned-preview promotion that turns a transient peek into a durable, docked, or detached panel; across every promotion and demotion the representation, provenance, and freshness truth is preserved and the reopen path is never lost",
        Z::RightInspector,
        "evidence:m5-pinned-preview-parity:001",
    );
    row.representation_classes = vec![
        R::PinnedPeek,
        R::ProvenanceStrip,
        R::StructuredPeek,
        R::TruncatedWithReopen,
    ];
    row.promotion_states = P::ALL.to_vec();
    row.source_freshness_labels = vec![
        S::LiveCanonical,
        S::CachedSnapshot,
        S::StaleInvalidated,
        S::ProviderAttributed,
        S::RefreshInFlight,
    ];
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness, L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::Windowing,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PromotionDroppedTruth,
        D::StalePreviewMistakenForLive,
        D::SourceFreshnessHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Splitter handle.
    let mut row = base_row(
        F::SplitterHandle,
        Q::Stable,
        "Shell/layout owner",
        "A splitter handle between resizable panes that is precise, keyboard-addressable, and serializable; it resets to a named default, clamps to a minimum width, and collapses to a rail while keeping a reopen path — never pointer-only",
        Z::MainWorkspace,
        "evidence:m5-splitter-parity:001",
    );
    row.pane_resize_states = PR::ALL.to_vec();
    row.required_labels = labels_with(&[L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::Windowing,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PointerOnlyResize,
        D::ResizeStateNotSerializable,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Pane-resize preset.
    let mut row = base_row(
        F::PaneResizePreset,
        Q::Stable,
        "Shell/layout owner",
        "A named pane-resize preset that serializes a layout ratio so a pane can snap to it, reset to it, or restore it across sessions and windows; the preset is keyboard-invokable and survives multi-window continuity",
        Z::MainWorkspace,
        "evidence:m5-pane-preset-parity:001",
    );
    row.pane_resize_states = vec![
        PR::Idle,
        PR::SnappedToPreset,
        PR::ResetToDefault,
        PR::ClampedToMinWidth,
        PR::CollapsedToRail,
    ];
    row.required_labels = labels_with(&[L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::Windowing,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ResizeStateNotSerializable,
        D::PointerOnlyResize,
        D::ProofStale,
    ];
    rows.push(row);

    // 9. Progress indicator.
    let mut row = base_row(
        F::ProgressIndicator,
        Q::Stable,
        "Shell/activity owner",
        "An ambient progress indicator (determinate or indeterminate) that never leaves critical progress visible only through a transient spinner; it attributes grouped batches, keeps a reopen path into durable history, and labels sampled or in-flight values",
        Z::StatusBar,
        "evidence:m5-progress-indicator-parity:001",
    );
    row.progress_states = PG::ALL.to_vec();
    row.source_freshness_labels = vec![S::LiveCanonical, S::RefreshInFlight, S::SampledApproximate];
    row.required_labels = labels_with(&[L::SourceProvider, L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::StatusBar,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SpinnerOnlyState,
        D::ProgressLostOnLookAway,
        D::GroupedProgressUnattributed,
        D::ProofStale,
    ];
    rows.push(row);

    // 10. Durable job row.
    let mut row = base_row(
        F::DurableJobRow,
        Q::Stable,
        "Shell/activity owner",
        "A durable job-row component in the activity / progress center that stays attributable and reopenable after the user looks away; succeeded, failed, and canceled rows keep their outcome and reason in history rather than vanishing with a spinner",
        Z::BottomPanel,
        "evidence:m5-durable-job-row-parity:001",
    );
    row.progress_states = PG::ALL.to_vec();
    row.source_freshness_labels = vec![
        S::LiveCanonical,
        S::ProviderAttributed,
        S::RefreshInFlight,
        S::CachedSnapshot,
    ];
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness, L::ReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ProgressLostOnLookAway,
        D::GroupedProgressUnattributed,
        D::SpinnerOnlyState,
        D::SourceFreshnessHidden,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ShellPrimitiveGovernanceReview {
    M5ShellPrimitiveGovernanceReview {
        ambient_instrumentation_overflow_safe: true,
        no_status_reflow_around_vanity_items: true,
        transient_inspect_preserves_source_and_freshness: true,
        pinned_preview_keeps_representation_truth: true,
        pane_resize_keyboard_addressable_and_serializable: true,
        progress_rows_durable_and_reopenable: true,
        no_critical_truth_hover_or_spinner_only: true,
        severe_state_displaces_vanity_not_truth: true,
        every_primitive_bound_to_shell_zone: true,
        every_primitive_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ShellPrimitiveConsumerProjection {
    M5ShellPrimitiveConsumerProjection {
        status_bar_consumes_matrix: true,
        hovercard_peek_consumes_representation_vocabulary: true,
        splitter_consumes_resize_state_vocabulary: true,
        activity_center_consumes_progress_vocabulary: true,
        support_export_reads_single_source: true,
        accessibility_bridge_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ShellPrimitiveProofFreshness {
    M5ShellPrimitiveProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ShellPrimitiveReleasePosture {
    M5ShellPrimitiveReleasePosture {
        release_packet_ref: "artifacts/release/m5-shell-primitives-proof/support_export.json"
            .to_owned(),
        shell_primitives_audit_ref: "artifacts/shell/m5-shell-primitives.md".to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SHELL_PRIMITIVES_SCHEMA_REF,
        M5_SHELL_PRIMITIVES_DOC_REF,
        M5_SHELL_PRIMITIVES_SHELL_ZONE_REF,
        M5_SHELL_PRIMITIVES_RESPONSIVE_CLASS_REF,
        M5_SHELL_PRIMITIVES_MULTI_WINDOW_PARITY_REF,
    ])
}

/// Builds the canonical frozen M5 shell-primitives matrix packet.
pub fn seeded_m5_shell_primitives_matrix() -> M5ShellPrimitivesMatrixPacket {
    M5ShellPrimitivesMatrixPacket::new(M5ShellPrimitivesMatrixPacketInput {
        packet_id: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 status-bar, transient-inspect, pane-control, and durable-progress-component matrix"
                .to_owned(),
        primitive_rows: primitive_rows(),
        vocabulary_set: M5ShellPrimitiveVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the pane-resize preset is held at Beta because a slice of
/// presets do not yet round-trip across multi-window restore; every primitive
/// stays visible.
pub fn seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed(
) -> M5ShellPrimitivesMatrixPacket {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.packet_id = "m5-shell-primitives:pane-resize-preset-beta:0001".to_owned();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::PaneResizePreset)
        .expect("pane-resize-preset row present");
    row.qualification = M5PrimitiveQualificationClass::Beta;
    packet
}

/// Narrowed variant: the pinned-preview promotion is narrowed to Preview pending
/// provenance-retention proof across all promotion transitions; every primitive
/// stays visible.
pub fn seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed(
) -> M5ShellPrimitivesMatrixPacket {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.packet_id = "m5-shell-primitives:pinned-preview-promotion-preview:0001".to_owned();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::PinnedPreviewPromotion)
        .expect("pinned-preview-promotion row present");
    row.qualification = M5PrimitiveQualificationClass::Preview;
    packet
}
