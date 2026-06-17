//! Governed portable-state export/import review-sheet model.
//!
//! This module is the settings-lane contract for the *review* a user or admin
//! sees before a portable-state package is exported or before its contents are
//! restored. It does not implement an export engine, a transport, or a restore
//! mutator; it defines the canonical record those surfaces must emit so an
//! export-review sheet, an import-review sheet, a diagnostics packet, and a
//! support packet all explain the same thing: which selected M5 artifact classes
//! are leaving the machine and under which data-class label, what is redacted and
//! why, what build produced the package, what integrity and signature evidence is
//! attached, and — for an import — how the package compares against current state
//! before any rehydration.
//!
//! The record reuses the portability vocabulary minted by
//! [`crate::m5_portable_state_and_restore`] (artifact classes, exclusion reasons,
//! migration/fidelity labels, missing-dependency kinds) rather than inventing
//! surface-local restore language. It adds the review-only concepts the spec
//! requires: the five explicit data-class labels (local-only, portable, shared,
//! redacted, machine-local), a redaction manifest, producer/build provenance, and
//! a compare-before-restore summary.
//!
//! The gate is fail-closed. A class whose body is secret material, a live
//! authority handle, or a machine-unique trust anchor can never be labeled
//! [`DataClassLabel::Portable`] or [`DataClassLabel::Shared`]; a redacted or
//! machine-local class can never be marked invisible (silently dropped); an
//! import review can never omit its compare summary; and a redacted class must
//! carry a matching redaction-manifest entry. All of these are build-time
//! invariants, so a dishonest review sheet cannot be constructed in the first
//! place.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::m5_portable_state_and_restore::model::{
    is_canonical_object_ref, ExclusionReason, MigrationLabel, MissingDependencyKind,
    PortableArtifactClass,
};

/// Stable record-kind tag for portable-state review-sheet records.
pub const M5_PORTABLE_STATE_REVIEW_RECORD_KIND: &str = "m5_portable_state_review_record";

/// Schema version for [`M5PortableStateReviewSheet`] records.
pub const M5_PORTABLE_STATE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by export, import, diagnostics, and support.
pub const M5_PORTABLE_STATE_REVIEW_SHARED_CONTRACT_REF: &str =
    "settings:m5_portable_state_review:v1";

/// Direction of the portable-state review sheet.
///
/// An export review is shown before a package is written; an import review is
/// shown before a package's contents are restored. The compare-before-restore
/// summary is required for imports and forbidden for exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDirection {
    /// Reviewing what will leave the machine before writing the package.
    Export,
    /// Reviewing what will be applied before rehydrating into current state.
    Import,
}

impl ReviewDirection {
    /// Returns the canonical token for this direction.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Export => "export",
            Self::Import => "import",
        }
    }

    /// Returns true when a compare-before-restore summary is required.
    pub const fn requires_compare(self) -> bool {
        matches!(self, Self::Import)
    }
}

/// Explicit data-class label preserved across export, import, diagnostics, and
/// support packets.
///
/// This is the user-facing classification of one selected artifact class. Unlike
/// the certification's portability disposition, the review label set names
/// [`Self::LocalOnly`] and [`Self::Shared`] explicitly so a reviewer can see at a
/// glance what leaves the machine and in what form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataClassLabel {
    /// Held on this machine only; intentionally excluded from the package.
    LocalOnly,
    /// Carried in full; the class round-trips through export/import.
    Portable,
    /// Carried in full and explicitly cleared for cross-user/fleet sharing.
    Shared,
    /// Carried as reference or metadata only; sensitive bodies are stripped.
    Redacted,
    /// Never serialized; remains on the originating machine (e.g. trust anchors).
    MachineLocal,
}

impl DataClassLabel {
    /// Returns the canonical token for this label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Portable => "portable",
            Self::Shared => "shared",
            Self::Redacted => "redacted",
            Self::MachineLocal => "machine_local",
        }
    }

    /// Returns true when content under this label leaves the machine in any form.
    pub const fn crosses_machine_boundary(self) -> bool {
        matches!(self, Self::Portable | Self::Shared | Self::Redacted)
    }

    /// Returns true when this label carries the class body across in full.
    pub const fn crosses_in_full(self) -> bool {
        matches!(self, Self::Portable | Self::Shared)
    }

    /// Returns true when this label requires an explicit exclusion reason.
    pub const fn requires_exclusion_reason(self) -> bool {
        matches!(self, Self::LocalOnly | Self::Redacted | Self::MachineLocal)
    }

    /// Returns true when a redaction-manifest entry may describe this label.
    pub const fn is_redactable(self) -> bool {
        matches!(self, Self::Redacted | Self::MachineLocal)
    }

    /// Every data-class label.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::Portable,
        Self::Shared,
        Self::Redacted,
        Self::MachineLocal,
    ];
}

/// How a redacted or withheld value was removed before it could leave the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionTechnique {
    /// Secret bodies (tokens, keys, passkeys) omitted entirely.
    SecretOmission,
    /// Live authority handles (tickets, sessions, sockets) omitted entirely.
    HandleOmission,
    /// File-system paths rewritten or stripped to hide local layout.
    PathRedaction,
    /// Host names or network identities stripped to hide the machine.
    HostRedaction,
    /// Body replaced by a reference/pointer rather than the content.
    ReferenceOnly,
}

impl RedactionTechnique {
    /// Returns the canonical token for this technique.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecretOmission => "secret_omission",
            Self::HandleOmission => "handle_omission",
            Self::PathRedaction => "path_redaction",
            Self::HostRedaction => "host_redaction",
            Self::ReferenceOnly => "reference_only",
        }
    }
}

/// Checksum/integrity evidence state for one reviewed class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumState {
    /// A checksum is present and was recomputed and matched.
    Verified,
    /// A checksum is present in the package but was not recomputed.
    Present,
    /// A checksum is present but did not match the content (tamper/corruption).
    Mismatch,
    /// No checksum is available for this class.
    Unavailable,
}

impl ChecksumState {
    /// Returns the canonical token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Present => "present",
            Self::Mismatch => "mismatch",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when integrity is known-broken and must block a commit.
    pub const fn blocks_commit(self) -> bool {
        matches!(self, Self::Mismatch)
    }
}

/// Signature/trust evidence state for one reviewed class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureState {
    /// A signature is present and the signer is trusted on this machine.
    Verified,
    /// A signature is present but trust was not evaluated.
    Present,
    /// A signature is present but the signer is not trusted here.
    Untrusted,
    /// The class is not signed.
    Unsigned,
    /// No signature mechanism is available for this class.
    Unavailable,
}

impl SignatureState {
    /// Returns the canonical token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Present => "present",
            Self::Untrusted => "untrusted",
            Self::Unsigned => "unsigned",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when this state warrants explicit review before a commit.
    pub const fn warrants_review(self) -> bool {
        matches!(self, Self::Untrusted)
    }
}

/// Redacted, never-raw classification of the machine that produced a package.
///
/// The producing host is recorded as a class, never as a raw hostname, so the
/// review can say "this package came from a foreign machine" without leaking a
/// machine-unique identity into a portable record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostProvenanceClass {
    /// The package was produced on this same machine.
    SameMachine,
    /// The package was produced elsewhere within the same managed fleet.
    ManagedFleet,
    /// The package was produced on an unrelated, foreign machine.
    ForeignMachine,
    /// The producing host class could not be determined.
    Unknown,
}

impl HostProvenanceClass {
    /// Returns the canonical token for this host class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameMachine => "same_machine",
            Self::ManagedFleet => "managed_fleet",
            Self::ForeignMachine => "foreign_machine",
            Self::Unknown => "unknown",
        }
    }
}

/// Surface that must render the same review truth before a commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewConsumerSurface {
    /// The export-review sheet shown before a package is written.
    ExportReview,
    /// The import-review sheet shown before a package is restored.
    ImportReview,
    /// The diagnostics packet describing a package.
    Diagnostics,
    /// The support packet describing a package.
    SupportPacket,
}

impl ReviewConsumerSurface {
    /// Returns the canonical token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportReview => "export_review",
            Self::ImportReview => "import_review",
            Self::Diagnostics => "diagnostics",
            Self::SupportPacket => "support_packet",
        }
    }

    /// Required surface set for review parity.
    pub const REQUIRED: [Self; 4] = [
        Self::ExportReview,
        Self::ImportReview,
        Self::Diagnostics,
        Self::SupportPacket,
    ];
}

/// One entry in the package's redaction manifest.
///
/// Every redacted class, and every withheld machine-local class that strips or
/// omits content, declares exactly what was removed and why so the redaction is
/// visible rather than silent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionManifestEntry {
    /// Artifact class whose content was redacted or withheld.
    pub artifact_class: PortableArtifactClass,
    /// How the content was removed.
    pub technique: RedactionTechnique,
    /// Why the content was removed; must match the class row's exclusion reason.
    pub reason: ExclusionReason,
    /// Count of fields/values redacted (for an at-a-glance magnitude).
    pub redacted_field_count: u32,
    /// Human-readable description of what was redacted.
    pub detail: String,
}

/// One reviewed artifact class in the package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewClassRow {
    /// Artifact class being reviewed.
    pub artifact_class: PortableArtifactClass,
    /// Explicit data-class label applied to this class.
    pub data_class: DataClassLabel,
    /// Exclusion reason for local-only, redacted, or machine-local classes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusion_reason: Option<ExclusionReason>,
    /// Estimated serialized size of this class in bytes.
    pub estimated_size_bytes: u64,
    /// Checksum/integrity evidence state for this class.
    pub checksum_state: ChecksumState,
    /// Signature/trust evidence state for this class.
    pub signature_state: SignatureState,
    /// Canonical ref to the included content or its reference manifest.
    pub content_ref: String,
    /// Whether this class is visible in the review (must be true for exclusions).
    pub visible_in_review: bool,
    /// Human-readable rationale shown to the reviewer.
    pub rationale: String,
}

impl ReviewClassRow {
    /// Returns true when this row's content leaves the machine in any form.
    pub fn crosses_machine_boundary(&self) -> bool {
        self.data_class.crosses_machine_boundary()
    }
}

/// Added/removed/changed counts for a compare-before-restore dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeCounts {
    /// Items present in the package but not in current state.
    pub added: u32,
    /// Items present in current state but not in the package.
    pub removed: u32,
    /// Items present in both but materially different.
    pub changed: u32,
}

impl ChangeCounts {
    /// A no-change delta.
    pub const ZERO: Self = Self {
        added: 0,
        removed: 0,
        changed: 0,
    };

    /// Returns true when any add/remove/change is present.
    pub const fn has_changes(self) -> bool {
        self.added > 0 || self.removed > 0 || self.changed > 0
    }

    /// Total number of changed items across all three buckets.
    pub const fn total(self) -> u32 {
        self.added + self.removed + self.changed
    }
}

/// Structured compare-before-restore summary for an import review.
///
/// The summary compares the package's contents against current state so an
/// import can be reviewed before rehydration: which panes and surfaces are
/// added, removed, or changed; which dependency classes would be missing; how
/// many secrets/handles are excluded; and how much path/host redaction the
/// package carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareSummary {
    /// Pane add/remove/change counts between package and current state.
    pub pane_delta: ChangeCounts,
    /// Surface add/remove/change counts between package and current state.
    pub surface_delta: ChangeCounts,
    /// Dependency classes that would be missing on restore.
    pub missing_dependency_classes: Vec<MissingDependencyKind>,
    /// Count of excluded secrets and live handles surfaced in the review.
    pub excluded_secret_handle_count: u32,
    /// Distinct exclusion reasons covered by the excluded classes.
    pub excluded_exclusion_reasons: Vec<ExclusionReason>,
    /// Count of path redactions applied across the package.
    pub path_redaction_count: u32,
    /// Count of host-identity redactions applied across the package.
    pub host_redaction_count: u32,
    /// Weakest restore-fidelity label implied by this comparison.
    pub fidelity_ceiling: MigrationLabel,
}

impl CompareSummary {
    /// Returns true when the comparison materially changes restore behavior.
    ///
    /// A comparison is material when panes or surfaces change, a dependency
    /// class would be missing, or the fidelity ceiling is below exact. Excluded
    /// secrets and redaction counts alone are expected and do not, by themselves,
    /// make a restore material.
    pub fn materially_changes_restore(&self) -> bool {
        self.pane_delta.has_changes()
            || self.surface_delta.has_changes()
            || !self.missing_dependency_classes.is_empty()
            || !self.fidelity_ceiling.implies_exact_fidelity()
    }
}

/// Producer/build provenance recorded for a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerProvenance {
    /// Product build label, e.g. the release/channel display string.
    pub product_build_label: String,
    /// Short commit identifier of the producing build.
    pub build_commit_short: String,
    /// Toolchain/release channel of the producing build.
    pub build_channel: String,
    /// Whether the producing build had uncommitted local changes.
    pub build_dirty: bool,
    /// Schema version the package was written under.
    pub package_schema_version: String,
    /// Schema version the importing/exporting client targets.
    pub target_schema_version: String,
    /// Redacted class of the producing host.
    pub host_class: HostProvenanceClass,
    /// Platform class (os/arch) of the producing build.
    pub platform: String,
}

impl ProducerProvenance {
    /// Returns true when the package and target schema versions match.
    pub fn schema_versions_match(&self) -> bool {
        self.package_schema_version == self.target_schema_version
    }
}

/// One surface-parity row for review truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSurfaceRow {
    /// Consumer surface.
    pub surface: ReviewConsumerSurface,
    /// Whether the surface consumes this shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface shows per-class data-class labels.
    pub shows_data_class_labels: bool,
    /// Whether the surface shows the redaction manifest.
    pub shows_redaction_manifest: bool,
    /// Whether the surface shows machine-local exclusions.
    pub shows_machine_local_exclusions: bool,
    /// Whether the surface shows producer/build provenance.
    pub shows_provenance: bool,
    /// Whether the surface shows the compare-before-restore summary.
    pub shows_compare: bool,
}

/// Derived pillar verdicts for the review contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPillars {
    /// Producer/build provenance and schema versions are present.
    pub provenance_present: bool,
    /// Redacted and machine-local classes are visible, never silently dropped.
    pub exclusions_visible: bool,
    /// No secret/handle/trust-anchor class crosses as portable or shared.
    pub secret_boundary_held: bool,
    /// No crossing class carries a known-broken checksum.
    pub integrity_reviewable: bool,
    /// An import carries its compare-before-restore summary.
    pub compare_available: bool,
    /// All required surfaces render the same review truth.
    pub labels_preserved: bool,
    /// The review carries structured per-class detail, not a size-only summary.
    pub structured_review_present: bool,
}

/// Reason a review sheet is narrowed below ready-to-commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewNarrowingReason {
    /// Producer/build provenance is incomplete.
    ProvenanceIncomplete,
    /// One or more surfaces omit required review truth.
    LabelsNotPreserved,
    /// The review reduces to a size/timestamp summary without structured detail.
    ReviewNotStructured,
    /// An import review is missing its compare-before-restore summary.
    CompareUnavailable,
    /// A crossing class carries a known-broken checksum.
    IntegrityMismatch,
    /// The comparison materially changes restore behavior.
    MaterialRestoreChange,
    /// A crossing class carries an untrusted signature.
    SignatureUntrusted,
    /// The package and target schema versions differ.
    SchemaVersionMismatch,
}

/// Derived readiness verdict for a review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewReadiness {
    /// Every pillar holds and nothing requires explicit review; safe to commit.
    Reviewable,
    /// Sound, but a material change, untrusted signature, or schema mismatch
    /// requires explicit review before the commit.
    ReviewRequired,
    /// A structural pillar failed; the package is not safely committable as-is.
    Blocked,
}

/// Derived qualification for the whole review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewQualification {
    /// Derived readiness class.
    pub readiness: ReviewReadiness,
    /// Whether the package and target schema versions match.
    pub schema_versions_match: bool,
    /// Whether the comparison materially changes restore behavior.
    pub materially_changes_restore: bool,
    /// Named narrowing reasons.
    pub narrowing_reasons: Vec<ReviewNarrowingReason>,
}

/// Input used to build a [`M5PortableStateReviewSheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PortableStateReviewInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Review direction.
    pub direction: ReviewDirection,
    /// Canonical ref to the package being reviewed.
    pub package_ref: String,
    /// Producer/build provenance.
    pub provenance: ProducerProvenance,
    /// Reviewed artifact-class rows.
    pub class_rows: Vec<ReviewClassRow>,
    /// Redaction manifest entries.
    pub redaction_manifest: Vec<RedactionManifestEntry>,
    /// Compare-before-restore summary (required for imports, forbidden for exports).
    pub compare: Option<CompareSummary>,
    /// Surface-parity rows.
    pub surfaces: Vec<ReviewSurfaceRow>,
}

/// Canonical portable-state export/import review-sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableStateReviewSheet {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Review direction.
    pub direction: ReviewDirection,
    /// Canonical ref to the package being reviewed.
    pub package_ref: String,
    /// Producer/build provenance.
    pub provenance: ProducerProvenance,
    /// Reviewed artifact-class rows.
    pub class_rows: Vec<ReviewClassRow>,
    /// Redaction manifest entries.
    pub redaction_manifest: Vec<RedactionManifestEntry>,
    /// Compare-before-restore summary, present only for imports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare: Option<CompareSummary>,
    /// Surface-parity rows.
    pub surfaces: Vec<ReviewSurfaceRow>,
    /// Data-class labels covered by the rows.
    pub data_class_coverage: Vec<DataClassLabel>,
    /// Redaction techniques covered by the manifest.
    pub redaction_technique_coverage: Vec<RedactionTechnique>,
    /// Missing-dependency kinds surfaced by the comparison.
    pub missing_dependency_coverage: Vec<MissingDependencyKind>,
    /// Total estimated package size in bytes across all rows.
    pub total_estimated_size_bytes: u64,
    /// Estimated size in bytes of classes that cross the machine boundary.
    pub crossing_estimated_size_bytes: u64,
    /// Derived pillar verdicts.
    pub pillars: ReviewPillars,
    /// Derived readiness qualification.
    pub qualification: ReviewQualification,
}

/// Reasons a portable-state review sheet cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// No reviewed class rows were supplied.
    NoReviewRows,
    /// An artifact class is reviewed more than once.
    DuplicateArtifactClass {
        /// The duplicated artifact class.
        class: PortableArtifactClass,
    },
    /// A canonical ref field is invalid.
    NonCanonicalRef {
        /// The field carrying the invalid ref.
        field: &'static str,
        /// The offending value.
        value: String,
    },
    /// A class carried as portable/shared names a serialization-forbidden body.
    SecretCarriedAsFullClass {
        /// The artifact class that would leak.
        class: PortableArtifactClass,
        /// The forbidden exclusion reason it declared.
        reason: ExclusionReason,
    },
    /// A portable/shared class declares an exclusion reason it cannot honor.
    FullClassWithExclusionReason {
        /// The class carried in full but declaring an exclusion.
        class: PortableArtifactClass,
    },
    /// A local-only, redacted, or machine-local class lacks an exclusion reason.
    MissingExclusionReason {
        /// The class without an exclusion reason.
        class: PortableArtifactClass,
    },
    /// A redacted or machine-local class is marked invisible (silently dropped).
    ExclusionSilentlyDropped {
        /// The class hidden from the review.
        class: PortableArtifactClass,
    },
    /// A redacted class has no matching redaction-manifest entry.
    RedactedClassMissingManifest {
        /// The redacted class without a manifest entry.
        class: PortableArtifactClass,
    },
    /// A redaction-manifest entry references a class not present in the rows.
    RedactionManifestUnknownClass {
        /// The unknown class.
        class: PortableArtifactClass,
    },
    /// A redaction-manifest entry references a non-redactable class.
    RedactionManifestForNonRedactableClass {
        /// The class that is neither redacted nor machine-local.
        class: PortableArtifactClass,
    },
    /// A redaction-manifest reason disagrees with the class row's reason.
    RedactionReasonMismatch {
        /// The class with mismatched reasons.
        class: PortableArtifactClass,
    },
    /// A class is described by more than one redaction-manifest entry.
    DuplicateRedactionManifest {
        /// The duplicated class.
        class: PortableArtifactClass,
    },
    /// An import review is missing its compare-before-restore summary.
    ImportReviewMissingCompare,
    /// An export review carries a compare-before-restore summary.
    ExportReviewWithCompare,
    /// A required provenance field is empty.
    EmptyProvenanceField {
        /// The empty field.
        field: &'static str,
    },
    /// A required consumer surface is missing.
    MissingConsumerSurface {
        /// The missing surface.
        surface: ReviewConsumerSurface,
    },
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoReviewRows => write!(f, "at least one reviewed class row is required"),
            Self::DuplicateArtifactClass { class } => {
                write!(f, "duplicated artifact class `{}`", class.as_str())
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::SecretCarriedAsFullClass { class, reason } => write!(
                f,
                "class `{}` cannot cross in full while declaring `{}`",
                class.as_str(),
                reason.as_str()
            ),
            Self::FullClassWithExclusionReason { class } => write!(
                f,
                "portable/shared class `{}` must not declare an exclusion reason",
                class.as_str()
            ),
            Self::MissingExclusionReason { class } => write!(
                f,
                "local-only/redacted/machine-local class `{}` requires an exclusion reason",
                class.as_str()
            ),
            Self::ExclusionSilentlyDropped { class } => write!(
                f,
                "excluded class `{}` must stay visible in the review",
                class.as_str()
            ),
            Self::RedactedClassMissingManifest { class } => write!(
                f,
                "redacted class `{}` requires a redaction-manifest entry",
                class.as_str()
            ),
            Self::RedactionManifestUnknownClass { class } => write!(
                f,
                "redaction-manifest references unknown class `{}`",
                class.as_str()
            ),
            Self::RedactionManifestForNonRedactableClass { class } => write!(
                f,
                "redaction-manifest references non-redactable class `{}`",
                class.as_str()
            ),
            Self::RedactionReasonMismatch { class } => write!(
                f,
                "redaction-manifest reason disagrees with row reason for class `{}`",
                class.as_str()
            ),
            Self::DuplicateRedactionManifest { class } => {
                write!(
                    f,
                    "duplicated redaction-manifest entry for `{}`",
                    class.as_str()
                )
            }
            Self::ImportReviewMissingCompare => {
                write!(
                    f,
                    "an import review requires a compare-before-restore summary"
                )
            }
            Self::ExportReviewWithCompare => {
                write!(f, "an export review must not carry a compare summary")
            }
            Self::EmptyProvenanceField { field } => {
                write!(f, "provenance field `{field}` must not be empty")
            }
            Self::MissingConsumerSurface { surface } => {
                write!(f, "missing consumer surface `{}`", surface.as_str())
            }
        }
    }
}

impl std::error::Error for BuildError {}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_owned(),
        })
    }
}

fn require_nonempty(field: &'static str, value: &str) -> Result<(), BuildError> {
    if value.trim().is_empty() {
        Err(BuildError::EmptyProvenanceField { field })
    } else {
        Ok(())
    }
}

impl M5PortableStateReviewSheet {
    /// Builds a derived review sheet from raw evidence rows.
    ///
    /// Returns a [`BuildError`] when a structural invariant or a fail-closed
    /// guardrail is violated, so a review that would leak secrets, hide an
    /// exclusion, or omit an import comparison cannot be constructed.
    pub fn build(mut input: M5PortableStateReviewInput) -> Result<Self, BuildError> {
        if input.class_rows.is_empty() {
            return Err(BuildError::NoReviewRows);
        }

        require_ref("package_ref", &input.package_ref)?;
        require_nonempty("product_build_label", &input.provenance.product_build_label)?;
        require_nonempty("build_commit_short", &input.provenance.build_commit_short)?;
        require_nonempty("build_channel", &input.provenance.build_channel)?;
        require_nonempty(
            "package_schema_version",
            &input.provenance.package_schema_version,
        )?;
        require_nonempty(
            "target_schema_version",
            &input.provenance.target_schema_version,
        )?;
        require_nonempty("platform", &input.provenance.platform)?;

        // Class rows: unique, refs valid, the secret boundary held, exclusion
        // reasons present where required, and exclusions never silently dropped.
        let mut seen_classes = BTreeSet::new();
        for row in &input.class_rows {
            if !seen_classes.insert(row.artifact_class) {
                return Err(BuildError::DuplicateArtifactClass {
                    class: row.artifact_class,
                });
            }
            require_ref("class_rows.content_ref", &row.content_ref)?;

            if row.data_class.crosses_in_full() {
                if let Some(reason) = row.exclusion_reason {
                    if reason.forbids_serialization() {
                        return Err(BuildError::SecretCarriedAsFullClass {
                            class: row.artifact_class,
                            reason,
                        });
                    }
                    return Err(BuildError::FullClassWithExclusionReason {
                        class: row.artifact_class,
                    });
                }
            } else if row.data_class.requires_exclusion_reason() && row.exclusion_reason.is_none() {
                return Err(BuildError::MissingExclusionReason {
                    class: row.artifact_class,
                });
            }

            // Any class that does not cross in full is an exclusion and must
            // stay visible in the review rather than being silently dropped.
            if !row.data_class.crosses_in_full() && !row.visible_in_review {
                return Err(BuildError::ExclusionSilentlyDropped {
                    class: row.artifact_class,
                });
            }
        }

        // Redaction manifest: unique, references known redactable rows, and
        // reasons agree with the class rows. Every redacted class needs an entry.
        let row_label: std::collections::BTreeMap<PortableArtifactClass, &ReviewClassRow> = input
            .class_rows
            .iter()
            .map(|row| (row.artifact_class, row))
            .collect();
        let mut manifest_classes = BTreeSet::new();
        for entry in &input.redaction_manifest {
            if !manifest_classes.insert(entry.artifact_class) {
                return Err(BuildError::DuplicateRedactionManifest {
                    class: entry.artifact_class,
                });
            }
            let Some(row) = row_label.get(&entry.artifact_class) else {
                return Err(BuildError::RedactionManifestUnknownClass {
                    class: entry.artifact_class,
                });
            };
            if !row.data_class.is_redactable() {
                return Err(BuildError::RedactionManifestForNonRedactableClass {
                    class: entry.artifact_class,
                });
            }
            if row.exclusion_reason != Some(entry.reason) {
                return Err(BuildError::RedactionReasonMismatch {
                    class: entry.artifact_class,
                });
            }
        }
        for row in &input.class_rows {
            if row.data_class == DataClassLabel::Redacted
                && !manifest_classes.contains(&row.artifact_class)
            {
                return Err(BuildError::RedactedClassMissingManifest {
                    class: row.artifact_class,
                });
            }
        }

        // Direction/compare coupling.
        match (input.direction, input.compare.is_some()) {
            (ReviewDirection::Import, false) => return Err(BuildError::ImportReviewMissingCompare),
            (ReviewDirection::Export, true) => return Err(BuildError::ExportReviewWithCompare),
            _ => {}
        }

        let present_surfaces: BTreeSet<ReviewConsumerSurface> =
            input.surfaces.iter().map(|row| row.surface).collect();
        for surface in ReviewConsumerSurface::REQUIRED {
            if !present_surfaces.contains(&surface) {
                return Err(BuildError::MissingConsumerSurface { surface });
            }
        }

        input.class_rows.sort_by_key(|row| row.artifact_class);
        input
            .redaction_manifest
            .sort_by_key(|entry| entry.artifact_class);
        input.surfaces.sort_by_key(|row| row.surface);

        let data_class_coverage: Vec<DataClassLabel> = input
            .class_rows
            .iter()
            .map(|row| row.data_class)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let redaction_technique_coverage: Vec<RedactionTechnique> = input
            .redaction_manifest
            .iter()
            .map(|entry| entry.technique)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let missing_dependency_coverage: Vec<MissingDependencyKind> = input
            .compare
            .as_ref()
            .map(|compare| {
                compare
                    .missing_dependency_classes
                    .iter()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            })
            .unwrap_or_default();

        let total_estimated_size_bytes = input
            .class_rows
            .iter()
            .map(|row| row.estimated_size_bytes)
            .sum();
        let crossing_estimated_size_bytes = input
            .class_rows
            .iter()
            .filter(|row| row.crosses_machine_boundary())
            .map(|row| row.estimated_size_bytes)
            .sum();

        // Pillars. Provenance fields were validated non-empty above; a schema
        // mismatch is handled as a review-required narrowing, not a hard block.
        let provenance_present = !input.provenance.product_build_label.trim().is_empty()
            && !input.provenance.build_commit_short.trim().is_empty()
            && !input.provenance.build_channel.trim().is_empty()
            && !input.provenance.package_schema_version.trim().is_empty()
            && !input.provenance.target_schema_version.trim().is_empty()
            && !input.provenance.platform.trim().is_empty();

        let exclusions_visible = input
            .class_rows
            .iter()
            .all(|row| row.data_class.crosses_in_full() || row.visible_in_review);

        let secret_boundary_held = input.class_rows.iter().all(|row| {
            !(row.data_class.crosses_in_full()
                && row
                    .exclusion_reason
                    .is_some_and(ExclusionReason::forbids_serialization))
        });

        let integrity_reviewable = input
            .class_rows
            .iter()
            .all(|row| !(row.crosses_machine_boundary() && row.checksum_state.blocks_commit()));

        let compare_available = match input.direction {
            ReviewDirection::Import => input.compare.is_some(),
            ReviewDirection::Export => true,
        };

        let labels_preserved = ReviewConsumerSurface::REQUIRED.iter().all(|surface| {
            input.surfaces.iter().any(|row| {
                row.surface == *surface
                    && row.consumes_shared_record
                    && row.shows_data_class_labels
                    && row.shows_redaction_manifest
                    && row.shows_machine_local_exclusions
                    && row.shows_provenance
                    && row.shows_compare
            })
        });

        let structured_review_present = input
            .class_rows
            .iter()
            .all(|row| !row.rationale.trim().is_empty())
            && (input.direction == ReviewDirection::Export || input.compare.is_some());

        let pillars = ReviewPillars {
            provenance_present,
            exclusions_visible,
            secret_boundary_held,
            integrity_reviewable,
            compare_available,
            labels_preserved,
            structured_review_present,
        };

        // Qualification.
        let schema_versions_match = input.provenance.schema_versions_match();
        let materially_changes_restore = input
            .compare
            .as_ref()
            .is_some_and(CompareSummary::materially_changes_restore);
        let signature_untrusted = input
            .class_rows
            .iter()
            .any(|row| row.crosses_machine_boundary() && row.signature_state.warrants_review());

        let mut narrowing_reasons = Vec::new();
        if !pillars.provenance_present {
            narrowing_reasons.push(ReviewNarrowingReason::ProvenanceIncomplete);
        }
        if !pillars.labels_preserved {
            narrowing_reasons.push(ReviewNarrowingReason::LabelsNotPreserved);
        }
        if !pillars.structured_review_present {
            narrowing_reasons.push(ReviewNarrowingReason::ReviewNotStructured);
        }
        if !pillars.compare_available {
            narrowing_reasons.push(ReviewNarrowingReason::CompareUnavailable);
        }
        if !pillars.integrity_reviewable {
            narrowing_reasons.push(ReviewNarrowingReason::IntegrityMismatch);
        }
        if materially_changes_restore {
            narrowing_reasons.push(ReviewNarrowingReason::MaterialRestoreChange);
        }
        if signature_untrusted {
            narrowing_reasons.push(ReviewNarrowingReason::SignatureUntrusted);
        }
        if input.direction == ReviewDirection::Import && !schema_versions_match {
            narrowing_reasons.push(ReviewNarrowingReason::SchemaVersionMismatch);
        }

        let structural_ok = pillars.provenance_present
            && pillars.labels_preserved
            && pillars.structured_review_present
            && pillars.compare_available
            && pillars.integrity_reviewable
            && pillars.exclusions_visible
            && pillars.secret_boundary_held;

        let readiness = if !structural_ok {
            ReviewReadiness::Blocked
        } else if materially_changes_restore
            || signature_untrusted
            || (input.direction == ReviewDirection::Import && !schema_versions_match)
        {
            ReviewReadiness::ReviewRequired
        } else {
            ReviewReadiness::Reviewable
        };

        let qualification = ReviewQualification {
            readiness,
            schema_versions_match,
            materially_changes_restore,
            narrowing_reasons,
        };

        Ok(Self {
            record_kind: M5_PORTABLE_STATE_REVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_PORTABLE_STATE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: M5_PORTABLE_STATE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            record_id: input.record_id,
            as_of: input.as_of,
            summary: input.summary,
            direction: input.direction,
            package_ref: input.package_ref,
            provenance: input.provenance,
            class_rows: input.class_rows,
            redaction_manifest: input.redaction_manifest,
            compare: input.compare,
            surfaces: input.surfaces,
            data_class_coverage,
            redaction_technique_coverage,
            missing_dependency_coverage,
            total_estimated_size_bytes,
            crossing_estimated_size_bytes,
            pillars,
            qualification,
        })
    }

    /// Renders a compact, export-safe support summary from the shared record.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("record_id: {}", self.record_id),
            format!("direction: {}", self.direction.as_str()),
            format!("readiness: {:?}", self.qualification.readiness),
            format!("class_rows: {}", self.class_rows.len()),
            format!("redactions: {}", self.redaction_manifest.len()),
            format!(
                "crossing_size_bytes: {}",
                self.crossing_estimated_size_bytes
            ),
            format!("schema_match: {}", self.qualification.schema_versions_match),
            format!(
                "material_change: {}",
                self.qualification.materially_changes_restore
            ),
        ]
    }
}
