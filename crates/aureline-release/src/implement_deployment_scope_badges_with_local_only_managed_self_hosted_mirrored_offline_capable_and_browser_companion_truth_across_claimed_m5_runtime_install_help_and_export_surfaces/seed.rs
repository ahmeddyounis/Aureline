//! Canonical seed builders for the M5 deployment-scope badge primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical deployment-scope badge primitive packet.
pub const M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_PACKET_ID: &str =
    "m5-deployment-scope-badge-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full deployment-scope state.
fn case(
    subject_label: &str,
    scope: M5DeploymentScopeBadgeValue,
    residual_dependency_repr: Option<&str>,
    last_evaluated_repr: &str,
) -> M5DeploymentScopeResolutionCase {
    M5DeploymentScopeResolutionCase::resolved(M5DeploymentScopeBadgeInput {
        subject_label: subject_label.to_owned(),
        scope,
        residual_dependency_repr: residual_dependency_repr.map(str::to_owned),
        last_evaluated_repr: last_evaluated_repr.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, scope, sovereignty,
/// residual-dependency, local-safe-continuity, next-action, explanation-field,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5DeploymentScopeConsumerSurface,
    qualification: M5BadgeQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5DeploymentScopeResolutionCase>,
) -> M5DeploymentScopeRow {
    M5DeploymentScopeRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BadgeSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5DeploymentScopeAnatomyPart::ALL.to_vec(),
        scope_values: M5DeploymentScopeBadgeValue::ALL.to_vec(),
        sovereignty_postures: M5DeploymentSovereigntyPosture::ALL.to_vec(),
        residual_dependency_classes: M5ResidualDependencyClass::ALL.to_vec(),
        local_safe_continuities: M5LocalSafeContinuity::ALL.to_vec(),
        next_actions: M5DeploymentScopeNextAction::ALL.to_vec(),
        explanation_fields: M5BadgeExplanationField::ALL.to_vec(),
        export_fields: M5DeploymentScopeExportField::ALL.to_vec(),
        accessibility_routes: M5BadgeAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5BadgeConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5BadgeDowngradeTrigger::DeploymentScopeUnstated,
            M5BadgeDowngradeTrigger::ExplanationDrawerMissing,
            M5BadgeDowngradeTrigger::AxisMergedIntoAnother,
            M5BadgeDowngradeTrigger::FilterKeyDropped,
            M5BadgeDowngradeTrigger::ExportLostBadgeMeaning,
            M5BadgeDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF,
            M5_DEPLOYMENT_SCOPE_BADGE_FAMILY_MATRIX_REF,
            M5_DEPLOYMENT_SCOPE_BADGE_RESIDUAL_REF,
            M5_DEPLOYMENT_SCOPE_BADGE_MIRROR_OFFLINE_REF,
        ]),
        example_resolutions,
        collapses_scope_into_support_lifecycle_or_channel: false,
        implies_lifecycle_from_deployment_scope: false,
        drops_residual_dependency_on_sovereignty_claim: false,
        drops_badge_meaning_in_export: false,
    }
}

fn badge_rows() -> Vec<M5DeploymentScopeRow> {
    use M5DeploymentScopeBadgeValue as Scope;

    vec![
    // 1. Runtime capability row — a local-only capability that is locally sovereign but
    //    still discloses its signing-and-update residual dependency, alongside a
    //    managed capability that is openly provider-governed and has no sovereignty to
    //    overstate (the scope-axis-independence proof: the scope spans locally sovereign
    //    through provider governed as its own axis).
    base_row(
        M5DeploymentScopeConsumerSurface::RuntimeCapabilityRow,
        M5BadgeQualificationClass::Stable,
        "Runtime scope badge owner",
        "The runtime capability row renders the shared deployment-scope badge so a local-only capability reads as locally sovereign while still disclosing the signing-and-update residual dependency it carries, and a managed capability reads as provider-governed with no local authority to overstate — proving the scope is its own axis and never collapses into support class, lifecycle, or channel",
        "evidence:m5-deployment-scope-parity:001",
        vec![
            case(
                "aureline runtime: local workspace index",
                Scope::LocalOnly,
                Some("signing:release-keyring/desktop"),
                "2026-07-01T00:00:00Z",
            ),
            case(
                "aureline runtime: managed capability plane",
                Scope::Managed,
                None,
                "2026-07-02T00:00:00Z",
            ),
        ],
    ),

    // 2. Install / deployment card — a self-hosted install that is operator-governed and
    //    discloses its operator-infrastructure dependency, alongside a managed install.
    base_row(
        M5DeploymentScopeConsumerSurface::InstallDeploymentCard,
        M5BadgeQualificationClass::Stable,
        "Install deployment scope badge owner",
        "The install / deployment card renders the shared deployment-scope badge so a self-hosted install reads as operator-governed while disclosing the operator-infrastructure residual dependency it still relies on, and a managed install reads as provider-governed — the same scope vocabulary an install reviewer reads elsewhere",
        "evidence:m5-deployment-scope-parity:002",
        vec![
            case(
                "aureline install: self-hosted control plane",
                Scope::SelfHosted,
                Some("operator:tenant-infra/postgres-and-object-store"),
                "2026-06-20T00:00:00Z",
            ),
            case(
                "aureline install: managed cloud tier",
                Scope::Managed,
                None,
                "2026-07-03T00:00:00Z",
            ),
        ],
    ),

    // 3. Help / About panel — an offline-capable capability that is offline resilient
    //    within its cached window, alongside a local-only capability, both disclosing
    //    their residual dependencies rather than overstating offline readiness.
    base_row(
        M5DeploymentScopeConsumerSurface::HelpAboutPanel,
        M5BadgeQualificationClass::Stable,
        "Help about scope badge owner",
        "The Help / About panel renders the shared deployment-scope badge so an offline-capable capability reads as offline resilient but only within its cached capability window, and a local-only capability reads as locally sovereign with its signing-and-update dependency stated — deployment posture stays visible whenever capabilities narrow or differ by operating mode",
        "evidence:m5-deployment-scope-parity:003",
        vec![
            case(
                "aureline help: offline docs bundle",
                Scope::OfflineCapable,
                Some("cache:capability-window/14d"),
                "2026-05-14T00:00:00Z",
            ),
            case(
                "aureline help: local shortcuts index",
                Scope::LocalOnly,
                Some("signing:release-keyring/help-pack"),
                "2026-06-30T00:00:00Z",
            ),
        ],
    ),

    // 4. Diagnostics report — a mirrored capability that is mirror synced and discloses
    //    its upstream-mirror-sync dependency, alongside a self-hosted capability.
    base_row(
        M5DeploymentScopeConsumerSurface::DiagnosticsReport,
        M5BadgeQualificationClass::Stable,
        "Diagnostics scope badge owner",
        "The diagnostics report renders the shared deployment-scope badge so a mirrored capability reads as mirror synced and continues with its last mirrored state, disclosing the upstream-mirror-sync residual dependency, and a self-hosted capability reads as operator-governed — the residual-dependency drawer keeps the badge from overstating sovereignty",
        "evidence:m5-deployment-scope-parity:004",
        vec![
            case(
                "aureline diagnostics: mirrored registry snapshot",
                Scope::Mirrored,
                Some("mirror:upstream-sync/registry-snapshot"),
                "2026-06-25T00:00:00Z",
            ),
            case(
                "aureline diagnostics: self-hosted collector",
                Scope::SelfHosted,
                Some("operator:tenant-infra/log-store"),
                "2026-07-04T00:00:00Z",
            ),
        ],
    ),

    // 5. Support export row — a browser-companion capability that is host delegated and
    //    discloses its host-browser-runtime dependency (an explicit product truth, not a
    //    hidden footnote), alongside a managed capability.
    base_row(
        M5DeploymentScopeConsumerSurface::SupportExportRow,
        M5BadgeQualificationClass::Stable,
        "Support export scope badge owner",
        "The support-export row renders the shared deployment-scope badge so a browser-companion capability reads as host delegated and continues within the host session, disclosing the host-browser-runtime residual dependency as an explicit product truth in exported evidence, and a managed capability reads as provider-governed — exported evidence never loses the scope's meaning",
        "evidence:m5-deployment-scope-parity:005",
        vec![
            case(
                "aureline support: browser companion capture",
                Scope::BrowserCompanion,
                Some("host:chromium-companion-runtime"),
                "2026-03-18T00:00:00Z",
            ),
            case(
                "aureline support: managed export service",
                Scope::Managed,
                None,
                "2026-07-05T00:00:00Z",
            ),
        ],
    ),

    // 6. Companion-mode card — a browser-companion capability and an offline-capable
    //    capability, both stated as explicit product truths (the browser-companion +
    //    offline/mirror explicitness proof).
    base_row(
        M5DeploymentScopeConsumerSurface::CompanionModeCard,
        M5BadgeQualificationClass::Stable,
        "Companion mode scope badge owner",
        "The companion-mode card renders the shared deployment-scope badge so a browser-companion capability reads as host delegated within the host session and an offline-capable capability reads as offline resilient within its cached window — browser companion and offline modes remain explicit product truths a user reads directly rather than hidden footnotes",
        "evidence:m5-deployment-scope-parity:006",
        vec![
            case(
                "aureline companion: browser review surface",
                Scope::BrowserCompanion,
                Some("host:companion-webview-runtime"),
                "2026-06-28T00:00:00Z",
            ),
            case(
                "aureline companion: offline follow-up queue",
                Scope::OfflineCapable,
                Some("cache:capability-window/7d"),
                "2026-06-11T00:00:00Z",
            ),
        ],
    ),
    ]
}

fn governance_review() -> M5DeploymentScopeGovernanceReview {
    M5DeploymentScopeGovernanceReview {
        deployment_scope_shown_as_distinct_cue: true,
        scope_never_collapsed_into_support_lifecycle_or_channel: true,
        deployment_scope_never_implies_lifecycle: true,
        deployment_scope_never_implies_support_class: true,
        sovereignty_claim_auto_discloses_residual_dependency: true,
        residual_dependency_note_preserves_scope_context: true,
        browser_companion_and_offline_modes_are_explicit_truths: true,
        local_safe_continuity_never_overstated: true,
        every_badge_opens_explanation_drawer: true,
        every_badge_is_separately_filterable: true,
        exported_evidence_keeps_scope_meaning: true,
        every_row_declares_accessibility_route: true,
    }
}

fn consumer_projection() -> M5DeploymentScopeConsumerProjection {
    M5DeploymentScopeConsumerProjection {
        runtime_install_help_surfaces_consume_shared_scope_badge: true,
        diagnostics_export_companion_surfaces_consume_shared_scope_badge: true,
        scope_filter_reads_single_source: true,
        sovereignty_posture_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DeploymentScopeProofFreshness {
    M5DeploymentScopeProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DeploymentScopeReleasePosture {
    M5DeploymentScopeReleasePosture {
        release_packet_ref: M5_DEPLOYMENT_SCOPE_BADGE_ARTIFACT_REF.to_owned(),
        badge_audit_ref: M5_DEPLOYMENT_SCOPE_BADGE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_DOC_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_FAMILY_MATRIX_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_RESIDUAL_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_MIRROR_OFFLINE_REF,
    ])
}

/// Builds the canonical M5 deployment-scope badge primitive packet.
pub fn seeded_m5_deployment_scope_badge_primitive_packet() -> M5DeploymentScopeBadgePrimitivePacket
{
    M5DeploymentScopeBadgePrimitivePacket::new(M5DeploymentScopeBadgePrimitivePacketInput {
        packet_id: M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 deployment-scope badge primitive: local-only/managed/self-hosted/mirrored/offline-capable/browser-companion operating mode as one distinct, composable cue with residual-dependency and local-safe continuity disclosure"
                .to_owned(),
        badge_rows: badge_rows(),
        vocabulary_set: M5DeploymentScopeVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the companion-mode card is held at Beta because a slice of companion
/// badges do not yet render the residual-dependency drawer on every profile; every badge
/// consumer stays visible.
pub fn seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed(
) -> M5DeploymentScopeBadgePrimitivePacket {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.packet_id = "m5-deployment-scope-badge-primitive:companion-beta:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DeploymentScopeConsumerSurface::CompanionModeCard)
        .expect("companion mode card present");
    row.qualification = M5BadgeQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics report is narrowed to Preview pending
/// residual-dependency parity proof across every export path; every badge consumer stays
/// visible.
pub fn seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed(
) -> M5DeploymentScopeBadgePrimitivePacket {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.packet_id = "m5-deployment-scope-badge-primitive:diagnostics-preview:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DeploymentScopeConsumerSurface::DiagnosticsReport)
        .expect("diagnostics report row present");
    row.qualification = M5BadgeQualificationClass::Preview;
    packet
}
