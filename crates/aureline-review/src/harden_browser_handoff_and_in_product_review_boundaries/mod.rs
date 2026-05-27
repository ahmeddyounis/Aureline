//! Hardened browser handoff and in-product review boundaries with provider/source
//! identity and return paths.
//!
//! This module owns the bounded contract that keeps browser handoff and
//! provider-linked review boundaries explicit about source, freshness, actor,
//! target, and return path. The stable line never hides hosted authority behind
//! local chrome, and every mutation crossing the in-product/browser boundary
//! remains previewable, attributable, and reversible.
//!
//! The record family includes:
//!
//! - [`ReviewBoundaryHardeningRecord`] — stable identity binding workspace,
//!   stabilization, and boundary-hardening state.
//! - [`BrowserHandoffBoundaryRecord`] — explicit browser-handoff boundary with
//!   typed origin, destination, object identity, and reversibility.
//! - [`InProductReviewBoundaryRecord`] — in-product review boundary with
//!   authority truth, provider vs local ownership, and exact freshness.
//! - [`ProviderSourceIdentityRecord`] — provider and source identity disclosure
//!   so the boundary never collapses into generic "open in browser".
//! - [`ReturnPathRecord`] — typed reversible return path with anchor kind,
//!   anchor ref, and replay posture.
//! - [`BoundaryFreshnessObservationRecord`] — freshness observation at the
//!   boundary, explicit about provider overlay staleness.
//! - [`BoundaryOwnershipSignalRecord`] — ownership signal at the boundary,
//!   keeping advisory and enforceable split.
//! - [`BoundaryHardeningCommandRecord`] — command-graph operations surfaced to
//!   the inspector (preview, approve, refresh, invalidate, handoff, return).
//! - [`BoundaryHardeningSupportExportPacket`] — redaction-safe support export.
//! - [`BoundaryHardeningInspectionRecord`] — compact boolean projection for CLI
//!   and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/harden_browser_handoff_and_in_product_review_boundaries.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/harden-browser-handoff-and-in-product-review-boundaries/`.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationPacket;
use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every review-boundary-hardening record.
pub const REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ReviewBoundaryHardeningPacket`].
pub const REVIEW_BOUNDARY_HARDENING_PACKET_RECORD_KIND: &str =
    "review_boundary_hardening_packet";

/// Stable record-kind tag for [`ReviewBoundaryHardeningRecord`].
pub const REVIEW_BOUNDARY_HARDENING_RECORD_KIND: &str = "review_boundary_hardening_record";

/// Stable record-kind tag for [`BrowserHandoffBoundaryRecord`].
pub const BROWSER_HANDOFF_BOUNDARY_RECORD_KIND: &str = "review_browser_handoff_boundary_record";

/// Stable record-kind tag for [`InProductReviewBoundaryRecord`].
pub const IN_PRODUCT_REVIEW_BOUNDARY_RECORD_KIND: &str =
    "review_in_product_review_boundary_record";

/// Stable record-kind tag for [`ProviderSourceIdentityRecord`].
pub const PROVIDER_SOURCE_IDENTITY_RECORD_KIND: &str = "review_provider_source_identity_record";

/// Stable record-kind tag for [`ReturnPathRecord`].
pub const RETURN_PATH_RECORD_KIND: &str = "review_return_path_record";

/// Stable record-kind tag for [`BoundaryFreshnessObservationRecord`].
pub const BOUNDARY_FRESHNESS_OBSERVATION_RECORD_KIND: &str =
    "review_boundary_freshness_observation_record";

/// Stable record-kind tag for [`BoundaryOwnershipSignalRecord`].
pub const BOUNDARY_OWNERSHIP_SIGNAL_RECORD_KIND: &str =
    "review_boundary_ownership_signal_record";

/// Stable record-kind tag for [`BoundaryHardeningCommandRecord`].
pub const BOUNDARY_HARDENING_COMMAND_RECORD_KIND: &str =
    "review_boundary_hardening_command_record";

/// Stable record-kind tag for [`BoundaryHardeningSupportExportPacket`].
pub const BOUNDARY_HARDENING_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_boundary_hardening_support_export_packet";

/// Stable record-kind tag for [`BoundaryHardeningInspectionRecord`].
pub const BOUNDARY_HARDENING_INSPECTION_RECORD_KIND: &str =
    "review_boundary_hardening_inspection_record";

/// Closed set of boundary-hardening states.
pub const BOUNDARY_HARDENING_STATES: &[&str] = &[
    "boundary_hardened",
    "boundary_degraded_provider_overlay_stale",
    "boundary_degraded_missing_return_path",
    "boundary_degraded_hidden_authority",
    "boundary_degraded_freshness_unknown",
    "boundary_degraded_ownership_ambiguous",
];

/// Closed set of boundary authority classes.
pub const BOUNDARY_AUTHORITY_CLASSES: &[&str] = &[
    "local_authoritative",
    "provider_authoritative",
    "local_and_provider_agree",
    "local_and_provider_disagree",
    "provider_overlay_missing",
];

/// Closed set of handoff boundary classes.
pub const HANDOFF_BOUNDARY_CLASSES: &[&str] = &[
    "handoff_reversible_typed",
    "handoff_typed_no_return_path",
    "handoff_untyped",
    "handoff_not_required",
    "handoff_authority_revoked",
];

/// Closed set of return path classes.
pub const RETURN_PATH_CLASSES: &[&str] = &[
    "return_to_review_anchor",
    "return_to_workspace",
    "return_to_change_lineage",
    "return_to_landing_strip",
    "return_path_missing",
    "return_path_expired",
];

/// Closed set of boundary freshness classes.
pub const BOUNDARY_FRESHNESS_CLASSES: &[&str] = &[
    "boundary_fresh",
    "boundary_stale_within_grace",
    "boundary_stale_blocks_mutation",
    "boundary_freshness_unknown",
];

/// Closed set of boundary ownership classes.
pub const BOUNDARY_OWNERSHIP_CLASSES: &[&str] = &[
    "ownership_advisory_at_boundary",
    "ownership_enforceable_at_boundary",
    "ownership_conflict_at_boundary",
    "ownership_not_disclosed",
];

/// Closed set of command classes for the boundary-hardening lane.
pub const BOUNDARY_HARDENING_COMMAND_CLASSES: &[&str] = &[
    "preview_boundary",
    "approve_boundary",
    "refresh_provider_identity",
    "refresh_return_path",
    "invalidate_boundary",
    "request_browser_handoff",
    "return_to_product_scope",
    "audit_boundary_state",
];

/// Closed set of consumer surfaces for boundary-hardening packets.
pub const BOUNDARY_HARDENING_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "review_landing_strip",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
];

/// Closed set of invalidation reasons that mark a boundary stale.
pub const BOUNDARY_HARDENING_INVALIDATION_REASONS: &[&str] = &[
    "provider_overlay_stale",
    "missing_return_path",
    "hidden_authority_detected",
    "freshness_unknown",
    "ownership_ambiguous",
    "browser_handoff_untyped",
    "approval_invalidated",
    "anchor_drifted",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a review boundary hardening to materialize on top of a beta
/// review-workspace packet and a review-stabilization packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBoundaryHardeningInput {
    /// Stable boundary-hardening identity.
    pub boundary_hardening_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Boundary-hardening state from the closed vocabulary.
    pub boundary_hardening_state: String,
    /// Browser-handoff boundary input.
    pub browser_handoff_boundary: BrowserHandoffBoundaryInput,
    /// In-product review boundary input.
    pub in_product_boundary: InProductReviewBoundaryInput,
    /// Provider/source identity input.
    pub provider_source_identity: ProviderSourceIdentityInput,
    /// Return path input.
    pub return_path: ReturnPathInput,
    /// Boundary freshness observation input.
    pub boundary_freshness: BoundaryFreshnessObservationInput,
    /// Boundary ownership signal inputs.
    pub boundary_ownership_signals: Vec<BoundaryOwnershipSignalInput>,
    /// Command-graph operations defined for this boundary hardening.
    pub commands: Vec<BoundaryHardeningCommandInput>,
    /// Support/export envelope input.
    pub support_export: BoundaryHardeningSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing the browser-handoff boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffBoundaryInput {
    /// Handoff boundary class from the closed vocabulary.
    pub handoff_boundary_class: String,
    /// Opaque handoff id ref, when a handoff is present.
    pub handoff_id_ref: String,
    /// Typed destination class, never a raw URL.
    pub destination_class: String,
    /// Opaque destination token resolved by the launcher.
    pub destination_ref: String,
    /// Provider-side object identity ref.
    pub object_identity_ref: String,
    /// Reason code explaining why browser handoff is used.
    pub reason_code: String,
    /// True when the handoff has a typed return path.
    pub return_path_present: bool,
    /// True when the handoff is reversible.
    pub reversible: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing the in-product review boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InProductReviewBoundaryInput {
    /// Boundary authority class from the closed vocabulary.
    pub boundary_authority_class: String,
    /// True when local review pack is authoritative at the boundary.
    pub local_authoritative: bool,
    /// True when provider overlay is authoritative at the boundary.
    pub provider_authoritative: bool,
    /// True when local and provider agree on the boundary truth.
    pub local_provider_agree: bool,
    /// True when a hidden authority (provider mutation behind local chrome) is
    /// detected.
    pub hidden_authority_detected: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing provider and source identity at the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSourceIdentityInput {
    /// Provider class token.
    pub provider_class: String,
    /// Opaque connected-provider record id ref.
    pub connected_provider_record_id_ref: String,
    /// Opaque provider object identity ref.
    pub provider_object_identity_ref: String,
    /// Source class naming which lane minted the boundary.
    pub source_class: String,
    /// Actor ref that initiated the boundary crossing.
    pub actor_ref: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing the return path for a reversible boundary crossing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnPathInput {
    /// Return path class from the closed vocabulary.
    pub return_path_class: String,
    /// Return anchor kind used to reopen the source review context.
    pub return_anchor_kind: String,
    /// Return anchor ref used for reversible handoff.
    pub return_anchor_ref: String,
    /// Replay posture from the browser-handoff contract.
    pub replay_posture: String,
    /// True when the return path is expired.
    pub expired: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing freshness at the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFreshnessObservationInput {
    /// Boundary freshness class from the closed vocabulary.
    pub boundary_freshness_class: String,
    /// Provider overlay freshness class at boundary observation time.
    pub provider_overlay_freshness_class: String,
    /// Timestamp of the last provider fetch that produced this observation.
    pub last_fetched_at: String,
    /// True when stale or unavailable evidence blocks mutation authority.
    pub blocks_mutation_when_stale: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing ownership at the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryOwnershipSignalInput {
    /// Boundary ownership class from the closed vocabulary.
    pub boundary_ownership_class: String,
    /// Scope path or pattern the ownership applies to.
    pub scope_pattern: String,
    /// Owner ref (team, individual, or policy identifier).
    pub owner_ref: String,
    /// True when this signal is enforceable at the boundary.
    pub enforceable: bool,
    /// True when this signal is advisory at the boundary.
    pub advisory: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one command-graph operation for a boundary hardening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryHardeningCommandInput {
    /// Stable command identity.
    pub command_id: String,
    /// Command class from the closed vocabulary.
    pub command_class: String,
    /// Target object ref the command would mutate.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when the command supports preview/dry-run.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Active blocked reasons preventing execution; empty when actionable.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row for the boundary-hardening support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryHardeningSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the boundary hardening.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}


// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Review-boundary-hardening record materialized from input plus workspace and
/// stabilization truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBoundaryHardeningRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable boundary-hardening identity.
    pub boundary_hardening_id: String,
    /// Review workspace this boundary hardening belongs to.
    pub review_workspace_id_ref: String,
    /// Stabilization this boundary hardening binds.
    pub stabilization_id_ref: String,
    /// Boundary-hardening state.
    pub boundary_hardening_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing boundary approval.
    pub blocked_reasons: Vec<String>,
    /// True when the boundary hardening is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the boundary hardening was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Browser-handoff boundary record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffBoundaryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening this handoff boundary belongs to.
    pub boundary_hardening_id_ref: String,
    /// Handoff boundary class.
    pub handoff_boundary_class: String,
    /// Opaque handoff id ref.
    pub handoff_id_ref: String,
    /// Typed destination class.
    pub destination_class: String,
    /// Opaque destination token resolved by the launcher.
    pub destination_ref: String,
    /// Provider-side object identity ref.
    pub object_identity_ref: String,
    /// Reason code.
    pub reason_code: String,
    /// True when the handoff has a typed return path.
    pub return_path_present: bool,
    /// True when the handoff is reversible.
    pub reversible: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// In-product review boundary record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InProductReviewBoundaryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening this in-product boundary belongs to.
    pub boundary_hardening_id_ref: String,
    /// Boundary authority class.
    pub boundary_authority_class: String,
    /// True when local review pack is authoritative.
    pub local_authoritative: bool,
    /// True when provider overlay is authoritative.
    pub provider_authoritative: bool,
    /// True when local and provider agree.
    pub local_provider_agree: bool,
    /// True when hidden authority is detected.
    pub hidden_authority_detected: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Provider/source identity record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSourceIdentityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening this identity belongs to.
    pub boundary_hardening_id_ref: String,
    /// Provider class token.
    pub provider_class: String,
    /// Opaque connected-provider record id ref.
    pub connected_provider_record_id_ref: String,
    /// Opaque provider object identity ref.
    pub provider_object_identity_ref: String,
    /// Source class naming which lane minted the boundary.
    pub source_class: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Return path record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnPathRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening this return path belongs to.
    pub boundary_hardening_id_ref: String,
    /// Return path class.
    pub return_path_class: String,
    /// Return anchor kind.
    pub return_anchor_kind: String,
    /// Return anchor ref.
    pub return_anchor_ref: String,
    /// Replay posture.
    pub replay_posture: String,
    /// True when the return path is expired.
    pub expired: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Boundary freshness observation record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFreshnessObservationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening this observation belongs to.
    pub boundary_hardening_id_ref: String,
    /// Boundary freshness class.
    pub boundary_freshness_class: String,
    /// Provider overlay freshness class.
    pub provider_overlay_freshness_class: String,
    /// Timestamp of the last provider fetch.
    pub last_fetched_at: String,
    /// True when stale evidence blocks mutation authority.
    pub blocks_mutation_when_stale: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Boundary ownership signal record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryOwnershipSignalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening this ownership signal belongs to.
    pub boundary_hardening_id_ref: String,
    /// Boundary ownership class.
    pub boundary_ownership_class: String,
    /// Scope path or pattern.
    pub scope_pattern: String,
    /// Owner ref.
    pub owner_ref: String,
    /// True when enforceable.
    pub enforceable: bool,
    /// True when advisory.
    pub advisory: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Command-graph operation record for a boundary hardening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryHardeningCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Boundary hardening this command belongs to.
    pub boundary_hardening_id_ref: String,
    /// Command class.
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when preview/dry-run is supported.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Active blocked reasons preventing execution.
    pub blocked_reasons: Vec<String>,
    /// True when the command is actionable from the current boundary state.
    pub actionable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the boundary-hardening lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryHardeningSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Boundary hardening this packet exports.
    pub boundary_hardening_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stabilization this packet exports.
    pub stabilization_id_ref: String,
    /// Stable context ref used to reopen the boundary hardening.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Command ids exported in this packet.
    pub command_id_refs: Vec<String>,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the export cites.
    pub source_schema_refs: Vec<String>,
    /// False so raw URLs cannot cross the support boundary.
    pub raw_url_export_allowed: bool,
    /// False so raw provider payloads cannot cross the support boundary.
    pub raw_provider_payload_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryHardeningInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Boundary hardening inspected by this row.
    pub boundary_hardening_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the boundary is hardened.
    pub boundary_hardened: bool,
    /// True when the boundary is degraded due to stale provider overlay.
    pub boundary_degraded_provider_overlay_stale: bool,
    /// True when the boundary is degraded due to missing return path.
    pub boundary_degraded_missing_return_path: bool,
    /// True when the boundary is degraded due to hidden authority.
    pub boundary_degraded_hidden_authority: bool,
    /// True when the boundary is degraded due to unknown freshness.
    pub boundary_degraded_freshness_unknown: bool,
    /// True when the boundary is degraded due to ambiguous ownership.
    pub boundary_degraded_ownership_ambiguous: bool,
    /// True when the browser handoff is reversible and typed.
    pub handoff_reversible_typed: bool,
    /// True when the in-product boundary is locally authoritative.
    pub in_product_local_authoritative: bool,
    /// True when the in-product boundary is provider authoritative.
    pub in_product_provider_authoritative: bool,
    /// True when local and provider agree on boundary truth.
    pub local_provider_agree: bool,
    /// True when hidden authority is detected.
    pub hidden_authority_detected: bool,
    /// True when the return path is present and not expired.
    pub return_path_present_and_valid: bool,
    /// True when the boundary freshness is current or within grace.
    pub boundary_fresh_or_within_grace: bool,
    /// True when the boundary freshness blocks mutation.
    pub boundary_freshness_blocks_mutation: bool,
    /// True when at least one ownership signal is enforceable at the boundary.
    pub enforceable_ownership_at_boundary: bool,
    /// True when at least one ownership signal is advisory at the boundary.
    pub advisory_ownership_at_boundary: bool,
    /// True when ownership signals conflict at the boundary.
    pub ownership_conflict_at_boundary: bool,
    /// True when the boundary hardening is actionable.
    pub actionable: bool,
    /// True when the boundary hardening is invalidated by any reason.
    pub invalidated: bool,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// Number of boundary ownership signal records.
    pub boundary_ownership_signal_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the boundary hardening context.
    pub support_export_reopenable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Review-boundary-hardening packet consumed by review surfaces and support
/// exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBoundaryHardeningPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the packet.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review workspace summary copied from the beta packet.
    pub review_workspace: crate::workspace::ReviewWorkspaceRecord,
    /// Stabilization summary copied from the stabilization packet.
    pub stabilization: crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationRecord,
    /// Review-boundary-hardening record.
    pub boundary_hardening: ReviewBoundaryHardeningRecord,
    /// Browser-handoff boundary record.
    pub browser_handoff_boundary: BrowserHandoffBoundaryRecord,
    /// In-product review boundary record.
    pub in_product_boundary: InProductReviewBoundaryRecord,
    /// Provider/source identity record.
    pub provider_source_identity: ProviderSourceIdentityRecord,
    /// Return path record.
    pub return_path: ReturnPathRecord,
    /// Boundary freshness observation record.
    pub boundary_freshness: BoundaryFreshnessObservationRecord,
    /// Boundary ownership signal records.
    pub boundary_ownership_signals: Vec<BoundaryOwnershipSignalRecord>,
    /// Command-graph operation records.
    pub commands: Vec<BoundaryHardeningCommandRecord>,
    /// Support/export packet.
    pub support_export: BoundaryHardeningSupportExportPacket,
    /// Inspection row.
    pub inspection: BoundaryHardeningInspectionRecord,
}

impl ReviewBoundaryHardeningPacket {
    /// Builds a review-boundary-hardening packet from a beta review-workspace
    /// packet and a review-stabilization packet.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewBoundaryHardeningValidationError`] when the input
    /// violates a boundary-hardening invariant.
    pub fn from_workspace_and_stabilization_packets(
        input: ReviewBoundaryHardeningInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
        stabilization_packet: &ReviewStabilizationPacket,
    ) -> Result<Self, ReviewBoundaryHardeningValidationError> {
        validate_input(&input, workspace_packet, stabilization_packet)?;

        let boundary_hardening = boundary_hardening_record(
            &input,
            workspace_packet,
            stabilization_packet,
        );
        let browser_handoff_boundary =
            browser_handoff_boundary_record(&input.browser_handoff_boundary, &boundary_hardening);
        let in_product_boundary =
            in_product_review_boundary_record(&input.in_product_boundary, &boundary_hardening);
        let provider_source_identity =
            provider_source_identity_record(&input.provider_source_identity, &boundary_hardening);
        let return_path = return_path_record(&input.return_path, &boundary_hardening);
        let boundary_freshness =
            boundary_freshness_observation_record(&input.boundary_freshness, &boundary_hardening);
        let boundary_ownership_signals = input
            .boundary_ownership_signals
            .iter()
            .map(|o| boundary_ownership_signal_record(o, &boundary_hardening))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|command| boundary_hardening_command_record(command, &boundary_hardening))
            .collect::<Vec<_>>();
        let support_export = boundary_hardening_support_export_packet(
            &input.support_export,
            &boundary_hardening,
            workspace_packet,
            stabilization_packet,
            &commands,
        );
        let inspection = boundary_hardening_inspection_record(
            &boundary_hardening,
            &browser_handoff_boundary,
            &in_product_boundary,
            &return_path,
            &boundary_freshness,
            &boundary_ownership_signals,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: REVIEW_BOUNDARY_HARDENING_PACKET_RECORD_KIND.to_string(),
            schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            stabilization: stabilization_packet.stabilization.clone(),
            boundary_hardening,
            browser_handoff_boundary,
            in_product_boundary,
            provider_source_identity,
            return_path,
            boundary_freshness,
            boundary_ownership_signals,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the boundary-hardening invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewBoundaryHardeningValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), ReviewBoundaryHardeningValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            REVIEW_BOUNDARY_HARDENING_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_boundary_hardening_record(
            &self.boundary_hardening,
            &self.review_workspace.review_workspace_id,
            &self.stabilization.stabilization_id,
        )?;
        validate_browser_handoff_boundary_record(
            &self.browser_handoff_boundary,
            &self.boundary_hardening.boundary_hardening_id,
        )?;
        validate_in_product_review_boundary_record(
            &self.in_product_boundary,
            &self.boundary_hardening.boundary_hardening_id,
        )?;
        validate_provider_source_identity_record(
            &self.provider_source_identity,
            &self.boundary_hardening.boundary_hardening_id,
        )?;
        validate_return_path_record(&self.return_path, &self.boundary_hardening.boundary_hardening_id)?;
        validate_boundary_freshness_observation_record(
            &self.boundary_freshness,
            &self.boundary_hardening.boundary_hardening_id,
        )?;
        for signal in &self.boundary_ownership_signals {
            validate_boundary_ownership_signal_record(
                signal,
                &self.boundary_hardening.boundary_hardening_id,
            )?;
        }
        for command in &self.commands {
            validate_command_record(command, &self.boundary_hardening.boundary_hardening_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.boundary_hardening,
            &self.commands,
        )?;
        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when boundary-truth axes are surfaced as separable
    /// inspectable truths.
    pub fn truths_are_separable(&self) -> bool {
        contains_token(
            BOUNDARY_HARDENING_STATES,
            &self.boundary_hardening.boundary_hardening_state,
        ) && contains_token(
            HANDOFF_BOUNDARY_CLASSES,
            &self.browser_handoff_boundary.handoff_boundary_class,
        ) && contains_token(
            BOUNDARY_AUTHORITY_CLASSES,
            &self.in_product_boundary.boundary_authority_class,
        ) && contains_token(
            RETURN_PATH_CLASSES,
            &self.return_path.return_path_class,
        ) && contains_token(
            BOUNDARY_FRESHNESS_CLASSES,
            &self.boundary_freshness.boundary_freshness_class,
        ) && self
            .boundary_ownership_signals
            .iter()
            .all(|s| contains_token(BOUNDARY_OWNERSHIP_CLASSES, &s.boundary_ownership_class))
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }

    /// Returns true when the browser handoff is reversible and typed.
    pub fn handoff_reversible_and_typed(&self) -> bool {
        self.browser_handoff_boundary.handoff_boundary_class == "handoff_reversible_typed"
            && self.browser_handoff_boundary.return_path_present
            && self.browser_handoff_boundary.reversible
    }

    /// Returns true when provider identity is fully disclosed at the boundary.
    pub fn provider_identity_disclosed(&self) -> bool {
        !self.provider_source_identity.provider_class.trim().is_empty()
            && !self.provider_source_identity.provider_object_identity_ref.trim().is_empty()
            && !self.provider_source_identity.actor_ref.trim().is_empty()
    }

    /// Returns true when every ownership signal is at least advisory or
    /// enforceable at the boundary.
    pub fn ownership_signals_properly_split(&self) -> bool {
        self.boundary_ownership_signals
            .iter()
            .all(|s| s.advisory || s.enforceable)
    }
}

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewBoundaryHardeningProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Boundary-hardening identity.
    pub boundary_hardening_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Stabilization identity.
    pub stabilization_id: String,
    /// Boundary-hardening state.
    pub boundary_hardening_state: String,
    /// True when the browser handoff is reversible and typed.
    pub handoff_reversible_typed: bool,
    /// True when the in-product boundary is locally authoritative.
    pub in_product_local_authoritative: bool,
    /// True when local and provider agree on boundary truth.
    pub local_provider_agree: bool,
    /// True when hidden authority is detected.
    pub hidden_authority_detected: bool,
    /// True when the return path is present and valid.
    pub return_path_present_and_valid: bool,
    /// True when boundary freshness blocks mutation.
    pub boundary_freshness_blocks_mutation: bool,
    /// True when ownership signals conflict at the boundary.
    pub ownership_conflict_at_boundary: bool,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Command count.
    pub command_count: usize,
    /// True when support/export can reopen the boundary hardening context.
    pub support_export_reopenable: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
}

/// Parses and validates a materialized review-boundary-hardening packet.
///
/// # Errors
///
/// Returns [`ReviewBoundaryHardeningError`] when the payload fails to parse or
/// violates the boundary-hardening invariants.
pub fn project_review_boundary_hardening_packet(
    payload: &str,
) -> Result<ReviewBoundaryHardeningProjection, ReviewBoundaryHardeningError> {
    let packet: ReviewBoundaryHardeningPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(ReviewBoundaryHardeningProjection::from(packet))
}

impl From<ReviewBoundaryHardeningPacket> for ReviewBoundaryHardeningProjection {
    fn from(packet: ReviewBoundaryHardeningPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            boundary_hardening_id: packet.boundary_hardening.boundary_hardening_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            stabilization_id: packet.stabilization.stabilization_id,
            boundary_hardening_state: packet.boundary_hardening.boundary_hardening_state,
            handoff_reversible_typed: packet.inspection.handoff_reversible_typed,
            in_product_local_authoritative: packet.inspection.in_product_local_authoritative,
            local_provider_agree: packet.inspection.local_provider_agree,
            hidden_authority_detected: packet.inspection.hidden_authority_detected,
            return_path_present_and_valid: packet.inspection.return_path_present_and_valid,
            boundary_freshness_blocks_mutation: packet.inspection.boundary_freshness_blocks_mutation,
            ownership_conflict_at_boundary: packet.inspection.ownership_conflict_at_boundary,
            invalidation_reasons: packet.boundary_hardening.invalidation_reasons.clone(),
            blocked_reasons: packet.boundary_hardening.blocked_reasons.clone(),
            command_count: packet.commands.len(),
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
        }
    }
}

/// Error returned when a review-boundary-hardening payload cannot be projected.
#[derive(Debug)]
pub enum ReviewBoundaryHardeningError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the boundary-hardening invariants.
    Validation(ReviewBoundaryHardeningValidationError),
}

impl fmt::Display for ReviewBoundaryHardeningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "review boundary hardening parse error: {err}"),
            Self::Validation(err) => {
                write!(formatter, "review boundary hardening validation error: {err}")
            }
        }
    }
}

impl std::error::Error for ReviewBoundaryHardeningError {}

impl From<serde_json::Error> for ReviewBoundaryHardeningError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<ReviewBoundaryHardeningValidationError> for ReviewBoundaryHardeningError {
    fn from(err: ReviewBoundaryHardeningValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for review-boundary-hardening packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewBoundaryHardeningValidationError {
    message: String,
}

impl ReviewBoundaryHardeningValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ReviewBoundaryHardeningValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ReviewBoundaryHardeningValidationError {}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn boundary_hardening_record(
    input: &ReviewBoundaryHardeningInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    stabilization_packet: &ReviewStabilizationPacket,
) -> ReviewBoundaryHardeningRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    invalidation_reasons.extend(derive_invalidation_reasons(
        &input.in_product_boundary,
        &input.browser_handoff_boundary,
        &input.boundary_freshness,
        &input.return_path,
        &input.boundary_ownership_signals,
    ));
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let blocked_reasons = derive_blocked_reasons(
        &input.boundary_hardening_state,
        &input.in_product_boundary,
        &input.browser_handoff_boundary,
        &input.boundary_freshness,
        &invalidation_reasons,
    );

    ReviewBoundaryHardeningRecord {
        record_kind: REVIEW_BOUNDARY_HARDENING_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id: input.boundary_hardening_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        stabilization_id_ref: stabilization_packet.stabilization.stabilization_id.clone(),
        boundary_hardening_state: input.boundary_hardening_state.clone(),
        invalidation_reasons,
        blocked_reasons,
        actionable: input.commands.iter().any(|c| c.blocked_reasons.is_empty()),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn browser_handoff_boundary_record(
    input: &BrowserHandoffBoundaryInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> BrowserHandoffBoundaryRecord {
    BrowserHandoffBoundaryRecord {
        record_kind: BROWSER_HANDOFF_BOUNDARY_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        handoff_boundary_class: input.handoff_boundary_class.clone(),
        handoff_id_ref: input.handoff_id_ref.clone(),
        destination_class: input.destination_class.clone(),
        destination_ref: input.destination_ref.clone(),
        object_identity_ref: input.object_identity_ref.clone(),
        reason_code: input.reason_code.clone(),
        return_path_present: input.return_path_present,
        reversible: input.reversible,
        summary_label: input.summary_label.clone(),
    }
}

fn in_product_review_boundary_record(
    input: &InProductReviewBoundaryInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> InProductReviewBoundaryRecord {
    InProductReviewBoundaryRecord {
        record_kind: IN_PRODUCT_REVIEW_BOUNDARY_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        boundary_authority_class: input.boundary_authority_class.clone(),
        local_authoritative: input.local_authoritative,
        provider_authoritative: input.provider_authoritative,
        local_provider_agree: input.local_provider_agree,
        hidden_authority_detected: input.hidden_authority_detected,
        summary_label: input.summary_label.clone(),
    }
}

fn provider_source_identity_record(
    input: &ProviderSourceIdentityInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> ProviderSourceIdentityRecord {
    ProviderSourceIdentityRecord {
        record_kind: PROVIDER_SOURCE_IDENTITY_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        provider_class: input.provider_class.clone(),
        connected_provider_record_id_ref: input.connected_provider_record_id_ref.clone(),
        provider_object_identity_ref: input.provider_object_identity_ref.clone(),
        source_class: input.source_class.clone(),
        actor_ref: input.actor_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn return_path_record(
    input: &ReturnPathInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> ReturnPathRecord {
    ReturnPathRecord {
        record_kind: RETURN_PATH_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        return_path_class: input.return_path_class.clone(),
        return_anchor_kind: input.return_anchor_kind.clone(),
        return_anchor_ref: input.return_anchor_ref.clone(),
        replay_posture: input.replay_posture.clone(),
        expired: input.expired,
        summary_label: input.summary_label.clone(),
    }
}

fn boundary_freshness_observation_record(
    input: &BoundaryFreshnessObservationInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> BoundaryFreshnessObservationRecord {
    BoundaryFreshnessObservationRecord {
        record_kind: BOUNDARY_FRESHNESS_OBSERVATION_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        boundary_freshness_class: input.boundary_freshness_class.clone(),
        provider_overlay_freshness_class: input.provider_overlay_freshness_class.clone(),
        last_fetched_at: input.last_fetched_at.clone(),
        blocks_mutation_when_stale: input.blocks_mutation_when_stale,
        summary_label: input.summary_label.clone(),
    }
}

fn boundary_ownership_signal_record(
    input: &BoundaryOwnershipSignalInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> BoundaryOwnershipSignalRecord {
    BoundaryOwnershipSignalRecord {
        record_kind: BOUNDARY_OWNERSHIP_SIGNAL_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        boundary_ownership_class: input.boundary_ownership_class.clone(),
        scope_pattern: input.scope_pattern.clone(),
        owner_ref: input.owner_ref.clone(),
        enforceable: input.enforceable,
        advisory: input.advisory,
        summary_label: input.summary_label.clone(),
    }
}

fn boundary_hardening_command_record(
    input: &BoundaryHardeningCommandInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
) -> BoundaryHardeningCommandRecord {
    BoundaryHardeningCommandRecord {
        record_kind: BOUNDARY_HARDENING_COMMAND_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        command_class: input.command_class.clone(),
        target_object_ref: input.target_object_ref.clone(),
        target_object_kind: input.target_object_kind.clone(),
        preview_supported: input.preview_supported,
        emits_audit_event: input.emits_audit_event,
        blocked_reasons: input.blocked_reasons.clone(),
        actionable: input.blocked_reasons.is_empty(),
        summary_label: input.summary_label.clone(),
    }
}

fn boundary_hardening_support_export_packet(
    input: &BoundaryHardeningSupportExportInput,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    stabilization_packet: &ReviewStabilizationPacket,
    commands: &[BoundaryHardeningCommandRecord],
) -> BoundaryHardeningSupportExportPacket {
    BoundaryHardeningSupportExportPacket {
        record_kind: BOUNDARY_HARDENING_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        stabilization_id_ref: stabilization_packet.stabilization.stabilization_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/harden_browser_handoff_and_in_product_review_boundaries.schema.json"
                .to_string(),
            "schemas/review/review_stabilization.schema.json".to_string(),
            "schemas/review/review_workspace.schema.json".to_string(),
            "schemas/review/anchor_id_alpha.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn boundary_hardening_inspection_record(
    boundary_hardening: &ReviewBoundaryHardeningRecord,
    browser_handoff_boundary: &BrowserHandoffBoundaryRecord,
    in_product_boundary: &InProductReviewBoundaryRecord,
    return_path: &ReturnPathRecord,
    boundary_freshness: &BoundaryFreshnessObservationRecord,
    boundary_ownership_signals: &[BoundaryOwnershipSignalRecord],
    commands: &[BoundaryHardeningCommandRecord],
    support_export: &BoundaryHardeningSupportExportPacket,
) -> BoundaryHardeningInspectionRecord {
    let boundary_hardened = boundary_hardening.boundary_hardening_state == "boundary_hardened";
    let boundary_degraded_provider_overlay_stale = boundary_hardening.boundary_hardening_state
        == "boundary_degraded_provider_overlay_stale";
    let boundary_degraded_missing_return_path =
        boundary_hardening.boundary_hardening_state == "boundary_degraded_missing_return_path";
    let boundary_degraded_hidden_authority =
        boundary_hardening.boundary_hardening_state == "boundary_degraded_hidden_authority";
    let boundary_degraded_freshness_unknown =
        boundary_hardening.boundary_hardening_state == "boundary_degraded_freshness_unknown";
    let boundary_degraded_ownership_ambiguous =
        boundary_hardening.boundary_hardening_state == "boundary_degraded_ownership_ambiguous";

    let handoff_reversible_typed = browser_handoff_boundary.handoff_boundary_class
        == "handoff_reversible_typed"
        && browser_handoff_boundary.return_path_present
        && browser_handoff_boundary.reversible;
    let in_product_local_authoritative = in_product_boundary.local_authoritative;
    let in_product_provider_authoritative = in_product_boundary.provider_authoritative;
    let local_provider_agree = in_product_boundary.local_provider_agree;
    let hidden_authority_detected = in_product_boundary.hidden_authority_detected;
    let return_path_present_and_valid =
        !return_path.return_anchor_ref.trim().is_empty() && !return_path.expired;
    let boundary_fresh_or_within_grace = boundary_freshness.boundary_freshness_class
        == "boundary_fresh"
        || boundary_freshness.boundary_freshness_class == "boundary_stale_within_grace";
    let boundary_freshness_blocks_mutation =
        boundary_freshness.boundary_freshness_class == "boundary_stale_blocks_mutation";
    let enforceable_ownership_at_boundary = boundary_ownership_signals.iter().any(|s| s.enforceable);
    let advisory_ownership_at_boundary = boundary_ownership_signals.iter().any(|s| s.advisory);
    let ownership_conflict_at_boundary =
        boundary_ownership_signals.iter().any(|s| s.boundary_ownership_class == "ownership_conflict_at_boundary");
    let actionable = boundary_hardening.actionable;
    let invalidated = !boundary_hardening.invalidation_reasons.is_empty();
    let preview_capable = commands.iter().any(|c| c.preview_supported);
    let support_export_reopenable =
        support_export_can_reopen(support_export, commands);

    BoundaryHardeningInspectionRecord {
        record_kind: BOUNDARY_HARDENING_INSPECTION_RECORD_KIND.to_string(),
        schema_version: REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        boundary_hardening_id_ref: boundary_hardening.boundary_hardening_id.clone(),
        review_workspace_id_ref: boundary_hardening.review_workspace_id_ref.clone(),
        boundary_hardened,
        boundary_degraded_provider_overlay_stale,
        boundary_degraded_missing_return_path,
        boundary_degraded_hidden_authority,
        boundary_degraded_freshness_unknown,
        boundary_degraded_ownership_ambiguous,
        handoff_reversible_typed,
        in_product_local_authoritative,
        in_product_provider_authoritative,
        local_provider_agree,
        hidden_authority_detected,
        return_path_present_and_valid,
        boundary_fresh_or_within_grace,
        boundary_freshness_blocks_mutation,
        enforceable_ownership_at_boundary,
        advisory_ownership_at_boundary,
        ownership_conflict_at_boundary,
        actionable,
        invalidated,
        command_count: commands.len(),
        boundary_ownership_signal_count: boundary_ownership_signals.len(),
        preview_capable,
        support_export_reopenable,
        summary_label: format!(
            "Review boundary hardening {} ({} command(s), {} ownership signal(s))",
            boundary_hardening.boundary_hardening_id,
            commands.len(),
            boundary_ownership_signals.len()
        ),
    }
}

fn support_export_can_reopen(
    export: &BoundaryHardeningSupportExportPacket,
    commands: &[BoundaryHardeningCommandRecord],
) -> bool {
    !export.reopen_context_ref.trim().is_empty()
        && !export.reopen_command_id_ref.trim().is_empty()
        && !export.raw_url_export_allowed
        && !export.raw_provider_payload_export_allowed
        && !commands.is_empty()
        && export
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export")
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_input(
    input: &ReviewBoundaryHardeningInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    stabilization_packet: &ReviewStabilizationPacket,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_nonempty(&input.boundary_hardening_id, "boundary_hardening_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    ensure_token(
        BOUNDARY_HARDENING_STATES,
        &input.boundary_hardening_state,
        "boundary_hardening_state",
    )?;

    ensure_token(
        HANDOFF_BOUNDARY_CLASSES,
        &input.browser_handoff_boundary.handoff_boundary_class,
        "handoff_boundary_class",
    )?;
    ensure_token(
        BOUNDARY_AUTHORITY_CLASSES,
        &input.in_product_boundary.boundary_authority_class,
        "boundary_authority_class",
    )?;
    ensure_token(
        RETURN_PATH_CLASSES,
        &input.return_path.return_path_class,
        "return_path_class",
    )?;
    ensure_token(
        BOUNDARY_FRESHNESS_CLASSES,
        &input.boundary_freshness.boundary_freshness_class,
        "boundary_freshness_class",
    )?;

    for signal in &input.boundary_ownership_signals {
        ensure_token(
            BOUNDARY_OWNERSHIP_CLASSES,
            &signal.boundary_ownership_class,
            "boundary_ownership_class",
        )?;
        if !signal.advisory && !signal.enforceable {
            return Err(boundary_hardening_validation_error(
                "boundary ownership signal must be advisory or enforceable (or both)",
            ));
        }
    }

    for command in &input.commands {
        ensure_token(
            BOUNDARY_HARDENING_COMMAND_CLASSES,
            &command.command_class,
            "command_class",
        )?;
    }

    ensure_nonempty(&input.support_export.support_export_id, "support_export_id")?;

    // Cross-packet consistency checks.
    if input.in_product_boundary.hidden_authority_detected
        && input.boundary_hardening_state == "boundary_hardened"
    {
        return Err(boundary_hardening_validation_error(
            "boundary_hardened state is incompatible with hidden_authority_detected",
        ));
    }

    if input.browser_handoff_boundary.reversible
        && input.browser_handoff_boundary.handoff_boundary_class != "handoff_reversible_typed"
    {
        return Err(boundary_hardening_validation_error(
            "reversible handoff must use handoff_reversible_typed class",
        ));
    }

    if input.return_path.expired
        && input.return_path.return_path_class != "return_path_expired"
    {
        return Err(boundary_hardening_validation_error(
            "expired return path must use return_path_expired class",
        ));
    }

    if input.in_product_boundary.local_authoritative
        && input.in_product_boundary.provider_authoritative
        && !input.in_product_boundary.local_provider_agree
    {
        return Err(boundary_hardening_validation_error(
            "when both local and provider are authoritative, local_provider_agree must be true",
        ));
    }

    if input.in_product_boundary.local_provider_agree
        && !input.in_product_boundary.local_authoritative
        && !input.in_product_boundary.provider_authoritative
    {
        return Err(boundary_hardening_validation_error(
            "local_provider_agree requires at least one authority side to be authoritative",
        ));
    }

    // Workspace/stabilization consistency.
    if workspace_packet.review_workspace.review_workspace_id
        != stabilization_packet.review_workspace.review_workspace_id
    {
        return Err(boundary_hardening_validation_error(
            "workspace packet review_workspace_id must match stabilization packet review_workspace_id",
        ));
    }

    Ok(())
}

fn validate_boundary_hardening_record(
    record: &ReviewBoundaryHardeningRecord,
    review_workspace_id: &str,
    stabilization_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REVIEW_BOUNDARY_HARDENING_RECORD_KIND,
        "boundary_hardening record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
        "boundary_hardening schema_version",
    )?;
    ensure_nonempty(&record.boundary_hardening_id, "boundary_hardening_id")?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        review_workspace_id,
        "boundary_hardening review_workspace_id_ref",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "boundary_hardening stabilization_id_ref",
    )?;
    ensure_token(
        BOUNDARY_HARDENING_STATES,
        &record.boundary_hardening_state,
        "boundary_hardening_state",
    )?;
    Ok(())
}

fn validate_browser_handoff_boundary_record(
    record: &BrowserHandoffBoundaryRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        BROWSER_HANDOFF_BOUNDARY_RECORD_KIND,
        "browser_handoff_boundary record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "browser_handoff_boundary boundary_hardening_id_ref",
    )?;
    ensure_token(
        HANDOFF_BOUNDARY_CLASSES,
        &record.handoff_boundary_class,
        "handoff_boundary_class",
    )?;
    Ok(())
}

fn validate_in_product_review_boundary_record(
    record: &InProductReviewBoundaryRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        IN_PRODUCT_REVIEW_BOUNDARY_RECORD_KIND,
        "in_product_boundary record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "in_product_boundary boundary_hardening_id_ref",
    )?;
    ensure_token(
        BOUNDARY_AUTHORITY_CLASSES,
        &record.boundary_authority_class,
        "boundary_authority_class",
    )?;
    Ok(())
}

fn validate_provider_source_identity_record(
    record: &ProviderSourceIdentityRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PROVIDER_SOURCE_IDENTITY_RECORD_KIND,
        "provider_source_identity record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "provider_source_identity boundary_hardening_id_ref",
    )?;
    ensure_nonempty(&record.provider_class, "provider_class")?;
    ensure_nonempty(&record.provider_object_identity_ref, "provider_object_identity_ref")?;
    ensure_nonempty(&record.actor_ref, "actor_ref")?;
    Ok(())
}

fn validate_return_path_record(
    record: &ReturnPathRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        RETURN_PATH_RECORD_KIND,
        "return_path record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "return_path boundary_hardening_id_ref",
    )?;
    ensure_token(
        RETURN_PATH_CLASSES,
        &record.return_path_class,
        "return_path_class",
    )?;
    Ok(())
}

fn validate_boundary_freshness_observation_record(
    record: &BoundaryFreshnessObservationRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        BOUNDARY_FRESHNESS_OBSERVATION_RECORD_KIND,
        "boundary_freshness record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "boundary_freshness boundary_hardening_id_ref",
    )?;
    ensure_token(
        BOUNDARY_FRESHNESS_CLASSES,
        &record.boundary_freshness_class,
        "boundary_freshness_class",
    )?;
    Ok(())
}

fn validate_boundary_ownership_signal_record(
    record: &BoundaryOwnershipSignalRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        BOUNDARY_OWNERSHIP_SIGNAL_RECORD_KIND,
        "boundary_ownership_signal record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "boundary_ownership_signal boundary_hardening_id_ref",
    )?;
    ensure_token(
        BOUNDARY_OWNERSHIP_CLASSES,
        &record.boundary_ownership_class,
        "boundary_ownership_class",
    )?;
    Ok(())
}

fn validate_command_record(
    record: &BoundaryHardeningCommandRecord,
    boundary_hardening_id: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        BOUNDARY_HARDENING_COMMAND_RECORD_KIND,
        "command record_kind",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "command boundary_hardening_id_ref",
    )?;
    ensure_token(
        BOUNDARY_HARDENING_COMMAND_CLASSES,
        &record.command_class,
        "command_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &BoundaryHardeningSupportExportPacket,
    boundary_hardening: &ReviewBoundaryHardeningRecord,
    commands: &[BoundaryHardeningCommandRecord],
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        BOUNDARY_HARDENING_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export record_kind",
    )?;
    ensure_eq(
        export.boundary_hardening_id_ref.as_str(),
        boundary_hardening.boundary_hardening_id.as_str(),
        "support_export boundary_hardening_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        boundary_hardening.review_workspace_id_ref.as_str(),
        "support_export review_workspace_id_ref",
    )?;
    ensure_eq(
        export.stabilization_id_ref.as_str(),
        boundary_hardening.stabilization_id_ref.as_str(),
        "support_export stabilization_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(boundary_hardening_validation_error(
            "support_export raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(boundary_hardening_validation_error(
            "support_export raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.command_id_refs.len() != commands.len() {
        return Err(boundary_hardening_validation_error(
            "support_export command_id_refs length must match commands length",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &BoundaryHardeningInspectionRecord,
    packet: &ReviewBoundaryHardeningPacket,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        BOUNDARY_HARDENING_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.boundary_hardening_id_ref.as_str(),
        packet.boundary_hardening.boundary_hardening_id.as_str(),
        "inspection boundary_hardening_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection review_workspace_id_ref",
    )?;
    if inspection.command_count != packet.commands.len() {
        return Err(boundary_hardening_validation_error(
            "inspection command_count must match commands length",
        ));
    }
    if inspection.boundary_ownership_signal_count != packet.boundary_ownership_signals.len() {
        return Err(boundary_hardening_validation_error(
            "inspection boundary_ownership_signal_count must match boundary_ownership_signals length",
        ));
    }
    if inspection.hidden_authority_detected != packet.in_product_boundary.hidden_authority_detected {
        return Err(boundary_hardening_validation_error(
            "inspection hidden_authority_detected must match in_product_boundary.hidden_authority_detected",
        ));
    }
    if inspection.handoff_reversible_typed != packet.browser_handoff_boundary.reversible {
        return Err(boundary_hardening_validation_error(
            "inspection handoff_reversible_typed must match browser_handoff_boundary.reversible when class is handoff_reversible_typed",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Derivation helpers
// ---------------------------------------------------------------------------

fn derive_invalidation_reasons(
    in_product: &InProductReviewBoundaryInput,
    handoff: &BrowserHandoffBoundaryInput,
    freshness: &BoundaryFreshnessObservationInput,
    return_path: &ReturnPathInput,
    boundary_ownership_signals: &[BoundaryOwnershipSignalInput],
) -> Vec<String> {
    let mut reasons = Vec::new();
    if in_product.hidden_authority_detected {
        reasons.push("hidden_authority_detected".to_string());
    }
    if handoff.handoff_boundary_class == "handoff_untyped" {
        reasons.push("browser_handoff_untyped".to_string());
    }
    if freshness.boundary_freshness_class == "boundary_freshness_unknown" {
        reasons.push("freshness_unknown".to_string());
    }
    if return_path.return_path_class == "return_path_missing"
        || return_path.return_path_class == "return_path_expired"
    {
        reasons.push("missing_return_path".to_string());
    }
    if boundary_ownership_signals.iter().any(|s| s.boundary_ownership_class == "ownership_conflict_at_boundary") {
        reasons.push("ownership_ambiguous".to_string());
    }
    reasons
}

fn derive_blocked_reasons(
    boundary_hardening_state: &str,
    in_product: &InProductReviewBoundaryInput,
    handoff: &BrowserHandoffBoundaryInput,
    freshness: &BoundaryFreshnessObservationInput,
    invalidation_reasons: &[String],
) -> Vec<String> {
    let mut reasons = Vec::new();
    if boundary_hardening_state == "boundary_degraded_provider_overlay_stale" {
        reasons.push("provider_overlay_stale".to_string());
    }
    if boundary_hardening_state == "boundary_degraded_missing_return_path" {
        reasons.push("missing_return_path".to_string());
    }
    if boundary_hardening_state == "boundary_degraded_hidden_authority" {
        reasons.push("hidden_authority_detected".to_string());
    }
    if boundary_hardening_state == "boundary_degraded_freshness_unknown" {
        reasons.push("freshness_unknown".to_string());
    }
    if boundary_hardening_state == "boundary_degraded_ownership_ambiguous" {
        reasons.push("ownership_ambiguous".to_string());
    }
    if in_product.hidden_authority_detected {
        reasons.push("hidden_authority_blocks_mutation".to_string());
    }
    if handoff.handoff_boundary_class == "handoff_authority_revoked" {
        reasons.push("handoff_authority_revoked".to_string());
    }
    if freshness.boundary_freshness_class == "boundary_stale_blocks_mutation" {
        reasons.push("freshness_blocks_mutation".to_string());
    }
    for reason in invalidation_reasons {
        if reason == "approval_invalidated" {
            reasons.push("approval_invalidated".to_string());
        }
        if reason == "anchor_drifted" {
            reasons.push("anchor_drifted".to_string());
        }
    }
    reasons.sort();
    reasons.dedup();
    reasons
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn boundary_hardening_validation_error(
    message: impl Into<String>,
) -> ReviewBoundaryHardeningValidationError {
    ReviewBoundaryHardeningValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(boundary_hardening_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    if value.trim().is_empty() {
        return Err(boundary_hardening_validation_error(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), ReviewBoundaryHardeningValidationError> {
    if !tokens.contains(&value) {
        return Err(boundary_hardening_validation_error(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

fn contains_token(tokens: &[&str], value: &str) -> bool {
    tokens.contains(&value)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_nonempty() {
        assert!(!BOUNDARY_HARDENING_STATES.is_empty());
        assert!(!BOUNDARY_AUTHORITY_CLASSES.is_empty());
        assert!(!HANDOFF_BOUNDARY_CLASSES.is_empty());
        assert!(!RETURN_PATH_CLASSES.is_empty());
        assert!(!BOUNDARY_FRESHNESS_CLASSES.is_empty());
        assert!(!BOUNDARY_OWNERSHIP_CLASSES.is_empty());
        assert!(!BOUNDARY_HARDENING_COMMAND_CLASSES.is_empty());
        assert!(!BOUNDARY_HARDENING_CONSUMER_SURFACES.is_empty());
        assert!(!BOUNDARY_HARDENING_INVALIDATION_REASONS.is_empty());
    }
}
