//! Canonical seed builders for the M5 toolchain-pin / switch-review primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical toolchain-pin / switch-review primitive packet.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_PACKET_ID: &str =
    "m5-toolchain-pin-switch-review-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one candidate layer.
fn layer(
    scope: M5PinScope,
    source: M5ToolchainSourceClass,
    selection_repr: &str,
    present: bool,
) -> M5PinCandidateLayer {
    M5PinCandidateLayer {
        scope,
        source,
        selection_repr: selection_repr.to_owned(),
        present,
    }
}

/// Builds a worked resolution case from a full toolchain-selection state.
fn case(
    target_title: &str,
    target_kind: M5ToolchainTargetKind,
    candidate_layers: Vec<M5PinCandidateLayer>,
    selection_health: M5SelectionHealth,
    switch_request: Option<M5SwitchRequest>,
) -> M5ToolchainSelectionResolutionCase {
    M5ToolchainSelectionResolutionCase::resolved(M5ToolchainSelectionResolutionInput {
        target_title: target_title.to_owned(),
        target_kind,
        candidate_layers,
        selection_health,
        switch_request,
    })
}

/// Builds a switch request.
fn switch(
    to_scope: M5PinScope,
    to_source: M5ToolchainSourceClass,
    to_selection_repr: &str,
    requires_restart: bool,
    requires_reconnect: bool,
    newly_blocked_actions: &[&str],
    safe_local_only_fallback: bool,
) -> M5SwitchRequest {
    M5SwitchRequest {
        to_scope,
        to_source,
        to_selection_repr: to_selection_repr.to_owned(),
        requires_restart,
        requires_reconnect,
        newly_blocked_actions: strings(newly_blocked_actions),
        safe_local_only_fallback,
    }
}

/// A base row with the shared fields filled in and the full pin-row-part,
/// inspector-part, switch-card-part, target-kind, pin-state, scope, health, action,
/// export-field, and accessibility parity every surface carries.
fn base_row(
    selector_surface: M5EnvironmentSelectorSurface,
    qualification: M5RuntimeBoundaryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
    example_resolutions: Vec<M5ToolchainSelectionResolutionCase>,
) -> M5EnvironmentSelectorRow {
    M5EnvironmentSelectorRow {
        selector_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        pin_row_parts: M5ToolchainPinRowPart::ALL.to_vec(),
        inspector_parts: M5PrecedenceInspectorPart::ALL.to_vec(),
        switch_card_parts: M5SwitchReviewCardPart::ALL.to_vec(),
        target_kinds: M5ToolchainTargetKind::ALL.to_vec(),
        pin_states: M5ToolchainPinState::ALL.to_vec(),
        pin_scopes: M5PinScope::ALL.to_vec(),
        selection_health_states: M5SelectionHealth::ALL.to_vec(),
        pin_actions: M5PinAction::ALL.to_vec(),
        export_fields: M5ToolchainSelectionExportField::ALL.to_vec(),
        accessibility_routes: M5RuntimeBoundaryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5RuntimeBoundaryDowngradeTrigger::ToolchainPinConflictHidden,
            M5RuntimeBoundaryDowngradeTrigger::RuntimeSourceUnexplained,
            M5RuntimeBoundaryDowngradeTrigger::RepairBlastRadiusUnderstated,
            M5RuntimeBoundaryDowngradeTrigger::ReversibilityOverstated,
            M5RuntimeBoundaryDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5RuntimeBoundaryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TOOLCHAIN_PIN_ROW_SCHEMA_REF,
            M5_PRECEDENCE_INSPECTOR_SCHEMA_REF,
            M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRECEDENCE_REF,
        ]),
        example_resolutions,
        silently_shadows_durable_pin: false,
        shows_degraded_as_resolved: false,
        invents_private_selection_grammar: false,
        hides_switch_blast_radius: false,
    }
}

fn selector_rows() -> Vec<M5EnvironmentSelectorRow> {
    use M5PinScope as Scope;
    use M5SelectionHealth as Health;
    use M5ToolchainSourceClass as Source;
    use M5ToolchainTargetKind as Kind;

    let mut rows = Vec::new();

    // 1. Status-bar selector — a cleanly project-pinned interpreter (PinnedResolved),
    //    and a policy override shadowing that pin (PinOverridden, shadow disclosed).
    rows.push(base_row(
        M5EnvironmentSelectorSurface::StatusBarSelector,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Status-bar selector owner",
        "The status-bar selector renders the shared pin row, precedence inspector, and switch-review card so a project-pinned interpreter reads as pinned-resolved at project scope, while a managed policy override reads as pin-overridden with the shadowed project pin still inspectable and a clear-override action attached",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-toolchain-status:001",
        vec![
            case(
                "status-py-resolved",
                Kind::Interpreter,
                vec![layer(Scope::ProjectScope, Source::PinFile, "py-3.12", true)],
                Health::Healthy,
                None,
            ),
            case(
                "status-py-policy-override",
                Kind::Interpreter,
                vec![
                    layer(Scope::PolicyScope, Source::WorkspaceSetting, "py-3.11", true),
                    layer(Scope::ProjectScope, Source::PinFile, "py-3.12", true),
                ],
                Health::Healthy,
                None,
            ),
        ],
    ));

    // 2. Command-palette switcher — a session override shadowing a user pin with a
    //    reviewed switch (PinOverridden), and an unpinned SDK default (Unpinned).
    rows.push(base_row(
        M5EnvironmentSelectorSurface::CommandPaletteSwitcher,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Command-palette switcher owner",
        "The command-palette switcher renders the shared components so a session override reads as pin-overridden with the shadowed user pin and a toolchain-scoped, fully reversible switch previewed before it is applied, while an unset SDK reads as unpinned on a global default",
        M5ShellZoneSlot::TransientOverlay,
        "evidence:m5-toolchain-palette:001",
        vec![
            case(
                "palette-sdk-session-override",
                Kind::Sdk,
                vec![
                    layer(Scope::SessionScope, Source::SessionOverride, "dotnet-8", true),
                    layer(Scope::UserScope, Source::WorkspaceSetting, "dotnet-7", true),
                ],
                Health::Healthy,
                Some(switch(
                    Scope::WorkspaceScope,
                    Source::WorkspaceSetting,
                    "dotnet-8",
                    true,
                    false,
                    &["run_task"],
                    true,
                )),
            ),
            case(
                "palette-sdk-unpinned",
                Kind::Sdk,
                vec![layer(
                    Scope::GlobalDefaultScope,
                    Source::SystemInstalled,
                    "dotnet-lts",
                    true,
                )],
                Health::Healthy,
                None,
            ),
        ],
    ));

    // 3. Settings toolchain row — a workspace setting shadowing a user pin
    //    (PinConflict, the AC1 example), and a cleanly user-pinned shell (PinnedResolved).
    rows.push(base_row(
        M5EnvironmentSelectorSurface::SettingsToolchainRow,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Settings toolchain row owner",
        "The settings toolchain row renders the shared components so a workspace setting that shadows a differing user pin reads as pin-conflict with the shadowed user layer disclosed and a revert action attached — never a silent shadow — while a lone user pin reads as pinned-resolved at user scope",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-toolchain-settings:001",
        vec![
            case(
                "settings-shell-workspace-shadows-user",
                Kind::Shell,
                vec![
                    layer(Scope::WorkspaceScope, Source::WorkspaceSetting, "bash-5.2", true),
                    layer(Scope::UserScope, Source::VersionManager, "zsh-5.9", true),
                ],
                Health::Healthy,
                None,
            ),
            case(
                "settings-shell-user-resolved",
                Kind::Shell,
                vec![layer(Scope::UserScope, Source::VersionManager, "zsh-5.9", true)],
                Health::Healthy,
                None,
            ),
        ],
    ));

    // 4. Interpreter picker — a project pin whose interpreter is missing
    //    (PinnedMissingFallback, repair attached), and a container-provided pin.
    rows.push(base_row(
        M5EnvironmentSelectorSurface::InterpreterPicker,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Interpreter picker owner",
        "The interpreter picker renders the shared components so a project pin whose interpreter is missing reads as pinned-missing-fallback with the degraded health explicit and a repair action attached, while a container-image interpreter reads as pinned-resolved at workspace scope",
        M5ShellZoneSlot::TransientOverlay,
        "evidence:m5-toolchain-interpreter:001",
        vec![
            case(
                "picker-py-missing",
                Kind::Interpreter,
                vec![layer(Scope::ProjectScope, Source::PinFile, "py-3.12", true)],
                Health::MissingUnavailable,
                None,
            ),
            case(
                "picker-py-container",
                Kind::Interpreter,
                vec![layer(
                    Scope::WorkspaceScope,
                    Source::ContainerImage,
                    "py-3.11",
                    true,
                )],
                Health::Healthy,
                None,
            ),
        ],
    ));

    // 5. SDK selector — a stale project SDK (DegradedStale, repair attached), and a
    //    session override with a mismatched version and a host-scoped switch.
    rows.push(base_row(
        M5EnvironmentSelectorSurface::SdkSelector,
        M5RuntimeBoundaryQualificationClass::Stable,
        "SDK selector owner",
        "The SDK selector renders the shared components so a stale project SDK keeps an explicit repair action rather than reading as cleanly resolved, while a session override whose version mismatches the pin previews a host-environment-scoped switch with its reversibility before it is applied",
        M5ShellZoneSlot::RightInspector,
        "evidence:m5-toolchain-sdk:001",
        vec![
            case(
                "sdk-project-stale",
                Kind::Sdk,
                vec![layer(Scope::ProjectScope, Source::VersionManager, "jdk-21", true)],
                Health::DegradedStale,
                None,
            ),
            case(
                "sdk-session-mismatch",
                Kind::Sdk,
                vec![
                    layer(Scope::SessionScope, Source::SessionOverride, "jdk-17", true),
                    layer(Scope::ProjectScope, Source::PinFile, "jdk-21", true),
                ],
                Health::MismatchedVersion,
                Some(switch(
                    Scope::HostScope,
                    Source::SystemInstalled,
                    "jdk-21",
                    false,
                    false,
                    &[],
                    false,
                )),
            ),
        ],
    ));

    // 6. Shell-profile picker — a host default shell (Unpinned), and a clean session
    //    override shell with a workspace-scoped, fully reversible switch.
    rows.push(base_row(
        M5EnvironmentSelectorSurface::ShellProfilePicker,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Shell-profile picker owner",
        "The shell-profile picker renders the shared components so a host-installed shell with no pin reads as unpinned at host scope, while a session-override shell reads as pinned-resolved with a clear-override action and a workspace-scoped, fully reversible switch previewed before it is applied",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-toolchain-shellprofile:001",
        vec![
            case(
                "shell-host-unpinned",
                Kind::Shell,
                vec![layer(Scope::HostScope, Source::SystemInstalled, "bash-5.2", true)],
                Health::Healthy,
                None,
            ),
            case(
                "shell-session-clean",
                Kind::Shell,
                vec![layer(
                    Scope::SessionScope,
                    Source::SessionOverride,
                    "fish-3.7",
                    true,
                )],
                Health::Healthy,
                Some(switch(
                    Scope::SessionScope,
                    Source::SessionOverride,
                    "zsh-5.9",
                    false,
                    false,
                    &["shell_integration"],
                    true,
                )),
            ),
        ],
    ));

    // 7. Kernel picker — a project kernel shadowing a user pin (PinConflict), and a
    //    workspace kernel with a multi-target, reconnect-required switch.
    rows.push(base_row(
        M5EnvironmentSelectorSurface::KernelPicker,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Kernel picker owner",
        "The kernel picker renders the shared components so a project kernel that shadows a differing user pin reads as pin-conflict with the shadowed user layer disclosed, while a workspace kernel previews a multi-target, reconnect-required switch with its manual-reversal steps before it is applied",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-toolchain-kernel:001",
        vec![
            case(
                "kernel-project-conflict",
                Kind::Kernel,
                vec![
                    layer(Scope::ProjectScope, Source::PinFile, "ipykernel-py312", true),
                    layer(Scope::UserScope, Source::WorkspaceSetting, "ipykernel-py311", true),
                ],
                Health::Healthy,
                None,
            ),
            case(
                "kernel-workspace-reconnect-switch",
                Kind::Kernel,
                vec![layer(
                    Scope::WorkspaceScope,
                    Source::ContainerImage,
                    "ipykernel-py312",
                    true,
                )],
                Health::Healthy,
                Some(switch(
                    Scope::ProjectScope,
                    Source::PinFile,
                    "ipykernel-py311",
                    true,
                    true,
                    &["debug", "attach"],
                    false,
                )),
            ),
        ],
    ));

    // 8. Runtime-target switcher — a project runtime with a toolchain-scoped,
    //    partially reversible restart switch, and a policy override shadowing two
    //    durable pins (PinOverridden).
    rows.push(base_row(
        M5EnvironmentSelectorSurface::RuntimeTargetSwitcher,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Runtime-target switcher owner",
        "The runtime-target switcher renders the shared components so a project runtime previews a toolchain-scoped, restart-required switch with its partial reversibility, while a policy override that shadows both a workspace and a user pin reads as pin-overridden with every shadowed layer disclosed",
        M5ShellZoneSlot::TitleContextBar,
        "evidence:m5-toolchain-runtime:001",
        vec![
            case(
                "runtime-project-restart-switch",
                Kind::Runtime,
                vec![layer(Scope::ProjectScope, Source::PinFile, "node-20", true)],
                Health::Healthy,
                Some(switch(
                    Scope::WorkspaceScope,
                    Source::WorkspaceSetting,
                    "node-18",
                    true,
                    false,
                    &[],
                    false,
                )),
            ),
            case(
                "runtime-policy-shadows-two",
                Kind::Runtime,
                vec![
                    layer(Scope::PolicyScope, Source::WorkspaceSetting, "node-18", true),
                    layer(Scope::WorkspaceScope, Source::ContainerImage, "node-20", true),
                    layer(Scope::UserScope, Source::VersionManager, "node-16", true),
                ],
                Health::Healthy,
                None,
            ),
        ],
    ));

    // 9. Repair-panel selector — a missing workspace runtime (PinnedMissingFallback,
    //    repair attached), and a mismatched project runtime with a host-scoped,
    //    fully reversible switch.
    rows.push(base_row(
        M5EnvironmentSelectorSurface::RepairPanelSelector,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Repair-panel selector owner",
        "The Project Doctor repair-panel selector renders the shared components so a missing workspace runtime reads as pinned-missing-fallback with a repair action attached, while a mismatched project runtime keeps its repair action and previews a host-environment-scoped, fully reversible switch to the global default",
        M5ShellZoneSlot::RightInspector,
        "evidence:m5-toolchain-repair:001",
        vec![
            case(
                "repair-workspace-missing",
                Kind::Runtime,
                vec![layer(
                    Scope::WorkspaceScope,
                    Source::ContainerImage,
                    "runtime-x",
                    true,
                )],
                Health::MissingUnavailable,
                None,
            ),
            case(
                "repair-project-mismatch-switch",
                Kind::Runtime,
                vec![layer(Scope::ProjectScope, Source::PinFile, "runtime-y", true)],
                Health::MismatchedVersion,
                Some(switch(
                    Scope::GlobalDefaultScope,
                    Source::SystemInstalled,
                    "runtime-lts",
                    false,
                    false,
                    &[],
                    true,
                )),
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ToolchainPinSwitchReviewGovernanceReview {
    M5ToolchainPinSwitchReviewGovernanceReview {
        one_primitive_carries_pin_precedence_and_switch: true,
        target_kind_selection_scope_and_source_always_shown: true,
        override_never_silently_shadows_durable_pin: true,
        winning_and_shadowed_layers_always_inspectable: true,
        predicted_blast_radius_always_shown_before_switch: true,
        degraded_selection_always_keeps_repair_action: true,
        support_export_reconstructs_pin_precedence_switch: true,
        no_surface_invents_second_selection_grammar: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ToolchainPinSwitchReviewConsumerProjection {
    M5ToolchainPinSwitchReviewConsumerProjection {
        environment_selectors_consume_shared_primitive: true,
        pin_resolver_reads_single_precedence_source: true,
        precedence_inspector_reads_single_layer_source: true,
        switch_review_reads_single_switch_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ToolchainPinSwitchReviewProofFreshness {
    M5ToolchainPinSwitchReviewProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ToolchainPinSwitchReviewReleasePosture {
    M5ToolchainPinSwitchReviewReleasePosture {
        release_packet_ref: M5_TOOLCHAIN_PIN_SWITCH_REVIEW_ARTIFACT_REF.to_owned(),
        selection_audit_ref: M5_TOOLCHAIN_PIN_SWITCH_REVIEW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TOOLCHAIN_PIN_ROW_SCHEMA_REF,
        M5_PRECEDENCE_INSPECTOR_SCHEMA_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_DOC_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_SHELL_ZONE_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_COMPONENT_MATRIX_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_TOOLCHAIN_MANAGER_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRECEDENCE_REF,
    ])
}

/// Builds the canonical M5 toolchain-pin / switch-review primitive packet.
pub fn seeded_m5_toolchain_pin_switch_review_primitive_packet(
) -> M5ToolchainPinSwitchReviewPrimitivePacket {
    M5ToolchainPinSwitchReviewPrimitivePacket::new(M5ToolchainPinSwitchReviewPrimitivePacketInput {
        packet_id: M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 toolchain pin row, precedence inspector, and switch-review card primitive: target kind, current selection, winning scope and source, pin state, shadowed layers, and switch blast radius"
                .to_owned(),
        selector_rows: selector_rows(),
        vocabulary_set: M5ToolchainPinSwitchReviewVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the repair-panel selector is held at Beta because a slice of
/// repair sessions do not yet render the switch-blast-radius cue on every profile;
/// every surface stays visible.
pub fn seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed(
) -> M5ToolchainPinSwitchReviewPrimitivePacket {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.packet_id = "m5-toolchain-pin-switch-review-primitive:repair-panel-beta:0001".to_owned();
    let row = packet
        .selector_rows
        .iter_mut()
        .find(|row| row.selector_surface == M5EnvironmentSelectorSurface::RepairPanelSelector)
        .expect("repair-panel selector row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the runtime-target switcher is narrowed to Preview pending
/// reversibility parity proof across every export path; every surface stays visible.
pub fn seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed(
) -> M5ToolchainPinSwitchReviewPrimitivePacket {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.packet_id =
        "m5-toolchain-pin-switch-review-primitive:runtime-target-preview:0001".to_owned();
    let row = packet
        .selector_rows
        .iter_mut()
        .find(|row| row.selector_surface == M5EnvironmentSelectorSurface::RuntimeTargetSwitcher)
        .expect("runtime-target switcher row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Preview;
    packet
}
