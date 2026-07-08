//! Canonical seed builders for the M5 selection-or-lock-state-contract primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical selection-or-lock-state-contract primitive packet.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_PACKET_ID: &str =
    "m5-selection-or-lock-state-contract-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked selection-or-lock resolution case from a full item state.
#[allow(clippy::too_many_arguments)]
fn state_case(
    item_kind: M5SelectionOrLockItemKind,
    selection_or_lock_state: M5SharedComponentStateClass,
    lock_owner: M5LockOwnerClass,
    state_cause: M5StateCauseClass,
    recovery_available: bool,
    inspectable: bool,
    high_contrast_active: bool,
    item_identity_ref: &str,
    state_style_ref: &str,
    disclosure_ref: &str,
) -> M5SelectionOrLockResolutionCase {
    M5SelectionOrLockResolutionCase::resolved(M5SelectionOrLockResolutionInput {
        item_kind,
        selection_or_lock_state,
        lock_owner,
        state_cause,
        recovery_available,
        inspectable,
        high_contrast_active,
        item_identity_ref: item_identity_ref.to_owned(),
        state_style_ref: state_style_ref.to_owned(),
        disclosure_ref: disclosure_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full selection-or-lock anatomy, states,
/// presentations, non-color cues, required disclosures, lock owner classes, state cause classes,
/// export fields, labels, and accessibility parity every item carries.
fn base_row(
    item_kind: M5SelectionOrLockItemKind,
    qualification: M5ComponentStateQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    state_examples: Vec<M5SelectionOrLockResolutionCase>,
) -> M5SelectionOrLockItemRow {
    M5SelectionOrLockItemRow {
        item_kind,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComponentStateSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComponentStateDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5SelectionOrLockAnatomyPart::ALL.to_vec(),
        selection_or_lock_states: selection_or_lock_states(),
        presentations: M5SelectionOrLockPresentation::ALL.to_vec(),
        non_color_cues: M5SelectionOrLockCue::ALL.to_vec(),
        required_disclosures: M5StateDisclosureTrigger::ALL.to_vec(),
        lock_owner_classes: M5LockOwnerClass::ALL.to_vec(),
        state_cause_classes: M5StateCauseClass::ALL.to_vec(),
        export_fields: M5SelectionOrLockExportField::ALL.to_vec(),
        accessibility_routes: M5ComponentStateAccessibilityRoute::ALL.to_vec(),
        required_labels: M5ComponentStateRequiredLabel::ALL.to_vec(),
        consumer_surfaces: M5ComponentStateConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ComponentStateDowngradeTrigger::LockOwnerMasked,
            M5ComponentStateDowngradeTrigger::CurrentSelectedCollapsed,
            M5ComponentStateDowngradeTrigger::ReadOnlyInspectabilityLost,
            M5ComponentStateDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF,
            M5_SELECTION_OR_LOCK_STATE_CONTRACT_STATE_RECOVERY_REF,
            M5_SELECTION_OR_LOCK_STATE_CONTRACT_OPERATIONAL_SURFACE_STATE_REF,
        ]),
        state_examples,
        collapses_selected_and_current: false,
        hides_lock_behind_disabled: false,
        drops_read_only_inspectability: false,
        invents_private_state_name: false,
    }
}

fn rows() -> Vec<M5SelectionOrLockItemRow> {
    use M5ComponentStateQualificationClass as Qual;
    use M5LockOwnerClass as Owner;
    use M5SelectionOrLockItemKind as Item;
    use M5SharedComponentStateClass as State;
    use M5StateCauseClass as Cause;

    vec![
        // 1. Tab — the durable-selection treatment and the current-location treatment, so a merely
        //    selected tab never reads as the actively current one. Neither is explainable, so no
        //    owner/cause/recovery detail is required; a selection marker and a distinct
        //    current-location indicator keep them apart without color.
        base_row(
            Item::Tab,
            Qual::Stable,
            "Tab strip owner",
            "The tab renders the shared selection-or-lock-state contract so a durably selected tab and the actively current tab stay distinct — the selection is carried by a selection marker and the current tab by a distinct current-location indicator, never by a color-only swap that would collapse the two",
            "evidence:m5-selection-or-lock-tab:001",
            vec![
                state_case(
                    Item::Tab,
                    State::Selected,
                    Owner::NoLock,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "item:editor-tabs.readme",
                    "token:state.tab.selected",
                    "",
                ),
                state_case(
                    Item::Tab,
                    State::Current,
                    Owner::NoLock,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "item:editor-tabs.active-file",
                    "token:state.tab.current",
                    "",
                ),
            ],
        ),
        // 2. Tree item — the durable-selection treatment and the explicit policy-lock treatment, so
        //    a selected tree node is distinct and a policy-locked node names its owner and recovery
        //    rather than reading as a plain disabled node.
        base_row(
            Item::TreeItem,
            Qual::Stable,
            "Navigation tree owner",
            "The tree item renders the shared selection-or-lock-state contract so a selected node stays distinct from the current one and a policy-locked node surfaces its policy owner, its cause, and its recovery path — a lock glyph with its owner, never a silent disabled dimming that would hide why the node cannot be changed",
            "evidence:m5-selection-or-lock-tree-item:001",
            vec![
                state_case(
                    Item::TreeItem,
                    State::Selected,
                    Owner::NoLock,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "item:explorer-tree.src-folder",
                    "token:state.tree_item.selected",
                    "",
                ),
                state_case(
                    Item::TreeItem,
                    State::Locked,
                    Owner::PolicyLock,
                    Cause::PolicyCause,
                    true,
                    false,
                    false,
                    "item:explorer-tree.protected-config",
                    "token:state.tree_item.locked",
                    "policy:workspace.admin-lock",
                ),
            ],
        ),
        // 3. Dense list row — the current-location treatment and the silently-unavailable disabled
        //    treatment, so a current row is distinct and a disabled row names its cause and recovery
        //    without carrying a lock owner it would then be masking.
        base_row(
            Item::ListRow,
            Qual::Stable,
            "Dense list owner",
            "The dense list row renders the shared selection-or-lock-state contract so the current row is distinct and a disabled row names why it is unavailable and how to recover — a dimmed treatment with an explicit reason, never a bare color change and never a hidden lock owner that should instead be modeled as locked",
            "evidence:m5-selection-or-lock-list-row:001",
            vec![
                state_case(
                    Item::ListRow,
                    State::Current,
                    Owner::NoLock,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "item:results-list.current-match",
                    "token:state.list_row.current",
                    "",
                ),
                state_case(
                    Item::ListRow,
                    State::Disabled,
                    Owner::NoLock,
                    Cause::PreconditionCause,
                    true,
                    false,
                    false,
                    "item:results-list.unmet-prerequisite-row",
                    "token:state.list_row.disabled",
                    "reason:prerequisite-unmet.select-target-first",
                ),
            ],
        ),
        // 4. Grid / table row — the inspectable read-only treatment and the trust-lock treatment,
        //    so a source-locked derived cell stays inspectable and a trust-blocked cell names its
        //    trust owner and review path, in high-contrast, never collapsing into disabled.
        base_row(
            Item::TableRow,
            Qual::Stable,
            "Grid / table owner",
            "The grid/table row renders the shared selection-or-lock-state contract so a source-of-truth derived cell stays read-only-inspectable and a trust-blocked cell surfaces its trust owner, its cause, and its review path — legible in high-contrast, a read-only glyph or a lock glyph with its owner, never a disabled treatment that would drop inspectability or hide the trust lock",
            "evidence:m5-selection-or-lock-table-row:001",
            vec![
                state_case(
                    Item::TableRow,
                    State::ReadOnly,
                    Owner::SourceLock,
                    Cause::PreconditionCause,
                    true,
                    true,
                    true,
                    "item:result-grid.derived-column",
                    "token:state.table_row.read_only",
                    "readonly:generated-column.derived-from-query",
                ),
                state_case(
                    Item::TableRow,
                    State::Locked,
                    Owner::TrustLock,
                    Cause::PolicyCause,
                    true,
                    false,
                    true,
                    "item:result-grid.unverified-source-cell",
                    "token:state.table_row.locked",
                    "trust:unverified-source.review-required",
                ),
            ],
        ),
        // 5. Badge — the permission-lock treatment and the silently-unavailable disabled treatment,
        //    so a permission-locked badge names its permission owner and request path and a disabled
        //    badge names its connectivity cause and reconnect path.
        base_row(
            Item::Badge,
            Qual::Stable,
            "Status badge owner",
            "The badge renders the shared selection-or-lock-state contract so a permission-locked badge surfaces its permission owner and the request-access path while a disabled badge names its connectivity cause and the reconnect path — a lock glyph with its owner or a dimmed treatment with its reason, never a color-only badge that hides which is which",
            "evidence:m5-selection-or-lock-badge:001",
            vec![
                state_case(
                    Item::Badge,
                    State::Locked,
                    Owner::PermissionLock,
                    Cause::PermissionCause,
                    true,
                    false,
                    false,
                    "item:status-badges.write-scope-badge",
                    "token:state.badge.locked",
                    "permission:write-scope.request-access",
                ),
                state_case(
                    Item::Badge,
                    State::Disabled,
                    Owner::NoLock,
                    Cause::ConnectivityCause,
                    true,
                    false,
                    false,
                    "item:status-badges.offline-badge",
                    "token:state.badge.disabled",
                    "reason:offline.reconnect-to-enable",
                ),
            ],
        ),
        // 6. Settings row — the inspectable read-only treatment and the ownership-lock treatment, so
        //    a managed read-only setting stays inspectable and an ownership-locked setting names the
        //    workspace owner and the escalation path rather than reading as disabled.
        base_row(
            Item::SettingsRow,
            Qual::Stable,
            "Settings sheet owner",
            "The settings row renders the shared selection-or-lock-state contract so a managed read-only setting stays inspectable and an ownership-locked setting surfaces the workspace owner, its cause, and the escalation path — a read-only glyph or a lock glyph with its owner, never a disabled row that would hide who owns the setting or whether it can still be inspected",
            "evidence:m5-selection-or-lock-settings-row:001",
            vec![
                state_case(
                    Item::SettingsRow,
                    State::ReadOnly,
                    Owner::NoLock,
                    Cause::PreconditionCause,
                    true,
                    true,
                    false,
                    "item:settings.managed-telemetry-toggle",
                    "token:state.settings_row.read_only",
                    "readonly:managed-setting.inspect-only",
                ),
                state_case(
                    Item::SettingsRow,
                    State::Locked,
                    Owner::OwnershipLock,
                    Cause::PolicyCause,
                    true,
                    false,
                    false,
                    "item:settings.workspace-name-field",
                    "token:state.settings_row.locked",
                    "ownership:workspace-owner.only-owner-can-change",
                ),
            ],
        ),
        // 7. Inspector entry — the source-locked read-only treatment and the silently-unavailable
        //    disabled treatment, so a derived inspector property stays inspectable and a
        //    no-selection inspector row names its cause and recovery path.
        base_row(
            Item::InspectorEntry,
            Qual::Stable,
            "Inspector owner",
            "The inspector entry renders the shared selection-or-lock-state contract so a source-of-truth computed property stays read-only-inspectable and a no-selection entry names its cause and the recovery path — a read-only glyph or a dimmed treatment with its reason, never a disabled entry that would drop inspectability or leave the user guessing why the field is inert",
            "evidence:m5-selection-or-lock-inspector-entry:001",
            vec![
                state_case(
                    Item::InspectorEntry,
                    State::ReadOnly,
                    Owner::SourceLock,
                    Cause::PreconditionCause,
                    true,
                    true,
                    false,
                    "item:inspector.computed-bounds",
                    "token:state.inspector_entry.read_only",
                    "readonly:computed-property.derived",
                ),
                state_case(
                    Item::InspectorEntry,
                    State::Disabled,
                    Owner::NoLock,
                    Cause::PreconditionCause,
                    true,
                    false,
                    false,
                    "item:inspector.no-selection-row",
                    "token:state.inspector_entry.disabled",
                    "reason:no-selection.select-a-node-first",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5SelectionOrLockGovernanceReview {
    M5SelectionOrLockGovernanceReview {
        items_distinguish_selected_current_read_only_disabled_locked: true,
        selected_and_current_never_collapse: true,
        read_only_never_collapses_into_disabled: true,
        locked_never_hidden_behind_disabled: true,
        state_meaning_never_color_only: true,
        owner_source_recovery_surfaced_when_explainable: true,
        states_keyboard_and_screen_reader_explainable: true,
        states_driven_by_shared_contract_and_tokens: true,
        no_one_off_per_surface_styling: true,
        states_stable_across_deployment_lines: true,
        states_stable_across_consumer_surfaces: true,
        every_item_declares_accessibility_route: true,
        support_export_reconstructs_state_truth: true,
        later_rows_cannot_invent_parallel_state_vocabulary: true,
    }
}

fn consumer_projection() -> M5SelectionOrLockConsumerProjection {
    M5SelectionOrLockConsumerProjection {
        items_consume_state_vocabulary: true,
        presentation_reads_single_source: true,
        disclosure_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5SelectionOrLockProofFreshness {
    M5SelectionOrLockProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SelectionOrLockReleasePosture {
    M5SelectionOrLockReleasePosture {
        release_packet_ref: M5_SELECTION_OR_LOCK_STATE_CONTRACT_ARTIFACT_REF.to_owned(),
        selection_or_lock_state_audit_ref: M5_SELECTION_OR_LOCK_STATE_CONTRACT_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_DOC_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_COMPONENT_MATRIX_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_FOCUS_SELECTION_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_STATE_RECOVERY_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_OPERATIONAL_SURFACE_STATE_REF,
    ])
}

/// Builds the canonical M5 selection-or-lock-state-contract packet.
pub fn seeded_m5_selection_or_lock_state_contract_packet() -> M5SelectionOrLockStateContractPacket {
    M5SelectionOrLockStateContractPacket::new(M5SelectionOrLockStateContractPacketInput {
        packet_id: M5_SELECTION_OR_LOCK_STATE_CONTRACT_PACKET_ID.to_owned(),
        matrix_label:
            "M5 selection-or-lock-state contract primitive: item kind, selection-or-lock state (selected/current/disabled/read-only/locked), derived presentation posture, required non-color cues, required disclosures (state cause / owner / block reason / recovery action), lock owner class, and the selected-vs-current / read-only-vs-disabled / locked-vs-disabled distinctness and owner-reason-recovery guarantees"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5SelectionOrLockVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the badge item is held at Beta because a slice of badge surfaces does not yet
/// name the lock owner on every profile; every item stays visible.
pub fn seeded_m5_selection_or_lock_state_contract_badge_beta_narrowed(
) -> M5SelectionOrLockStateContractPacket {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.packet_id = "m5-selection-or-lock-state-contract-primitive:badge-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.item_kind == M5SelectionOrLockItemKind::Badge)
        .expect("badge row present");
    row.qualification = M5ComponentStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the inspector entry item is narrowed to Preview pending read-only
/// inspectability parity proof across every density; every item stays visible.
pub fn seeded_m5_selection_or_lock_state_contract_inspector_entry_preview_narrowed(
) -> M5SelectionOrLockStateContractPacket {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.packet_id =
        "m5-selection-or-lock-state-contract-primitive:inspector-entry-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.item_kind == M5SelectionOrLockItemKind::InspectorEntry)
        .expect("inspector-entry row present");
    row.qualification = M5ComponentStateQualificationClass::Preview;
    packet
}
