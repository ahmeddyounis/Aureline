//! Published supportability runbooks, field playbooks, and incident/advisory
//! packet integration for the stable line.
//!
//! This module defines the canonical M4 stable-line contracts for runbook
//! source classes, executable step envelopes, deviation notes, field-playbook
//! packets, and incident/advisory integration packets. Every record carries
//! closed vocabulary, exact-build identity joins, explicit browser or console
//! handoff metadata, and a metadata-safe support projection.
//!
//! The [`SupportabilityRunbookCatalog`] mirrors the boundary schema at
//! [`/schemas/support/publish_supportability_runbooks_field_playbooks_and_incident_advisory.schema.json`].
//!
//! ## Runbook source classes
//!
//! - [`RunbookSourceClass::RepoLocal`] — versioned runbook checked into the
//!   workspace or repo; claims executable authority when current.
//! - [`RunbookSourceClass::ReviewedDocsPack`] — reviewed docs pack with signer
//!   and freshness metadata; authoritative within its compatibility window.
//! - [`RunbookSourceClass::ManagedCatalog`] — managed enterprise catalog entry
//!   with auditable versioning and approver policy.
//! - [`RunbookSourceClass::BrowserOnlyVendorDocs`] — vendor documentation
//!   reachable only through a browser; reference-only, never claims execution.
//!
//! ## Step envelope contract
//!
//! Every [`RunbookStepEnvelope`] carries a stable step id, closed step class,
//! target-selector scope, approval requirement, expected evidence outputs, and
//! explicit handoff metadata so execution cannot widen authority implicitly.
//!
//! ## Deviation notes
//!
//! [`DeviationNote`] records are emitted whenever an operator departs from the
//! declared step sequence. They survive export and later investigation as
//! first-class metadata.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Record kinds and stable refs
// ---------------------------------------------------------------------------

/// Stable record-kind tag for a supportability runbook catalog entry.
pub const SUPPORTABILITY_RUNBOOK_CATALOG_ENTRY_RECORD_KIND: &str =
    "supportability_runbook_catalog_entry_record";

/// Stable record-kind tag for the published supportability runbook catalog.
pub const SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND: &str =
    "supportability_runbook_catalog_record";

/// Stable record-kind tag for a field-playbook packet.
pub const FIELD_PLAYBOOK_PACKET_RECORD_KIND: &str = "field_playbook_packet_record";

/// Stable record-kind tag for an incident/advisory integration packet.
pub const INCIDENT_ADVISORY_PACKET_RECORD_KIND: &str = "incident_advisory_packet_record";

/// Stable record-kind tag for a deviation note.
pub const DEVIATION_NOTE_RECORD_KIND: &str = "runbook_deviation_note_record";

/// Integer schema version for published supportability records.
pub const PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PUBLISHED_SUPPORTABILITY_SCHEMA_REF: &str =
    "schemas/support/publish_supportability_runbooks_field_playbooks_and_incident_advisory.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const PUBLISHED_SUPPORTABILITY_DOC_REF: &str =
    "docs/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PUBLISHED_SUPPORTABILITY_ARTIFACT_REF: &str =
    "artifacts/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PUBLISHED_SUPPORTABILITY_FIXTURE_DIR: &str =
    "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory";

/// Repo-relative path of the protected fixture manifest.
pub const PUBLISHED_SUPPORTABILITY_FIXTURE_MANIFEST_REF: &str =
    "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/manifest.yaml";

// ---------------------------------------------------------------------------
// Fixture embeds
// ---------------------------------------------------------------------------

const FIXTURE_SOURCES: &[(&str, &str)] = &[
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/repo_local_crash_recovery_runbook.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/repo_local_crash_recovery_runbook.yaml"
        )),
    ),
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/reviewed_docs_pack_safe_mode_entry.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/reviewed_docs_pack_safe_mode_entry.yaml"
        )),
    ),
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/managed_catalog_enterprise_policy_repair.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/managed_catalog_enterprise_policy_repair.yaml"
        )),
    ),
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/browser_only_vendor_docs_handoff.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/browser_only_vendor_docs_handoff.yaml"
        )),
    ),
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/incident_workspace_with_deviation.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/incident_workspace_with_deviation.yaml"
        )),
    ),
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/advisory_packet_known_limit.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/advisory_packet_known_limit.yaml"
        )),
    ),
    (
        "fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/field_playbook_extension_bisect.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/publish-supportability-runbooks-field-playbooks-and-incident-advisory/field_playbook_extension_bisect.yaml"
        )),
    ),
];

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Runbook source class with explicit authoritative posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceClass {
    /// Versioned runbook checked into the workspace or repo.
    RepoLocal,
    /// Reviewed docs pack with signer and freshness metadata.
    ReviewedDocsPack,
    /// Managed enterprise catalog with auditable versioning.
    ManagedCatalog,
    /// Vendor documentation reachable only through a browser.
    BrowserOnlyVendorDocs,
}

impl RunbookSourceClass {
    /// Every required runbook source class, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::RepoLocal,
        Self::ReviewedDocsPack,
        Self::ManagedCatalog,
        Self::BrowserOnlyVendorDocs,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepoLocal => "repo_local",
            Self::ReviewedDocsPack => "reviewed_docs_pack",
            Self::ManagedCatalog => "managed_catalog",
            Self::BrowserOnlyVendorDocs => "browser_only_vendor_docs",
        }
    }

    /// Returns true when the source may not claim executable authority.
    pub const fn is_reference_only(self) -> bool {
        matches!(self, Self::BrowserOnlyVendorDocs)
    }
}

/// Authoritative posture for a runbook or playbook source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritativePosture {
    /// The source is authoritative for in-product execution.
    Authoritative,
    /// The source is reference-only and does not claim execution authority.
    ReferenceOnly,
    /// Authority is delegated to a managed administrator.
    ManagedAdmin,
    /// Authority cannot be verified; user review is required.
    Unverifiable,
}

impl AuthoritativePosture {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::ReferenceOnly => "reference_only",
            Self::ManagedAdmin => "managed_admin",
            Self::Unverifiable => "unverifiable",
        }
    }
}

/// Closed runbook step-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepClass {
    /// Read-only evidence gathering.
    Observe,
    /// Read-only validation of scope, health, or expected behavior.
    Verify,
    /// Protected target mutation or mitigation.
    Mitigate,
    /// Rollback, restore, or compensating action.
    Rollback,
    /// External or internal communication step.
    Communicate,
}

impl StepClass {
    /// Every required step class, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::Observe,
        Self::Verify,
        Self::Mitigate,
        Self::Rollback,
        Self::Communicate,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Verify => "verify",
            Self::Mitigate => "mitigate",
            Self::Rollback => "rollback",
            Self::Communicate => "communicate",
        }
    }

    /// Returns true for steps that can change protected state.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }
}

/// Closed target-selector scope vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetSelectorScope {
    /// Local workspace target.
    LocalWorkspace,
    /// Runtime target selected by execution context.
    RuntimeTarget,
    /// Environment-scoped target such as region or namespace.
    EnvironmentScope,
    /// Service or resource-level target.
    ServiceResource,
    /// External browser or console target.
    BrowserConsoleExternal,
    /// Selector could not be resolved and requires review.
    UnresolvedRequiresReview,
}

impl TargetSelectorScope {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RuntimeTarget => "runtime_target",
            Self::EnvironmentScope => "environment_scope",
            Self::ServiceResource => "service_resource",
            Self::BrowserConsoleExternal => "browser_console_external",
            Self::UnresolvedRequiresReview => "unresolved_requires_review",
        }
    }

    /// Returns true when the target identity is not resolved in-product.
    pub const fn requires_external_handoff(self) -> bool {
        matches!(
            self,
            Self::BrowserConsoleExternal | Self::UnresolvedRequiresReview
        )
    }
}

/// Approval requirement class for a runbook step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequirementClass {
    /// No approval required.
    NoApprovalRequired,
    /// Runtime approval ticket required.
    RuntimeApprovalTicket,
    /// Two-person approval required.
    TwoPersonApproval,
    /// Policy grant required.
    PolicyGrant,
    /// Break-glass approval required.
    BreakGlass,
    /// Approval is forbidden for this step.
    ApprovalForbidden,
}

impl ApprovalRequirementClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalRequired => "no_approval_required",
            Self::RuntimeApprovalTicket => "runtime_approval_ticket",
            Self::TwoPersonApproval => "two_person_approval",
            Self::PolicyGrant => "policy_grant",
            Self::BreakGlass => "break_glass",
            Self::ApprovalForbidden => "approval_forbidden",
        }
    }
}

/// Expected evidence output class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOutputClass {
    /// Command result packet reference.
    CommandResultPacketRef,
    /// Runbook step result reference.
    RunbookStepResultRef,
    /// Incident timeline entry reference.
    IncidentTimelineEntryRef,
    /// Support bundle item reference.
    SupportBundleItemRef,
    /// Support packet index reference.
    SupportPacketIndexRef,
    /// Approval ticket reference.
    ApprovalTicketRef,
    /// Rollback handle reference.
    RollbackHandleRef,
    /// Provider callback reference.
    ProviderCallbackRef,
    /// Action ledger entry reference.
    ActionLedgerEntryRef,
    /// Deviation note reference.
    DeviationNoteRef,
}

impl EvidenceOutputClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandResultPacketRef => "command_result_packet_ref",
            Self::RunbookStepResultRef => "runbook_step_result_ref",
            Self::IncidentTimelineEntryRef => "incident_timeline_entry_ref",
            Self::SupportBundleItemRef => "support_bundle_item_ref",
            Self::SupportPacketIndexRef => "support_packet_index_ref",
            Self::ApprovalTicketRef => "approval_ticket_ref",
            Self::RollbackHandleRef => "rollback_handle_ref",
            Self::ProviderCallbackRef => "provider_callback_ref",
            Self::ActionLedgerEntryRef => "action_ledger_entry_ref",
            Self::DeviationNoteRef => "deviation_note_ref",
        }
    }
}

/// Console or browser handoff kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffKind {
    /// Browser-based vendor documentation.
    BrowserDocs,
    /// Vendor console mutation required.
    VendorConsole,
    /// Provider approval workflow.
    ProviderApproval,
    /// External control plane operation.
    ExternalControlPlane,
}

impl HandoffKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserDocs => "browser_docs",
            Self::VendorConsole => "vendor_console",
            Self::ProviderApproval => "provider_approval",
            Self::ExternalControlPlane => "external_control_plane",
        }
    }
}

/// Handoff reason class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReasonClass {
    /// Documentation is browser-only.
    BrowserOnlyVendorDocs,
    /// Provider console is the only available path.
    ProviderConsoleOnly,
    /// In-product action is unsupported for this target.
    InProductActionUnsupported,
    /// Policy requires external approval.
    PolicyRequiresExternalApproval,
    /// Target identity cannot be verified in-product.
    TargetUnverifiable,
}

impl HandoffReasonClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserOnlyVendorDocs => "browser_only_vendor_docs",
            Self::ProviderConsoleOnly => "provider_console_only",
            Self::InProductActionUnsupported => "in_product_action_unsupported",
            Self::PolicyRequiresExternalApproval => "policy_requires_external_approval",
            Self::TargetUnverifiable => "target_unverifiable",
        }
    }
}

/// Runbook document freshness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFreshnessState {
    /// The runbook version is current.
    ExactCurrent,
    /// The runbook is within an accepted grace window.
    WarmWithinGrace,
    /// The runbook is stale and requires review before mutation.
    StaleRequiresReview,
    /// A newer runbook supersedes this packet.
    SupersededRequiresUpgrade,
    /// The required runbook source is missing.
    MissingRequiresBlock,
    /// The source is browser-only reference material.
    BrowserOnlyReference,
}

impl DocsFreshnessState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCurrent => "exact_current",
            Self::WarmWithinGrace => "warm_within_grace",
            Self::StaleRequiresReview => "stale_requires_review",
            Self::SupersededRequiresUpgrade => "superseded_requires_upgrade",
            Self::MissingRequiresBlock => "missing_requires_block",
            Self::BrowserOnlyReference => "browser_only_reference",
        }
    }

    /// Returns true when live mutation can be considered after other gates pass.
    pub const fn allows_live_mutation(self) -> bool {
        matches!(self, Self::ExactCurrent | Self::WarmWithinGrace)
    }
}

/// Export-right class for a runbook source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRightClass {
    /// May be exported as metadata-safe reference.
    MetadataSafeExport,
    /// May be exported with redaction profile applied.
    RedactedExport,
    /// May be referenced but not embedded in export.
    ReferenceOnly,
    /// Export is forbidden for this source.
    ExportForbidden,
}

impl ExportRightClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::RedactedExport => "redacted_export",
            Self::ReferenceOnly => "reference_only",
            Self::ExportForbidden => "export_forbidden",
        }
    }
}

/// Redaction class for support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default redaction posture.
    MetadataSafeDefault,
    /// Restricted to operator-only viewing.
    OperatorOnlyRestricted,
    /// Restricted to internal support channels.
    InternalSupportRestricted,
    /// Signing evidence only.
    SigningEvidenceOnly,
    /// Private triage only.
    PrivateTriageOnly,
}

impl RedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
            Self::PrivateTriageOnly => "private_triage_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Core structs
// ---------------------------------------------------------------------------

/// One expected evidence output for a runbook step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedEvidenceOutput {
    /// Stable evidence output id.
    pub evidence_output_id: String,
    /// Closed evidence class.
    pub evidence_class: EvidenceOutputClass,
    /// Whether this evidence is required for step completion.
    pub required_for_completion: bool,
    /// Redaction state for this evidence on export.
    pub redaction_state: String,
}

/// Explicit browser or console handoff metadata for a step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffMetadata {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Closed handoff kind.
    pub handoff_kind: HandoffKind,
    /// Target console URI class (not the raw URI).
    pub target_console_uri_class: String,
    /// Target context reference.
    pub target_context_ref: String,
    /// Closed handoff reason class.
    pub reason_class: HandoffReasonClass,
    /// Whether in-product mutation is supported for this target.
    pub in_product_mutation_supported: bool,
    /// Whether the external control plane is authoritative.
    pub external_control_plane_authoritative: bool,
    /// Return anchor reference for handoff completion.
    pub return_anchor_ref: Option<String>,
}

/// Executable step envelope with stable identity and bounded authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepEnvelope {
    /// Stable step id.
    pub step_id: String,
    /// Step ordinal within the runbook.
    pub ordinal: u32,
    /// Closed step class.
    pub step_class: StepClass,
    /// Reviewer-facing step title.
    pub title: String,
    /// Reviewer-facing intent summary.
    pub intent_summary: String,
    /// Closed target-selector scope.
    pub target_selector_scope: TargetSelectorScope,
    /// Target identity reference, if resolved.
    pub target_identity_ref: Option<String>,
    /// Closed approval requirement class.
    pub approval_requirement: ApprovalRequirementClass,
    /// Approval ticket reference, if required.
    pub approval_ticket_ref: Option<String>,
    /// Expected evidence outputs for this step.
    pub expected_evidence_outputs: Vec<ExpectedEvidenceOutput>,
    /// Explicit handoff metadata, if the step pivots external.
    pub handoff_metadata: Option<HandoffMetadata>,
    /// Whether a deviation note is required when departing from this step.
    pub deviation_note_required: bool,
    /// Rollback step reference, if applicable.
    pub rollback_step_ref: Option<String>,
}

/// Durable deviation note for a departed step sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviationNote {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable deviation note id.
    pub deviation_note_id: String,
    /// Actor reference (opaque).
    pub actor_ref: String,
    /// Reason class token.
    pub reason_class: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Evidence refs supporting the deviation.
    pub evidence_refs: Vec<String>,
    /// Step id that was deviated from.
    pub departed_step_id: String,
    /// Runbook packet id where the deviation occurred.
    pub runbook_packet_id: String,
    /// UTC timestamp when the deviation was recorded.
    pub created_at: String,
}

/// Source document metadata with signer, freshness, and export rights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDocumentBlock {
    /// Source reference (opaque id or path class).
    pub source_ref: String,
    /// Source revision or version.
    pub source_revision: String,
    /// Closed runbook source class.
    pub source_class: RunbookSourceClass,
    /// Closed authoritative posture.
    pub authoritative_posture: AuthoritativePosture,
    /// Closed docs freshness state.
    pub docs_freshness_state: DocsFreshnessState,
    /// Signer or source owner reference.
    pub signer_or_source_ref: String,
    /// Reviewer-facing approver policy summary.
    pub approver_policy_summary: String,
    /// Closed export-right class.
    pub export_right: ExportRightClass,
}

/// Owner block for a runbook or playbook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerBlock {
    /// Owner reference.
    pub owner_ref: String,
    /// Escalation owner reference.
    pub escalation_owner_ref: String,
    /// Backup owner reference, if any.
    pub backup_owner_ref: Option<String>,
    /// Review cadence token.
    pub review_cadence: String,
}

/// Compatibility window for a runbook packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityWindow {
    /// Version floor reference.
    pub version_floor_ref: String,
    /// Version ceiling reference.
    pub version_ceiling_ref: String,
    /// Valid-from timestamp.
    pub valid_from: String,
    /// Valid-until timestamp, if bounded.
    pub valid_until: Option<String>,
    /// Expiry behavior token.
    pub expiry_behavior: String,
}

/// Field-playbook packet for stable-line operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldPlaybookPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Runbook id this packet belongs to.
    pub runbook_id: String,
    /// Packet version string.
    pub packet_version: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Owner block.
    pub owner: OwnerBlock,
    /// Source document metadata.
    pub source_document: SourceDocumentBlock,
    /// Compatibility window.
    pub compatibility_window: CompatibilityWindow,
    /// Supported target selector scopes.
    pub supported_target_scopes: Vec<TargetSelectorScope>,
    /// Step envelopes.
    pub steps: Vec<RunbookStepEnvelope>,
    /// Default redaction class.
    pub redaction_class: RedactionClass,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// UTC authored timestamp.
    pub authored_at: String,
    /// UTC last-reviewed timestamp.
    pub last_reviewed_at: String,
}

impl FieldPlaybookPacket {
    /// Returns true when the packet is metadata-safe and claims no hidden
    /// authority.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.steps.is_empty()
            && self.steps.iter().all(|step| {
                // Mutating steps require explicit approval and target identity.
                if step.step_class.is_mutating() {
                    step.approval_ticket_ref.is_some()
                        || step.approval_requirement == ApprovalRequirementClass::NoApprovalRequired
                } else {
                    true
                }
            })
    }

    /// Returns true when every mutating step has an approval ticket ref or
    /// explicitly declares no approval required.
    pub fn mutating_steps_have_approval_or_explicit_waiver(&self) -> bool {
        self.steps.iter().all(|step| {
            if step.step_class.is_mutating() {
                step.approval_ticket_ref.is_some()
                    || step.approval_requirement == ApprovalRequirementClass::NoApprovalRequired
                    || step.approval_requirement == ApprovalRequirementClass::ApprovalForbidden
            } else {
                true
            }
        })
    }
}

/// Incident/advisory integration packet for the stable line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentAdvisoryPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Incident id join key.
    pub incident_id: String,
    /// Advisory id join key, if applicable.
    pub advisory_id: Option<String>,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Owner block.
    pub owner: OwnerBlock,
    /// Source document metadata.
    pub source_document: SourceDocumentBlock,
    /// Compatibility window.
    pub compatibility_window: CompatibilityWindow,
    /// Field-playbook packet reference.
    pub field_playbook_ref: String,
    /// Action ledger entry refs.
    pub action_ledger_entry_refs: Vec<String>,
    /// Deviation notes.
    pub deviation_notes: Vec<DeviationNote>,
    /// Console handoff refs.
    pub console_handoff_refs: Vec<String>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// UTC emitted timestamp.
    pub emitted_at: String,
}

impl IncidentAdvisoryPacket {
    /// Returns true when the packet is metadata-safe.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .deviation_notes
                .iter()
                .all(|note| note.record_kind == DEVIATION_NOTE_RECORD_KIND)
    }
}

/// One entry in the supportability runbook catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportabilityRunbookCatalogEntry {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable entry id.
    pub entry_id: String,
    /// Closed runbook source class.
    pub source_class: RunbookSourceClass,
    /// Field-playbook packet.
    pub playbook_packet: FieldPlaybookPacket,
    /// Incident/advisory packet, if applicable.
    pub incident_advisory_packet: Option<IncidentAdvisoryPacket>,
    /// Fixture reference path.
    pub fixture_ref: String,
}

/// Supportability runbook catalog for the stable line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportabilityRunbookCatalog {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Schema reference.
    pub schema_ref: String,
    /// Doc reference.
    pub doc_ref: String,
    /// Artifact reference.
    pub artifact_ref: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Catalog entries.
    pub entries: Vec<SupportabilityRunbookCatalogEntry>,
}

impl SupportabilityRunbookCatalog {
    /// Returns an iterator over the playbook packets in the catalog.
    pub fn playbook_packets(&self) -> impl Iterator<Item = &FieldPlaybookPacket> {
        self.entries.iter().map(|e| &e.playbook_packet)
    }

    /// Returns an iterator over the incident/advisory packets in the catalog.
    pub fn incident_advisory_packets(&self) -> impl Iterator<Item = &IncidentAdvisoryPacket> {
        self.entries
            .iter()
            .filter_map(|e| e.incident_advisory_packet.as_ref())
    }

    /// Validates the catalog against the stable-line contract.
    pub fn validate(&self) -> Vec<SupportabilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "catalog.schema_version",
                &self.catalog_id,
                "catalog schema_version must be 1",
            );
        }
        if self.record_kind != SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND {
            push_violation(
                &mut violations,
                "catalog.record_kind",
                &self.catalog_id,
                "catalog record_kind mismatch",
            );
        }
        if !self.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "catalog.raw_private_material_excluded",
                &self.catalog_id,
                "catalog must exclude raw private material",
            );
        }
        if !self.ambient_authority_excluded {
            push_violation(
                &mut violations,
                "catalog.ambient_authority_excluded",
                &self.catalog_id,
                "catalog must exclude ambient authority",
            );
        }

        let mut covered_sources = BTreeSet::new();
        for entry in &self.entries {
            covered_sources.insert(entry.source_class);
            self.validate_entry(entry, &mut violations);
        }

        for required in RunbookSourceClass::REQUIRED {
            if !covered_sources.contains(&required) {
                push_violation(
                    &mut violations,
                    "catalog.required_source_class_missing",
                    &self.catalog_id,
                    &format!(
                        "missing required runbook source class: {}",
                        required.as_str()
                    ),
                );
            }
        }

        violations
    }

    fn validate_entry(
        &self,
        entry: &SupportabilityRunbookCatalogEntry,
        violations: &mut Vec<SupportabilityViolation>,
    ) {
        if entry.record_kind != SUPPORTABILITY_RUNBOOK_CATALOG_ENTRY_RECORD_KIND {
            push_violation(
                violations,
                "entry.record_kind",
                &entry.entry_id,
                "entry record_kind mismatch",
            );
        }

        let packet = &entry.playbook_packet;
        if packet.record_kind != FIELD_PLAYBOOK_PACKET_RECORD_KIND {
            push_violation(
                violations,
                "entry.playbook.record_kind",
                &packet.packet_id,
                "playbook packet record_kind mismatch",
            );
        }
        if packet.schema_version != PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION {
            push_violation(
                violations,
                "entry.playbook.schema_version",
                &packet.packet_id,
                "playbook packet schema_version must be 1",
            );
        }
        if packet.steps.is_empty() {
            push_violation(
                violations,
                "entry.playbook.steps.empty",
                &packet.packet_id,
                "playbook packet must have at least one step",
            );
        }

        // Step ordinals must be unique and contiguous from 0.
        let mut ordinals: Vec<u32> = packet.steps.iter().map(|s| s.ordinal).collect();
        ordinals.sort();
        for (expected, actual) in ordinals.iter().enumerate() {
            if *actual != expected as u32 {
                push_violation(
                    violations,
                    "entry.playbook.steps.ordinal",
                    &packet.packet_id,
                    "step ordinals must be contiguous starting from 0",
                );
                break;
            }
        }

        // Mutating steps must have approval ticket ref or explicit waiver.
        for step in &packet.steps {
            if step.step_class.is_mutating()
                && step.approval_ticket_ref.is_none()
                && step.approval_requirement != ApprovalRequirementClass::NoApprovalRequired
                && step.approval_requirement != ApprovalRequirementClass::ApprovalForbidden
            {
                push_violation(
                    violations,
                    "entry.playbook.step.approval",
                    &step.step_id,
                    "mutating step must carry an approval_ticket_ref or explicit waiver",
                );
            }
            if step.target_selector_scope.requires_external_handoff()
                && step.handoff_metadata.is_none()
            {
                push_violation(
                    violations,
                    "entry.playbook.step.handoff",
                    &step.step_id,
                    "step with external target scope must carry handoff_metadata",
                );
            }
        }

        // If browser-only source, authoritative posture must be reference_only.
        if entry.source_class == RunbookSourceClass::BrowserOnlyVendorDocs
            && packet.source_document.authoritative_posture != AuthoritativePosture::ReferenceOnly
        {
            push_violation(
                violations,
                "entry.playbook.source.authoritative_posture",
                &packet.packet_id,
                "browser_only_vendor_docs source must have reference_only posture",
            );
        }

        if let Some(ref incident) = entry.incident_advisory_packet {
            if incident.record_kind != INCIDENT_ADVISORY_PACKET_RECORD_KIND {
                push_violation(
                    violations,
                    "entry.incident.record_kind",
                    &incident.packet_id,
                    "incident/advisory packet record_kind mismatch",
                );
            }
            if incident.schema_version != PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION {
                push_violation(
                    violations,
                    "entry.incident.schema_version",
                    &incident.packet_id,
                    "incident/advisory packet schema_version must be 1",
                );
            }
            if !incident.raw_private_material_excluded || !incident.ambient_authority_excluded {
                push_violation(
                    violations,
                    "entry.incident.privacy",
                    &incident.packet_id,
                    "incident/advisory packet must exclude raw private material and ambient authority",
                );
            }
            for note in &incident.deviation_notes {
                if note.record_kind != DEVIATION_NOTE_RECORD_KIND {
                    push_violation(
                        violations,
                        "entry.incident.deviation_note.record_kind",
                        &note.deviation_note_id,
                        "deviation note record_kind mismatch",
                    );
                }
            }
        }
    }
}

/// One validation issue found in the supportability catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportabilityViolation {
    /// Stable validation check id.
    pub check_id: String,
    /// Artifact or row ref associated with the violation.
    pub ref_id: String,
    /// Copy-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<SupportabilityViolation>,
    check_id: &str,
    ref_id: &str,
    message: &str,
) {
    violations.push(SupportabilityViolation {
        check_id: check_id.to_owned(),
        ref_id: ref_id.to_owned(),
        message: message.to_owned(),
    });
}

pub mod doctor_projection;

// ---------------------------------------------------------------------------
// Fixture loading
// ---------------------------------------------------------------------------

/// Loads a [`FieldPlaybookPacket`] from a YAML string.
///
/// # Errors
///
/// Returns a YAML deserialization error when the input does not match the
/// packet shape.
pub fn load_field_playbook_packet(yaml: &str) -> Result<FieldPlaybookPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads an [`IncidentAdvisoryPacket`] from a YAML string.
///
/// # Errors
///
/// Returns a YAML deserialization error when the input does not match the
/// packet shape.
pub fn load_incident_advisory_packet(
    yaml: &str,
) -> Result<IncidentAdvisoryPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a [`SupportabilityRunbookCatalogEntry`] from a YAML string.
///
/// # Errors
///
/// Returns a YAML deserialization error when the input does not match the
/// entry shape.
pub fn load_catalog_entry(
    yaml: &str,
) -> Result<SupportabilityRunbookCatalogEntry, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the embedded fixture corpus as a [`SupportabilityRunbookCatalog`].
///
/// # Errors
///
/// Returns a YAML deserialization error when any embedded fixture fails to
/// parse.
pub fn current_supportability_runbook_catalog(
) -> Result<SupportabilityRunbookCatalog, serde_yaml::Error> {
    let mut entries = Vec::with_capacity(FIXTURE_SOURCES.len());
    for (_fixture_ref, yaml) in FIXTURE_SOURCES {
        let entry: SupportabilityRunbookCatalogEntry = serde_yaml::from_str(yaml)?;
        entries.push(entry);
    }

    Ok(SupportabilityRunbookCatalog {
        record_kind: SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND.to_owned(),
        schema_version: PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION,
        catalog_id: "support.m4.publish_supportability_runbooks.baseline.v1".to_owned(),
        generated_at: "2026-06-02T00:00:00Z".to_owned(),
        schema_ref: PUBLISHED_SUPPORTABILITY_SCHEMA_REF.to_owned(),
        doc_ref: PUBLISHED_SUPPORTABILITY_DOC_REF.to_owned(),
        artifact_ref: PUBLISHED_SUPPORTABILITY_ARTIFACT_REF.to_owned(),
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        entries,
    })
}
