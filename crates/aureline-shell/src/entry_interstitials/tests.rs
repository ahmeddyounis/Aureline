use super::*;

use std::path::{Path, PathBuf};

use serde_json::Value;

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/high_authority_entry")
}

/// Loads a fixture and strips editor-only meta keys (`$schema`, `__fixture__`)
/// so the remaining object is exactly the serialized record.
fn load_fixture_value(name: &str) -> Value {
    let path = fixture_dir().join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    let mut value: Value = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {name} must parse: {err}"));
    if let Value::Object(map) = &mut value {
        map.retain(|key, _| !key.starts_with('$') && !key.starts_with("__"));
    }
    value
}

fn target(kind: EntryTargetKind, truth: TargetTruthState) -> TargetScope {
    TargetScope {
        target_kind: kind,
        object_identity_ref: format!("obj:{}:01", kind.as_str()),
        target_label: "Reviewed target".to_string(),
        workspace_scope_label: "Active workspace".to_string(),
        tenant_scope_label: None,
        channel_or_build_owner_label: None,
        truth_state: truth,
        identity_review_required: false,
    }
}

fn plain_local_deep_link() -> EntryInterstitialRequest {
    EntryInterstitialRequest {
        interstitial_id: "ei:plain:01".to_string(),
        kind: EntryInterstitialKind::ProtocolDeepLink,
        source_class: EntrySourceClass::OsShell,
        source_label: "OS file association".to_string(),
        requested_action: RequestedActionClass::OpenExistingContext,
        target: target(EntryTargetKind::LocalFile, TargetTruthState::ExactAvailable),
        authority_effect: AuthorityEffectClass::NoChange,
        crosses_tenant_boundary: false,
        canonical_command_id: "cmd:workspace.open_file".to_string(),
    }
}

#[test]
fn plain_local_open_takes_fast_path() {
    let decision = evaluate_entry_interstitial(&plain_local_deep_link());
    assert!(!decision.requires_interstitial());
    match decision {
        EntryInterstitialDecision::PlainLocalOpen(admission) => {
            assert_eq!(admission.record_kind, PLAIN_LOCAL_OPEN_RECORD_KIND);
            assert_eq!(admission.canonical_command_ref, "cmd:workspace.open_file");
            assert_eq!(admission.reason, "plain_local_open_no_boundary_crossed");
        }
        other => panic!("expected plain local open, got {other:?}"),
    }
}

#[test]
fn local_notification_reopen_of_exact_file_is_fast_path() {
    let mut request = plain_local_deep_link();
    request.kind = EntryInterstitialKind::NotificationReopen;
    request.source_class = EntrySourceClass::OsNotification;
    request.requested_action = RequestedActionClass::AcknowledgeAndReopen;
    request.canonical_command_id = "cmd:notification.open_review".to_string();
    // OS notification source is non-local but, with an exact in-scope target
    // and no identity review, the reopen does not cross a boundary.
    let decision = evaluate_entry_interstitial(&request);
    assert!(!decision.requires_interstitial());
}

#[test]
fn auth_callback_return_always_requires_interstitial() {
    let request = EntryInterstitialRequest {
        interstitial_id: "ei:auth:01".to_string(),
        kind: EntryInterstitialKind::AuthCallbackReturn,
        source_class: EntrySourceClass::AuthProviderCallback,
        source_label: "System browser sign-in".to_string(),
        requested_action: RequestedActionClass::AuthReturn,
        target: target(
            EntryTargetKind::AuthSession,
            TargetTruthState::ExactAvailable,
        ),
        authority_effect: AuthorityEffectClass::AuthScopeRequired,
        crosses_tenant_boundary: false,
        canonical_command_id: "cmd:auth.complete_system_browser_return".to_string(),
    };
    let decision = evaluate_entry_interstitial(&request);
    let record = decision.record().expect("interstitial required");
    assert!(record
        .boundary_classes
        .contains(&BoundaryClass::AuthorityWidening));
    assert!(record.silent_execution_forbidden);
    assert!(record.confirm_matches_canonical());
    assert!(record.authority_not_widened);
}

#[test]
fn confirm_action_binds_exactly_to_canonical_command() {
    let request = EntryInterstitialRequest {
        interstitial_id: "ei:remote:01".to_string(),
        kind: EntryInterstitialKind::RemoteTargetOpen,
        source_class: EntrySourceClass::SystemDefaultBrowser,
        source_label: "Default browser deep link".to_string(),
        requested_action: RequestedActionClass::RemoteTargetOpen,
        target: target(
            EntryTargetKind::RemoteWorkspace,
            TargetTruthState::ExactAvailable,
        ),
        authority_effect: AuthorityEffectClass::RemoteAuthorityRebind,
        crosses_tenant_boundary: false,
        canonical_command_id: "cmd:workspace.open_remote_target".to_string(),
    };
    let decision = evaluate_entry_interstitial(&request);
    let record = decision.record().expect("interstitial required");
    let confirm = record.confirm_action().expect("confirm present");
    assert_eq!(confirm.command_id, "cmd:workspace.open_remote_target");
    assert_eq!(confirm.command_id, record.canonical_command_ref);
    assert!(record.confirm_matches_canonical());
    assert!(record
        .boundary_classes
        .contains(&BoundaryClass::RemoteBoundary));
}

#[test]
fn missing_target_preserves_intent_with_placeholder_and_disables_confirm() {
    let request = EntryInterstitialRequest {
        interstitial_id: "ei:missing:01".to_string(),
        kind: EntryInterstitialKind::ProtocolDeepLink,
        source_class: EntrySourceClass::SystemDefaultBrowser,
        source_label: "Default browser deep link".to_string(),
        requested_action: RequestedActionClass::OpenExistingContext,
        target: target(
            EntryTargetKind::WorkspaceRoot,
            TargetTruthState::MissingOrUnmounted,
        ),
        authority_effect: AuthorityEffectClass::NoChange,
        crosses_tenant_boundary: false,
        canonical_command_id: "cmd:workspace.open_folder".to_string(),
    };
    let decision = evaluate_entry_interstitial(&request);
    let record = decision.record().expect("interstitial required");

    let placeholder = record
        .target_placeholder
        .as_ref()
        .expect("placeholder present");
    assert!(placeholder.announced);
    assert!(!placeholder.fallback_actions.is_empty());
    assert!(placeholder
        .fallback_actions
        .contains(&FallbackAction::LocateMissingTarget));

    // Confirm must be disabled because the exact target cannot open.
    let confirm = record.confirm_action().expect("confirm present");
    assert!(!confirm.enabled);

    // Intent is preserved through a placeholder, never an empty shell.
    assert!(record.reopens_exact_or_placeholder());
    assert!(!record.reopens_generic_home);
}

#[test]
fn notification_reopen_of_moved_object_never_lands_on_generic_home() {
    let request = EntryInterstitialRequest {
        interstitial_id: "ei:notif:moved:01".to_string(),
        kind: EntryInterstitialKind::NotificationReopen,
        source_class: EntrySourceClass::OsNotification,
        source_label: "OS notification".to_string(),
        requested_action: RequestedActionClass::AcknowledgeAndReopen,
        target: target(
            EntryTargetKind::ReviewOrWorkItem,
            TargetTruthState::MovedOrAliasChanged,
        ),
        authority_effect: AuthorityEffectClass::NoChange,
        crosses_tenant_boundary: false,
        canonical_command_id: "cmd:notification.open_review".to_string(),
    };
    let decision = evaluate_entry_interstitial(&request);
    let record = decision.record().expect("interstitial required");
    assert!(!record.reopens_generic_home);
    assert!(record.reopens_exact_or_placeholder());
    assert!(record
        .target_placeholder
        .as_ref()
        .expect("placeholder")
        .fallback_actions
        .contains(&FallbackAction::LocateMissingTarget));
}

#[test]
fn collaboration_join_crosses_tenant_and_remote_boundaries() {
    let request = EntryInterstitialRequest {
        interstitial_id: "ei:collab:01".to_string(),
        kind: EntryInterstitialKind::CollaborationJoin,
        source_class: EntrySourceClass::CollaborationService,
        source_label: "Collaboration invite".to_string(),
        requested_action: RequestedActionClass::JoinPresence,
        target: TargetScope {
            target_kind: EntryTargetKind::CollaborationSession,
            object_identity_ref: "obj:collab:42".to_string(),
            target_label: "Pair session".to_string(),
            workspace_scope_label: "Shared workspace".to_string(),
            tenant_scope_label: Some("Partner org".to_string()),
            channel_or_build_owner_label: None,
            truth_state: TargetTruthState::ExactAvailable,
            identity_review_required: true,
        },
        authority_effect: AuthorityEffectClass::TenantScopeChange,
        crosses_tenant_boundary: true,
        canonical_command_id: "cmd:collaboration.join_session".to_string(),
    };
    let decision = evaluate_entry_interstitial(&request);
    let record = decision.record().expect("interstitial required");
    assert!(record
        .boundary_classes
        .contains(&BoundaryClass::TenantBoundary));
    assert!(record
        .boundary_classes
        .contains(&BoundaryClass::RemoteBoundary));
    assert!(record
        .boundary_classes
        .contains(&BoundaryClass::AuthorityWidening));
}

#[test]
fn write_log_round_trips_via_serde() {
    let decision = evaluate_entry_interstitial(&EntryInterstitialRequest {
        interstitial_id: "ei:managed:roundtrip".to_string(),
        kind: EntryInterstitialKind::ManagedResume,
        source_class: EntrySourceClass::ManagedAdminSurface,
        source_label: "Managed admin surface".to_string(),
        requested_action: RequestedActionClass::ManagedResume,
        target: TargetScope {
            target_kind: EntryTargetKind::ManagedWorkspace,
            object_identity_ref: "obj:managed:7".to_string(),
            target_label: "Managed workspace".to_string(),
            workspace_scope_label: "Acme / platform".to_string(),
            tenant_scope_label: Some("Acme Corp".to_string()),
            channel_or_build_owner_label: Some("stable".to_string()),
            truth_state: TargetTruthState::ExactAvailable,
            identity_review_required: true,
        },
        authority_effect: AuthorityEffectClass::PolicyBoundaryReview,
        crosses_tenant_boundary: true,
        canonical_command_id: "cmd:workspace.restore_from_checkpoint".to_string(),
    });
    let record = decision.record().expect("interstitial required").clone();

    let dir = tempfile::tempdir().expect("tempdir");
    write_entry_interstitial_log(dir.path(), &record).expect("write");
    let read =
        std::fs::read_to_string(dir.path().join("entry_interstitial_latest.json")).expect("read");
    let back: EntryInterstitialRecord = serde_json::from_str(&read).expect("parse");
    assert_eq!(back, record);
}

// --- Fixture-backed cases. Each fixture is the canonical serialized output of
// the materializer; one shared builder feeds both the per-fixture assertions
// and the ignored regenerator, so a fixture can never drift from the code. ---

/// The full set of fixture cases, keyed by file name. Each entry is the request
/// the materializer runs on.
fn fixture_cases() -> Vec<(&'static str, EntryInterstitialRequest)> {
    vec![
        (
            "protocol_deep_link_managed_open.json",
            EntryInterstitialRequest {
                interstitial_id: "ei:fixture:deeplink:01".to_string(),
                kind: EntryInterstitialKind::ProtocolDeepLink,
                source_class: EntrySourceClass::SystemDefaultBrowser,
                source_label: "Default browser deep link".to_string(),
                requested_action: RequestedActionClass::ResumeSession,
                target: TargetScope {
                    target_kind: EntryTargetKind::ManagedWorkspace,
                    object_identity_ref: "obj:managed-ws:deeplink".to_string(),
                    target_label: "Acme managed workspace".to_string(),
                    workspace_scope_label: "Acme / platform".to_string(),
                    tenant_scope_label: Some("Acme Corp".to_string()),
                    channel_or_build_owner_label: Some("stable / admin-owned".to_string()),
                    truth_state: TargetTruthState::ExactAvailable,
                    identity_review_required: true,
                },
                authority_effect: AuthorityEffectClass::PolicyBoundaryReview,
                crosses_tenant_boundary: true,
                canonical_command_id: "cmd:workspace.restore_from_checkpoint".to_string(),
            },
        ),
        (
            "auth_callback_return.json",
            EntryInterstitialRequest {
                interstitial_id: "ei:fixture:auth:01".to_string(),
                kind: EntryInterstitialKind::AuthCallbackReturn,
                source_class: EntrySourceClass::AuthProviderCallback,
                source_label: "System browser sign-in".to_string(),
                requested_action: RequestedActionClass::AuthReturn,
                target: TargetScope {
                    target_kind: EntryTargetKind::AuthSession,
                    object_identity_ref: "obj:auth-session:01".to_string(),
                    target_label: "Sign-in to Acme tenant".to_string(),
                    workspace_scope_label: "Acme / platform".to_string(),
                    tenant_scope_label: Some("Acme Corp".to_string()),
                    channel_or_build_owner_label: None,
                    truth_state: TargetTruthState::ExactAvailable,
                    identity_review_required: false,
                },
                authority_effect: AuthorityEffectClass::AuthScopeRequired,
                crosses_tenant_boundary: false,
                canonical_command_id: "cmd:auth.complete_system_browser_return".to_string(),
            },
        ),
        (
            "remote_target_open_unreachable.json",
            EntryInterstitialRequest {
                interstitial_id: "ei:fixture:remote:01".to_string(),
                kind: EntryInterstitialKind::RemoteTargetOpen,
                source_class: EntrySourceClass::SystemDefaultBrowser,
                source_label: "Default browser deep link".to_string(),
                requested_action: RequestedActionClass::RemoteTargetOpen,
                target: TargetScope {
                    target_kind: EntryTargetKind::RemoteWorkspace,
                    object_identity_ref: "obj:remote-ws:01".to_string(),
                    target_label: "Remote dev workspace".to_string(),
                    workspace_scope_label: "team / remote".to_string(),
                    tenant_scope_label: None,
                    channel_or_build_owner_label: None,
                    truth_state: TargetTruthState::RemoteUnreachable,
                    identity_review_required: false,
                },
                authority_effect: AuthorityEffectClass::RemoteAuthorityRebind,
                crosses_tenant_boundary: false,
                canonical_command_id: "cmd:workspace.open_remote_target".to_string(),
            },
        ),
        (
            "notification_reopen_exact.json",
            EntryInterstitialRequest {
                interstitial_id: "ei:fixture:notif:01".to_string(),
                kind: EntryInterstitialKind::NotificationReopen,
                source_class: EntrySourceClass::OsNotification,
                source_label: "OS notification".to_string(),
                requested_action: RequestedActionClass::AcknowledgeAndReopen,
                target: TargetScope {
                    target_kind: EntryTargetKind::ManagedWorkspace,
                    object_identity_ref: "obj:managed-review:01".to_string(),
                    target_label: "Managed review thread".to_string(),
                    workspace_scope_label: "Acme / platform".to_string(),
                    tenant_scope_label: Some("Acme Corp".to_string()),
                    channel_or_build_owner_label: None,
                    truth_state: TargetTruthState::ExactAvailable,
                    identity_review_required: true,
                },
                authority_effect: AuthorityEffectClass::PolicyBoundaryReview,
                crosses_tenant_boundary: true,
                canonical_command_id: "cmd:notification.open_review".to_string(),
            },
        ),
        (
            "managed_resume.json",
            EntryInterstitialRequest {
                interstitial_id: "ei:fixture:managed:01".to_string(),
                kind: EntryInterstitialKind::ManagedResume,
                source_class: EntrySourceClass::ManagedAdminSurface,
                source_label: "Managed admin surface".to_string(),
                requested_action: RequestedActionClass::ManagedResume,
                target: TargetScope {
                    target_kind: EntryTargetKind::ManagedWorkspace,
                    object_identity_ref: "obj:managed-ws:01".to_string(),
                    target_label: "Acme managed workspace".to_string(),
                    workspace_scope_label: "Acme / platform".to_string(),
                    tenant_scope_label: Some("Acme Corp".to_string()),
                    channel_or_build_owner_label: Some("stable / admin-owned".to_string()),
                    truth_state: TargetTruthState::ExactAvailable,
                    identity_review_required: true,
                },
                authority_effect: AuthorityEffectClass::PolicyBoundaryReview,
                crosses_tenant_boundary: true,
                canonical_command_id: "cmd:workspace.restore_from_checkpoint".to_string(),
            },
        ),
        (
            "collaboration_join.json",
            EntryInterstitialRequest {
                interstitial_id: "ei:fixture:collab:01".to_string(),
                kind: EntryInterstitialKind::CollaborationJoin,
                source_class: EntrySourceClass::CollaborationService,
                source_label: "Collaboration invite".to_string(),
                requested_action: RequestedActionClass::JoinPresence,
                target: TargetScope {
                    target_kind: EntryTargetKind::CollaborationSession,
                    object_identity_ref: "obj:collab:01".to_string(),
                    target_label: "Pair session".to_string(),
                    workspace_scope_label: "Shared workspace".to_string(),
                    tenant_scope_label: Some("Partner org".to_string()),
                    channel_or_build_owner_label: None,
                    truth_state: TargetTruthState::ExactAvailable,
                    identity_review_required: true,
                },
                authority_effect: AuthorityEffectClass::TenantScopeChange,
                crosses_tenant_boundary: true,
                canonical_command_id: "cmd:collaboration.join_session".to_string(),
            },
        ),
    ]
}

#[test]
fn every_fixture_case_matches_materialized_record() {
    for (name, request) in fixture_cases() {
        let decision = evaluate_entry_interstitial(&request);
        let record = decision
            .record()
            .unwrap_or_else(|| panic!("fixture case {name} must require an interstitial"));
        let produced = serde_json::to_value(record).expect("serialize record");
        let expected = load_fixture_value(name);
        assert_eq!(
            produced, expected,
            "materialized record drifted from {name}"
        );
    }
}

#[test]
fn fixture_plain_local_open_admission() {
    let decision = evaluate_entry_interstitial(&plain_local_deep_link());
    let admission = match decision {
        EntryInterstitialDecision::PlainLocalOpen(admission) => admission,
        other => panic!("expected plain local open, got {other:?}"),
    };
    let produced = serde_json::to_value(&admission).expect("serialize");
    let expected = load_fixture_value("plain_local_open_fast_path.json");
    assert_eq!(produced, expected);
}

/// Returns the request whose fixture file name is `name`.
fn fixture_request(name: &str) -> EntryInterstitialRequest {
    fixture_cases()
        .into_iter()
        .find(|(case_name, _)| *case_name == name)
        .map(|(_, request)| request)
        .unwrap_or_else(|| panic!("no fixture case named {name}"))
}

#[test]
fn fixture_support_packet_matches_projection() {
    let request = fixture_request("managed_resume.json");
    let decision = evaluate_entry_interstitial(&request);
    let record = decision.record().expect("interstitial required");
    let packet = super::support_export::EntryInterstitialSupportPacket::from_record(record);
    let produced = serde_json::to_value(&packet).expect("serialize packet");
    let expected = load_fixture_value("support_packet_managed_resume.json");
    assert_eq!(produced, expected);
}

/// Regenerates every fixture from the materializer. Run with:
/// `cargo test -p aureline-shell -- --ignored regenerate_high_authority_entry_fixtures`
#[test]
#[ignore = "writes fixture files; run manually to regenerate"]
fn regenerate_high_authority_entry_fixtures() {
    let dir = fixture_dir();
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    for (name, request) in fixture_cases() {
        let decision = evaluate_entry_interstitial(&request);
        let record = decision.record().expect("interstitial required");
        let json = serde_json::to_string_pretty(record).expect("serialize");
        std::fs::write(dir.join(name), json + "\n").expect("write fixture");
    }

    let decision = evaluate_entry_interstitial(&plain_local_deep_link());
    if let EntryInterstitialDecision::PlainLocalOpen(admission) = decision {
        let json = serde_json::to_string_pretty(&admission).expect("serialize");
        std::fs::write(dir.join("plain_local_open_fast_path.json"), json + "\n")
            .expect("write fixture");
    } else {
        panic!("expected plain local open");
    }

    // Support-export packet projection for one representative record.
    let decision = evaluate_entry_interstitial(&fixture_request("managed_resume.json"));
    let record = decision.record().expect("interstitial required");
    let packet = super::support_export::EntryInterstitialSupportPacket::from_record(record);
    let json = serde_json::to_string_pretty(&packet).expect("serialize");
    std::fs::write(dir.join("support_packet_managed_resume.json"), json + "\n")
        .expect("write fixture");
}
