//! Canonical seed builders for the M5 provider-account-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical account-row primitive packet.
pub const M5_PROVIDER_ACCOUNT_ROW_PACKET_ID: &str = "m5-provider-account-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked provider-account-row resolution case from a full account state.
#[allow(clippy::too_many_arguments)]
fn account_case(
    identity_class: M5ProviderIdentityClass,
    connection_state: M5AccountConnectionState,
    tenant_scope: M5TenantScopeClass,
    write_scope: M5ProviderWriteScope,
    session_freshness: M5ProviderAccountSessionFreshness,
    has_local_drafts: bool,
    account_label: &str,
    account_identity_ref: &str,
) -> M5ProviderAccountRowResolutionCase {
    M5ProviderAccountRowResolutionCase::resolved(M5ProviderAccountRowResolutionInput {
        identity_class,
        connection_state,
        tenant_scope,
        write_scope,
        session_freshness,
        has_local_drafts,
        account_label: account_label.to_owned(),
        account_identity_ref: account_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full account-row anatomy, identity
/// class, connection state, tenant scope, write scope, session freshness, posture, access
/// capability, action, export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5ProviderAccountConsumerSurface,
    qualification: M5ProviderQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    account_examples: Vec<M5ProviderAccountRowResolutionCase>,
) -> M5ProviderAccountConsumerRow {
    M5ProviderAccountConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ProviderSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ProviderDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ProviderAccountRowAnatomyPart::ALL.to_vec(),
        identity_classes: M5ProviderIdentityClass::ALL.to_vec(),
        connection_states: M5AccountConnectionState::ALL.to_vec(),
        tenant_scopes: M5TenantScopeClass::ALL.to_vec(),
        write_scopes: M5ProviderWriteScope::ALL.to_vec(),
        session_freshness_states: M5ProviderAccountSessionFreshness::ALL.to_vec(),
        row_postures: M5ProviderAccountRowPosture::ALL.to_vec(),
        access_capabilities: M5ProviderAccountAccessCapability::ALL.to_vec(),
        row_actions: M5ProviderAccountRowAction::ALL.to_vec(),
        export_fields: M5ProviderAccountRowExportField::ALL.to_vec(),
        accessibility_routes: M5ProviderAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ProviderConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ProviderDowngradeTrigger::ConnectionStateUnstated,
            M5ProviderDowngradeTrigger::TenantScopeUnstated,
            M5ProviderDowngradeTrigger::WriteScopeUnstated,
            M5ProviderDowngradeTrigger::AlternateStateLabelInvented,
            M5ProviderDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF,
            M5_PROVIDER_ACCOUNT_ROW_CONNECTED_ACCOUNT_REF,
            M5_PROVIDER_ACCOUNT_ROW_ACCOUNT_SCOPE_REF,
        ]),
        account_examples,
        masks_connection_or_scope: false,
        collapses_states_into_generic_connected: false,
        overstates_cached_as_live: false,
        forces_blind_credential_reentry: false,
    }
}

fn rows() -> Vec<M5ProviderAccountConsumerRow> {
    use M5AccountConnectionState as State;
    use M5ProviderAccountSessionFreshness as Fresh;
    use M5ProviderIdentityClass as Identity;
    use M5ProviderWriteScope as Scope;
    use M5TenantScopeClass as Tenant;

    vec![
        // 1. Account-settings panel — a fully signed-in org member with full write and a
        //    fresh session (the highest-trust read-and-write row), and an unlinked, not-yet-
        //    configured account that offers only sign-in.
        base_row(
            M5ProviderAccountConsumerSurface::AccountSettingsPanel,
            M5ProviderQualificationClass::Stable,
            "Account settings panel owner",
            "The account-settings panel renders the shared provider-account row so a signed-in organization member with full write and a fresh session reads as the highest-trust signed-in row exposing reveal/remove, and an unlinked account with nothing configured reads as a not-configured row that can only be signed in — never a generic connected chip",
            "evidence:m5-account-row-account-settings:001",
            vec![
                account_case(
                    Identity::OrganizationMember,
                    State::SignedIn,
                    Tenant::OrgScoped,
                    Scope::FullWrite,
                    Fresh::FreshSession,
                    false,
                    "acme-eng org account",
                    "account:acme-eng:org-member",
                ),
                account_case(
                    Identity::UnlinkedIdentity,
                    State::NotConfigured,
                    Tenant::UnknownTenant,
                    Scope::ScopeUnknown,
                    Fresh::NeverAuthenticated,
                    false,
                    "no provider connected",
                    "account:unconfigured:slot-1",
                ),
            ],
        ),
        // 2. Provider status bar — a limited-scope personal account with comment-only write
        //    (a limited read/write row that offers retry), and a signed-in service account
        //    with read-only write scope (a live read-only row).
        base_row(
            M5ProviderAccountConsumerSurface::ProviderStatusBar,
            M5ProviderQualificationClass::Stable,
            "Provider status bar owner",
            "The provider status bar renders the shared provider-account row so a limited-scope personal account with comment-only write reads as a limited-scope row that can still read and comment and offers retry, and a signed-in service account with a read-only scope reads as a live read-only row that never implies write — so a user can tell read from write from the row alone",
            "evidence:m5-account-row-status-bar:001",
            vec![
                account_case(
                    Identity::PersonalAccount,
                    State::LimitedScope,
                    Tenant::PersonalScope,
                    Scope::CommentOnly,
                    Fresh::FreshSession,
                    false,
                    "personal github (comment scope)",
                    "account:personal:comment-scope",
                ),
                account_case(
                    Identity::ServiceAccount,
                    State::SignedIn,
                    Tenant::SingleTenant,
                    Scope::ReadOnly,
                    Fresh::FreshSession,
                    false,
                    "read-only CI service account",
                    "account:service:read-only",
                ),
            ],
        ),
        // 3. Connection picker — a stale-session org member holding local drafts (a cached-
        //    inspect-only row that must reauth without losing drafts), and an offline cached-
        //    read delegated credential (also cached-inspect-only); proves cached never reads
        //    as live and retry/remove preserve drafts.
        base_row(
            M5ProviderAccountConsumerSurface::ConnectionPicker,
            M5ProviderQualificationClass::Stable,
            "Connection picker owner",
            "The connection picker renders the shared provider-account row so a stale-session org member with local drafts reads as a cached-inspect-only row that must re-authenticate before a live write yet keeps its drafts and offers retry, and an offline cached-read delegated credential reads as a cached-inspect-only row — neither ever presenting a cached read as a live read/write",
            "evidence:m5-account-row-connection-picker:001",
            vec![
                account_case(
                    Identity::OrganizationMember,
                    State::StaleSession,
                    Tenant::OrgScoped,
                    Scope::FullWrite,
                    Fresh::ExpiredSession,
                    true,
                    "acme-eng org account (session expired)",
                    "account:acme-eng:stale-session",
                ),
                account_case(
                    Identity::DelegatedCredential,
                    State::OfflineCachedRead,
                    Tenant::MultiTenant,
                    Scope::NoWrite,
                    Fresh::UnknownFreshness,
                    true,
                    "delegated credential (offline mirror)",
                    "account:delegated:offline-cached",
                ),
            ],
        ),
        // 4. Headless / CLI accounts — a policy-blocked installation grant with a revoked
        //    token (a no-access row that still offers retry/remove), and a signed-in org
        //    member with status-only write on a near-expiry session (a limited read/write row
        //    that offers retry ahead of expiry); proves the same grammar works headless.
        base_row(
            M5ProviderAccountConsumerSurface::HeadlessCliAccounts,
            M5ProviderQualificationClass::Stable,
            "Headless CLI accounts owner",
            "The headless / CLI accounts surface renders the shared provider-account row so a policy-blocked installation grant with a revoked token reads as a no-access row that still offers retry and remove without blind re-entry, and a signed-in org member with status-only write on a near-expiry session reads as a limited read/write row that offers retry ahead of expiry — proving the same account grammar works headless",
            "evidence:m5-account-row-headless-cli:001",
            vec![
                account_case(
                    Identity::InstallationGrant,
                    State::PolicyBlocked,
                    Tenant::ProjectScoped,
                    Scope::NoWrite,
                    Fresh::RevokedToken,
                    false,
                    "installation grant (policy blocked)",
                    "account:install-grant:policy-blocked",
                ),
                account_case(
                    Identity::OrganizationMember,
                    State::SignedIn,
                    Tenant::OrgScoped,
                    Scope::StatusOnly,
                    Fresh::NearExpiry,
                    false,
                    "acme-eng status-only bot",
                    "account:acme-eng:status-only",
                ),
            ],
        ),
        // 5. Support account export — a fully signed-in personal account with full write and
        //    local drafts (a read-and-write row whose support export preserves continuity),
        //    and a limited-scope org member on a read-only scope near expiry (a live read-only
        //    row that offers retry); the same row a support agent reads elsewhere.
        base_row(
            M5ProviderAccountConsumerSurface::SupportAccountExport,
            M5ProviderQualificationClass::Stable,
            "Support account export owner",
            "The support account export renders the shared provider-account row so a signed-in personal account with full write and local drafts reads as a read-and-write row whose export preserves support continuity without leaking credentials, and a limited-scope org member on a read-only scope near expiry reads as a live read-only row that offers retry — the same row a support agent reads elsewhere",
            "evidence:m5-account-row-support-export:001",
            vec![
                account_case(
                    Identity::PersonalAccount,
                    State::SignedIn,
                    Tenant::PersonalScope,
                    Scope::FullWrite,
                    Fresh::FreshSession,
                    true,
                    "personal account (full write)",
                    "account:personal:full-write",
                ),
                account_case(
                    Identity::OrganizationMember,
                    State::LimitedScope,
                    Tenant::OrgScoped,
                    Scope::ReadOnly,
                    Fresh::NearExpiry,
                    false,
                    "acme-eng read-only reviewer",
                    "account:acme-eng:read-only-reviewer",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5ProviderAccountRowGovernanceReview {
    M5ProviderAccountRowGovernanceReview {
        account_row_shows_provider_identity: true,
        account_row_shows_connection_state: true,
        account_row_shows_tenant_scope: true,
        account_row_shows_write_scope: true,
        account_row_shows_session_freshness: true,
        account_row_shows_access_capability: true,
        states_never_collapse_into_generic_connected: true,
        cached_inspect_never_reads_as_live: true,
        retry_remove_preserve_local_drafts: true,
        account_rows_stable_across_deployment_lines: true,
        account_rows_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_account_truth: true,
        later_rows_cannot_invent_parallel_account_vocabulary: true,
    }
}

fn consumer_projection() -> M5ProviderAccountRowConsumerProjection {
    M5ProviderAccountRowConsumerProjection {
        provider_surfaces_consume_account_vocabulary: true,
        row_posture_reads_single_source: true,
        access_capability_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5ProviderAccountRowProofFreshness {
    M5ProviderAccountRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ProviderAccountRowReleasePosture {
    M5ProviderAccountRowReleasePosture {
        release_packet_ref: M5_PROVIDER_ACCOUNT_ROW_ARTIFACT_REF.to_owned(),
        provider_account_audit_ref: M5_PROVIDER_ACCOUNT_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF,
        M5_PROVIDER_ACCOUNT_ROW_DOC_REF,
        M5_PROVIDER_ACCOUNT_ROW_COMPONENT_MATRIX_REF,
        M5_PROVIDER_ACCOUNT_ROW_CONNECTED_ACCOUNT_REF,
        M5_PROVIDER_ACCOUNT_ROW_ACCOUNT_SCOPE_REF,
    ])
}

/// Builds the canonical M5 provider-account-row packet.
pub fn seeded_m5_provider_account_row_packet() -> M5ProviderAccountRowPacket {
    M5ProviderAccountRowPacket::new(M5ProviderAccountRowPacketInput {
        packet_id: M5_PROVIDER_ACCOUNT_ROW_PACKET_ID.to_owned(),
        matrix_label:
            "M5 provider-account-row primitive: provider identity, not-configured/signed-in/limited-scope/stale-session/offline-cached-read/policy-blocked connection state, tenant/org scope, effective write scope, token/session freshness, derived row posture, read/write/inspect access capability, and bounded reveal-scope/sign-in/retry/remove/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5ProviderAccountRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the connection-picker consumer is narrowed to Preview pending
/// cached-versus-live access-capability parity proof across every deployment; every
/// consumer stays visible.
pub fn seeded_m5_provider_account_row_connection_picker_preview_narrowed(
) -> M5ProviderAccountRowPacket {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.packet_id =
        "m5-provider-account-row-primitive:connection-picker-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ProviderAccountConsumerSurface::ConnectionPicker)
        .expect("connection-picker row present");
    row.qualification = M5ProviderQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI accounts consumer is held at Beta because a slice
/// of headless rows do not yet render the keyboard route cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed(
) -> M5ProviderAccountRowPacket {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.packet_id =
        "m5-provider-account-row-primitive:headless-cli-accounts-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ProviderAccountConsumerSurface::HeadlessCliAccounts)
        .expect("headless-cli-accounts row present");
    row.qualification = M5ProviderQualificationClass::Beta;
    packet
}
