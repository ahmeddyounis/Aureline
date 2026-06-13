//! Launch-inspector and command-runtime explain sheets that answer, for one
//! invocation route, *where this runs*, *why this toolchain*, *what it can
//! access*, and *who approved it*.
//!
//! The frozen runtime-authority matrix states what *may* be granted per claimed
//! M5 executing surface; the execution-surface resolution packet states which
//! sandbox profile and toolchain back-end a surface resolves to on a platform;
//! and the capability-envelope packet states the concrete authority issued for
//! one execution. This module is the **operator-facing projection** of those
//! three truths: an export-safe explain sheet, one per launch route, that
//! answers the four questions an operator actually asks before trusting a
//! launch — and that every consuming surface (desktop, CLI/headless, help/About,
//! support export, diagnostics, release evidence) renders identically instead of
//! cloning per-surface approval or capability prose.
//!
//! Each [`M5LaunchExplainSheet`] is bound to one [`M5LaunchRoute`] (desktop, CLI,
//! AI, recipe, extension, remote, or companion) and a representative matrix
//! executing surface, and carries four answer sections:
//!
//! - [`M5WhereItRuns`] — the sandbox profile, execution back-end, platform,
//!   profile-resolution status, off-device flag, and verified target identity.
//! - [`M5WhyThisToolchain`] — the resolved toolchain label, its back-end class,
//!   and the export-safe reason it was selected.
//! - [`M5WhatItCanAccess`] — the granted capability classes (always a subset of
//!   the matrix row), the allowed roots/sinks/endpoints, the secret scope, and
//!   the capability-envelope reference it projects.
//! - [`M5WhoApprovedIt`] — the approval-ticket posture and reference, the issuer,
//!   the governing policy epoch, the ordered decision chain, and the always-false
//!   `self_issued_by_executor` flag.
//!
//! The track invariant holds end to end: no ambient privilege; no AI, recipe,
//! extension, browser-routed, or remote helper self-issues authority; and if an
//! explainability packet is missing, stale, or unsupported on the platform, the
//! sheet **degrades to an explicit partial or unsupported label that still
//! carries all four answer sections and a named degraded reason** — it never
//! silently omits an authority fact. Off-device routes (remote, companion)
//! preserve the identical sheet shape, including a verified target identity, even
//! when execution is brokered by another runtime.
//!
//! The boundary schema is
//! [`schemas/execution-auth/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.schema.json`](../../../../schemas/execution-auth/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.schema.json).
//! The contract doc is
//! [`docs/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.md`](../../../../docs/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.md).
//! The protected fixture directory is
//! [`fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/`](../../../../fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5ApprovalTicketPosture, M5CapabilityClass,
    M5DegradedFallback, M5ExecutingSurface, M5RuntimeAuthorityDowngradeTrigger, M5SandboxProfile,
    M5SecretScope, M5UnsupportedProfileBehavior, M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF, M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
};
use super::implement_execution_surface_classes_sandbox_profile_descriptors_and_unsupported_or_stricter_profile_truth::{
    M5ExecutionBackendClass, M5ExecutionPlatform, M5ProfileResolutionStatus,
    M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF, M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
};
use super::ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::{
    frozen_stable_m5_capability_envelope_packet, M5_CAPABILITY_ENVELOPE_DOC_REF,
    M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5LaunchInspectorPacket`].
pub const M5_LAUNCH_INSPECTOR_RECORD_KIND: &str = "add_m5_launch_inspector_explain_sheets";

/// Schema version for the M5 launch-inspector explain-sheet packet records.
pub const M5_LAUNCH_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_LAUNCH_INSPECTOR_SCHEMA_REF: &str =
    "schemas/execution-auth/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LAUNCH_INSPECTOR_DOC_REF: &str =
    "docs/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LAUNCH_INSPECTOR_ARTIFACT_REF: &str =
    "artifacts/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_LAUNCH_INSPECTOR_SUMMARY_REF: &str =
    "artifacts/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_LAUNCH_INSPECTOR_FIXTURE_DIR: &str =
    "fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces";

/// Stable packet id minted by [`frozen_stable_m5_launch_inspector_packet`].
pub const M5_LAUNCH_INSPECTOR_PACKET_ID: &str = "m5-launch-inspector-explain-sheets:stable:0001";

/// Launch route an invocation arrives through.
///
/// The four questions an operator asks before trusting a launch must be
/// answerable identically regardless of which route initiated it. Helper routes
/// ([`Self::is_helper_route`]) — AI, recipe, extension, and remote — must never
/// self-issue authority; off-primary-device routes ([`Self::is_off_primary_device`])
/// — remote and companion — preserve the identical sheet shape off-device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchRoute {
    /// A launch initiated from the desktop shell.
    Desktop,
    /// A launch initiated from the CLI / headless runner.
    Cli,
    /// A launch initiated by an AI tool invocation.
    Ai,
    /// A launch initiated by a saved automation recipe.
    Recipe,
    /// A launch initiated by a loaded extension.
    Extension,
    /// A launch initiated against a remote / managed runtime.
    Remote,
    /// A launch initiated from a paired companion app or device.
    Companion,
}

impl M5LaunchRoute {
    /// Every launch route, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Desktop,
        Self::Cli,
        Self::Ai,
        Self::Recipe,
        Self::Extension,
        Self::Remote,
        Self::Companion,
    ];

    /// Stable token recorded in the explain sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Cli => "cli",
            Self::Ai => "ai",
            Self::Recipe => "recipe",
            Self::Extension => "extension",
            Self::Remote => "remote",
            Self::Companion => "companion",
        }
    }

    /// Whether this route is an untrusted helper that must never self-issue
    /// authority and must carry an externally issued approval lineage.
    pub const fn is_helper_route(self) -> bool {
        matches!(
            self,
            Self::Ai | Self::Recipe | Self::Extension | Self::Remote
        )
    }

    /// Whether this route runs off the primary device and therefore must carry a
    /// verified target identity and an off-device-marked sheet.
    pub const fn is_off_primary_device(self) -> bool {
        matches!(self, Self::Remote | Self::Companion)
    }
}

/// Reason a toolchain / execution back-end was selected for a launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToolchainSelectionReason {
    /// The project pinned this toolchain explicitly.
    ProjectPinnedToolchain,
    /// Policy mandated this runtime for the surface.
    PolicyMandatedRuntime,
    /// The platform default runtime was used because nothing stricter applied.
    PlatformDefaultRuntime,
    /// A stricter fallback runtime was selected because the preferred one was unavailable.
    NarrowedFallbackRuntime,
    /// Execution is brokered through a remote / managed runtime.
    RemoteBrokeredRuntime,
}

impl M5ToolchainSelectionReason {
    /// Stable token recorded in the explain sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectPinnedToolchain => "project_pinned_toolchain",
            Self::PolicyMandatedRuntime => "policy_mandated_runtime",
            Self::PlatformDefaultRuntime => "platform_default_runtime",
            Self::NarrowedFallbackRuntime => "narrowed_fallback_runtime",
            Self::RemoteBrokeredRuntime => "remote_brokered_runtime",
        }
    }
}

/// Overall explainability status of a launch explain sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplainStatus {
    /// All four answers are fully resolved at full authority.
    Complete,
    /// The launch is narrowed; the sheet carries a named degraded reason but
    /// still answers all four questions.
    PartialDegraded,
    /// The launch's sandbox profile is unsupported on this platform; the sheet
    /// carries an explicit unsupported reason and still answers all four questions.
    UnsupportedOnPlatform,
}

impl M5ExplainStatus {
    /// Stable token recorded in the explain sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::PartialDegraded => "partial_degraded",
            Self::UnsupportedOnPlatform => "unsupported_on_platform",
        }
    }

    /// Whether this status represents a full-authority, non-degraded launch.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Reason class for a degraded or unsupported explain sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplainDegradationReason {
    /// The explainability proof packet is stale beyond its freshness SLO.
    StaleProofPacket,
    /// An explainability packet for this launch is missing.
    MissingExplainabilityPacket,
    /// The default sandbox profile is unsupported on this platform.
    UnsupportedProfileOnPlatform,
    /// The approval ticket backing this launch has expired.
    ApprovalTicketExpired,
    /// The required enforcement backend is missing.
    EnforcementBackendMissing,
    /// The governing policy epoch was superseded.
    PolicyEpochSuperseded,
    /// The execution target identity could not be verified.
    TargetIdentityUnverified,
}

impl M5ExplainDegradationReason {
    /// Stable token recorded in the explain sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleProofPacket => "stale_proof_packet",
            Self::MissingExplainabilityPacket => "missing_explainability_packet",
            Self::UnsupportedProfileOnPlatform => "unsupported_profile_on_platform",
            Self::ApprovalTicketExpired => "approval_ticket_expired",
            Self::EnforcementBackendMissing => "enforcement_backend_missing",
            Self::PolicyEpochSuperseded => "policy_epoch_superseded",
            Self::TargetIdentityUnverified => "target_identity_unverified",
        }
    }
}

/// "Where this runs" answer block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WhereItRuns {
    /// The matrix executing surface this launch resolves to.
    pub executing_surface: M5ExecutingSurface,
    /// Sandbox profile the launch runs under (the matrix default or fully inert).
    pub sandbox_profile: M5SandboxProfile,
    /// Execution back-end class the launch runs on.
    pub execution_backend: M5ExecutionBackendClass,
    /// Platform the launch resolves on.
    pub platform: M5ExecutionPlatform,
    /// Profile-resolution status on this platform.
    pub profile_resolution_status: M5ProfileResolutionStatus,
    /// True when execution runs off-device or is brokered by another runtime.
    pub off_device: bool,
    /// Export-safe target identity (host label, path, or resource id).
    pub target_identity: String,
    /// True when the target identity has been verified.
    pub target_identity_verified: bool,
    /// Export-safe one-line isolation summary an operator can read directly.
    pub isolation_label: String,
}

/// "Why this toolchain" answer block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WhyThisToolchain {
    /// Export-safe toolchain / runtime label.
    pub toolchain_label: String,
    /// Execution back-end class backing the toolchain.
    pub backend_class: M5ExecutionBackendClass,
    /// The reason this toolchain was selected.
    pub selection_reason: M5ToolchainSelectionReason,
    /// Export-safe reference to where the toolchain was resolved from.
    pub resolved_from_ref: String,
}

/// "What it can access" answer block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WhatItCanAccess {
    /// Capability classes this launch may exercise (a subset of the matrix row).
    pub granted_capability_classes: Vec<M5CapabilityClass>,
    /// Export-safe allowed roots, sinks, and endpoints.
    pub allowed_scope_labels: Vec<String>,
    /// Secret scope for this launch.
    pub secret_scope: M5SecretScope,
    /// Export-safe reference to the capability envelope this launch projects.
    pub capability_envelope_ref: String,
}

/// "Who approved it" answer block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WhoApprovedIt {
    /// Approval-ticket posture this launch was authorized under.
    pub approval_posture: M5ApprovalTicketPosture,
    /// Export-safe approval-ticket reference (never a live ticket signature).
    pub approval_ticket_ref: String,
    /// Export-safe external issuer label.
    pub issuer_label: String,
    /// Export-safe governing policy-epoch label.
    pub policy_epoch_label: String,
    /// Ordered export-safe lineage refs from policy epoch to launch.
    pub decision_chain: Vec<String>,
    /// Always false: the executor never self-issues this authority.
    pub self_issued_by_executor: bool,
}

/// Degraded / unsupported reason block carried by a non-complete sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplainDegradation {
    /// Reason class.
    pub reason: M5ExplainDegradationReason,
    /// What the launch narrows to when full authority cannot be honored.
    pub narrowed_to: M5DegradedFallback,
    /// Export-safe human explanation; never empty when a degradation is present.
    pub explanation: String,
}

/// One launch-inspector explain sheet for a single launch route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchExplainSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// The launch route this sheet explains.
    pub route: M5LaunchRoute,
    /// Overall explainability status.
    pub status: M5ExplainStatus,
    /// "Where this runs" answer.
    pub where_it_runs: M5WhereItRuns,
    /// "Why this toolchain" answer.
    pub why_this_toolchain: M5WhyThisToolchain,
    /// "What it can access" answer.
    pub what_it_can_access: M5WhatItCanAccess,
    /// "Who approved it" answer.
    pub who_approved_it: M5WhoApprovedIt,
    /// Named degraded reason; present exactly when `status` is not `complete`.
    pub degradation: Option<M5ExplainDegradation>,
    /// Downgrade triggers applied to this launch; empty when nominal.
    pub applied_downgrade_triggers: Vec<M5RuntimeAuthorityDowngradeTrigger>,
    /// Unsupported-profile behavior; present when the profile is unsupported.
    pub unsupported_profile_behavior: Option<M5UnsupportedProfileBehavior>,
    /// Per-sheet redaction class token.
    pub redaction_class_token: String,
}

impl M5LaunchExplainSheet {
    /// The executing surface this sheet resolves to.
    pub fn surface(&self) -> M5ExecutingSurface {
        self.where_it_runs.executing_surface
    }
}

/// Trust and isolation review block for the explain-sheet packet.
///
/// Every field encodes a hard invariant; all must hold for the packet to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchInspectorTrustReview {
    /// No explain sheet grants ambient machine privilege.
    pub no_ambient_machine_privilege: bool,
    /// No helper route self-issues authority.
    pub no_self_issued_authority_by_helpers: bool,
    /// All four answers are present on every sheet, including degraded ones.
    pub all_four_answers_present_even_when_degraded: bool,
    /// Degraded or unsupported sheets carry an explicit named reason.
    pub degraded_state_reasons_explicit: bool,
    /// Granted access never widens the matrix row it projects.
    pub access_never_widens_matrix: bool,
    /// Secret references are handle-only; no raw secret material is shown.
    pub secret_refs_handle_only_no_raw_material: bool,
    /// Off-device routes preserve the identical sheet shape and verified identity.
    pub off_device_preserves_sheet_semantics: bool,
    /// Enforcement fails closed or narrows when it cannot be honored, never widening.
    pub fail_closed_when_enforcement_unavailable: bool,
    /// No raw secret material is exported inside sheets or support packets.
    pub no_raw_secret_material_in_exports: bool,
}

/// Consumer projection block for the explain-sheet packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchInspectorConsumerProjection {
    /// Desktop launch inspector shows the full sheet.
    pub desktop_shows_explain_sheet: bool,
    /// Command palette and policy inspector reference the same sheets.
    pub command_and_policy_reference_same_sheets: bool,
    /// CLI / headless shows the full sheet.
    pub cli_headless_shows_explain_sheet: bool,
    /// Support export shows the full sheet.
    pub support_export_shows_explain_sheet: bool,
    /// Diagnostics shows the full sheet.
    pub diagnostics_shows_explain_sheet: bool,
    /// Help / About shows an explain-sheet summary.
    pub help_about_shows_explain_summary: bool,
    /// Release evidence consumes these sheets instead of cloning per-surface prose.
    pub release_evidence_consumes_sheets: bool,
    /// Every launch route projects the identical four answers and degraded reasons.
    pub all_routes_project_same_facts: bool,
}

/// Proof-freshness block for the explain-sheet packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchInspectorProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when a stale or missing proof automatically narrows the affected sheets
    /// to a partial / unsupported label instead of omitting authority facts.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5LaunchInspectorPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LaunchInspectorPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Launch explain sheets, one per route.
    pub sheets: Vec<M5LaunchExplainSheet>,
    /// Trust review block.
    pub trust_review: M5LaunchInspectorTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5LaunchInspectorConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LaunchInspectorProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 launch-inspector explain-sheet packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchInspectorPacket {
    /// Record kind; must equal [`M5_LAUNCH_INSPECTOR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LAUNCH_INSPECTOR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Launch explain sheets, one per route.
    pub sheets: Vec<M5LaunchExplainSheet>,
    /// Trust review block.
    pub trust_review: M5LaunchInspectorTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5LaunchInspectorConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LaunchInspectorProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LaunchInspectorPacket {
    /// Builds an M5 launch-inspector packet from frozen input.
    pub fn new(input: M5LaunchInspectorPacketInput) -> Self {
        Self {
            record_kind: M5_LAUNCH_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: M5_LAUNCH_INSPECTOR_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            sheets: input.sheets,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 launch-inspector packet invariants.
    pub fn validate(&self) -> Vec<M5LaunchInspectorViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LAUNCH_INSPECTOR_RECORD_KIND {
            violations.push(M5LaunchInspectorViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LAUNCH_INSPECTOR_SCHEMA_VERSION {
            violations.push(M5LaunchInspectorViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LaunchInspectorViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_sheets(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 launch-inspector packet serializes"),
        ) {
            violations.push(M5LaunchInspectorViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 launch-inspector packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let degraded = self
            .sheets
            .iter()
            .filter(|sheet| !sheet.status.is_complete())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Launch-Inspector Explain Sheets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Sheets: {} ({} degraded or unsupported)\n",
            self.sheets.len(),
            degraded
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Explain sheets by route\n\n");
        for sheet in &self.sheets {
            out.push_str(&format!(
                "- **{}** ({}) — status: {}\n",
                sheet.route.as_str(),
                sheet.sheet_id,
                sheet.status.as_str()
            ));
            out.push_str(&format!(
                "  - Where it runs: {} on {} ({}) via {}{}\n",
                sheet.where_it_runs.executing_surface.as_str(),
                sheet.where_it_runs.platform.as_str(),
                sheet.where_it_runs.sandbox_profile.as_str(),
                sheet.where_it_runs.execution_backend.as_str(),
                if sheet.where_it_runs.off_device {
                    ", off-device"
                } else {
                    ""
                }
            ));
            out.push_str(&format!(
                "  - Why this toolchain: {} ({})\n",
                sheet.why_this_toolchain.toolchain_label,
                sheet.why_this_toolchain.selection_reason.as_str()
            ));
            let caps = sheet
                .what_it_can_access
                .granted_capability_classes
                .iter()
                .map(|cap| cap.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - What it can access: {caps} · Secret scope: {}\n",
                sheet.what_it_can_access.secret_scope.as_str()
            ));
            out.push_str(&format!(
                "  - Who approved it: {} (`{}`) · Posture: {} · Epoch: {}\n",
                sheet.who_approved_it.issuer_label,
                sheet.who_approved_it.approval_ticket_ref,
                sheet.who_approved_it.approval_posture.as_str(),
                sheet.who_approved_it.policy_epoch_label
            ));
            if let Some(degradation) = &sheet.degradation {
                out.push_str(&format!(
                    "  - Degraded: {} → narrows to {} ({})\n",
                    degradation.reason.as_str(),
                    degradation.narrowed_to.as_str(),
                    degradation.explanation
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 launch-inspector export.
#[derive(Debug)]
pub enum M5LaunchInspectorArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LaunchInspectorViolation>),
}

impl fmt::Display for M5LaunchInspectorArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 launch-inspector export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 launch-inspector export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LaunchInspectorArtifactError {}

/// Validation failures emitted by [`M5LaunchInspectorPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LaunchInspectorViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required launch route has no explain sheet.
    RequiredRouteMissing,
    /// A sheet is missing required identity fields.
    SheetIncomplete,
    /// A "where it runs" answer is missing required facts.
    WhereAnswerIncomplete,
    /// A "why this toolchain" answer is missing required facts.
    WhyAnswerIncomplete,
    /// A "what it can access" answer is missing required facts.
    WhatAnswerIncomplete,
    /// A "who approved it" answer is missing required facts.
    WhoAnswerIncomplete,
    /// A sheet grants a capability class outside its matrix surface row.
    CapabilityWidensBeyondMatrix,
    /// A sheet runs under a sandbox profile that widens its matrix default.
    SandboxProfileWidens,
    /// A helper route self-issues authority instead of carrying external lineage.
    SelfIssuedAuthorityForbidden,
    /// An elevated capability is granted without an externally issued ticket ref.
    ElevatedCapabilityWithoutTicket,
    /// The secret scope and granted capabilities are inconsistent.
    SecretScopeInconsistent,
    /// An off-device sheet binds to an unverified target identity.
    OffDeviceTargetUnverified,
    /// A non-complete sheet omits its named degraded reason.
    DegradedReasonMissing,
    /// A complete sheet carries a degraded reason or downgrade triggers.
    StatusInconsistent,
    /// A degraded reason carries no explanation.
    DegradedExplanationMissing,
    /// An unsupported sheet omits its unsupported-profile behavior.
    UnsupportedBehaviorMissing,
    /// The capability-envelope reference points at an uncovered surface.
    EnvelopeReferenceUncovered,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5LaunchInspectorViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredRouteMissing => "required_route_missing",
            Self::SheetIncomplete => "sheet_incomplete",
            Self::WhereAnswerIncomplete => "where_answer_incomplete",
            Self::WhyAnswerIncomplete => "why_answer_incomplete",
            Self::WhatAnswerIncomplete => "what_answer_incomplete",
            Self::WhoAnswerIncomplete => "who_answer_incomplete",
            Self::CapabilityWidensBeyondMatrix => "capability_widens_beyond_matrix",
            Self::SandboxProfileWidens => "sandbox_profile_widens",
            Self::SelfIssuedAuthorityForbidden => "self_issued_authority_forbidden",
            Self::ElevatedCapabilityWithoutTicket => "elevated_capability_without_ticket",
            Self::SecretScopeInconsistent => "secret_scope_inconsistent",
            Self::OffDeviceTargetUnverified => "off_device_target_unverified",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::StatusInconsistent => "status_inconsistent",
            Self::DegradedExplanationMissing => "degraded_explanation_missing",
            Self::UnsupportedBehaviorMissing => "unsupported_behavior_missing",
            Self::EnvelopeReferenceUncovered => "envelope_reference_uncovered",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen stable M5 launch-inspector packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_LAUNCH_INSPECTOR_ARTIFACT_REF`]; the dumper emits this packet and a
/// test asserts the checked-in artifact deserializes back to it unchanged.
pub fn frozen_stable_m5_launch_inspector_packet() -> M5LaunchInspectorPacket {
    let sheets = vec![
        desktop_sheet(),
        cli_sheet(),
        ai_sheet(),
        recipe_sheet(),
        extension_sheet(),
        remote_sheet(),
        companion_sheet(),
    ];

    M5LaunchInspectorPacket::new(M5LaunchInspectorPacketInput {
        packet_id: M5_LAUNCH_INSPECTOR_PACKET_ID.to_owned(),
        packet_label: "M5 Launch-Inspector Explain Sheets".to_owned(),
        sheets,
        trust_review: M5LaunchInspectorTrustReview {
            no_ambient_machine_privilege: true,
            no_self_issued_authority_by_helpers: true,
            all_four_answers_present_even_when_degraded: true,
            degraded_state_reasons_explicit: true,
            access_never_widens_matrix: true,
            secret_refs_handle_only_no_raw_material: true,
            off_device_preserves_sheet_semantics: true,
            fail_closed_when_enforcement_unavailable: true,
            no_raw_secret_material_in_exports: true,
        },
        consumer_projection: M5LaunchInspectorConsumerProjection {
            desktop_shows_explain_sheet: true,
            command_and_policy_reference_same_sheets: true,
            cli_headless_shows_explain_sheet: true,
            support_export_shows_explain_sheet: true,
            diagnostics_shows_explain_sheet: true,
            help_about_shows_explain_summary: true,
            release_evidence_consumes_sheets: true,
            all_routes_project_same_facts: true,
        },
        proof_freshness: M5LaunchInspectorProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            M5_LAUNCH_INSPECTOR_SCHEMA_REF.to_owned(),
            M5_LAUNCH_INSPECTOR_DOC_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_SCHEMA_REF.to_owned(),
            M5_CAPABILITY_ENVELOPE_DOC_REF.to_owned(),
            M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF.to_owned(),
            M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF.to_owned(),
            M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in stable M5 launch-inspector export.
pub fn current_stable_m5_launch_inspector_export(
) -> Result<M5LaunchInspectorPacket, M5LaunchInspectorArtifactError> {
    let packet: M5LaunchInspectorPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/support_export.json"
    )))
    .map_err(M5LaunchInspectorArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LaunchInspectorArtifactError::Validation(violations))
    }
}

fn nominal_who(
    posture: M5ApprovalTicketPosture,
    ticket: &str,
    issuer: &str,
    chain_tail: &str,
) -> M5WhoApprovedIt {
    M5WhoApprovedIt {
        approval_posture: posture,
        approval_ticket_ref: ticket.to_owned(),
        issuer_label: issuer.to_owned(),
        policy_epoch_label: "policy-epoch:m5:0007".to_owned(),
        decision_chain: vec![
            "policy-epoch:m5:0007".to_owned(),
            issuer.to_owned(),
            chain_tail.to_owned(),
        ],
        self_issued_by_executor: false,
    }
}

fn desktop_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:desktop:0001".to_owned(),
        route: M5LaunchRoute::Desktop,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::NotebookKernel,
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            execution_backend: M5ExecutionBackendClass::LocalIsolated,
            platform: M5ExecutionPlatform::MacosDesktop,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: false,
            target_identity: "kernel://project/notebook-7".to_owned(),
            target_identity_verified: true,
            isolation_label: "Isolated local subprocess on this device.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Project notebook kernel (pinned)".to_owned(),
            backend_class: M5ExecutionBackendClass::LocalIsolated,
            selection_reason: M5ToolchainSelectionReason::ProjectPinnedToolchain,
            resolved_from_ref: "project-config://toolchain/notebook".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
                M5CapabilityClass::NetworkEgress,
            ],
            allowed_scope_labels: vec![
                "filesystem_root:workspace://project=read_write".to_owned(),
                "network_endpoint:broker://transport-plane=send_only".to_owned(),
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
            capability_envelope_ref: "envelope:notebook-kernel:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerSession,
            "ticket:notebook-kernel:0001",
            "issuer:approval-broker:local",
            "ticket:notebook-kernel:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn cli_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:cli:0001".to_owned(),
        route: M5LaunchRoute::Cli,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::ScaffoldHook,
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            execution_backend: M5ExecutionBackendClass::LocalIsolated,
            platform: M5ExecutionPlatform::HeadlessCi,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: false,
            target_identity: "workspace://project/templates".to_owned(),
            target_identity_verified: true,
            isolation_label: "Isolated local subprocess on the headless runner.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Project scaffold generator (pinned)".to_owned(),
            backend_class: M5ExecutionBackendClass::LocalIsolated,
            selection_reason: M5ToolchainSelectionReason::ProjectPinnedToolchain,
            resolved_from_ref: "project-config://toolchain/scaffold".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
            ],
            allowed_scope_labels: vec![
                "filesystem_root:workspace://project/templates=read_write".to_owned()
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
            capability_envelope_ref: "envelope:scaffold-hook:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:scaffold-hook:0001",
            "issuer:approval-broker:local",
            "ticket:scaffold-hook:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn ai_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:ai:0001".to_owned(),
        route: M5LaunchRoute::Ai,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::AiTool,
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            execution_backend: M5ExecutionBackendClass::LocalIsolated,
            platform: M5ExecutionPlatform::MacosDesktop,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: false,
            target_identity: "workspace://project".to_owned(),
            target_identity_verified: true,
            isolation_label: "Isolated local subprocess on this device.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Isolated AI tool runtime".to_owned(),
            backend_class: M5ExecutionBackendClass::LocalIsolated,
            selection_reason: M5ToolchainSelectionReason::PolicyMandatedRuntime,
            resolved_from_ref: "policy-epoch://m5/0007/ai-tool".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            allowed_scope_labels: vec![
                "filesystem_root:workspace://project/src=read_write".to_owned(),
                "network_endpoint:broker://transport-plane=send_only".to_owned(),
            ],
            secret_scope: M5SecretScope::HandleOnlyDelegated,
            capability_envelope_ref: "envelope:ai-tool:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:ai-tool:0001",
            "issuer:approval-broker:local",
            "ticket:ai-tool:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn recipe_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:recipe:0001".to_owned(),
        route: M5LaunchRoute::Recipe,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::Recipe,
            sandbox_profile: M5SandboxProfile::SubprocessIsolatedLocal,
            execution_backend: M5ExecutionBackendClass::LocalIsolated,
            platform: M5ExecutionPlatform::LinuxDesktop,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: false,
            target_identity: "workspace://project".to_owned(),
            target_identity_verified: true,
            isolation_label: "Isolated local subprocess on this device.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Saved recipe runner (pinned)".to_owned(),
            backend_class: M5ExecutionBackendClass::LocalIsolated,
            selection_reason: M5ToolchainSelectionReason::ProjectPinnedToolchain,
            resolved_from_ref: "recipe://nightly-format".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::ReadWorkspace,
                M5CapabilityClass::WriteWorkspace,
                M5CapabilityClass::ProcessSpawn,
            ],
            allowed_scope_labels: vec![
                "filesystem_root:workspace://project/src=read_write".to_owned()
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
            capability_envelope_ref: "envelope:recipe:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerScope,
            "ticket:recipe:0001",
            "issuer:approval-broker:local",
            "ticket:recipe:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn extension_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:extension:0001".to_owned(),
        route: M5LaunchRoute::Extension,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::RequestApiSend,
            sandbox_profile: M5SandboxProfile::BrokeredNetworkOnly,
            execution_backend: M5ExecutionBackendClass::BrokeredNetwork,
            platform: M5ExecutionPlatform::MacosDesktop,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: false,
            target_identity: "https://api.example.test/v1/orders".to_owned(),
            target_identity_verified: true,
            isolation_label: "Network egress brokered through the transport plane.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Brokered HTTPS transport".to_owned(),
            backend_class: M5ExecutionBackendClass::BrokeredNetwork,
            selection_reason: M5ToolchainSelectionReason::RemoteBrokeredRuntime,
            resolved_from_ref: "extension://request-sender/manifest".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            allowed_scope_labels: vec![
                "network_endpoint:https://api.example.test/v1=send_only".to_owned()
            ],
            secret_scope: M5SecretScope::HandleOnlyDelegated,
            capability_envelope_ref: "envelope:request-api-send:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerScope,
            "ticket:request-api-send:0001",
            "issuer:approval-broker:local",
            "ticket:request-api-send:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn remote_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:remote:0001".to_owned(),
        route: M5LaunchRoute::Remote,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::RemoteMutation,
            sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
            execution_backend: M5ExecutionBackendClass::RemoteIsolated,
            platform: M5ExecutionPlatform::ManagedRemoteRuntime,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: true,
            target_identity: "remote://managed-runtime/deployment".to_owned(),
            target_identity_verified: true,
            isolation_label: "Isolated remote runtime confined to a managed sandbox.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Managed remote runtime".to_owned(),
            backend_class: M5ExecutionBackendClass::RemoteIsolated,
            selection_reason: M5ToolchainSelectionReason::RemoteBrokeredRuntime,
            resolved_from_ref: "remote-broker://managed-runtime".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::RemoteMutation,
                M5CapabilityClass::NetworkEgress,
                M5CapabilityClass::SecretHandleProjection,
            ],
            allowed_scope_labels: vec![
                "data_sink:remote://managed-runtime/deployment=read_write".to_owned(),
                "network_endpoint:https://managed-runtime.example.test/mutation=send_only"
                    .to_owned(),
            ],
            secret_scope: M5SecretScope::ScopedBrokeredSecret,
            capability_envelope_ref: "envelope:remote-mutation:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:remote-mutation:0001",
            "issuer:remote-broker:managed",
            "ticket:remote-mutation:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn companion_sheet() -> M5LaunchExplainSheet {
    M5LaunchExplainSheet {
        sheet_id: "launch-explain:companion:0001".to_owned(),
        route: M5LaunchRoute::Companion,
        status: M5ExplainStatus::Complete,
        where_it_runs: M5WhereItRuns {
            executing_surface: M5ExecutingSurface::BrowserRoutedAction,
            sandbox_profile: M5SandboxProfile::IsolatedRemoteRuntime,
            execution_backend: M5ExecutionBackendClass::RemoteIsolated,
            platform: M5ExecutionPlatform::ManagedRemoteRuntime,
            profile_resolution_status: M5ProfileResolutionStatus::Supported,
            off_device: true,
            target_identity: "https://app.example.test".to_owned(),
            target_identity_verified: true,
            isolation_label: "Isolated remote runtime routed from the companion device.".to_owned(),
        },
        why_this_toolchain: M5WhyThisToolchain {
            toolchain_label: "Companion-paired remote browser runtime".to_owned(),
            backend_class: M5ExecutionBackendClass::RemoteIsolated,
            selection_reason: M5ToolchainSelectionReason::RemoteBrokeredRuntime,
            resolved_from_ref: "companion://paired-device/browser-route".to_owned(),
        },
        what_it_can_access: M5WhatItCanAccess {
            granted_capability_classes: vec![
                M5CapabilityClass::BrowserNavigation,
                M5CapabilityClass::NetworkEgress,
            ],
            allowed_scope_labels: vec![
                "network_endpoint:https://app.example.test=navigate".to_owned()
            ],
            secret_scope: M5SecretScope::NoSecretAccess,
            capability_envelope_ref: "envelope:browser-routed-action:0001".to_owned(),
        },
        who_approved_it: nominal_who(
            M5ApprovalTicketPosture::TicketRequiredPerAction,
            "ticket:browser-routed-action:0001",
            "issuer:approval-broker:local",
            "ticket:browser-routed-action:0001",
        ),
        degradation: None,
        applied_downgrade_triggers: vec![],
        unsupported_profile_behavior: None,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn validate_source_contracts(
    packet: &M5LaunchInspectorPacket,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LAUNCH_INSPECTOR_SCHEMA_REF,
        M5_LAUNCH_INSPECTOR_DOC_REF,
        M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
        M5_CAPABILITY_ENVELOPE_DOC_REF,
        M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
        M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF,
        M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LaunchInspectorViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_sheets(
    packet: &M5LaunchInspectorPacket,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();
    let allowed_by_surface: BTreeMap<M5ExecutingSurface, BTreeSet<M5CapabilityClass>> = matrix
        .surface_rows
        .iter()
        .map(|row| {
            (
                row.surface,
                row.allowed_capability_classes.iter().copied().collect(),
            )
        })
        .collect();
    let default_profile_by_surface: BTreeMap<M5ExecutingSurface, M5SandboxProfile> = matrix
        .surface_rows
        .iter()
        .map(|row| (row.surface, row.default_sandbox_profile))
        .collect();

    let envelope_surfaces: BTreeSet<M5ExecutingSurface> =
        frozen_stable_m5_capability_envelope_packet()
            .envelopes
            .iter()
            .map(|envelope| envelope.surface)
            .collect();

    let present: BTreeSet<M5LaunchRoute> = packet.sheets.iter().map(|sheet| sheet.route).collect();
    for required in M5LaunchRoute::ALL {
        if !present.contains(&required) {
            violations.push(M5LaunchInspectorViolation::RequiredRouteMissing);
            return;
        }
    }

    for sheet in &packet.sheets {
        validate_sheet_identity(sheet, violations);
        validate_sheet_answers(
            sheet,
            &allowed_by_surface,
            &default_profile_by_surface,
            violations,
        );
        validate_sheet_status(sheet, violations);

        let surface = sheet.surface();
        if !envelope_surfaces.contains(&surface) {
            violations.push(M5LaunchInspectorViolation::EnvelopeReferenceUncovered);
        }
    }
}

fn validate_sheet_identity(
    sheet: &M5LaunchExplainSheet,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    if sheet.sheet_id.trim().is_empty() || sheet.redaction_class_token.trim().is_empty() {
        violations.push(M5LaunchInspectorViolation::SheetIncomplete);
    }
}

fn validate_sheet_answers(
    sheet: &M5LaunchExplainSheet,
    allowed_by_surface: &BTreeMap<M5ExecutingSurface, BTreeSet<M5CapabilityClass>>,
    default_profile_by_surface: &BTreeMap<M5ExecutingSurface, M5SandboxProfile>,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    let where_it_runs = &sheet.where_it_runs;
    let what = &sheet.what_it_can_access;
    let who = &sheet.who_approved_it;

    if where_it_runs.target_identity.trim().is_empty()
        || where_it_runs.isolation_label.trim().is_empty()
    {
        violations.push(M5LaunchInspectorViolation::WhereAnswerIncomplete);
    }
    if sheet.why_this_toolchain.toolchain_label.trim().is_empty()
        || sheet.why_this_toolchain.resolved_from_ref.trim().is_empty()
    {
        violations.push(M5LaunchInspectorViolation::WhyAnswerIncomplete);
    }
    if what.granted_capability_classes.is_empty()
        || what.allowed_scope_labels.is_empty()
        || what
            .allowed_scope_labels
            .iter()
            .any(|label| label.trim().is_empty())
        || what.capability_envelope_ref.trim().is_empty()
    {
        violations.push(M5LaunchInspectorViolation::WhatAnswerIncomplete);
    }
    if who.issuer_label.trim().is_empty()
        || who.policy_epoch_label.trim().is_empty()
        || who.decision_chain.is_empty()
        || who.decision_chain.iter().any(|ref_| ref_.trim().is_empty())
    {
        violations.push(M5LaunchInspectorViolation::WhoAnswerIncomplete);
    }

    let surface = sheet.surface();
    if let Some(allowed) = allowed_by_surface.get(&surface) {
        if what
            .granted_capability_classes
            .iter()
            .any(|cap| !allowed.contains(cap))
        {
            violations.push(M5LaunchInspectorViolation::CapabilityWidensBeyondMatrix);
        }
    }
    if let Some(default_profile) = default_profile_by_surface.get(&surface) {
        if where_it_runs.sandbox_profile != *default_profile
            && where_it_runs.sandbox_profile != M5SandboxProfile::InertNoExecution
        {
            violations.push(M5LaunchInspectorViolation::SandboxProfileWidens);
        }
    }

    if sheet.route.is_helper_route() && who.self_issued_by_executor {
        violations.push(M5LaunchInspectorViolation::SelfIssuedAuthorityForbidden);
    }

    let has_elevated = what
        .granted_capability_classes
        .iter()
        .any(|cap| cap.is_elevated());
    if has_elevated && who.approval_ticket_ref.trim().is_empty() {
        violations.push(M5LaunchInspectorViolation::ElevatedCapabilityWithoutTicket);
    }

    let projects_secret = what
        .granted_capability_classes
        .iter()
        .any(|cap| cap.requires_secret_scope());
    let secret_consistent = if projects_secret {
        what.secret_scope.grants_secret_access()
    } else {
        !what.secret_scope.grants_secret_access()
    };
    if !secret_consistent {
        violations.push(M5LaunchInspectorViolation::SecretScopeInconsistent);
    }

    // Off-device routes must carry a verified target identity unless they are
    // explicitly degraded for an unverified identity.
    let degraded_for_identity = sheet
        .applied_downgrade_triggers
        .contains(&M5RuntimeAuthorityDowngradeTrigger::TargetIdentityUnverified);
    if where_it_runs.off_device && !where_it_runs.target_identity_verified && !degraded_for_identity
    {
        violations.push(M5LaunchInspectorViolation::OffDeviceTargetUnverified);
    }
}

fn validate_sheet_status(
    sheet: &M5LaunchExplainSheet,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    match sheet.status {
        M5ExplainStatus::Complete => {
            if sheet.degradation.is_some()
                || !sheet.applied_downgrade_triggers.is_empty()
                || sheet.where_it_runs.profile_resolution_status
                    != M5ProfileResolutionStatus::Supported
            {
                violations.push(M5LaunchInspectorViolation::StatusInconsistent);
            }
        }
        M5ExplainStatus::PartialDegraded | M5ExplainStatus::UnsupportedOnPlatform => {
            match &sheet.degradation {
                None => violations.push(M5LaunchInspectorViolation::DegradedReasonMissing),
                Some(degradation) => {
                    if degradation.explanation.trim().is_empty() {
                        violations.push(M5LaunchInspectorViolation::DegradedExplanationMissing);
                    }
                }
            }
        }
    }

    if sheet.status == M5ExplainStatus::UnsupportedOnPlatform
        && sheet.unsupported_profile_behavior.is_none()
    {
        violations.push(M5LaunchInspectorViolation::UnsupportedBehaviorMissing);
    }
}

fn validate_trust_review(
    packet: &M5LaunchInspectorPacket,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.no_ambient_machine_privilege,
        review.no_self_issued_authority_by_helpers,
        review.all_four_answers_present_even_when_degraded,
        review.degraded_state_reasons_explicit,
        review.access_never_widens_matrix,
        review.secret_refs_handle_only_no_raw_material,
        review.off_device_preserves_sheet_semantics,
        review.fail_closed_when_enforcement_unavailable,
        review.no_raw_secret_material_in_exports,
    ] {
        if !ok {
            violations.push(M5LaunchInspectorViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LaunchInspectorPacket,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_shows_explain_sheet,
        projection.command_and_policy_reference_same_sheets,
        projection.cli_headless_shows_explain_sheet,
        projection.support_export_shows_explain_sheet,
        projection.diagnostics_shows_explain_sheet,
        projection.help_about_shows_explain_summary,
        projection.release_evidence_consumes_sheets,
        projection.all_routes_project_same_facts,
    ] {
        if !ok {
            violations.push(M5LaunchInspectorViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LaunchInspectorPacket,
    violations: &mut Vec<M5LaunchInspectorViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LaunchInspectorViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
