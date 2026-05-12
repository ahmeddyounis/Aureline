//! Unit and fixture tests for the bounded typed-permission-prompt
//! wedge.
//!
//! Coverage:
//!
//! - the protected walk where the user opens an install-review row for
//!   a verified-publisher extension, the prompt offers approve and
//!   deny with explicit persistence / deny-path semantics, and a
//!   subsequent approve transitions the prompt to a clean approved
//!   state without firing any invariant;
//! - the protected walk where the user opens an unverified-publisher
//!   review-only row, the prompt suppresses approve (no auto-admit),
//!   only deny / details / safer alternatives are offered, and a
//!   subsequent deny transitions the prompt to a clean denied state;
//! - the named failure drill where a buggy caller mints a prompt
//!   against an install-review card that already carries typed
//!   invariants AND strips the actor identity, the deny-path label,
//!   and the persistence label — the wedge surfaces every typed
//!   invariant rather than collapsing to a generic "Allow?";
//! - adjacent drills covering offering approve on a denied fact-grid
//!   decision, suppressing the deny path, declaring approved while
//!   blocked, and grant-persistence-label inconsistency;
//! - serde round-trip and deterministic plaintext rendering;
//! - fixture replay end-to-end against three JSON fixtures under
//!   `fixtures/permissions/publish_or_ecosystem_cases/`.

use std::path::Path;

use serde::Deserialize;

use aureline_extensions::manifest_baseline::{
    DeclaredVsEffectiveDiffEntry, EffectivePermissionBaselineRecord, EffectivePermissionDiffClass,
    ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord, HostContractFamilyClass,
    InstallDecisionClass, InstallDecisionReasonClass, ManifestInstallDecisionRecord,
    ManifestOriginSourceClass, ManifestScopeCompletenessClass, PermissionScopeClass,
    PermissionScopeEntry, PublisherLifecycleStateClass, PublisherTrustTierClass, RedactionClass,
    SummaryFreshnessClass, EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND,
    EXTENSION_MANIFEST_BASELINE_RECORD_KIND, EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
    MANIFEST_INSTALL_DECISION_RECORD_KIND,
};

use crate::install_review_fact_grid::{
    ActivationBudgetClass, InstallReviewFactGridWedge, RollbackPostureClass,
};

use super::*;

// ---------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------

fn verified_admit_manifest() -> ExtensionManifestBaselineRecord {
    ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_owned(),
        extension_identity: "acme-labs/prose-helper".to_owned(),
        extension_version: "1.4.2".to_owned(),
        extension_lifecycle_state_class: ExtensionLifecycleStateClass::Published,
        host_contract_family_class: HostContractFamilyClass::WasmComponentModel,
        manifest_origin_source_class: ManifestOriginSourceClass::PublicRegistry,
        origin_source_label: "public registry: registry.aureline.dev".to_owned(),
        publisher_identity_ref: "publisher:acme-labs".to_owned(),
        publisher_display_label: "Acme Labs".to_owned(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        publisher_lifecycle_state_class: PublisherLifecycleStateClass::Active,
        publisher_signing_key_ref: "key:acme-labs:ed25519:2026-q2".to_owned(),
        declared_permissions: vec![
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::FilesystemRead,
                scope_target: "workspace:/docs/**".to_owned(),
                scope_constraint: Some("read-only under declared workspace prefix".to_owned()),
                rationale_label: "Read prose documents for grammar suggestions.".to_owned(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::AiProviderAccess,
                scope_target: "connected-provider:ai:acme-default".to_owned(),
                scope_constraint: Some("requires user-configured provider link".to_owned()),
                rationale_label: "Use the user's AI provider to refine suggestions.".to_owned(),
            },
        ],
        manifest_scope_completeness_class: ManifestScopeCompletenessClass::Complete,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_effective() -> EffectivePermissionBaselineRecord {
    let manifest = verified_admit_manifest();
    EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: manifest
            .declared_permissions
            .iter()
            .map(|p| DeclaredVsEffectiveDiffEntry {
                scope_class: p.scope_class,
                scope_target: p.scope_target.clone(),
                diff_class: EffectivePermissionDiffClass::Unchanged,
                narrowing_reason_label: "unchanged".to_owned(),
            })
            .collect(),
        widening_attempted_blocked_count: 0,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_decision() -> ManifestInstallDecisionRecord {
    ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_owned(),
        install_decision_class: InstallDecisionClass::Admit,
        install_decision_reason_class: InstallDecisionReasonClass::AdmittedNoViolation,
        decision_summary: "Admitted: complete manifest, attributed publisher, no widening attempt."
            .to_owned(),
        decided_at: "2026-05-11T08:00:01Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_fact_grid_wedge() -> InstallReviewFactGridWedge {
    InstallReviewFactGridWedge::new(
        verified_admit_manifest(),
        verified_admit_effective(),
        verified_admit_decision(),
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    )
}

fn verified_admit_prompt_wedge() -> PermissionPromptWedge {
    PermissionPromptWedge::from_install_review_wedge(
        &verified_admit_fact_grid_wedge(),
        "permission_prompt.extension.acme_labs.prose_helper.install",
        PermissionPromptRequester {
            requester_class: RequesterClass::Extension,
            requester_class_token: RequesterClass::Extension.as_str().to_owned(),
            requester_ref: "extension:acme-labs/prose-helper".to_owned(),
            requester_display_label: "Acme Labs Prose Helper".to_owned(),
            request_origin_label:
                "Extension install/review wedge initiated by user on extensions surface."
                    .to_owned(),
        },
        PermissionPromptAuthorityOwner {
            issuer_class: AuthorityIssuerClass::Shell,
            issuer_class_token: AuthorityIssuerClass::Shell.as_str().to_owned(),
            issuer_source_ref: "source.shell.user_approval".to_owned(),
            issuer_source_label: "Shell — user-facing approval lane".to_owned(),
        },
        ScopeFilterClass::CurrentRoot,
        "Current workspace root (docs/) for read-only manifest scope.",
        GrantScopeClass::Workspace,
        "Workspace — grant remembered for this workspace until revoked.",
        PermissionPromptDenialBranch {
            degraded_capability_class: DegradedCapabilityClass::ReadOnlyInspectionContinues,
            degraded_capability_token: DegradedCapabilityClass::ReadOnlyInspectionContinues
                .as_str()
                .to_owned(),
            deny_path_label:
                "Local editing and read-only inspection of the manifest continue; extension is not enabled."
                    .to_owned(),
            preserved_work_refs: vec![
                "workspace.local_editing.current_root".to_owned(),
                "extension.manifest.metadata_only_view".to_owned(),
            ],
        },
        PermissionPromptQuestions {
            who_is_asking:
                "The Acme Labs Prose Helper extension manifest is asking for filesystem-read and AI-provider access."
                    .to_owned(),
            what_boundary:
                "Workspace docs/ filesystem read plus AI-provider access via the user's connected provider."
                    .to_owned(),
            why_needed:
                "The extension reads prose documents and asks the user's AI provider for grammar suggestions."
                    .to_owned(),
            what_changes_if_allowed:
                "The extension activates and gains workspace-scoped read access plus AI-provider use under the user's existing provider link."
                    .to_owned(),
            what_works_if_denied:
                "Local editing, manifest inspection, and read-only extension metadata remain available."
                    .to_owned(),
            grant_persistence_statement:
                "Workspace grant — remembered for this workspace only until revoked from Settings."
                    .to_owned(),
        },
    )
}

fn unverified_review_only_fact_grid_wedge() -> InstallReviewFactGridWedge {
    let mut manifest = verified_admit_manifest();
    manifest.manifest_baseline_id = "manifest_baseline:indie-author/quick-notes:0.2.0".to_owned();
    manifest.extension_identity = "indie-author/quick-notes".to_owned();
    manifest.extension_version = "0.2.0".to_owned();
    manifest.publisher_identity_ref = "publisher:indie-author".to_owned();
    manifest.publisher_display_label = "Indie Author".to_owned();
    manifest.publisher_trust_tier_class = PublisherTrustTierClass::UnverifiedPublisher;
    manifest.publisher_signing_key_ref = "key:indie-author:ed25519:2026-q1".to_owned();
    manifest.host_contract_family_class = HostContractFamilyClass::WasmCoreModule;
    manifest.declared_permissions = vec![PermissionScopeEntry {
        scope_class: PermissionScopeClass::UiCommandContribute,
        scope_target: "command:quick_notes.open".to_owned(),
        scope_constraint: Some("contributes one command id".to_owned()),
        rationale_label: "Contribute a command for opening the user's quick notes.".to_owned(),
    }];

    let effective = EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: vec![DeclaredVsEffectiveDiffEntry {
            scope_class: PermissionScopeClass::UiCommandContribute,
            scope_target: "command:quick_notes.open".to_owned(),
            diff_class: EffectivePermissionDiffClass::Unchanged,
            narrowing_reason_label: "unchanged".to_owned(),
        }],
        widening_attempted_blocked_count: 0,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let decision = ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        install_decision_class: InstallDecisionClass::ReviewOnly,
        install_decision_reason_class: InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher,
        decision_summary:
            "Review-only: unverified publisher; manifest landed for review without enabling."
                .to_owned(),
        decided_at: "2026-05-11T08:00:01Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::UninstallWithUserStateRetained,
    )
    .with_degraded(DegradedStateToken::Limited)
}

fn unverified_review_only_prompt_wedge() -> PermissionPromptWedge {
    PermissionPromptWedge::from_install_review_wedge(
        &unverified_review_only_fact_grid_wedge(),
        "permission_prompt.extension.indie_author.quick_notes.review",
        PermissionPromptRequester {
            requester_class: RequesterClass::Extension,
            requester_class_token: RequesterClass::Extension.as_str().to_owned(),
            requester_ref: "extension:indie-author/quick-notes".to_owned(),
            requester_display_label: "Indie Author Quick Notes".to_owned(),
            request_origin_label:
                "Extension install/review wedge — unverified publisher review path.".to_owned(),
        },
        PermissionPromptAuthorityOwner {
            issuer_class: AuthorityIssuerClass::Shell,
            issuer_class_token: AuthorityIssuerClass::Shell.as_str().to_owned(),
            issuer_source_ref: "source.shell.user_approval".to_owned(),
            issuer_source_label: "Shell — user-facing approval lane".to_owned(),
        },
        ScopeFilterClass::CurrentRoot,
        "Current workspace root — UI command contribution only.",
        GrantScopeClass::Once,
        "Once — review only; no remembered grant created.",
        PermissionPromptDenialBranch {
            degraded_capability_class: DegradedCapabilityClass::ReadOnlyInspectionContinues,
            degraded_capability_token: DegradedCapabilityClass::ReadOnlyInspectionContinues
                .as_str()
                .to_owned(),
            deny_path_label:
                "Manifest remains visible for review; the extension is not enabled.".to_owned(),
            preserved_work_refs: vec![
                "workspace.local_editing.current_root".to_owned(),
                "extension.manifest.metadata_only_view".to_owned(),
            ],
        },
        PermissionPromptQuestions {
            who_is_asking:
                "Indie Author Quick Notes (unverified publisher) is asking to contribute one UI command."
                    .to_owned(),
            what_boundary: "Workspace UI command contribution under the quick_notes namespace."
                .to_owned(),
            why_needed:
                "The extension wants to expose a command that opens the user's quick notes."
                    .to_owned(),
            what_changes_if_allowed:
                "Approval is suppressed for review-only rows; nothing changes until the publisher is verified."
                    .to_owned(),
            what_works_if_denied:
                "Local editing and manifest inspection remain available; the extension is not enabled."
                    .to_owned(),
            grant_persistence_statement:
                "Once — review-only path does not mint a remembered grant.".to_owned(),
        },
    )
    .with_degraded(DegradedStateToken::Limited)
}

// ---------------------------------------------------------------------------
// Protected walk: verified-publisher admit. Approve transitions to a clean
// approved card.
// ---------------------------------------------------------------------------

#[test]
fn protected_walk_verified_admit_pending_card_offers_approve_and_deny() {
    let wedge = verified_admit_prompt_wedge();
    let card = wedge.card();
    assert_eq!(card.record_kind, TYPED_PERMISSION_PROMPT_RECORD_KIND);
    assert_eq!(card.schema_version, TYPED_PERMISSION_PROMPT_SCHEMA_VERSION);
    assert_eq!(
        card.prototype_label_token,
        "m1_prototype_typed_permission_prompt"
    );
    assert!(card.install_review_card_clean);
    assert_eq!(card.upstream_install_decision_class_token, "admit");
    assert!(card.approve_action_offered);
    assert!(card.deny_action_offered);
    assert!(matches!(
        card.decision_state,
        PermissionPromptDecisionState::Pending
    ));
    let roles: Vec<&str> = card
        .decision_actions
        .iter()
        .map(|a| a.action_role_token.as_str())
        .collect();
    assert!(roles.contains(&"primary_approve"));
    assert!(roles.contains(&"primary_deny"));
    assert!(roles.contains(&"details"));
    assert!(
        card.invariants.is_empty(),
        "invariants: {:?}",
        card.invariants
    );
    let claim_tokens: Vec<&str> = card.claim_limits.iter().map(|r| r.token.as_str()).collect();
    assert_eq!(
        claim_tokens,
        vec![
            "single_bounded_wedge_only",
            "no_org_policy_pack_productization",
            "no_admin_approval_console",
            "no_multi_lane_permission_system",
        ]
    );
}

#[test]
fn protected_walk_verified_admit_then_approve_renders_clean_approved_card() {
    let card = verified_admit_prompt_wedge()
        .with_decision_state(PermissionPromptDecisionState::Approved)
        .card();
    assert!(card.is_clean_approve());
    assert!(card.invariants.is_empty());
    assert_eq!(card.decision_state_token, "approved");
    assert!(card.summary_line.contains("decision=admit"));
    assert!(card.summary_line.ends_with("approved"));
}

#[test]
fn protected_walk_verified_admit_then_deny_renders_clean_denied_card() {
    let card = verified_admit_prompt_wedge()
        .with_decision_state(PermissionPromptDecisionState::Denied)
        .card();
    assert!(card.is_clean_deny());
    assert!(card.invariants.is_empty());
    assert_eq!(card.decision_state_token, "denied");
}

#[test]
fn protected_walk_unverified_review_only_suppresses_approve() {
    let card = unverified_review_only_prompt_wedge().card();
    assert!(!card.approve_action_offered);
    assert!(card.deny_action_offered);
    assert_eq!(card.upstream_install_decision_class_token, "review_only");
    assert_eq!(card.degraded_token.as_deref(), Some("Limited"));
    // No invariant violations: a review-only path that suppresses
    // approve is an honest decision.
    assert!(
        card.invariants.is_empty(),
        "invariants: {:?}",
        card.invariants
    );
    let roles: Vec<&str> = card
        .decision_actions
        .iter()
        .map(|a| a.action_role_token.as_str())
        .collect();
    assert!(!roles.contains(&"primary_approve"));
    assert!(roles.contains(&"primary_deny"));
}

// ---------------------------------------------------------------------------
// Failure drill: incomplete authority facts must not collapse to generic copy.
// ---------------------------------------------------------------------------

#[test]
fn failure_drill_incomplete_authority_facts_refuses_to_collapse_to_generic_copy() {
    // Build an install-review wedge whose fact-grid record carries
    // typed invariants (the install-review failure drill from
    // M01-104). Then build a permission prompt against that card,
    // stripping the actor display label, the issuer source ref, the
    // deny-path label, the grant persistence label, and one prompt
    // question. The permission-prompt wedge MUST surface every typed
    // invariant rather than collapsing the prompt into a generic
    // "Allow?".
    let mut manifest = verified_admit_manifest();
    manifest.publisher_identity_ref = String::new();
    manifest.manifest_origin_source_class = ManifestOriginSourceClass::UnknownSourceClass;
    manifest.origin_source_label = "(unknown)".to_owned();
    if let Some(row) = manifest.declared_permissions.get_mut(0) {
        row.rationale_label = String::new();
    }
    manifest.manifest_scope_completeness_class =
        ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing;
    let mut effective = verified_admit_effective();
    effective.manifest_baseline_ref = manifest.manifest_baseline_id.clone();
    let mut decision = verified_admit_decision();
    decision.manifest_baseline_ref = manifest.manifest_baseline_id.clone();
    decision.decision_summary = "Buggy admit attempt against incomplete facts.".to_owned();

    let fact_grid_wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    );

    let prompt_wedge = PermissionPromptWedge::from_install_review_wedge(
        &fact_grid_wedge,
        "permission_prompt.failure_drill.incomplete_authority_facts",
        PermissionPromptRequester {
            requester_class: RequesterClass::Extension,
            requester_class_token: RequesterClass::Extension.as_str().to_owned(),
            requester_ref: String::new(),
            requester_display_label: String::new(),
            request_origin_label: "Buggy caller dropped the requester identity.".to_owned(),
        },
        PermissionPromptAuthorityOwner {
            issuer_class: AuthorityIssuerClass::Shell,
            issuer_class_token: AuthorityIssuerClass::Shell.as_str().to_owned(),
            issuer_source_ref: String::new(),
            issuer_source_label: String::new(),
        },
        ScopeFilterClass::CurrentRoot,
        String::new(),
        GrantScopeClass::Workspace,
        String::new(),
        PermissionPromptDenialBranch {
            degraded_capability_class: DegradedCapabilityClass::NoDegradeAvailable,
            degraded_capability_token: DegradedCapabilityClass::NoDegradeAvailable
                .as_str()
                .to_owned(),
            deny_path_label: String::new(),
            preserved_work_refs: Vec::new(),
        },
        PermissionPromptQuestions {
            who_is_asking: String::new(),
            what_boundary: "Some boundary.".to_owned(),
            why_needed: "Some reason.".to_owned(),
            what_changes_if_allowed: "Something changes.".to_owned(),
            what_works_if_denied: String::new(),
            grant_persistence_statement: String::new(),
        },
    )
    .with_force_offer_approve_on_blocked(true);

    let card = prompt_wedge.card();
    assert!(card.has_invariant_violations);
    let tokens: Vec<&str> = card
        .invariants
        .iter()
        .map(|r| r.violation_token.as_str())
        .collect();
    for expected in [
        "requester_identity_missing",
        "authority_owner_missing",
        "scope_missing",
        "grant_persistence_missing",
        "deny_path_missing",
        "prompt_question_unanswered",
        "approve_offered_with_blocked_install_review",
    ] {
        assert!(
            tokens.contains(&expected),
            "expected invariant {expected} to fire; got {tokens:?}",
        );
    }
    assert!(card.summary_line.contains("INVARIANTS BLOCKED"));
}

// ---------------------------------------------------------------------------
// Adjacent drills
// ---------------------------------------------------------------------------

#[test]
fn approve_offered_against_denied_decision_surfaces_typed_invariant() {
    let mut decision = verified_admit_decision();
    decision.install_decision_class = InstallDecisionClass::Denied;
    decision.install_decision_reason_class =
        InstallDecisionReasonClass::EffectivePermissionWideningAttempted;
    decision.decision_summary = "Denied: widening attempt blocked.".to_owned();
    let fact_grid_wedge = InstallReviewFactGridWedge::new(
        verified_admit_manifest(),
        verified_admit_effective(),
        decision,
        ActivationBudgetClass::NotApplicableInstallDenied,
        RollbackPostureClass::NotYetAdmittedNoRollbackNeeded,
    );
    let prompt_wedge = PermissionPromptWedge::from_install_review_wedge(
        &fact_grid_wedge,
        "permission_prompt.adjacent_drill.approve_on_denied",
        PermissionPromptRequester {
            requester_class: RequesterClass::Extension,
            requester_class_token: RequesterClass::Extension.as_str().to_owned(),
            requester_ref: "extension:acme-labs/prose-helper".to_owned(),
            requester_display_label: "Acme Labs Prose Helper".to_owned(),
            request_origin_label: "Adjacent drill.".to_owned(),
        },
        PermissionPromptAuthorityOwner {
            issuer_class: AuthorityIssuerClass::Shell,
            issuer_class_token: AuthorityIssuerClass::Shell.as_str().to_owned(),
            issuer_source_ref: "source.shell.user_approval".to_owned(),
            issuer_source_label: "Shell — user-facing approval lane".to_owned(),
        },
        ScopeFilterClass::CurrentRoot,
        "Current workspace root.",
        GrantScopeClass::Once,
        "Once — denied path.",
        PermissionPromptDenialBranch {
            degraded_capability_class: DegradedCapabilityClass::LocalOnlyContinues,
            degraded_capability_token: DegradedCapabilityClass::LocalOnlyContinues
                .as_str()
                .to_owned(),
            deny_path_label: "Local-only continues.".to_owned(),
            preserved_work_refs: vec!["workspace.local_editing.current_root".to_owned()],
        },
        PermissionPromptQuestions {
            who_is_asking: "x".to_owned(),
            what_boundary: "x".to_owned(),
            why_needed: "x".to_owned(),
            what_changes_if_allowed: "x".to_owned(),
            what_works_if_denied: "x".to_owned(),
            grant_persistence_statement: "x once x".to_owned(),
        },
    )
    .with_force_offer_approve_on_blocked(true);
    let card = prompt_wedge.card();
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "approve_offered_against_denied_decision"));
}

#[test]
fn suppressed_deny_action_surfaces_typed_invariant() {
    let wedge = verified_admit_prompt_wedge().with_suppress_deny_action(true);
    let card = wedge.card();
    assert!(!card.deny_action_offered);
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "no_deny_action_path"));
}

#[test]
fn approved_while_blocked_surfaces_typed_invariant() {
    // Approve a card whose required fields are stripped — the
    // approved_while_blocked invariant fires on top of the
    // requester/authority invariants.
    let mut wedge = verified_admit_prompt_wedge();
    wedge.requester.requester_ref = String::new();
    wedge.requester.requester_display_label = String::new();
    let card = wedge
        .with_decision_state(PermissionPromptDecisionState::Approved)
        .card();
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "approved_while_blocked"));
}

#[test]
fn grant_persistence_label_inconsistency_surfaces_typed_invariant() {
    let mut wedge = verified_admit_prompt_wedge();
    // Grant scope is `workspace` but the label says "session".
    wedge.grant_persistence_label = "Session — until the shell exits.".to_owned();
    let card = wedge.card();
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "grant_persistence_inconsistent"));
}

#[test]
fn policy_service_issuer_adds_request_admin_review_action() {
    let mut wedge = verified_admit_prompt_wedge();
    wedge.authority_owner.issuer_class = AuthorityIssuerClass::PolicyService;
    wedge.authority_owner.issuer_class_token =
        AuthorityIssuerClass::PolicyService.as_str().to_owned();
    let card = wedge.card();
    let roles: Vec<&str> = card
        .decision_actions
        .iter()
        .map(|a| a.action_role_token.as_str())
        .collect();
    assert!(roles.contains(&"request_admin_review"));
}

#[test]
fn render_plaintext_quotes_every_section_in_stable_order() {
    let card = verified_admit_prompt_wedge().card();
    let text = card.render_plaintext();
    assert!(text.starts_with("[m1_prototype_typed_permission_prompt]"));
    assert!(text.contains("prompt_id=permission_prompt.extension.acme_labs.prose_helper.install"));
    assert!(text.contains("install_review_clean=true"));
    assert!(text.contains("upstream_install_decision=admit"));
    assert!(text.contains("requester:"));
    assert!(text.contains("class=extension"));
    assert!(text.contains("authority_owner:"));
    assert!(text.contains("issuer=shell"));
    assert!(text.contains("scope:"));
    assert!(text.contains("grant_scope=workspace"));
    assert!(text.contains("requested_permissions:"));
    assert!(text.contains("filesystem_read=workspace:/docs/**"));
    assert!(text.contains("consequence:"));
    assert!(text.contains("denial_branch:"));
    assert!(text.contains("degraded_capability=read_only_inspection_continues"));
    assert!(text.contains("prompt_questions:"));
    assert!(text.contains("who_is_asking="));
    assert!(text.contains("what_works_if_denied="));
    assert!(text.contains("grant_persistence_statement="));
    assert!(text.contains("decision_actions:"));
    assert!(text.contains("role=primary_approve"));
    assert!(text.contains("role=primary_deny"));
    assert!(text.contains("decision_state=pending"));
    assert!(text.contains("approve_offered=true deny_offered=true"));
    assert!(text.contains("single_bounded_wedge_only"));
    assert!(text.contains("no_org_policy_pack_productization"));
    assert!(text.contains("invariants:\n  - clean"));
    assert!(text.contains("summary:"));
}

#[test]
fn record_round_trips_through_serde_json() {
    let card = verified_admit_prompt_wedge().card();
    let json = serde_json::to_string(&card).expect("serialise");
    let parsed: TypedPermissionPromptRecord =
        serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, card);
}

// ---------------------------------------------------------------------------
// Fixture-driven replays
// ---------------------------------------------------------------------------

#[test]
fn fixture_protected_walk_verified_publisher_approve_replays_into_the_wedge() {
    let fixture: PromptFixture = load_fixture("protected_walk_verified_publisher_approve.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.is_clean_approve());
}

#[test]
fn fixture_protected_walk_unverified_publisher_deny_replays_into_the_wedge() {
    let fixture: PromptFixture = load_fixture("protected_walk_unverified_publisher_deny.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.is_clean_deny());
    assert!(!card.approve_action_offered);
}

#[test]
fn fixture_failure_drill_incomplete_authority_facts_surfaces_typed_invariants() {
    let fixture: PromptFixture = load_fixture("failure_drill_incomplete_authority_facts.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.has_invariant_violations);
    let expected = fixture
        .expect
        .expected_violation_tokens
        .as_ref()
        .expect("failure drill must list expected violation tokens");
    for token in expected {
        assert!(
            card.invariants.iter().any(|r| &r.violation_token == token),
            "expected invariant {token} to fire on failure drill card",
        );
    }
}

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct PromptFixture {
    prompt_id: String,
    #[serde(default)]
    degraded_token: Option<String>,
    decision_state: String,
    #[serde(default)]
    suppress_deny_action: bool,
    #[serde(default)]
    force_offer_approve_on_blocked: bool,
    requester: FixtureRequester,
    authority_owner: FixtureAuthorityOwner,
    scope_filter_class: String,
    scope_target_label: String,
    grant_scope_class: String,
    grant_persistence_label: String,
    denial_branch: FixtureDenialBranch,
    prompt_questions: FixtureQuestions,
    install_review: FixtureInstallReview,
    expect: PromptFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureRequester {
    requester_class: String,
    requester_ref: String,
    requester_display_label: String,
    request_origin_label: String,
}

#[derive(Debug, Deserialize)]
struct FixtureAuthorityOwner {
    issuer_class: String,
    issuer_source_ref: String,
    issuer_source_label: String,
}

#[derive(Debug, Deserialize)]
struct FixtureDenialBranch {
    degraded_capability_class: String,
    deny_path_label: String,
    preserved_work_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureQuestions {
    who_is_asking: String,
    what_boundary: String,
    why_needed: String,
    what_changes_if_allowed: String,
    what_works_if_denied: String,
    grant_persistence_statement: String,
}

#[derive(Debug, Deserialize)]
struct FixtureInstallReview {
    manifest_baseline: FixtureManifest,
    effective_permission_baseline: FixtureEffective,
    install_decision: FixtureDecision,
    activation_budget_class: String,
    rollback_posture_class: String,
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    manifest_baseline_id: String,
    extension_identity: String,
    extension_version: String,
    extension_lifecycle_state_class: String,
    host_contract_family_class: String,
    manifest_origin_source_class: String,
    origin_source_label: String,
    publisher_identity_ref: String,
    publisher_display_label: String,
    publisher_trust_tier_class: String,
    publisher_lifecycle_state_class: String,
    publisher_signing_key_ref: String,
    declared_permissions: Vec<FixtureDeclaredPermission>,
    manifest_scope_completeness_class: String,
}

#[derive(Debug, Deserialize)]
struct FixtureDeclaredPermission {
    scope_class: String,
    scope_target: String,
    #[serde(default)]
    scope_constraint: Option<String>,
    rationale_label: String,
}

#[derive(Debug, Deserialize)]
struct FixtureEffective {
    declared_vs_effective_diff: Vec<FixtureDiffEntry>,
    widening_attempted_blocked_count: u32,
}

#[derive(Debug, Deserialize)]
struct FixtureDiffEntry {
    scope_class: String,
    scope_target: String,
    diff_class: String,
    narrowing_reason_label: String,
}

#[derive(Debug, Deserialize)]
struct FixtureDecision {
    install_decision_class: String,
    install_decision_reason_class: String,
    decision_summary: String,
}

#[derive(Debug, Deserialize)]
struct PromptFixtureExpect {
    has_invariant_violations: bool,
    decision_state: String,
    approve_action_offered: bool,
    deny_action_offered: bool,
    #[serde(default)]
    summary_contains: Option<String>,
    #[serde(default)]
    expected_violation_tokens: Option<Vec<String>>,
}

fn load_fixture(name: &str) -> PromptFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/permissions/publish_or_ecosystem_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn build_card_from_fixture(fixture: &PromptFixture) -> TypedPermissionPromptRecord {
    let manifest = ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: fixture
            .install_review
            .manifest_baseline
            .manifest_baseline_id
            .clone(),
        extension_identity: fixture
            .install_review
            .manifest_baseline
            .extension_identity
            .clone(),
        extension_version: fixture
            .install_review
            .manifest_baseline
            .extension_version
            .clone(),
        extension_lifecycle_state_class: parse_extension_lifecycle(
            &fixture
                .install_review
                .manifest_baseline
                .extension_lifecycle_state_class,
        ),
        host_contract_family_class: parse_host_contract_family(
            &fixture
                .install_review
                .manifest_baseline
                .host_contract_family_class,
        ),
        manifest_origin_source_class: parse_origin_source(
            &fixture
                .install_review
                .manifest_baseline
                .manifest_origin_source_class,
        ),
        origin_source_label: fixture
            .install_review
            .manifest_baseline
            .origin_source_label
            .clone(),
        publisher_identity_ref: fixture
            .install_review
            .manifest_baseline
            .publisher_identity_ref
            .clone(),
        publisher_display_label: fixture
            .install_review
            .manifest_baseline
            .publisher_display_label
            .clone(),
        publisher_trust_tier_class: parse_publisher_trust_tier(
            &fixture
                .install_review
                .manifest_baseline
                .publisher_trust_tier_class,
        ),
        publisher_lifecycle_state_class: parse_publisher_lifecycle(
            &fixture
                .install_review
                .manifest_baseline
                .publisher_lifecycle_state_class,
        ),
        publisher_signing_key_ref: fixture
            .install_review
            .manifest_baseline
            .publisher_signing_key_ref
            .clone(),
        declared_permissions: fixture
            .install_review
            .manifest_baseline
            .declared_permissions
            .iter()
            .map(|p| PermissionScopeEntry {
                scope_class: parse_scope_class(&p.scope_class),
                scope_target: p.scope_target.clone(),
                scope_constraint: p.scope_constraint.clone(),
                rationale_label: p.rationale_label.clone(),
            })
            .collect(),
        manifest_scope_completeness_class: parse_manifest_scope_completeness(
            &fixture
                .install_review
                .manifest_baseline
                .manifest_scope_completeness_class,
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let effective = EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: fixture
            .install_review
            .effective_permission_baseline
            .declared_vs_effective_diff
            .iter()
            .map(|d| DeclaredVsEffectiveDiffEntry {
                scope_class: parse_scope_class(&d.scope_class),
                scope_target: d.scope_target.clone(),
                diff_class: parse_diff_class(&d.diff_class),
                narrowing_reason_label: d.narrowing_reason_label.clone(),
            })
            .collect(),
        widening_attempted_blocked_count: fixture
            .install_review
            .effective_permission_baseline
            .widening_attempted_blocked_count,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let decision = ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        install_decision_class: parse_install_decision_class(
            &fixture
                .install_review
                .install_decision
                .install_decision_class,
        ),
        install_decision_reason_class: parse_install_decision_reason_class(
            &fixture
                .install_review
                .install_decision
                .install_decision_reason_class,
        ),
        decision_summary: fixture
            .install_review
            .install_decision
            .decision_summary
            .clone(),
        decided_at: "2026-05-11T08:00:01Z".to_owned(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    };

    let fact_grid_wedge = InstallReviewFactGridWedge::new(
        manifest,
        effective,
        decision,
        parse_activation_budget(&fixture.install_review.activation_budget_class),
        parse_rollback_posture(&fixture.install_review.rollback_posture_class),
    );

    let requester_class = parse_requester_class(&fixture.requester.requester_class);
    let issuer_class = parse_issuer_class(&fixture.authority_owner.issuer_class);

    let mut prompt_wedge = PermissionPromptWedge::from_install_review_wedge(
        &fact_grid_wedge,
        fixture.prompt_id.clone(),
        PermissionPromptRequester {
            requester_class,
            requester_class_token: requester_class.as_str().to_owned(),
            requester_ref: fixture.requester.requester_ref.clone(),
            requester_display_label: fixture.requester.requester_display_label.clone(),
            request_origin_label: fixture.requester.request_origin_label.clone(),
        },
        PermissionPromptAuthorityOwner {
            issuer_class,
            issuer_class_token: issuer_class.as_str().to_owned(),
            issuer_source_ref: fixture.authority_owner.issuer_source_ref.clone(),
            issuer_source_label: fixture.authority_owner.issuer_source_label.clone(),
        },
        parse_scope_filter(&fixture.scope_filter_class),
        fixture.scope_target_label.clone(),
        parse_grant_scope(&fixture.grant_scope_class),
        fixture.grant_persistence_label.clone(),
        PermissionPromptDenialBranch {
            degraded_capability_class: parse_degraded_capability(
                &fixture.denial_branch.degraded_capability_class,
            ),
            degraded_capability_token: parse_degraded_capability(
                &fixture.denial_branch.degraded_capability_class,
            )
            .as_str()
            .to_owned(),
            deny_path_label: fixture.denial_branch.deny_path_label.clone(),
            preserved_work_refs: fixture.denial_branch.preserved_work_refs.clone(),
        },
        PermissionPromptQuestions {
            who_is_asking: fixture.prompt_questions.who_is_asking.clone(),
            what_boundary: fixture.prompt_questions.what_boundary.clone(),
            why_needed: fixture.prompt_questions.why_needed.clone(),
            what_changes_if_allowed: fixture.prompt_questions.what_changes_if_allowed.clone(),
            what_works_if_denied: fixture.prompt_questions.what_works_if_denied.clone(),
            grant_persistence_statement: fixture
                .prompt_questions
                .grant_persistence_statement
                .clone(),
        },
    )
    .with_decision_state(parse_decision_state(&fixture.decision_state))
    .with_suppress_deny_action(fixture.suppress_deny_action)
    .with_force_offer_approve_on_blocked(fixture.force_offer_approve_on_blocked);
    if let Some(token) = &fixture.degraded_token {
        prompt_wedge = prompt_wedge.with_degraded(parse_degraded(token));
    }
    prompt_wedge.card()
}

fn assert_fixture_matches(card: &TypedPermissionPromptRecord, fixture: &PromptFixture) {
    assert_eq!(
        card.has_invariant_violations, fixture.expect.has_invariant_violations,
        "has_invariant_violations mismatch"
    );
    assert_eq!(
        card.decision_state_token, fixture.expect.decision_state,
        "decision_state mismatch"
    );
    assert_eq!(
        card.approve_action_offered, fixture.expect.approve_action_offered,
        "approve_action_offered mismatch"
    );
    assert_eq!(
        card.deny_action_offered, fixture.expect.deny_action_offered,
        "deny_action_offered mismatch"
    );
    if let Some(needle) = &fixture.expect.summary_contains {
        assert!(
            card.summary_line.contains(needle),
            "summary_line {:?} should contain {:?}",
            card.summary_line,
            needle,
        );
    }
}

fn parse_requester_class(token: &str) -> RequesterClass {
    match token {
        "extension" => RequesterClass::Extension,
        "extension_publisher_flow" => RequesterClass::ExtensionPublisherFlow,
        "user_initiated_install" => RequesterClass::UserInitiatedInstall,
        other => panic!("unknown requester_class {other}"),
    }
}

fn parse_issuer_class(token: &str) -> AuthorityIssuerClass {
    match token {
        "shell" => AuthorityIssuerClass::Shell,
        "policy_service" => AuthorityIssuerClass::PolicyService,
        other => panic!("unknown issuer_class {other}"),
    }
}

fn parse_scope_filter(token: &str) -> ScopeFilterClass {
    match token {
        "current_root" => ScopeFilterClass::CurrentRoot,
        "named_workset" => ScopeFilterClass::NamedWorkset,
        "full_workspace" => ScopeFilterClass::FullWorkspace,
        "policy_limited_view" => ScopeFilterClass::PolicyLimitedView,
        other => panic!("unknown scope_filter_class {other}"),
    }
}

fn parse_grant_scope(token: &str) -> GrantScopeClass {
    match token {
        "once" => GrantScopeClass::Once,
        "session" => GrantScopeClass::Session,
        "workspace" => GrantScopeClass::Workspace,
        "profile" => GrantScopeClass::Profile,
        "policy_managed" => GrantScopeClass::PolicyManaged,
        other => panic!("unknown grant_scope_class {other}"),
    }
}

fn parse_degraded_capability(token: &str) -> DegradedCapabilityClass {
    match token {
        "no_degrade_available" => DegradedCapabilityClass::NoDegradeAvailable,
        "local_only_continues" => DegradedCapabilityClass::LocalOnlyContinues,
        "read_only_inspection_continues" => DegradedCapabilityClass::ReadOnlyInspectionContinues,
        "preview_only_continues" => DegradedCapabilityClass::PreviewOnlyContinues,
        "metadata_only_export" => DegradedCapabilityClass::MetadataOnlyExport,
        "install_disabled_capability_removed" => {
            DegradedCapabilityClass::InstallDisabledCapabilityRemoved
        }
        other => panic!("unknown degraded_capability_class {other}"),
    }
}

fn parse_decision_state(token: &str) -> PermissionPromptDecisionState {
    match token {
        "pending" => PermissionPromptDecisionState::Pending,
        "approved" => PermissionPromptDecisionState::Approved,
        "denied" => PermissionPromptDecisionState::Denied,
        "cancelled" => PermissionPromptDecisionState::Cancelled,
        "blocked_by_policy" => PermissionPromptDecisionState::BlockedByPolicy,
        other => panic!("unknown decision_state {other}"),
    }
}

fn parse_publisher_trust_tier(token: &str) -> PublisherTrustTierClass {
    match token {
        "verified_publisher" => PublisherTrustTierClass::VerifiedPublisher,
        "community_publisher" => PublisherTrustTierClass::CommunityPublisher,
        "organisational_publisher" => PublisherTrustTierClass::OrganisationalPublisher,
        "unverified_publisher" => PublisherTrustTierClass::UnverifiedPublisher,
        "quarantined_publisher" => PublisherTrustTierClass::QuarantinedPublisher,
        "anonymous_publisher_class" => PublisherTrustTierClass::AnonymousPublisherClass,
        other => panic!("unknown publisher_trust_tier_class {other}"),
    }
}

fn parse_publisher_lifecycle(token: &str) -> PublisherLifecycleStateClass {
    match token {
        "active" => PublisherLifecycleStateClass::Active,
        "preview" => PublisherLifecycleStateClass::Preview,
        "deprecated" => PublisherLifecycleStateClass::Deprecated,
        "retired" => PublisherLifecycleStateClass::Retired,
        "quarantined" => PublisherLifecycleStateClass::Quarantined,
        other => panic!("unknown publisher_lifecycle_state_class {other}"),
    }
}

fn parse_extension_lifecycle(token: &str) -> ExtensionLifecycleStateClass {
    match token {
        "published" => ExtensionLifecycleStateClass::Published,
        "preview" => ExtensionLifecycleStateClass::Preview,
        "deprecated" => ExtensionLifecycleStateClass::Deprecated,
        "retired" => ExtensionLifecycleStateClass::Retired,
        "quarantined" => ExtensionLifecycleStateClass::Quarantined,
        other => panic!("unknown extension_lifecycle_state_class {other}"),
    }
}

fn parse_origin_source(token: &str) -> ManifestOriginSourceClass {
    match token {
        "public_registry" => ManifestOriginSourceClass::PublicRegistry,
        "private_registry" => ManifestOriginSourceClass::PrivateRegistry,
        "mirror" => ManifestOriginSourceClass::Mirror,
        "offline_bundle" => ManifestOriginSourceClass::OfflineBundle,
        "vendored_local" => ManifestOriginSourceClass::VendoredLocal,
        "unknown_source_class" => ManifestOriginSourceClass::UnknownSourceClass,
        other => panic!("unknown manifest_origin_source_class {other}"),
    }
}

fn parse_host_contract_family(token: &str) -> HostContractFamilyClass {
    match token {
        "wasm_component_model" => HostContractFamilyClass::WasmComponentModel,
        "wasm_core_module" => HostContractFamilyClass::WasmCoreModule,
        "external_host_process" => HostContractFamilyClass::ExternalHostProcess,
        "helper_binary" => HostContractFamilyClass::HelperBinary,
        "remote_side_component" => HostContractFamilyClass::RemoteSideComponent,
        "compatibility_bridge" => HostContractFamilyClass::CompatibilityBridge,
        other => panic!("unknown host_contract_family_class {other}"),
    }
}

fn parse_scope_class(token: &str) -> PermissionScopeClass {
    match token {
        "filesystem_read" => PermissionScopeClass::FilesystemRead,
        "filesystem_write" => PermissionScopeClass::FilesystemWrite,
        "shell_execute" => PermissionScopeClass::ShellExecute,
        "network_egress" => PermissionScopeClass::NetworkEgress,
        "ai_provider_access" => PermissionScopeClass::AiProviderAccess,
        "connected_provider_access" => PermissionScopeClass::ConnectedProviderAccess,
        "secret_handle_use" => PermissionScopeClass::SecretHandleUse,
        "workspace_settings_read" => PermissionScopeClass::WorkspaceSettingsRead,
        "workspace_settings_write" => PermissionScopeClass::WorkspaceSettingsWrite,
        "execution_context_bind" => PermissionScopeClass::ExecutionContextBind,
        "subscription_subscribe" => PermissionScopeClass::SubscriptionSubscribe,
        "ui_command_contribute" => PermissionScopeClass::UiCommandContribute,
        "capability_inherit" => PermissionScopeClass::CapabilityInherit,
        other => panic!("unknown permission_scope_class {other}"),
    }
}

fn parse_diff_class(token: &str) -> EffectivePermissionDiffClass {
    match token {
        "unchanged" => EffectivePermissionDiffClass::Unchanged,
        "narrowed" => EffectivePermissionDiffClass::Narrowed,
        "denied" => EffectivePermissionDiffClass::Denied,
        "step_up_required" => EffectivePermissionDiffClass::StepUpRequired,
        "widening_attempted_blocked" => EffectivePermissionDiffClass::WideningAttemptedBlocked,
        other => panic!("unknown effective_permission_diff_class {other}"),
    }
}

fn parse_manifest_scope_completeness(token: &str) -> ManifestScopeCompletenessClass {
    match token {
        "complete" => ManifestScopeCompletenessClass::Complete,
        "incomplete_publisher_missing" => {
            ManifestScopeCompletenessClass::IncompletePublisherMissing
        }
        "incomplete_origin_missing" => ManifestScopeCompletenessClass::IncompleteOriginMissing,
        "incomplete_permission_rationale_missing" => {
            ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing
        }
        "incomplete_lifecycle_unknown" => {
            ManifestScopeCompletenessClass::IncompleteLifecycleUnknown
        }
        other => panic!("unknown manifest_scope_completeness_class {other}"),
    }
}

fn parse_install_decision_class(token: &str) -> InstallDecisionClass {
    match token {
        "admit" => InstallDecisionClass::Admit,
        "admit_with_step_up" => InstallDecisionClass::AdmitWithStepUp,
        "review_only" => InstallDecisionClass::ReviewOnly,
        "denied" => InstallDecisionClass::Denied,
        other => panic!("unknown install_decision_class {other}"),
    }
}

fn parse_install_decision_reason_class(token: &str) -> InstallDecisionReasonClass {
    match token {
        "admitted_no_violation" => InstallDecisionReasonClass::AdmittedNoViolation,
        "step_up_required_by_policy_pack" => InstallDecisionReasonClass::StepUpRequiredByPolicyPack,
        "review_only_unverified_publisher" => {
            InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher
        }
        "publisher_identity_required" => InstallDecisionReasonClass::PublisherIdentityRequired,
        "publisher_anonymous" => InstallDecisionReasonClass::PublisherAnonymous,
        "publisher_quarantined" => InstallDecisionReasonClass::PublisherQuarantined,
        "publisher_lifecycle_retired" => InstallDecisionReasonClass::PublisherLifecycleRetired,
        "extension_lifecycle_retired" => InstallDecisionReasonClass::ExtensionLifecycleRetired,
        "manifest_scope_incomplete" => InstallDecisionReasonClass::ManifestScopeIncomplete,
        "manifest_origin_unknown" => InstallDecisionReasonClass::ManifestOriginUnknown,
        "declared_permission_rationale_required" => {
            InstallDecisionReasonClass::DeclaredPermissionRationaleRequired
        }
        "effective_permission_widening_attempted" => {
            InstallDecisionReasonClass::EffectivePermissionWideningAttempted
        }
        "lifecycle_state_unknown_class" => InstallDecisionReasonClass::LifecycleStateUnknownClass,
        other => panic!("unknown install_decision_reason_class {other}"),
    }
}

fn parse_activation_budget(token: &str) -> ActivationBudgetClass {
    match token {
        "eager_within_workspace_only" => ActivationBudgetClass::EagerWithinWorkspaceOnly,
        "lazy_on_demand_only" => ActivationBudgetClass::LazyOnDemandOnly,
        "lazy_on_event_subscription" => ActivationBudgetClass::LazyOnEventSubscription,
        "restricted_step_up_required" => ActivationBudgetClass::RestrictedStepUpRequired,
        "denied_by_policy_pack" => ActivationBudgetClass::DeniedByPolicyPack,
        "not_applicable_install_denied" => ActivationBudgetClass::NotApplicableInstallDenied,
        other => panic!("unknown activation_budget_class {other}"),
    }
}

fn parse_rollback_posture(token: &str) -> RollbackPostureClass {
    match token {
        "clean_uninstall_and_state_purge" => RollbackPostureClass::CleanUninstallAndStatePurge,
        "uninstall_with_user_state_retained" => {
            RollbackPostureClass::UninstallWithUserStateRetained
        }
        "quarantine_only_pending_publisher_review" => {
            RollbackPostureClass::QuarantineOnlyPendingPublisherReview
        }
        "uninstall_blocked_pending_admin_review" => {
            RollbackPostureClass::UninstallBlockedPendingAdminReview
        }
        "not_yet_admitted_no_rollback_needed" => {
            RollbackPostureClass::NotYetAdmittedNoRollbackNeeded
        }
        other => panic!("unknown rollback_posture_class {other}"),
    }
}

fn parse_degraded(token: &str) -> DegradedStateToken {
    match token {
        "Warming" => DegradedStateToken::Warming,
        "Cached" => DegradedStateToken::Cached,
        "Partial" => DegradedStateToken::Partial,
        "Stale" => DegradedStateToken::Stale,
        "Offline" => DegradedStateToken::Offline,
        "PolicyBlocked" => DegradedStateToken::PolicyBlocked,
        "Limited" => DegradedStateToken::Limited,
        "Unsupported" => DegradedStateToken::Unsupported,
        "Experimental" => DegradedStateToken::Experimental,
        "RetestPending" => DegradedStateToken::RetestPending,
        other => panic!("unknown degraded_token {other}"),
    }
}
