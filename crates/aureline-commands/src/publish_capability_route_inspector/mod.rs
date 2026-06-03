//! Published capability-route inspector across deep links, approvals, tunnels,
//! remote targets, provider handoffs, and replay-safe command flows.
//!
//! This module consolidates the route-origin and capability-disclosure lane
//! (owned by [`crate::stabilize_client_origin_route_class`]) into one
//! export-safe proof packet that binds the [`CapabilityRouteInspector`] to the
//! six externally routed or high-risk flow families every stable surface must
//! cover:
//!
//! - **deep-link flows** — protocol-handler and share-link invocations that
//!   reach a command from outside the local process;
//! - **approval flows** — preview-gated, approval-lined write-capable paths;
//! - **tunnel flows** — SSH, dev-tunnel, reverse-tunnel, and provider-tunnel
//!   routed actions;
//! - **remote-target flows** — actions whose effects land on a remote workspace
//!   or helper;
//! - **provider-handoff flows** — browser-companion and provider-callback
//!   handoffs that cross to an external managed or BYOK provider; and
//! - **replay-safe command flows** — rerun, recipe, and macro automation paths
//!   that replay a prior command without silently widening authority.
//!
//! For each flow family the packet records:
//!
//! - the **inspector publication** — the capability-route inspector is reachable,
//!   shows route class, target identity, capability boundary, approval scope,
//!   expiry, and revalidation triggers;
//! - the **lineage preservation** — one machine-readable lineage object survives
//!   from preview through execution, audit, support export, and shiproom proof
//!   without requiring private-team reconstruction;
//! - the **drift-and-reapproval policy** — route drift, target drift, policy
//!   drift, host drift, or approval expiry forces visible reapproval or replay
//!   review instead of silent replay;
//! - the **reversibility guard** — browser and provider handoffs stay typed and
//!   reversible; opening externally may not widen authority beyond what the
//!   inspector disclosed; and
//! - the **keyboard reachability** — the inspector is reachable from review
//!   sheets, command previews, and diagnostic/support surfaces via keyboard.
//!
//! It does not re-derive the descriptor, registry, invocation, authority,
//! high-risk hardening, command-parity, client-origin/route-class, or
//! traffic-origin/exposure-chip models. This packet references them by stable
//! schema ref and adds the flow-specific publication, lineage, drift,
//! reversibility, and keyboard-reachability invariants the stable line needs.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw URLs, hostnames, IPs, ports, paths,
//! query strings, credentials, and billing-account ids stay outside the support
//! boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_client_origin_route_class::CapabilityRouteInspector;
use crate::stabilize_command_contract::{
    CommandContractEvidenceExport, CommandSurfaceClass, StableContractRefs,
    SurfaceQualificationClass,
};

/// Stable record-kind tag carried by [`CapabilityRouteInspectorPacket`].
pub const CAPABILITY_ROUTE_INSPECTOR_RECORD_KIND: &str = "capability_route_inspector_packet";

/// Schema version for capability-route inspector publication records.
pub const CAPABILITY_ROUTE_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CAPABILITY_ROUTE_INSPECTOR_SCHEMA_REF: &str =
    "schemas/commands/capability_route_inspector.schema.json";

/// Repo-relative path of the publication doc.
pub const CAPABILITY_ROUTE_INSPECTOR_DOC_REF: &str =
    "docs/commands/m4/publish_capability_route_inspector.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const CAPABILITY_ROUTE_INSPECTOR_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const CAPABILITY_ROUTE_INSPECTOR_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const CAPABILITY_ROUTE_INSPECTOR_FIXTURE_DIR: &str =
    "fixtures/commands/m4/publish_capability_route_inspector";

/// Repo-relative path of the checked-in support export artifact.
pub const CAPABILITY_ROUTE_INSPECTOR_ARTIFACT_REF: &str =
    "artifacts/commands/m4/publish_capability_route_inspector/support_export.json";

/// Repo-relative path of the checked-in Markdown summary.
pub const CAPABILITY_ROUTE_INSPECTOR_SUMMARY_REF: &str =
    "artifacts/commands/m4/publish_capability_route_inspector/summary.md";

/// Flow family across which the capability-route inspector must be published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowClass {
    /// Protocol-handler or share-link deep-link invocation.
    DeepLink,
    /// Preview-gated, approval-lined write-capable path.
    Approval,
    /// SSH, dev-tunnel, reverse-tunnel, or provider-tunnel routed action.
    Tunnel,
    /// Action whose effects land on a remote workspace or helper.
    RemoteTarget,
    /// Browser-companion or provider-callback handoff to an external provider.
    ProviderHandoff,
    /// Replay-safe rerun, recipe, or macro automation path.
    ReplaySafeCommand,
}

impl FlowClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeepLink => "deep_link",
            Self::Approval => "approval",
            Self::Tunnel => "tunnel",
            Self::RemoteTarget => "remote_target",
            Self::ProviderHandoff => "provider_handoff",
            Self::ReplaySafeCommand => "replay_safe_command",
        }
    }

    /// Flow classes the inspector must cover to claim stable publication.
    pub const fn required_coverage() -> [Self; 6] {
        [
            Self::DeepLink,
            Self::Approval,
            Self::Tunnel,
            Self::RemoteTarget,
            Self::ProviderHandoff,
            Self::ReplaySafeCommand,
        ]
    }
}

/// Drift class that forces visible reapproval rather than silent replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftClass {
    /// The route or provider changed since the last grant.
    RouteDrift,
    /// The target identity changed since the last grant.
    TargetDrift,
    /// The policy epoch advanced since the last grant.
    PolicyDrift,
    /// The host or workspace boundary changed since the last grant.
    HostDrift,
    /// The approval expiry passed.
    ApprovalExpiry,
}

impl DriftClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteDrift => "route_drift",
            Self::TargetDrift => "target_drift",
            Self::PolicyDrift => "policy_drift",
            Self::HostDrift => "host_drift",
            Self::ApprovalExpiry => "approval_expiry",
        }
    }

    /// Drift classes a stable publication packet must enumerate.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::RouteDrift,
            Self::TargetDrift,
            Self::PolicyDrift,
            Self::HostDrift,
            Self::ApprovalExpiry,
        ]
    }
}

/// One flow-family binding that records how the capability-route inspector is
/// published for a specific flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowPublicationRecord {
    /// Flow family this record covers.
    pub flow_class: FlowClass,
    /// True when the inspector is reachable from this flow.
    pub inspector_reachable: bool,
    /// True when the inspector discloses route class on this flow.
    pub discloses_route_class: bool,
    /// True when the inspector discloses target identity on this flow.
    pub discloses_target_identity: bool,
    /// True when the inspector discloses capability boundary on this flow.
    pub discloses_capability_boundary: bool,
    /// True when the inspector discloses approval scope and expiry on this flow.
    pub discloses_approval_scope_and_expiry: bool,
    /// True when the inspector discloses revalidation triggers on this flow.
    pub discloses_revalidation_triggers: bool,
    /// True when lineage is preserved end to end for this flow.
    pub lineage_preserved: bool,
    /// True when drift forces visible reapproval on this flow.
    pub drift_forces_reapproval: bool,
    /// True when this flow never widens authority beyond the inspector disclosure.
    pub no_authority_widening: bool,
    /// True when browser/provider handoffs on this flow are typed and reversible.
    pub reversible_when_external: bool,
    /// True when the flow enforces the same policy checks as all others.
    pub policy_checked: bool,
}

impl FlowPublicationRecord {
    fn preserves_full_publication(&self) -> bool {
        self.inspector_reachable
            && self.discloses_route_class
            && self.discloses_target_identity
            && self.discloses_capability_boundary
            && self.discloses_approval_scope_and_expiry
            && self.discloses_revalidation_triggers
            && self.lineage_preserved
            && self.drift_forces_reapproval
            && self.no_authority_widening
            && self.reversible_when_external
            && self.policy_checked
    }
}

/// The drift-and-reapproval policy enforced across all published flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReapprovalPolicyRecord {
    /// True when reapproval is mandatory on drift for claimed-stable rows.
    pub required: bool,
    /// Drift classes that force visible reapproval.
    pub drift_classes: Vec<DriftClass>,
    /// True when route drift forces reapproval.
    pub route_drift_forces_reapproval: bool,
    /// True when target drift forces reapproval.
    pub target_drift_forces_reapproval: bool,
    /// True when policy drift forces reapproval.
    pub policy_drift_forces_reapproval: bool,
    /// True when host drift forces reapproval.
    pub host_drift_forces_reapproval: bool,
    /// True when approval expiry forces reapproval.
    pub approval_expiry_forces_reapproval: bool,
    /// True when silent replay is blocked on any drift.
    pub no_silent_replay_on_drift: bool,
    /// True when replay review is surfaced before replay on drift.
    pub replay_review_surfaced: bool,
}

impl ReapprovalPolicyRecord {
    fn covers_all_drifts(&self) -> bool {
        self.route_drift_forces_reapproval
            && self.target_drift_forces_reapproval
            && self.policy_drift_forces_reapproval
            && self.host_drift_forces_reapproval
            && self.approval_expiry_forces_reapproval
            && self.no_silent_replay_on_drift
            && self.replay_review_surfaced
    }
}

/// The lineage-preservation contract that keeps one inspector object from
/// preview through execution, audit, support export, and shiproom proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineagePreservationRecord {
    /// True when one inspector object survives the full lifecycle.
    pub single_object_through_lifecycle: bool,
    /// True when the inspector is preserved in the preview record.
    pub preserved_in_preview: bool,
    /// True when the inspector is preserved in the execution record.
    pub preserved_in_execution: bool,
    /// True when the inspector is preserved in the audit record.
    pub preserved_in_audit: bool,
    /// True when the inspector is preserved in the support export.
    pub preserved_in_support_export: bool,
    /// True when the inspector is preserved in the shiproom proof packet.
    pub preserved_in_shiproom_proof: bool,
    /// True when private-team reconstruction is never required.
    pub no_private_team_reconstruction: bool,
    /// Stable lineage ref that binds all lifecycle stages.
    pub lineage_ref: String,
}

impl LineagePreservationRecord {
    fn guards_hold(&self) -> bool {
        self.single_object_through_lifecycle
            && self.preserved_in_preview
            && self.preserved_in_execution
            && self.preserved_in_audit
            && self.preserved_in_support_export
            && self.preserved_in_shiproom_proof
            && self.no_private_team_reconstruction
            && !self.lineage_ref.trim().is_empty()
    }
}

/// Keyboard-reachability record for the inspector on review sheets, command
/// previews, and diagnostic/support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardReachabilityRecord {
    /// True when the inspector is keyboard-reachable on claimed-stable rows.
    pub keyboard_reachable: bool,
    /// True when the inspector is reachable from review sheets.
    pub reachable_from_review_sheet: bool,
    /// True when the inspector is reachable from command previews.
    pub reachable_from_command_preview: bool,
    /// True when the inspector is reachable from diagnostic surfaces.
    pub reachable_from_diagnostic_surface: bool,
    /// True when the inspector is reachable from support surfaces.
    pub reachable_from_support_surface: bool,
    /// Stable keyboard-shortcut ref (never raw key sequence carrying user data).
    pub keyboard_shortcut_ref: String,
}

impl KeyboardReachabilityRecord {
    fn guards_hold(&self) -> bool {
        self.keyboard_reachable
            && self.reachable_from_review_sheet
            && self.reachable_from_command_preview
            && self.reachable_from_diagnostic_surface
            && self.reachable_from_support_surface
            && !self.keyboard_shortcut_ref.trim().is_empty()
    }
}

/// One cross-surface publication parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSurfaceRow {
    /// Command invocation surface this row covers.
    pub surface_class: CommandSurfaceClass,
    /// True when the inspector is published on this surface.
    pub inspector_published: bool,
    /// True when route class is disclosed on this surface.
    pub discloses_route_class: bool,
    /// True when target identity is disclosed on this surface.
    pub discloses_target_identity: bool,
    /// True when capability boundary is disclosed on this surface.
    pub discloses_capability_boundary: bool,
    /// True when approval scope and expiry are disclosed on this surface.
    pub discloses_approval_scope_and_expiry: bool,
    /// True when revalidation triggers are disclosed on this surface.
    pub discloses_revalidation_triggers: bool,
    /// True when this surface enforces the same policy checks as all others.
    pub policy_checked: bool,
    /// True when this surface never widens the capability boundary.
    pub no_capability_widening: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl PublicationSurfaceRow {
    fn preserves_full_publication(&self) -> bool {
        self.inspector_published
            && self.discloses_route_class
            && self.discloses_target_identity
            && self.discloses_capability_boundary
            && self.discloses_approval_scope_and_expiry
            && self.discloses_revalidation_triggers
            && self.policy_checked
            && self.no_capability_widening
    }
}

/// Constructor input for [`CapabilityRouteInspectorPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRouteInspectorPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Command or action family id this packet covers.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The capability-route inspector published by this packet.
    pub capability_route_inspector: CapabilityRouteInspector,
    /// Flow-family publication records.
    pub flow_records: Vec<FlowPublicationRecord>,
    /// The drift-and-reapproval policy.
    pub reapproval_policy: ReapprovalPolicyRecord,
    /// The lineage-preservation contract.
    pub lineage_preservation: LineagePreservationRecord,
    /// The keyboard-reachability record.
    pub keyboard_reachability: KeyboardReachabilityRecord,
    /// Cross-surface publication parity rows.
    pub surface_rows: Vec<PublicationSurfaceRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe capability-route inspector publication record.
///
/// Binds the capability-route inspector to deep-link, approval, tunnel,
/// remote-target, provider-handoff, and replay-safe command flows, and
/// enforces lineage preservation, drift reapproval, reversibility, and
/// keyboard reachability on every claimed stable row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRouteInspectorPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Command or action family id this packet covers.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The capability-route inspector published by this packet.
    pub capability_route_inspector: CapabilityRouteInspector,
    /// Flow-family publication records.
    pub flow_records: Vec<FlowPublicationRecord>,
    /// The drift-and-reapproval policy.
    pub reapproval_policy: ReapprovalPolicyRecord,
    /// The lineage-preservation contract.
    pub lineage_preservation: LineagePreservationRecord,
    /// The keyboard-reachability record.
    pub keyboard_reachability: KeyboardReachabilityRecord,
    /// Cross-surface publication parity rows.
    pub surface_rows: Vec<PublicationSurfaceRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CapabilityRouteInspectorPacket {
    /// Builds a capability-route inspector publication packet from canonical rows.
    pub fn new(input: CapabilityRouteInspectorPacketInput) -> Self {
        Self {
            record_kind: CAPABILITY_ROUTE_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: CAPABILITY_ROUTE_INSPECTOR_SCHEMA_VERSION,
            packet_id: input.packet_id,
            command_family_id: input.command_family_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            capability_route_inspector: input.capability_route_inspector,
            flow_records: input.flow_records,
            reapproval_policy: input.reapproval_policy,
            lineage_preservation: input.lineage_preservation,
            keyboard_reachability: input.keyboard_reachability,
            surface_rows: input.surface_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's stable-line invariants.
    ///
    /// Returns every [`CapabilityRouteInspectorViolation`] found; an empty vec
    /// means the packet is conformant.
    pub fn validate(&self) -> Vec<CapabilityRouteInspectorViolation> {
        let mut violations = Vec::new();
        if self.record_kind != CAPABILITY_ROUTE_INSPECTOR_RECORD_KIND {
            violations.push(CapabilityRouteInspectorViolation::WrongRecordKind);
        }
        if self.schema_version != CAPABILITY_ROUTE_INSPECTOR_SCHEMA_VERSION {
            violations.push(CapabilityRouteInspectorViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.command_family_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CapabilityRouteInspectorViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_capability_route_inspector(self, &mut violations);
        validate_flow_records(self, &mut violations);
        validate_reapproval_policy(self, &mut violations);
        validate_lineage_preservation(self, &mut violations);
        validate_keyboard_reachability(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("capability-route inspector packet serializes"),
        ) {
            violations.push(CapabilityRouteInspectorViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("capability-route inspector packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .surface_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# Capability-Route Inspector Publication\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Command family: `{}`\n", self.command_family_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Inspector id: `{}`\n",
            self.capability_route_inspector.inspector_id
        ));
        out.push_str(&format!("- Flow records: {}\n", self.flow_records.len()));
        out.push_str(&format!(
            "- Drift classes enforced: {}\n",
            self.reapproval_policy.drift_classes.len()
        ));
        out.push_str(&format!(
            "- Lineage preserved: {}\n",
            self.lineage_preservation.single_object_through_lifecycle
        ));
        out.push_str(&format!(
            "- Keyboard reachable: {}\n",
            self.keyboard_reachability.keyboard_reachable
        ));
        out.push_str(&format!(
            "- Inspector surfaces: {} ({} narrowed below Stable)\n",
            self.surface_rows.len(),
            narrowed_surfaces
        ));
        out
    }
}

/// Errors emitted when reading the checked-in support export.
#[derive(Debug)]
pub enum CapabilityRouteInspectorArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CapabilityRouteInspectorViolation>),
}

impl fmt::Display for CapabilityRouteInspectorArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "capability-route inspector export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "capability-route inspector export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CapabilityRouteInspectorArtifactError {}

/// Validation failures emitted by [`CapabilityRouteInspectorPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityRouteInspectorViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The bound canonical contract refs drifted from the single registry/schema.
    ContractRefsNotCanonical,
    /// The capability-route inspector is missing required fields or guards.
    InspectorGuardsBroken,
    /// Cross-flow coverage is incomplete (missing a required flow class).
    FlowCoverageMissing,
    /// A stable flow dropped required disclosure, lineage, or reversibility.
    FlowPublicationParityBroken,
    /// A flow narrowed below Stable still claims the Stable lane.
    UnqualifiedFlowClaimsStable,
    /// The drift-and-reapproval policy is not required on a stable row.
    ReapprovalPolicyNotRequired,
    /// Drift class coverage is incomplete.
    DriftCoverageMissing,
    /// A drift reapproval cue is missing.
    ReapprovalPolicyCueMissing,
    /// The lineage-preservation contract is missing required fields.
    LineagePreservationGuardsBroken,
    /// The keyboard-reachability record is missing required fields.
    KeyboardReachabilityGuardsBroken,
    /// Cross-surface coverage is incomplete (missing a required surface class).
    InspectorSurfaceCoverageMissing,
    /// A stable, reachable surface dropped required disclosure or policy parity.
    InspectorSurfaceParityBroken,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl CapabilityRouteInspectorViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::InspectorGuardsBroken => "inspector_guards_broken",
            Self::FlowCoverageMissing => "flow_coverage_missing",
            Self::FlowPublicationParityBroken => "flow_publication_parity_broken",
            Self::UnqualifiedFlowClaimsStable => "unqualified_flow_claims_stable",
            Self::ReapprovalPolicyNotRequired => "reapproval_policy_not_required",
            Self::DriftCoverageMissing => "drift_coverage_missing",
            Self::ReapprovalPolicyCueMissing => "reapproval_policy_cue_missing",
            Self::LineagePreservationGuardsBroken => "lineage_preservation_guards_broken",
            Self::KeyboardReachabilityGuardsBroken => "keyboard_reachability_guards_broken",
            Self::InspectorSurfaceCoverageMissing => "inspector_surface_coverage_missing",
            Self::InspectorSurfaceParityBroken => "inspector_surface_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in capability-route inspector support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_capability_route_inspector_export(
) -> Result<CapabilityRouteInspectorPacket, CapabilityRouteInspectorArtifactError> {
    let packet: CapabilityRouteInspectorPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/m4/publish_capability_route_inspector/support_export.json"
    )))
    .map_err(CapabilityRouteInspectorArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CapabilityRouteInspectorArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    for required in [
        CAPABILITY_ROUTE_INSPECTOR_DOC_REF,
        CAPABILITY_ROUTE_INSPECTOR_SCHEMA_REF,
        CAPABILITY_ROUTE_INSPECTOR_DESCRIPTOR_CONTRACT_REF,
        CAPABILITY_ROUTE_INSPECTOR_PARITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(CapabilityRouteInspectorViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    if packet.contract_refs != StableContractRefs::canonical() {
        violations.push(CapabilityRouteInspectorViolation::ContractRefsNotCanonical);
    }
}

fn validate_capability_route_inspector(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    let inspector = &packet.capability_route_inspector;
    if !inspector.guards_hold() {
        violations.push(CapabilityRouteInspectorViolation::InspectorGuardsBroken);
    }
    if !inspector.covers_all_revalidation_triggers() {
        violations.push(CapabilityRouteInspectorViolation::InspectorGuardsBroken);
    }
}

fn validate_flow_records(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    for required in FlowClass::required_coverage() {
        if !packet
            .flow_records
            .iter()
            .any(|record| record.flow_class == required)
        {
            violations.push(CapabilityRouteInspectorViolation::FlowCoverageMissing);
            break;
        }
    }
    for record in &packet.flow_records {
        // A flow narrowed below Stable may not claim the Stable lane.
        if record.flow_class == FlowClass::ProviderHandoff
            || record.flow_class == FlowClass::Tunnel
            || record.flow_class == FlowClass::RemoteTarget
        {
            // These flows are inherently external; if claimed stable they must
            // preserve full publication.
        }
        // A stable flow must preserve full disclosure, lineage, and reversibility.
        if packet.claimed_stable
            && record.inspector_reachable
            && !record.preserves_full_publication()
        {
            violations.push(CapabilityRouteInspectorViolation::FlowPublicationParityBroken);
            break;
        }
    }
}

fn validate_reapproval_policy(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    let policy = &packet.reapproval_policy;
    if packet.claimed_stable && !policy.required {
        violations.push(CapabilityRouteInspectorViolation::ReapprovalPolicyNotRequired);
    }
    for required in DriftClass::required_coverage() {
        if !policy.drift_classes.iter().any(|item| *item == required) {
            violations.push(CapabilityRouteInspectorViolation::DriftCoverageMissing);
            break;
        }
    }
    if policy.required && !policy.covers_all_drifts() {
        violations.push(CapabilityRouteInspectorViolation::ReapprovalPolicyCueMissing);
    }
}

fn validate_lineage_preservation(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    if !packet.lineage_preservation.guards_hold() {
        violations.push(CapabilityRouteInspectorViolation::LineagePreservationGuardsBroken);
    }
}

fn validate_keyboard_reachability(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    if packet.claimed_stable && !packet.keyboard_reachability.guards_hold() {
        violations.push(CapabilityRouteInspectorViolation::KeyboardReachabilityGuardsBroken);
    }
}

fn validate_surface_rows(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(CapabilityRouteInspectorViolation::InspectorSurfaceCoverageMissing);
            break;
        }
    }
    for row in &packet.surface_rows {
        // A surface narrowed below Stable may not claim the Stable lane.
        if row.claimed_stable && !row.qualification.is_stable() {
            violations.push(CapabilityRouteInspectorViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
        // A stable, reachable surface must preserve full disclosure and policy parity.
        if row.qualification.is_stable()
            && row.inspector_published
            && !row.preserves_full_publication()
        {
            violations.push(CapabilityRouteInspectorViolation::InspectorSurfaceParityBroken);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &CapabilityRouteInspectorPacket,
    violations: &mut Vec<CapabilityRouteInspectorViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(CapabilityRouteInspectorViolation::EvidenceExportRefsMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_diff")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
