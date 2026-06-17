//! Canonical M5 workspace-serialization qualification packet: the single qualification layer that
//! turns serialization and restore fidelity into named qualification rows — for remembered-state
//! classes, restore fidelity, portable-state review, migration/remap handling, and missing-surface
//! continuity — across every claimed profile and deployment mode, and automatically narrows the
//! marketed restore claim before publication wherever the evidence is stale, missing, or
//! downgraded.
//!
//! This packet is the certification layer above the
//! [`crate::m5_serialization_and_restore_matrix`] matrix. It does not re-derive each surface's
//! restore truth — it reuses the matrix's restore-fidelity vocabulary
//! ([`RestoreFidelityClass`]), ingests the matrix's published fidelity for the row's
//! artifact-class/surface ([`QualificationRow::matrix_claim`]), runs the per-row qualification
//! drills ([`QualificationDrill`]) — schema jump, foreign package, display topology, missing
//! extension, placeholder continuity, accessibility, and downgrade — scores how fresh the
//! qualification evidence is ([`EvidenceFreshness`]), and publishes the restore-fidelity claim
//! ([`RestoreFidelityClass`]) no input can exceed.
//!
//! The qualification gate is non-inheriting and fail-closed. The published fidelity is the weakest
//! ceiling implied by the row's declared maximum, the matrix claim, the evidence freshness, and the
//! drill outcomes ([`QualificationRow::effective_fidelity`]), so a matrix-narrowed surface, stale or
//! missing evidence, or an unproven, narrowed, or failed drill all narrow or withhold the row
//! automatically rather than leaving a profile green by inertia. A row that declared a stronger
//! claim than the gate permits has its published fidelity lowered, its
//! [`QualificationDowngradeReason`]s and [`QualificationDowngradePath`] recomputed, and its
//! [`ClaimPublication`] decision recomputed; all are validated against the gate so a downgrade can
//! never be asserted or hidden by hand. This is the guardrail the spec demands: no blanket "restore
//! supported" claim survives without a per-profile qualification row, freshness, and a downgrade
//! path, and no profile is marked green because a nearby profile passed a superficially similar
//! restore flow.
//!
//! Because every required consumer surface — docs/help, support export, companion/browser handoff,
//! release center, and shiproom — binds to this one packet via a
//! [`QualificationConsumerBinding`] that must ingest the packet, preserve its published fidelity and
//! recovery paths, and narrow with it, a row narrowed here cannot stay authoritative on a docs
//! badge, a support packet, a companion handoff card, a release-center claim, or a shiproom row.
//! Each binding is stamped with the active scope snapshot so support and evidence packets can
//! reconstruct the scope the qualification answered.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-serialization-qualification.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, live authority handles, or workspace
//! contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_serialization_and_restore_matrix::RestoreFidelityClass;

/// Supported M5 serialization-qualification packet schema version.
pub const M5_SERIALIZATION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SERIALIZATION_QUALIFICATION_RECORD_KIND: &str = "m5_serialization_qualification";

/// Repo-relative path to the checked-in packet.
pub const M5_SERIALIZATION_QUALIFICATION_PATH: &str =
    "artifacts/workspace/m5/m5-serialization-qualification.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SERIALIZATION_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/workspace/m5-serialization-qualification.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SERIALIZATION_QUALIFICATION_DOC_REF: &str =
    "docs/workspace/m5/m5-serialization-qualification.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_SERIALIZATION_QUALIFICATION_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-serialization-qualification.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SERIALIZATION_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-serialization-qualification";

/// Repo-relative path to the upstream serialization-and-restore matrix this packet qualifies.
pub const M5_SERIALIZATION_QUALIFICATION_MATRIX_PACKET_REF: &str =
    "artifacts/workspace/m5/m5-serialization-and-restore-matrix.json";

/// Repo-relative path to the shiproom claim packet that renders this qualification.
pub const M5_SERIALIZATION_QUALIFICATION_CLAIM_PACKET_REF: &str =
    "artifacts/shiproom/m5-serialization-claim-packet/m5_serialization_claim_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_SERIALIZATION_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-serialization-qualification.json"
));

/// One of the five serialization families this packet qualifies.
///
/// Each family names a distinct slice of remembered-state truth. The packet keeps them separate so
/// that remembered-state inspection, restore fidelity, portable-state review, migration/remap
/// handling, and missing-surface continuity are never qualified as one another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationFamily {
    /// Remembered-state class inspection: which artifact classes are remembered, last-write, and
    /// schema version.
    RememberedState,
    /// Restore fidelity: how faithfully a desktop restore, crash recovery, or resume rehydrates.
    RestoreFidelity,
    /// Portable-state review: export/import comparison, redaction, and machine-local exclusions.
    PortableStateReview,
    /// Schema-migration and display-topology remap handling on restore and import.
    MigrationRemap,
    /// Missing-surface continuity: slot-preserving placeholder cards for absent dependencies.
    MissingSurfaceContinuity,
}

impl QualificationFamily {
    /// Every serialization family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RememberedState,
        Self::RestoreFidelity,
        Self::PortableStateReview,
        Self::MigrationRemap,
        Self::MissingSurfaceContinuity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RememberedState => "remembered_state",
            Self::RestoreFidelity => "restore_fidelity",
            Self::PortableStateReview => "portable_state_review",
            Self::MigrationRemap => "migration_remap",
            Self::MissingSurfaceContinuity => "missing_surface_continuity",
        }
    }
}

/// The deployment mode a qualification row is proven on.
///
/// Qualification never crosses deployment modes: a row proven on desktop says nothing about a
/// managed-fleet or companion/browser restore of the same family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentMode {
    /// A local desktop install.
    Desktop,
    /// A managed/enterprise fleet deployment.
    ManagedFleet,
    /// A companion or browser re-entry handoff.
    CompanionBrowser,
}

impl DeploymentMode {
    /// Every deployment mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::Desktop, Self::ManagedFleet, Self::CompanionBrowser];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::ManagedFleet => "managed_fleet",
            Self::CompanionBrowser => "companion_browser",
        }
    }
}

/// A qualification drill the suite runs for every claimed serialization row.
///
/// A row is never published above [`RestoreFidelityClass::ManualReview`] unless every required
/// drill ran and passed cleanly; an unproven, narrowed, or failed drill narrows or withholds the
/// published fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDrill {
    /// A schema-version jump / forward-migration drill.
    SchemaJump,
    /// A foreign- / imported-package provenance drill.
    ForeignPackage,
    /// A display-topology remap drill.
    DisplayTopology,
    /// A missing-extension placeholder drill.
    MissingExtension,
    /// A slot-preserving placeholder-continuity drill.
    PlaceholderContinuity,
    /// A keyboard, list/table, and screen-reader accessibility drill.
    Accessibility,
    /// An automatic claim-narrowing and recovery downgrade drill.
    Downgrade,
}

impl QualificationDrill {
    /// Every required drill, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SchemaJump,
        Self::ForeignPackage,
        Self::DisplayTopology,
        Self::MissingExtension,
        Self::PlaceholderContinuity,
        Self::Accessibility,
        Self::Downgrade,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaJump => "schema_jump",
            Self::ForeignPackage => "foreign_package",
            Self::DisplayTopology => "display_topology",
            Self::MissingExtension => "missing_extension",
            Self::PlaceholderContinuity => "placeholder_continuity",
            Self::Accessibility => "accessibility",
            Self::Downgrade => "downgrade",
        }
    }
}

/// The outcome of one qualification drill.
///
/// Ordered by [`DrillOutcome::fidelity_ceiling`]: a passed drill backs exact restore, a narrowed
/// drill caps at compatible restore, and a failed or not-run drill caps at manual review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillOutcome {
    /// The drill ran and passed cleanly.
    Passed,
    /// The drill ran but only proved the row for a narrower slice.
    Narrowed,
    /// The drill ran and failed.
    Failed,
    /// The drill did not run; the row is unproven.
    NotRun,
}

impl DrillOutcome {
    /// Every drill outcome, in declaration order.
    pub const ALL: [Self; 4] = [Self::Passed, Self::Narrowed, Self::Failed, Self::NotRun];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Narrowed => "narrowed",
            Self::Failed => "failed",
            Self::NotRun => "not_run",
        }
    }

    /// Highest restore fidelity this outcome permits a row to publish.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::Passed => RestoreFidelityClass::ExactRestore,
            Self::Narrowed => RestoreFidelityClass::CompatibleRestore,
            Self::Failed | Self::NotRun => RestoreFidelityClass::ManualReview,
        }
    }

    /// Whether the outcome narrows the row to a slice.
    pub const fn is_narrowed(self) -> bool {
        matches!(self, Self::Narrowed)
    }

    /// Whether the outcome leaves the row unproven (failed or never run).
    pub const fn is_unproven(self) -> bool {
        matches!(self, Self::Failed | Self::NotRun)
    }

    /// Whether the drill ran at all, so it must carry an evidence ref.
    pub const fn was_run(self) -> bool {
        !matches!(self, Self::NotRun)
    }
}

/// How fresh the qualification evidence backing a row is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The qualification evidence is current.
    Current,
    /// The qualification evidence is aging but in tolerance; caps at compatible restore.
    Aging,
    /// The qualification evidence is expired; caps at layout-only.
    Expired,
    /// The qualification evidence is missing; caps at manual review.
    Missing,
}

impl EvidenceFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Aging, Self::Expired, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// Highest restore fidelity this freshness state permits a row to publish.
    ///
    /// The mapping is identical to the serialization matrix's own freshness condition, so a stale
    /// qualification narrows in lockstep with the matrix.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::Current => RestoreFidelityClass::ExactRestore,
            Self::Aging => RestoreFidelityClass::CompatibleRestore,
            Self::Expired => RestoreFidelityClass::LayoutOnly,
            Self::Missing => RestoreFidelityClass::ManualReview,
        }
    }

    /// Whether this state raises the [`QualificationDowngradeReason::EvidenceStale`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// The marketed-claim decision the gate publishes for a row.
///
/// This is the auto-narrowing the spec demands: a row publishes its full claim only where it
/// qualifies its declared fidelity with fresh evidence and clean drills; otherwise the claim is
/// narrowed to the qualified fidelity or withheld entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPublication {
    /// The full restore claim is published; the row qualified its declared fidelity cleanly.
    Published,
    /// The claim is auto-narrowed to the qualified fidelity below what the row declared.
    Narrowed,
    /// No restore claim is published; the row qualifies only for manual review.
    Withheld,
}

impl ClaimPublication {
    /// Every claim-publication decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::Published, Self::Narrowed, Self::Withheld];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the gate narrowed or withheld the row's marketed claim.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Published)
    }
}

/// A headline reason the qualification gate narrows a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDowngradeReason {
    /// The upstream serialization matrix already narrowed the surface below exact restore.
    MatrixNarrowed,
    /// The qualification evidence is aging, expired, or missing.
    EvidenceStale,
    /// At least one qualification drill proved the row only for a narrower slice.
    DrillNarrowed,
    /// At least one qualification drill failed or never ran.
    DrillFailed,
}

impl QualificationDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MatrixNarrowed,
        Self::EvidenceStale,
        Self::DrillNarrowed,
        Self::DrillFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatrixNarrowed => "matrix_narrowed",
            Self::EvidenceStale => "evidence_stale",
            Self::DrillNarrowed => "drill_narrowed",
            Self::DrillFailed => "drill_failed",
        }
    }
}

/// The exact recovery path surfaced when a row's published claim is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDowngradePath {
    /// Rerun the failed, not-run, or narrowed qualification drills.
    RerunDrills,
    /// Refresh the aging, expired, or missing qualification evidence.
    RefreshEvidence,
    /// Adopt the serialization matrix's narrowing rather than re-asserting a broader claim.
    AdoptMatrixNarrowing,
    /// Withhold the restore claim from publication.
    WithholdClaim,
    /// No downgrade is needed; only valid when the row publishes a clean full claim.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl QualificationDowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RerunDrills,
        Self::RefreshEvidence,
        Self::AdoptMatrixNarrowing,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunDrills => "rerun_drills",
            Self::RefreshEvidence => "refresh_evidence",
            Self::AdoptMatrixNarrowing => "adopt_matrix_narrowing",
            Self::WithholdClaim => "withhold_claim",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the row owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A downstream surface that must ingest this qualification packet and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationConsumerSurface {
    /// Docs and help/service-health surfaces.
    DocsHelp,
    /// Support export bundle.
    SupportExport,
    /// Companion and browser re-entry handoff copy.
    CompanionBrowserHandoff,
    /// Release center claim and proof index.
    ReleaseCenter,
    /// Shiproom claim packet.
    Shiproom,
}

impl QualificationConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::DocsHelp,
        Self::SupportExport,
        Self::CompanionBrowserHandoff,
        Self::ReleaseCenter,
        Self::Shiproom,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::CompanionBrowserHandoff => "companion_browser_handoff",
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
        }
    }
}

/// The weaker (lower-rank) of two restore-fidelity classes.
fn weaker(a: RestoreFidelityClass, b: RestoreFidelityClass) -> RestoreFidelityClass {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// The outcome of one qualification drill, with its evidence ref and capture time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationDrillResult {
    /// Drill this result records.
    pub drill: QualificationDrill,
    /// Outcome of the drill.
    pub outcome: DrillOutcome,
    /// Ref to the drill's evidence; required whenever the drill ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Capture timestamp for the drill run.
    pub checked_at: String,
}

impl QualificationDrillResult {
    /// Whether the result carries the evidence ref its outcome requires.
    pub fn has_required_evidence(&self) -> bool {
        if self.outcome.was_run() {
            self.evidence_ref
                .as_ref()
                .is_some_and(|r| !r.trim().is_empty())
        } else {
            true
        }
    }
}

/// One qualification row for a claimed serialization family on one profile and deployment mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationRow {
    /// Stable qualification-row id.
    pub row_id: String,
    /// Serialization family this row qualifies.
    pub family: QualificationFamily,
    /// Profile (channel/platform slice) this row qualifies, e.g. `desktop.stable`.
    pub profile: String,
    /// Deployment mode this row is proven on.
    pub deployment_mode: DeploymentMode,
    /// Owner accountable for the row's evidence and conformance.
    pub owner: String,
    /// Restore fidelity the upstream serialization matrix published for this surface.
    ///
    /// The qualification gate can only narrow from here; it never re-broadens a matrix-narrowed
    /// surface.
    pub matrix_claim: RestoreFidelityClass,
    /// How fresh the qualification evidence backing this row is.
    pub evidence_freshness: EvidenceFreshness,
    /// Per-drill outcomes; one result per required drill.
    #[serde(default)]
    pub drill_results: Vec<QualificationDrillResult>,
    /// Restore fidelity the row's own evidence asserts, before the gate.
    pub declared_fidelity: RestoreFidelityClass,
    /// Restore fidelity actually published after the gate narrows the row.
    ///
    /// Must equal [`QualificationRow::effective_fidelity`].
    pub published_fidelity: RestoreFidelityClass,
    /// Marketed-claim decision the gate publishes; must equal the recomputed decision.
    pub claim_publication: ClaimPublication,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<QualificationDowngradeReason>,
    /// Recovery path surfaced when the claim is narrowed or withheld.
    pub downgrade_path: QualificationDowngradePath,
    /// Remembered-state artifact classes or surfaces this row still qualifies.
    #[serde(default)]
    pub qualified_classes: Vec<String>,
    /// Caveats attached to the published claim.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the claim.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the upstream serialization matrix packet this row qualifies.
    pub matrix_packet_ref: String,
    /// Ref to the matrix artifact-class/surface row this qualification narrows from.
    pub matrix_row_ref: String,
    /// Ref to the qualification suite backing the row.
    pub conformance_ref: String,
    /// Ref to the row's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the qualification answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Ref to the machine-readable qualification receipt.
    pub qualification_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl QualificationRow {
    /// The fidelity the row's own evidence asserted, before gate narrowing.
    pub fn capability_floor(&self) -> RestoreFidelityClass {
        self.declared_fidelity
    }

    /// Highest fidelity the drills permit, the weakest ceiling across every required drill.
    ///
    /// A missing required drill caps the row at manual review, so an incompletely drilled row can
    /// never read as qualified.
    pub fn drill_ceiling(&self) -> RestoreFidelityClass {
        let mut ceiling = RestoreFidelityClass::ExactRestore;
        for drill in QualificationDrill::ALL {
            let outcome = self
                .drill_results
                .iter()
                .find(|r| r.drill == drill)
                .map(|r| r.outcome.fidelity_ceiling())
                .unwrap_or(RestoreFidelityClass::ManualReview);
            ceiling = weaker(ceiling, outcome);
        }
        ceiling
    }

    /// The fidelity the gate permits this row to publish.
    ///
    /// Lowers the declared fidelity to the weakest ceiling implied by the matrix claim, the evidence
    /// freshness, and the drill outcomes, so a matrix-narrowed surface, stale evidence, or an
    /// unproven, narrowed, or failed drill can never publish an exact-restore claim.
    pub fn effective_fidelity(&self) -> RestoreFidelityClass {
        let mut fidelity = self.capability_floor();
        fidelity = weaker(fidelity, self.matrix_claim);
        fidelity = weaker(fidelity, self.evidence_freshness.fidelity_ceiling());
        fidelity = weaker(fidelity, self.drill_ceiling());
        fidelity
    }

    /// Whether any required drill proved the row only for a narrower slice.
    pub fn has_narrowed_drill(&self) -> bool {
        self.drill_results.iter().any(|r| r.outcome.is_narrowed())
    }

    /// Whether any required drill failed or never ran.
    pub fn has_unproven_drill(&self) -> bool {
        QualificationDrill::ALL.iter().any(|&drill| {
            self.drill_results
                .iter()
                .find(|r| r.drill == drill)
                .map(|r| r.outcome.is_unproven())
                .unwrap_or(true)
        })
    }

    /// The headline downgrade reasons recomputed from the row's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<QualificationDowngradeReason> {
        let mut reasons = Vec::new();
        if self.matrix_claim.rank() < RestoreFidelityClass::ExactRestore.rank() {
            reasons.push(QualificationDowngradeReason::MatrixNarrowed);
        }
        if self.evidence_freshness.is_stale_trigger() {
            reasons.push(QualificationDowngradeReason::EvidenceStale);
        }
        if self.has_narrowed_drill() {
            reasons.push(QualificationDowngradeReason::DrillNarrowed);
        }
        if self.has_unproven_drill() {
            reasons.push(QualificationDowngradeReason::DrillFailed);
        }
        reasons
    }

    /// The recovery path the gate must record, derived from the row's observed states.
    ///
    /// Ordered by severity: a withheld row points at a withhold, an unproven or narrowed drill
    /// points at a drill rerun, stale evidence points at a refresh, a matrix-only narrowing points
    /// at adopting that narrowing, and a clean row needs nothing.
    pub fn computed_downgrade_path(&self) -> QualificationDowngradePath {
        if self.effective_fidelity() == RestoreFidelityClass::ManualReview {
            QualificationDowngradePath::WithholdClaim
        } else if self.has_unproven_drill() || self.has_narrowed_drill() {
            QualificationDowngradePath::RerunDrills
        } else if self.evidence_freshness.is_stale_trigger() {
            QualificationDowngradePath::RefreshEvidence
        } else if self.matrix_claim.rank() < RestoreFidelityClass::ExactRestore.rank() {
            QualificationDowngradePath::AdoptMatrixNarrowing
        } else {
            QualificationDowngradePath::NoneNeeded
        }
    }

    /// The marketed-claim decision the gate must record, derived from the row's observed states.
    pub fn computed_publication(&self) -> ClaimPublication {
        if self.effective_fidelity() == RestoreFidelityClass::ManualReview {
            ClaimPublication::Withheld
        } else if self.is_downgraded() || !self.computed_downgrade_reasons().is_empty() {
            ClaimPublication::Narrowed
        } else {
            ClaimPublication::Published
        }
    }

    /// Whether the row publishes a clean full restore claim.
    pub fn is_published(&self) -> bool {
        self.computed_publication() == ClaimPublication::Published
    }

    /// Whether the gate narrowed the published fidelity below what the row declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_fidelity().rank() < self.capability_floor().rank()
    }

    /// Whether the row covers every required drill exactly once.
    pub fn covers_all_drills(&self) -> bool {
        let mut seen = BTreeSet::new();
        for result in &self.drill_results {
            seen.insert(result.drill);
        }
        QualificationDrill::ALL.iter().all(|d| seen.contains(d))
            && self.drill_results.len() == QualificationDrill::ALL.len()
    }

    /// Whether the row carries its own non-empty matrix, conformance, evidence, scope, and receipt
    /// refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.matrix_packet_ref.trim().is_empty()
            && !self.matrix_row_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
            && !self.qualification_receipt_ref.trim().is_empty()
    }

    /// Whether the stored published fidelity, decision, reasons, and path all agree with the
    /// recomputed gate.
    pub fn gate_consistent(&self) -> bool {
        self.published_fidelity == self.effective_fidelity()
            && self.claim_publication == self.computed_publication()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.downgrade_path == self.computed_downgrade_path()
    }
}

/// One binding wiring a downstream surface to this qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: QualificationConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Qualification packet id this surface ingests.
    pub qualification_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this qualification packet rather than a parallel sheet.
    pub ingests_qualification_packet: bool,
    /// True when the surface preserves the published fidelity verbatim.
    pub preserves_published_fidelity: bool,
    /// True when the surface preserves the recovery paths verbatim.
    pub preserves_downgrade_paths: bool,
    /// True when the surface narrows automatically as rows are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl QualificationConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.qualification_packet_id_ref == packet_id
            && self.ingests_qualification_packet
            && self.preserves_published_fidelity
            && self.preserves_downgrade_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SerializationQualificationSummary {
    /// Total qualification rows.
    pub total_rows: usize,
    /// Number of claimed families.
    pub family_count: usize,
    /// Rows publishing a full restore claim.
    pub published_rows: usize,
    /// Rows whose claim was auto-narrowed.
    pub narrowed_rows: usize,
    /// Rows whose claim was withheld.
    pub withheld_rows: usize,
    /// Rows whose published fidelity was downgraded below what they declared.
    pub downgraded_rows: usize,
    /// Rows carrying at least one downgrade reason.
    pub rows_with_downgrade_reasons: usize,
    /// Rows whose qualification evidence is aging, expired, or missing.
    pub stale_evidence_rows: usize,
    /// Rows with at least one narrowed, failed, or not-run drill.
    pub rows_with_imperfect_drills: usize,
}

/// A redaction-safe export row projected from a qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SerializationQualificationExportRow {
    /// Qualification-row id.
    pub row_id: String,
    /// Family token.
    pub family: String,
    /// Profile slice.
    pub profile: String,
    /// Deployment-mode token.
    pub deployment_mode: String,
    /// Owner accountable for the row.
    pub owner: String,
    /// Matrix-claim token the row narrows from.
    pub matrix_claim: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Declared-fidelity token.
    pub declared_fidelity: String,
    /// Published-fidelity token.
    pub published_fidelity: String,
    /// Claim-publication token.
    pub claim_publication: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Remembered-state classes or surfaces still qualified.
    pub qualified_classes: Vec<String>,
    /// Caveats attached to the published claim.
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Matrix-packet ref this row qualifies.
    pub matrix_packet_ref: String,
    /// Scope snapshot the qualification answered.
    pub scope_snapshot_ref: String,
    /// Qualification-receipt ref.
    pub qualification_receipt_ref: String,
    /// Whether the row publishes a full restore claim.
    pub published: bool,
    /// Whether the published fidelity was downgraded below the declared fidelity.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the canonical qualification index downstream
/// surfaces render instead of restating each row's fidelity by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SerializationQualificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5SerializationQualificationExportRow>,
    /// Whether every row's published fidelity and decision agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows that publish a full restore claim.
    pub published_count: usize,
    /// Rows the gate narrowed.
    pub narrowed_count: usize,
    /// Rows the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 serialization-qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SerializationQualification {
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
    /// Ref to the upstream serialization-and-restore matrix this packet qualifies.
    pub matrix_packet_ref: String,
    /// Claimed families; at least one row per family.
    pub families: Vec<QualificationFamily>,
    /// Closed deployment-mode vocabulary.
    pub deployment_modes: Vec<DeploymentMode>,
    /// Closed restore-fidelity vocabulary, reused from the serialization matrix.
    pub fidelity_labels: Vec<RestoreFidelityClass>,
    /// Closed drill vocabulary.
    pub drills: Vec<QualificationDrill>,
    /// Closed drill-outcome vocabulary.
    pub drill_outcomes: Vec<DrillOutcome>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed claim-publication vocabulary.
    pub claim_publications: Vec<ClaimPublication>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<QualificationDowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<QualificationDowngradeReason>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<QualificationConsumerSurface>,
    /// Qualification rows.
    #[serde(default)]
    pub rows: Vec<QualificationRow>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<QualificationConsumerBinding>,
    /// Summary counts.
    pub summary: M5SerializationQualificationSummary,
}

impl M5SerializationQualification {
    /// Rows for a claimed family.
    pub fn rows_for_family(
        &self,
        family: QualificationFamily,
    ) -> impl Iterator<Item = &QualificationRow> {
        self.rows.iter().filter(move |r| r.family == family)
    }

    /// Returns the row with the given id.
    pub fn row(&self, row_id: &str) -> Option<&QualificationRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Rows that publish a full restore claim.
    pub fn published_rows(&self) -> impl Iterator<Item = &QualificationRow> {
        self.rows.iter().filter(|r| r.is_published())
    }

    /// Rows the gate auto-narrowed.
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &QualificationRow> {
        self.rows
            .iter()
            .filter(|r| r.computed_publication() == ClaimPublication::Narrowed)
    }

    /// Rows the gate withheld entirely.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &QualificationRow> {
        self.rows
            .iter()
            .filter(|r| r.computed_publication() == ClaimPublication::Withheld)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: QualificationConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every row's stored published fidelity, decision, reasons, and path agree with the
    /// recomputed gate.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5SerializationQualificationSummary {
        let count_publication = |publication: ClaimPublication| {
            self.rows
                .iter()
                .filter(|r| r.claim_publication == publication)
                .count()
        };
        M5SerializationQualificationSummary {
            total_rows: self.rows.len(),
            family_count: self.families.len(),
            published_rows: count_publication(ClaimPublication::Published),
            narrowed_rows: count_publication(ClaimPublication::Narrowed),
            withheld_rows: count_publication(ClaimPublication::Withheld),
            downgraded_rows: self.rows.iter().filter(|r| r.is_downgraded()).count(),
            rows_with_downgrade_reasons: self
                .rows
                .iter()
                .filter(|r| !r.downgrade_reasons.is_empty())
                .count(),
            stale_evidence_rows: self
                .rows
                .iter()
                .filter(|r| r.evidence_freshness.is_stale_trigger())
                .count(),
            rows_with_imperfect_drills: self
                .rows
                .iter()
                .filter(|r| r.has_narrowed_drill() || r.has_unproven_drill())
                .count(),
        }
    }

    /// Produces the qualification index downstream surfaces — docs/help, support export,
    /// companion/browser handoff, release center, and shiproom — render instead of restating each
    /// row's qualification by hand.
    pub fn export_projection(&self) -> M5SerializationQualificationExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|r| M5SerializationQualificationExportRow {
                row_id: r.row_id.clone(),
                family: r.family.as_str().to_owned(),
                profile: r.profile.clone(),
                deployment_mode: r.deployment_mode.as_str().to_owned(),
                owner: r.owner.clone(),
                matrix_claim: r.matrix_claim.as_str().to_owned(),
                evidence_freshness: r.evidence_freshness.as_str().to_owned(),
                declared_fidelity: r.declared_fidelity.as_str().to_owned(),
                published_fidelity: r.published_fidelity.as_str().to_owned(),
                claim_publication: r.claim_publication.as_str().to_owned(),
                downgrade_reasons: r
                    .downgrade_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                downgrade_path: r.downgrade_path.as_str().to_owned(),
                qualified_classes: r.qualified_classes.clone(),
                caveats: r.caveats.clone(),
                stale_or_missing_fields: r.stale_or_missing_fields.clone(),
                matrix_packet_ref: r.matrix_packet_ref.clone(),
                scope_snapshot_ref: r.scope_snapshot_ref.clone(),
                qualification_receipt_ref: r.qualification_receipt_ref.clone(),
                published: r.is_published(),
                downgraded: r.is_downgraded(),
                summary: format!(
                    "{} on {} ({}): matrix {}, evidence {}, declared {}, published {} ({}), recovery {}",
                    r.family.as_str(),
                    r.profile,
                    r.deployment_mode.as_str(),
                    r.matrix_claim.as_str(),
                    r.evidence_freshness.as_str(),
                    r.declared_fidelity.as_str(),
                    r.published_fidelity.as_str(),
                    r.claim_publication.as_str(),
                    r.downgrade_path.as_str()
                ),
            })
            .collect();
        M5SerializationQualificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            published_count: self.published_rows().count(),
            narrowed_count: self.narrowed_rows().count(),
            withheld_count: self.withheld_rows().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact qualification report.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SerializationQualificationSupportExport {
        M5SerializationQualificationSupportExport {
            record_kind: M5_SERIALIZATION_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SERIALIZATION_QUALIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            qualification_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            qualification: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5SerializationQualificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<QualificationFamily> = self.families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut covered_families = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5SerializationQualificationViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            covered_families.insert(row.family);
            if !claimed.contains(&row.family) {
                violations.push(M5SerializationQualificationViolation::UnclaimedFamilyRow {
                    row_id: row.row_id.clone(),
                    family: row.family.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed family must carry at least one row, so a family never inherits a
        // qualification from an adjacent one.
        for &family in &self.families {
            if !covered_families.contains(&family) {
                violations.push(M5SerializationQualificationViolation::MissingFamilyRow {
                    family: family.as_str(),
                });
            }
        }

        // Every required consumer surface must bind to this packet and narrow with it, so a
        // narrowed row cannot stay green on a downstream surface by inertia.
        for surface in QualificationConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(
                    M5SerializationQualificationViolation::MissingConsumerBinding {
                        surface: surface.as_str(),
                    },
                );
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(
                    M5SerializationQualificationViolation::ConsumerBindingDrift {
                        binding_ref: binding.binding_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SerializationQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SerializationQualificationViolation>) {
        if self.schema_version != M5_SERIALIZATION_QUALIFICATION_SCHEMA_VERSION {
            violations.push(
                M5SerializationQualificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_SERIALIZATION_QUALIFICATION_RECORD_KIND {
            violations.push(
                M5SerializationQualificationViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("matrix_packet_ref", &self.matrix_packet_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SerializationQualificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.matrix_packet_ref != M5_SERIALIZATION_QUALIFICATION_MATRIX_PACKET_REF {
            violations.push(
                M5SerializationQualificationViolation::MatrixPacketMismatch {
                    expected: M5_SERIALIZATION_QUALIFICATION_MATRIX_PACKET_REF,
                },
            );
        }
        for (field, ok) in [
            (
                "families",
                self.families == QualificationFamily::ALL.to_vec(),
            ),
            (
                "deployment_modes",
                self.deployment_modes == DeploymentMode::ALL.to_vec(),
            ),
            (
                "fidelity_labels",
                self.fidelity_labels == RestoreFidelityClass::ALL.to_vec(),
            ),
            ("drills", self.drills == QualificationDrill::ALL.to_vec()),
            (
                "drill_outcomes",
                self.drill_outcomes == DrillOutcome::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "claim_publications",
                self.claim_publications == ClaimPublication::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == QualificationDowngradePath::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == QualificationDowngradeReason::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == QualificationConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    M5SerializationQualificationViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<M5SerializationQualificationViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("profile", &row.profile),
            ("owner", &row.owner),
            ("matrix_packet_ref", &row.matrix_packet_ref),
            ("matrix_row_ref", &row.matrix_row_ref),
            ("conformance_ref", &row.conformance_ref),
            ("evidence_ref", &row.evidence_ref),
            ("scope_snapshot_ref", &row.scope_snapshot_ref),
            ("qualification_receipt_ref", &row.qualification_receipt_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SerializationQualificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // The row must qualify the canonical matrix packet, so a qualification never narrows from a
        // packet other than the matrix it claims to gate.
        if row.matrix_packet_ref != M5_SERIALIZATION_QUALIFICATION_MATRIX_PACKET_REF {
            violations.push(
                M5SerializationQualificationViolation::MatrixPacketMismatch {
                    expected: M5_SERIALIZATION_QUALIFICATION_MATRIX_PACKET_REF,
                },
            );
        }

        // The row must cover every required drill exactly once, so an incompletely drilled row is
        // never qualified by omission.
        if !row.covers_all_drills() {
            violations.push(
                M5SerializationQualificationViolation::IncompleteDrillCoverage {
                    row_id: row.row_id.clone(),
                },
            );
        }
        for result in &row.drill_results {
            if result.checked_at.trim().is_empty() {
                violations.push(M5SerializationQualificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "drill_results.checked_at",
                });
            }
            if !result.has_required_evidence() {
                violations.push(
                    M5SerializationQualificationViolation::DrillMissingEvidence {
                        row_id: row.row_id.clone(),
                        drill: result.drill.as_str(),
                    },
                );
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5SerializationQualificationViolation::DuplicateDowngradeReason {
                        row_id: row.row_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published fidelity must equal the gate's recomputed ceiling, so a matrix-narrowed,
        // stale, or under-drilled row can never read as exact restore.
        let effective = row.effective_fidelity();
        if row.published_fidelity != effective {
            violations.push(M5SerializationQualificationViolation::OverstatedFidelity {
                row_id: row.row_id.clone(),
                published: row.published_fidelity.as_str(),
                computed: effective.as_str(),
            });
        }

        // The published fidelity may never exceed the matrix claim, the cornerstone of the
        // non-inheritance guarantee: a qualification never re-broadens a matrix-narrowed surface.
        if row.published_fidelity.rank() > row.matrix_claim.rank() {
            violations.push(M5SerializationQualificationViolation::ExceedsMatrix {
                row_id: row.row_id.clone(),
                published: row.published_fidelity.as_str(),
                matrix: row.matrix_claim.as_str(),
            });
        }

        let required_publication = row.computed_publication();
        if row.claim_publication != required_publication {
            violations.push(M5SerializationQualificationViolation::PublicationMismatch {
                row_id: row.row_id.clone(),
                declared: row.claim_publication.as_str(),
                required: required_publication.as_str(),
            });
        }

        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(
                M5SerializationQualificationViolation::DowngradeReasonsMismatch {
                    row_id: row.row_id.clone(),
                },
            );
        }

        let computed_path = row.computed_downgrade_path();
        if row.downgrade_path != computed_path {
            violations.push(
                M5SerializationQualificationViolation::DowngradePathMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.downgrade_path.as_str(),
                    required: computed_path.as_str(),
                },
            );
        }

        // A narrowed or withheld row must offer a real recovery path, list a caveat, and name what
        // is stale, so a degraded row never drops its recovery semantics or hides why it narrowed.
        if row.claim_publication.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(
                    M5SerializationQualificationViolation::MissingDowngradePath {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.caveats.is_empty() {
                violations.push(M5SerializationQualificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5SerializationQualificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A row that still publishes a claim must name at least one qualified class or surface.
        if row.claim_publication != ClaimPublication::Withheld && row.qualified_classes.is_empty() {
            violations.push(M5SerializationQualificationViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "qualified_classes",
            });
        }

        // A published row must be genuinely whole-provable: the matrix claim is exact, the evidence
        // is current, every drill passed, the declared fidelity is exact, and nothing narrows it.
        // This is the guardrail against a blanket 'restore supported' claim over an unproven row.
        if row.is_published()
            && (row.matrix_claim != RestoreFidelityClass::ExactRestore
                || row.evidence_freshness != EvidenceFreshness::Current
                || row.drill_ceiling() != RestoreFidelityClass::ExactRestore
                || row.capability_floor() != RestoreFidelityClass::ExactRestore
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered())
        {
            violations.push(
                M5SerializationQualificationViolation::PublishedRowNotWhole {
                    row_id: row.row_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the M5 serialization-qualification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SerializationQualificationViolation {
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
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A qualification-row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A claimed family has no row.
    MissingFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A row covers a family the packet does not claim.
    UnclaimedFamilyRow {
        /// Row id.
        row_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A row or the packet binds to a matrix packet other than the canonical one.
    MatrixPacketMismatch {
        /// Expected matrix-packet path.
        expected: &'static str,
    },
    /// A row does not cover every required drill exactly once.
    IncompleteDrillCoverage {
        /// Row id.
        row_id: String,
    },
    /// A drill that ran carries no evidence ref.
    DrillMissingEvidence {
        /// Row id.
        row_id: String,
        /// Drill token.
        drill: &'static str,
    },
    /// A row lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Row id.
        row_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A row publishes a fidelity beyond what the gate computes.
    OverstatedFidelity {
        /// Row id.
        row_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Computed effective fidelity token.
        computed: &'static str,
    },
    /// A row publishes a fidelity above the upstream matrix claim.
    ExceedsMatrix {
        /// Row id.
        row_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Matrix claim token.
        matrix: &'static str,
    },
    /// A row's claim-publication decision disagrees with the gate.
    PublicationMismatch {
        /// Row id.
        row_id: String,
        /// Declared publication token.
        declared: &'static str,
        /// Required publication token.
        required: &'static str,
    },
    /// A row's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        row_id: String,
    },
    /// A row's downgrade path disagrees with the recomputed path.
    DowngradePathMismatch {
        /// Row id.
        row_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A narrowed or withheld row offers no recovery path.
    MissingDowngradePath {
        /// Row id.
        row_id: String,
    },
    /// A published row still narrows a state or carries a downgrade reason.
    PublishedRowNotWhole {
        /// Row id.
        row_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints qualification truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5SerializationQualificationViolation {
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
            Self::DuplicateRowId { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::MissingFamilyRow { family } => {
                write!(f, "missing row for claimed family {family}")
            }
            Self::UnclaimedFamilyRow { row_id, family } => {
                write!(f, "row {row_id} covers unclaimed family {family}")
            }
            Self::MatrixPacketMismatch { expected } => {
                write!(
                    f,
                    "matrix_packet_ref must be the canonical serialization matrix {expected}"
                )
            }
            Self::IncompleteDrillCoverage { row_id } => {
                write!(f, "row {row_id} does not cover every required drill once")
            }
            Self::DrillMissingEvidence { row_id, drill } => {
                write!(f, "row {row_id} drill {drill} ran without an evidence ref")
            }
            Self::DuplicateDowngradeReason { row_id, reason } => {
                write!(f, "row {row_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedFidelity {
                row_id,
                published,
                computed,
            } => write!(
                f,
                "row {row_id} publishes fidelity {published} but the gate computes {computed}"
            ),
            Self::ExceedsMatrix {
                row_id,
                published,
                matrix,
            } => write!(
                f,
                "row {row_id} publishes fidelity {published} above matrix claim {matrix}"
            ),
            Self::PublicationMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records publication {declared} but the gate requires {required}"
            ),
            Self::DowngradeReasonsMismatch { row_id } => {
                write!(f, "row {row_id} downgrade reasons disagree with the gate")
            }
            Self::DowngradePathMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::MissingDowngradePath { row_id } => {
                write!(
                    f,
                    "row {row_id} is narrowed or withheld but offers no recovery path"
                )
            }
            Self::PublishedRowNotWhole { row_id } => {
                write!(
                    f,
                    "row {row_id} publishes a full claim but narrows a state or carries a downgrade reason"
                )
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(
                    f,
                    "binding {binding_ref} does not preserve qualification truth"
                )
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for M5SerializationQualificationViolation {}

/// Stable record-kind tag for [`M5SerializationQualificationSupportExport`].
pub const M5_SERIALIZATION_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_serialization_qualification_support_export";

/// Support-export wrapper preserving the qualification report verbatim for support and evidence
/// packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SerializationQualificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub qualification_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact qualification report preserved by the export.
    pub qualification: M5SerializationQualification,
}

impl M5SerializationQualificationSupportExport {
    /// Whether the export preserves the same packet id and a clean report.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_SERIALIZATION_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_SERIALIZATION_QUALIFICATION_SCHEMA_VERSION
            && self.qualification_packet_id_ref == self.qualification.packet_id
            && self.raw_private_material_excluded
            && self.qualification.validate().is_empty()
    }
}

/// Loads the embedded M5 serialization-qualification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SerializationQualification`].
pub fn current_m5_serialization_qualification(
) -> Result<M5SerializationQualification, serde_json::Error> {
    serde_json::from_str(M5_SERIALIZATION_QUALIFICATION_JSON)
}

#[cfg(test)]
mod tests;
