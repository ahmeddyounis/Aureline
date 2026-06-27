//! Canonical M5 runbook governance matrix and object model.
//!
//! This module freezes the runbook object model Aureline ships and the matrix
//! that governs it. The matrix ([`M5RunbookGovernancePacket`]) names the six
//! governed runbook object classes ([`RunbookObjectClass`]) — the
//! [source descriptor](RunbookSourceDescriptor), the
//! [step descriptor](RunbookStepDescriptor), the
//! [execution record](RunbookExecutionRecord), the [deviation note](DeviationNote),
//! the [control-plane handoff packet](ControlPlaneHandoffPacket), and the
//! [archival/export object](ArchivalExportObject) — and for each names its owner,
//! its first consumer, the schema that is its source of truth, and the proof
//! packet that keeps it current.
//!
//! Each [claimed runbook-backed surface](RunbookSurfaceClaim) binds the governed
//! objects it depends on. The matrix resolves, per surface:
//!
//! - a [`RunbookSurfaceStatus`] (mapped / provisional / unmapped) reflecting the
//!   *true* contract coverage, independent of waivers, so the dashboard never
//!   hides a real gap;
//! - a [`RunbookGate`] the release/public-truth automation reads — a surface that
//!   binds an object the matrix does not govern, or whose proof is missing, is
//!   *blocked* from Stable promotion (and named, never hidden), while a surface
//!   whose proof is stale *auto-narrows* below Stable before promotion;
//! - an effective [`RunbookClaimClass`] after the gate applies, floored at Beta
//!   for any unwaived narrowing gap and at the disclosed waived claim for any
//!   accepted blocking gap.
//!
//! The runbook *instances* — the seeded [operator scenarios](RunbookExecutionRecord)
//! — demonstrate the object model in practice: each declares its source authority,
//! runs typed steps, records deviation lineage, keeps console/browser handoff
//! attributable, and archives an export-safe record. Companions follow or request
//! within declared scope but never mint a hidden privileged mutate channel.
//!
//! Incident workspaces, operator dashboards, docs/help, companions, and support
//! bundles consume this one inventory. Raw provider/console payloads, credential
//! bodies, and secret material stay outside the support boundary.
//!
//! - Matrix schema:
//!   [`schemas/runbooks/m5-runbook-governance.schema.json`](../../../../../schemas/runbooks/m5-runbook-governance.schema.json)
//! - Source-descriptor schema:
//!   [`schemas/runbooks/m5-runbook-source.schema.json`](../../../../../schemas/runbooks/m5-runbook-source.schema.json)
//! - Step-descriptor schema:
//!   [`schemas/runbooks/m5-runbook-step.schema.json`](../../../../../schemas/runbooks/m5-runbook-step.schema.json)
//! - Execution-record schema:
//!   [`schemas/runbooks/m5-runbook-execution.schema.json`](../../../../../schemas/runbooks/m5-runbook-execution.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-governance.md`](../../../../../docs/runbooks/m5-runbook-governance.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_runbook_governance_packet, seeded_m5_runbook_governance_packet_missing_proof_blocked,
    seeded_m5_runbook_governance_packet_stale_proof_narrowed,
    seeded_m5_runbook_governance_packet_waived_narrowed, seeded_operator_scenario_records,
    M5_RUNBOOK_GOVERNANCE_PACKET_ID,
};

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Record-kind tag carried by [`M5RunbookGovernancePacket`].
pub const M5_RUNBOOK_GOVERNANCE_RECORD_KIND: &str = "m5_runbook_governance";

/// Schema version for the governance matrix packet.
pub const M5_RUNBOOK_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag carried by [`M5RunbookGovernanceMatrix`].
pub const M5_RUNBOOK_GOVERNANCE_MATRIX_RECORD_KIND: &str = "m5_runbook_governance_matrix";

/// Schema version for the matrix dashboard projection.
pub const M5_RUNBOOK_GOVERNANCE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag carried by [`RunbookSourceDescriptor`].
pub const M5_RUNBOOK_SOURCE_RECORD_KIND: &str = "m5_runbook_source_descriptor";

/// Record-kind tag carried by [`RunbookStepDescriptor`].
pub const M5_RUNBOOK_STEP_RECORD_KIND: &str = "m5_runbook_step_descriptor";

/// Record-kind tag carried by [`RunbookExecutionRecord`].
pub const M5_RUNBOOK_EXECUTION_RECORD_KIND: &str = "m5_runbook_execution_record";

/// Object-instance schema version shared by the source, step, and execution records.
pub const M5_RUNBOOK_OBJECT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the matrix boundary schema.
pub const M5_RUNBOOK_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-governance.schema.json";

/// Repo-relative path of the source-descriptor schema.
pub const M5_RUNBOOK_SOURCE_SCHEMA_REF: &str = "schemas/runbooks/m5-runbook-source.schema.json";

/// Repo-relative path of the step-descriptor schema.
pub const M5_RUNBOOK_STEP_SCHEMA_REF: &str = "schemas/runbooks/m5-runbook-step.schema.json";

/// Repo-relative path of the execution-record schema.
pub const M5_RUNBOOK_EXECUTION_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-execution.schema.json";

/// Repo-relative path of the governance matrix doc.
pub const M5_RUNBOOK_GOVERNANCE_DOC_REF: &str = "docs/runbooks/m5-runbook-governance.md";

/// Repo-relative path of the published canonical matrix inventory.
pub const M5_RUNBOOK_GOVERNANCE_MATRIX_REF: &str = "artifacts/runbooks/m5-runbook-governance.json";

/// Repo-relative path of the release-grade governance support export.
pub const M5_RUNBOOK_GOVERNANCE_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-governance.json";

/// Repo-relative directory of the operator-scenario execution-record fixtures.
pub const M5_RUNBOOK_OPERATOR_SCENARIO_DIR: &str = "fixtures/runbooks/m5-operator-scenarios/";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_RUNBOOK_MESSAGE_ID_PREFIX: &str = "runbooks_governance.";

/// One of the six governed runbook object classes the matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookObjectClass {
    /// Runbook source descriptor declaring where authority comes from.
    SourceDescriptor,
    /// Executable step descriptor declaring its step class, scope, and boundary.
    StepDescriptor,
    /// Execution record capturing what ran, with deviation lineage.
    ExecutionRecord,
    /// Deviation note recording a departure from declared guidance.
    DeviationNote,
    /// Console/browser control-plane handoff packet.
    ControlPlaneHandoff,
    /// Archival/export object for retained execution history.
    ArchivalExport,
}

impl RunbookObjectClass {
    /// Every governed object class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceDescriptor,
        Self::StepDescriptor,
        Self::ExecutionRecord,
        Self::DeviationNote,
        Self::ControlPlaneHandoff,
        Self::ArchivalExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceDescriptor => "source_descriptor",
            Self::StepDescriptor => "step_descriptor",
            Self::ExecutionRecord => "execution_record",
            Self::DeviationNote => "deviation_note",
            Self::ControlPlaneHandoff => "control_plane_handoff",
            Self::ArchivalExport => "archival_export",
        }
    }

    /// Repo-relative schema that is this object class's source of truth.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::SourceDescriptor => M5_RUNBOOK_SOURCE_SCHEMA_REF,
            Self::StepDescriptor => M5_RUNBOOK_STEP_SCHEMA_REF,
            // Execution records embed deviation notes, handoff packets, and the
            // archival/export object, so the execution schema is their source of truth.
            Self::ExecutionRecord
            | Self::DeviationNote
            | Self::ControlPlaneHandoff
            | Self::ArchivalExport => M5_RUNBOOK_EXECUTION_SCHEMA_REF,
        }
    }
}

/// Where a runbook's authority comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceClass {
    /// Authored and owned in-repo as first-party governed guidance.
    VendoredFirstParty,
    /// Authored inside the operating organization's own runbook library.
    OrganizationAuthored,
    /// Imported from an external vendor console as read-only reference.
    ImportedVendorConsole,
    /// Drafted by a companion and pending human approval before it gains authority.
    CompanionDrafted,
    /// Reconstructed from archived execution history.
    ArchivedExecution,
}

impl RunbookSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::VendoredFirstParty,
        Self::OrganizationAuthored,
        Self::ImportedVendorConsole,
        Self::CompanionDrafted,
        Self::ArchivedExecution,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VendoredFirstParty => "vendored_first_party",
            Self::OrganizationAuthored => "organization_authored",
            Self::ImportedVendorConsole => "imported_vendor_console",
            Self::CompanionDrafted => "companion_drafted",
            Self::ArchivedExecution => "archived_execution",
        }
    }

    /// True when this source class carries standing authority to execute mutating
    /// steps without a per-execution promotion. Companion drafts and imported
    /// vendor-console references do not.
    pub const fn carries_execution_authority(self) -> bool {
        matches!(
            self,
            Self::VendoredFirstParty | Self::OrganizationAuthored | Self::ArchivedExecution
        )
    }
}

/// What class of step is being executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepClass {
    /// Read-only inspection of state.
    Inspect,
    /// Read-only diagnosis or hypothesis formation.
    Diagnose,
    /// Scoped mutating mitigation.
    Mitigate,
    /// Mutating rollback to a prior state.
    Rollback,
    /// Pivot to an external console or browser boundary.
    ConsoleHandoff,
    /// Explicit human approval gate.
    Approval,
    /// Record a note or annotation; non-mutating.
    Annotate,
}

impl RunbookStepClass {
    /// Every step class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Inspect,
        Self::Diagnose,
        Self::Mitigate,
        Self::Rollback,
        Self::ConsoleHandoff,
        Self::Approval,
        Self::Annotate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Diagnose => "diagnose",
            Self::Mitigate => "mitigate",
            Self::Rollback => "rollback",
            Self::ConsoleHandoff => "console_handoff",
            Self::Approval => "approval",
            Self::Annotate => "annotate",
        }
    }

    /// True when the step changes target state.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }

    /// True when the step crosses Aureline's governed plane to an external boundary.
    pub const fn is_console_handoff(self) -> bool {
        matches!(self, Self::ConsoleHandoff)
    }
}

/// What scope or approval a step requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookApprovalScope {
    /// Read-only; no approval is required.
    NoApprovalReadOnly,
    /// Scoped change the operator may self-approve within declared bounds.
    ScopedSelfApprove,
    /// Requires explicit human approval before execution.
    RequiresHumanApproval,
    /// Requires privileged human approval (elevated scope).
    RequiresPrivilegedApproval,
    /// A mutate path a companion is prohibited from creating; never a hidden channel.
    ProhibitedHiddenMutate,
}

impl RunbookApprovalScope {
    /// Every approval scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoApprovalReadOnly,
        Self::ScopedSelfApprove,
        Self::RequiresHumanApproval,
        Self::RequiresPrivilegedApproval,
        Self::ProhibitedHiddenMutate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalReadOnly => "no_approval_read_only",
            Self::ScopedSelfApprove => "scoped_self_approve",
            Self::RequiresHumanApproval => "requires_human_approval",
            Self::RequiresPrivilegedApproval => "requires_privileged_approval",
            Self::ProhibitedHiddenMutate => "prohibited_hidden_mutate",
        }
    }

    /// True when a companion may act under this scope without minting privilege.
    pub const fn companion_may_act(self) -> bool {
        matches!(self, Self::NoApprovalReadOnly | Self::ScopedSelfApprove)
    }
}

/// Lineage class for a departure from declared guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviationClass {
    /// The step ran exactly as declared.
    NoDeviation,
    /// A declared parameter was adjusted within scope.
    ParameterAdjusted,
    /// A declared step was skipped.
    StepSkipped,
    /// An ad-hoc step not in the declared guidance was added.
    StepAddedAdHoc,
    /// Execution was aborted mid-step.
    AbortedMidStep,
    /// An unplanned console/browser pivot occurred.
    ConsolePivotUnplanned,
}

impl DeviationClass {
    /// Every deviation class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoDeviation,
        Self::ParameterAdjusted,
        Self::StepSkipped,
        Self::StepAddedAdHoc,
        Self::AbortedMidStep,
        Self::ConsolePivotUnplanned,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDeviation => "no_deviation",
            Self::ParameterAdjusted => "parameter_adjusted",
            Self::StepSkipped => "step_skipped",
            Self::StepAddedAdHoc => "step_added_ad_hoc",
            Self::AbortedMidStep => "aborted_mid_step",
            Self::ConsolePivotUnplanned => "console_pivot_unplanned",
        }
    }

    /// True when the step departed from declared guidance.
    pub const fn is_deviation(self) -> bool {
        !matches!(self, Self::NoDeviation)
    }
}

/// Control-plane boundary a step or handoff sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPlaneBoundaryClass {
    /// Stays inside Aureline's governed plane.
    InAppGoverned,
    /// Hands off to a browser surface; remains attributable.
    BrowserHandoff,
    /// Hands off to an external vendor console; remains attributable.
    VendorConsoleHandoff,
    /// Crosses an authentication boundary into an external authority.
    AuthBoundaryCross,
}

impl ControlPlaneBoundaryClass {
    /// Every boundary class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InAppGoverned,
        Self::BrowserHandoff,
        Self::VendorConsoleHandoff,
        Self::AuthBoundaryCross,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InAppGoverned => "in_app_governed",
            Self::BrowserHandoff => "browser_handoff",
            Self::VendorConsoleHandoff => "vendor_console_handoff",
            Self::AuthBoundaryCross => "auth_boundary_cross",
        }
    }

    /// True when the boundary leaves Aureline's governed plane and so requires an
    /// attributable handoff packet.
    pub const fn leaves_governed_plane(self) -> bool {
        !matches!(self, Self::InAppGoverned)
    }
}

/// A surface or system that consumes governed runbook objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookConsumer {
    /// Incident workspace surfaces.
    IncidentWorkspace,
    /// Operator dashboard surfaces.
    OperatorDashboard,
    /// Docs and Help surfaces.
    DocsHelp,
    /// Companion surfaces that follow or request within scope.
    Companion,
    /// Support bundle export surfaces.
    SupportBundle,
    /// Release center / public-truth automation.
    ReleaseCenter,
}

impl RunbookConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IncidentWorkspace,
        Self::OperatorDashboard,
        Self::DocsHelp,
        Self::Companion,
        Self::SupportBundle,
        Self::ReleaseCenter,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentWorkspace => "incident_workspace",
            Self::OperatorDashboard => "operator_dashboard",
            Self::DocsHelp => "docs_help",
            Self::Companion => "companion",
            Self::SupportBundle => "support_bundle",
            Self::ReleaseCenter => "release_center",
        }
    }
}

/// Freshness of a governed object's proof packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofFreshnessState {
    /// The proof packet is current.
    Current,
    /// The proof packet has fallen outside its freshness window; consumers narrow.
    Stale,
    /// No usable proof packet exists; consumers block.
    Missing,
}

impl ProofFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// Green/yellow/red coverage status for a claimed runbook-backed surface,
/// reflecting true contract coverage independent of waivers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSurfaceStatus {
    /// Every bound object is mapped and its proof current.
    Mapped,
    /// A bound object's proof is stale; the surface ships at a narrowed claim.
    Provisional,
    /// A bound object is unmapped or its proof is missing; the surface is unmapped.
    Unmapped,
}

impl RunbookSurfaceStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 3] = [Self::Mapped, Self::Provisional, Self::Unmapped];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mapped => "mapped",
            Self::Provisional => "provisional",
            Self::Unmapped => "unmapped",
        }
    }

    /// The traffic-light signal this status maps to.
    pub const fn signal(self) -> RunbookSignal {
        match self {
            Self::Mapped => RunbookSignal::Green,
            Self::Provisional => RunbookSignal::Yellow,
            Self::Unmapped => RunbookSignal::Red,
        }
    }
}

/// Traffic-light signal for the published matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSignal {
    /// Mapped.
    Green,
    /// Provisional.
    Yellow,
    /// Unmapped.
    Red,
}

impl RunbookSignal {
    /// Every signal, in declaration order.
    pub const ALL: [Self; 3] = [Self::Green, Self::Yellow, Self::Red];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

/// Release-gate decision the release/public-truth automation reads for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookGate {
    /// The surface may promote to Stable at its full claim.
    Governed,
    /// The surface auto-narrows to a disclosed reduced claim before promotion.
    Narrowed,
    /// The surface is blocked from Stable promotion by a missing object or proof.
    Blocked,
}

impl RunbookGate {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::Governed, Self::Narrowed, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Governed => "governed",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// True when the decision blocks Stable promotion.
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Public claim class an object or surface can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookClaimClass {
    /// Stable, fully governed.
    Stable,
    /// Beta, narrowed below Stable.
    Beta,
    /// Preview.
    Preview,
    /// Held from public claim.
    Held,
    /// Unavailable.
    Unavailable,
}

impl RunbookClaimClass {
    /// Every claim class, in declaration order (least to most restrictive).
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Held,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Held => "held",
            Self::Unavailable => "unavailable",
        }
    }

    /// Restrictiveness rank (Stable least, Unavailable most).
    const fn rank(self) -> u8 {
        match self {
            Self::Stable => 0,
            Self::Beta => 1,
            Self::Preview => 2,
            Self::Held => 3,
            Self::Unavailable => 4,
        }
    }

    /// The more restrictive of two claim classes.
    fn more_restrictive(self, other: Self) -> Self {
        if self.rank() >= other.rank() {
            self
        } else {
            other
        }
    }
}

/// One kind of coverage gap a claimed surface can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookGapKind {
    /// A bound object class has no governing contract in the matrix.
    ObjectMappingMissing,
    /// A bound object's proof packet is stale.
    ProofStale,
    /// A bound object's proof packet is missing.
    ProofMissing,
}

impl RunbookGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ObjectMappingMissing,
        Self::ProofStale,
        Self::ProofMissing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectMappingMissing => "object_mapping_missing",
            Self::ProofStale => "proof_stale",
            Self::ProofMissing => "proof_missing",
        }
    }

    /// True when this gap blocks Stable promotion without a waiver.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::ObjectMappingMissing | Self::ProofMissing)
    }
}

/// One governed runbook source descriptor: where a runbook's authority comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceDescriptor {
    /// Record kind; must equal [`M5_RUNBOOK_SOURCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_OBJECT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable source id.
    pub source_id: String,
    /// Reviewer-facing label.
    pub source_label: String,
    /// Where the runbook's authority comes from.
    pub source_class: RunbookSourceClass,
    /// Opaque ref to the authority (in-repo doc id, org library ref, or vendor console id).
    pub authority_ref: String,
    /// Owner role accountable for the source.
    pub owner_role: String,
    /// Default approval scope steps inherit unless they declare their own.
    pub default_approval_scope: RunbookApprovalScope,
    /// Whether a companion may *request* execution within declared scope.
    pub companion_may_request: bool,
    /// Whether the source descriptor is export-safe metadata.
    pub exportable: bool,
    /// Redaction class applied on export.
    pub redaction_class: String,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl RunbookSourceDescriptor {
    /// Validates a source descriptor's invariants.
    pub fn validate(&self) -> Vec<M5RunbookGovernanceViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_SOURCE_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_OBJECT_SCHEMA_VERSION
        {
            out.push(M5RunbookGovernanceViolation::WrongObjectRecordKind);
        }
        if self.source_id.trim().is_empty()
            || self.source_label.trim().is_empty()
            || self.authority_ref.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.redaction_class.trim().is_empty()
        {
            out.push(M5RunbookGovernanceViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookGovernanceViolation::UnprefixedMessageId);
        }
        // An imported vendor-console reference or companion draft carries no standing
        // execution authority, so it must not self-approve mutating steps by default.
        if !self.source_class.carries_execution_authority()
            && matches!(
                self.default_approval_scope,
                RunbookApprovalScope::ScopedSelfApprove
            )
        {
            out.push(M5RunbookGovernanceViolation::SourceAuthorityOverreach);
        }
        out
    }
}

/// One governed executable step descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStepDescriptor {
    /// Record kind; must equal [`M5_RUNBOOK_STEP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_OBJECT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable step id.
    pub step_id: String,
    /// Reviewer-facing label.
    pub step_label: String,
    /// What class of step is being executed.
    pub step_class: RunbookStepClass,
    /// What scope or approval the step requires.
    pub approval_scope: RunbookApprovalScope,
    /// The control-plane boundary the step sits on.
    pub control_plane_boundary: ControlPlaneBoundaryClass,
    /// True when the step changes target state (mirrors [`RunbookStepClass::is_mutating`]).
    pub mutating: bool,
    /// Expected evidence outputs the step produces.
    pub expected_evidence_outputs: Vec<String>,
    /// Whether a companion may execute this step within declared scope.
    pub companion_permitted: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl RunbookStepDescriptor {
    /// Validates a step descriptor's invariants.
    pub fn validate(&self) -> Vec<M5RunbookGovernanceViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_STEP_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_OBJECT_SCHEMA_VERSION
        {
            out.push(M5RunbookGovernanceViolation::WrongObjectRecordKind);
        }
        if self.step_id.trim().is_empty() || self.step_label.trim().is_empty() {
            out.push(M5RunbookGovernanceViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookGovernanceViolation::UnprefixedMessageId);
        }
        // The mutating flag must match the step class.
        if self.mutating != self.step_class.is_mutating() {
            out.push(M5RunbookGovernanceViolation::StepMutatingFlagMismatch);
        }
        // A console-handoff step must declare an out-of-plane boundary; an in-app
        // step must not claim a handoff boundary.
        if self.step_class.is_console_handoff()
            != self.control_plane_boundary.leaves_governed_plane()
        {
            out.push(M5RunbookGovernanceViolation::StepBoundaryMismatch);
        }
        // A mutating step that requires no approval, or that a companion may run while
        // requiring approval, would be a hidden privileged mutate channel.
        if self.mutating
            && matches!(
                self.approval_scope,
                RunbookApprovalScope::NoApprovalReadOnly
            )
        {
            out.push(M5RunbookGovernanceViolation::HiddenMutateChannel);
        }
        if self.companion_permitted && !self.approval_scope.companion_may_act() {
            out.push(M5RunbookGovernanceViolation::HiddenMutateChannel);
        }
        out
    }
}

/// One governed deviation note recording a departure from declared guidance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviationNote {
    /// Stable deviation id.
    pub deviation_id: String,
    /// Deviation lineage class.
    pub deviation_class: DeviationClass,
    /// The declared step id the deviation departs from (or the ad-hoc step's id).
    pub from_step_id: String,
    /// Stable message id naming the rationale; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub rationale_message_id: String,
    /// Role accountable for approving the deviation.
    pub approver_role: String,
    /// Always true for a recorded deviation: the departure is attributable.
    pub attributable: bool,
}

/// One governed console/browser control-plane handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlPlaneHandoffPacket {
    /// Stable handoff id.
    pub handoff_id: String,
    /// The control-plane boundary the handoff crosses.
    pub boundary_class: ControlPlaneBoundaryClass,
    /// Opaque, redaction-safe ref to the handoff target (console id / browser route).
    pub target_ref: String,
    /// Opaque ref attributing the handoff to a session/actor.
    pub attribution_ref: String,
    /// Whether control returns to Aureline's governed plane after the pivot.
    pub returns_to_governed_plane: bool,
    /// Always false: a handoff never mints a hidden privileged mutate channel.
    pub creates_hidden_mutate_channel: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

/// One governed archival/export object for retained execution history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivalExportObject {
    /// Stable archival id.
    pub archival_id: String,
    /// Whether the execution history is archived.
    pub archived: bool,
    /// Whether the archived record is export-safe.
    pub export_safe: bool,
    /// Retention class governing the archived record.
    pub retention_class: String,
    /// Support-pack item id used in redacted exports.
    pub support_pack_item_id: String,
    /// Always false: archival exports carry metadata, not raw content.
    pub raw_content_exported: bool,
}

/// Outcome of one executed step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepOutcomeClass {
    /// The step completed as declared.
    Completed,
    /// The step was skipped (recorded as a deviation).
    Skipped,
    /// The step handed off to an external boundary.
    HandedOff,
    /// The step is awaiting an approval gate.
    AwaitingApproval,
    /// The step was aborted and requires review.
    AbortedRequiresReview,
}

impl StepOutcomeClass {
    /// Every outcome class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Completed,
        Self::Skipped,
        Self::HandedOff,
        Self::AwaitingApproval,
        Self::AbortedRequiresReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Skipped => "skipped",
            Self::HandedOff => "handed_off",
            Self::AwaitingApproval => "awaiting_approval",
            Self::AbortedRequiresReview => "aborted_requires_review",
        }
    }
}

/// One executed step inside an execution record: the step that ran, its outcome,
/// the deviation lineage entry, and any control-plane handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutedStepResult {
    /// The governed step descriptor that ran.
    pub step: RunbookStepDescriptor,
    /// Outcome of the step.
    pub outcome: StepOutcomeClass,
    /// Deviation lineage entry for the step (`no_deviation` when clean).
    pub deviation: DeviationNote,
    /// Control-plane handoff packet when the step pivoted out of the governed plane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff: Option<ControlPlaneHandoffPacket>,
    /// Evidence refs the step produced.
    pub evidence_refs: Vec<String>,
}

/// One governed runbook execution record: an operator scenario demonstrating the
/// object model end to end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionRecord {
    /// Record kind; must equal [`M5_RUNBOOK_EXECUTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_OBJECT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable execution id.
    pub execution_id: String,
    /// Reviewer-facing label.
    pub execution_label: String,
    /// The source descriptor whose authority the execution ran under.
    pub source: RunbookSourceDescriptor,
    /// Operator role accountable for the execution.
    pub operator_role: String,
    /// Whether a companion drove this execution within declared scope.
    pub companion_driven: bool,
    /// The executed steps, in order.
    pub executed_steps: Vec<ExecutedStepResult>,
    /// Deviation lineage: every non-clean deviation across the steps, in order.
    pub deviation_lineage: Vec<DeviationNote>,
    /// The archival/export object for the retained history.
    pub archival_export: ArchivalExportObject,
    /// True when every step, deviation, and handoff is attributable.
    pub attributable: bool,
    /// True when no step or handoff minted a hidden privileged mutate channel.
    pub no_hidden_mutate_channel: bool,
    /// Redaction class applied on export.
    pub redaction_class: String,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl RunbookExecutionRecord {
    /// Recomputes the derived rollup fields (deviation lineage, attribution, and
    /// the no-hidden-mutate invariant) from the executed steps. The seed calls
    /// this so the rollups never need hand-maintenance.
    pub fn recompute(&mut self) {
        self.deviation_lineage = self
            .executed_steps
            .iter()
            .filter(|s| s.deviation.deviation_class.is_deviation())
            .map(|s| s.deviation.clone())
            .collect();
        self.attributable = self.executed_steps.iter().all(|s| {
            (!s.deviation.deviation_class.is_deviation() || s.deviation.attributable)
                && s.handoff
                    .as_ref()
                    .map(|h| !h.attribution_ref.trim().is_empty())
                    .unwrap_or(true)
        });
        self.no_hidden_mutate_channel = self.executed_steps.iter().all(|s| {
            let step_ok = s.step.validate().is_empty();
            let handoff_ok = s
                .handoff
                .as_ref()
                .map(|h| !h.creates_hidden_mutate_channel)
                .unwrap_or(true);
            step_ok && handoff_ok
        });
    }

    /// Validates an execution record's invariants.
    pub fn validate(&self) -> Vec<M5RunbookGovernanceViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_EXECUTION_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_OBJECT_SCHEMA_VERSION
        {
            out.push(M5RunbookGovernanceViolation::WrongObjectRecordKind);
        }
        if self.execution_id.trim().is_empty()
            || self.execution_label.trim().is_empty()
            || self.operator_role.trim().is_empty()
            || self.redaction_class.trim().is_empty()
        {
            out.push(M5RunbookGovernanceViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookGovernanceViolation::UnprefixedMessageId);
        }
        out.extend(self.source.validate());
        if self.executed_steps.is_empty() {
            out.push(M5RunbookGovernanceViolation::ExecutionHasNoSteps);
        }

        for result in &self.executed_steps {
            out.extend(result.step.validate());
            // A step that leaves the governed plane must carry an attributable handoff.
            let leaves = result.step.control_plane_boundary.leaves_governed_plane();
            match &result.handoff {
                Some(handoff) => {
                    if handoff.creates_hidden_mutate_channel {
                        out.push(M5RunbookGovernanceViolation::HiddenMutateChannel);
                    }
                    if handoff.attribution_ref.trim().is_empty() {
                        out.push(M5RunbookGovernanceViolation::UnattributableHandoff);
                    }
                    if !handoff
                        .detail_message_id
                        .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
                    {
                        out.push(M5RunbookGovernanceViolation::UnprefixedMessageId);
                    }
                }
                None if leaves => {
                    out.push(M5RunbookGovernanceViolation::UnattributableHandoff);
                }
                None => {}
            }
            // A recorded deviation must be attributable.
            if result.deviation.deviation_class.is_deviation() && !result.deviation.attributable {
                out.push(M5RunbookGovernanceViolation::UnattributableDeviation);
            }
            // A companion may only drive steps it is permitted for.
            if self.companion_driven
                && !result.step.companion_permitted
                && result.step.step_class.is_mutating()
            {
                out.push(M5RunbookGovernanceViolation::CompanionScopeOverreach);
            }
        }

        // The stored rollups must match a fresh recompute.
        let mut probe = self.clone();
        probe.recompute();
        if probe.deviation_lineage != self.deviation_lineage
            || probe.attributable != self.attributable
            || probe.no_hidden_mutate_channel != self.no_hidden_mutate_channel
        {
            out.push(M5RunbookGovernanceViolation::ExecutionRollupDrift);
        }

        if self.archival_export.raw_content_exported {
            out.push(M5RunbookGovernanceViolation::RawBoundaryMaterialInExport);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("execution record serializes"),
        ) {
            out.push(M5RunbookGovernanceViolation::RawBoundaryMaterialInExport);
        }
        out
    }

    /// Deterministic export-safe JSON for the execution record.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only record fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("execution record serializes")
    }
}

/// One governed runbook object's matrix row: the object class, its owner, its
/// first consumer, its source-of-truth schema, and its proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookObjectContract {
    /// The governed object class.
    pub object_class: RunbookObjectClass,
    /// Reviewer-facing label.
    pub object_label: String,
    /// Owner role accountable for keeping this object current.
    pub owner_role: String,
    /// The first consumer that reads this object.
    pub first_consumer: RunbookConsumer,
    /// Repo-relative schema that is the object's source of truth.
    pub schema_ref: String,
    /// Repo-relative proof packet that keeps the object current.
    pub proof_ref: String,
    /// Freshness of the proof packet.
    pub proof_freshness: ProofFreshnessState,
    /// The controlled-vocabulary tokens this object governs (source/step classes, etc.).
    pub governed_vocab: Vec<String>,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

/// One active waiver accepting a disclosed reduced claim for a single blocking gap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookWaiver {
    /// Stable waiver id.
    pub waiver_id: String,
    /// The gap kind the waiver scopes.
    pub gap_kind: RunbookGapKind,
    /// The object class the waiver scopes; the waiver only covers this object.
    pub object_class: RunbookObjectClass,
    /// Stable message id naming the reason; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Owner role accountable for the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry timestamp of the waiver.
    pub expires_at: String,
    /// The disclosed reduced claim accepted under this waiver.
    pub narrowed_to: RunbookClaimClass,
}

/// One coverage gap on a claimed runbook-backed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSurfaceGap {
    /// Surface this gap applies to.
    pub surface_id: String,
    /// The bound object class the gap concerns.
    pub object_class: RunbookObjectClass,
    /// The kind of gap.
    pub gap_kind: RunbookGapKind,
    /// Whether this gap was accepted under an active waiver.
    pub waived: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// Derived verdict fields computed from a surface's gaps and waivers.
struct DerivedVerdict {
    status: RunbookSurfaceStatus,
    signal: RunbookSignal,
    gate: RunbookGate,
    effective_class: RunbookClaimClass,
}

fn derive_verdict(
    claimed: RunbookClaimClass,
    gaps: &[RunbookSurfaceGap],
    waivers: &[RunbookWaiver],
) -> DerivedVerdict {
    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    // The status reflects true coverage, independent of waivers, so the matrix
    // never hides a real gap behind a waiver.
    let status = if any_blocking {
        RunbookSurfaceStatus::Unmapped
    } else if any_narrowing {
        RunbookSurfaceStatus::Provisional
    } else {
        RunbookSurfaceStatus::Mapped
    };

    let unwaived_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking() && !g.waived);
    let any_gap = !gaps.is_empty();

    let gate = if unwaived_blocking {
        RunbookGate::Blocked
    } else if any_gap {
        RunbookGate::Narrowed
    } else {
        RunbookGate::Governed
    };

    let effective_class = match gate {
        RunbookGate::Governed => claimed,
        RunbookGate::Blocked => RunbookClaimClass::Held,
        RunbookGate::Narrowed => {
            let mut effective = RunbookClaimClass::Stable;
            if gaps.iter().any(|g| !g.waived) {
                effective = effective.more_restrictive(RunbookClaimClass::Beta);
            }
            for waiver in waivers {
                effective = effective.more_restrictive(waiver.narrowed_to);
            }
            effective
        }
    };

    DerivedVerdict {
        status,
        signal: status.signal(),
        gate,
        effective_class,
    }
}

/// One claimed runbook-backed surface: the consumer it serves, the governed
/// objects it binds, its verdict, active waivers, and exact gaps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSurfaceClaim {
    /// Stable surface id, unique within the packet.
    pub surface_id: String,
    /// Reviewer-facing surface label.
    pub surface_label: String,
    /// The consumer family this surface belongs to.
    pub consumer: RunbookConsumer,
    /// Owner role accountable for keeping this surface's claim current.
    pub owner_role: String,
    /// Public claim the surface wants to keep.
    pub claimed_class: RunbookClaimClass,
    /// The governed object classes the surface depends on.
    pub bound_object_classes: Vec<RunbookObjectClass>,
    /// Effective claim after the gate applies.
    pub effective_class: RunbookClaimClass,
    /// Green/yellow/red coverage status.
    pub status: RunbookSurfaceStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: RunbookSignal,
    /// Release-gate decision the release/public-truth automation reads.
    pub gate_decision: RunbookGate,
    /// Active waivers accepting a disclosed reduced claim for one blocking gap each.
    pub waivers: Vec<RunbookWaiver>,
    /// Exact coverage gaps for this surface.
    pub gaps: Vec<RunbookSurfaceGap>,
    /// Stable message id for the status; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl RunbookSurfaceClaim {
    /// Recomputes the gaps and verdict from the matrix object contracts, so the
    /// claim is always generated from the same checked-in inventory Aureline ships.
    pub fn recompute(&mut self, contracts: &[RunbookObjectContract]) {
        let mut gaps = Vec::new();
        let waived = |kind: RunbookGapKind, object_class: RunbookObjectClass| -> bool {
            self.waivers
                .iter()
                .any(|w| w.gap_kind == kind && w.object_class == object_class)
        };
        let mut push_gap = |object_class: RunbookObjectClass, kind: RunbookGapKind| {
            gaps.push(RunbookSurfaceGap {
                surface_id: self.surface_id.clone(),
                object_class,
                gap_kind: kind,
                waived: waived(kind, object_class),
                cause_message_id: format!(
                    "{}{}.{}.{}.gap",
                    M5_RUNBOOK_MESSAGE_ID_PREFIX,
                    self.surface_id,
                    object_class.as_str(),
                    kind.as_str()
                ),
            });
        };

        for &object_class in &self.bound_object_classes {
            match contracts.iter().find(|c| c.object_class == object_class) {
                None => push_gap(object_class, RunbookGapKind::ObjectMappingMissing),
                Some(contract) => match contract.proof_freshness {
                    ProofFreshnessState::Current => {}
                    ProofFreshnessState::Stale => {
                        push_gap(object_class, RunbookGapKind::ProofStale)
                    }
                    ProofFreshnessState::Missing => {
                        push_gap(object_class, RunbookGapKind::ProofMissing)
                    }
                },
            }
        }

        gaps.sort_by(|a, b| {
            a.object_class
                .as_str()
                .cmp(b.object_class.as_str())
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });
        self.gaps = gaps;

        let verdict = derive_verdict(self.claimed_class, &self.gaps, &self.waivers);
        self.status = verdict.status;
        self.signal = verdict.signal;
        self.gate_decision = verdict.gate;
        self.effective_class = verdict.effective_class;
    }

    /// True when the surface is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the surface auto-narrowed below its claim.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.gate_decision, RunbookGate::Narrowed)
    }

    /// True when the surface is fully governed for Stable promotion.
    pub fn is_governed(&self) -> bool {
        matches!(self.gate_decision, RunbookGate::Governed)
    }
}

/// Packet-level release gate aggregating the per-surface gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookReleaseGate {
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted surface ids blocked from Stable promotion.
    pub blocked_surface_ids: Vec<String>,
    /// Sorted surface ids that auto-narrowed below their claim.
    pub narrowed_surface_ids: Vec<String>,
    /// Sorted surface ids fully governed for Stable promotion.
    pub governed_surface_ids: Vec<String>,
    /// Sorted surface ids carrying at least one active waiver.
    pub waived_surface_ids: Vec<String>,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookVocabularySet {
    /// Object-class tokens.
    pub object_classes: Vec<String>,
    /// Source-class tokens.
    pub source_classes: Vec<String>,
    /// Step-class tokens.
    pub step_classes: Vec<String>,
    /// Approval-scope tokens.
    pub approval_scopes: Vec<String>,
    /// Deviation-class tokens.
    pub deviation_classes: Vec<String>,
    /// Control-plane boundary tokens.
    pub control_plane_boundaries: Vec<String>,
    /// Step-outcome tokens.
    pub step_outcomes: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Proof-freshness tokens.
    pub proof_freshness_states: Vec<String>,
    /// Surface-status tokens.
    pub surface_statuses: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Claim-class tokens.
    pub claim_classes: Vec<String>,
}

impl RunbookVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        fn tokens<T: AsToken + Copy, const N: usize>(values: [T; N]) -> Vec<String> {
            values.iter().map(|v| v.as_token().to_owned()).collect()
        }
        Self {
            object_classes: tokens(RunbookObjectClass::ALL),
            source_classes: tokens(RunbookSourceClass::ALL),
            step_classes: tokens(RunbookStepClass::ALL),
            approval_scopes: tokens(RunbookApprovalScope::ALL),
            deviation_classes: tokens(DeviationClass::ALL),
            control_plane_boundaries: tokens(ControlPlaneBoundaryClass::ALL),
            step_outcomes: tokens(StepOutcomeClass::ALL),
            consumers: tokens(RunbookConsumer::ALL),
            proof_freshness_states: tokens(ProofFreshnessState::ALL),
            surface_statuses: tokens(RunbookSurfaceStatus::ALL),
            signals: tokens(RunbookSignal::ALL),
            gate_decisions: tokens(RunbookGate::ALL),
            gap_kinds: tokens(RunbookGapKind::ALL),
            claim_classes: tokens(RunbookClaimClass::ALL),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Internal trait letting [`RunbookVocabularySet::canonical`] map any vocabulary
/// enum to its stable token without repeating the closure per enum.
trait AsToken {
    fn as_token(&self) -> &'static str;
}

macro_rules! impl_as_token {
    ($($ty:ty),+ $(,)?) => {
        $(impl AsToken for $ty {
            fn as_token(&self) -> &'static str {
                self.as_str()
            }
        })+
    };
}

impl_as_token!(
    RunbookObjectClass,
    RunbookSourceClass,
    RunbookStepClass,
    RunbookApprovalScope,
    DeviationClass,
    ControlPlaneBoundaryClass,
    StepOutcomeClass,
    RunbookConsumer,
    ProofFreshnessState,
    RunbookSurfaceStatus,
    RunbookSignal,
    RunbookGate,
    RunbookGapKind,
    RunbookClaimClass,
);

/// Governance conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookConformanceReview {
    /// Every one of the six governed object classes has a matrix contract.
    pub every_object_class_governed: bool,
    /// Every governed object names an owner, a first consumer, and a proof packet.
    pub every_object_names_owner_consumer_and_proof: bool,
    /// Every claimed surface binds at least one governed object.
    pub every_surface_binds_governed_objects: bool,
    /// A bound object with no matrix contract blocks Stable promotion.
    pub missing_object_blocks_stable_promotion: bool,
    /// A stale or missing proof narrows or blocks before Stable promotion.
    pub stale_or_missing_proof_gates_before_stable: bool,
    /// Active waivers are disclosed with scope, owner, and expiry.
    pub waivers_disclosed_with_scope_owner_and_expiry: bool,
    /// Exact coverage gaps are named per surface.
    pub exact_gaps_named: bool,
    /// Runbooks declare source authority, step class, scope, and expected evidence.
    pub runbooks_declare_authority_step_scope_and_evidence: bool,
    /// Console/browser pivots and archived history stay attributable.
    pub console_pivots_and_archives_stay_attributable: bool,
    /// Companions act only within declared scope; no hidden mutate channels.
    pub companions_bounded_no_hidden_mutate_channels: bool,
    /// The matrix is generated from the same checked-in object contracts.
    pub generated_from_checked_in_contracts: bool,
    /// Support export carries no raw boundary material.
    pub support_export_carries_no_raw_boundary_material: bool,
}

impl RunbookConformanceReview {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_object_class_governed
            && self.every_object_names_owner_consumer_and_proof
            && self.every_surface_binds_governed_objects
            && self.missing_object_blocks_stable_promotion
            && self.stale_or_missing_proof_gates_before_stable
            && self.waivers_disclosed_with_scope_owner_and_expiry
            && self.exact_gaps_named
            && self.runbooks_declare_authority_step_scope_and_evidence
            && self.console_pivots_and_archives_stay_attributable
            && self.companions_bounded_no_hidden_mutate_channels
            && self.generated_from_checked_in_contracts
            && self.support_export_carries_no_raw_boundary_material
    }
}

/// Consumer projection block: who reads the governed runbook objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookConsumerProjection {
    /// Incident workspaces reference the runbook contract inventory.
    pub incident_workspace_references_inventory: bool,
    /// Operator dashboards reference the runbook contract inventory.
    pub operator_dashboard_references_inventory: bool,
    /// Docs and Help reference the runbook contract inventory.
    pub docs_help_references_inventory: bool,
    /// Companions follow or request within declared scope.
    pub companion_follows_within_declared_scope: bool,
    /// Support export ships the governed runbook objects.
    pub support_export_ships_runbook_objects: bool,
    /// The release center gates promotion on the matrix.
    pub release_center_gates_on_matrix: bool,
}

impl RunbookConsumerProjection {
    /// True when every projection holds.
    pub fn all_hold(&self) -> bool {
        self.incident_workspace_references_inventory
            && self.operator_dashboard_references_inventory
            && self.docs_help_references_inventory
            && self.companion_follows_within_declared_scope
            && self.support_export_ships_runbook_objects
            && self.release_center_gates_on_matrix
    }
}

/// Compact green/yellow/red governance matrix — the published scoreboard the
/// incident, operator, docs/help, release, and support surfaces all read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookGovernanceMatrix {
    /// Record kind; must equal [`M5_RUNBOOK_GOVERNANCE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_GOVERNANCE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Cross-ref to the packet this matrix projects.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the matrix was computed as-of.
    pub evaluated_at: String,
    /// Total governed object classes.
    pub total_objects: u32,
    /// Total claimed surfaces.
    pub total_surfaces: u32,
    /// Green (mapped) surface count.
    pub green_count: u32,
    /// Yellow (provisional) surface count.
    pub yellow_count: u32,
    /// Red (unmapped) surface count.
    pub red_count: u32,
    /// Mapped surface ids (sorted).
    pub mapped_surface_ids: Vec<String>,
    /// Provisional surface ids (sorted).
    pub provisional_surface_ids: Vec<String>,
    /// Unmapped surface ids (sorted).
    pub unmapped_surface_ids: Vec<String>,
    /// Surface ids that auto-narrowed below their claim (sorted).
    pub narrowed_surface_ids: Vec<String>,
    /// Surface ids blocked from Stable promotion (sorted).
    pub blocked_surface_ids: Vec<String>,
    /// Surface ids carrying at least one active waiver (sorted).
    pub waived_surface_ids: Vec<String>,
    /// Active waiver ids (sorted).
    pub active_waiver_ids: Vec<String>,
    /// Object classes whose proof is stale (sorted tokens).
    pub stale_proof_object_classes: Vec<String>,
    /// Object classes whose proof is missing (sorted tokens).
    pub missing_proof_object_classes: Vec<String>,
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Exact coverage gaps across all surfaces.
    pub coverage_gaps: Vec<RunbookSurfaceGap>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Stable message id; prefixed [`M5_RUNBOOK_MESSAGE_ID_PREFIX`].
    pub matrix_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Constructor input for [`M5RunbookGovernancePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookGovernancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the matrix was computed as-of.
    pub evaluated_at: String,
    /// The governed object contracts.
    pub object_contracts: Vec<RunbookObjectContract>,
    /// Per-surface claims.
    pub surface_claims: Vec<RunbookSurfaceClaim>,
    /// Controlled-vocabulary set.
    pub vocabulary_set: RunbookVocabularySet,
    /// Conformance review block.
    pub conformance_review: RunbookConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: RunbookConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: RunbookReleaseGate,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookGovernancePacket {
    /// Record kind; must equal [`M5_RUNBOOK_GOVERNANCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_GOVERNANCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the matrix was computed as-of.
    pub evaluated_at: String,
    /// The governed object contracts.
    pub object_contracts: Vec<RunbookObjectContract>,
    /// Per-surface claims.
    pub surface_claims: Vec<RunbookSurfaceClaim>,
    /// Controlled-vocabulary set.
    pub vocabulary_set: RunbookVocabularySet,
    /// Conformance review block.
    pub conformance_review: RunbookConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: RunbookConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: RunbookReleaseGate,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookGovernancePacket {
    /// Builds a governance packet from seed input.
    pub fn new(input: M5RunbookGovernancePacketInput) -> Self {
        Self {
            record_kind: M5_RUNBOOK_GOVERNANCE_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            object_contracts: input.object_contracts,
            surface_claims: input.surface_claims,
            vocabulary_set: input.vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            release_gate: input.release_gate,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release/public-truth automation must hold Stable promotion
    /// because at least one claimed surface is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Surface ids currently blocked from Stable promotion.
    pub fn blocked_surface_ids(&self) -> Vec<&str> {
        self.surface_claims
            .iter()
            .filter(|s| s.is_blocked())
            .map(|s| s.surface_id.as_str())
            .collect()
    }

    /// Finds a governed object contract by class.
    pub fn object_contract(&self, class: RunbookObjectClass) -> Option<&RunbookObjectContract> {
        self.object_contracts
            .iter()
            .find(|c| c.object_class == class)
    }

    /// Finds a surface claim by id.
    pub fn surface(&self, surface_id: &str) -> Option<&RunbookSurfaceClaim> {
        self.surface_claims
            .iter()
            .find(|s| s.surface_id == surface_id)
    }

    /// Builds the compact green/yellow/red matrix projection from the rows.
    pub fn matrix(&self) -> M5RunbookGovernanceMatrix {
        let by_status = |status: RunbookSurfaceStatus| -> Vec<String> {
            let mut ids: Vec<String> = self
                .surface_claims
                .iter()
                .filter(|s| s.status == status)
                .map(|s| s.surface_id.clone())
                .collect();
            ids.sort();
            ids
        };
        let by_predicate = |predicate: &dyn Fn(&RunbookSurfaceClaim) -> bool| -> Vec<String> {
            let mut ids: Vec<String> = self
                .surface_claims
                .iter()
                .filter(|s| predicate(s))
                .map(|s| s.surface_id.clone())
                .collect();
            ids.sort();
            ids
        };
        let proof_classes = |state: ProofFreshnessState| -> Vec<String> {
            let mut tokens: Vec<String> = self
                .object_contracts
                .iter()
                .filter(|c| c.proof_freshness == state)
                .map(|c| c.object_class.as_str().to_owned())
                .collect();
            tokens.sort();
            tokens
        };

        let mut active_waiver_ids: Vec<String> = self
            .surface_claims
            .iter()
            .flat_map(|s| s.waivers.iter().map(|w| w.waiver_id.clone()))
            .collect();
        active_waiver_ids.sort();

        let mut coverage_gaps: Vec<RunbookSurfaceGap> = self
            .surface_claims
            .iter()
            .flat_map(|s| s.gaps.iter().cloned())
            .collect();
        coverage_gaps.sort_by(|a, b| {
            a.surface_id
                .cmp(&b.surface_id)
                .then(a.object_class.as_str().cmp(b.object_class.as_str()))
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        let count_signal = |signal: RunbookSignal| -> u32 {
            self.surface_claims
                .iter()
                .filter(|s| s.signal == signal)
                .count() as u32
        };

        let blocked_surface_ids = by_predicate(&|s| s.is_blocked());

        M5RunbookGovernanceMatrix {
            record_kind: M5_RUNBOOK_GOVERNANCE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_GOVERNANCE_MATRIX_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            report_label: self.report_label.clone(),
            evaluated_at: self.evaluated_at.clone(),
            total_objects: self.object_contracts.len() as u32,
            total_surfaces: self.surface_claims.len() as u32,
            green_count: count_signal(RunbookSignal::Green),
            yellow_count: count_signal(RunbookSignal::Yellow),
            red_count: count_signal(RunbookSignal::Red),
            mapped_surface_ids: by_status(RunbookSurfaceStatus::Mapped),
            provisional_surface_ids: by_status(RunbookSurfaceStatus::Provisional),
            unmapped_surface_ids: by_status(RunbookSurfaceStatus::Unmapped),
            narrowed_surface_ids: by_predicate(&|s| s.is_narrowed()),
            blocked_surface_ids: blocked_surface_ids.clone(),
            waived_surface_ids: by_predicate(&|s| !s.waivers.is_empty()),
            active_waiver_ids,
            stale_proof_object_classes: proof_classes(ProofFreshnessState::Stale),
            missing_proof_object_classes: proof_classes(ProofFreshnessState::Missing),
            blocks_stable_promotion: !blocked_surface_ids.is_empty(),
            coverage_gaps,
            source_contract_refs: self.source_contract_refs.clone(),
            matrix_message_id: format!("{}matrix", M5_RUNBOOK_MESSAGE_ID_PREFIX),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Validates the governance packet invariants.
    pub fn validate(&self) -> Vec<M5RunbookGovernanceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RUNBOOK_GOVERNANCE_RECORD_KIND {
            violations.push(M5RunbookGovernanceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_GOVERNANCE_SCHEMA_VERSION {
            violations.push(M5RunbookGovernanceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RunbookGovernanceViolation::MissingIdentity);
        }

        validate_object_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surfaces(self, &mut violations);
        validate_release_gate_aggregate(self, &mut violations);
        validate_matrix(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook governance serializes"),
        ) {
            violations.push(M5RunbookGovernanceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook governance serializes")
    }

    /// Deterministic export-safe JSON for the matrix projection.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only matrix fails.
    pub fn matrix_json(&self) -> String {
        serde_json::to_string_pretty(&self.matrix())
            .expect("m5 runbook governance matrix serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let matrix = self.matrix();
        let mut out = String::new();
        out.push_str("# M5 Runbook-Governance Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Governed objects: {} ({} schemas under `schemas/runbooks/`)\n",
            matrix.total_objects, 4
        ));
        out.push_str(&format!(
            "- Surfaces: {} ({} mapped, {} provisional, {} unmapped)\n",
            matrix.total_surfaces, matrix.green_count, matrix.yellow_count, matrix.red_count
        ));
        out.push_str(&format!(
            "- Release gate: {} ({} blocked, {} narrowed, {} governed)\n",
            if self.release_gate.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            },
            self.release_gate.blocked_surface_ids.len(),
            self.release_gate.narrowed_surface_ids.len(),
            self.release_gate.governed_surface_ids.len()
        ));
        out.push_str(&format!(
            "- Active waivers: {}\n",
            matrix.active_waiver_ids.len()
        ));

        out.push_str("\n## Governed runbook objects\n\n");
        out.push_str("| Object | Owner | First consumer | Source of truth | Proof | Freshness |\n");
        out.push_str("|--------|-------|----------------|-----------------|-------|-----------|\n");
        for contract in &self.object_contracts {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                contract.object_class.as_str(),
                contract.owner_role,
                contract.first_consumer.as_str(),
                contract.schema_ref,
                contract.proof_ref,
                contract.proof_freshness.as_str()
            ));
        }

        out.push_str("\n## Claimed runbook-backed surfaces\n\n");
        for surface in &self.surface_claims {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}` ({}), claim `{}` → `{}`, gate `{}`\n",
                surface.surface_id,
                surface.consumer.as_str(),
                surface.status.as_str(),
                surface.signal.as_str(),
                surface.claimed_class.as_str(),
                surface.effective_class.as_str(),
                surface.gate_decision.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", surface.owner_role));
            let bound: Vec<&str> = surface
                .bound_object_classes
                .iter()
                .map(|c| c.as_str())
                .collect();
            out.push_str(&format!("  - Binds: {}\n", bound.join(", ")));
            for gap in &surface.gaps {
                out.push_str(&format!(
                    "  - Gap: `{}` on `{}`{}\n",
                    gap.gap_kind.as_str(),
                    gap.object_class.as_str(),
                    if gap.waived { " (waived)" } else { "" }
                ));
            }
        }
        out
    }
}

/// Validation failures for the runbook governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookGovernanceViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A governed object class has no matrix contract.
    ObjectClassNotGoverned,
    /// A governed object contract is missing its owner, consumer, or proof.
    ObjectContractIncomplete,
    /// A governed object contract cites the wrong source-of-truth schema.
    ObjectContractWrongSchema,
    /// A claimed surface binds no governed objects.
    SurfaceBindsNoObjects,
    /// A claimed surface's stored verdict drifted from a fresh recompute.
    SurfaceVerdictDrift,
    /// A waiver is missing its scope, owner, or expiry.
    WaiverIncomplete,
    /// A waiver scopes a non-blocking gap kind.
    WaiverScopesNonBlockingGap,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The aggregate release gate disagrees with the per-surface gates.
    ReleaseGateAggregateMismatch,
    /// The matrix projection disagrees with the rows.
    MatrixMismatch,
    /// A conformance-review flag does not hold.
    ConformanceReviewFailed,
    /// A consumer-projection flag does not hold.
    ConsumerProjectionFailed,
    /// An embedded object record carries the wrong record kind or schema version.
    WrongObjectRecordKind,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// A source descriptor claims authority it does not carry.
    SourceAuthorityOverreach,
    /// A step's mutating flag disagrees with its step class.
    StepMutatingFlagMismatch,
    /// A step's boundary disagrees with its step class.
    StepBoundaryMismatch,
    /// A step or handoff would create a hidden privileged mutate channel.
    HiddenMutateChannel,
    /// A handoff that leaves the governed plane is unattributable.
    UnattributableHandoff,
    /// A recorded deviation is unattributable.
    UnattributableDeviation,
    /// An execution record declares no steps.
    ExecutionHasNoSteps,
    /// An execution record's rollups drifted from a fresh recompute.
    ExecutionRollupDrift,
    /// A companion drove a step outside its declared scope.
    CompanionScopeOverreach,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookGovernanceViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ObjectClassNotGoverned => "object_class_not_governed",
            Self::ObjectContractIncomplete => "object_contract_incomplete",
            Self::ObjectContractWrongSchema => "object_contract_wrong_schema",
            Self::SurfaceBindsNoObjects => "surface_binds_no_objects",
            Self::SurfaceVerdictDrift => "surface_verdict_drift",
            Self::WaiverIncomplete => "waiver_incomplete",
            Self::WaiverScopesNonBlockingGap => "waiver_scopes_non_blocking_gap",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ReleaseGateAggregateMismatch => "release_gate_aggregate_mismatch",
            Self::MatrixMismatch => "matrix_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::ConsumerProjectionFailed => "consumer_projection_failed",
            Self::WrongObjectRecordKind => "wrong_object_record_kind",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::SourceAuthorityOverreach => "source_authority_overreach",
            Self::StepMutatingFlagMismatch => "step_mutating_flag_mismatch",
            Self::StepBoundaryMismatch => "step_boundary_mismatch",
            Self::HiddenMutateChannel => "hidden_mutate_channel",
            Self::UnattributableHandoff => "unattributable_handoff",
            Self::UnattributableDeviation => "unattributable_deviation",
            Self::ExecutionHasNoSteps => "execution_has_no_steps",
            Self::ExecutionRollupDrift => "execution_rollup_drift",
            Self::CompanionScopeOverreach => "companion_scope_overreach",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

fn validate_object_contracts(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    let governed: BTreeSet<RunbookObjectClass> = packet
        .object_contracts
        .iter()
        .map(|c| c.object_class)
        .collect();
    for class in RunbookObjectClass::ALL {
        if !governed.contains(&class) {
            violations.push(M5RunbookGovernanceViolation::ObjectClassNotGoverned);
        }
    }
    for contract in &packet.object_contracts {
        if contract.object_label.trim().is_empty()
            || contract.owner_role.trim().is_empty()
            || contract.schema_ref.trim().is_empty()
            || contract.proof_ref.trim().is_empty()
            || contract.governed_vocab.is_empty()
        {
            violations.push(M5RunbookGovernanceViolation::ObjectContractIncomplete);
        }
        if contract.schema_ref != contract.object_class.schema_ref() {
            violations.push(M5RunbookGovernanceViolation::ObjectContractWrongSchema);
        }
        if !contract
            .detail_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
        {
            violations.push(M5RunbookGovernanceViolation::UnprefixedMessageId);
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RunbookGovernanceViolation::VocabularyMismatch);
    }
}

fn validate_surfaces(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    for surface in &packet.surface_claims {
        if surface.bound_object_classes.is_empty() {
            violations.push(M5RunbookGovernanceViolation::SurfaceBindsNoObjects);
        }
        if !surface
            .status_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
            || !surface
                .gate_message_id
                .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
        {
            violations.push(M5RunbookGovernanceViolation::UnprefixedMessageId);
        }

        // Recompute the verdict from the matrix contracts and compare.
        let mut probe = surface.clone();
        probe.recompute(&packet.object_contracts);
        if probe.gaps != surface.gaps
            || probe.status != surface.status
            || probe.signal != surface.signal
            || probe.gate_decision != surface.gate_decision
            || probe.effective_class != surface.effective_class
        {
            violations.push(M5RunbookGovernanceViolation::SurfaceVerdictDrift);
        }

        for waiver in &surface.waivers {
            if waiver.waiver_id.trim().is_empty()
                || waiver.owner_role.trim().is_empty()
                || waiver.expires_at.trim().is_empty()
                || !waiver
                    .reason_message_id
                    .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
            {
                violations.push(M5RunbookGovernanceViolation::WaiverIncomplete);
            }
            if !waiver.gap_kind.is_blocking() {
                violations.push(M5RunbookGovernanceViolation::WaiverScopesNonBlockingGap);
            }
        }
    }
}

fn validate_release_gate_aggregate(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    let sorted = |mut ids: Vec<String>| {
        ids.sort();
        ids
    };
    let blocked = sorted(
        packet
            .surface_claims
            .iter()
            .filter(|s| s.is_blocked())
            .map(|s| s.surface_id.clone())
            .collect(),
    );
    let narrowed = sorted(
        packet
            .surface_claims
            .iter()
            .filter(|s| s.is_narrowed())
            .map(|s| s.surface_id.clone())
            .collect(),
    );
    let governed = sorted(
        packet
            .surface_claims
            .iter()
            .filter(|s| s.is_governed())
            .map(|s| s.surface_id.clone())
            .collect(),
    );
    let waived = sorted(
        packet
            .surface_claims
            .iter()
            .filter(|s| !s.waivers.is_empty())
            .map(|s| s.surface_id.clone())
            .collect(),
    );

    let gate = &packet.release_gate;
    if gate.blocks_stable_promotion == blocked.is_empty()
        || sorted(gate.blocked_surface_ids.clone()) != blocked
        || sorted(gate.narrowed_surface_ids.clone()) != narrowed
        || sorted(gate.governed_surface_ids.clone()) != governed
        || sorted(gate.waived_surface_ids.clone()) != waived
        || !gate
            .gate_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX)
    {
        violations.push(M5RunbookGovernanceViolation::ReleaseGateAggregateMismatch);
    }
}

fn validate_matrix(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    let matrix = packet.matrix();
    let green = packet
        .surface_claims
        .iter()
        .filter(|s| s.signal == RunbookSignal::Green)
        .count() as u32;
    let yellow = packet
        .surface_claims
        .iter()
        .filter(|s| s.signal == RunbookSignal::Yellow)
        .count() as u32;
    let red = packet
        .surface_claims
        .iter()
        .filter(|s| s.signal == RunbookSignal::Red)
        .count() as u32;
    if matrix.green_count != green
        || matrix.yellow_count != yellow
        || matrix.red_count != red
        || matrix.total_surfaces != packet.surface_claims.len() as u32
        || matrix.total_objects != packet.object_contracts.len() as u32
        || matrix.blocks_stable_promotion != packet.blocks_stable_promotion()
    {
        violations.push(M5RunbookGovernanceViolation::MatrixMismatch);
    }
}

fn validate_conformance_review(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    if !packet.conformance_review.all_hold() {
        violations.push(M5RunbookGovernanceViolation::ConformanceReviewFailed);
    }
}

fn validate_consumer_projection(
    packet: &M5RunbookGovernancePacket,
    violations: &mut Vec<M5RunbookGovernanceViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(M5RunbookGovernanceViolation::ConsumerProjectionFailed);
    }
}

/// Keys and substrings that must never appear in an export-safe packet. Mirrors
/// the redaction posture of the other governed lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden boundary material. Returns true when a
/// key (case-insensitive) contains a forbidden substring.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_boundary_material(child)
        }),
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
