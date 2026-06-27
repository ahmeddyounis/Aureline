//! Canonical seed builders for the M5 runbook handoff register.
//!
//! These builders are the single producer of the checked-in handoff register, the
//! published inventory, the Markdown proof, and the per-handoff projection fixtures.
//! The headless emitter and the inline tests both call them so the in-code register,
//! the artifacts, and the fixtures never drift. The register reuses the live handoff
//! packets embedded in the governance lane's operator-scenario execution records, so
//! the same pivots operator history and incident packets surface are the ones the
//! register catalogs — a handoff is never a privileged exception path.

use super::*;

use crate::m5_runbook_governance::{
    seeded_operator_scenario_records, ControlPlaneHandoffPacket, HandoffDestinationClass,
    HandoffReasonClass, ReturnAnchor, ReturnAnchorObjectClass, M5_RUNBOOK_MESSAGE_ID_PREFIX,
};

/// Stable register id for the canonical handoff register.
pub const M5_RUNBOOK_HANDOFF_REGISTER_ID: &str = "m5-runbook-handoff-register:stable:0001";

/// Evaluation / mint timestamp for the canonical register.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The live handoffs embedded in the operator-scenario execution records. Cloning
/// them here is what keeps the register and the execution truth the same objects.
fn operator_scenario_handoffs() -> Vec<ControlPlaneHandoffPacket> {
    seeded_operator_scenario_records()
        .iter()
        .flat_map(|r| r.executed_steps.clone())
        .filter_map(|s| s.handoff)
        .collect()
}

/// Builds a catalog handoff packet that demonstrates one destination class without
/// being embedded in an execution record. Its return anchor still names a complete,
/// reachable initiating object so the pivot stays attributable.
#[allow(clippy::too_many_arguments)]
fn catalog_handoff(
    handoff_id: &str,
    destination_class: HandoffDestinationClass,
    reason_class: HandoffReasonClass,
    initiating_object_class: ReturnAnchorObjectClass,
    initiating_object_ref: &str,
    target_continuity_ref: &str,
    evidence_continuity_ref: &str,
    narrowed_authority: Option<&str>,
) -> ControlPlaneHandoffPacket {
    ControlPlaneHandoffPacket {
        handoff_id: handoff_id.to_owned(),
        boundary_class: destination_class.boundary_class(),
        destination_class,
        reason_class,
        reference_plane_state: destination_class.reference_plane_state(),
        target_ref: format!("console-ref:{handoff_id}"),
        destination_object_ref: format!("console-object:{handoff_id}"),
        attribution_ref: format!("session-ref:{handoff_id}"),
        return_anchor: ReturnAnchor {
            initiating_object_class,
            initiating_object_ref: initiating_object_ref.to_owned(),
            target_continuity_ref: target_continuity_ref.to_owned(),
            evidence_continuity_ref: evidence_continuity_ref.to_owned(),
            return_message_id: format!(
                "{}handoff.{}.return",
                M5_RUNBOOK_MESSAGE_ID_PREFIX, handoff_id
            ),
        },
        narrowed_authority_message_id: narrowed_authority.map(|m| {
            format!(
                "{}handoff.{}.narrowed.{m}",
                M5_RUNBOOK_MESSAGE_ID_PREFIX, handoff_id
            )
        }),
        returns_to_governed_plane: true,
        creates_hidden_mutate_channel: false,
        detail_message_id: format!("{}handoff.{}", M5_RUNBOOK_MESSAGE_ID_PREFIX, handoff_id),
    }
}

/// A browser-only reference-documentation consultation: read-only, never control.
fn browser_reference_consult() -> ControlPlaneHandoffPacket {
    catalog_handoff(
        "vendor-scaling-docs",
        HandoffDestinationClass::BrowserReferenceDoc,
        HandoffReasonClass::ConsultReferenceDocumentation,
        ReturnAnchorObjectClass::RunbookStep,
        "vendor-console-handoff:step:vendor.console",
        "target:vendor-console/scaling-group",
        "evidence:vendor:handoff",
        None,
    )
}

/// A hosted browser application surface that is itself the true control plane.
fn hosted_dashboard_inspect() -> ControlPlaneHandoffPacket {
    catalog_handoff(
        "hosted-status-dashboard",
        HandoffDestinationClass::BrowserAppSurface,
        HandoffReasonClass::InspectVendorState,
        ReturnAnchorObjectClass::IncidentWorkspace,
        "incident:vendor-scale:0014",
        "target:vendor-console/scaling-group",
        "evidence:vendor:status",
        Some("read_only_dashboard"),
    )
}

/// An external authentication authority challenge before returning to the runbook.
fn sso_challenge() -> ControlPlaneHandoffPacket {
    catalog_handoff(
        "identity-provider-sso",
        HandoffDestinationClass::ExternalAuthAuthority,
        HandoffReasonClass::CompleteAuthChallenge,
        ReturnAnchorObjectClass::RunbookExecution,
        "vendor-console-handoff",
        "target:vendor-console/scaling-group",
        "evidence:vendor:handoff",
        None,
    )
}

/// The checked-in governed handoff packets: the live operator-scenario handoffs plus
/// catalog handoffs demonstrating every destination class and both reference-plane
/// states.
pub fn seeded_runbook_handoff_packets() -> Vec<ControlPlaneHandoffPacket> {
    let mut handoffs = operator_scenario_handoffs();
    handoffs.push(browser_reference_consult());
    handoffs.push(hosted_dashboard_inspect());
    handoffs.push(sso_challenge());
    handoffs
}

/// Builds one reference-plane catalog entry.
fn reference_plane_entry(
    entry_id: &str,
    entry_label: &str,
    destination_class: HandoffDestinationClass,
) -> ReferencePlaneEntry {
    let reference_plane_state = destination_class.reference_plane_state();
    ReferencePlaneEntry {
        record_kind: M5_RUNBOOK_REFERENCE_PLANE_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_HANDOFF_SCHEMA_VERSION,
        entry_id: entry_id.to_owned(),
        entry_label: entry_label.to_owned(),
        destination_class,
        reference_plane_state,
        is_true_control_plane: reference_plane_state.is_handoff_required(),
        is_reference_only: reference_plane_state.is_reference_only(),
        executable_in_product: false,
        authority_note_message_id: format!(
            "{}reference_plane.{}.authority",
            M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX, entry_id
        ),
        detail_message_id: format!(
            "{}reference_plane.{}",
            M5_RUNBOOK_HANDOFF_MESSAGE_ID_PREFIX, entry_id
        ),
    }
}

/// The checked-in reference-plane catalog: the destinations that remain the true
/// control plane (handoff-required) and the browser-only reference docs that stay
/// reference-only and can never present as in-product control.
pub fn seeded_reference_plane_entries() -> Vec<ReferencePlaneEntry> {
    vec![
        reference_plane_entry(
            "ref:vendor-scaling-console",
            "Vendor scaling console (true control plane)",
            HandoffDestinationClass::VendorConsole,
        ),
        reference_plane_entry(
            "ref:vendor-scaling-docs",
            "Vendor scaling reference docs (browser, read-only)",
            HandoffDestinationClass::BrowserReferenceDoc,
        ),
        reference_plane_entry(
            "ref:hosted-status-dashboard",
            "Hosted status dashboard (browser app surface)",
            HandoffDestinationClass::BrowserAppSurface,
        ),
        reference_plane_entry(
            "ref:identity-provider-sso",
            "Identity provider SSO challenge (external auth authority)",
            HandoffDestinationClass::ExternalAuthAuthority,
        ),
    ]
}

/// The canonical runbook handoff register: every governed handoff projected into the
/// shared transition vocabulary, plus the reference-plane catalog.
pub fn seeded_m5_runbook_handoff_register() -> M5RunbookHandoffRegister {
    M5RunbookHandoffRegister::new(M5RunbookHandoffRegisterInput {
        register_id: M5_RUNBOOK_HANDOFF_REGISTER_ID.to_owned(),
        report_label: "M5 runbook control-plane handoff register".to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        handoffs: seeded_runbook_handoff_packets(),
        reference_plane: seeded_reference_plane_entries(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}
