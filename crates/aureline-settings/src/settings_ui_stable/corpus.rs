//! Deterministic claimed-stable matrix for settings-UI certifications.
//!
//! Every record here is a genuine projection of the **live** settings runtime.
//! The corpus builds an [`EffectiveSettingsResolver`] over the seeded
//! [`SchemaRegistry`], installs overlays representing the active profile, the
//! synced user profile, machine-local state, the workspace, a temporary
//! profile, and the policy-owned ceiling, then projects each visible setting
//! through [`crate::inspector::inspect_setting`] and each previewable write
//! through [`crate::inspector::preview_write`]. The profile-switch review is a
//! real diff between two resolver states. So a certification record can never
//! drift from what the resolver actually resolves.
//!
//! Four postures pin the matrix:
//!
//! - `nominal` — every setting resolves through one record, the shadow chain
//!   exposes the active profile, temporary profile, machine-local, synced,
//!   workspace, and policy-owned contributors, every previewable write is
//!   scope-explicit, every surface shares one truth, and the profile-switch
//!   review is complete. Qualifies **Stable**.
//! - `temporary_profile_active` — a temporary profile wins more settings, so the
//!   inspector shows the temporary-profile contributor as the effective winner.
//!   Still qualifies **Stable**.
//! - `surface_in_preview` — one setting row's settings-UI treatment still
//!   carries a Preview marker, so the posture is narrowed below Stable by its
//!   lowest surface marker instead of inheriting an adjacent green row.
//! - `prose_clone_drill` — an adversarial posture where the migration / import
//!   review surface clones manually maintained prose instead of consuming the
//!   shared record; the lane detects it and narrows the posture below Stable
//!   with a named reason.

use crate::inspector::{
    inspect_setting, preview_write, EffectiveSettingInspectionRecord, InspectorShadowRow,
    SettingWritePreviewRecord, SettingWritePreviewRequest, SettingsInspectionContext,
    WriteActorClass, WriteReasonClass,
};
use crate::resolver::{CapabilityState, EffectiveSettingsResolver, PolicyConstraint, ScopeOverlay};
use crate::schema::{LifecycleLabel, SchemaRegistry, SettingScope, SettingValue};

use super::model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure,
    CertificationClaimCeiling, CertificationInput, CertificationUpstream, ContributorClass,
    EffectiveSettingRow, EntryRouteRecord, LayoutMode, LayoutModeDisclosure, LifecycleMarker,
    PreviewableWriteRow, ProfileSwitchChange, ProfileSwitchReview, RouteSurface,
    SettingsUiCertification, ShadowContributorRow, StableClaimClass, SurfaceClass,
    SurfaceParityRow,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const REGISTRY_REF: &str = "settings:schema_registry_seed:v1";
const INSPECTOR_CONTRACT_REF: &str = "settings:effective_inspector_alpha:v1";
const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/settings-ui";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/settings-ui";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-settings-ui";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-settings-ui";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-settings-ui";

const EGRESS_SETTING: &str = "security.ai.egress_policy";
const LABS_SETTING: &str = "shell.labs.wedge_inspector_enabled";

/// One scenario in the claimed-stable certification matrix.
#[derive(Debug, Clone)]
pub struct SettingsUiScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest surface).
    pub expected_surface_marker: LifecycleMarker,
    record: SettingsUiCertification,
}

impl SettingsUiScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> SettingsUiCertification {
        self.record.clone()
    }
}

/// One whole-posture spec.
struct ScenarioSpec {
    scenario_id: &'static str,
    posture_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    /// Whether the temporary profile wins additional settings.
    temporary_profile_winner: bool,
    /// A setting id whose settings-UI treatment is still in Preview.
    surface_preview_setting: Option<&'static str>,
    /// A surface that clones prose instead of consuming the shared record.
    prose_clone_surface: Option<SurfaceClass>,
    claim_ceiling: CertificationClaimCeiling,
}

fn full_claim_ceiling() -> CertificationClaimCeiling {
    CertificationClaimCeiling {
        asserts_every_setting_resolves_one_record: true,
        asserts_shadow_chain_exposes_contributors: true,
        asserts_writes_scope_explicit: true,
        asserts_surfaces_share_one_truth: true,
        asserts_profile_switch_review_complete: true,
        asserts_setting_ids_canonical: true,
    }
}

fn lifecycle_marker(label: LifecycleLabel) -> LifecycleMarker {
    match label {
        LifecycleLabel::Stable => LifecycleMarker::Stable,
        LifecycleLabel::Preview | LifecycleLabel::Experimental => LifecycleMarker::Preview,
        LifecycleLabel::Deprecated | LifecycleLabel::Retired => LifecycleMarker::Beta,
    }
}

fn restart_impact(token: &str) -> String {
    match token {
        "no_restart" => "Applies live; no restart required.",
        "reload_view" => "Reload affected views to apply.",
        "reload_workspace" => "Reload the active workspace to apply.",
        "restart_extensions" => "Restart extension hosts to apply.",
        "restart_process" => "Restart the application to apply.",
        "reopen_workspace" => "Reopen the active workspace to apply.",
        "restart_shell" => "Restart the desktop shell to apply.",
        _ => "See the setting's declared restart posture.",
    }
    .to_owned()
}

/// Builds the configured resolver for a scenario: the active profile, the synced
/// user profile, machine-local state, the workspace, a temporary profile, and
/// the policy-owned ceiling.
fn configured_resolver(temporary_profile_winner: bool) -> EffectiveSettingsResolver {
    let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());

    let mut active_profile = ScopeOverlay::new(
        SettingScope::ImportedProfileDefault,
        "Active profile defaults",
    );
    active_profile.set_value("editor.tab_size", SettingValue::Integer(2));
    active_profile.set_value("ui.theme", SettingValue::String("light".into()));
    resolver
        .set_overlay(active_profile)
        .expect("active-profile overlay");

    let mut synced = ScopeOverlay::new(SettingScope::UserGlobal, "Synced user profile");
    synced.set_value("editor.tab_size", SettingValue::Integer(4));
    synced.set_value("editor.format_on_save", SettingValue::Boolean(true));
    synced.set_value("ui.density", SettingValue::String("comfortable".into()));
    synced.set_value(
        EGRESS_SETTING,
        SettingValue::String("any_hosted_provider".into()),
    );
    resolver.set_overlay(synced).expect("synced overlay");

    let mut machine = ScopeOverlay::new(SettingScope::MachineSpecific, "This machine");
    machine.set_value("ui.motion", SettingValue::String("reduced".into()));
    resolver.set_overlay(machine).expect("machine overlay");

    let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace settings");
    workspace.set_value("editor.tab_size", SettingValue::Integer(8));
    resolver.set_overlay(workspace).expect("workspace overlay");

    let mut temporary = ScopeOverlay::new(
        SettingScope::SessionOverride,
        "Temporary profile (review session)",
    );
    temporary.set_value("ui.theme", SettingValue::String("dark".into()));
    if temporary_profile_winner {
        temporary.set_value("editor.tab_size", SettingValue::Integer(2));
    }
    resolver.set_overlay(temporary).expect("temporary overlay");

    let mut policy =
        ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
    policy.set_policy_constraint(
        EGRESS_SETTING,
        PolicyConstraint::SingleValue {
            value: SettingValue::String("approved_hosted_providers_only".into()),
        },
    );
    resolver.set_overlay(policy).expect("policy overlay");

    let egress_dep = resolver
        .registry()
        .definition(EGRESS_SETTING)
        .expect("egress definition")
        .capability_dependencies[0]
        .clone();
    resolver.set_capability_state(
        &egress_dep,
        CapabilityState::satisfied("identity_mode=managed_convenience"),
    );
    let labs_dep = resolver
        .registry()
        .definition(LABS_SETTING)
        .expect("labs definition")
        .capability_dependencies[0]
        .clone();
    resolver.set_capability_state(&labs_dep, CapabilityState::satisfied("flag=on"));

    resolver
}

fn inspection_context(resolver: &EffectiveSettingsResolver) -> SettingsInspectionContext {
    let egress_dep = resolver
        .registry()
        .definition(EGRESS_SETTING)
        .expect("egress definition")
        .capability_dependencies[0]
        .clone();
    let labs_dep = resolver
        .registry()
        .definition(LABS_SETTING)
        .expect("labs definition")
        .capability_dependencies[0]
        .clone();
    SettingsInspectionContext::new()
        .with_last_applied_revision(EGRESS_SETTING, "settings-rev:00042")
        .with_capability_state(&egress_dep, true, "identity_mode=managed_convenience")
        .with_capability_state(&labs_dep, true, "flag=on")
}

fn contributor_for(scope_token: &str) -> ContributorClass {
    ContributorClass::from_scope_token(scope_token)
        .unwrap_or_else(|| panic!("unmapped scope token {scope_token:?}"))
}

fn shadow_rows(rows: &[InspectorShadowRow]) -> Vec<ShadowContributorRow> {
    rows.iter()
        .map(|row| ShadowContributorRow {
            scope: row.scope.clone(),
            contributor_class: contributor_for(&row.scope),
            source_label: row.source_label.clone(),
            value_preview: row.value_preview.clone(),
            relation: row.relation.clone(),
            winner: row.winner,
        })
        .collect()
}

fn dependency_keys(record: &EffectiveSettingInspectionRecord) -> Vec<String> {
    record
        .definition
        .capability_dependencies
        .iter()
        .map(|dep| match &dep.required_ref {
            Some(required) => format!("{}:{required}", dep.kind),
            None => dep.kind.clone(),
        })
        .collect()
}

fn effective_row(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    setting_id: &str,
    surface_preview_setting: Option<&str>,
) -> EffectiveSettingRow {
    let record = inspect_setting(resolver, setting_id, context)
        .unwrap_or_else(|err| panic!("inspect {setting_id}: {err}"));
    let lifecycle = resolver
        .registry()
        .definition(setting_id)
        .map(|def| lifecycle_marker(def.lifecycle_label))
        .unwrap_or(LifecycleMarker::Stable);
    let is_preview_surface = surface_preview_setting == Some(setting_id);
    let surface_marker = if is_preview_surface {
        LifecycleMarker::Preview
    } else {
        LifecycleMarker::Stable
    };
    let waiver_ref = is_preview_surface
        .then(|| format!("aureline://waiver/settings-ui-{setting_id}-surface-preview"));

    EffectiveSettingRow {
        setting_id: record.setting_id.clone(),
        declared_type: record.definition.declared_type.clone(),
        winning_value_summary: record.winning_value_summary.clone(),
        winning_scope: record.winning_scope.clone(),
        winning_contributor: contributor_for(&record.winning_scope),
        source_label: record.source_label.clone(),
        setting_lifecycle: lifecycle,
        surface_marker,
        allowed_scopes: record.definition.allowed_scopes.clone(),
        default_value_preview: record.definition.default_value_preview.clone(),
        migration_aliases: record.definition.migration_aliases.clone(),
        restart_posture: record.restart_state.restart_posture.clone(),
        restart_required: record.restart_state.restart_required,
        sensitivity_class: record.definition.sensitivity_class.clone(),
        redaction_class: record.definition.redaction_class.clone(),
        preview_class: record.definition.preview_class.clone(),
        capability_dependencies: dependency_keys(&record),
        help_doc_ref: record.definition.help_doc_ref.clone(),
        lock_state: record.lock_state.clone(),
        lock_reason: record.lock_reason.clone(),
        escalation_path_ref: format!("aureline://diagnostics/settings/{setting_id}"),
        shadow_chain: shadow_rows(&record.shadow_chain),
        effective_record_ref: format!("aureline://effective-setting/{setting_id}"),
        setting_id_canonical: true,
        resolves_through_one_record: false,
        waiver_ref,
        conforms: false,
    }
}

fn effective_settings(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    surface_preview_setting: Option<&str>,
) -> Vec<EffectiveSettingRow> {
    let ids: Vec<String> = resolver.registry().ids().map(str::to_owned).collect();
    ids.iter()
        .map(|id| effective_row(resolver, context, id, surface_preview_setting))
        .collect()
}

struct WriteSpec {
    setting_id: &'static str,
    target_scope: SettingScope,
    proposed_value: SettingValue,
    reason_class: WriteReasonClass,
    checkpoint_ref: Option<&'static str>,
    approval_ticket_ref: Option<&'static str>,
}

fn previewable_write(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    spec: &WriteSpec,
) -> PreviewableWriteRow {
    let preview: SettingWritePreviewRecord = preview_write(
        resolver,
        SettingWritePreviewRequest {
            setting_id: spec.setting_id.to_owned(),
            target_scope: spec.target_scope,
            proposed_value: spec.proposed_value.clone(),
            actor_class: WriteActorClass::UserCommand,
            reason_class: spec.reason_class,
            checkpoint_ref: spec.checkpoint_ref.map(str::to_owned),
            approval_ticket_ref: spec.approval_ticket_ref.map(str::to_owned),
        },
        context,
    );
    let scope_token = spec.target_scope.as_str();
    let restart_posture = preview
        .restart_posture
        .clone()
        .unwrap_or_else(|| "no_restart".to_owned());
    let lifecycle_dependency = resolver
        .registry()
        .definition(spec.setting_id)
        .and_then(|def| def.capability_dependencies.first())
        .map(|dep| match &dep.required_ref {
            Some(required) => format!("capability:{}:{required}", dep.kind.as_str()),
            None => format!("capability:{}", dep.kind.as_str()),
        });

    PreviewableWriteRow {
        setting_id: preview.setting_id.clone(),
        target_scope: scope_token.to_owned(),
        target_contributor: contributor_for(scope_token),
        target_artifact_ref: format!(
            "aureline://settings-artifact/{scope_token}/{}",
            preview.setting_id
        ),
        scope_explicit: preview.destination_preview.scope_explicit,
        verdict: preview.verdict.clone(),
        denied: preview.verdict == "denied",
        blocked_write_reason: preview.denial_reason.clone(),
        diagnostics_entry_ref: format!(
            "aureline://diagnostics/settings-write/{}",
            preview.setting_id
        ),
        restart_impact: restart_impact(&restart_posture),
        restart_posture,
        preview_required: preview.preview_required,
        checkpoint_required: preview.checkpoint_required,
        approval_required: preview.approval_required,
        lifecycle_dependency,
        conforms: false,
    }
}

fn previewable_writes(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Vec<PreviewableWriteRow> {
    let specs = [
        WriteSpec {
            setting_id: "editor.tab_size",
            target_scope: SettingScope::UserGlobal,
            proposed_value: SettingValue::Integer(6),
            reason_class: WriteReasonClass::UserEdit,
            checkpoint_ref: None,
            approval_ticket_ref: None,
        },
        WriteSpec {
            setting_id: "ui.theme",
            target_scope: SettingScope::UserGlobal,
            proposed_value: SettingValue::String("system".into()),
            reason_class: WriteReasonClass::UserEdit,
            checkpoint_ref: None,
            approval_ticket_ref: None,
        },
        WriteSpec {
            setting_id: EGRESS_SETTING,
            target_scope: SettingScope::Workspace,
            proposed_value: SettingValue::String("approved_hosted_providers_only".into()),
            reason_class: WriteReasonClass::ProfileApply,
            checkpoint_ref: Some("aureline://checkpoint/settings-write/egress-workspace"),
            approval_ticket_ref: Some("aureline://approval/settings-write/egress-workspace"),
        },
        WriteSpec {
            setting_id: EGRESS_SETTING,
            target_scope: SettingScope::UserGlobal,
            proposed_value: SettingValue::String("any_hosted_provider".into()),
            reason_class: WriteReasonClass::UserEdit,
            checkpoint_ref: None,
            approval_ticket_ref: None,
        },
        WriteSpec {
            setting_id: "vfs.watcher.fallback_polling_ms",
            target_scope: SettingScope::UserGlobal,
            proposed_value: SettingValue::Integer(500),
            reason_class: WriteReasonClass::UserEdit,
            checkpoint_ref: None,
            approval_ticket_ref: None,
        },
    ];
    specs
        .iter()
        .map(|spec| previewable_write(resolver, context, spec))
        .collect()
}

fn surface_parity(prose_clone_surface: Option<SurfaceClass>) -> Vec<SurfaceParityRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface| {
            let clones = prose_clone_surface == Some(surface);
            SurfaceParityRow {
                surface_class: surface,
                consumes_shared_record: !clones,
                clones_prose: clones,
                shared_contract_ref: super::model::SETTINGS_UI_SHARED_CONTRACT_REF.to_owned(),
                record_ref: format!("aureline://settings-ui-surface/{}", surface.as_str()),
                waiver_ref: clones
                    .then(|| format!("aureline://waiver/settings-ui-{}-prose", surface.as_str())),
                conforms: false,
            }
        })
        .collect()
}

/// Builds the profile-switch review as a real diff between the active profile and
/// the temporary profile being switched to.
fn profile_switch_review() -> ProfileSwitchReview {
    let before = review_resolver(false);
    let after = review_resolver(true);

    let ids: Vec<String> = before.registry().ids().map(str::to_owned).collect();
    let mut immediate_changes = Vec::new();
    let mut restart_required_changes = Vec::new();
    let mut excluded_machine_specific = Vec::new();
    let mut narrowing_effects = Vec::new();

    for id in &ids {
        let def = before.registry().definition(id).expect("definition");
        if def.is_machine_specific {
            excluded_machine_specific.push(id.clone());
            continue;
        }
        let before_value = before.resolve(id).expect("resolve before");
        let after_value = after.resolve(id).expect("resolve after");
        let narrowed = matches!(
            after_value.lock_state,
            crate::resolver::LockState::PolicyLocked
                | crate::resolver::LockState::PolicyConstrained
        );
        if narrowed {
            narrowing_effects.push(id.clone());
        }
        if before_value.value == after_value.value {
            continue;
        }
        let restart_required = !matches!(
            def.restart_posture,
            crate::schema::RestartPosture::NoRestart
        );
        let change = ProfileSwitchChange {
            setting_id: id.clone(),
            before_value_preview: before_value.value.preview(),
            after_value_preview: after_value.value.preview(),
            restart_required,
            narrowed,
        };
        if restart_required {
            restart_required_changes.push(change);
        } else {
            immediate_changes.push(change);
        }
    }
    immediate_changes.sort_by(|a, b| a.setting_id.cmp(&b.setting_id));
    restart_required_changes.sort_by(|a, b| a.setting_id.cmp(&b.setting_id));
    excluded_machine_specific.sort();
    narrowing_effects.sort();
    narrowing_effects.dedup();

    ProfileSwitchReview {
        from_profile_ref: "aureline://profile/active-default".to_owned(),
        to_profile_ref: "aureline://profile/temporary-review".to_owned(),
        to_profile_is_temporary: true,
        immediate_changes,
        restart_required_changes,
        excluded_machine_specific,
        narrowing_effects,
        creates_rollback_checkpoint: true,
        rollback_checkpoint_ref: Some(
            "aureline://checkpoint/profile-switch/active-to-temporary".to_owned(),
        ),
        summary: "Switching to the temporary review profile changes editor and theme settings \
                  immediately, requires an extension-host restart for the AI egress control, \
                  excludes machine-local filesystem tuning, narrows the egress control under the \
                  active admin policy, and creates a rollback checkpoint before apply."
            .to_owned(),
        conforms: false,
    }
}

fn review_resolver(profile_b: bool) -> EffectiveSettingsResolver {
    let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());

    let mut active = ScopeOverlay::new(SettingScope::ImportedProfileDefault, "Active profile");
    active.set_value(
        "ui.theme",
        SettingValue::String(if profile_b { "dark" } else { "light" }.into()),
    );
    resolver.set_overlay(active).expect("review active overlay");

    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "Profile user layer");
    user.set_value(
        "editor.tab_size",
        SettingValue::Integer(if profile_b { 2 } else { 4 }),
    );
    user.set_value(
        EGRESS_SETTING,
        SettingValue::String(
            if profile_b {
                "approved_hosted_providers_only"
            } else {
                "disabled"
            }
            .into(),
        ),
    );
    resolver.set_overlay(user).expect("review user overlay");

    let mut machine = ScopeOverlay::new(SettingScope::MachineSpecific, "This machine");
    machine.set_value(
        "vfs.watcher.fallback_polling_ms",
        SettingValue::Integer(2_500),
    );
    resolver
        .set_overlay(machine)
        .expect("review machine overlay");

    let mut policy = ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle");
    policy.set_policy_constraint(
        EGRESS_SETTING,
        PolicyConstraint::NarrowedSet {
            allowed: vec![
                SettingValue::String("disabled".into()),
                SettingValue::String("approved_hosted_providers_only".into()),
            ],
        },
    );
    resolver.set_overlay(policy).expect("review policy overlay");

    resolver
}

fn accessibility() -> AccessibilityDisclosure {
    let action_labels: Vec<String> = required_recovery_routes()
        .into_iter()
        .map(|route| route.action_label)
        .collect();
    let layout_modes = LayoutMode::REQUIRED
        .into_iter()
        .map(|mode| LayoutModeDisclosure {
            mode,
            row_narration_available: true,
            recovery_affordances_reachable: true,
        })
        .collect();
    AccessibilityDisclosure {
        focus_order_index: 0,
        tab_stop_count: 6,
        row_narration: "Effective-configuration inspector for the active settings posture"
            .to_owned(),
        action_labels,
        layout_modes,
    }
}

fn routes() -> Vec<EntryRouteRecord> {
    RouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!("aureline://route/settings-ui/{}", surface.as_str()),
            keyboard_reachable: true,
            activates_same_record: true,
        })
        .collect()
}

fn upstream(scenario_id: &str, setting_ids: Vec<String>) -> CertificationUpstream {
    CertificationUpstream {
        registry_ref: REGISTRY_REF.to_owned(),
        resolver_state_ref: format!("aureline://resolver-state/settings-ui-{scenario_id}"),
        inspector_contract_ref: INSPECTOR_CONTRACT_REF.to_owned(),
        certified_setting_ids: setting_ids,
    }
}

fn build_scenario(spec: &ScenarioSpec) -> SettingsUiScenario {
    let resolver = configured_resolver(spec.temporary_profile_winner);
    let context = inspection_context(&resolver);
    let effective_settings = effective_settings(&resolver, &context, spec.surface_preview_setting);
    let setting_ids: Vec<String> = effective_settings
        .iter()
        .map(|row| row.setting_id.clone())
        .collect();

    let input = CertificationInput {
        record_id: spec.scenario_id.to_owned(),
        as_of: CORPUS_AS_OF.to_owned(),
        posture_id: spec.posture_id.to_owned(),
        posture_label: spec.posture_label.to_owned(),
        title: spec.title.to_owned(),
        summary: spec.summary.to_owned(),
        effective_settings,
        previewable_writes: previewable_writes(&resolver, &context),
        surface_parity: surface_parity(spec.prose_clone_surface),
        profile_switch_review: profile_switch_review(),
        claim_ceiling: spec.claim_ceiling,
        recovery_routes: required_recovery_routes(),
        routes: routes(),
        accessibility: accessibility(),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(spec.scenario_id, setting_ids),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_owned(),
        support_export_ref: SUPPORT_EXPORT_REF.to_owned(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_owned(),
            EVIDENCE_FIXTURE_REF.to_owned(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_owned()],
    };
    let record = SettingsUiCertification::build(input)
        .unwrap_or_else(|err| panic!("scenario {} must build: {err}", spec.scenario_id));

    SettingsUiScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id),
        expected_posture: record.posture_id.clone(),
        expected_claim_class: record.stable_qualification.claim_class,
        expected_qualifies_stable: record.stable_qualification.qualifies_stable,
        expected_surface_marker: record.surface_lifecycle_marker,
        record,
    }
}

/// Returns the deterministic claimed-stable certification matrix.
pub fn settings_ui_corpus() -> Vec<SettingsUiScenario> {
    let specs = [
        ScenarioSpec {
            scenario_id: "nominal",
            posture_id: "settings_ui_nominal",
            posture_label: "Nominal settings UI",
            title: "Settings UI is certified across the effective-configuration inspector",
            summary: "Every visible setting resolves through one effective-setting record; the \
                      shadow chain exposes the active profile, temporary profile, machine-local, \
                      synced, workspace, and policy-owned contributors; previewable writes are \
                      scope-explicit; every surface shares one truth; and the profile-switch \
                      review is complete.",
            temporary_profile_winner: false,
            surface_preview_setting: None,
            prose_clone_surface: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "temporary_profile_active",
            posture_id: "settings_ui_temporary_profile_active",
            posture_label: "Temporary profile active",
            title: "A temporary profile wins settings and is named in the shadow chain",
            summary: "A temporary profile applied for the session wins additional settings; the \
                      inspector shows the temporary-profile contributor as the effective winner \
                      rather than implying a flat value, and the posture stays Stable.",
            temporary_profile_winner: true,
            surface_preview_setting: None,
            prose_clone_surface: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "surface_in_preview",
            posture_id: "settings_ui_surface_in_preview",
            posture_label: "Labs setting surface still in Preview",
            title: "A below-Stable row narrows the certification instead of inheriting green",
            summary: "Every setting resolves through one record, but the labs setting's \
                      settings-UI treatment still carries a Preview marker; the posture is \
                      narrowed below Stable by its lowest surface marker rather than inheriting \
                      an adjacent green row.",
            temporary_profile_winner: false,
            surface_preview_setting: Some(LABS_SETTING),
            prose_clone_surface: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "prose_clone_drill",
            posture_id: "settings_ui_prose_clone_drill",
            posture_label: "Migration review clones prose drill",
            title: "A surface that clones prose narrows the certification with a named reason",
            summary: "An adversarial posture where the migration / import review surface clones \
                      manually maintained prose instead of consuming the shared setting-definition \
                      registry and effective-setting record; the lane detects the divergence and \
                      narrows the posture below Stable with a named reason.",
            temporary_profile_winner: false,
            surface_preview_setting: None,
            prose_clone_surface: Some(SurfaceClass::MigrationImportReview),
            claim_ceiling: CertificationClaimCeiling {
                asserts_surfaces_share_one_truth: false,
                ..full_claim_ceiling()
            },
        },
    ];
    let scenarios: Vec<SettingsUiScenario> = specs.iter().map(build_scenario).collect();
    debug_assert!(scenarios.iter().all(|scenario| {
        is_canonical_object_ref(&scenario.record.diagnostics_export_ref)
            && is_canonical_object_ref(&scenario.record.support_export_ref)
    }));
    scenarios
}
