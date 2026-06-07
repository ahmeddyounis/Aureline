//! Deterministic stable corpus for structured-input truth.

use super::model::{
    AccessibilityReview, ApplyTiming, BuildImpactClass, ClientLimitation, ClientScope,
    CodeBackedFieldTruth, FieldAction, FieldActionClass, FieldKind, FieldRowContract,
    FormSurfaceClass, FormTruthPacketRecord, PathBasis, PathFieldTruth, PathLocationClass,
    PrecedenceCandidate, ReferenceFieldTruth, RequirementState, SecretExportBehavior,
    SecretFieldTruth, SecretStorageMode, SideEffectClass, SourcePrecedence, SourcePrecedenceAudit,
    StagedApplyPacket, ValidationClass, ValidationResult, ValidationState, WizardStepState,
    FORM_TRUTH_NOTICE, FORM_TRUTH_RECORD_KIND, FORM_TRUTH_SCHEMA_VERSION,
    FORM_TRUTH_SHARED_CONTRACT_REF,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-06-07T00:00:00Z";

const PERSISTED_ACROSS: &[&str] = &["tab_change", "reconnect", "client_handoff"];

/// One stable scenario in the form truth corpus.
#[derive(Debug, Clone)]
pub struct FormTruthScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected form apply timing.
    pub expected_apply_timing: ApplyTiming,
    /// Expected client scope.
    pub expected_client_scope: ClientScope,
    /// Expected surface class.
    pub expected_surface_class: FormSurfaceClass,
    record: FormTruthPacketRecord,
}

impl FormTruthScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> FormTruthPacketRecord {
        self.record.clone()
    }
}

/// Returns the deterministic structured-input truth corpus.
pub fn forms_parameter_source_and_staged_apply_corpus() -> Vec<FormTruthScenario> {
    vec![
        immediate_settings(),
        staged_workspace_settings_remote(),
        preview_first_provider_publish(),
        policy_locked_managed_setup(),
        offline_stale_recovery(),
        browser_companion_restricted(),
        restricted_client_provider_account(),
    ]
}

fn immediate_settings() -> FormTruthScenario {
    let fields = vec![field(
        "editor.wrap",
        "Line wrapping",
        "on",
        RequirementState::Optional,
        FieldKind::Plain,
        SourcePrecedence::UserOverride,
        ApplyTiming::Immediate,
        validation(
            ValidationClass::LocalSyntax,
            ValidationState::Ready,
            "Boolean preference is valid for the active editor.",
            false,
        ),
    )];
    scenario(
        "forms-truth:settings-immediate-user-override",
        "settings_immediate_user_override.json",
        FormSurfaceClass::SettingsEditor,
        ClientScope::Desktop,
        "Editor settings",
        "active editor preference",
        fields,
        vec![audit(
            "editor.wrap",
            SourcePrecedence::UserOverride,
            "on",
            "User override wins over the default for this local profile.",
            vec![
                candidate(
                    SourcePrecedence::Default,
                    "off",
                    10,
                    false,
                    Some("User override"),
                ),
                candidate(SourcePrecedence::UserOverride, "on", 80, true, None),
            ],
        )],
        StagedApplyPacket {
            apply_timing: ApplyTiming::Immediate,
            dirty: false,
            dirty_scopes: vec![],
            dirty_state_persisted_across: vec![],
            review_sheet_ref: None,
            checkpoint_ref: None,
            revert_action_label: Some("Revert line wrapping".into()),
            target_scope: "Active editor group".into(),
            side_effects: vec![SideEffectClass::EditorOnly],
            build_impact_class: BuildImpactClass::NarrowLocal,
            final_submit_label: "Apply line wrapping".into(),
            save_and_resume_disclosed: false,
        },
        None,
        vec![],
    )
}

fn staged_workspace_settings_remote() -> FormTruthScenario {
    let mut fields = vec![
        path_field(
            "workspace.output_dir",
            "Build output directory",
            "target/remote",
            SourcePrecedence::Workspace,
            ApplyTiming::Staged,
            PathBasis::WorkspaceRelative,
            PathLocationClass::Remote,
            "remote-target:devcontainer-rust",
        ),
        field(
            "toolchain.channel",
            "Rust toolchain",
            "stable",
            RequirementState::Required,
            FieldKind::Plain,
            SourcePrecedence::Detected,
            ApplyTiming::Staged,
            pending_validation(),
        ),
    ];
    fields.push(field(
        "profile.imported_theme",
        "Imported theme",
        "Solarized Light",
        RequirementState::Optional,
        FieldKind::Plain,
        SourcePrecedence::Imported,
        ApplyTiming::Staged,
        validation(
            ValidationClass::SchemaModel,
            ValidationState::Ready,
            "Imported theme maps to a known theme package.",
            false,
        ),
    ));
    scenario(
        "forms-truth:remote-staged-workspace-settings",
        "remote_staged_workspace_settings.json",
        FormSurfaceClass::SettingsEditor,
        ClientScope::RemoteWorkspace,
        "Workspace settings",
        "remote dev container workspace",
        fields,
        vec![
            audit(
                "workspace.output_dir",
                SourcePrecedence::Workspace,
                "target/remote",
                "Workspace value wins because no policy value is set for this path.",
                vec![
                    candidate(
                        SourcePrecedence::Default,
                        "target",
                        10,
                        false,
                        Some("Workspace value"),
                    ),
                    candidate(SourcePrecedence::Workspace, "target/remote", 60, true, None),
                ],
            ),
            audit(
                "toolchain.channel",
                SourcePrecedence::Detected,
                "stable",
                "Detected remote toolchain is used until the user stages an override.",
                vec![
                    candidate(
                        SourcePrecedence::Default,
                        "stable",
                        10,
                        false,
                        Some("Detected"),
                    ),
                    candidate(SourcePrecedence::Detected, "stable", 40, true, None),
                ],
            ),
            audit(
                "profile.imported_theme",
                SourcePrecedence::Imported,
                "Solarized Light",
                "Imported value is preserved until the user chooses a new workspace value.",
                vec![
                    candidate(
                        SourcePrecedence::Default,
                        "Aureline Dark",
                        10,
                        false,
                        Some("Imported"),
                    ),
                    candidate(
                        SourcePrecedence::Imported,
                        "Solarized Light",
                        50,
                        true,
                        None,
                    ),
                ],
            ),
        ],
        staged_packet("Apply workspace settings", BuildImpactClass::WorkspaceWide),
        Some(WizardStepState {
            current_step: 2,
            total_steps: 3,
            blocked_prerequisites: vec![],
            save_and_resume_visible: true,
            final_submit_label: "Apply workspace settings".into(),
        }),
        vec![],
    )
}

fn preview_first_provider_publish() -> FormTruthScenario {
    let fields = vec![
        secret_field(
            "provider.token",
            "Provider token",
            "vault://ci/provider-token",
            SourcePrecedence::SecretReference,
            ApplyTiming::PreviewFirst,
            SecretStorageMode::ExternalVaultReference,
        ),
        reference_field(
            "publish.target",
            "Publish target",
            "Production project",
            SourcePrecedence::UserOverride,
            ApplyTiming::PreviewFirst,
            "provider/project/prod-42",
        ),
        code_backed_field(
            "publish.comments",
            "Generated comments",
            "4 comments",
            SourcePrecedence::Workspace,
            ApplyTiming::PreviewFirst,
        ),
    ];
    scenario(
        "forms-truth:provider-preview-first-publish",
        "provider_preview_first_publish.json",
        FormSurfaceClass::PublishReview,
        ClientScope::Desktop,
        "Publish review",
        "provider project prod-42",
        fields,
        vec![
            audit(
                "provider.token",
                SourcePrecedence::SecretReference,
                "vault reference",
                "Secret reference wins because raw tokens are never written to the publish packet.",
                vec![candidate(
                    SourcePrecedence::SecretReference,
                    "vault reference",
                    90,
                    true,
                    None,
                )],
            ),
            audit(
                "publish.target",
                SourcePrecedence::UserOverride,
                "Production project",
                "User override wins for this one publish run.",
                vec![
                    candidate(
                        SourcePrecedence::Default,
                        "Staging project",
                        10,
                        false,
                        Some("User override"),
                    ),
                    candidate(
                        SourcePrecedence::UserOverride,
                        "Production project",
                        80,
                        true,
                        None,
                    ),
                ],
            ),
            audit(
                "publish.comments",
                SourcePrecedence::Workspace,
                "4 comments",
                "Workspace review bundle defines the generated comment set.",
                vec![candidate(
                    SourcePrecedence::Workspace,
                    "4 comments",
                    60,
                    true,
                    None,
                )],
            ),
        ],
        StagedApplyPacket {
            apply_timing: ApplyTiming::PreviewFirst,
            dirty: true,
            dirty_scopes: vec!["publish target".into(), "generated comments".into()],
            dirty_state_persisted_across: persisted_across(),
            review_sheet_ref: Some("aureline://review-sheet/provider-publish-prod-42".into()),
            checkpoint_ref: Some("checkpoint://provider-publish/prod-42".into()),
            revert_action_label: Some("Revert publish draft".into()),
            target_scope: "Provider project prod-42".into(),
            side_effects: vec![
                SideEffectClass::NetworkCall,
                SideEffectClass::RemoteMutation,
            ],
            build_impact_class: BuildImpactClass::RemoteOrProvider,
            final_submit_label: "Publish 4 comments".into(),
            save_and_resume_disclosed: true,
        },
        Some(WizardStepState {
            current_step: 3,
            total_steps: 3,
            blocked_prerequisites: vec![],
            save_and_resume_visible: true,
            final_submit_label: "Publish 4 comments".into(),
        }),
        vec![],
    )
}

fn policy_locked_managed_setup() -> FormTruthScenario {
    let fields = vec![
        field(
            "region",
            "Workspace region",
            "us-gov-west",
            RequirementState::Required,
            FieldKind::Plain,
            SourcePrecedence::Policy,
            ApplyTiming::PolicyLocked,
            validation(
                ValidationClass::PolicyEntitlement,
                ValidationState::Blocked,
                "Region is managed by organization policy; choose an allowed non-region option or open policy details.",
                true,
            ),
        ),
        secret_field(
            "managed.identity",
            "Managed identity",
            "policy provided",
            SourcePrecedence::SecretReference,
            ApplyTiming::PolicyLocked,
            SecretStorageMode::PolicyProvided,
        ),
    ];
    scenario(
        "forms-truth:managed-policy-locked-setup",
        "managed_policy_locked_setup.json",
        FormSurfaceClass::PolicyReview,
        ClientScope::ManagedWorkspace,
        "Managed workspace setup",
        "organization-managed workspace",
        fields,
        vec![
            audit(
                "region",
                SourcePrecedence::Policy,
                "us-gov-west",
                "Policy wins over detected and user values for managed workspaces.",
                vec![
                    candidate(
                        SourcePrecedence::Detected,
                        "us-west-2",
                        40,
                        false,
                        Some("Policy"),
                    ),
                    candidate(
                        SourcePrecedence::UserOverride,
                        "eu-central-1",
                        80,
                        false,
                        Some("Policy"),
                    ),
                    candidate(SourcePrecedence::Policy, "us-gov-west", 100, true, None),
                ],
            ),
            audit(
                "managed.identity",
                SourcePrecedence::SecretReference,
                "policy provided",
                "Policy-provided identity is passed by reference and cannot be revealed here.",
                vec![candidate(
                    SourcePrecedence::SecretReference,
                    "policy provided",
                    100,
                    true,
                    None,
                )],
            ),
        ],
        StagedApplyPacket {
            apply_timing: ApplyTiming::PolicyLocked,
            dirty: false,
            dirty_scopes: vec![],
            dirty_state_persisted_across: vec![],
            review_sheet_ref: Some("aureline://policy/org-managed-workspace-region".into()),
            checkpoint_ref: None,
            revert_action_label: None,
            target_scope: "Managed workspace policy envelope".into(),
            side_effects: vec![SideEffectClass::PolicyChange],
            build_impact_class: BuildImpactClass::PolicyManaged,
            final_submit_label: "View policy details".into(),
            save_and_resume_disclosed: false,
        },
        None,
        vec![],
    )
}

fn offline_stale_recovery() -> FormTruthScenario {
    let mut stale = validation(
        ValidationClass::EnvironmentProbe,
        ValidationState::Stale,
        "Last probe was for a previous target and will rerun when connectivity returns.",
        true,
    );
    stale.dependent_inputs = vec!["workspace.target".into(), "runtime.image".into()];
    stale.invalidates_on_target_change = true;
    let fields = vec![
        path_field(
            "repair.workspace_path",
            "Workspace path",
            "./service",
            SourcePrecedence::Default,
            ApplyTiming::Staged,
            PathBasis::WorkspaceRelative,
            PathLocationClass::Workspace,
            "workspace-root:cached-service",
        ),
        field(
            "repair.target",
            "Repair target",
            "cached remote target",
            RequirementState::Conditional,
            FieldKind::ObjectReference,
            SourcePrecedence::Detected,
            ApplyTiming::Staged,
            stale,
        ),
    ];
    let mut record =
        scenario(
            "forms-truth:offline-stale-recovery",
            "offline_stale_recovery.json",
            FormSurfaceClass::RecoveryReview,
            ClientScope::OfflineDegraded,
            "Recovery review",
            "cached remote workspace",
            fields,
            vec![
            audit(
                "repair.workspace_path",
                SourcePrecedence::Default,
                "./service",
                "Default path is retained because offline recovery has no fresher workspace value.",
                vec![candidate(SourcePrecedence::Default, "./service", 10, true, None)],
            ),
            audit(
                "repair.target",
                SourcePrecedence::Detected,
                "cached remote target",
                "Detected cached target is shown as stale until reconnect validation succeeds.",
                vec![candidate(SourcePrecedence::Detected, "cached remote target", 40, true, None)],
            ),
        ],
            staged_packet("Resume repair review", BuildImpactClass::WorkspaceWide),
            Some(WizardStepState {
                current_step: 1,
                total_steps: 2,
                blocked_prerequisites: vec!["Reconnect to refresh target probe".into()],
                save_and_resume_visible: true,
                final_submit_label: "Resume repair review".into(),
            }),
            vec![],
        );
    if let Some(field) = record
        .record
        .fields
        .iter_mut()
        .find(|field| field.field_id == "repair.target")
    {
        field.reference_truth = Some(ReferenceFieldTruth {
            display_label: "cached remote target".into(),
            stable_id_path: "remote/target/cached-service".into(),
        });
    }
    record
}

fn browser_companion_restricted() -> FormTruthScenario {
    let fields = vec![field(
        "scaffold.destination",
        "Destination folder",
        "not available in browser companion",
        RequirementState::Required,
        FieldKind::Path,
        SourcePrecedence::RuntimePrompt,
        ApplyTiming::PreviewFirst,
        validation(
            ValidationClass::PolicyEntitlement,
            ValidationState::Blocked,
            "Browser companion cannot choose a local destination; hand off to desktop before creating files.",
            true,
        ),
    )];
    let mut record = scenario(
        "forms-truth:browser-companion-restricted-scaffold",
        "browser_companion_restricted_scaffold.json",
        FormSurfaceClass::ScaffoldWizard,
        ClientScope::BrowserCompanion,
        "Create workspace",
        "local filesystem scaffold",
        fields,
        vec![audit(
            "scaffold.destination",
            SourcePrecedence::RuntimePrompt,
            "desktop handoff required",
            "Runtime prompt is deferred because the browser companion lacks local filesystem authority.",
            vec![candidate(SourcePrecedence::RuntimePrompt, "desktop handoff required", 30, true, None)],
        )],
        StagedApplyPacket {
            apply_timing: ApplyTiming::PreviewFirst,
            dirty: true,
            dirty_scopes: vec!["scaffold parameters".into()],
            dirty_state_persisted_across: persisted_across(),
            review_sheet_ref: Some("aureline://review-sheet/scaffold-browser-handoff".into()),
            checkpoint_ref: Some("checkpoint://scaffold/browser-handoff".into()),
            revert_action_label: Some("Discard scaffold draft".into()),
            target_scope: "Desktop filesystem after handoff".into(),
            side_effects: vec![SideEffectClass::FileWrite, SideEffectClass::ProcessSpawn],
            build_impact_class: BuildImpactClass::WorkspaceWide,
            final_submit_label: "Create workspace on desktop".into(),
            save_and_resume_disclosed: true,
        },
        Some(WizardStepState {
            current_step: 1,
            total_steps: 3,
            blocked_prerequisites: vec!["Desktop handoff required for destination folder".into()],
            save_and_resume_visible: true,
            final_submit_label: "Create workspace on desktop".into(),
        }),
        vec![ClientLimitation {
            field_id: "scaffold.destination".into(),
            reason: "Local destination picker is unavailable in browser companion.".into(),
            disclosed_before_final_step: true,
        }],
    );
    if let Some(field) = record
        .record
        .fields
        .iter_mut()
        .find(|field| field.field_id == "scaffold.destination")
    {
        field.path_truth = Some(PathFieldTruth {
            basis: PathBasis::WorkspaceRelative,
            location_class: PathLocationClass::Local,
            displayed_path: "desktop handoff required".into(),
            basis_ref: "handoff://desktop/scaffold-destination".into(),
        });
    }
    record
}

fn restricted_client_provider_account() -> FormTruthScenario {
    let mut account = field(
        "provider.account",
        "Provider account",
        "organization account requires desktop authority",
        RequirementState::Required,
        FieldKind::ObjectReference,
        SourcePrecedence::Workspace,
        ApplyTiming::PreviewFirst,
        validation(
            ValidationClass::RemoteAuth,
            ValidationState::Blocked,
            "Restricted client can inspect the account but cannot complete provider authorization.",
            true,
        ),
    );
    account.reference_truth = Some(ReferenceFieldTruth {
        display_label: "organization account".into(),
        stable_id_path: "provider/account/org-primary".into(),
    });
    scenario(
        "forms-truth:restricted-client-provider-account",
        "restricted_client_provider_account.json",
        FormSurfaceClass::ProviderAccountFlow,
        ClientScope::RestrictedClient,
        "Provider account",
        "organization provider account",
        vec![account],
        vec![audit(
            "provider.account",
            SourcePrecedence::Workspace,
            "organization account",
            "Workspace account reference remains visible, but restricted authority blocks authorization here.",
            vec![
                candidate(SourcePrecedence::Workspace, "organization account", 60, true, None),
                candidate(
                    SourcePrecedence::UserOverride,
                    "personal account",
                    80,
                    false,
                    Some("restricted client block"),
                ),
            ],
        )],
        StagedApplyPacket {
            apply_timing: ApplyTiming::PreviewFirst,
            dirty: true,
            dirty_scopes: vec!["provider account".into()],
            dirty_state_persisted_across: persisted_across(),
            review_sheet_ref: Some("aureline://review-sheet/provider-account-restricted".into()),
            checkpoint_ref: Some("checkpoint://provider-account/restricted-client".into()),
            revert_action_label: Some("Discard account change".into()),
            target_scope: "Provider authorization after desktop handoff".into(),
            side_effects: vec![SideEffectClass::NetworkCall],
            build_impact_class: BuildImpactClass::RemoteOrProvider,
            final_submit_label: "Authorize on desktop".into(),
            save_and_resume_disclosed: true,
        },
        Some(WizardStepState {
            current_step: 2,
            total_steps: 3,
            blocked_prerequisites: vec!["Desktop authorization required".into()],
            save_and_resume_visible: true,
            final_submit_label: "Authorize on desktop".into(),
        }),
        vec![ClientLimitation {
            field_id: "provider.account".into(),
            reason: "Restricted client cannot complete provider authorization.".into(),
            disclosed_before_final_step: true,
        }],
    )
}

fn scenario(
    scenario_id: &'static str,
    fixture_filename: &'static str,
    surface_class: FormSurfaceClass,
    client_scope: ClientScope,
    title: &str,
    affected_scope: &str,
    fields: Vec<FieldRowContract>,
    precedence_audits: Vec<SourcePrecedenceAudit>,
    staged_apply: StagedApplyPacket,
    wizard_state: Option<WizardStepState>,
    client_limitations: Vec<ClientLimitation>,
) -> FormTruthScenario {
    let apply_timing = staged_apply.apply_timing;
    let record = FormTruthPacketRecord {
        record_kind: FORM_TRUTH_RECORD_KIND.into(),
        schema_version: FORM_TRUTH_SCHEMA_VERSION,
        notice: FORM_TRUTH_NOTICE.into(),
        shared_contract_ref: FORM_TRUTH_SHARED_CONTRACT_REF.into(),
        packet_id: scenario_id.into(),
        as_of: CORPUS_AS_OF.into(),
        surface_class,
        client_scope,
        title: title.into(),
        affected_scope: affected_scope.into(),
        fields,
        precedence_audits,
        staged_apply,
        wizard_state,
        client_limitations,
        accessibility_review: AccessibilityReview {
            keyboard_operable: true,
            screen_reader_operable: true,
            ime_safe: true,
            rtl_safe: true,
            reduced_motion_safe: true,
            focus_return_target: "active field row".into(),
        },
        support_export_lines: vec![
            format!("form_truth: {scenario_id}"),
            format!("surface: {}", surface_class.as_str()),
            format!("client_scope: {}", client_scope.as_str()),
            format!("apply_timing: {}", apply_timing.as_str()),
            "raw_secret_values: omitted".into(),
        ],
    };
    FormTruthScenario {
        scenario_id,
        fixture_filename,
        expected_apply_timing: apply_timing,
        expected_client_scope: client_scope,
        expected_surface_class: surface_class,
        record,
    }
}

fn field(
    field_id: &str,
    label: &str,
    value: &str,
    requirement: RequirementState,
    field_kind: FieldKind,
    source_class: SourcePrecedence,
    apply_timing: ApplyTiming,
    validation: ValidationResult,
) -> FieldRowContract {
    FieldRowContract {
        field_id: field_id.into(),
        label: label.into(),
        current_value_summary: value.into(),
        requirement,
        field_kind,
        source_class,
        source_label: source_class.display_label().into(),
        validation,
        apply_timing,
        actions: vec![
            FieldAction {
                action_class: FieldActionClass::OpenDetails,
                label: "Open details".into(),
            },
            FieldAction {
                action_class: FieldActionClass::Reset,
                label: "Reset".into(),
            },
        ],
        secret_truth: None,
        path_truth: None,
        reference_truth: None,
        code_backed_truth: None,
    }
}

fn path_field(
    field_id: &str,
    label: &str,
    value: &str,
    source_class: SourcePrecedence,
    apply_timing: ApplyTiming,
    basis: PathBasis,
    location_class: PathLocationClass,
    basis_ref: &str,
) -> FieldRowContract {
    let mut row = field(
        field_id,
        label,
        value,
        RequirementState::Required,
        FieldKind::Path,
        source_class,
        apply_timing,
        validation(
            ValidationClass::EnvironmentProbe,
            ValidationState::Ready,
            "Path resolves in the declared basis.",
            false,
        ),
    );
    row.path_truth = Some(PathFieldTruth {
        basis,
        location_class,
        displayed_path: value.into(),
        basis_ref: basis_ref.into(),
    });
    row
}

fn secret_field(
    field_id: &str,
    label: &str,
    value: &str,
    source_class: SourcePrecedence,
    apply_timing: ApplyTiming,
    storage_mode: SecretStorageMode,
) -> FieldRowContract {
    let mut row = field(
        field_id,
        label,
        value,
        RequirementState::Required,
        FieldKind::Secret,
        source_class,
        apply_timing,
        validation(
            ValidationClass::RemoteAuth,
            ValidationState::Ready,
            "Secret reference is valid without exporting the raw value.",
            false,
        ),
    );
    row.actions.push(FieldAction {
        action_class: FieldActionClass::Revoke,
        label: "Revoke reference".into(),
    });
    row.secret_truth = Some(SecretFieldTruth {
        storage_mode,
        reveal_friction: "Require explicit reveal and recent authentication.".into(),
        copy_warning: "Copying may expose sensitive material outside Aureline.".into(),
        clear_or_revoke_action: "Revoke reference".into(),
        export_behavior: SecretExportBehavior::ReferenceOnly,
    });
    row
}

fn reference_field(
    field_id: &str,
    label: &str,
    value: &str,
    source_class: SourcePrecedence,
    apply_timing: ApplyTiming,
    stable_id_path: &str,
) -> FieldRowContract {
    let mut row = field(
        field_id,
        label,
        value,
        RequirementState::Required,
        FieldKind::ObjectReference,
        source_class,
        apply_timing,
        validation(
            ValidationClass::RemoteAuth,
            ValidationState::Ready,
            "Referenced provider object is available for this account.",
            false,
        ),
    );
    row.reference_truth = Some(ReferenceFieldTruth {
        display_label: value.into(),
        stable_id_path: stable_id_path.into(),
    });
    row
}

fn code_backed_field(
    field_id: &str,
    label: &str,
    value: &str,
    source_class: SourcePrecedence,
    apply_timing: ApplyTiming,
) -> FieldRowContract {
    let mut row = field(
        field_id,
        label,
        value,
        RequirementState::Required,
        FieldKind::CodeBacked,
        source_class,
        apply_timing,
        validation(
            ValidationClass::DryRunPreview,
            ValidationState::Ready,
            "Dry-run preview is available before broad-impact writes.",
            false,
        ),
    );
    row.code_backed_truth = Some(CodeBackedFieldTruth {
        diff_preview_ref: "aureline://diff/provider-publish-comments".into(),
        preserves_comments: true,
        preserves_unknown_fields: true,
        broad_impact_disclosure: "Preview shows all generated comment writes before publish."
            .into(),
    });
    row
}

fn validation(
    validation_class: ValidationClass,
    state: ValidationState,
    message: &str,
    blocks_submit: bool,
) -> ValidationResult {
    ValidationResult {
        validation_class,
        state,
        message: message.into(),
        blocks_submit,
        last_known_state: None,
        last_known_message: None,
        validation_epoch: 2,
        target_epoch: 2,
        dependent_inputs: vec![],
        invalidates_on_target_change: false,
    }
}

fn pending_validation() -> ValidationResult {
    ValidationResult {
        validation_class: ValidationClass::EnvironmentProbe,
        state: ValidationState::Pending,
        message: "Remote toolchain probe is running; last known compatible result remains visible."
            .into(),
        blocks_submit: false,
        last_known_state: Some(ValidationState::Ready),
        last_known_message: Some(
            "Remote toolchain was compatible on the previous target epoch.".into(),
        ),
        validation_epoch: 3,
        target_epoch: 4,
        dependent_inputs: vec!["remote.target".into(), "toolchain.channel".into()],
        invalidates_on_target_change: true,
    }
}

fn candidate(
    source_class: SourcePrecedence,
    value_summary: &str,
    precedence_rank: u32,
    effective: bool,
    superseded_by: Option<&str>,
) -> PrecedenceCandidate {
    PrecedenceCandidate {
        source_class,
        source_label: source_class.display_label().into(),
        value_summary: value_summary.into(),
        precedence_rank,
        effective,
        superseded_by: superseded_by.map(str::to_owned),
    }
}

fn audit(
    field_id: &str,
    effective_source: SourcePrecedence,
    effective_value_summary: &str,
    winning_reason: &str,
    candidates: Vec<PrecedenceCandidate>,
) -> SourcePrecedenceAudit {
    SourcePrecedenceAudit {
        field_id: field_id.into(),
        effective_source,
        effective_value_summary: effective_value_summary.into(),
        winning_reason: winning_reason.into(),
        candidates,
    }
}

fn staged_packet(
    final_submit_label: &str,
    build_impact_class: BuildImpactClass,
) -> StagedApplyPacket {
    StagedApplyPacket {
        apply_timing: ApplyTiming::Staged,
        dirty: true,
        dirty_scopes: vec!["settings".into()],
        dirty_state_persisted_across: persisted_across(),
        review_sheet_ref: Some("aureline://review-sheet/staged-settings".into()),
        checkpoint_ref: Some("checkpoint://settings/staged".into()),
        revert_action_label: Some("Revert staged changes".into()),
        target_scope: "Current workspace".into(),
        side_effects: vec![SideEffectClass::FileWrite],
        build_impact_class,
        final_submit_label: final_submit_label.into(),
        save_and_resume_disclosed: true,
    }
}

fn persisted_across() -> Vec<String> {
    PERSISTED_ACROSS
        .iter()
        .map(|value| (*value).into())
        .collect()
}
