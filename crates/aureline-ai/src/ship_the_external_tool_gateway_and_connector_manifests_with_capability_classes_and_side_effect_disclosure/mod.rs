//! External-tool gateway connector manifests with capability classes and
//! side-effect disclosure.
//!
//! This module ships the external-tool gateway into one export-safe truth packet
//! whose unit of truth is a [`ConnectorManifestRow`]: a single governed external
//! connector binding the capability classes it advertises, the side-effect
//! disclosures that say — for every effect it can produce — how the effect is
//! previewed, gated, audited, and reversed, and the provider/locality mode,
//! runtime boundary, network behavior, credential posture, and output-trust
//! posture it runs under. The packet is the canonical connector-manifest source
//! for shell, docs, support export, and release tooling; consumers project it
//! instead of re-deriving capability, side-effect, or trust state by hand.
//!
//! The packet refuses to present a connector greener than its disclosure posture
//! can back. Every connector must declare at least one capability class and a
//! side-effect disclosure for every effect it can produce. A mutating side effect
//! is held to the same preview, policy, and audit bar as a first-party command:
//! it must preview before it applies, carry a real approval gate, and be audited.
//! An irreversible external publish must be externally auditable, and a declared
//! reversibility must agree with the effect class. A connector whose output
//! crosses the network is tainted by default; a trusted-output posture is allowed
//! only on a local boundary published under a signed identity, and a local
//! boundary may never advertise a remote network behavior. A blocked connector —
//! policy-blocked, trust-blocked, quarantined, or withdrawn — narrows its claim
//! instead of staying behind a Stable, Beta, or Preview label, and a connector
//! still awaiting first-use review may not claim Stable. Every connector carries a
//! closed set of downgrade rules — including the proof-stale and
//! provider-unavailable triggers — that narrow the claim instead of hiding the
//! connector, reusing the qualification, downgrade-trigger, and rollback-posture
//! vocabularies frozen by the M5 AI workflow matrix lane, the mode vocabulary
//! frozen by the routing-policy lane, and the capability, side-effect, boundary,
//! network, credential, and output-trust vocabularies frozen by the tool-gateway
//! baseline, so no connector row may stay greener than its evidence.
//!
//! Raw endpoint URLs, raw spawn commands, credential bodies, raw API keys, OAuth
//! tokens, and raw request/response bodies stay outside the support boundary; the
//! packet carries modes, classes, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json`](../../../../schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json).
//! The contract doc is
//! [`docs/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure.md`](../../../../docs/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/`](../../../../fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::tool_gateway::{
    ToolApprovalPostureClass, ToolCapabilityClass, ToolCredentialPostureClass,
    ToolNetworkBehaviorClass, ToolOutputTrustPostureClass, ToolPublisherSourceClass,
    ToolRuntimeBoundaryClass, ToolSideEffectClass, TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`ConnectorManifestPacket`].
pub const CONNECTOR_MANIFEST_RECORD_KIND: &str =
    "ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure";

/// Schema version for connector-manifest records.
pub const CONNECTOR_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CONNECTOR_MANIFEST_SCHEMA_REF: &str =
    "schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json";

/// Repo-relative path of the connector-manifest contract doc.
pub const CONNECTOR_MANIFEST_DOC_REF: &str =
    "docs/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure.md";

/// Repo-relative path of the protected fixture directory.
pub const CONNECTOR_MANIFEST_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure";

/// Repo-relative path of the checked support-export artifact.
pub const CONNECTOR_MANIFEST_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CONNECTOR_MANIFEST_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure.md";

/// How a connector's side effect is previewed before it is applied.
///
/// A mutating side effect must preview before it applies, matching the bar a
/// first-party command is held to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectPreviewClass {
    /// A full preview of the effect is shown and confirmed before it applies.
    PreviewRequiredBeforeApply,
    /// A reviewable diff of the effect is shown before it applies.
    DiffPreviewBeforeApply,
    /// A dry run of the effect is shown before it applies.
    DryRunPreviewBeforeApply,
    /// The effect is inspect-only, so no apply-time preview is needed.
    InspectOnlyNoPreviewNeeded,
    /// No preview is available, so the effect must block until one exists.
    PreviewUnavailableMustBlock,
}

impl SideEffectPreviewClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRequiredBeforeApply => "preview_required_before_apply",
            Self::DiffPreviewBeforeApply => "diff_preview_before_apply",
            Self::DryRunPreviewBeforeApply => "dry_run_preview_before_apply",
            Self::InspectOnlyNoPreviewNeeded => "inspect_only_no_preview_needed",
            Self::PreviewUnavailableMustBlock => "preview_unavailable_must_block",
        }
    }

    /// Whether this class previews the effect before it applies.
    pub const fn previews_before_apply(self) -> bool {
        matches!(
            self,
            Self::PreviewRequiredBeforeApply
                | Self::DiffPreviewBeforeApply
                | Self::DryRunPreviewBeforeApply
        )
    }
}

/// How a connector's side effect is recorded for later audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectAuditClass {
    /// The effect is audited into the shared evidence timeline.
    AuditedToEvidenceTimeline,
    /// The effect is audited into the support export.
    AuditedToSupportExport,
    /// The effect is audited into local history only.
    AuditedLocalHistoryOnly,
    /// The effect is not audited.
    NotAudited,
}

impl SideEffectAuditClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuditedToEvidenceTimeline => "audited_to_evidence_timeline",
            Self::AuditedToSupportExport => "audited_to_support_export",
            Self::AuditedLocalHistoryOnly => "audited_local_history_only",
            Self::NotAudited => "not_audited",
        }
    }

    /// Whether the effect is audited anywhere.
    pub const fn is_audited(self) -> bool {
        !matches!(self, Self::NotAudited)
    }

    /// Whether the effect is audited to a durable, exportable surface.
    pub const fn is_externally_auditable(self) -> bool {
        matches!(
            self,
            Self::AuditedToEvidenceTimeline | Self::AuditedToSupportExport
        )
    }
}

/// Reversibility of a connector's side effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectReversibilityClass {
    /// The effect produces no observable change.
    NoSideEffect,
    /// The effect can be reversed inside the workspace.
    ReversibleInWorkspace,
    /// The effect can be reversed by restoring a checkpoint.
    CheckpointReversible,
    /// The effect publishes externally and cannot be reversed.
    IrreversibleExternalPublish,
}

impl SideEffectReversibilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffect => "no_side_effect",
            Self::ReversibleInWorkspace => "reversible_in_workspace",
            Self::CheckpointReversible => "checkpoint_reversible",
            Self::IrreversibleExternalPublish => "irreversible_external_publish",
        }
    }
}

/// Operational state of a connector manifest at mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorManifestStateClass {
    /// Admitted and ready to dispatch.
    Admitted,
    /// Admitted but awaiting first-use review before any material run.
    PendingFirstUseReview,
    /// Blocked by policy.
    PolicyBlocked,
    /// Blocked by workspace trust.
    TrustBlocked,
    /// Quarantined because its signature could not be verified.
    QuarantinedSignature,
    /// Withdrawn from this gateway.
    Withdrawn,
}

impl ConnectorManifestStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::PendingFirstUseReview => "pending_first_use_review",
            Self::PolicyBlocked => "policy_blocked",
            Self::TrustBlocked => "trust_blocked",
            Self::QuarantinedSignature => "quarantined_signature",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Whether the state admits a new material invocation.
    pub const fn admits_invocation(self) -> bool {
        matches!(self, Self::Admitted)
    }

    /// Whether downstream surfaces must display a typed block reason.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::PolicyBlocked | Self::TrustBlocked | Self::QuarantinedSignature | Self::Withdrawn
        )
    }
}

/// One disclosed side effect a connector can produce.
///
/// Each disclosure binds the side-effect class to how it is previewed, gated,
/// audited, and reversed, so the connector can never produce an effect the
/// manifest did not disclose under first-party command rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectDisclosure {
    /// Side-effect class this disclosure covers.
    pub side_effect_class: ToolSideEffectClass,
    /// How the effect is previewed before it applies.
    pub preview: SideEffectPreviewClass,
    /// Approval gate required before the effect applies.
    pub approval_posture: ToolApprovalPostureClass,
    /// How the effect is audited.
    pub audit: SideEffectAuditClass,
    /// Reversibility of the effect.
    pub reversibility: SideEffectReversibilityClass,
    /// Review-safe disclosure label shown to the user before the effect applies.
    pub disclosure_label: String,
}

impl SideEffectDisclosure {
    /// Whether this disclosure covers a mutating effect held to the
    /// first-party command preview, policy, and audit bar.
    pub fn is_mutating(&self) -> bool {
        self.side_effect_class.requires_approval_gate()
    }

    /// Whether this disclosure carries a real approval gate.
    pub fn has_approval_gate(&self) -> bool {
        self.approval_posture.requires_approval_gate() || self.approval_posture.denies_dispatch()
    }
}

/// One downgrade rule that narrows a connector's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the connector narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One connector manifest binding capability classes and side-effect disclosure
/// for a governed external-tool connector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorManifestRow {
    /// Stable connector-manifest id.
    pub manifest_id: String,
    /// Human-readable connector label.
    pub connector_label: String,
    /// Connector family label.
    pub connector_family_label: String,
    /// Connector capability version.
    pub connector_capability_version: String,
    /// Opaque ref to the matching gateway descriptor, when one exists.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub descriptor_ref: String,
    /// Source/publisher class.
    pub publisher_source_class: ToolPublisherSourceClass,
    /// Opaque ref to the signed publisher identity record.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub publisher_identity_ref: String,
    /// Provider/locality mode the connector resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Runtime boundary class.
    pub runtime_boundary_class: ToolRuntimeBoundaryClass,
    /// Network behavior class.
    pub network_behavior_class: ToolNetworkBehaviorClass,
    /// Credential posture class.
    pub credential_posture_class: ToolCredentialPostureClass,
    /// Output trust posture class.
    pub output_trust_posture_class: ToolOutputTrustPostureClass,
    /// Operational state at mint time.
    pub state: ConnectorManifestStateClass,
    /// Qualification class claimed for this connector.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Capability classes the connector advertises.
    pub capability_classes: Vec<ToolCapabilityClass>,
    /// Disclosed side effects the connector can produce.
    pub side_effect_disclosures: Vec<SideEffectDisclosure>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<ConnectorDowngradeRule>,
    /// Rollback posture for a connector-policy change.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
    /// Review-safe explanation of the connector posture.
    pub explanation_label: String,
}

impl ConnectorManifestRow {
    /// Whether this connector carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Whether the connector's output bytes must be fenced by default.
    pub fn outputs_are_tainted_by_default(&self) -> bool {
        self.output_trust_posture_class.is_tainted_by_default()
    }

    /// Whether the connector advertises any mutating side effect.
    pub fn has_mutating_side_effect(&self) -> bool {
        self.side_effect_disclosures
            .iter()
            .any(SideEffectDisclosure::is_mutating)
    }

    /// Whether the connector advertises an irreversible external publish.
    pub fn has_irreversible_publish(&self) -> bool {
        self.side_effect_disclosures.iter().any(|disclosure| {
            disclosure.side_effect_class == ToolSideEffectClass::ExternalIrreversiblePublish
        })
    }

    /// The disclosure for `side_effect_class`, if present.
    pub fn disclosure(
        &self,
        side_effect_class: ToolSideEffectClass,
    ) -> Option<&SideEffectDisclosure> {
        self.side_effect_disclosures
            .iter()
            .find(|disclosure| disclosure.side_effect_class == side_effect_class)
    }

    /// Qualification this connector narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this connector.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Connector `{}`\n", self.manifest_id));
        out.push_str(&format!("- Connector: `{}`\n", self.connector_label));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Mode: `{}`\n", self.resolved_mode.as_str()));
        out.push_str(&format!("- State: `{}`\n", self.state.as_str()));
        out.push_str(&format!(
            "- Boundary: `{}` / network `{}` / credential `{}` / output `{}`\n",
            self.runtime_boundary_class.as_str(),
            self.network_behavior_class.as_str(),
            self.credential_posture_class.as_str(),
            self.output_trust_posture_class.as_str()
        ));
        out.push_str("- Capabilities:\n");
        for capability in &self.capability_classes {
            out.push_str(&format!("  - `{}`\n", capability.as_str()));
        }
        out.push_str("- Side-effect disclosures:\n");
        for disclosure in &self.side_effect_disclosures {
            out.push_str(&format!(
                "  - `{}` / preview `{}` / approval `{}` / audit `{}` / reversibility `{}` ({})\n",
                disclosure.side_effect_class.as_str(),
                disclosure.preview.as_str(),
                disclosure.approval_posture.as_str(),
                disclosure.audit.as_str(),
                disclosure.reversibility.as_str(),
                disclosure.disclosure_label
            ));
        }
        out
    }
}

/// Proof freshness block for the connector-manifest packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorManifestProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed connectors.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ConnectorManifestPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorManifestPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Connector manifest rows.
    pub connectors: Vec<ConnectorManifestRow>,
    /// Proof freshness block.
    pub proof_freshness: ConnectorManifestProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe external-tool connector-manifest packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorManifestPacket {
    /// Record kind; must equal [`CONNECTOR_MANIFEST_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CONNECTOR_MANIFEST_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Connector manifest rows.
    pub connectors: Vec<ConnectorManifestRow>,
    /// Proof freshness block.
    pub proof_freshness: ConnectorManifestProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ConnectorManifestPacket {
    /// Builds a connector-manifest packet from stable-lane input.
    pub fn new(input: ConnectorManifestPacketInput) -> Self {
        Self {
            record_kind: CONNECTOR_MANIFEST_RECORD_KIND.to_owned(),
            schema_version: CONNECTOR_MANIFEST_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            connectors: input.connectors,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the connector-manifest invariants.
    pub fn validate(&self) -> Vec<ConnectorManifestViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CONNECTOR_MANIFEST_RECORD_KIND {
            violations.push(ConnectorManifestViolation::WrongRecordKind);
        }
        if self.schema_version != CONNECTOR_MANIFEST_SCHEMA_VERSION {
            violations.push(ConnectorManifestViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ConnectorManifestViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_connectors_present(self, &mut violations);
        for connector in &self.connectors {
            validate_connector(connector, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("connector manifest packet serializes"),
        ) {
            violations.push(ConnectorManifestViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of connectors carrying a publicly claimed qualification.
    pub fn claimed_connector_count(&self) -> usize {
        self.connectors.iter().filter(|c| c.is_claimed()).count()
    }

    /// Count of connectors in a blocked state.
    pub fn blocked_connector_count(&self) -> usize {
        self.connectors
            .iter()
            .filter(|c| c.state.is_blocked())
            .count()
    }

    /// Count of connectors advertising a mutating side effect.
    pub fn mutating_connector_count(&self) -> usize {
        self.connectors
            .iter()
            .filter(|c| c.has_mutating_side_effect())
            .count()
    }

    /// Returns the connector row for `manifest_id`, if present.
    pub fn connector(&self, manifest_id: &str) -> Option<&ConnectorManifestRow> {
        self.connectors
            .iter()
            .find(|c| c.manifest_id == manifest_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("connector manifest packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# External-Tool Gateway Connector Manifests With Capability Classes And Side-Effect Disclosure\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Connectors: {} ({} claimed, {} mutating, {} blocked)\n",
            self.connectors.len(),
            self.claimed_connector_count(),
            self.mutating_connector_count(),
            self.blocked_connector_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Connector inspectors\n\n");
        for connector in &self.connectors {
            out.push_str(&connector.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in connector-manifest export.
#[derive(Debug)]
pub enum ConnectorManifestArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ConnectorManifestViolation>),
}

impl fmt::Display for ConnectorManifestArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "connector manifest export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "connector manifest export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ConnectorManifestArtifactError {}

/// Validation failures emitted by [`ConnectorManifestPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectorManifestViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no connectors.
    NoConnectors,
    /// A manifest id appears more than once.
    DuplicateConnector,
    /// A connector row is missing a required identity or label field.
    ConnectorRowIncomplete,
    /// A connector requires a publisher identity ref but none is set.
    ConnectorMissingPublisherIdentity,
    /// A connector advertises no capability classes.
    ConnectorMissingCapabilities,
    /// A connector discloses no side effects.
    ConnectorMissingSideEffectDisclosures,
    /// A side-effect disclosure is missing its disclosure label.
    SideEffectDisclosureIncomplete,
    /// A side-effect class is disclosed more than once.
    DuplicateSideEffectDisclosure,
    /// A mutating side effect does not preview before it applies.
    MutatingSideEffectWithoutPreview,
    /// A mutating side effect carries no approval gate.
    MutatingSideEffectWithoutApproval,
    /// A mutating side effect is not audited.
    MutatingSideEffectWithoutAudit,
    /// An irreversible external publish is not externally auditable.
    IrreversiblePublishNotExternallyAudited,
    /// A disclosure's reversibility disagrees with its side-effect class.
    SideEffectReversibilityMismatch,
    /// A trusted-output connector does not have a local runtime boundary.
    TrustedOutputRequiresLocalBoundary,
    /// A trusted-output connector does not have a signed publisher identity.
    TrustedOutputRequiresSignedPublisher,
    /// A local connector advertises a remote network behavior.
    LocalConnectorAdvertisesRemoteNetwork,
    /// A connector whose output crosses the network is not tainted by default.
    RemoteConnectorOutputNotTainted,
    /// A blocked connector still claims a public qualification.
    BlockedConnectorClaimsQualification,
    /// A connector pending first-use review claims Stable.
    PendingReviewClaimsStable,
    /// A claimed connector is missing required evidence packet refs.
    ClaimedConnectorMissingEvidence,
    /// A claimed connector's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// A connector has no downgrade rules.
    DowngradeRulesMissing,
    /// A connector's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A connector's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ConnectorManifestViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoConnectors => "no_connectors",
            Self::DuplicateConnector => "duplicate_connector",
            Self::ConnectorRowIncomplete => "connector_row_incomplete",
            Self::ConnectorMissingPublisherIdentity => "connector_missing_publisher_identity",
            Self::ConnectorMissingCapabilities => "connector_missing_capabilities",
            Self::ConnectorMissingSideEffectDisclosures => {
                "connector_missing_side_effect_disclosures"
            }
            Self::SideEffectDisclosureIncomplete => "side_effect_disclosure_incomplete",
            Self::DuplicateSideEffectDisclosure => "duplicate_side_effect_disclosure",
            Self::MutatingSideEffectWithoutPreview => "mutating_side_effect_without_preview",
            Self::MutatingSideEffectWithoutApproval => "mutating_side_effect_without_approval",
            Self::MutatingSideEffectWithoutAudit => "mutating_side_effect_without_audit",
            Self::IrreversiblePublishNotExternallyAudited => {
                "irreversible_publish_not_externally_audited"
            }
            Self::SideEffectReversibilityMismatch => "side_effect_reversibility_mismatch",
            Self::TrustedOutputRequiresLocalBoundary => "trusted_output_requires_local_boundary",
            Self::TrustedOutputRequiresSignedPublisher => {
                "trusted_output_requires_signed_publisher"
            }
            Self::LocalConnectorAdvertisesRemoteNetwork => {
                "local_connector_advertises_remote_network"
            }
            Self::RemoteConnectorOutputNotTainted => "remote_connector_output_not_tainted",
            Self::BlockedConnectorClaimsQualification => "blocked_connector_claims_qualification",
            Self::PendingReviewClaimsStable => "pending_review_claims_stable",
            Self::ClaimedConnectorMissingEvidence => "claimed_connector_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingProviderUnavailable => {
                "downgrade_rule_missing_provider_unavailable"
            }
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in connector-manifest export.
pub fn current_connector_manifest_export(
) -> Result<ConnectorManifestPacket, ConnectorManifestArtifactError> {
    let packet: ConnectorManifestPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/support_export.json"
    )))
    .map_err(ConnectorManifestArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ConnectorManifestArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

fn validate_source_contracts(
    packet: &ConnectorManifestPacket,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CONNECTOR_MANIFEST_SCHEMA_REF,
        CONNECTOR_MANIFEST_DOC_REF,
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        ROUTING_POLICY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ConnectorManifestViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_connectors_present(
    packet: &ConnectorManifestPacket,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    if packet.connectors.is_empty() {
        violations.push(ConnectorManifestViolation::NoConnectors);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for connector in &packet.connectors {
        if !seen.insert(connector.manifest_id.as_str()) {
            violations.push(ConnectorManifestViolation::DuplicateConnector);
        }
    }
}

fn validate_connector(
    connector: &ConnectorManifestRow,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    if connector.manifest_id.trim().is_empty()
        || connector.connector_label.trim().is_empty()
        || connector.connector_family_label.trim().is_empty()
        || connector.connector_capability_version.trim().is_empty()
        || connector.explanation_label.trim().is_empty()
    {
        violations.push(ConnectorManifestViolation::ConnectorRowIncomplete);
    }

    if connector
        .publisher_source_class
        .requires_publisher_identity()
        && connector.publisher_identity_ref.trim().is_empty()
    {
        violations.push(ConnectorManifestViolation::ConnectorMissingPublisherIdentity);
    }

    if connector.capability_classes.is_empty() {
        violations.push(ConnectorManifestViolation::ConnectorMissingCapabilities);
    }

    validate_side_effect_disclosures(connector, violations);
    validate_boundary_posture(connector, violations);
    validate_claim_state(connector, violations);
    validate_downgrade_rules(connector, violations);
}

fn validate_side_effect_disclosures(
    connector: &ConnectorManifestRow,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    if connector.side_effect_disclosures.is_empty() {
        violations.push(ConnectorManifestViolation::ConnectorMissingSideEffectDisclosures);
        return;
    }

    let mut seen: BTreeSet<ToolSideEffectClass> = BTreeSet::new();
    for disclosure in &connector.side_effect_disclosures {
        if !seen.insert(disclosure.side_effect_class) {
            violations.push(ConnectorManifestViolation::DuplicateSideEffectDisclosure);
        }

        if disclosure.disclosure_label.trim().is_empty() {
            violations.push(ConnectorManifestViolation::SideEffectDisclosureIncomplete);
        }

        // A mutating connector side effect is held to the same preview, policy,
        // and audit bar as a first-party command.
        if disclosure.is_mutating() {
            if !disclosure.preview.previews_before_apply() {
                violations.push(ConnectorManifestViolation::MutatingSideEffectWithoutPreview);
            }
            if !disclosure.has_approval_gate() {
                violations.push(ConnectorManifestViolation::MutatingSideEffectWithoutApproval);
            }
            if !disclosure.audit.is_audited() {
                violations.push(ConnectorManifestViolation::MutatingSideEffectWithoutAudit);
            }
        }

        // An irreversible external publish must be durably, exportably audited.
        if disclosure.side_effect_class == ToolSideEffectClass::ExternalIrreversiblePublish
            && !disclosure.audit.is_externally_auditable()
        {
            violations.push(ConnectorManifestViolation::IrreversiblePublishNotExternallyAudited);
        }

        // The disclosed reversibility must agree with the effect class for the
        // two unambiguous cases.
        let reversibility_ok = match disclosure.side_effect_class {
            ToolSideEffectClass::InspectOnly => {
                disclosure.reversibility == SideEffectReversibilityClass::NoSideEffect
            }
            ToolSideEffectClass::ExternalIrreversiblePublish => {
                disclosure.reversibility
                    == SideEffectReversibilityClass::IrreversibleExternalPublish
            }
            _ => true,
        };
        if !reversibility_ok {
            violations.push(ConnectorManifestViolation::SideEffectReversibilityMismatch);
        }
    }
}

fn validate_boundary_posture(
    connector: &ConnectorManifestRow,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    // A trusted-output posture is only credible on a signed local boundary.
    if !connector.output_trust_posture_class.is_tainted_by_default() {
        if !connector.runtime_boundary_class.is_local() {
            violations.push(ConnectorManifestViolation::TrustedOutputRequiresLocalBoundary);
        }
        if connector.publisher_identity_ref.trim().is_empty() {
            violations.push(ConnectorManifestViolation::TrustedOutputRequiresSignedPublisher);
        }
    }

    // A local boundary may never advertise a remote network behavior.
    if connector.runtime_boundary_class.is_local() && connector.network_behavior_class.is_remote() {
        violations.push(ConnectorManifestViolation::LocalConnectorAdvertisesRemoteNetwork);
    }

    // A connector whose output crosses the network is tainted by default.
    if connector.network_behavior_class.is_remote()
        && !connector.output_trust_posture_class.is_tainted_by_default()
    {
        violations.push(ConnectorManifestViolation::RemoteConnectorOutputNotTainted);
    }
}

fn validate_claim_state(
    connector: &ConnectorManifestRow,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    // A blocked connector narrows its claim instead of staying behind a public
    // qualification.
    if connector.is_claimed() && connector.state.is_blocked() {
        violations.push(ConnectorManifestViolation::BlockedConnectorClaimsQualification);
    }

    // A connector still awaiting first-use review may not claim Stable.
    if connector.state == ConnectorManifestStateClass::PendingFirstUseReview
        && connector.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(ConnectorManifestViolation::PendingReviewClaimsStable);
    }

    if connector.is_claimed() && connector.evidence_packet_refs.is_empty() {
        violations.push(ConnectorManifestViolation::ClaimedConnectorMissingEvidence);
    }

    // A claimed connector whose connector-policy change can be reversed must have
    // drilled that reversal; a non-applicable posture carries no reversal.
    if connector.is_claimed()
        && connector.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !connector.rollback_verified
    {
        violations.push(ConnectorManifestViolation::ClaimedRollbackUnverified);
    }
}

fn validate_downgrade_rules(
    connector: &ConnectorManifestRow,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    if connector.downgrade_rules.is_empty() {
        violations.push(ConnectorManifestViolation::DowngradeRulesMissing);
        return;
    }

    if !connector
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(ConnectorManifestViolation::DowngradeRuleMissingProofStale);
    }

    // Provider outages and quota exhaustion narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !connector
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(ConnectorManifestViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(connector.claimed_qualification);
    for rule in &connector.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(ConnectorManifestViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &ConnectorManifestPacket,
    violations: &mut Vec<ConnectorManifestViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ConnectorManifestViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// Mirrors the tool-gateway baseline: the connector vocabulary itself carries
/// tokens such as `byok_secret_broker`, so the check rejects raw transport and
/// credential material rather than bare substrings like `secret`.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key=")
        || lower.contains("api-key=")
        || lower.contains("raw_api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
}
