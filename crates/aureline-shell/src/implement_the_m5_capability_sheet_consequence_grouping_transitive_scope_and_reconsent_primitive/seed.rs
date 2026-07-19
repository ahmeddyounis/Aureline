// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 capability-sheet primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, the worked resolutions, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical capability-sheet-primitive packet.
pub const M5_CAPABILITY_SHEET_PRIMITIVE_PACKET_ID: &str =
    "m5-capability-sheet-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one requested-capability item with the common defaults filled in.
#[allow(clippy::too_many_arguments)]
fn request(
    capability_token: &str,
    consequence_class: M5CapabilityConsequenceClass,
    purpose: &str,
    decision: M5CapabilityDecision,
    policy_predecision: M5CapabilityPolicyPredecision,
    is_transitive: bool,
    transitive_origin_repr: Option<&str>,
    reduced_mode_available: bool,
    re_consent_triggered: bool,
    has_prior_grant: bool,
) -> M5CapabilityRequestItem {
    M5CapabilityRequestItem {
        capability_token: capability_token.to_owned(),
        consequence_class,
        purpose_repr: purpose.to_owned(),
        decision,
        policy_predecision,
        is_transitive,
        transitive_origin_repr: transitive_origin_repr.map(str::to_owned),
        reduced_mode_available,
        re_consent_triggered,
        has_prior_grant,
    }
}

/// Builds a worked resolution case from a lane, actor, and requests.
fn sheet_case(
    surface_family: M5CapabilitySurfaceFamily,
    actor_identity_repr: &str,
    requests: Vec<M5CapabilityRequestItem>,
) -> M5CapabilitySheetResolutionCase {
    M5CapabilitySheetResolutionCase::resolved(M5CapabilitySheetResolutionInput {
        surface_family,
        actor_identity_repr: actor_identity_repr.to_owned(),
        requests,
    })
}

/// A base row with the shared fields filled in and the full anatomy, consequence,
/// scope-state, consent-disclosure, focus-behavior, and export-field parity every
/// lane carries.
fn base_row(
    surface_family: M5CapabilitySurfaceFamily,
    qualification: M5TrustQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_sheets: Vec<M5CapabilitySheetResolutionCase>,
) -> M5CapabilitySheetSurfaceRow {
    M5CapabilitySheetSurfaceRow {
        surface_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // Capability sheets are transient overlays: a modal / side sheet presented
        // on top of the working surface, never buried in a static settings pane.
        shell_zone_slot: M5ShellZoneSlot::TransientOverlay,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5CapabilitySheetAnatomyPart::ALL.to_vec(),
        consequence_classes: M5CapabilityConsequenceClass::ALL.to_vec(),
        scope_states: M5CapabilityScopeState::ALL.to_vec(),
        consent_disclosures: M5CapabilityConsentDisclosure::ALL.to_vec(),
        focus_behaviors: M5CapabilitySheetFocusBehavior::ALL.to_vec(),
        export_fields: M5CapabilitySheetExportField::ALL.to_vec(),
        accessibility_routes: M5TrustAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5TrustComponentDowngradeTrigger::ConsequenceGroupingDropped,
            M5TrustComponentDowngradeTrigger::TransitiveScopeHidden,
            M5TrustComponentDowngradeTrigger::ReConsentSkipped,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5TrustComponentDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_CAPABILITY_SHEET_SCHEMA_REF,
            M5_CAPABILITY_SHEET_CONTRACT_REF,
            M5_CAPABILITY_SHEET_EFFECTIVE_PERMISSION_REF,
            M5_CAPABILITY_SHEET_PERMISSION_PROMPT_REF,
        ]),
        example_sheets,
        drops_consequence_grouping: false,
        hides_transitive_scope: false,
        skips_required_re_consent: false,
        drops_export_or_audit_truth: false,
    }
}

fn surface_rows() -> Vec<M5CapabilitySheetSurfaceRow> {
    use M5CapabilityConsequenceClass as CC;
    use M5CapabilityDecision as D;
    use M5CapabilityPolicyPredecision as PP;

    let mut rows = Vec::with_capacity(6);

    // 1. Extension install — a read granted in full and a workspace modification
    //    granted at a reduced scope. This is the reduced-mode acceptance-criterion
    //    example: the narrower grant is visible and applied before approval.
    rows.push(base_row(
        M5CapabilitySurfaceFamily::ExtensionInstall,
        M5TrustQualificationClass::Stable,
        "Extension trust owner",
        "The extension-install lane renders the shared capability sheet so an extension's requests read grouped by consequence — a full read of local context and a reduced-scope workspace modification — with the reduced grant disclosed and revocable from the extension trust surface, never a generic 'grant access?' prompt",
        "evidence:m5-capability-sheet-extension:001",
        vec![sheet_case(
            M5CapabilitySurfaceFamily::ExtensionInstall,
            "extension:formatter-pack",
            vec![
                request(
                    "read_workspace_files",
                    CC::ReadLocalContext,
                    "Read workspace files to format them",
                    D::ApproveFull,
                    PP::NoPolicy,
                    false,
                    None,
                    false,
                    false,
                    false,
                ),
                request(
                    "modify_open_documents",
                    CC::ModifyWorkspace,
                    "Apply formatting edits to open documents",
                    D::ApproveReduced,
                    PP::NoPolicy,
                    false,
                    None,
                    true,
                    false,
                    false,
                ),
            ],
        )],
    ));

    // 2. AI tool request — an execute request not yet granted, plus a network
    //    capability that entered scope transitively via a dependency and is
    //    disclosed but not granted. This is the transitive-scope acceptance
    //    example: the widened scope is visible before approval.
    rows.push(base_row(
        M5CapabilitySurfaceFamily::AiToolRequest,
        M5TrustQualificationClass::Stable,
        "AI tool-gateway owner",
        "The AI-tool lane renders the shared capability sheet so an execute-code request reads as requested-not-granted and a network capability pulled in transitively by a dependency is disclosed with its origin before approval, grouped by consequence rather than by tool API name",
        "evidence:m5-capability-sheet-ai:001",
        vec![sheet_case(
            M5CapabilitySurfaceFamily::AiToolRequest,
            "ai-tool:code-search",
            vec![
                request(
                    "run_analysis_command",
                    CC::ExecuteCode,
                    "Run a read-only analysis command over the project",
                    D::RequestedNotGranted,
                    PP::NoPolicy,
                    false,
                    None,
                    false,
                    false,
                    false,
                ),
                request(
                    "reach_index_endpoint",
                    CC::NetworkAccess,
                    "Reach the shared index service the analysis depends on",
                    D::RequestedNotGranted,
                    PP::NoPolicy,
                    true,
                    Some("dependency:shared-index-client"),
                    false,
                    false,
                    false,
                ),
            ],
        )],
    ));

    // 3. Provider route — a network capability that was previously granted and now
    //    requires re-consent because the requested scope widened. The remembered
    //    grant is revocable from the provider trust surface.
    rows.push(base_row(
        M5CapabilitySurfaceFamily::ProviderRoute,
        M5TrustQualificationClass::Stable,
        "Connected-provider registry owner",
        "The provider-route lane renders the shared capability sheet so a remembered network grant whose scope widened reads as re-consent-required rather than silently re-using the old grant, and the grant stays revocable from the provider trust surface",
        "evidence:m5-capability-sheet-provider:001",
        vec![sheet_case(
            M5CapabilitySurfaceFamily::ProviderRoute,
            "provider-route:hosted-model",
            vec![request(
                "reach_provider_endpoint",
                CC::NetworkAccess,
                "Send requests to the connected provider route",
                D::RequestedNotGranted,
                PP::NoPolicy,
                false,
                None,
                false,
                true,
                true,
            )],
        )],
    ));

    // 4. Remote connector — a credential-access capability that was granted and is
    //    now revoked; the revocation is kept in history, not erased.
    rows.push(base_row(
        M5CapabilitySurfaceFamily::RemoteConnector,
        M5TrustQualificationClass::Stable,
        "Remote-connector trust owner",
        "The remote-connector lane renders the shared capability sheet so revoking a credential-access grant reads as revoked-with-history — the change is kept in the chronology and the connector cannot silently keep the access",
        "evidence:m5-capability-sheet-remote:001",
        vec![sheet_case(
            M5CapabilitySurfaceFamily::RemoteConnector,
            "remote-connector:build-farm",
            vec![request(
                "read_credential_handle",
                CC::CredentialAccess,
                "Read the stored credential handle for the remote build farm",
                D::Revoke,
                PP::NoPolicy,
                false,
                None,
                false,
                false,
                true,
            )],
        )],
    ));

    // 5. Automation flow — an execute-code capability pre-approved by policy and
    //    granted in full, alongside a system-control capability pre-denied by
    //    policy that can never be approved locally and reads as requested-not-
    //    granted. This preserves policy pre-approve / pre-deny states.
    rows.push(base_row(
        M5CapabilitySurfaceFamily::AutomationFlow,
        M5TrustQualificationClass::Stable,
        "Automation governance owner",
        "The automation-flow lane renders the shared capability sheet so a policy pre-approved execute-code capability grants in full while a policy pre-denied system-control capability stays requested-not-granted and can never be approved locally — the policy pre-decisions are preserved on the sheet",
        "evidence:m5-capability-sheet-automation:001",
        vec![sheet_case(
            M5CapabilitySurfaceFamily::AutomationFlow,
            "automation-flow:nightly-sync",
            vec![
                request(
                    "run_sync_command",
                    CC::ExecuteCode,
                    "Run the nightly synchronization command",
                    D::ApproveFull,
                    PP::PreApproved,
                    false,
                    None,
                    false,
                    false,
                    false,
                ),
                request(
                    "control_host_process",
                    CC::SystemControl,
                    "Manage host processes during synchronization",
                    D::RequestedNotGranted,
                    PP::PreDenied,
                    false,
                    None,
                    false,
                    false,
                    false,
                ),
            ],
        )],
    ));

    // 6. Privileged helper — a system-control capability granted in full and a
    //    credential capability pulled in transitively and disclosed before
    //    approval. The full grant is revocable from the privileged-helper trust
    //    surface.
    rows.push(base_row(
        M5CapabilitySurfaceFamily::PrivilegedHelper,
        M5TrustQualificationClass::Stable,
        "Privileged-helper trust owner",
        "The privileged-helper lane renders the shared capability sheet so an elevated system-control grant is approved in full and revocable from the helper trust surface, while a credential capability the helper pulls in transitively is disclosed with its origin before approval",
        "evidence:m5-capability-sheet-helper:001",
        vec![sheet_case(
            M5CapabilitySurfaceFamily::PrivilegedHelper,
            "privileged-helper:installer-service",
            vec![
                request(
                    "manage_system_service",
                    CC::SystemControl,
                    "Install and manage the helper system service",
                    D::ApproveFull,
                    PP::NoPolicy,
                    false,
                    None,
                    false,
                    false,
                    false,
                ),
                request(
                    "read_credential_store_entry",
                    CC::CredentialAccess,
                    "Read the credential store entry the installer depends on",
                    D::RequestedNotGranted,
                    PP::NoPolicy,
                    true,
                    Some("dependency:credential-broker-client"),
                    false,
                    false,
                    false,
                ),
            ],
        )],
    ));

    rows
}

fn governance_review() -> M5CapabilitySheetGovernanceReview {
    M5CapabilitySheetGovernanceReview {
        one_sheet_groups_by_consequence: true,
        transitive_scope_always_disclosed: true,
        reduced_mode_visible_before_approval: true,
        policy_and_re_consent_preserved: true,
        remembered_approvals_revocable_from_stable_surface: true,
        support_export_keeps_capability_vocabulary: true,
        no_surface_uses_generic_access_prompt: true,
        every_sheet_bound_to_shell_zone: true,
        every_sheet_declares_accessibility_route: true,
        later_sheets_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5CapabilitySheetConsumerProjection {
    M5CapabilitySheetConsumerProjection {
        trust_lanes_consume_shared_sheet: true,
        resolver_reads_single_scope_ladder: true,
        revoke_path_reads_single_source: true,
        transitive_disclosure_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CapabilitySheetProofFreshness {
    M5CapabilitySheetProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CapabilitySheetReleasePosture {
    M5CapabilitySheetReleasePosture {
        release_packet_ref: M5_CAPABILITY_SHEET_ARTIFACT_REF.to_owned(),
        capability_sheet_audit_ref: M5_CAPABILITY_SHEET_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CAPABILITY_SHEET_SCHEMA_REF,
        M5_CAPABILITY_SHEET_DOC_REF,
        M5_CAPABILITY_SHEET_SHELL_ZONE_REF,
        M5_CAPABILITY_SHEET_COMPONENT_MATRIX_REF,
        M5_CAPABILITY_SHEET_CONTRACT_REF,
        M5_CAPABILITY_SHEET_EFFECTIVE_PERMISSION_REF,
        M5_CAPABILITY_SHEET_PERMISSION_PROMPT_REF,
    ])
}

/// Builds the canonical M5 capability-sheet-primitive packet.
pub fn seeded_m5_capability_sheet_primitive_packet() -> M5CapabilitySheetPrimitivePacket {
    M5CapabilitySheetPrimitivePacket::new(M5CapabilitySheetPrimitivePacketInput {
        packet_id: M5_CAPABILITY_SHEET_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 capability-sheet primitive: consequence grouping, transitive scope, reduced mode, and revoke / re-consent"
                .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5CapabilitySheetVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the automation-flow lane is held at Beta because a slice of
/// policy pre-deny explanations do not yet render on every profile; every lane
/// stays visible.
pub fn seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed(
) -> M5CapabilitySheetPrimitivePacket {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.packet_id = "m5-capability-sheet-primitive:automation-flow-beta:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CapabilitySurfaceFamily::AutomationFlow)
        .expect("automation-flow row present");
    row.qualification = M5TrustQualificationClass::Beta;
    packet
}

/// Narrowed variant: the privileged-helper lane is narrowed to Preview pending
/// transitive-origin disclosure parity across every export path; every lane stays
/// visible.
pub fn seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed(
) -> M5CapabilitySheetPrimitivePacket {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.packet_id = "m5-capability-sheet-primitive:privileged-helper-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CapabilitySurfaceFamily::PrivilegedHelper)
        .expect("privileged-helper row present");
    row.qualification = M5TrustQualificationClass::Preview;
    packet
}
