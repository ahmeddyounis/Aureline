//! Canonical seed builders for the browser-handoff / delegated-credential controls.
//!
//! These builders are the single producer of the checked-in support export and
//! the scenario fixtures. The headless emitter and the inline tests both call
//! them so the in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical browser-handoff / delegated-credential packet.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_PACKET_ID: &str =
    "m5-browser-device-code-handoff-delegated-credential-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card_source_refs() -> Vec<String> {
    strings(&[
        M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SYSTEM_BROWSER_REF,
    ])
}

fn row_source_refs() -> Vec<String> {
    strings(&[
        M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PROJECTION_REF,
    ])
}

/// Builds a browser-or-device-code handoff card, deriving the handoff-boundary class, the
/// out-of-app claim, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn handoff_card(
    card_id: &str,
    provider_org_label: &str,
    auth_handoff_class: M5AuthHandoffClass,
    flow_kind_label: &str,
    credential_class: M5CredentialClass,
    reveal_posture: M5CredentialRevealPosture,
) -> BrowserDeviceCodeHandoffCard {
    let disclosure = resolve_handoff_boundary(auth_handoff_class);
    BrowserDeviceCodeHandoffCard {
        component: M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard,
        card_id: card_id.to_owned(),
        provider_org_label: provider_org_label.to_owned(),
        auth_handoff_class,
        flow_kind_label: flow_kind_label.to_owned(),
        credential_class,
        reveal_posture,
        handoff_boundary_class: disclosure.handoff_boundary_class,
        claims_out_of_app_boundary: disclosure.is_out_of_app_system_browser,
        system_browser_note: if disclosure.needs_system_browser_note {
            "Out-of-app system browser: authentication completes in your trusted browser, not in this app"
                .to_owned()
        } else {
            String::new()
        },
        device_code_note: if disclosure.needs_device_code_note {
            "Enter the shown device code on a second device; the code expires shortly and can be re-issued"
                .to_owned()
        } else {
            String::new()
        },
        local_capture_disclosure_note: if disclosure.needs_local_capture_disclosure_note {
            "This in-app capture is a weaker boundary; the system browser or a device code is preferred"
                .to_owned()
        } else {
            String::new()
        },
        delegated_deferred_note: if disclosure.needs_delegated_deferred_note {
            "This handoff is delegated or deferred until back online; it is not a direct sign-in"
                .to_owned()
        } else {
            String::new()
        },
        fallback_state_note: format!(
            "Fallback state: if the {} path is unavailable, a safer boundary or a local hold is offered",
            auth_handoff_class.as_str()
        ),
        local_continuity_note:
            "Local continuity: cached read-only context still works while the handoff is pending"
                .to_owned(),
        safer_boundary_rationale_note:
            "A safer boundary is preferred because it keeps the raw secret out of this app process"
                .to_owned(),
        storage_and_reveal_note: format!(
            "Obtains class {}; reveal posture {}",
            credential_class.as_str(),
            reveal_posture.as_str()
        ),
        default_actions: BrowserDeviceCodeHandoffAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "provider_org",
            "flow_kind",
            "handoff_boundary",
            "fallback_state",
            "local_continuity",
            "safer_boundary_rationale",
            "credential_class",
            "reveal_posture",
        ]),
        source_contract_refs: card_source_refs(),
        masks_storage_or_reveal_posture: false,
        blurs_handoff_into_generic_sign_in: false,
        uses_friendly_connected_wording: false,
    }
}

/// Builds a delegated-credential row, deriving the identity origin and the required notes
/// from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn delegated_row(
    row_id: &str,
    delegated_identity_state: M5DelegatedIdentityState,
    source_identity_label: &str,
    target_scope: DelegatedTargetScope,
    storage_mode: M5CredentialStorageMode,
    lifecycle_state: M5CredentialLifecycleState,
    policy_owner_label: &str,
) -> DelegatedCredentialRow {
    let disclosure = resolve_delegated_identity_origin(delegated_identity_state, storage_mode);
    DelegatedCredentialRow {
        component: M5CredentialComponentFamily::DelegatedCredentialRow,
        row_id: row_id.to_owned(),
        delegated_identity_state,
        source_identity_label: source_identity_label.to_owned(),
        target_scope,
        target_scope_note: format!(
            "Target scope: this delegation grants access to the {} boundary only",
            target_scope.as_str()
        ),
        storage_mode,
        lifecycle_state,
        expiration_note: format!(
            "Expiration / lifecycle: currently {}; rotation or revoke changes this",
            lifecycle_state.as_str()
        ),
        policy_owner_label: policy_owner_label.to_owned(),
        identity_origin: disclosure.identity_origin,
        claims_locally_stored: disclosure.is_locally_stored,
        forwarded_note: if disclosure.needs_forwarded_note {
            "Forwarded identity: this acts on behalf of another principal, not a local credential"
                .to_owned()
        } else {
            String::new()
        },
        remote_vault_note: if disclosure.needs_remote_vault_note {
            "Remote-vault identity: the material is held in a remote vault, not stored locally"
                .to_owned()
        } else {
            String::new()
        },
        service_issued_note: if disclosure.needs_service_issued_note {
            "Service-issued identity: a service account is acting, not your local identity"
                .to_owned()
        } else {
            String::new()
        },
        revoked_note: if disclosure.needs_revoked_note {
            "Delegation revoked: this identity can no longer act; stop-forward has taken effect"
                .to_owned()
        } else {
            String::new()
        },
        storage_class_note: format!("Storage class: material lives as {}", storage_mode.as_str()),
        identity_and_delegation_note: format!(
            "Acting identity {}; origin classified as {}",
            delegated_identity_state.as_str(),
            disclosure.identity_origin.as_str()
        ),
        default_actions: DelegatedCredentialRowAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "source_identity",
            "target_scope",
            "storage_class",
            "expiration",
            "policy_owner",
            "identity_origin",
        ]),
        source_contract_refs: row_source_refs(),
        masks_forwarded_or_delegated_identity: false,
        implies_raw_secret_exportable: false,
        uses_friendly_connected_wording: false,
    }
}

fn handoff_cards() -> Vec<BrowserDeviceCodeHandoffCard> {
    use M5AuthHandoffClass as Handoff;
    use M5CredentialClass as Class;
    use M5CredentialRevealPosture as Reveal;

    vec![
        // 1. System-browser redirect: an out-of-app boundary — the safest path.
        handoff_card(
            "handoff-system-browser",
            "GitHub (acme-org)",
            Handoff::SystemBrowserRedirect,
            "System-browser sign-in",
            Class::OauthToken,
            Reveal::HandleOnly,
        ),
        // 2. Device-code poll: a secondary-device boundary that names its code / expiry.
        handoff_card(
            "handoff-device-code",
            "Azure DevOps (contoso)",
            Handoff::DeviceCodePoll,
            "Device-code sign-in",
            Class::DeviceCodeGrant,
            Reveal::NeverRevealed,
        ),
        // 3. Embedded prompt: an in-app local capture that must say a safer boundary is preferred.
        handoff_card(
            "handoff-embedded-prompt",
            "Self-hosted GitLab (internal)",
            Handoff::EmbeddedPrompt,
            "In-app credential capture",
            Class::PersonalAccessToken,
            Reveal::MaskedLastFour,
        ),
        // 4. Passkey step-up: also an in-app local capture boundary.
        handoff_card(
            "handoff-passkey-step-up",
            "Aureline account (self)",
            Handoff::PasskeyStepUp,
            "Passkey step-up",
            Class::ClientCertificate,
            Reveal::ClipboardScoped,
        ),
        // 5. Delegated forward: a delegated handoff, never a direct sign-in.
        handoff_card(
            "handoff-delegated-forward",
            "Release bot (platform-team)",
            Handoff::DelegatedForward,
            "Delegated forward to release bot",
            Class::SshOrSigningKey,
            Reveal::PolicyBlockedReveal,
        ),
        // 6. Offline deferred: a handoff deferred until back online.
        handoff_card(
            "handoff-offline-deferred",
            "npm registry (mirror)",
            Handoff::OfflineDeferred,
            "Offline-deferred sign-in",
            Class::ApiKey,
            Reveal::HandleOnly,
        ),
    ]
}

fn delegated_rows() -> Vec<DelegatedCredentialRow> {
    use DelegatedTargetScope as Scope;
    use M5CredentialLifecycleState as Lifecycle;
    use M5CredentialStorageMode as Storage;
    use M5DelegatedIdentityState as Identity;

    vec![
        // 1. Locally stored: the local identity acting directly, backed by the OS keychain.
        delegated_row(
            "delegated-local",
            Identity::LocalIdentity,
            "You (local identity)",
            Scope::Provider,
            Storage::OsKeychain,
            Lifecycle::ActiveCurrent,
            "You",
        ),
        // 2. Forwarded: a forwarded identity, distinct from a locally stored credential.
        delegated_row(
            "delegated-forwarded",
            Identity::ForwardedIdentity,
            "Forwarded from teammate (a.jordan)",
            Scope::Registry,
            Storage::EncryptedVault,
            Lifecycle::RefreshNeeded,
            "Registry admin",
        ),
        // 3. Remote-vault: delegated on behalf, material held in a broker handle.
        delegated_row(
            "delegated-remote-vault",
            Identity::DelegatedOnBehalf,
            "On behalf of release owner",
            Scope::Release,
            Storage::SecretBrokerHandle,
            Lifecycle::RotationDue,
            "Release policy owner",
        ),
        // 4. Remote-vault: impersonation scoped, material held as an external reference.
        delegated_row(
            "delegated-impersonation",
            Identity::ImpersonationScoped,
            "Impersonating support agent (scoped)",
            Scope::Request,
            Storage::ExternalReference,
            Lifecycle::Superseded,
            "Support policy owner",
        ),
        // 5. Service-issued: a service account acting, regardless of storage.
        delegated_row(
            "delegated-service-account",
            Identity::ServiceAccountActing,
            "CI service account (ci-runner)",
            Scope::Database,
            Storage::SessionMemoryOnly,
            Lifecycle::Expired,
            "Platform policy owner",
        ),
        // 6. Forwarded + revoked: a revoked delegation that must name its revoked state.
        delegated_row(
            "delegated-revoked",
            Identity::DelegationRevoked,
            "Revoked forward (former contractor)",
            Scope::Remote,
            Storage::EncryptedVault,
            Lifecycle::Revoked,
            "Security policy owner",
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CredentialDowngradeTrigger> {
    vec![
        M5CredentialDowngradeTrigger::AuthHandoffClassUnstated,
        M5CredentialDowngradeTrigger::DelegatedIdentityUnstated,
        M5CredentialDowngradeTrigger::LifecycleStateHidden,
        M5CredentialDowngradeTrigger::RevealPostureUnstated,
        M5CredentialDowngradeTrigger::CredentialClassUnstated,
        M5CredentialDowngradeTrigger::AlternateStateLabelInvented,
        M5CredentialDowngradeTrigger::FriendlyConnectedWordingUsed,
        M5CredentialDowngradeTrigger::SessionOnlyFallbackHidden,
        M5CredentialDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> BrowserHandoffDelegatedCredentialTrustReview {
    BrowserHandoffDelegatedCredentialTrustReview {
        card_shows_provider_org_and_flow_kind: true,
        handoff_flows_never_blur_into_generic_sign_in: true,
        system_browser_device_code_local_capture_stay_distinct: true,
        safer_boundary_rationale_always_shown: true,
        card_shows_fallback_state_and_local_continuity: true,
        device_code_and_expiry_shown_where_relevant: true,
        row_shows_source_identity_and_target_scope: true,
        row_shows_storage_class_and_expiration: true,
        identity_origin_derived_never_asserted: true,
        forwarded_delegated_never_reads_as_local: true,
        remote_vault_and_service_issued_stay_distinct: true,
        policy_owner_and_stop_forward_rotate_present: true,
        raw_secret_handling_never_normalized: true,
        no_friendly_connected_wording: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> BrowserHandoffDelegatedCredentialConsumerProjection {
    BrowserHandoffDelegatedCredentialConsumerProjection {
        card_shows_flow_kind_and_boundary_without_docs: true,
        safer_boundary_visible_before_local_capture: true,
        row_shows_identity_origin_and_scope_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> BrowserHandoffDelegatedCredentialProofFreshness {
    BrowserHandoffDelegatedCredentialProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF,
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
        M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical browser-handoff / delegated-credential controls packet.
pub fn seeded_browser_handoff_delegated_credential_controls(
) -> BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket {
    BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket::new(
        BrowserDeviceCodeHandoffDelegatedCredentialControlsPacketInput {
            packet_id: BROWSER_HANDOFF_DELEGATED_CREDENTIAL_PACKET_ID.to_owned(),
            surface_label:
                "M5 browser-or-device-code handoff cards and delegated-credential rows: provider/org, auth-handoff flow kind, derived handoff boundary, fallback state, local continuity, device code/expiry, why a safer boundary is preferred, source identity, target scope, storage class, expiration, policy owner, and local-versus-forwarded-versus-remote-vault-versus-service-issued identity origin"
                    .to_owned(),
            handoff_cards: handoff_cards(),
            delegated_rows: delegated_rows(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
            trust_review: trust_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights an in-app local-capture handoff card that must disclose
/// why a safer boundary is preferred and never blur into a generic sign-in state. Every
/// auth-handoff class and handoff-boundary class stays covered so the fixture validates on
/// its own.
pub fn seeded_browser_handoff_delegated_credential_controls_handoff_local_capture(
) -> BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket {
    let mut packet = seeded_browser_handoff_delegated_credential_controls();
    packet.packet_id =
        "m5-browser-device-code-handoff-delegated-credential-controls:fixture:handoff-local-capture"
            .to_owned();
    packet.surface_label =
        "M5 handoff cards: an in-app local capture always says why a safer boundary is preferred"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a forwarded / delegated identity that must never read as a
/// locally stored credential. Every delegated-identity state and identity origin stays
/// covered so the fixture validates on its own.
pub fn seeded_browser_handoff_delegated_credential_controls_delegated_forwarded_identity(
) -> BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket {
    let mut packet = seeded_browser_handoff_delegated_credential_controls();
    packet.packet_id =
        "m5-browser-device-code-handoff-delegated-credential-controls:fixture:delegated-forwarded-identity"
            .to_owned();
    packet.surface_label =
        "M5 delegated rows: a forwarded or delegated identity never reads as locally stored"
            .to_owned();
    packet
}
