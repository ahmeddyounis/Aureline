//! Typed register of third-party import provenance and local-fork review truth per M5 import.
//!
//! The sibling [`m5_boundary_and_upstream_durability`](crate::m5_boundary_and_upstream_durability)
//! matrix records, per asset lane, *whether* a third-party-import or generated-code control
//! is satisfied as one coarse [`ControlState`](crate::m5_boundary_and_upstream_durability::ControlState),
//! and the [`m5_compliance_and_notice_binding`](crate::m5_compliance_and_notice_binding)
//! register publishes DCO/CLA, licensing, and SBOM/notice truth per artifact family. Neither
//! makes each protected-path import inspectable as a durable record: where it came from, what
//! license it carries, which upstream version it pins, how far it has diverged, who owns its
//! updates, who generated it and how to regenerate it, and — for a long-lived fork or an
//! effectively single-source import — whether an explicit sponsor/fork/replace decision and a
//! current divergence review exist.
//!
//! This module is that import-truth layer. For every protected-path import used by an M5
//! family it records one [`ImportRecord`] that states, in one copy-safe record:
//!
//! - the **import provenance** ([`ImportProvenance`]): whether the origin is attributed, the
//!   SPDX license is identified, and the upstream version is pinned;
//! - the **update ownership** ([`UpdateOwnership`]): whether the import has an assigned update
//!   owner — so a critical import is never left ownerless because it is "just build-time";
//! - the **divergence profile** ([`DivergenceProfile`]): the local-modification posture, the
//!   divergence age, and the divergence-review state;
//! - the **sponsor/fork/replace decision** ([`DecisionRecord`]): required for a long-lived
//!   fork or single-source import, so a curated dependency never drifts into quiet permanent
//!   divergence;
//! - the **generated-code provenance** ([`GeneratorProvenance`]): the generator identity and
//!   the regeneration path — never buried for checked-in generated code.
//!
//! Each record also carries a [`manifest_scan_posture`](ImportRecord::manifest_scan_posture)
//! (what the dependency-health/import scan found) and a
//! [`surface_posture`](ImportRecord::surface_posture) (what the user/admin import surface
//! shows). The two **must agree**: a record may never show a clean surface over a scan that
//! found gaps, so a clean import card can never mask an ownerless, unattributed, or
//! generator-free import.
//!
//! A record is [`ImportState::Cleared`] only when provenance holds, the import is owned, any
//! required divergence review is current, any required decision is recorded, generated-code
//! provenance is complete, the proof is fresh, and the owner signed. Otherwise it narrows on
//! the *specific* axis that thinned out — a provenance gap, an ownership gap, a
//! divergence/decision gap, a generator gap, or stale proof — never collapsing to one global
//! flag. A narrowed record drops its [`ImportRecord::effective_label`] below the launch
//! cutline and may never publish an effective label wider than the one it declares.
//!
//! The [`ImportRule`] set names the closed conditions that gate promotion. An *inherited*
//! narrowing — a subject whose declared label already sits below the cutline, or a gap held by
//! an unexpired waiver — is gated upstream and does not itself hold promotion; an
//! *import-layer* failure on a subject whose declared label is still at or above the cutline
//! holds promotion through a stop rule, recorded in [`ImportRegister::publication`]. The
//! cross-cutting [`ManifestSurfaceParity`] block summarizes scan/surface agreement over every
//! subject.
//!
//! The register is checked in at
//! `artifacts/governance/m5-import-provenance-and-fork-review.json` and embedded here, so this
//! typed consumer and the CI gate agree on every record without a cargo build in CI. The model
//! is metadata-only: every field is a typed state, a boolean flag, a small count, a label, or
//! an opaque ref. It carries no credential bodies, raw provider payloads, source contents, or
//! signatures. Date arithmetic (recomputing proof, review, and waiver freshness against an
//! `as_of` date) lives in the CI gate and the integration test; this model enforces the
//! invariants that hold regardless of the clock: scan/surface parity, the no-widening ceiling,
//! control/fact consistency, reason/state coherence, summary agreement, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket, SupportClass, Waiver,
};
use crate::m5_versioned_boundary_manifests::M5Family;

/// Supported register schema version.
pub const M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_RECORD_KIND: &str =
    "m5_import_provenance_and_fork_review_register";

/// Repo-relative path to the checked-in register.
pub const M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_PATH: &str =
    "artifacts/governance/m5-import-provenance-and-fork-review.json";

/// Embedded checked-in register JSON.
pub const M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-import-provenance-and-fork-review.json"
));

/// The kind of import a record governs.
///
/// The same import truth is published for vendored third-party imports, checked-in generated
/// artifacts, long-lived local forks, and effectively single-source curated imports — so a gap
/// on a generated artifact or a quietly drifting fork cannot hide behind a clean vendored
/// import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportKind {
    /// A vendored/bundled third-party source import.
    ThirdPartyImport,
    /// Checked-in generated code produced by a generator.
    GeneratedArtifact,
    /// A long-lived local fork of an upstream.
    LocalFork,
    /// An effectively single-source curated import.
    CuratedSingleSource,
}

impl ImportKind {
    /// Every import kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ThirdPartyImport,
        Self::GeneratedArtifact,
        Self::LocalFork,
        Self::CuratedSingleSource,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThirdPartyImport => "third_party_import",
            Self::GeneratedArtifact => "generated_artifact",
            Self::LocalFork => "local_fork",
            Self::CuratedSingleSource => "curated_single_source",
        }
    }

    /// True when this kind is a long-lived fork or single-source import that requires an
    /// explicit sponsor/fork/replace decision.
    pub fn requires_decision(self) -> bool {
        matches!(self, Self::LocalFork | Self::CuratedSingleSource)
    }

    /// True when this kind is checked-in generated code that must record its generator
    /// provenance.
    pub fn is_generated(self) -> bool {
        matches!(self, Self::GeneratedArtifact)
    }
}

/// Whether the import's origin is attributed to an upstream source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginState {
    /// The origin is attributed to a recorded upstream source.
    Attributed,
    /// The origin is not attributed.
    Unattributed,
}

impl OriginState {
    /// Every origin state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Attributed, Self::Unattributed];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attributed => "attributed",
            Self::Unattributed => "unattributed",
        }
    }
}

/// Whether the import's license is identified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseState {
    /// An SPDX license is identified for the import.
    Identified,
    /// No license is identified.
    Unidentified,
    /// A license does not apply (e.g. first-party generated code).
    NotApplicable,
}

impl LicenseState {
    /// Every license state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Identified, Self::Unidentified, Self::NotApplicable];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identified => "identified",
            Self::Unidentified => "unidentified",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the import pins a specific upstream version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpstreamPinState {
    /// A specific upstream version is pinned.
    Pinned,
    /// The import tracks a floating upstream branch rather than a pin.
    Floating,
    /// An upstream version does not apply (e.g. first-party generated code).
    NotApplicable,
}

impl UpstreamPinState {
    /// Every pin state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Pinned, Self::Floating, Self::NotApplicable];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Floating => "floating",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the import has an assigned update owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipState {
    /// The import has an assigned update owner.
    Owned,
    /// The import has no assigned update owner.
    Ownerless,
}

impl OwnershipState {
    /// Every ownership state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Owned, Self::Ownerless];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Owned => "owned",
            Self::Ownerless => "ownerless",
        }
    }
}

/// The local-modification posture of an import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceState {
    /// No local modifications; the import tracks upstream exactly.
    InSync,
    /// Local modifications are present.
    Diverged,
    /// The import is maintained as a long-lived fork.
    Forked,
}

impl DivergenceState {
    /// Every divergence state, in declaration order.
    pub const ALL: [Self; 3] = [Self::InSync, Self::Diverged, Self::Forked];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::Diverged => "diverged",
            Self::Forked => "forked",
        }
    }

    /// True when this posture requires a divergence review (anything other than in-sync).
    pub fn requires_review(self) -> bool {
        matches!(self, Self::Diverged | Self::Forked)
    }
}

/// The state of an import's divergence review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceReviewState {
    /// The divergence review is current.
    Current,
    /// The divergence review has aged past its window.
    Stale,
    /// No divergence review is captured.
    Missing,
    /// A divergence review does not apply (the import is in sync).
    NotRequired,
}

impl DivergenceReviewState {
    /// Every review state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Missing, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::NotRequired => "not_required",
        }
    }
}

/// The state of an import's sponsor/fork/replace decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionState {
    /// A sponsor/fork/replace decision is recorded.
    Recorded,
    /// A decision is required but still pending.
    Pending,
    /// A decision does not apply to this import.
    NotRequired,
}

impl DecisionState {
    /// Every decision state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Recorded, Self::Pending, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
            Self::Pending => "pending",
            Self::NotRequired => "not_required",
        }
    }
}

/// The disposition a recorded decision settles on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionDisposition {
    /// Sponsor the upstream so it stays maintained.
    SponsorUpstream,
    /// Maintain the local fork deliberately.
    MaintainFork,
    /// Replace the dependency with another source.
    ReplaceDependency,
    /// No disposition (no recorded decision).
    None,
}

impl DecisionDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SponsorUpstream,
        Self::MaintainFork,
        Self::ReplaceDependency,
        Self::None,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SponsorUpstream => "sponsor_upstream",
            Self::MaintainFork => "maintain_fork",
            Self::ReplaceDependency => "replace_dependency",
            Self::None => "none",
        }
    }

    /// True when the disposition names a settled choice.
    pub fn is_settled(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The posture a scan or a surface reports for an import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    /// No import gap found.
    Clear,
    /// One or more import gaps found.
    GapsFound,
}

impl Posture {
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

/// An import-governance control dimension a record must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// Import provenance: origin attributed, license identified, upstream pinned.
    ImportProvenance,
    /// Update ownership: an assigned update owner.
    UpdateOwnership,
    /// Divergence review: a current review for a diverged/forked import.
    DivergenceReview,
    /// Decision path: a recorded sponsor/fork/replace decision for a long-lived import.
    DecisionPath,
    /// Generator provenance: generator identity and regeneration path for generated code.
    GeneratorProvenance,
    /// Manifest/surface parity: the import scan and the user/admin surface agree.
    ManifestSurfaceParity,
}

impl ControlDimension {
    /// Every control dimension, in declaration order. Every record declares each once.
    pub const ALL: [Self; 6] = [
        Self::ImportProvenance,
        Self::UpdateOwnership,
        Self::DivergenceReview,
        Self::DecisionPath,
        Self::GeneratorProvenance,
        Self::ManifestSurfaceParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportProvenance => "import_provenance",
            Self::UpdateOwnership => "update_ownership",
            Self::DivergenceReview => "divergence_review",
            Self::DecisionPath => "decision_path",
            Self::GeneratorProvenance => "generator_provenance",
            Self::ManifestSurfaceParity => "manifest_surface_parity",
        }
    }
}

/// Satisfaction state of one control binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The control holds for this import.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this import.
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
pub enum ImportState {
    /// Provenance, ownership, divergence/decision, generator, and proof all hold.
    Cleared,
    /// An origin/license/upstream provenance gap is present.
    NarrowedProvenance,
    /// The import is ownerless.
    NarrowedOwnership,
    /// A divergence review or a sponsor/fork/replace decision is missing or stale.
    NarrowedDivergence,
    /// Generated-code generator identity or regeneration path is missing.
    NarrowedGenerator,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The import is withdrawn.
    Withdrawn,
}

impl ImportState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Cleared,
        Self::NarrowedProvenance,
        Self::NarrowedOwnership,
        Self::NarrowedDivergence,
        Self::NarrowedGenerator,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cleared => "cleared",
            Self::NarrowedProvenance => "narrowed_provenance",
            Self::NarrowedOwnership => "narrowed_ownership",
            Self::NarrowedDivergence => "narrowed_divergence",
            Self::NarrowedGenerator => "narrowed_generator",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not cleared, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Cleared | Self::Withdrawn)
    }
}

/// A reason a record narrowed. Closed vocabulary; every reason is watched by an
/// [`ImportRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReason {
    /// The import origin is not attributed.
    OriginUnattributed,
    /// The import license is not identified.
    LicenseUnidentified,
    /// The upstream version is floating rather than pinned.
    UpstreamVersionFloating,
    /// The import has no assigned update owner.
    UpdateOwnerMissing,
    /// The divergence review has aged past its window.
    DivergenceReviewStale,
    /// No divergence review is captured for a diverged/forked import.
    DivergenceReviewMissing,
    /// No sponsor/fork/replace decision is recorded for a long-lived import.
    DecisionRecordMissing,
    /// Checked-in generated code does not record its generator identity.
    GeneratorIdentityMissing,
    /// Checked-in generated code does not record its regeneration path.
    RegenerationPathMissing,
    /// The import proof packet aged past its freshness SLO.
    ImportProofStale,
    /// No import proof packet is captured.
    ImportProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl ImportReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::OriginUnattributed,
        Self::LicenseUnidentified,
        Self::UpstreamVersionFloating,
        Self::UpdateOwnerMissing,
        Self::DivergenceReviewStale,
        Self::DivergenceReviewMissing,
        Self::DecisionRecordMissing,
        Self::GeneratorIdentityMissing,
        Self::RegenerationPathMissing,
        Self::ImportProofStale,
        Self::ImportProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginUnattributed => "origin_unattributed",
            Self::LicenseUnidentified => "license_unidentified",
            Self::UpstreamVersionFloating => "upstream_version_floating",
            Self::UpdateOwnerMissing => "update_owner_missing",
            Self::DivergenceReviewStale => "divergence_review_stale",
            Self::DivergenceReviewMissing => "divergence_review_missing",
            Self::DecisionRecordMissing => "decision_record_missing",
            Self::GeneratorIdentityMissing => "generator_identity_missing",
            Self::RegenerationPathMissing => "regeneration_path_missing",
            Self::ImportProofStale => "import_proof_stale",
            Self::ImportProofMissing => "import_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            ImportState::NarrowedProvenance => 0,
            ImportState::NarrowedOwnership => 1,
            ImportState::NarrowedDivergence => 2,
            ImportState::NarrowedGenerator => 3,
            _ => 4,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> ImportState {
        match self {
            Self::OriginUnattributed
            | Self::LicenseUnidentified
            | Self::UpstreamVersionFloating => ImportState::NarrowedProvenance,
            Self::UpdateOwnerMissing => ImportState::NarrowedOwnership,
            Self::DivergenceReviewStale
            | Self::DivergenceReviewMissing
            | Self::DecisionRecordMissing => ImportState::NarrowedDivergence,
            Self::GeneratorIdentityMissing | Self::RegenerationPathMissing => {
                ImportState::NarrowedGenerator
            }
            Self::ImportProofStale
            | Self::ImportProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ImportState::NarrowedStale,
        }
    }

    /// The control dimension this reason belongs to.
    pub const fn dimension(self) -> ControlDimension {
        match self {
            Self::OriginUnattributed
            | Self::LicenseUnidentified
            | Self::UpstreamVersionFloating => ControlDimension::ImportProvenance,
            Self::UpdateOwnerMissing => ControlDimension::UpdateOwnership,
            Self::DivergenceReviewStale | Self::DivergenceReviewMissing => {
                ControlDimension::DivergenceReview
            }
            Self::DecisionRecordMissing => ControlDimension::DecisionPath,
            Self::GeneratorIdentityMissing | Self::RegenerationPathMissing => {
                ControlDimension::GeneratorProvenance
            }
            Self::ImportProofStale
            | Self::ImportProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ControlDimension::ManifestSurfaceParity,
        }
    }
}

/// An action an [`ImportRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Attribute the import origin.
    AttributeOrigin,
    /// Identify the import license.
    IdentifyLicense,
    /// Pin the upstream version.
    PinUpstreamVersion,
    /// Assign an update owner.
    AssignUpdateOwner,
    /// Refresh the divergence review.
    RefreshDivergenceReview,
    /// Record the sponsor/fork/replace decision.
    RecordSponsorForkReplaceDecision,
    /// Record the generator identity.
    RecordGeneratorIdentity,
    /// Record the regeneration path.
    RecordRegenerationPath,
    /// Refresh the import proof packet.
    RefreshImportProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl ImportAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::HoldPromotion,
        Self::AttributeOrigin,
        Self::IdentifyLicense,
        Self::PinUpstreamVersion,
        Self::AssignUpdateOwner,
        Self::RefreshDivergenceReview,
        Self::RecordSponsorForkReplaceDecision,
        Self::RecordGeneratorIdentity,
        Self::RecordRegenerationPath,
        Self::RefreshImportProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::AttributeOrigin => "attribute_origin",
            Self::IdentifyLicense => "identify_license",
            Self::PinUpstreamVersion => "pin_upstream_version",
            Self::AssignUpdateOwner => "assign_update_owner",
            Self::RefreshDivergenceReview => "refresh_divergence_review",
            Self::RecordSponsorForkReplaceDecision => "record_sponsor_fork_replace_decision",
            Self::RecordGeneratorIdentity => "record_generator_identity",
            Self::RecordRegenerationPath => "record_regeneration_path",
            Self::RefreshImportProof => "refresh_import_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No import-layer stop rule fires; promotion may proceed.
    Proceed,
    /// An import-layer stop rule fires; hold promotion.
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

/// Import provenance: origin attribution, license identification, and upstream pin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportProvenance {
    /// Origin attribution state.
    pub origin_state: OriginState,
    /// License identification state.
    pub license_state: LicenseState,
    /// Upstream pin state.
    pub upstream_pin_state: UpstreamPinState,
    /// SPDX license id (empty unless the license is identified).
    pub spdx_license_id: String,
    /// Upstream version (empty unless pinned).
    pub upstream_version: String,
    /// Reference to the upstream origin.
    pub origin_ref: String,
    /// Reference to the license record.
    pub license_ref: String,
}

impl ImportProvenance {
    /// True when the origin is not attributed.
    pub fn origin_unattributed(&self) -> bool {
        self.origin_state == OriginState::Unattributed
    }

    /// True when the license is not identified.
    pub fn license_unidentified(&self) -> bool {
        self.license_state == LicenseState::Unidentified
    }

    /// True when the upstream version is floating rather than pinned.
    pub fn upstream_floating(&self) -> bool {
        self.upstream_pin_state == UpstreamPinState::Floating
    }
}

/// Update ownership for an import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateOwnership {
    /// Ownership state.
    pub ownership_state: OwnershipState,
    /// Owning team or role (empty when ownerless).
    pub update_owner_ref: String,
    /// Reference to the last recorded update.
    pub last_update_ref: String,
}

impl UpdateOwnership {
    /// True when the import has no assigned update owner.
    pub fn owner_missing(&self) -> bool {
        self.ownership_state == OwnershipState::Ownerless
    }
}

/// The divergence profile of an import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DivergenceProfile {
    /// Local-modification posture.
    pub divergence_state: DivergenceState,
    /// Number of local patches applied over upstream.
    pub local_patch_count: u32,
    /// Days the import has diverged from upstream.
    pub divergence_age_days: u32,
    /// Divergence-review state.
    pub review_state: DivergenceReviewState,
    /// Reference to the divergence review.
    pub review_ref: String,
}

/// The sponsor/fork/replace decision for an import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionRecord {
    /// Decision state.
    pub decision_state: DecisionState,
    /// Settled disposition.
    pub disposition: DecisionDisposition,
    /// Reference to the decision record.
    pub decision_ref: String,
    /// Reference to the review board that recorded it.
    pub review_board_ref: String,
}

/// Generated-code provenance for an import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratorProvenance {
    /// True when this import is checked-in generated code.
    pub applies: bool,
    /// True when the generator identity is recorded.
    pub generator_identity_present: bool,
    /// True when the regeneration path is recorded.
    pub regeneration_path_present: bool,
    /// Reference to the generator identity (empty when generation does not apply).
    pub generator_ref: String,
    /// Reference to the regeneration path (empty when generation does not apply).
    pub regeneration_path_ref: String,
}

impl GeneratorProvenance {
    /// True when generated code does not record its generator identity.
    pub fn identity_missing(&self) -> bool {
        self.applies && !self.generator_identity_present
    }

    /// True when generated code does not record its regeneration path.
    pub fn regeneration_path_missing(&self) -> bool {
        self.applies && !self.regeneration_path_present
    }
}

/// One import-governance control binding on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportControl {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// One third-party import / generated-code / fork record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportRecord {
    /// Stable record id.
    pub record_id: String,
    /// The M5 family this import serves.
    pub family: M5Family,
    /// The kind of import.
    pub import_kind: ImportKind,
    /// Human-readable title.
    pub title: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this import is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this record declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the subject.
    pub support_class: SupportClass,
    /// Import provenance.
    pub provenance: ImportProvenance,
    /// Update ownership.
    pub ownership: UpdateOwnership,
    /// Divergence profile.
    pub divergence: DivergenceProfile,
    /// Sponsor/fork/replace decision.
    pub decision: DecisionRecord,
    /// Generated-code provenance.
    pub generator: GeneratorProvenance,
    /// Per-dimension control bindings.
    pub controls: Vec<ImportControl>,
    /// What the dependency-health/import scan found.
    pub manifest_scan_posture: Posture,
    /// What the user/admin import surface shows.
    pub surface_posture: Posture,
    /// Reference to the import scan.
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
    pub import_state: ImportState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ImportReason>,
    /// The label the record effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this record (Help/About, release-center, support/procurement).
    pub surfaces: Vec<String>,
    /// Reviewable reason the record carries its state.
    pub rationale: String,
}

impl ImportRecord {
    /// True when the record is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(ImportReason::WaiverExpired)
    }

    /// True when the record carries the given active reason.
    pub fn has_active_reason(&self, reason: ImportReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the record holds a cleared state.
    pub fn is_cleared(&self) -> bool {
        self.import_state == ImportState::Cleared
    }

    /// True when the subject declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// True when this import requires a divergence review.
    pub fn requires_divergence_review(&self) -> bool {
        self.divergence.divergence_state.requires_review()
    }

    /// True when this import requires a sponsor/fork/replace decision.
    pub fn requires_decision(&self) -> bool {
        self.import_kind.requires_decision()
    }

    /// True when a required divergence review has gone stale.
    pub fn review_stale(&self) -> bool {
        self.requires_divergence_review()
            && self.divergence.review_state == DivergenceReviewState::Stale
    }

    /// True when a required divergence review is missing.
    pub fn review_missing(&self) -> bool {
        self.requires_divergence_review()
            && self.divergence.review_state == DivergenceReviewState::Missing
    }

    /// True when a required sponsor/fork/replace decision is still pending.
    pub fn decision_missing(&self) -> bool {
        self.requires_decision() && self.decision.decision_state == DecisionState::Pending
    }

    /// True when any import gap (other than proof/sign-off) is present.
    pub fn has_import_gap(&self) -> bool {
        self.provenance.origin_unattributed()
            || self.provenance.license_unidentified()
            || self.provenance.upstream_floating()
            || self.ownership.owner_missing()
            || self.review_stale()
            || self.review_missing()
            || self.decision_missing()
            || self.generator.identity_missing()
            || self.generator.regeneration_path_missing()
    }

    /// The expected control state for a dimension, derived from the subject's facts.
    pub fn expected_control_state(&self, dimension: ControlDimension) -> ControlState {
        match dimension {
            ControlDimension::ImportProvenance => {
                if self.provenance.origin_unattributed()
                    || self.provenance.license_unidentified()
                    || self.provenance.upstream_floating()
                {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::UpdateOwnership => {
                if self.ownership.owner_missing() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::DivergenceReview => {
                if !self.requires_divergence_review() {
                    ControlState::NotApplicable
                } else if self.review_stale() || self.review_missing() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::DecisionPath => {
                if !self.requires_decision() {
                    ControlState::NotApplicable
                } else if self.decision_missing() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::GeneratorProvenance => {
                if !self.generator.applies {
                    ControlState::NotApplicable
                } else if self.generator.identity_missing()
                    || self.generator.regeneration_path_missing()
                {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::ManifestSurfaceParity => {
                if self.manifest_scan_posture != self.surface_posture {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
        }
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> ImportState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return ImportState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => ImportState::Cleared,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            ImportState::Cleared => self.declared_label,
            ImportState::Withdrawn => LifecycleLabel::Withdrawn,
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
    pub fn computed_posture(&self) -> Posture {
        if self.import_state.is_narrowed() {
            Posture::GapsFound
        } else {
            Posture::Clear
        }
    }

    /// True when the record may hold promotion: a release-blocking subject, narrowed by an
    /// import-layer gap, declaring a label at or above the cutline, and not held by an
    /// unexpired waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.import_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }

    /// True when the scan and the surface agree.
    pub fn scan_surface_agree(&self) -> bool {
        self.manifest_scan_posture == self.surface_posture
    }
}

/// A closed stop-rule that gates promotion on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: ImportReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: ImportAction,
    /// True when the rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportCutline {
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
    /// Third-party import register.
    pub third_party_import_register_ref: String,
    /// Release-facing import manifest.
    pub import_manifest_ref: String,
    /// Dependency register.
    pub dependency_register_ref: String,
    /// Critical-upstream health scorecard.
    pub critical_upstream_scorecard_ref: String,
    /// Generated-artifact lineage contract.
    pub generated_lineage_ref: String,
    /// Package inventory (protected-path posture).
    pub package_inventory_ref: String,
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

/// Cross-cutting manifest/surface parity summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestSurfaceParity {
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
pub struct ImportSummary {
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
    /// Records in the `narrowed_ownership` state.
    pub state_narrowed_ownership: usize,
    /// Records in the `narrowed_divergence` state.
    pub state_narrowed_divergence: usize,
    /// Records in the `narrowed_generator` state.
    pub state_narrowed_generator: usize,
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
    /// Records carrying an origin/license/upstream provenance gap.
    pub provenance_gaps: usize,
    /// Records that are ownerless.
    pub ownership_gaps: usize,
    /// Records carrying a divergence-review or decision gap.
    pub divergence_gaps: usize,
    /// Records carrying a generator identity/regeneration-path gap.
    pub generator_gaps: usize,
    /// Records of kind `third_party_import`.
    pub third_party_imports: usize,
    /// Records of kind `generated_artifact`.
    pub generated_artifacts: usize,
    /// Records that are long-lived forks or single-source imports.
    pub long_lived_imports: usize,
    /// Records with a recorded sponsor/fork/replace decision.
    pub decisions_recorded: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of import-provenance and fork-review records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportRegister {
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
    pub import_cutline: ImportCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed import-kind vocabulary.
    pub import_kinds: Vec<ImportKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed origin-state vocabulary.
    pub origin_states: Vec<OriginState>,
    /// Closed license-state vocabulary.
    pub license_states: Vec<LicenseState>,
    /// Closed upstream-pin-state vocabulary.
    pub upstream_pin_states: Vec<UpstreamPinState>,
    /// Closed ownership-state vocabulary.
    pub ownership_states: Vec<OwnershipState>,
    /// Closed divergence-state vocabulary.
    pub divergence_states: Vec<DivergenceState>,
    /// Closed divergence-review-state vocabulary.
    pub divergence_review_states: Vec<DivergenceReviewState>,
    /// Closed decision-state vocabulary.
    pub decision_states: Vec<DecisionState>,
    /// Closed decision-disposition vocabulary.
    pub decision_dispositions: Vec<DecisionDisposition>,
    /// Closed posture vocabulary.
    pub postures: Vec<Posture>,
    /// Closed import-state vocabulary.
    pub import_states: Vec<ImportState>,
    /// Closed import-reason vocabulary.
    pub import_reasons: Vec<ImportReason>,
    /// Closed import-action vocabulary.
    pub import_actions: Vec<ImportAction>,
    /// Stop rules.
    pub rules: Vec<ImportRule>,
    /// Per-import records.
    pub records: Vec<ImportRecord>,
    /// Cross-cutting manifest/surface parity summary.
    pub manifest_surface_parity: ManifestSurfaceParity,
    /// Promotion verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: ImportSummary,
}

impl ImportRegister {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&ImportRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Returns the cleared records.
    pub fn records_cleared(&self) -> Vec<&ImportRecord> {
        self.records.iter().filter(|r| r.is_cleared()).collect()
    }

    /// Returns the narrowed records.
    pub fn records_narrowed(&self) -> Vec<&ImportRecord> {
        self.records
            .iter()
            .filter(|r| r.import_state.is_narrowed())
            .collect()
    }

    /// Returns the records of a given import kind.
    pub fn records_of_kind(&self, kind: ImportKind) -> Vec<&ImportRecord> {
        self.records
            .iter()
            .filter(|r| r.import_kind == kind)
            .collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: ImportReason) -> Option<&ImportRule> {
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

    /// Recomputes the cross-cutting manifest/surface parity summary.
    pub fn computed_manifest_surface_parity(&self) -> ManifestSurfaceParity {
        ManifestSurfaceParity {
            parity_gate: self.manifest_surface_parity.parity_gate.clone(),
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
                .filter(|r| r.surface_posture == Posture::GapsFound)
                .count(),
            all_subjects_agree: self.records.iter().all(|r| r.scan_surface_agree()),
            rationale: self.manifest_surface_parity.rationale.clone(),
        }
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> ImportSummary {
        let count_state = |state: ImportState| {
            self.records
                .iter()
                .filter(|r| r.import_state == state)
                .count()
        };
        ImportSummary {
            total_records: self.records.len(),
            records_cleared: self.records_cleared().len(),
            records_narrowed: self.records_narrowed().len(),
            state_cleared: count_state(ImportState::Cleared),
            state_narrowed_provenance: count_state(ImportState::NarrowedProvenance),
            state_narrowed_ownership: count_state(ImportState::NarrowedOwnership),
            state_narrowed_divergence: count_state(ImportState::NarrowedDivergence),
            state_narrowed_generator: count_state(ImportState::NarrowedGenerator),
            state_narrowed_stale: count_state(ImportState::NarrowedStale),
            state_withdrawn: count_state(ImportState::Withdrawn),
            release_blocking_total: self.records.iter().filter(|r| r.release_blocking).count(),
            release_blocking_narrowed: self
                .records
                .iter()
                .filter(|r| r.release_blocking && r.import_state.is_narrowed())
                .count(),
            records_on_active_waiver: self.records.iter().filter(|r| r.is_waived()).count(),
            provenance_gaps: self
                .records
                .iter()
                .filter(|r| {
                    r.provenance.origin_unattributed()
                        || r.provenance.license_unidentified()
                        || r.provenance.upstream_floating()
                })
                .count(),
            ownership_gaps: self
                .records
                .iter()
                .filter(|r| r.ownership.owner_missing())
                .count(),
            divergence_gaps: self
                .records
                .iter()
                .filter(|r| r.review_stale() || r.review_missing() || r.decision_missing())
                .count(),
            generator_gaps: self
                .records
                .iter()
                .filter(|r| {
                    r.generator.identity_missing() || r.generator.regeneration_path_missing()
                })
                .count(),
            third_party_imports: self
                .records
                .iter()
                .filter(|r| r.import_kind == ImportKind::ThirdPartyImport)
                .count(),
            generated_artifacts: self
                .records
                .iter()
                .filter(|r| r.import_kind == ImportKind::GeneratedArtifact)
                .count(),
            long_lived_imports: self
                .records
                .iter()
                .filter(|r| r.requires_decision())
                .count(),
            decisions_recorded: self
                .records
                .iter()
                .filter(|r| r.decision.decision_state == DecisionState::Recorded)
                .count(),
            total_active_reasons: self.records.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, release-center publication, support
    /// exports, and procurement packets. It carries only the family, kind, declared and
    /// effective labels, state, the per-axis posture, the divergence/decision summary, active
    /// reasons, and surfaces — never the detailed scan, review, and proof internals.
    pub fn reuse_projection(&self) -> Vec<ImportReuseRow> {
        self.records
            .iter()
            .map(|r| ImportReuseRow {
                record_id: r.record_id.clone(),
                family: r.family,
                import_kind: r.import_kind,
                declared_label: r.declared_label,
                effective_label: r.effective_label,
                support_class: r.support_class,
                import_state: r.import_state,
                release_blocking: r.release_blocking,
                scan_surface_agree: r.scan_surface_agree(),
                divergence_state: r.divergence.divergence_state,
                decision_disposition: r.decision.disposition,
                active_reasons: r.active_reasons.clone(),
                surfaces: r.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.records.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every import kind must be exercised by at least one record.
        for kind in ImportKind::ALL {
            if !self.records.iter().any(|r| r.import_kind == kind) {
                v.push(RegisterViolation::ImportKindUncovered { kind });
            }
        }

        // Every reason must have a stop rule.
        for reason in ImportReason::ALL {
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
        if self.manifest_surface_parity != self.computed_manifest_surface_parity() {
            v.push(RegisterViolation::ManifestSurfaceParityMismatch);
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
        if self.import_kinds != ImportKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "import_kinds",
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
        if self.origin_states != OriginState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "origin_states",
            });
        }
        if self.license_states != LicenseState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "license_states",
            });
        }
        if self.upstream_pin_states != UpstreamPinState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "upstream_pin_states",
            });
        }
        if self.ownership_states != OwnershipState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "ownership_states",
            });
        }
        if self.divergence_states != DivergenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "divergence_states",
            });
        }
        if self.divergence_review_states != DivergenceReviewState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "divergence_review_states",
            });
        }
        if self.decision_states != DecisionState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "decision_states",
            });
        }
        if self.decision_dispositions != DecisionDisposition::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "decision_dispositions",
            });
        }
        if self.postures != Posture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "postures" });
        }
        if self.import_states != ImportState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "import_states",
            });
        }
        if self.import_reasons != ImportReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "import_reasons",
            });
        }
        if self.import_actions != ImportAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "import_actions",
            });
        }
        if self.import_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "import_cutline",
            });
        }
    }

    fn validate_record(
        &self,
        r: &ImportRecord,
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

        self.validate_fact_consistency(r, v);
        self.validate_controls(r, v);
        self.validate_reason_evidence(r, v);
        self.validate_scan_surface(r, v);
        self.validate_state_and_label(r, v);
    }

    /// Each fact block must be internally consistent — so a state token can never sit over a
    /// contradicting fact (an "identified" license with no SPDX id, an "owned" import with no
    /// owner, a "recorded" decision with no disposition, generated provenance on a non-generated
    /// import).
    fn validate_fact_consistency(&self, r: &ImportRecord, v: &mut Vec<RegisterViolation>) {
        let p = &r.provenance;
        // license identified ⟺ spdx id present.
        let spdx_present = !p.spdx_license_id.trim().is_empty();
        if (p.license_state == LicenseState::Identified) != spdx_present {
            v.push(RegisterViolation::LicenseFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // upstream pinned ⟺ version present.
        let version_present = !p.upstream_version.trim().is_empty();
        if (p.upstream_pin_state == UpstreamPinState::Pinned) != version_present {
            v.push(RegisterViolation::UpstreamFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // owned ⟺ owner present.
        let owned = r.ownership.ownership_state == OwnershipState::Owned;
        let owner_present = !r.ownership.update_owner_ref.trim().is_empty();
        if owned != owner_present {
            v.push(RegisterViolation::OwnershipFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // a divergence review applies iff the posture requires one.
        let review_applies = r.divergence.review_state != DivergenceReviewState::NotRequired;
        if review_applies != r.requires_divergence_review() {
            v.push(
                RegisterViolation::DivergenceReviewApplicabilityInconsistent {
                    record_id: r.record_id.clone(),
                },
            );
        }
        // in-sync ⟺ no local patches.
        let in_sync = r.divergence.divergence_state == DivergenceState::InSync;
        if in_sync != (r.divergence.local_patch_count == 0) {
            v.push(RegisterViolation::DivergencePatchCountInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // a decision applies iff the kind requires one.
        let decision_applies = r.decision.decision_state != DecisionState::NotRequired;
        if decision_applies != r.requires_decision() {
            v.push(RegisterViolation::DecisionApplicabilityInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // a settled disposition ⟺ a recorded decision.
        let recorded = r.decision.decision_state == DecisionState::Recorded;
        if recorded != r.decision.disposition.is_settled() {
            v.push(RegisterViolation::DecisionDispositionInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // generator provenance applies iff the import is generated code.
        if r.generator.applies != r.import_kind.is_generated() {
            v.push(RegisterViolation::GeneratorApplicabilityInconsistent {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_controls(&self, r: &ImportRecord, v: &mut Vec<RegisterViolation>) {
        // Every control dimension must be declared exactly once, and its declared state
        // must equal the state its facts imply — so a control can never assert "satisfied"
        // over a gap.
        for dimension in ControlDimension::ALL {
            let matches: Vec<&ImportControl> = r
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

    /// Every active reason must be justified by the record's own facts, and every structural
    /// gap must surface its reason.
    fn validate_reason_evidence(&self, r: &ImportRecord, v: &mut Vec<RegisterViolation>) {
        let origin_unattributed = r.provenance.origin_unattributed();
        let license_unidentified = r.provenance.license_unidentified();
        let upstream_floating = r.provenance.upstream_floating();
        let owner_missing = r.ownership.owner_missing();
        let review_stale = r.review_stale();
        let review_missing = r.review_missing();
        let decision_missing = r.decision_missing();
        let identity_missing = r.generator.identity_missing();
        let regen_missing = r.generator.regeneration_path_missing();
        let proof_stale = r.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = r.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !r.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &r.active_reasons {
            let justified = match reason {
                ImportReason::OriginUnattributed => origin_unattributed,
                ImportReason::LicenseUnidentified => license_unidentified,
                ImportReason::UpstreamVersionFloating => upstream_floating,
                ImportReason::UpdateOwnerMissing => owner_missing,
                ImportReason::DivergenceReviewStale => review_stale,
                ImportReason::DivergenceReviewMissing => review_missing,
                ImportReason::DecisionRecordMissing => decision_missing,
                ImportReason::GeneratorIdentityMissing => identity_missing,
                ImportReason::RegenerationPathMissing => regen_missing,
                ImportReason::ImportProofStale => proof_stale,
                ImportReason::ImportProofMissing => proof_missing,
                ImportReason::OwnerSignoffMissing => signoff_missing,
                ImportReason::WaiverExpired => r.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    record_id: r.record_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: ImportReason, v: &mut Vec<RegisterViolation>| {
            if present && !r.has_active_reason(reason) {
                v.push(RegisterViolation::GapWithoutReason {
                    record_id: r.record_id.clone(),
                    reason,
                });
            }
        };
        require(origin_unattributed, ImportReason::OriginUnattributed, v);
        require(license_unidentified, ImportReason::LicenseUnidentified, v);
        require(upstream_floating, ImportReason::UpstreamVersionFloating, v);
        require(owner_missing, ImportReason::UpdateOwnerMissing, v);
        require(review_stale, ImportReason::DivergenceReviewStale, v);
        require(review_missing, ImportReason::DivergenceReviewMissing, v);
        require(decision_missing, ImportReason::DecisionRecordMissing, v);
        require(identity_missing, ImportReason::GeneratorIdentityMissing, v);
        require(regen_missing, ImportReason::RegenerationPathMissing, v);
        require(proof_stale, ImportReason::ImportProofStale, v);
        require(proof_missing, ImportReason::ImportProofMissing, v);
        require(signoff_missing, ImportReason::OwnerSignoffMissing, v);
    }

    /// The scan and the surface must agree, and the posture must reflect the gaps — a clean
    /// surface may never sit over a scan that found an ownerless, unattributed, or
    /// generator-free import.
    fn validate_scan_surface(&self, r: &ImportRecord, v: &mut Vec<RegisterViolation>) {
        if r.manifest_scan_posture != r.surface_posture {
            v.push(RegisterViolation::ManifestScanSurfaceDisagreement {
                record_id: r.record_id.clone(),
            });
        }
        let computed = r.computed_posture();
        if r.surface_posture != computed || r.manifest_scan_posture != computed {
            v.push(RegisterViolation::PostureMismatch {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_state_and_label(&self, r: &ImportRecord, v: &mut Vec<RegisterViolation>) {
        // cleared ⇒ no reasons; narrowed ⇒ at least one reason.
        if r.is_cleared() && !r.active_reasons.is_empty() {
            v.push(RegisterViolation::ClearedWithActiveReason {
                record_id: r.record_id.clone(),
            });
        }
        if r.import_state.is_narrowed() && r.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                record_id: r.record_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if r.import_state != r.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                record_id: r.record_id.clone(),
                declared: r.import_state,
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
        if r.import_state.is_narrowed() && r.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                record_id: r.record_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReuseRow {
    /// Record id.
    pub record_id: String,
    /// Family.
    pub family: M5Family,
    /// Import kind.
    pub import_kind: ImportKind,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Import state.
    pub import_state: ImportState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// True when the scan and the surface agree.
    pub scan_surface_agree: bool,
    /// Divergence posture.
    pub divergence_state: DivergenceState,
    /// Recorded decision disposition.
    pub decision_disposition: DecisionDisposition,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ImportReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the import-provenance and fork-review register.
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
    /// An import kind has no record.
    ImportKindUncovered {
        /// Uncovered kind.
        kind: ImportKind,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: ImportReason,
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
    /// A record's license state disagrees with its SPDX id.
    LicenseFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's upstream pin state disagrees with its version.
    UpstreamFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's ownership state disagrees with its owner ref.
    OwnershipFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's divergence-review applicability disagrees with its posture.
    DivergenceReviewApplicabilityInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's local-patch count disagrees with its divergence posture.
    DivergencePatchCountInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's decision applicability disagrees with its import kind.
    DecisionApplicabilityInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's decision disposition disagrees with its decision state.
    DecisionDispositionInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's generator applicability disagrees with its import kind.
    GeneratorApplicabilityInconsistent {
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
        reason: ImportReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Record id.
        record_id: String,
        /// Missing reason.
        reason: ImportReason,
    },
    /// A record's scan and surface postures disagree.
    ManifestScanSurfaceDisagreement {
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
        declared: ImportState,
        /// Computed state.
        computed: ImportState,
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
    /// The recorded manifest/surface parity disagrees with the computed summary.
    ManifestSurfaceParityMismatch,
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
            Self::ImportKindUncovered { kind } => {
                write!(f, "import kind {} has no record", kind.as_str())
            }
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
            Self::LicenseFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} license state disagrees with its SPDX id"
                )
            }
            Self::UpstreamFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} upstream pin state disagrees with its version"
                )
            }
            Self::OwnershipFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} ownership state disagrees with its owner ref"
                )
            }
            Self::DivergenceReviewApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} divergence-review applicability disagrees with its posture"
            ),
            Self::DivergencePatchCountInconsistent { record_id } => write!(
                f,
                "record {record_id} local-patch count disagrees with its divergence posture"
            ),
            Self::DecisionApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} decision applicability disagrees with its import kind"
            ),
            Self::DecisionDispositionInconsistent { record_id } => write!(
                f,
                "record {record_id} decision disposition disagrees with its decision state"
            ),
            Self::GeneratorApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} generator applicability disagrees with its import kind"
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
            Self::ManifestScanSurfaceDisagreement { record_id } => {
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
            Self::ManifestSurfaceParityMismatch => {
                write!(
                    f,
                    "manifest_surface_parity disagrees with the computed summary"
                )
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the records"),
        }
    }
}

impl Error for RegisterViolation {}

/// Loads the embedded import-provenance and fork-review register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`ImportRegister`] — including when a record carries a token outside any closed
/// vocabulary.
pub fn current_m5_import_provenance_and_fork_review() -> Result<ImportRegister, serde_json::Error> {
    serde_json::from_str(M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
