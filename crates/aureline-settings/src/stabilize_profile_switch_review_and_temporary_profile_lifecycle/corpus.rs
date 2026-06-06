//! Deterministic corpus for profile-switch lifecycle certification.

use super::model::{
    validate_profile_switch_lifecycle_record, ApplyAuditRow, ApplyTimingClass,
    ArtifactExclusionClass, ConflictSourceClass, ExcludedMachineStateRow, ImportConflictReviewRow,
    LocalAuthoritativeReason, NarrowingEffectRow, ProfileArtifactBoundaryRow, ProfileCardRow,
    ProfileDurabilityClass, ProfileScopeClass, ProfileSourceClass, ProfileSwitchDeltaRow,
    ProfileSwitchLifecycleCertification, ProfileSwitchLifecyclePillars,
    ProfileSwitchLifecycleQualification, ProfileSwitchNarrowingReason, ProfileSwitchReviewSheet,
    StableClaimClass, SurfaceClass, SurfaceTruthRow, SyncFallbackRow, TemporaryProfileActionClass,
    TemporaryProfileActionRow, TemporaryProfileLifecycle, PROFILE_SWITCH_REVIEW_RECORD_KIND,
    PROFILE_SWITCH_REVIEW_SCHEMA_VERSION, PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-06-06T12:00:00Z";

const RECORD_REF: &str = "aureline://settings/profile-switch-lifecycle";
const ACTIVE_PROFILE_REF: &str = "aureline://profile/daily-rust-work";
const TEMP_PROFILE_REF: &str = "aureline://profile/troubleshooting-session";
const IMPORTED_PROFILE_REF: &str = "aureline://profile/imported-vscode-dark";
const SYNC_PROFILE_REF: &str = "aureline://profile/synced-laptop-copy";
const CHECKPOINT_REF: &str = "aureline://checkpoint/profile-switch-daily-to-troubleshooting";
const IMPORT_CHECKPOINT_REF: &str = "aureline://checkpoint/imported-profile-apply";
const SYNC_CHECKPOINT_REF: &str = "aureline://checkpoint/sync-profile-merge";
const CHANGE_SUMMARY_REF: &str =
    "aureline://change-summary/profile-switch-daily-to-troubleshooting";
const ROLLBACK_BANNER_REF: &str =
    "aureline://rollback-banner/profile-switch-daily-to-troubleshooting";

/// One scenario in the profile-switch lifecycle corpus.
#[derive(Debug, Clone)]
pub struct ProfileSwitchLifecycleScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    record: ProfileSwitchLifecycleCertification,
}

impl ProfileSwitchLifecycleScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> ProfileSwitchLifecycleCertification {
        self.record.clone()
    }
}

/// Returns the deterministic corpus for profile-switch lifecycle certification.
pub fn profile_switch_lifecycle_corpus() -> Vec<ProfileSwitchLifecycleScenario> {
    vec![
        scenario(
            "stable-daily-switch-review",
            "stable-daily-switch-review.json",
            "Daily profile switch review distinguishes live changes, restart-required deltas, machine-local exclusions, narrowing effects, and rollback checkpoint creation.",
            None,
        ),
        scenario(
            "temporary-troubleshooting-lifecycle",
            "temporary-troubleshooting-lifecycle.json",
            "Troubleshooting profile remains visibly temporary, exposes discard, promote, and compare actions, and restricts persistence until promotion review.",
            None,
        ),
        scenario(
            "local-authoritative-sync-fallback",
            "local-authoritative-sync-fallback.json",
            "Stale, unavailable, undecryptable, and policy-denied sync data degrade to local-authoritative file portability with no hidden cloud authority.",
            None,
        ),
        scenario(
            "widening-import-refused",
            "widening-import-refused.json",
            "Imported profile tries to widen network egress and extension authority, but the field-aware review refuses the widening and keeps rollback truth inspectable.",
            None,
        ),
        scenario(
            "missing-rollback-drill",
            "missing-rollback-drill.json",
            "A drill posture omits one rollback checkpoint so the lane narrows below Stable instead of inheriting the green path.",
            Some(ProfileSwitchNarrowingReason::RollbackCheckpointMissing),
        ),
    ]
}

fn scenario(
    scenario_id: &'static str,
    fixture_filename: &str,
    summary: &str,
    narrowing: Option<ProfileSwitchNarrowingReason>,
) -> ProfileSwitchLifecycleScenario {
    let stable = narrowing.is_none();
    let record = build_record(scenario_id, summary, narrowing);
    validate_profile_switch_lifecycle_record(&record)
        .unwrap_or_else(|errors| panic!("{scenario_id} invalid: {errors:?}"));
    ProfileSwitchLifecycleScenario {
        scenario_id,
        fixture_filename: fixture_filename.to_owned(),
        expected_claim_class: if stable {
            StableClaimClass::Stable
        } else {
            StableClaimClass::Beta
        },
        expected_qualifies_stable: stable,
        record,
    }
}

fn build_record(
    record_id: &str,
    summary: &str,
    narrowing: Option<ProfileSwitchNarrowingReason>,
) -> ProfileSwitchLifecycleCertification {
    let mut apply_audit = apply_audit_rows();
    if narrowing == Some(ProfileSwitchNarrowingReason::RollbackCheckpointMissing) {
        if let Some(row) = apply_audit
            .iter_mut()
            .find(|row| row.apply_id == "sync-apply")
        {
            row.rollback_checkpoint_ref = None;
            row.rollback_inspectable = false;
        }
    }

    let pillars = derive_pillars(&apply_audit, narrowing);
    let stable_qualification = derive_qualification(&pillars, narrowing);
    ProfileSwitchLifecycleCertification {
        record_kind: PROFILE_SWITCH_REVIEW_RECORD_KIND.to_owned(),
        schema_version: PROFILE_SWITCH_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        record_id: record_id.to_owned(),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: summary.to_owned(),
        profile_cards: profile_cards(),
        switch_review: switch_review_sheet(),
        temporary_profile: temporary_profile_lifecycle(),
        artifact_boundaries: artifact_boundaries(),
        import_conflicts: import_conflicts(),
        apply_audit,
        sync_fallbacks: sync_fallbacks(),
        surface_truth: surface_truth(),
        pillars,
        stable_qualification,
    }
}

fn derive_pillars(
    apply_audit: &[ApplyAuditRow],
    narrowing: Option<ProfileSwitchNarrowingReason>,
) -> ProfileSwitchLifecyclePillars {
    let rollback_checkpoints_created = narrowing
        != Some(ProfileSwitchNarrowingReason::RollbackCheckpointMissing)
        && apply_audit.iter().all(|row| {
            !row.durable_state_changed
                || (row.rollback_checkpoint_ref.is_some() && row.rollback_inspectable)
        });
    ProfileSwitchLifecyclePillars {
        switch_review_complete: true,
        temporary_lifecycle_complete: true,
        artifact_boundary_held: true,
        import_conflicts_non_widening: true,
        rollback_checkpoints_created,
        local_authoritative_fallback_visible: true,
        surfaces_share_truth: true,
    }
}

fn derive_qualification(
    pillars: &ProfileSwitchLifecyclePillars,
    narrowing: Option<ProfileSwitchNarrowingReason>,
) -> ProfileSwitchLifecycleQualification {
    let all_pillars = pillars.switch_review_complete
        && pillars.temporary_lifecycle_complete
        && pillars.artifact_boundary_held
        && pillars.import_conflicts_non_widening
        && pillars.rollback_checkpoints_created
        && pillars.local_authoritative_fallback_visible
        && pillars.surfaces_share_truth;
    if all_pillars && narrowing.is_none() {
        ProfileSwitchLifecycleQualification {
            claim_class: StableClaimClass::Stable,
            qualifies_stable: true,
            narrowing_reasons: Vec::new(),
        }
    } else {
        ProfileSwitchLifecycleQualification {
            claim_class: StableClaimClass::Beta,
            qualifies_stable: false,
            narrowing_reasons: vec![
                narrowing.unwrap_or(ProfileSwitchNarrowingReason::RollbackCheckpointMissing)
            ],
        }
    }
}

fn profile_cards() -> Vec<ProfileCardRow> {
    vec![
        ProfileCardRow {
            profile_ref: ACTIVE_PROFILE_REF.to_owned(),
            name: "Daily Rust Work".to_owned(),
            purpose: "Durable editing profile for Rust workspaces.".to_owned(),
            source_class: ProfileSourceClass::DurableUserProfile,
            durability_class: ProfileDurabilityClass::Durable,
            included_scopes: vec![
                ProfileScopeClass::VisualAppearance,
                ProfileScopeClass::Keybindings,
                ProfileScopeClass::Snippets,
                ProfileScopeClass::ExtensionSelection,
                ProfileScopeClass::Layout,
                ProfileScopeClass::TasksAndLaunch,
            ],
            exportable_without_secrets: true,
            visible_state_badge: true,
        },
        ProfileCardRow {
            profile_ref: TEMP_PROFILE_REF.to_owned(),
            name: "Troubleshooting Session".to_owned(),
            purpose: "Session-only profile used to isolate extension and egress behavior."
                .to_owned(),
            source_class: ProfileSourceClass::TroubleshootingProfile,
            durability_class: ProfileDurabilityClass::DiscardedOnExit,
            included_scopes: vec![
                ProfileScopeClass::VisualAppearance,
                ProfileScopeClass::ExtensionSelection,
                ProfileScopeClass::AiAndNetworkDefaults,
            ],
            exportable_without_secrets: true,
            visible_state_badge: true,
        },
        ProfileCardRow {
            profile_ref: IMPORTED_PROFILE_REF.to_owned(),
            name: "Imported VS Code Dark".to_owned(),
            purpose: "Imported profile artifact pending field-aware review.".to_owned(),
            source_class: ProfileSourceClass::ImportedProfile,
            durability_class: ProfileDurabilityClass::InspectOnly,
            included_scopes: vec![
                ProfileScopeClass::VisualAppearance,
                ProfileScopeClass::Keybindings,
                ProfileScopeClass::Snippets,
                ProfileScopeClass::ExtensionSelection,
            ],
            exportable_without_secrets: true,
            visible_state_badge: true,
        },
        ProfileCardRow {
            profile_ref: SYNC_PROFILE_REF.to_owned(),
            name: "Synced Laptop Copy".to_owned(),
            purpose: "Optional synced profile payload degraded to local-authoritative review."
                .to_owned(),
            source_class: ProfileSourceClass::SyncedProfile,
            durability_class: ProfileDurabilityClass::InspectOnly,
            included_scopes: vec![
                ProfileScopeClass::VisualAppearance,
                ProfileScopeClass::Keybindings,
                ProfileScopeClass::AiAndNetworkDefaults,
            ],
            exportable_without_secrets: true,
            visible_state_badge: true,
        },
    ]
}

fn switch_review_sheet() -> ProfileSwitchReviewSheet {
    ProfileSwitchReviewSheet {
        from_profile_ref: ACTIVE_PROFILE_REF.to_owned(),
        to_profile_ref: TEMP_PROFILE_REF.to_owned(),
        immediate_changes: vec![
            delta(
                "theme-live",
                "ui.theme",
                ProfileScopeClass::VisualAppearance,
                "Solar".to_owned(),
                "High Contrast Dark".to_owned(),
                ApplyTimingClass::Immediate,
                "Applies live to open views.".to_owned(),
                false,
                false,
                true,
            ),
            delta(
                "keymap-live",
                "keybindings.primary_preset",
                ProfileScopeClass::Keybindings,
                "aureline-default".to_owned(),
                "vscode-compatible".to_owned(),
                ApplyTimingClass::Immediate,
                "Applies live to command dispatch.".to_owned(),
                false,
                false,
                true,
            ),
            delta(
                "snippet-live",
                "snippets.rust.enabled_set",
                ProfileScopeClass::Snippets,
                "daily-rust".to_owned(),
                "minimal-troubleshooting".to_owned(),
                ApplyTimingClass::Immediate,
                "Applies live to snippet picker.".to_owned(),
                false,
                false,
                true,
            ),
            delta(
                "layout-live",
                "layout.side_panel_default",
                ProfileScopeClass::Layout,
                "project-and-tests".to_owned(),
                "diagnostics-only".to_owned(),
                ApplyTimingClass::Immediate,
                "Applies live to the current shell.".to_owned(),
                false,
                false,
                true,
            ),
        ],
        restart_required_changes: vec![
            delta(
                "extension-host-restart",
                "extensions.enabled_selection",
                ProfileScopeClass::ExtensionSelection,
                "rust-analyzer, crates, gitlens".to_owned(),
                "rust-analyzer only".to_owned(),
                ApplyTimingClass::RestartRequired,
                "Extension host restart required.".to_owned(),
                true,
                false,
                true,
            ),
            delta(
                "ai-egress-guarded",
                "security.ai.egress_policy",
                ProfileScopeClass::AiAndNetworkDefaults,
                "approved hosted providers".to_owned(),
                "deny all hosted providers".to_owned(),
                ApplyTimingClass::RestartRequired,
                "AI route services restart required.".to_owned(),
                true,
                false,
                true,
            ),
        ],
        excluded_machine_state: vec![
            ExcludedMachineStateRow {
                exclusion_id: "display-binding".to_owned(),
                state_class: "window.display_binding".to_owned(),
                reason: "Display ids and monitor geometry are machine-local.".to_owned(),
                separate_addendum_required: true,
            },
            ExcludedMachineStateRow {
                exclusion_id: "trust-store-pointer".to_owned(),
                state_class: "security.os_trust_store_pointer".to_owned(),
                reason: "Trust-store pointers are machine-unique authority.".to_owned(),
                separate_addendum_required: true,
            },
        ],
        narrowing_effects: vec![
            NarrowingEffectRow {
                effect_id: "network-egress-narrowed".to_owned(),
                affected_area: "AI/network egress".to_owned(),
                explanation:
                    "Hosted-provider egress narrows to deny-all for the troubleshooting session."
                        .to_owned(),
                narrows_only: true,
            },
            NarrowingEffectRow {
                effect_id: "extension-authority-narrowed".to_owned(),
                affected_area: "Extension behavior".to_owned(),
                explanation: "Extension selection narrows to the minimal signed set.".to_owned(),
                narrows_only: true,
            },
        ],
        durable_state_changes_materially: true,
        creates_rollback_checkpoint: true,
        rollback_checkpoint_ref: Some(CHECKPOINT_REF.to_owned()),
        change_summary_ref: CHANGE_SUMMARY_REF.to_owned(),
        rollback_banner_ref: ROLLBACK_BANNER_REF.to_owned(),
    }
}

fn delta(
    delta_id: &str,
    field_path: &str,
    scope_class: ProfileScopeClass,
    before_value_preview: String,
    after_value_preview: String,
    apply_timing: ApplyTimingClass,
    restart_posture: String,
    narrows_behavior: bool,
    would_widen_authority: bool,
    widening_refused_or_reviewed: bool,
) -> ProfileSwitchDeltaRow {
    ProfileSwitchDeltaRow {
        delta_id: delta_id.to_owned(),
        field_path: field_path.to_owned(),
        scope_class,
        before_value_preview,
        after_value_preview,
        apply_timing,
        restart_posture,
        narrows_behavior,
        would_widen_authority,
        widening_refused_or_reviewed,
    }
}

fn temporary_profile_lifecycle() -> TemporaryProfileLifecycle {
    TemporaryProfileLifecycle {
        profile_ref: TEMP_PROFILE_REF.to_owned(),
        source_class: ProfileSourceClass::TroubleshootingProfile,
        durability_class: ProfileDurabilityClass::DiscardedOnExit,
        badge_label: "Troubleshooting profile - discarded on exit unless promoted".to_owned(),
        lifetime_or_expiry: "Current session; expires when the shell exits.".to_owned(),
        restricted_persistence_rules: vec![
            "Session edits are not written to the durable profile until Promote opens a scoped review.".to_owned(),
            "Secrets, trust approvals, and machine-local bindings cannot be promoted.".to_owned(),
            "Discard removes session-only values and leaves the durable profile unchanged.".to_owned(),
        ],
        actions: vec![
            TemporaryProfileActionRow {
                action_class: TemporaryProfileActionClass::Discard,
                target_ref: "aureline://action/profile-discard-troubleshooting".to_owned(),
                keyboard_reachable: true,
                persistence_effect_visible: true,
            },
            TemporaryProfileActionRow {
                action_class: TemporaryProfileActionClass::Promote,
                target_ref: "aureline://action/profile-promote-troubleshooting".to_owned(),
                keyboard_reachable: true,
                persistence_effect_visible: true,
            },
            TemporaryProfileActionRow {
                action_class: TemporaryProfileActionClass::CompareToDurableProfile,
                target_ref: "aureline://action/profile-compare-troubleshooting".to_owned(),
                keyboard_reachable: true,
                persistence_effect_visible: true,
            },
        ],
        state_boundary_visible: true,
    }
}

fn artifact_boundaries() -> Vec<ProfileArtifactBoundaryRow> {
    vec![ProfileArtifactBoundaryRow {
        artifact_ref: "aureline://artifact/profile-daily-rust-work".to_owned(),
        artifact_shape: "*.aureprofile.json".to_owned(),
        schema_version: "profile-artifact.schema.v1".to_owned(),
        text_based: true,
        diffable: true,
        exportable_without_forbidden_material: true,
        excluded_classes: ArtifactExclusionClass::REQUIRED.to_vec(),
    }]
}

fn import_conflicts() -> Vec<ImportConflictReviewRow> {
    vec![
        ImportConflictReviewRow {
            conflict_id: "import-egress-widening-refused".to_owned(),
            source_class: ConflictSourceClass::ImportedProfile,
            field_path: "security.ai.egress_policy".to_owned(),
            local_value_preview: "approved hosted providers".to_owned(),
            incoming_value_preview: "any public endpoint".to_owned(),
            effective_value_preview: "approved hosted providers".to_owned(),
            field_aware: true,
            scope_aware: true,
            would_widen_authority: true,
            widening_refused: true,
            local_authoritative_reason: Some(LocalAuthoritativeReason::PolicyDenied),
            offered_choices: vec![
                "keep local".to_owned(),
                "review narrower import".to_owned(),
                "decline imported field".to_owned(),
            ],
        },
        ImportConflictReviewRow {
            conflict_id: "sync-stale-keybindings-local-wins".to_owned(),
            source_class: ConflictSourceClass::SyncedProfile,
            field_path: "keybindings.primary_preset".to_owned(),
            local_value_preview: "aureline-default revision 18".to_owned(),
            incoming_value_preview: "aureline-default revision 12".to_owned(),
            effective_value_preview: "aureline-default revision 18".to_owned(),
            field_aware: true,
            scope_aware: true,
            would_widen_authority: false,
            widening_refused: false,
            local_authoritative_reason: Some(LocalAuthoritativeReason::StaleRemote),
            offered_choices: vec![
                "keep local".to_owned(),
                "compare remote".to_owned(),
                "import from file".to_owned(),
            ],
        },
        ImportConflictReviewRow {
            conflict_id: "extension-permission-widening-refused".to_owned(),
            source_class: ConflictSourceClass::ImportedProfile,
            field_path: "extensions.permission_set".to_owned(),
            local_value_preview: "minimal signed set".to_owned(),
            incoming_value_preview: "debugger and filesystem broad write".to_owned(),
            effective_value_preview: "minimal signed set".to_owned(),
            field_aware: true,
            scope_aware: true,
            would_widen_authority: true,
            widening_refused: true,
            local_authoritative_reason: Some(LocalAuthoritativeReason::PolicyDenied),
            offered_choices: vec![
                "keep local".to_owned(),
                "open extension review".to_owned(),
                "decline imported extension permissions".to_owned(),
            ],
        },
    ]
}

fn apply_audit_rows() -> Vec<ApplyAuditRow> {
    vec![
        ApplyAuditRow {
            apply_id: "profile-switch-apply".to_owned(),
            apply_source: "switch".to_owned(),
            durable_state_changed: true,
            change_summary_ref: CHANGE_SUMMARY_REF.to_owned(),
            rollback_checkpoint_ref: Some(CHECKPOINT_REF.to_owned()),
            rollback_inspectable: true,
        },
        ApplyAuditRow {
            apply_id: "import-apply".to_owned(),
            apply_source: "import".to_owned(),
            durable_state_changed: true,
            change_summary_ref: "aureline://change-summary/imported-profile-review".to_owned(),
            rollback_checkpoint_ref: Some(IMPORT_CHECKPOINT_REF.to_owned()),
            rollback_inspectable: true,
        },
        ApplyAuditRow {
            apply_id: "sync-apply".to_owned(),
            apply_source: "sync".to_owned(),
            durable_state_changed: true,
            change_summary_ref: "aureline://change-summary/sync-profile-merge".to_owned(),
            rollback_checkpoint_ref: Some(SYNC_CHECKPOINT_REF.to_owned()),
            rollback_inspectable: true,
        },
    ]
}

fn sync_fallbacks() -> Vec<SyncFallbackRow> {
    LocalAuthoritativeReason::REQUIRED
        .iter()
        .copied()
        .map(|reason| SyncFallbackRow {
            reason,
            local_durable_state_authoritative: true,
            file_based_portability_visible: true,
            no_hidden_cloud_authority_claim: true,
        })
        .collect()
}

fn surface_truth() -> Vec<SurfaceTruthRow> {
    SurfaceClass::REQUIRED
        .iter()
        .copied()
        .map(|surface_class| SurfaceTruthRow {
            surface_class,
            record_ref: RECORD_REF.to_owned(),
            consumes_shared_contract: true,
            shows_profile_state: true,
            shows_restart_delta_truth: true,
            shows_rollback_checkpoint: true,
            shows_local_authoritative_fallback: true,
        })
        .collect()
}
