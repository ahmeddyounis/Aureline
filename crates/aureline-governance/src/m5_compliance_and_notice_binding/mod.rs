//! Typed register of repository-compliance and notice-binding truth per M5 subject.
//!
//! The sibling [`m5_boundary_and_upstream_durability`](crate::m5_boundary_and_upstream_durability)
//! matrix records, per asset lane, *whether* a repository-compliance control is
//! satisfied as one coarse [`ControlState`](crate::m5_boundary_and_upstream_durability::ControlState).
//! It answers *is the lane durable right now?* — but it collapses contribution
//! provenance, file-level licensing, and SBOM/notice hygiene into a single satisfied/
//! unsatisfied flag and does not, per claimed M5 artifact family, docs pack, and
//! mirrored output, publish the inspectable compliance truth a contributor, admin, or
//! procurement reviewer reads off the product.
//!
//! This module is that compliance-truth layer. For every claimed M5 subject it records
//! one [`ComplianceRecord`] that states, in one copy-safe record:
//!
//! - the **DCO/CLA contribution-provenance** lane truth ([`ContributionProvenance`]):
//!   whether every contribution is signed off, whether the contributor agreement is on
//!   file, and how many commits still lack provenance;
//! - the **REUSE/SPDX file-level licensing** coverage ([`LicensingCoverage`]): how many
//!   files carry SPDX/REUSE licensing, how many gaps are covered by a *documented*
//!   exception, and whether any exception is undocumented;
//! - the **notice inventory** state ([`NoticeInventory`]): whether the third-party
//!   notice inventory is complete, partial, or missing — surfaced as a first-class
//!   state, never hidden behind a green SBOM badge;
//! - the **SBOM/notice binding** ([`SbomNoticeBinding`]): whether the SPDX primary SBOM
//!   is present, whether the CycloneDX export is available, and whether the SBOM is
//!   actually bound to the notice inventory;
//! - the **mirror/offline binding** ([`MirrorBinding`]): whether the compliance
//!   artifacts are mirrored and whether that mirror is fresh.
//!
//! Each record also carries a [`scan_posture`](ComplianceRecord::scan_posture) (what the
//! repository-compliance scan found) and a [`surface_posture`](ComplianceRecord::surface_posture)
//! (what the user/admin notice/SBOM surface shows). The two **must agree**: a record may
//! never show a clean surface over a scan that found gaps, so a green SBOM can never mask
//! a missing notice or a licensing gap.
//!
//! A record is [`ComplianceState::Cleared`] only when provenance holds, licensing
//! coverage is complete, the notice inventory is complete, the SBOM is present and bound,
//! the mirror is fresh, the proof is fresh, and the owner signed. Otherwise it narrows on
//! the *specific* axis that thinned out — a provenance gap, a licensing gap, a notice gap,
//! an SBOM/binding gap, a stale mirror, or stale proof — never collapsing to one global
//! flag. A narrowed record drops its [`ComplianceRecord::effective_label`] below the
//! launch cutline and may never publish an effective label wider than the one it declares.
//!
//! The [`ComplianceRule`] set names the closed conditions that gate promotion. An
//! *inherited* narrowing — a subject whose declared label already sits below the cutline,
//! or a gap held by an unexpired waiver — is gated upstream and does not itself hold
//! promotion; a *compliance-layer* failure on a subject whose declared label is still at
//! or above the cutline holds promotion through a stop rule, recorded in
//! [`ComplianceRegister::publication`]. The cross-cutting [`ScanSurfaceParity`] block
//! summarizes scan/surface agreement over every subject.
//!
//! The register is checked in at
//! `artifacts/governance/m5-compliance-and-notice-binding.json` and embedded here, so this
//! typed consumer and the CI gate agree on every record without a cargo build in CI. The
//! model is metadata-only: every field is a typed state, a boolean flag, a small count, a
//! label, or an opaque ref. It carries no credential bodies, raw provider payloads,
//! signatures, or SBOM contents. Date arithmetic (recomputing proof, mirror, and waiver
//! freshness against an `as_of` date) lives in the CI gate and the integration test; this
//! model enforces the invariants that hold regardless of the clock: scan/surface parity,
//! the no-widening ceiling, control/fact consistency, reason/state coherence, summary
//! agreement, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket, SupportClass, Waiver,
};
use crate::m5_versioned_boundary_manifests::M5Family;

/// Supported register schema version.
pub const M5_COMPLIANCE_AND_NOTICE_BINDING_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_COMPLIANCE_AND_NOTICE_BINDING_RECORD_KIND: &str =
    "m5_compliance_and_notice_binding_register";

/// Repo-relative path to the checked-in register.
pub const M5_COMPLIANCE_AND_NOTICE_BINDING_PATH: &str =
    "artifacts/governance/m5-compliance-and-notice-binding.json";

/// Embedded checked-in register JSON.
pub const M5_COMPLIANCE_AND_NOTICE_BINDING_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-compliance-and-notice-binding.json"
));

/// The kind of subject a compliance record governs.
///
/// The same compliance truth is published for shipped artifact families, the docs packs
/// that document them, and the mirrored/offline outputs that redistribute them — so a gap
/// on a docs pack or a mirror cannot hide behind a clean artifact family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// A shipped M5 artifact family.
    ArtifactFamily,
    /// A documentation pack for a family.
    DocsPack,
    /// A mirrored/offline redistribution of a family's artifacts.
    MirroredOutput,
}

impl ScopeKind {
    /// Every scope kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::ArtifactFamily, Self::DocsPack, Self::MirroredOutput];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactFamily => "artifact_family",
            Self::DocsPack => "docs_pack",
            Self::MirroredOutput => "mirrored_output",
        }
    }
}

/// DCO (Developer Certificate of Origin) sign-off lane truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DcoState {
    /// Every contribution carries a DCO sign-off.
    AllSigned,
    /// One or more contributions lack a DCO sign-off.
    GapsPresent,
    /// DCO does not apply to this subject (no inbound contribution path).
    NotRequired,
}

impl DcoState {
    /// Every DCO state, in declaration order.
    pub const ALL: [Self; 3] = [Self::AllSigned, Self::GapsPresent, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllSigned => "all_signed",
            Self::GapsPresent => "gaps_present",
            Self::NotRequired => "not_required",
        }
    }
}

/// CLA (Contributor License Agreement) lane truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaState {
    /// The contributor agreement is on file for every contributor.
    OnFile,
    /// One or more contributors have an unresolved agreement.
    Unresolved,
    /// A contributor agreement does not apply to this subject.
    NotRequired,
}

impl ClaState {
    /// Every CLA state, in declaration order.
    pub const ALL: [Self; 3] = [Self::OnFile, Self::Unresolved, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnFile => "on_file",
            Self::Unresolved => "unresolved",
            Self::NotRequired => "not_required",
        }
    }
}

/// Third-party notice-inventory state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeState {
    /// Every required third-party notice is present.
    Complete,
    /// Some required notices are present, some are missing.
    Partial,
    /// No notice inventory is captured.
    Missing,
    /// A notice inventory does not apply to this subject.
    NotRequired,
}

impl NoticeState {
    /// Every notice state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Complete,
        Self::Partial,
        Self::Missing,
        Self::NotRequired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::NotRequired => "not_required",
        }
    }
}

/// The binding state between the SBOM and the notice inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SbomBindingState {
    /// The SBOM is bound to the notice inventory.
    Bound,
    /// The SBOM exists but is not bound to a notice inventory.
    Unbound,
    /// SBOM binding does not apply to this subject.
    NotApplicable,
}

impl SbomBindingState {
    /// Every binding state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Bound, Self::Unbound, Self::NotApplicable];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::Unbound => "unbound",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// An SBOM output format the subject publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SbomFormat {
    /// SPDX is the primary, canonical SBOM output.
    SpdxPrimary,
    /// CycloneDX is offered as an export.
    CyclonedxExport,
}

impl SbomFormat {
    /// Every SBOM format, in declaration order.
    pub const ALL: [Self; 2] = [Self::SpdxPrimary, Self::CyclonedxExport];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpdxPrimary => "spdx_primary",
            Self::CyclonedxExport => "cyclonedx_export",
        }
    }
}

/// The compliance posture a scan or a surface reports for a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompliancePosture {
    /// No compliance gap found.
    Clear,
    /// One or more compliance gaps found.
    GapsFound,
}

impl CompliancePosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::Clear, Self::GapsFound];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::GapsFound => "gaps_found",
        }
    }
}

/// A repository-compliance control dimension a record must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// DCO/CLA contribution provenance.
    ContributionProvenance,
    /// REUSE/SPDX file-level licensing coverage.
    FileLevelLicensing,
    /// Third-party notice inventory completeness.
    NoticeInventory,
    /// SBOM presence and SBOM/notice binding.
    SbomNoticeBinding,
    /// Mirror/offline freshness.
    MirrorFreshness,
    /// Scan/surface parity: the scan and the user/admin surface agree.
    ScanSurfaceParity,
}

impl ControlDimension {
    /// Every control dimension, in declaration order. Every record declares each once.
    pub const ALL: [Self; 6] = [
        Self::ContributionProvenance,
        Self::FileLevelLicensing,
        Self::NoticeInventory,
        Self::SbomNoticeBinding,
        Self::MirrorFreshness,
        Self::ScanSurfaceParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContributionProvenance => "contribution_provenance",
            Self::FileLevelLicensing => "file_level_licensing",
            Self::NoticeInventory => "notice_inventory",
            Self::SbomNoticeBinding => "sbom_notice_binding",
            Self::MirrorFreshness => "mirror_freshness",
            Self::ScanSurfaceParity => "scan_surface_parity",
        }
    }
}

/// Satisfaction state of one control binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The control holds for this subject.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this subject.
    NotApplicable,
}

impl ControlState {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The state a record earns after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceState {
    /// Provenance, licensing, notices, SBOM, mirror, and proof all hold.
    Cleared,
    /// A DCO/CLA contribution-provenance gap is present.
    NarrowedProvenance,
    /// A REUSE/SPDX file-level licensing gap is present.
    NarrowedLicensing,
    /// The notice inventory is partial or missing.
    NarrowedNotice,
    /// The SBOM is missing, unbound, or its required export is unavailable.
    NarrowedSbom,
    /// The compliance mirror/offline pack is stale.
    NarrowedMirror,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The subject is withdrawn.
    Withdrawn,
}

impl ComplianceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Cleared,
        Self::NarrowedProvenance,
        Self::NarrowedLicensing,
        Self::NarrowedNotice,
        Self::NarrowedSbom,
        Self::NarrowedMirror,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cleared => "cleared",
            Self::NarrowedProvenance => "narrowed_provenance",
            Self::NarrowedLicensing => "narrowed_licensing",
            Self::NarrowedNotice => "narrowed_notice",
            Self::NarrowedSbom => "narrowed_sbom",
            Self::NarrowedMirror => "narrowed_mirror",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not cleared, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Cleared | Self::Withdrawn)
    }
}

/// A reason a record narrowed. Closed vocabulary; every reason is watched by a
/// [`ComplianceRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceReason {
    /// One or more contributions lack a DCO sign-off.
    DcoSignoffMissing,
    /// A contributor agreement is unresolved.
    ClaUnresolved,
    /// File-level SPDX/REUSE coverage is incomplete.
    LicensingCoverageIncomplete,
    /// A licensing exception is undocumented.
    LicenseExceptionUndocumented,
    /// The notice inventory is partial.
    NoticeInventoryPartial,
    /// The notice inventory is missing.
    NoticeInventoryMissing,
    /// The SPDX primary SBOM is missing.
    SbomPrimaryMissing,
    /// The SBOM is not bound to the notice inventory.
    SbomNoticeBindingBroken,
    /// The required CycloneDX export is unavailable.
    CyclonedxExportUnavailable,
    /// The compliance mirror/offline pack is stale.
    MirrorStale,
    /// The proof packet aged past its freshness SLO.
    ComplianceProofStale,
    /// No proof packet is captured.
    ComplianceProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl ComplianceReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::DcoSignoffMissing,
        Self::ClaUnresolved,
        Self::LicensingCoverageIncomplete,
        Self::LicenseExceptionUndocumented,
        Self::NoticeInventoryPartial,
        Self::NoticeInventoryMissing,
        Self::SbomPrimaryMissing,
        Self::SbomNoticeBindingBroken,
        Self::CyclonedxExportUnavailable,
        Self::MirrorStale,
        Self::ComplianceProofStale,
        Self::ComplianceProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DcoSignoffMissing => "dco_signoff_missing",
            Self::ClaUnresolved => "cla_unresolved",
            Self::LicensingCoverageIncomplete => "licensing_coverage_incomplete",
            Self::LicenseExceptionUndocumented => "license_exception_undocumented",
            Self::NoticeInventoryPartial => "notice_inventory_partial",
            Self::NoticeInventoryMissing => "notice_inventory_missing",
            Self::SbomPrimaryMissing => "sbom_primary_missing",
            Self::SbomNoticeBindingBroken => "sbom_notice_binding_broken",
            Self::CyclonedxExportUnavailable => "cyclonedx_export_unavailable",
            Self::MirrorStale => "mirror_stale",
            Self::ComplianceProofStale => "compliance_proof_stale",
            Self::ComplianceProofMissing => "compliance_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            ComplianceState::NarrowedProvenance => 0,
            ComplianceState::NarrowedLicensing => 1,
            ComplianceState::NarrowedNotice => 2,
            ComplianceState::NarrowedSbom => 3,
            ComplianceState::NarrowedMirror => 4,
            _ => 5,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> ComplianceState {
        match self {
            Self::DcoSignoffMissing | Self::ClaUnresolved => ComplianceState::NarrowedProvenance,
            Self::LicensingCoverageIncomplete | Self::LicenseExceptionUndocumented => {
                ComplianceState::NarrowedLicensing
            }
            Self::NoticeInventoryPartial | Self::NoticeInventoryMissing => {
                ComplianceState::NarrowedNotice
            }
            Self::SbomPrimaryMissing
            | Self::SbomNoticeBindingBroken
            | Self::CyclonedxExportUnavailable => ComplianceState::NarrowedSbom,
            Self::MirrorStale => ComplianceState::NarrowedMirror,
            Self::ComplianceProofStale
            | Self::ComplianceProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ComplianceState::NarrowedStale,
        }
    }

    /// The control dimension this reason belongs to.
    pub const fn dimension(self) -> ControlDimension {
        match self {
            Self::DcoSignoffMissing | Self::ClaUnresolved => {
                ControlDimension::ContributionProvenance
            }
            Self::LicensingCoverageIncomplete | Self::LicenseExceptionUndocumented => {
                ControlDimension::FileLevelLicensing
            }
            Self::NoticeInventoryPartial | Self::NoticeInventoryMissing => {
                ControlDimension::NoticeInventory
            }
            Self::SbomPrimaryMissing
            | Self::SbomNoticeBindingBroken
            | Self::CyclonedxExportUnavailable => ControlDimension::SbomNoticeBinding,
            Self::MirrorStale => ControlDimension::MirrorFreshness,
            Self::ComplianceProofStale
            | Self::ComplianceProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ControlDimension::ScanSurfaceParity,
        }
    }
}

/// An action a [`ComplianceRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Collect the missing DCO sign-off.
    CollectDcoSignoff,
    /// Resolve the contributor agreement.
    ResolveCla,
    /// Complete the file-level licensing coverage.
    CompleteLicensingCoverage,
    /// Document the licensing exception.
    DocumentLicenseException,
    /// Complete the notice inventory.
    CompleteNoticeInventory,
    /// Generate the SPDX primary SBOM.
    GenerateSpdxSbom,
    /// Rebind the SBOM to the notice inventory.
    RebindSbomNotices,
    /// Enable the CycloneDX export.
    EnableCyclonedxExport,
    /// Refresh the compliance mirror/offline pack.
    RefreshMirror,
    /// Refresh the compliance proof packet.
    RefreshComplianceProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl ComplianceAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::HoldPromotion,
        Self::CollectDcoSignoff,
        Self::ResolveCla,
        Self::CompleteLicensingCoverage,
        Self::DocumentLicenseException,
        Self::CompleteNoticeInventory,
        Self::GenerateSpdxSbom,
        Self::RebindSbomNotices,
        Self::EnableCyclonedxExport,
        Self::RefreshMirror,
        Self::RefreshComplianceProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::CollectDcoSignoff => "collect_dco_signoff",
            Self::ResolveCla => "resolve_cla",
            Self::CompleteLicensingCoverage => "complete_licensing_coverage",
            Self::DocumentLicenseException => "document_license_exception",
            Self::CompleteNoticeInventory => "complete_notice_inventory",
            Self::GenerateSpdxSbom => "generate_spdx_sbom",
            Self::RebindSbomNotices => "rebind_sbom_notices",
            Self::EnableCyclonedxExport => "enable_cyclonedx_export",
            Self::RefreshMirror => "refresh_mirror",
            Self::RefreshComplianceProof => "refresh_compliance_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No compliance-layer stop rule fires; promotion may proceed.
    Proceed,
    /// A compliance-layer stop rule fires; hold promotion.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// DCO/CLA contribution-provenance lane truth for a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContributionProvenance {
    /// DCO sign-off state.
    pub dco_state: DcoState,
    /// Contributor-agreement state.
    pub cla_state: ClaState,
    /// Number of commits still lacking a DCO sign-off.
    pub unsigned_commit_count: u32,
    /// Reference to the DCO/merge audit.
    pub dco_audit_ref: String,
    /// Reference to the contributor-agreement register.
    pub cla_register_ref: String,
}

impl ContributionProvenance {
    /// True when one or more contributions lack a DCO sign-off.
    pub fn dco_gap(&self) -> bool {
        self.dco_state == DcoState::GapsPresent
    }

    /// True when a contributor agreement is unresolved.
    pub fn cla_gap(&self) -> bool {
        self.cla_state == ClaState::Unresolved
    }

    /// True when contribution provenance does not apply to this subject.
    pub fn not_applicable(&self) -> bool {
        self.dco_state == DcoState::NotRequired && self.cla_state == ClaState::NotRequired
    }
}

/// REUSE/SPDX file-level licensing coverage for a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicensingCoverage {
    /// Total files in scope.
    pub files_total: u32,
    /// Files carrying SPDX/REUSE licensing.
    pub files_spdx_covered: u32,
    /// Files covered by a documented licensing exception.
    pub documented_exceptions: u32,
    /// Files with an undocumented licensing exception.
    pub undocumented_exceptions: u32,
    /// Reference to the REUSE/SPDX coverage report.
    pub reuse_report_ref: String,
}

impl LicensingCoverage {
    /// True when SPDX coverage plus documented exceptions do not account for every file.
    pub fn coverage_incomplete(&self) -> bool {
        self.files_spdx_covered + self.documented_exceptions < self.files_total
    }

    /// True when any licensing exception is undocumented.
    pub fn exception_undocumented(&self) -> bool {
        self.undocumented_exceptions > 0
    }
}

/// Third-party notice-inventory state for a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoticeInventory {
    /// Inventory state.
    pub notice_state: NoticeState,
    /// Total notices required.
    pub entries_total: u32,
    /// Notices present.
    pub entries_present: u32,
    /// Reference to the notice inventory artifact.
    pub notice_inventory_ref: String,
}

impl NoticeInventory {
    /// True when the inventory is partial.
    pub fn is_partial(&self) -> bool {
        self.notice_state == NoticeState::Partial
    }

    /// True when no inventory is captured.
    pub fn is_missing(&self) -> bool {
        self.notice_state == NoticeState::Missing
    }

    /// True when a notice inventory does not apply to this subject.
    pub fn not_applicable(&self) -> bool {
        self.notice_state == NoticeState::NotRequired
    }
}

/// SBOM presence and SBOM/notice binding for a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SbomNoticeBinding {
    /// True when the SPDX primary SBOM is present.
    pub spdx_primary_present: bool,
    /// True when the CycloneDX export is available.
    pub cyclonedx_export_available: bool,
    /// Binding state between the SBOM and the notice inventory.
    pub binding_state: SbomBindingState,
    /// SBOM formats this subject publishes.
    pub formats: Vec<SbomFormat>,
    /// Reference to the SBOM artifact.
    pub sbom_ref: String,
    /// Reference to the SBOM/notice binding record.
    pub notice_binding_ref: String,
}

impl SbomNoticeBinding {
    /// True when the SPDX primary SBOM is missing.
    pub fn primary_missing(&self) -> bool {
        !self.spdx_primary_present
    }

    /// True when the SBOM is not bound to a notice inventory.
    pub fn binding_broken(&self) -> bool {
        self.binding_state == SbomBindingState::Unbound
    }
}

/// Mirror/offline binding for a subject's compliance artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorBinding {
    /// True when the compliance artifacts must be mirrored for offline use.
    pub mirror_required: bool,
    /// Freshness state of the mirror.
    pub mirror_freshness: FreshnessSloState,
    /// Reference to the mirror/offline pack.
    pub mirror_ref: String,
}

impl MirrorBinding {
    /// True when a required mirror has aged past its freshness window.
    pub fn is_stale(&self) -> bool {
        self.mirror_required && self.mirror_freshness == FreshnessSloState::Breached
    }
}

/// One repository-compliance control binding on a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceControl {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// One repository-compliance and notice-binding record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceRecord {
    /// Stable record id.
    pub record_id: String,
    /// The M5 family this record governs.
    pub family: M5Family,
    /// The kind of subject.
    pub scope_kind: ScopeKind,
    /// Human-readable title.
    pub title: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this subject is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this record declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the subject.
    pub support_class: SupportClass,
    /// DCO/CLA contribution-provenance lane truth.
    pub provenance: ContributionProvenance,
    /// REUSE/SPDX file-level licensing coverage.
    pub licensing: LicensingCoverage,
    /// Notice-inventory state.
    pub notices: NoticeInventory,
    /// SBOM presence and SBOM/notice binding.
    pub sbom: SbomNoticeBinding,
    /// Mirror/offline binding.
    pub mirror: MirrorBinding,
    /// Per-dimension control bindings.
    pub controls: Vec<ComplianceControl>,
    /// What the repository-compliance scan found.
    pub scan_posture: CompliancePosture,
    /// What the user/admin notice/SBOM surface shows.
    pub surface_posture: CompliancePosture,
    /// Reference to the compliance scan.
    pub scan_ref: String,
    /// Reference to the user/admin surface.
    pub surface_ref: String,
    /// Proof packet grounding the record.
    pub proof_packet: ProofPacket,
    /// Optional waiver holding a gap provisionally.
    pub waiver: Option<Waiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// State earned after narrowing.
    pub compliance_state: ComplianceState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ComplianceReason>,
    /// The label the record effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this record (Help/About, docs, support/eval packets).
    pub surfaces: Vec<String>,
    /// Reviewable reason the record carries its state.
    pub rationale: String,
}

impl ComplianceRecord {
    /// True when the record is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(ComplianceReason::WaiverExpired)
    }

    /// True when the record carries the given active reason.
    pub fn has_active_reason(&self, reason: ComplianceReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the record holds a cleared state.
    pub fn is_cleared(&self) -> bool {
        self.compliance_state == ComplianceState::Cleared
    }

    /// True when the subject declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// True when the required CycloneDX export is unavailable.
    pub fn cyclonedx_gap(&self) -> bool {
        self.release_blocking && !self.sbom.cyclonedx_export_available
    }

    /// True when any compliance gap (other than proof/sign-off) is present.
    pub fn has_compliance_gap(&self) -> bool {
        self.provenance.dco_gap()
            || self.provenance.cla_gap()
            || self.licensing.coverage_incomplete()
            || self.licensing.exception_undocumented()
            || self.notices.is_partial()
            || self.notices.is_missing()
            || self.sbom.primary_missing()
            || self.sbom.binding_broken()
            || self.cyclonedx_gap()
            || self.mirror.is_stale()
    }

    /// The expected control state for a dimension, derived from the subject's facts.
    pub fn expected_control_state(&self, dimension: ControlDimension) -> ControlState {
        match dimension {
            ControlDimension::ContributionProvenance => {
                if self.provenance.not_applicable() {
                    ControlState::NotApplicable
                } else if self.provenance.dco_gap() || self.provenance.cla_gap() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::FileLevelLicensing => {
                if self.licensing.coverage_incomplete() || self.licensing.exception_undocumented() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::NoticeInventory => {
                if self.notices.not_applicable() {
                    ControlState::NotApplicable
                } else if self.notices.is_partial() || self.notices.is_missing() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::SbomNoticeBinding => {
                if self.sbom.binding_state == SbomBindingState::NotApplicable {
                    ControlState::NotApplicable
                } else if self.sbom.primary_missing()
                    || self.sbom.binding_broken()
                    || self.cyclonedx_gap()
                {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::MirrorFreshness => {
                if !self.mirror.mirror_required {
                    ControlState::NotApplicable
                } else if self.mirror.is_stale() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::ScanSurfaceParity => {
                if self.scan_posture != self.surface_posture {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
        }
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> ComplianceState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return ComplianceState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => ComplianceState::Cleared,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            ComplianceState::Cleared => self.declared_label,
            ComplianceState::Withdrawn => LifecycleLabel::Withdrawn,
            _ => {
                // Narrowing drops the subject below the cutline: take the
                // less-supported of the declared label and beta.
                if self.declared_label.rank() <= LifecycleLabel::Beta.rank() {
                    self.declared_label
                } else {
                    LifecycleLabel::Beta
                }
            }
        }
    }

    /// The posture implied by the record's state: gaps found iff narrowed.
    pub fn computed_posture(&self) -> CompliancePosture {
        if self.compliance_state.is_narrowed() {
            CompliancePosture::GapsFound
        } else {
            CompliancePosture::Clear
        }
    }

    /// True when the record may hold promotion: a release-blocking subject, narrowed by a
    /// compliance-layer gap, declaring a label at or above the cutline, and not held by an
    /// unexpired waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.compliance_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }

    /// True when the scan and the surface agree.
    pub fn scan_surface_agree(&self) -> bool {
        self.scan_posture == self.surface_posture
    }
}

/// A closed stop-rule that gates promotion on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: ComplianceReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: ComplianceAction,
    /// True when the rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceCutline {
    /// The cutline level (`stable`).
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Description.
    pub description: String,
}

/// Canonical source registers this register binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContractRefs {
    /// Contribution-governance / DCO-CLA register.
    pub contribution_governance_ref: String,
    /// REUSE/SPDX file-level licensing report.
    pub reuse_spdx_report_ref: String,
    /// Third-party notice inventory.
    pub notice_inventory_ref: String,
    /// SBOM index.
    pub sbom_index_ref: String,
    /// Mirror/offline index.
    pub mirror_index_ref: String,
    /// Open/local-boundary and upstream-durability matrix.
    pub durability_matrix_ref: String,
    /// Canonical M5 evidence index.
    pub m5_evidence_index_ref: String,
}

/// Promotion verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// Stable promotion-gate id.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PublicationDecision,
    /// Firing rule ids.
    pub blocking_rule_ids: Vec<String>,
    /// Offending record ids.
    pub blocking_record_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Cross-cutting scan/surface parity summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanSurfaceParity {
    /// Stable parity-gate id.
    pub parity_gate: String,
    /// Total subjects.
    pub subjects_total: usize,
    /// Subjects whose scan and surface agree.
    pub subjects_in_agreement: usize,
    /// Subjects whose scan and surface disagree.
    pub subjects_in_disagreement: usize,
    /// Subjects whose surface reports gaps found.
    pub subjects_with_gaps: usize,
    /// True when every subject's scan and surface agree.
    pub all_subjects_agree: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceSummary {
    /// Total records.
    pub total_records: usize,
    /// Cleared records.
    pub records_cleared: usize,
    /// Narrowed records.
    pub records_narrowed: usize,
    /// Records in the `cleared` state.
    pub state_cleared: usize,
    /// Records in the `narrowed_provenance` state.
    pub state_narrowed_provenance: usize,
    /// Records in the `narrowed_licensing` state.
    pub state_narrowed_licensing: usize,
    /// Records in the `narrowed_notice` state.
    pub state_narrowed_notice: usize,
    /// Records in the `narrowed_sbom` state.
    pub state_narrowed_sbom: usize,
    /// Records in the `narrowed_mirror` state.
    pub state_narrowed_mirror: usize,
    /// Records in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Records in the `withdrawn` state.
    pub state_withdrawn: usize,
    /// Release-blocking records.
    pub release_blocking_total: usize,
    /// Release-blocking records that are narrowed.
    pub release_blocking_narrowed: usize,
    /// Records held by an active waiver.
    pub records_on_active_waiver: usize,
    /// Records carrying a DCO/CLA provenance gap.
    pub provenance_gaps: usize,
    /// Records carrying a file-level licensing gap.
    pub licensing_gaps: usize,
    /// Records whose notice inventory is partial or missing.
    pub notice_gaps: usize,
    /// Records carrying an SBOM/binding/export gap.
    pub sbom_gaps: usize,
    /// Records whose mirror is stale.
    pub mirror_gaps: usize,
    /// Records whose SPDX primary SBOM is present.
    pub spdx_primary_present: usize,
    /// Records whose CycloneDX export is available.
    pub cyclonedx_export_available: usize,
    /// Records whose notice inventory is complete.
    pub notices_complete: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of repository-compliance and notice-binding records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register id.
    pub register_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Date the register was last reconciled.
    pub as_of: String,
    /// Canonical source registers.
    pub source_contract_refs: SourceContractRefs,
    /// Launch cutline.
    pub compliance_cutline: ComplianceCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed scope-kind vocabulary.
    pub scope_kinds: Vec<ScopeKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed DCO-state vocabulary.
    pub dco_states: Vec<DcoState>,
    /// Closed CLA-state vocabulary.
    pub cla_states: Vec<ClaState>,
    /// Closed notice-state vocabulary.
    pub notice_states: Vec<NoticeState>,
    /// Closed SBOM-binding-state vocabulary.
    pub sbom_binding_states: Vec<SbomBindingState>,
    /// Closed SBOM-format vocabulary.
    pub sbom_formats: Vec<SbomFormat>,
    /// Closed posture vocabulary.
    pub postures: Vec<CompliancePosture>,
    /// Closed compliance-state vocabulary.
    pub compliance_states: Vec<ComplianceState>,
    /// Closed compliance-reason vocabulary.
    pub compliance_reasons: Vec<ComplianceReason>,
    /// Closed compliance-action vocabulary.
    pub compliance_actions: Vec<ComplianceAction>,
    /// Stop rules.
    pub rules: Vec<ComplianceRule>,
    /// Per-subject records.
    pub records: Vec<ComplianceRecord>,
    /// Cross-cutting scan/surface parity summary.
    pub scan_surface_parity: ScanSurfaceParity,
    /// Promotion verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: ComplianceSummary,
}

impl ComplianceRegister {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&ComplianceRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Returns the artifact-family record for a family.
    pub fn artifact_family_record(&self, family: M5Family) -> Option<&ComplianceRecord> {
        self.records
            .iter()
            .find(|r| r.family == family && r.scope_kind == ScopeKind::ArtifactFamily)
    }

    /// Returns the cleared records.
    pub fn records_cleared(&self) -> Vec<&ComplianceRecord> {
        self.records.iter().filter(|r| r.is_cleared()).collect()
    }

    /// Returns the narrowed records.
    pub fn records_narrowed(&self) -> Vec<&ComplianceRecord> {
        self.records
            .iter()
            .filter(|r| r.compliance_state.is_narrowed())
            .collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: ComplianceReason) -> Option<&ComplianceRule> {
        self.rules.iter().find(|rule| rule.trigger_reason == reason)
    }

    /// Recomputes the firing rule ids: a blocking rule fires when a promotion-holding
    /// record carries its trigger reason at an applicable label.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if !rule.blocks_promotion {
                continue;
            }
            let fires = self.records.iter().any(|r| {
                r.holds_promotion()
                    && r.has_active_reason(rule.trigger_reason)
                    && rule.applies_to_labels.contains(&r.declared_label)
            });
            if fires {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the offending record ids: promotion-holding records carrying a reason
    /// watched by a firing blocking rule.
    pub fn computed_blocking_record_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for r in &self.records {
            if !r.holds_promotion() {
                continue;
            }
            let blocked = r.active_reasons.iter().any(|reason| {
                self.rule_for(*reason).is_some_and(|rule| {
                    rule.blocks_promotion && rule.applies_to_labels.contains(&r.declared_label)
                })
            });
            if blocked {
                ids.insert(r.record_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the promotion decision.
    pub fn computed_decision(&self) -> PublicationDecision {
        if self.computed_blocking_record_ids().is_empty() {
            PublicationDecision::Proceed
        } else {
            PublicationDecision::Hold
        }
    }

    /// Recomputes the cross-cutting scan/surface parity summary.
    pub fn computed_scan_surface_parity(&self) -> ScanSurfaceParity {
        ScanSurfaceParity {
            parity_gate: self.scan_surface_parity.parity_gate.clone(),
            subjects_total: self.records.len(),
            subjects_in_agreement: self
                .records
                .iter()
                .filter(|r| r.scan_surface_agree())
                .count(),
            subjects_in_disagreement: self
                .records
                .iter()
                .filter(|r| !r.scan_surface_agree())
                .count(),
            subjects_with_gaps: self
                .records
                .iter()
                .filter(|r| r.surface_posture == CompliancePosture::GapsFound)
                .count(),
            all_subjects_agree: self.records.iter().all(|r| r.scan_surface_agree()),
            rationale: self.scan_surface_parity.rationale.clone(),
        }
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> ComplianceSummary {
        let count_state = |state: ComplianceState| {
            self.records
                .iter()
                .filter(|r| r.compliance_state == state)
                .count()
        };
        ComplianceSummary {
            total_records: self.records.len(),
            records_cleared: self.records_cleared().len(),
            records_narrowed: self.records_narrowed().len(),
            state_cleared: count_state(ComplianceState::Cleared),
            state_narrowed_provenance: count_state(ComplianceState::NarrowedProvenance),
            state_narrowed_licensing: count_state(ComplianceState::NarrowedLicensing),
            state_narrowed_notice: count_state(ComplianceState::NarrowedNotice),
            state_narrowed_sbom: count_state(ComplianceState::NarrowedSbom),
            state_narrowed_mirror: count_state(ComplianceState::NarrowedMirror),
            state_narrowed_stale: count_state(ComplianceState::NarrowedStale),
            state_withdrawn: count_state(ComplianceState::Withdrawn),
            release_blocking_total: self.records.iter().filter(|r| r.release_blocking).count(),
            release_blocking_narrowed: self
                .records
                .iter()
                .filter(|r| r.release_blocking && r.compliance_state.is_narrowed())
                .count(),
            records_on_active_waiver: self.records.iter().filter(|r| r.is_waived()).count(),
            provenance_gaps: self
                .records
                .iter()
                .filter(|r| r.provenance.dco_gap() || r.provenance.cla_gap())
                .count(),
            licensing_gaps: self
                .records
                .iter()
                .filter(|r| {
                    r.licensing.coverage_incomplete() || r.licensing.exception_undocumented()
                })
                .count(),
            notice_gaps: self
                .records
                .iter()
                .filter(|r| r.notices.is_partial() || r.notices.is_missing())
                .count(),
            sbom_gaps: self
                .records
                .iter()
                .filter(|r| {
                    r.sbom.primary_missing() || r.sbom.binding_broken() || r.cyclonedx_gap()
                })
                .count(),
            mirror_gaps: self.records.iter().filter(|r| r.mirror.is_stale()).count(),
            spdx_primary_present: self
                .records
                .iter()
                .filter(|r| r.sbom.spdx_primary_present)
                .count(),
            cyclonedx_export_available: self
                .records
                .iter()
                .filter(|r| r.sbom.cyclonedx_export_available)
                .count(),
            notices_complete: self
                .records
                .iter()
                .filter(|r| r.notices.notice_state == NoticeState::Complete)
                .count(),
            total_active_reasons: self.records.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, docs publication, support exports,
    /// and evaluation packets. It carries only the family, scope, declared and effective
    /// labels, state, the per-axis gap flags, active reasons, and surfaces — never the
    /// detailed scan, audit, and proof internals.
    pub fn reuse_projection(&self) -> Vec<ComplianceReuseRow> {
        self.records
            .iter()
            .map(|r| ComplianceReuseRow {
                record_id: r.record_id.clone(),
                family: r.family,
                scope_kind: r.scope_kind,
                declared_label: r.declared_label,
                effective_label: r.effective_label,
                support_class: r.support_class,
                compliance_state: r.compliance_state,
                release_blocking: r.release_blocking,
                scan_surface_agree: r.scan_surface_agree(),
                spdx_primary_present: r.sbom.spdx_primary_present,
                cyclonedx_export_available: r.sbom.cyclonedx_export_available,
                notice_state: r.notices.notice_state,
                active_reasons: r.active_reasons.clone(),
                surfaces: r.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_COMPLIANCE_AND_NOTICE_BINDING_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_COMPLIANCE_AND_NOTICE_BINDING_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.records.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every family must be covered by exactly one artifact-family record.
        for family in M5Family::ALL {
            let count = self
                .records
                .iter()
                .filter(|r| r.family == family && r.scope_kind == ScopeKind::ArtifactFamily)
                .count();
            if count == 0 {
                v.push(RegisterViolation::FamilyUncovered { family });
            } else if count > 1 {
                v.push(RegisterViolation::FamilyDuplicated { family });
            }
        }

        // Every reason must have a stop rule.
        for reason in ComplianceReason::ALL {
            if self.rule_for(reason).is_none() {
                v.push(RegisterViolation::ReasonUncoveredByRule { reason });
            }
        }

        let mut seen = BTreeSet::new();
        for r in &self.records {
            self.validate_record(r, &mut seen, &mut v);
        }

        // Verdict, parity, and summary coherence.
        if self.publication.decision != self.computed_decision() {
            v.push(RegisterViolation::PublicationDecisionInconsistent);
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            v.push(RegisterViolation::PublicationBlockingRulesMismatch);
        }
        if self.publication.blocking_record_ids != self.computed_blocking_record_ids() {
            v.push(RegisterViolation::PublicationBlockingRecordsMismatch);
        }
        if self.scan_surface_parity != self.computed_scan_surface_parity() {
            v.push(RegisterViolation::ScanSurfaceParityMismatch);
        }
        if self.summary != self.computed_summary() {
            v.push(RegisterViolation::SummaryMismatch);
        }

        v
    }

    fn validate_vocabularies(&self, v: &mut Vec<RegisterViolation>) {
        if self.families != M5Family::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "families" });
        }
        if self.scope_kinds != ScopeKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "scope_kinds",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.control_dimensions != ControlDimension::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "control_dimensions",
            });
        }
        if self.dco_states != DcoState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "dco_states",
            });
        }
        if self.cla_states != ClaState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "cla_states",
            });
        }
        if self.notice_states != NoticeState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "notice_states",
            });
        }
        if self.sbom_binding_states != SbomBindingState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "sbom_binding_states",
            });
        }
        if self.sbom_formats != SbomFormat::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "sbom_formats",
            });
        }
        if self.postures != CompliancePosture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "postures" });
        }
        if self.compliance_states != ComplianceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "compliance_states",
            });
        }
        if self.compliance_reasons != ComplianceReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "compliance_reasons",
            });
        }
        if self.compliance_actions != ComplianceAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "compliance_actions",
            });
        }
        if self.compliance_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "compliance_cutline",
            });
        }
    }

    fn validate_record(
        &self,
        r: &ComplianceRecord,
        seen: &mut BTreeSet<String>,
        v: &mut Vec<RegisterViolation>,
    ) {
        for (field, value) in [
            ("record_id", &r.record_id),
            ("title", &r.title),
            ("subject_ref", &r.subject_ref),
            ("subject_summary", &r.subject_summary),
            ("rationale", &r.rationale),
        ] {
            if value.trim().is_empty() {
                v.push(RegisterViolation::EmptyField {
                    record_id: r.record_id.clone(),
                    field_name: field,
                });
            }
        }
        if !seen.insert(r.record_id.clone()) {
            v.push(RegisterViolation::DuplicateRecordId {
                record_id: r.record_id.clone(),
            });
        }
        if r.surfaces.is_empty() {
            v.push(RegisterViolation::RecordMissingSurfaces {
                record_id: r.record_id.clone(),
            });
        }
        if r.licensing.files_spdx_covered + r.licensing.documented_exceptions
            > r.licensing.files_total
        {
            v.push(RegisterViolation::LicensingCountsExceedTotal {
                record_id: r.record_id.clone(),
            });
        }
        if r.notices.entries_present > r.notices.entries_total {
            v.push(RegisterViolation::NoticeCountsExceedTotal {
                record_id: r.record_id.clone(),
            });
        }
        if r.sbom.spdx_primary_present && !r.sbom.formats.contains(&SbomFormat::SpdxPrimary) {
            v.push(RegisterViolation::SpdxPresentWithoutFormat {
                record_id: r.record_id.clone(),
            });
        }
        if r.sbom.cyclonedx_export_available
            && !r.sbom.formats.contains(&SbomFormat::CyclonedxExport)
        {
            v.push(RegisterViolation::CyclonedxAvailableWithoutFormat {
                record_id: r.record_id.clone(),
            });
        }

        self.validate_controls(r, v);
        self.validate_reason_evidence(r, v);
        self.validate_scan_surface(r, v);
        self.validate_state_and_label(r, v);
    }

    fn validate_controls(&self, r: &ComplianceRecord, v: &mut Vec<RegisterViolation>) {
        // Every control dimension must be declared exactly once, and its declared state
        // must equal the state its facts imply — so a control can never assert "satisfied"
        // over a gap.
        for dimension in ControlDimension::ALL {
            let matches: Vec<&ComplianceControl> = r
                .controls
                .iter()
                .filter(|c| c.dimension == dimension)
                .collect();
            if matches.len() != 1 {
                v.push(RegisterViolation::ControlDimensionNotDeclaredOnce {
                    record_id: r.record_id.clone(),
                    dimension,
                });
                continue;
            }
            let expected = r.expected_control_state(dimension);
            if matches[0].state != expected {
                v.push(RegisterViolation::ControlStateInconsistent {
                    record_id: r.record_id.clone(),
                    dimension,
                });
            }
        }
    }

    /// Every active reason must be justified by the record's own facts, and every
    /// structural gap must surface its reason.
    fn validate_reason_evidence(&self, r: &ComplianceRecord, v: &mut Vec<RegisterViolation>) {
        let dco_gap = r.provenance.dco_gap();
        let cla_gap = r.provenance.cla_gap();
        let licensing_incomplete = r.licensing.coverage_incomplete();
        let exception_undocumented = r.licensing.exception_undocumented();
        let notice_partial = r.notices.is_partial();
        let notice_missing = r.notices.is_missing();
        let spdx_missing = r.sbom.primary_missing();
        let binding_broken = r.sbom.binding_broken();
        let cyclonedx_gap = r.cyclonedx_gap();
        let mirror_stale = r.mirror.is_stale();
        let proof_stale = r.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = r.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !r.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &r.active_reasons {
            let justified = match reason {
                ComplianceReason::DcoSignoffMissing => dco_gap,
                ComplianceReason::ClaUnresolved => cla_gap,
                ComplianceReason::LicensingCoverageIncomplete => licensing_incomplete,
                ComplianceReason::LicenseExceptionUndocumented => exception_undocumented,
                ComplianceReason::NoticeInventoryPartial => notice_partial,
                ComplianceReason::NoticeInventoryMissing => notice_missing,
                ComplianceReason::SbomPrimaryMissing => spdx_missing,
                ComplianceReason::SbomNoticeBindingBroken => binding_broken,
                ComplianceReason::CyclonedxExportUnavailable => cyclonedx_gap,
                ComplianceReason::MirrorStale => mirror_stale,
                ComplianceReason::ComplianceProofStale => proof_stale,
                ComplianceReason::ComplianceProofMissing => proof_missing,
                ComplianceReason::OwnerSignoffMissing => signoff_missing,
                ComplianceReason::WaiverExpired => r.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    record_id: r.record_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: ComplianceReason, v: &mut Vec<RegisterViolation>| {
            if present && !r.has_active_reason(reason) {
                v.push(RegisterViolation::GapWithoutReason {
                    record_id: r.record_id.clone(),
                    reason,
                });
            }
        };
        require(dco_gap, ComplianceReason::DcoSignoffMissing, v);
        require(cla_gap, ComplianceReason::ClaUnresolved, v);
        require(
            licensing_incomplete,
            ComplianceReason::LicensingCoverageIncomplete,
            v,
        );
        require(
            exception_undocumented,
            ComplianceReason::LicenseExceptionUndocumented,
            v,
        );
        require(notice_partial, ComplianceReason::NoticeInventoryPartial, v);
        require(notice_missing, ComplianceReason::NoticeInventoryMissing, v);
        require(spdx_missing, ComplianceReason::SbomPrimaryMissing, v);
        require(binding_broken, ComplianceReason::SbomNoticeBindingBroken, v);
        require(
            cyclonedx_gap,
            ComplianceReason::CyclonedxExportUnavailable,
            v,
        );
        require(mirror_stale, ComplianceReason::MirrorStale, v);
        require(proof_stale, ComplianceReason::ComplianceProofStale, v);
        require(proof_missing, ComplianceReason::ComplianceProofMissing, v);
        require(signoff_missing, ComplianceReason::OwnerSignoffMissing, v);
    }

    /// The scan and the surface must agree, and the posture must reflect the gaps — a
    /// green surface may never sit over a scan that found a missing notice or a
    /// licensing/provenance gap.
    fn validate_scan_surface(&self, r: &ComplianceRecord, v: &mut Vec<RegisterViolation>) {
        if r.scan_posture != r.surface_posture {
            v.push(RegisterViolation::ScanSurfaceDisagreement {
                record_id: r.record_id.clone(),
            });
        }
        let computed = r.computed_posture();
        if r.surface_posture != computed || r.scan_posture != computed {
            v.push(RegisterViolation::PostureMismatch {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_state_and_label(&self, r: &ComplianceRecord, v: &mut Vec<RegisterViolation>) {
        // cleared ⇒ no reasons; narrowed ⇒ at least one reason.
        if r.is_cleared() && !r.active_reasons.is_empty() {
            v.push(RegisterViolation::ClearedWithActiveReason {
                record_id: r.record_id.clone(),
            });
        }
        if r.compliance_state.is_narrowed() && r.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                record_id: r.record_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if r.compliance_state != r.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                record_id: r.record_id.clone(),
                declared: r.compliance_state,
                computed: r.computed_state(),
            });
        }
        // never widen: effective may not rank above declared.
        if r.effective_label.rank() > r.declared_label.rank() {
            v.push(RegisterViolation::EffectiveLabelExceedsDeclared {
                record_id: r.record_id.clone(),
            });
        }
        // effective must equal the computed effective label.
        if r.effective_label != r.computed_effective_label() {
            v.push(RegisterViolation::EffectiveLabelMismatch {
                record_id: r.record_id.clone(),
            });
        }
        // a narrowed record must drop below the cutline.
        if r.compliance_state.is_narrowed() && r.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                record_id: r.record_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplianceReuseRow {
    /// Record id.
    pub record_id: String,
    /// Family.
    pub family: M5Family,
    /// Scope kind.
    pub scope_kind: ScopeKind,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Compliance state.
    pub compliance_state: ComplianceState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// True when the scan and the surface agree.
    pub scan_surface_agree: bool,
    /// True when the SPDX primary SBOM is present.
    pub spdx_primary_present: bool,
    /// True when the CycloneDX export is available.
    pub cyclonedx_export_available: bool,
    /// Notice-inventory state.
    pub notice_state: NoticeState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ComplianceReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the compliance-and-notice-binding register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found.
        actual: u32,
    },
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no records.
    EmptyRegister,
    /// A family has no artifact-family record.
    FamilyUncovered {
        /// Uncovered family.
        family: M5Family,
    },
    /// A family has more than one artifact-family record.
    FamilyDuplicated {
        /// Duplicated family.
        family: M5Family,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: ComplianceReason,
    },
    /// A record id appears more than once.
    DuplicateRecordId {
        /// Duplicate id.
        record_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Record id.
        record_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A record lists no reuse surfaces.
    RecordMissingSurfaces {
        /// Record id.
        record_id: String,
    },
    /// A record's SPDX-covered plus documented-exception count exceeds its file total.
    LicensingCountsExceedTotal {
        /// Record id.
        record_id: String,
    },
    /// A record's present-notice count exceeds its required total.
    NoticeCountsExceedTotal {
        /// Record id.
        record_id: String,
    },
    /// A record marks the SPDX primary present but does not list the format.
    SpdxPresentWithoutFormat {
        /// Record id.
        record_id: String,
    },
    /// A record marks the CycloneDX export available but does not list the format.
    CyclonedxAvailableWithoutFormat {
        /// Record id.
        record_id: String,
    },
    /// A control dimension is not declared exactly once.
    ControlDimensionNotDeclaredOnce {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// A control's declared state disagrees with the facts it governs.
    ControlStateInconsistent {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// An active reason is not justified by the record's fields.
    ReasonNotJustified {
        /// Record id.
        record_id: String,
        /// Offending reason.
        reason: ComplianceReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Record id.
        record_id: String,
        /// Missing reason.
        reason: ComplianceReason,
    },
    /// A record's scan and surface postures disagree.
    ScanSurfaceDisagreement {
        /// Record id.
        record_id: String,
    },
    /// A record's posture disagrees with the gaps its state implies.
    PostureMismatch {
        /// Record id.
        record_id: String,
    },
    /// A cleared record carries an active reason.
    ClearedWithActiveReason {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record carries no reason.
    NarrowedWithoutReason {
        /// Record id.
        record_id: String,
    },
    /// The record state disagrees with the active reasons.
    StateReasonMismatch {
        /// Record id.
        record_id: String,
        /// Declared state.
        declared: ComplianceState,
        /// Computed state.
        computed: ComplianceState,
    },
    /// The effective label ranks above the declared label.
    EffectiveLabelExceedsDeclared {
        /// Record id.
        record_id: String,
    },
    /// The effective label disagrees with the computed effective label.
    EffectiveLabelMismatch {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record did not drop below the cutline.
    NarrowedAboveCutline {
        /// Record id.
        record_id: String,
    },
    /// The promotion decision disagrees with the firing rules.
    PublicationDecisionInconsistent,
    /// The recorded blocking rule ids disagree with the computed set.
    PublicationBlockingRulesMismatch,
    /// The recorded blocking record ids disagree with the computed set.
    PublicationBlockingRecordsMismatch,
    /// The recorded scan/surface parity disagrees with the computed summary.
    ScanSurfaceParityMismatch,
    /// The summary counts disagree with the records.
    SummaryMismatch,
}

impl fmt::Display for RegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no records"),
            Self::FamilyUncovered { family } => {
                write!(
                    f,
                    "family {} has no artifact-family record",
                    family.as_str()
                )
            }
            Self::FamilyDuplicated { family } => write!(
                f,
                "family {} has more than one artifact-family record",
                family.as_str()
            ),
            Self::ReasonUncoveredByRule { reason } => {
                write!(f, "reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateRecordId { record_id } => {
                write!(f, "duplicate record id {record_id}")
            }
            Self::EmptyField {
                record_id,
                field_name,
            } => write!(f, "record {record_id} has empty field {field_name}"),
            Self::RecordMissingSurfaces { record_id } => {
                write!(f, "record {record_id} lists no reuse surfaces")
            }
            Self::LicensingCountsExceedTotal { record_id } => write!(
                f,
                "record {record_id} SPDX-covered plus documented exceptions exceed its file total"
            ),
            Self::NoticeCountsExceedTotal { record_id } => {
                write!(
                    f,
                    "record {record_id} present-notice count exceeds its total"
                )
            }
            Self::SpdxPresentWithoutFormat { record_id } => {
                write!(
                    f,
                    "record {record_id} marks SPDX present but omits the spdx_primary format"
                )
            }
            Self::CyclonedxAvailableWithoutFormat { record_id } => write!(
                f,
                "record {record_id} marks CycloneDX available but omits the cyclonedx_export format"
            ),
            Self::ControlDimensionNotDeclaredOnce {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} does not declare control {} exactly once",
                dimension.as_str()
            ),
            Self::ControlStateInconsistent {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} control {} state disagrees with its facts",
                dimension.as_str()
            ),
            Self::ReasonNotJustified { record_id, reason } => write!(
                f,
                "record {record_id} names reason {} which its fields do not justify",
                reason.as_str()
            ),
            Self::GapWithoutReason { record_id, reason } => write!(
                f,
                "record {record_id} has a structural gap but does not name reason {}",
                reason.as_str()
            ),
            Self::ScanSurfaceDisagreement { record_id } => {
                write!(f, "record {record_id} scan and surface postures disagree")
            }
            Self::PostureMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} posture disagrees with the gaps its state implies"
                )
            }
            Self::ClearedWithActiveReason { record_id } => {
                write!(
                    f,
                    "cleared record {record_id} carries an active narrowing reason"
                )
            }
            Self::NarrowedWithoutReason { record_id } => {
                write!(f, "narrowed record {record_id} names no reason")
            }
            Self::StateReasonMismatch {
                record_id,
                declared,
                computed,
            } => write!(
                f,
                "record {record_id} records state {} but its reasons imply {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::EffectiveLabelExceedsDeclared { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label is wider than its declared label"
                )
            }
            Self::EffectiveLabelMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label disagrees with its state"
                )
            }
            Self::NarrowedAboveCutline { record_id } => {
                write!(
                    f,
                    "narrowed record {record_id} did not drop below the cutline"
                )
            }
            Self::PublicationDecisionInconsistent => {
                write!(f, "promotion decision disagrees with the firing rules")
            }
            Self::PublicationBlockingRulesMismatch => {
                write!(
                    f,
                    "publication blocking_rule_ids disagree with the computed set"
                )
            }
            Self::PublicationBlockingRecordsMismatch => {
                write!(
                    f,
                    "publication blocking_record_ids disagree with the computed set"
                )
            }
            Self::ScanSurfaceParityMismatch => {
                write!(f, "scan_surface_parity disagrees with the computed summary")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the records"),
        }
    }
}

impl Error for RegisterViolation {}

/// Loads the embedded compliance-and-notice-binding register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`ComplianceRegister`] — including when a record carries a token outside any closed
/// vocabulary.
pub fn current_m5_compliance_and_notice_binding() -> Result<ComplianceRegister, serde_json::Error> {
    serde_json::from_str(M5_COMPLIANCE_AND_NOTICE_BINDING_JSON)
}

#[cfg(test)]
mod tests;
