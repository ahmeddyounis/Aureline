//! Canonical M5 conformance scorecards, validator diagnostics, and
//! reference-workspace linkage — certifiability for the new M5 artifact families.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per marketed M5 artifact family and the
//! [`m5_marketplace_fact_views`](crate::m5_marketplace_fact_views) project that truth into
//! the storefront, this module proves that a family's *support claim* is backed by a
//! current conformance or compatibility scorecard, a named owner, a linked archetype,
//! and a linked reference workspace — not by first-party or bridge-backed status alone.
//! A [`ConformanceScorecard`] carries one [`ConformanceLabel`] from the stable
//! Native/Bridge/Partial/Unsupported/Retest-pending vocabulary, the validator
//! diagnostics an author must clear, and the evidence linkage a support claim rides on.
//!
//! The scorecard is honest by construction. The [`CertificationDisposition`] a
//! scorecard publishes is **not** stored by hand: it is recomputed from the scorecard's
//! facts as the widest [`CertificationSignal::min_disposition`] over every detected
//! [`CertificationSignal`], and the stored disposition, signal set, and effective
//! support class must equal that recomputation or validation fails. The lane guardrail
//! rides that recomputation: stale or unknown evidence, a missing owner, an unlinked
//! archetype, an unlinked reference workspace, missing conformance evidence, a validator
//! failure, a retest-pending label, or an unsupported label each force
//! [`CertificationDisposition::Uncertified`], and an uncertified scorecard's effective
//! support class is forced to [`SupportClass::Unsupported`]. A first-party or
//! bridge-backed family therefore can never publish a support claim without a current,
//! owned, evidence-linked scorecard.
//!
//! Every scorecard also exports both machine-readable and human-readable validator
//! diagnostics — each with a stable code, a [`ValidatorSeverity`], the
//! [`ValidatorDomain`] it concerns, a message, and a remediation — so authors and
//! maintainers can fix schema, capability, compatibility, permission, or provenance
//! issues without private tribal knowledge, and issue reports, release evidence, and
//! enterprise evaluations consume the same scorecard and validator vocabulary.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-conformance-and-validators.json` and embedded here, so
//! this typed consumer and any CI gate agree on every record without a cargo build in
//! CI. The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, signing secrets, or evidence
//! bodies.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ArtifactFamily, CompatibilityLabel, EvidenceFreshness, RuntimeOrigin, SupportClass,
};

/// Supported M5 conformance-and-validators schema version.
pub const M5_CONFORMANCE_AND_VALIDATORS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_CONFORMANCE_AND_VALIDATORS_RECORD_KIND: &str = "m5_conformance_and_validators";

/// Repo-relative path to the checked-in packet.
pub const M5_CONFORMANCE_AND_VALIDATORS_PATH: &str =
    "artifacts/ecosystem/m5/m5-conformance-and-validators.json";

/// Embedded checked-in packet JSON.
pub const M5_CONFORMANCE_AND_VALIDATORS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-conformance-and-validators.json"
));

/// The conformance and runtime posture of an artifact family against its target.
///
/// This is the stable Native/Bridge/Partial/Unsupported/Retest-pending vocabulary that
/// marketplace badges, docs badges, release evidence, and support exports all consume.
/// A label is never vague support copy: it caps the support class a family may publish
/// through [`ConformanceLabel::support_ceiling`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceLabel {
    /// Conforms natively on the target runtime.
    Native,
    /// Conforms through a compatibility bridge or local-model host.
    Bridge,
    /// Conforms only partially; some conformance cases are unmet.
    Partial,
    /// Does not conform on the target; carries no positive claim.
    Unsupported,
    /// A prior result has lapsed and the family must be re-tested before it claims.
    RetestPending,
}

impl ConformanceLabel {
    /// Every conformance label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Native,
        Self::Bridge,
        Self::Partial,
        Self::Unsupported,
        Self::RetestPending,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bridge => "bridge",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::RetestPending => "retest_pending",
        }
    }

    /// Highest support class this conformance label permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Native => SupportClass::FullySupported,
            Self::Bridge | Self::Partial => SupportClass::BestEffortSupported,
            Self::Unsupported | Self::RetestPending => SupportClass::Unsupported,
        }
    }

    /// Whether the label is bridge-backed or partial rather than native.
    pub const fn is_non_native_trigger(self) -> bool {
        matches!(self, Self::Bridge | Self::Partial)
    }

    /// Whether the label is unsupported on the target.
    pub const fn is_unsupported_trigger(self) -> bool {
        matches!(self, Self::Unsupported)
    }

    /// Whether the label requires a re-test before the family may claim again.
    pub const fn is_retest_trigger(self) -> bool {
        matches!(self, Self::RetestPending)
    }
}

/// Severity of one validator diagnostic.
///
/// Ordered low-to-high by [`ValidatorSeverity::rank`]: an [`ValidatorSeverity::Error`]
/// is a hard failure that blocks certification, while an [`ValidatorSeverity::Info`]
/// is advisory only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorSeverity {
    /// Advisory only; does not narrow certification.
    Info,
    /// A non-blocking issue; narrows certification to conditional.
    Warning,
    /// A blocking failure; blocks certification.
    Error,
}

impl ValidatorSeverity {
    /// Every validator severity, in declaration order.
    pub const ALL: [Self; 3] = [Self::Info, Self::Warning, Self::Error];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Monotonic rank; higher means a stronger failure.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
        }
    }
}

/// The subject a validator diagnostic concerns.
///
/// The domain makes a diagnostic actionable: an author knows whether to fix a schema, a
/// declared capability, a compatibility target, a permission manifest, provenance, the
/// reference-workspace linkage, or the activation budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorDomain {
    /// The package or manifest schema.
    Schema,
    /// A declared capability.
    Capability,
    /// Compatibility against the install target.
    Compatibility,
    /// The permission manifest.
    Permission,
    /// Signature or build provenance.
    Provenance,
    /// Reference-workspace or archetype linkage.
    ReferenceWorkspace,
    /// Activation budget or runtime cost.
    ActivationBudget,
    /// Listing metadata or documentation.
    Metadata,
}

impl ValidatorDomain {
    /// Every validator domain, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Schema,
        Self::Capability,
        Self::Compatibility,
        Self::Permission,
        Self::Provenance,
        Self::ReferenceWorkspace,
        Self::ActivationBudget,
        Self::Metadata,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Capability => "capability",
            Self::Compatibility => "compatibility",
            Self::Permission => "permission",
            Self::Provenance => "provenance",
            Self::ReferenceWorkspace => "reference_workspace",
            Self::ActivationBudget => "activation_budget",
            Self::Metadata => "metadata",
        }
    }
}

/// One actionable validator diagnostic for a scorecard.
///
/// A diagnostic is actionable by construction: it pairs a stable [`code`](Self::code)
/// and [`ValidatorDomain`] with a human-readable [`message`](Self::message) and a
/// concrete [`remediation`](Self::remediation), so an author or maintainer can fix the
/// issue without private tribal knowledge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatorDiagnostic {
    /// Stable diagnostic code (for example, `schema.deprecated_field`).
    pub code: String,
    /// Subject the diagnostic concerns.
    pub domain: ValidatorDomain,
    /// Severity of the diagnostic.
    pub severity: ValidatorSeverity,
    /// Human-readable description of the issue.
    pub message: String,
    /// Concrete remediation an author can act on.
    pub remediation: String,
    /// Opaque ref to where the issue was found.
    pub evidence_ref: String,
}

/// A certification or trust signal a scorecard surfaces.
///
/// Each signal is recomputed from the scorecard's facts; the scorecard's stored
/// [`ConformanceScorecard::certification_signals`] must equal the recomputed set. Each
/// signal carries a fixed [`CertificationSignal::min_disposition`], so the published
/// [`CertificationDisposition`] is a pure function of which signals fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSignal {
    /// The conformance label is bridge-backed or partial rather than native.
    NonNativeRuntime,
    /// At least one validator diagnostic is a warning.
    ValidatorWarning,
    /// The qualifying conformance or compatibility evidence is not current.
    EvidenceNotCurrent,
    /// No owner is named for the scorecard.
    OwnerMissing,
    /// No archetype descriptor is linked.
    ArchetypeUnlinked,
    /// No reference workspace is linked.
    ReferenceWorkspaceUnlinked,
    /// A conformance or compatibility evidence ref is missing.
    ConformanceEvidenceMissing,
    /// At least one validator diagnostic is an error.
    ValidatorFailure,
    /// The conformance label requires a re-test before the family may claim.
    RetestPending,
    /// The conformance label is unsupported on the target.
    Unsupported,
}

impl CertificationSignal {
    /// Every certification signal, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::NonNativeRuntime,
        Self::ValidatorWarning,
        Self::EvidenceNotCurrent,
        Self::OwnerMissing,
        Self::ArchetypeUnlinked,
        Self::ReferenceWorkspaceUnlinked,
        Self::ConformanceEvidenceMissing,
        Self::ValidatorFailure,
        Self::RetestPending,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonNativeRuntime => "non_native_runtime",
            Self::ValidatorWarning => "validator_warning",
            Self::EvidenceNotCurrent => "evidence_not_current",
            Self::OwnerMissing => "owner_missing",
            Self::ArchetypeUnlinked => "archetype_unlinked",
            Self::ReferenceWorkspaceUnlinked => "reference_workspace_unlinked",
            Self::ConformanceEvidenceMissing => "conformance_evidence_missing",
            Self::ValidatorFailure => "validator_failure",
            Self::RetestPending => "retest_pending",
            Self::Unsupported => "unsupported",
        }
    }

    /// The minimum certification disposition this signal forces.
    ///
    /// The review-class signals — a non-native runtime and a validator warning — force
    /// [`CertificationDisposition::ConditionallyCertified`]. The guardrail-class signals
    /// — stale evidence, a missing owner, an unlinked archetype, an unlinked reference
    /// workspace, missing conformance evidence, a validator failure, a retest-pending
    /// label, or an unsupported label — force [`CertificationDisposition::Uncertified`],
    /// so a family cannot publish a support claim without a current, owned,
    /// evidence-linked scorecard.
    pub const fn min_disposition(self) -> CertificationDisposition {
        match self {
            Self::NonNativeRuntime | Self::ValidatorWarning => {
                CertificationDisposition::ConditionallyCertified
            }
            Self::EvidenceNotCurrent
            | Self::OwnerMissing
            | Self::ArchetypeUnlinked
            | Self::ReferenceWorkspaceUnlinked
            | Self::ConformanceEvidenceMissing
            | Self::ValidatorFailure
            | Self::RetestPending
            | Self::Unsupported => CertificationDisposition::Uncertified,
        }
    }
}

/// The disposition a scorecard publishes.
///
/// Ordered low-to-high by [`CertificationDisposition::rank`]: a
/// [`CertificationDisposition::Certified`] scorecard backs a full claim, and a
/// [`CertificationDisposition::Uncertified`] scorecard backs no support claim at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDisposition {
    /// No signal applies; the scorecard is current and fully backed.
    Certified,
    /// Only review-class signals apply; certified with disclosed conditions.
    ConditionallyCertified,
    /// At least one guardrail-class signal applies; no support claim is backed.
    Uncertified,
}

impl CertificationDisposition {
    /// Every certification disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Certified,
        Self::ConditionallyCertified,
        Self::Uncertified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ConditionallyCertified => "conditionally_certified",
            Self::Uncertified => "uncertified",
        }
    }

    /// Monotonic rank; higher means a weaker certification.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Certified => 0,
            Self::ConditionallyCertified => 1,
            Self::Uncertified => 2,
        }
    }

    /// The weaker (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// A conformance/compatibility scorecard for one M5 artifact family.
///
/// The scorecard reproduces the support-claim fact set — package identity, runtime
/// origin, conformance and compatibility labels, claimed and effective support, and
/// evidence freshness — and links the owner, archetype, reference workspaces, and
/// conformance/compatibility evidence a support claim rides on. The published
/// disposition, signal set, and effective support class are recomputed from those
/// facts and must equal the recomputation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceScorecard {
    /// Stable scorecard id.
    pub scorecard_id: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this scorecard resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Conformance and runtime posture label.
    pub conformance_label: ConformanceLabel,
    /// Compatibility label against the install target.
    pub compatibility_label: CompatibilityLabel,
    /// Support class the family wants to publish.
    pub claimed_support_class: SupportClass,
    /// Recomputed effective support class; must equal the recomputed value.
    pub effective_support_class: SupportClass,
    /// Evidence freshness of the qualifying conformance or compatibility result.
    pub evidence_freshness: EvidenceFreshness,
    /// Opaque ref to the owner accountable for the claim (empty when unowned).
    #[serde(default)]
    pub owner_ref: String,
    /// Opaque ref to the linked archetype descriptor (empty when unlinked).
    #[serde(default)]
    pub archetype_ref: String,
    /// Opaque refs to linked reference workspaces (empty when unlinked).
    #[serde(default)]
    pub reference_workspace_refs: Vec<String>,
    /// Opaque ref to the conformance-suite evidence (empty when missing).
    #[serde(default)]
    pub conformance_ref: String,
    /// Opaque ref to the compatibility-report evidence (empty when missing).
    #[serde(default)]
    pub compatibility_ref: String,
    /// Validator diagnostics an author must clear.
    #[serde(default)]
    pub validator_diagnostics: Vec<ValidatorDiagnostic>,
    /// Recomputed certification signals; must equal the recomputed set.
    #[serde(default)]
    pub certification_signals: Vec<CertificationSignal>,
    /// Recomputed certification disposition; must equal the recomputed value.
    pub certification_disposition: CertificationDisposition,
    /// Ref binding this scorecard into support and release surfaces.
    pub support_export_ref: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl ConformanceScorecard {
    /// Whether the scorecard carries every evidence linkage a support claim requires.
    ///
    /// A support claim must name an owner, link an archetype and a reference workspace,
    /// and cite conformance and compatibility evidence; a scorecard that drops any of
    /// these can back no positive claim.
    pub fn is_evidence_backed(&self) -> bool {
        !self.owner_ref.trim().is_empty()
            && !self.archetype_ref.trim().is_empty()
            && !self.reference_workspace_refs.is_empty()
            && self
                .reference_workspace_refs
                .iter()
                .all(|r| !r.trim().is_empty())
            && !self.conformance_ref.trim().is_empty()
            && !self.compatibility_ref.trim().is_empty()
    }

    /// Whether any validator diagnostic carries the given severity.
    pub fn has_severity(&self, severity: ValidatorSeverity) -> bool {
        self.validator_diagnostics
            .iter()
            .any(|d| d.severity == severity)
    }

    /// The certification signals recomputed from this scorecard's facts, in canonical
    /// order.
    pub fn computed_certification_signals(&self) -> Vec<CertificationSignal> {
        CertificationSignal::ALL
            .into_iter()
            .filter(|signal| self.signal_detected(*signal))
            .collect()
    }

    fn signal_detected(&self, signal: CertificationSignal) -> bool {
        match signal {
            CertificationSignal::NonNativeRuntime => self.conformance_label.is_non_native_trigger(),
            CertificationSignal::ValidatorWarning => self.has_severity(ValidatorSeverity::Warning),
            CertificationSignal::EvidenceNotCurrent => !self.evidence_freshness.is_current(),
            CertificationSignal::OwnerMissing => self.owner_ref.trim().is_empty(),
            CertificationSignal::ArchetypeUnlinked => self.archetype_ref.trim().is_empty(),
            CertificationSignal::ReferenceWorkspaceUnlinked => {
                self.reference_workspace_refs.is_empty()
                    || self
                        .reference_workspace_refs
                        .iter()
                        .any(|r| r.trim().is_empty())
            }
            CertificationSignal::ConformanceEvidenceMissing => {
                self.conformance_ref.trim().is_empty() || self.compatibility_ref.trim().is_empty()
            }
            CertificationSignal::ValidatorFailure => self.has_severity(ValidatorSeverity::Error),
            CertificationSignal::RetestPending => self.conformance_label.is_retest_trigger(),
            CertificationSignal::Unsupported => self.conformance_label.is_unsupported_trigger(),
        }
    }

    /// The certification disposition recomputed from this scorecard's facts.
    pub fn computed_certification_disposition(&self) -> CertificationDisposition {
        self.computed_certification_signals().into_iter().fold(
            CertificationDisposition::Certified,
            |disposition, signal| disposition.widen(signal.min_disposition()),
        )
    }

    /// The effective support class recomputed from this scorecard's facts.
    ///
    /// The effective class is the weakest of the claimed class and every ceiling — the
    /// conformance label, the runtime origin, the compatibility label, and the evidence
    /// freshness — and is forced to [`SupportClass::Unsupported`] when the scorecard is
    /// [`CertificationDisposition::Uncertified`], so an unbacked claim collapses to no
    /// claim.
    pub fn computed_effective_support_class(&self) -> SupportClass {
        if self.computed_certification_disposition() == CertificationDisposition::Uncertified {
            return SupportClass::Unsupported;
        }
        self.claimed_support_class
            .min(self.conformance_label.support_ceiling())
            .min(self.runtime_origin.support_ceiling())
            .min(self.compatibility_label.support_ceiling())
            .min(self.evidence_freshness.support_ceiling())
    }

    /// Whether the stored disposition, signals, and effective support agree with the
    /// recomputed values.
    pub fn is_consistent(&self) -> bool {
        self.certification_signals == self.computed_certification_signals()
            && self.certification_disposition == self.computed_certification_disposition()
            && self.effective_support_class == self.computed_effective_support_class()
    }

    /// Count of validator diagnostics at the given severity.
    pub fn severity_count(&self, severity: ValidatorSeverity) -> usize {
        self.validator_diagnostics
            .iter()
            .filter(|d| d.severity == severity)
            .count()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ConformanceAndValidatorsSummary {
    /// Total scorecards.
    pub total_scorecards: usize,
    /// Scorecards that are certified.
    pub certified_scorecards: usize,
    /// Scorecards that are conditionally certified.
    pub conditionally_certified_scorecards: usize,
    /// Scorecards that are uncertified.
    pub uncertified_scorecards: usize,
    /// Scorecards labelled native.
    pub native_scorecards: usize,
    /// Scorecards labelled bridge.
    pub bridge_scorecards: usize,
    /// Scorecards labelled partial.
    pub partial_scorecards: usize,
    /// Scorecards labelled retest-pending.
    pub retest_pending_scorecards: usize,
    /// Scorecards labelled unsupported.
    pub unsupported_scorecards: usize,
    /// Scorecards carrying every evidence linkage.
    pub evidence_backed_scorecards: usize,
    /// Scorecards with at least one validator failure.
    pub scorecards_with_validator_failures: usize,
    /// Scorecards with at least one validator warning.
    pub scorecards_with_validator_warnings: usize,
    /// Total validator diagnostics across all scorecards.
    pub total_validator_diagnostics: usize,
    /// Distinct package kinds across scorecards.
    pub distinct_package_kinds: usize,
}

/// A machine-readable export row projected from a scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConformanceScorecardRow {
    /// Scorecard id.
    pub scorecard_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Runtime-origin token.
    pub runtime_origin: String,
    /// Conformance-label token.
    pub conformance_label: String,
    /// Compatibility-label token.
    pub compatibility_label: String,
    /// Claimed-support-class token.
    pub claimed_support_class: String,
    /// Effective-support-class token.
    pub effective_support_class: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Certification-disposition token.
    pub certification_disposition: String,
    /// Certification-signal tokens.
    pub certification_signals: Vec<String>,
    /// Validator-failure count.
    pub validator_failure_count: usize,
    /// Validator-warning count.
    pub validator_warning_count: usize,
    /// Owner ref.
    pub owner_ref: String,
    /// Archetype ref.
    pub archetype_ref: String,
    /// Reference-workspace refs.
    pub reference_workspace_refs: Vec<String>,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Whether the scorecard carries every evidence linkage.
    pub evidence_backed: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A flat validator-report row for issue reports and release evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ValidatorReportRow {
    /// Scorecard id the diagnostic belongs to.
    pub scorecard_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Diagnostic code.
    pub code: String,
    /// Validator-domain token.
    pub domain: String,
    /// Validator-severity token.
    pub severity: String,
    /// Human-readable message.
    pub message: String,
    /// Concrete remediation.
    pub remediation: String,
    /// Opaque evidence ref.
    pub evidence_ref: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConformanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Machine-readable scorecard rows.
    pub scorecard_rows: Vec<M5ConformanceScorecardRow>,
    /// Flat validator report across every scorecard.
    pub validator_report: Vec<M5ValidatorReportRow>,
    /// Whether every scorecard is recomputation-consistent.
    pub all_scorecards_consistent: bool,
    /// Scorecards that are uncertified.
    pub uncertified_count: usize,
    /// Total validator failures across every scorecard.
    pub validator_failure_count: usize,
}

/// The typed M5 conformance-and-validators packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ConformanceAndValidators {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed runtime-origin vocabulary (reused from the governance matrix).
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed support-class vocabulary (reused from the governance matrix).
    pub support_classes: Vec<SupportClass>,
    /// Closed compatibility-label vocabulary (reused from the governance matrix).
    pub compatibility_labels: Vec<CompatibilityLabel>,
    /// Closed evidence-freshness vocabulary (reused from the governance matrix).
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed conformance-label vocabulary.
    pub conformance_labels: Vec<ConformanceLabel>,
    /// Closed validator-severity vocabulary.
    pub validator_severities: Vec<ValidatorSeverity>,
    /// Closed validator-domain vocabulary.
    pub validator_domains: Vec<ValidatorDomain>,
    /// Closed certification-signal vocabulary.
    pub certification_signals: Vec<CertificationSignal>,
    /// Closed certification-disposition vocabulary.
    pub certification_dispositions: Vec<CertificationDisposition>,
    /// Conformance/compatibility scorecards.
    #[serde(default)]
    pub scorecards: Vec<ConformanceScorecard>,
    /// Summary counts.
    pub summary: M5ConformanceAndValidatorsSummary,
}

impl M5ConformanceAndValidators {
    /// Returns the scorecard with the given id.
    pub fn scorecard(&self, scorecard_id: &str) -> Option<&ConformanceScorecard> {
        self.scorecards
            .iter()
            .find(|s| s.scorecard_id == scorecard_id)
    }

    /// Recomputes the summary block from the scorecards.
    pub fn computed_summary(&self) -> M5ConformanceAndValidatorsSummary {
        let count_disposition = |d: CertificationDisposition| {
            self.scorecards
                .iter()
                .filter(|s| s.certification_disposition == d)
                .count()
        };
        let count_label = |l: ConformanceLabel| {
            self.scorecards
                .iter()
                .filter(|s| s.conformance_label == l)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.scorecards.iter().map(|s| s.package_kind).collect();
        M5ConformanceAndValidatorsSummary {
            total_scorecards: self.scorecards.len(),
            certified_scorecards: count_disposition(CertificationDisposition::Certified),
            conditionally_certified_scorecards: count_disposition(
                CertificationDisposition::ConditionallyCertified,
            ),
            uncertified_scorecards: count_disposition(CertificationDisposition::Uncertified),
            native_scorecards: count_label(ConformanceLabel::Native),
            bridge_scorecards: count_label(ConformanceLabel::Bridge),
            partial_scorecards: count_label(ConformanceLabel::Partial),
            retest_pending_scorecards: count_label(ConformanceLabel::RetestPending),
            unsupported_scorecards: count_label(ConformanceLabel::Unsupported),
            evidence_backed_scorecards: self
                .scorecards
                .iter()
                .filter(|s| s.is_evidence_backed())
                .count(),
            scorecards_with_validator_failures: self
                .scorecards
                .iter()
                .filter(|s| s.has_severity(ValidatorSeverity::Error))
                .count(),
            scorecards_with_validator_warnings: self
                .scorecards
                .iter()
                .filter(|s| s.has_severity(ValidatorSeverity::Warning))
                .count(),
            total_validator_diagnostics: self
                .scorecards
                .iter()
                .map(|s| s.validator_diagnostics.len())
                .sum(),
            distinct_package_kinds: package_kinds.len(),
        }
    }

    /// Whether every scorecard agrees with its recomputation.
    pub fn all_records_consistent(&self) -> bool {
        self.scorecards
            .iter()
            .all(ConformanceScorecard::is_consistent)
    }

    /// Produces an export projection that downstream surfaces — marketplace badges,
    /// docs badges, support exports, and release/audit packets — render instead of
    /// restating conformance, support, and validator status by hand.
    pub fn export_projection(&self) -> M5ConformanceExportProjection {
        let scorecard_rows = self
            .scorecards
            .iter()
            .map(|s| M5ConformanceScorecardRow {
                scorecard_id: s.scorecard_id.clone(),
                package_kind: s.package_kind.as_str().to_owned(),
                runtime_origin: s.runtime_origin.as_str().to_owned(),
                conformance_label: s.conformance_label.as_str().to_owned(),
                compatibility_label: s.compatibility_label.as_str().to_owned(),
                claimed_support_class: s.claimed_support_class.as_str().to_owned(),
                effective_support_class: s.effective_support_class.as_str().to_owned(),
                evidence_freshness: s.evidence_freshness.as_str().to_owned(),
                certification_disposition: s.certification_disposition.as_str().to_owned(),
                certification_signals: s
                    .certification_signals
                    .iter()
                    .map(|sig| sig.as_str().to_owned())
                    .collect(),
                validator_failure_count: s.severity_count(ValidatorSeverity::Error),
                validator_warning_count: s.severity_count(ValidatorSeverity::Warning),
                owner_ref: s.owner_ref.clone(),
                archetype_ref: s.archetype_ref.clone(),
                reference_workspace_refs: s.reference_workspace_refs.clone(),
                governance_family_ref: s.governance_family_ref.clone(),
                evidence_backed: s.is_evidence_backed(),
                summary: format!(
                    "{}: conformance {}, compatibility {}, claimed {}, effective {}, evidence {}, disposition {}",
                    s.package_kind.as_str(),
                    s.conformance_label.as_str(),
                    s.compatibility_label.as_str(),
                    s.claimed_support_class.as_str(),
                    s.effective_support_class.as_str(),
                    s.evidence_freshness.as_str(),
                    s.certification_disposition.as_str(),
                ),
            })
            .collect();
        let validator_report = self
            .scorecards
            .iter()
            .flat_map(|s| {
                s.validator_diagnostics
                    .iter()
                    .map(move |d| M5ValidatorReportRow {
                        scorecard_id: s.scorecard_id.clone(),
                        package_kind: s.package_kind.as_str().to_owned(),
                        code: d.code.clone(),
                        domain: d.domain.as_str().to_owned(),
                        severity: d.severity.as_str().to_owned(),
                        message: d.message.clone(),
                        remediation: d.remediation.clone(),
                        evidence_ref: d.evidence_ref.clone(),
                    })
            })
            .collect();
        M5ConformanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            scorecard_rows,
            validator_report,
            all_scorecards_consistent: self.all_records_consistent(),
            uncertified_count: self
                .scorecards
                .iter()
                .filter(|s| s.certification_disposition == CertificationDisposition::Uncertified)
                .count(),
            validator_failure_count: self
                .scorecards
                .iter()
                .map(|s| s.severity_count(ValidatorSeverity::Error))
                .sum(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ConformanceAndValidatorsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for scorecard in &self.scorecards {
            if !seen.insert(scorecard.scorecard_id.clone()) {
                violations.push(M5ConformanceAndValidatorsViolation::DuplicateScorecardId {
                    scorecard_id: scorecard.scorecard_id.clone(),
                });
            }
            self.validate_scorecard(scorecard, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5ConformanceAndValidatorsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ConformanceAndValidatorsViolation>) {
        if self.schema_version != M5_CONFORMANCE_AND_VALIDATORS_SCHEMA_VERSION {
            violations.push(
                M5ConformanceAndValidatorsViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_CONFORMANCE_AND_VALIDATORS_RECORD_KIND {
            violations.push(M5ConformanceAndValidatorsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ConformanceAndValidatorsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "compatibility_labels",
                self.compatibility_labels == CompatibilityLabel::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "conformance_labels",
                self.conformance_labels == ConformanceLabel::ALL.to_vec(),
            ),
            (
                "validator_severities",
                self.validator_severities == ValidatorSeverity::ALL.to_vec(),
            ),
            (
                "validator_domains",
                self.validator_domains == ValidatorDomain::ALL.to_vec(),
            ),
            (
                "certification_signals",
                self.certification_signals == CertificationSignal::ALL.to_vec(),
            ),
            (
                "certification_dispositions",
                self.certification_dispositions == CertificationDisposition::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5ConformanceAndValidatorsViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_scorecard(
        &self,
        scorecard: &ConformanceScorecard,
        violations: &mut Vec<M5ConformanceAndValidatorsViolation>,
    ) {
        for (field, value) in [
            ("scorecard_id", &scorecard.scorecard_id),
            ("display_label", &scorecard.display_label),
            ("governance_family_ref", &scorecard.governance_family_ref),
            ("support_export_ref", &scorecard.support_export_ref),
            ("summary", &scorecard.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ConformanceAndValidatorsViolation::EmptyField {
                    id: scorecard.scorecard_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_signals = BTreeSet::new();
        for signal in &scorecard.certification_signals {
            if !seen_signals.insert(*signal) {
                violations.push(
                    M5ConformanceAndValidatorsViolation::DuplicateCertificationSignal {
                        id: scorecard.scorecard_id.clone(),
                        signal: signal.as_str(),
                    },
                );
            }
        }

        let mut seen_codes = BTreeSet::new();
        for diagnostic in &scorecard.validator_diagnostics {
            self.validate_diagnostic(scorecard, diagnostic, violations);
            if !seen_codes.insert(diagnostic.code.clone()) {
                violations.push(
                    M5ConformanceAndValidatorsViolation::DuplicateDiagnosticCode {
                        id: scorecard.scorecard_id.clone(),
                        code: diagnostic.code.clone(),
                    },
                );
            }
        }

        // The published signals must equal the recomputed set, so a widening can never
        // be asserted or hidden by hand.
        if scorecard.certification_signals != scorecard.computed_certification_signals() {
            violations.push(
                M5ConformanceAndValidatorsViolation::CertificationSignalsMismatch {
                    id: scorecard.scorecard_id.clone(),
                },
            );
        }

        // The published disposition must equal the recomputed disposition.
        let disposition = scorecard.computed_certification_disposition();
        if scorecard.certification_disposition != disposition {
            violations.push(
                M5ConformanceAndValidatorsViolation::CertificationDispositionMismatch {
                    id: scorecard.scorecard_id.clone(),
                    stored: scorecard.certification_disposition.as_str(),
                    computed: disposition.as_str(),
                },
            );
        }

        // The published effective support class must equal the recomputed value.
        let effective = scorecard.computed_effective_support_class();
        if scorecard.effective_support_class != effective {
            violations.push(
                M5ConformanceAndValidatorsViolation::EffectiveSupportMismatch {
                    id: scorecard.scorecard_id.clone(),
                    stored: scorecard.effective_support_class.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The lane guardrail: a scorecard that publishes any support claim must be
        // evidence-backed and not uncertified. The recomputation forces this, but the
        // explicit check makes the guardrail legible to readers and gates fixtures that
        // try to assert a claim by hand.
        if scorecard.effective_support_class != SupportClass::Unsupported
            && (!scorecard.is_evidence_backed()
                || scorecard.certification_disposition == CertificationDisposition::Uncertified)
        {
            violations.push(
                M5ConformanceAndValidatorsViolation::SupportClaimedWithoutEvidence {
                    id: scorecard.scorecard_id.clone(),
                },
            );
        }
    }

    fn validate_diagnostic(
        &self,
        scorecard: &ConformanceScorecard,
        diagnostic: &ValidatorDiagnostic,
        violations: &mut Vec<M5ConformanceAndValidatorsViolation>,
    ) {
        for (field, value) in [
            ("code", &diagnostic.code),
            ("message", &diagnostic.message),
            ("remediation", &diagnostic.remediation),
            ("evidence_ref", &diagnostic.evidence_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ConformanceAndValidatorsViolation::EmptyDiagnosticField {
                    id: scorecard.scorecard_id.clone(),
                    field_name: field,
                });
            }
        }
    }
}

/// A validation violation for the M5 conformance-and-validators packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ConformanceAndValidatorsViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Scorecard or packet-envelope id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A required validator-diagnostic field is empty.
    EmptyDiagnosticField {
        /// Scorecard id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A scorecard id appears more than once.
    DuplicateScorecardId {
        /// Duplicate scorecard id.
        scorecard_id: String,
    },
    /// A scorecard lists a certification signal more than once.
    DuplicateCertificationSignal {
        /// Scorecard id.
        id: String,
        /// Signal token.
        signal: &'static str,
    },
    /// A scorecard lists a diagnostic code more than once.
    DuplicateDiagnosticCode {
        /// Scorecard id.
        id: String,
        /// Diagnostic code.
        code: String,
    },
    /// A scorecard's certification signals disagree with the recomputed set.
    CertificationSignalsMismatch {
        /// Scorecard id.
        id: String,
    },
    /// A scorecard's stored disposition disagrees with the recomputed value.
    CertificationDispositionMismatch {
        /// Scorecard id.
        id: String,
        /// Stored disposition token.
        stored: &'static str,
        /// Recomputed disposition token.
        computed: &'static str,
    },
    /// A scorecard's stored effective support disagrees with the recomputed value.
    EffectiveSupportMismatch {
        /// Scorecard id.
        id: String,
        /// Stored support token.
        stored: &'static str,
        /// Recomputed support token.
        computed: &'static str,
    },
    /// A scorecard publishes a support claim without evidence or certification.
    SupportClaimedWithoutEvidence {
        /// Scorecard id.
        id: String,
    },
    /// The summary counts disagree with the scorecards.
    SummaryMismatch,
}

impl fmt::Display for M5ConformanceAndValidatorsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::EmptyDiagnosticField { id, field_name } => {
                write!(
                    f,
                    "scorecard {id} has a diagnostic with empty field {field_name}"
                )
            }
            Self::DuplicateScorecardId { scorecard_id } => {
                write!(f, "duplicate scorecard id {scorecard_id}")
            }
            Self::DuplicateCertificationSignal { id, signal } => {
                write!(f, "scorecard {id} repeats certification signal {signal}")
            }
            Self::DuplicateDiagnosticCode { id, code } => {
                write!(f, "scorecard {id} repeats diagnostic code {code}")
            }
            Self::CertificationSignalsMismatch { id } => {
                write!(
                    f,
                    "scorecard {id} certification signals disagree with the recomputed set"
                )
            }
            Self::CertificationDispositionMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "scorecard {id} publishes disposition {stored} but the recomputed disposition is {computed}"
                )
            }
            Self::EffectiveSupportMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "scorecard {id} publishes effective support {stored} but the recomputed value is {computed}"
                )
            }
            Self::SupportClaimedWithoutEvidence { id } => {
                write!(
                    f,
                    "scorecard {id} publishes a support claim without a current, owned, evidence-linked scorecard"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the scorecards")
            }
        }
    }
}

impl Error for M5ConformanceAndValidatorsViolation {}

/// Loads the embedded M5 conformance-and-validators packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ConformanceAndValidators`].
pub fn current_m5_conformance_and_validators(
) -> Result<M5ConformanceAndValidators, serde_json::Error> {
    serde_json::from_str(M5_CONFORMANCE_AND_VALIDATORS_JSON)
}

#[cfg(test)]
mod tests;
