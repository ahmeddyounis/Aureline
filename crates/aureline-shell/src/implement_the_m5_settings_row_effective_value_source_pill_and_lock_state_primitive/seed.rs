// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 settings-row primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, the worked resolutions, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical settings-row-primitive packet.
pub const M5_SETTINGS_ROW_PRIMITIVE_PACKET_ID: &str = "m5-settings-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a setting key and contributions.
fn case(
    setting_key: &str,
    contributions: Vec<M5SettingsSourceContribution>,
    pending_reload: bool,
    invalid_value_held: bool,
    held_value_repr: Option<&str>,
) -> M5SettingsRowResolutionCase {
    M5SettingsRowResolutionCase::resolved(M5SettingsRowResolutionInput {
        setting_key: setting_key.to_owned(),
        contributions,
        pending_reload,
        invalid_value_held,
        held_value_repr: held_value_repr.map(str::to_owned),
    })
}

/// A base row with the shared fields filled in and the full anatomy, state,
/// source-pill, lock-disclosure, focus-behavior, and export-field parity every
/// surface carries.
fn base_row(
    surface_family: M5SettingsSurfaceFamily,
    qualification: M5TrustQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
    example_resolutions: Vec<M5SettingsRowResolutionCase>,
) -> M5SettingsRowSurfaceRow {
    M5SettingsRowSurfaceRow {
        surface_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5SettingsRowAnatomyPart::ALL.to_vec(),
        row_states: M5SettingsRowState::ALL.to_vec(),
        source_pills: M5SettingSourcePill::ALL.to_vec(),
        lock_disclosures: M5SettingsLockDisclosure::ALL.to_vec(),
        focus_behaviors: M5SettingsRowFocusBehavior::ALL.to_vec(),
        export_fields: M5SettingsRowExportField::ALL.to_vec(),
        accessibility_routes: M5TrustAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5TrustComponentDowngradeTrigger::EffectiveConfiguredConflated,
            M5TrustComponentDowngradeTrigger::SourcePillMissing,
            M5TrustComponentDowngradeTrigger::LockStateUnexplained,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5TrustComponentDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SETTINGS_ROW_SCHEMA_REF,
            M5_SETTINGS_ROW_EFFECTIVE_SETTING_REF,
            M5_SETTINGS_ROW_LOCK_STATE_REF,
        ]),
        example_resolutions,
        conflates_effective_and_configured: false,
        hides_user_configured_when_locked: false,
        invents_private_row_grammar: false,
        drops_export_or_audit_truth: false,
    }
}

fn surface_rows() -> Vec<M5SettingsRowSurfaceRow> {
    use M5SettingSourcePill as SP;
    use M5SettingsSourceContribution as SC;

    let mut rows = Vec::with_capacity(7);

    // 1. Admin / enterprise — a policy-locked value that keeps the user's value
    //    visible. This is the acceptance-criterion example: enforced value and
    //    lock source shown together without hiding the user-configured value.
    rows.push(base_row(
        M5SettingsSurfaceFamily::AdminEnterprise,
        M5TrustQualificationClass::Stable,
        "Admin/enterprise settings owner",
        "The admin/enterprise settings surface renders the shared settings row: a policy-managed value shows its enforced value and lock source together, the user-configured value is retained and shown, and the effective-versus-configured difference is available via view-diff",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-admin:001",
        vec![case(
            "admin.telemetry_sharing",
            vec![
                SC::new(SP::DefaultValue, "disabled"),
                SC::new(SP::UserConfigured, "enabled"),
                SC::locked(SP::PolicyManaged, "disabled"),
            ],
            false,
            false,
            None,
        )],
    ));

    // 2. Workspace trust — the user's own value wins.
    rows.push(base_row(
        M5SettingsSurfaceFamily::WorkspaceTrust,
        M5TrustQualificationClass::Stable,
        "Workspace-trust settings owner",
        "The workspace/project trust settings surface renders the shared settings row so the user-authored trust level reads as the effective value with a user-configured source pill, never confused with an inherited or enforced value",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-trust:001",
        vec![case(
            "workspace.trust_level",
            vec![
                SC::new(SP::DefaultValue, "restricted"),
                SC::new(SP::UserConfigured, "trusted"),
            ],
            false,
            false,
            None,
        )],
    ));

    // 3. AI / model — a workspace override and a redacted credential-managed value.
    rows.push(base_row(
        M5SettingsSurfaceFamily::AiModel,
        M5TrustQualificationClass::Stable,
        "AI/model settings owner",
        "The AI/model settings surface renders the shared settings row so a workspace override names its source, and a credential-managed value is redacted to a managed token rather than exposing material — never conflating a redacted managed value with a user value",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-ai:001",
        vec![
            case(
                "ai.default_response_style",
                vec![
                    SC::new(SP::DefaultValue, "balanced"),
                    SC::new(SP::WorkspaceConfigured, "creative"),
                ],
                false,
                false,
                None,
            ),
            case(
                "ai.provider_credential_ref",
                vec![
                    SC::new(SP::DefaultValue, M5_SETTINGS_REDACTED_VALUE_REPR),
                    SC::locked(SP::PolicyManaged, M5_SETTINGS_REDACTED_VALUE_REPR),
                ],
                false,
                false,
                None,
            ),
        ],
    ));

    // 4. Network / proxy — an environment override supersedes the user value.
    rows.push(base_row(
        M5SettingsSurfaceFamily::NetworkProxy,
        M5TrustQualificationClass::Stable,
        "Network/proxy settings owner",
        "The network/proxy settings surface renders the shared settings row so an environment override shows as the effective value with its source pill while the user-configured value is retained and the view-diff affordance discloses the difference",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-network:001",
        vec![case(
            "network.proxy_mode",
            vec![
                SC::new(SP::DefaultValue, "auto"),
                SC::new(SP::UserConfigured, "auto"),
                SC::new(SP::EnvironmentOverride, "manual"),
            ],
            false,
            false,
            None,
        )],
    ));

    // 5. Execution / runtime — a staged change pending a reload to apply.
    rows.push(base_row(
        M5SettingsSurfaceFamily::ExecutionRuntime,
        M5TrustQualificationClass::Stable,
        "Execution/runtime settings owner",
        "The execution/runtime settings surface renders the shared settings row so a staged change reads as pending-reload-to-apply rather than silently taking effect, keeping the effective value and the staged user value distinct",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-execution:001",
        vec![case(
            "execution.max_parallel_jobs",
            vec![
                SC::new(SP::DefaultValue, "four"),
                SC::new(SP::UserConfigured, "eight"),
            ],
            true,
            false,
            None,
        )],
    ));

    // 6. Extension settings — an inherited default with no configured value.
    rows.push(base_row(
        M5SettingsSurfaceFamily::ExtensionSettings,
        M5TrustQualificationClass::Stable,
        "Extension settings owner",
        "The extension settings surface renders the shared settings row so an unconfigured setting reads as inherited-from-default with a default-value source pill rather than presenting the default as a user choice",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-extension:001",
        vec![case(
            "extension.autoupdate",
            vec![SC::new(SP::DefaultValue, "enabled")],
            false,
            false,
            None,
        )],
    ));

    // 7. Update / config channel — a rejected invalid value holds the prior, and a
    //    remote profile supplies a higher-source value.
    rows.push(base_row(
        M5SettingsSurfaceFamily::UpdateChannel,
        M5TrustQualificationClass::Stable,
        "Update/config-channel settings owner",
        "The update/config-channel settings surface renders the shared settings row so an invalid value holds the prior effective value instead of applying, and a remote profile value shows its source pill — every state reconstructable from the shared export model",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-settings-row-update:001",
        vec![
            case(
                "update.channel",
                vec![
                    SC::new(SP::DefaultValue, "stable"),
                    SC::new(SP::UserConfigured, "not-a-channel"),
                ],
                false,
                true,
                Some("stable"),
            ),
            case(
                "update.check_frequency",
                vec![
                    SC::new(SP::DefaultValue, "daily"),
                    SC::new(SP::RemoteProfile, "weekly"),
                ],
                false,
                false,
                None,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5SettingsRowGovernanceReview {
    M5SettingsRowGovernanceReview {
        one_primitive_carries_effective_versus_configured: true,
        source_pill_and_lock_state_always_explained: true,
        locked_value_never_hides_user_configured: true,
        view_diff_and_source_detail_consistent: true,
        search_landing_and_highlight_consistent: true,
        support_export_reconstructs_effective_value: true,
        no_surface_invents_second_row_grammar: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SettingsRowConsumerProjection {
    M5SettingsRowConsumerProjection {
        config_surfaces_consume_shared_primitive: true,
        resolver_reads_single_precedence_ladder: true,
        lock_explainer_reads_single_source: true,
        search_and_deep_link_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5SettingsRowProofFreshness {
    M5SettingsRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingsRowReleasePosture {
    M5SettingsRowReleasePosture {
        release_packet_ref: M5_SETTINGS_ROW_ARTIFACT_REF.to_owned(),
        settings_row_audit_ref: M5_SETTINGS_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTINGS_ROW_SCHEMA_REF,
        M5_SETTINGS_ROW_DOC_REF,
        M5_SETTINGS_ROW_SHELL_ZONE_REF,
        M5_SETTINGS_ROW_COMPONENT_MATRIX_REF,
        M5_SETTINGS_ROW_EFFECTIVE_SETTING_REF,
        M5_SETTINGS_ROW_LOCK_STATE_REF,
    ])
}

/// Builds the canonical M5 settings-row-primitive packet.
pub fn seeded_m5_settings_row_primitive_packet() -> M5SettingsRowPrimitivePacket {
    M5SettingsRowPrimitivePacket::new(M5SettingsRowPrimitivePacketInput {
        packet_id: M5_SETTINGS_ROW_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 settings-row primitive: effective value, source pill, lock state, view-diff, and source detail"
                .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5SettingsRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the admin/enterprise surface is held at Beta because a slice
/// of enterprise policy-lock explanations do not yet render the override-request
/// path on every profile; every surface stays visible.
pub fn seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed(
) -> M5SettingsRowPrimitivePacket {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.packet_id = "m5-settings-row-primitive:admin-enterprise-beta:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5SettingsSurfaceFamily::AdminEnterprise)
        .expect("admin/enterprise row present");
    row.qualification = M5TrustQualificationClass::Beta;
    packet
}

/// Narrowed variant: the update/config-channel surface is narrowed to Preview
/// pending invalid-value-held parity proof across every export path; every surface
/// stays visible.
pub fn seeded_m5_settings_row_primitive_update_channel_preview_narrowed(
) -> M5SettingsRowPrimitivePacket {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.packet_id = "m5-settings-row-primitive:update-channel-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5SettingsSurfaceFamily::UpdateChannel)
        .expect("update/config-channel row present");
    row.qualification = M5TrustQualificationClass::Preview;
    packet
}
