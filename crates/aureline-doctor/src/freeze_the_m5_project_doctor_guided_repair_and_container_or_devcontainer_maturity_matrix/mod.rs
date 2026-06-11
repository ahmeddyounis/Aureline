//! Frozen M5 Project Doctor, guided-repair, and container/devcontainer maturity
//! matrix with a non-inheriting promotion gate.
//!
//! This module owns the canonical packet that certifies, for every blocked-user
//! recovery capability and every deployment profile, whether that cell carries a
//! current feature scorecard, diagnosis-latency corpus, compatibility story, and
//! rollback path of its own — rather than inheriting trust from an adjacent cell
//! or an older stable-line claim. Each [`MaturityRow`] names its
//! [`RecoveryCapability`], its [`DeploymentProfile`], the maturity the underlying
//! lane assessed, the evidence freshness, the repair [`ReversalClass`], the
//! desktop/CLI/support [`SupportParity`], and the proof refs that back it.
//!
//! The model is a promotion gate, not a label store. The maturity that may be
//! *published* for a row is derived deterministically from its inputs: a row that
//! is stale, served through an unavailable engine/route, past its diagnosis-latency
//! SLO, or missing its scorecard, latency corpus, rollback path, or boundary proof
//! cannot publish a full certification, and its [`NarrowingAction`] records exactly
//! how the gate narrowed it. Because [`MaturityRow::published_maturity`] and
//! [`MaturityRow::narrowing_action`] are validated against the recomputed gate
//! decision, release/public-truth tooling can prove that underqualified rows narrow
//! automatically before publication and that no row publishes beyond what its own
//! proof supports.
//!
//! Certification stays cell-specific. The packet pins the recovery-capability and
//! deployment-profile vocabulary and requires one row for every (capability,
//! profile) cell, so a strong local lane never implies maturity on a remote or
//! containerized one, and a row never covers a profile the matrix does not claim.
//! Guided-repair rows additionally must carry a concrete reversal class so repair
//! safety can never widen through support folklore.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/doctor-repair-container-maturity-matrix.json` and embedded
//! here, so this typed consumer and any CI gate agree on every row without a cargo
//! build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, or mount/port/tunnel
//! secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported Doctor/repair/container maturity-matrix packet schema version.
pub const DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_RECORD_KIND: &str =
    "doctor_repair_container_maturity_matrix";

/// Repo-relative path to the checked-in packet.
pub const DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_PATH: &str =
    "artifacts/doctor/m5/doctor-repair-container-maturity-matrix.json";

/// Embedded checked-in packet JSON.
pub const DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/doctor-repair-container-maturity-matrix.json"
));

/// A blocked-user recovery capability the maturity matrix makes claims about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCapability {
    /// Project Doctor probe families, finding codes, and explainability.
    ProjectDoctor,
    /// Guided-repair / repair-transaction classes and reversal behavior.
    GuidedRepair,
    /// Container and devcontainer boundary depth (engine, mount, port, tunnel).
    ContainerBoundary,
}

impl RecoveryCapability {
    /// Every recovery capability, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ProjectDoctor,
        Self::GuidedRepair,
        Self::ContainerBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDoctor => "project_doctor",
            Self::GuidedRepair => "guided_repair",
            Self::ContainerBoundary => "container_boundary",
        }
    }

    /// Whether a row for this capability performs mutating repair work and so
    /// must carry a concrete reversal class.
    pub const fn is_mutating_repair(self) -> bool {
        matches!(self, Self::GuidedRepair)
    }
}

/// A deployment profile each capability is certified against independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    /// Local workspace on the host machine.
    LocalWorkspace,
    /// Remote workspace reached over SSH.
    RemoteSsh,
    /// Workspace running inside a container engine.
    Container,
    /// Workspace running inside a devcontainer.
    Devcontainer,
}

impl DeploymentProfile {
    /// Every deployment profile, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalWorkspace,
        Self::RemoteSsh,
        Self::Container,
        Self::Devcontainer,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RemoteSsh => "remote_ssh",
            Self::Container => "container",
            Self::Devcontainer => "devcontainer",
        }
    }
}

/// Maturity class of a recovery capability on a deployment profile.
///
/// Ordered low-to-high by [`MaturityClass::rank`]: an [`MaturityClass::Unsupported`]
/// cell carries no claim, and an [`MaturityClass::Certified`] cell carries a full,
/// current, evidence-backed claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaturityClass {
    /// Full, current, evidence-backed certification.
    Certified,
    /// Partial coverage; published as provisional/beta depth only.
    Provisional,
    /// Below the bar for any positive depth claim.
    Underqualified,
    /// Not supported on this profile; carries no claim.
    Unsupported,
}

impl MaturityClass {
    /// Every maturity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Provisional,
        Self::Underqualified,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Provisional => "provisional",
            Self::Underqualified => "underqualified",
            Self::Unsupported => "unsupported",
        }
    }

    /// Monotonic rank; higher means a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::Underqualified => 1,
            Self::Provisional => 2,
            Self::Certified => 3,
        }
    }

    /// The weaker (lower-rank) of two maturity classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// Freshness of a row's proof relative to its evidence-freshness SLO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Proof is current within its freshness SLO.
    Current,
    /// Proof is present but past its freshness SLO.
    Stale,
    /// Proof has expired and no longer backs a live claim.
    Expired,
    /// Proof freshness cannot be established.
    Unknown,
}

impl EvidenceFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Expired, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the proof is current within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Highest maturity this freshness alone permits a row to publish.
    ///
    /// Only `current` proof may publish a full certification; stale or unknown
    /// proof narrows to provisional, and expired proof narrows to underqualified.
    pub const fn maturity_ceiling(self) -> MaturityClass {
        match self {
            Self::Current => MaturityClass::Certified,
            Self::Stale | Self::Unknown => MaturityClass::Provisional,
            Self::Expired => MaturityClass::Underqualified,
        }
    }
}

/// The reversal class of a repair, keeping repair safety canonical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversalClass {
    /// Fully reversible with no side effects.
    Reversible,
    /// Reversible only through a captured checkpoint receipt.
    Checkpointed,
    /// Cannot be reversed once applied.
    Irreversible,
    /// Read-only lane; no repair is performed.
    NotApplicable,
}

impl ReversalClass {
    /// Every reversal class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Reversible,
        Self::Checkpointed,
        Self::Irreversible,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "reversible",
            Self::Checkpointed => "checkpointed",
            Self::Irreversible => "irreversible",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Desktop/CLI/support parity carried alongside a row's certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportParity {
    /// Desktop, CLI, and support-export surfaces all carry the capability.
    Full,
    /// Desktop and CLI carry it; support-export parity is partial.
    DesktopCli,
    /// Only the desktop surface carries it.
    DesktopOnly,
    /// No surface carries the capability on this profile.
    Unavailable,
}

impl SupportParity {
    /// Every support-parity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Full,
        Self::DesktopCli,
        Self::DesktopOnly,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::DesktopCli => "desktop_cli",
            Self::DesktopOnly => "desktop_only",
            Self::Unavailable => "unavailable",
        }
    }
}

/// A reason a row cannot promote at full strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockingReason {
    /// The proof packet is stale.
    Stale,
    /// The container engine or remote route is unavailable.
    EngineUnavailable,
    /// The diagnosis-latency SLO is breached for this cell.
    LatencySloBreached,
    /// The feature scorecard or diagnosis-latency proof corpus is missing.
    MissingProofCorpus,
    /// No durable rollback path is bound to this cell.
    MissingRollbackPath,
    /// The mount/port/tunnel boundary scope is unverified.
    BoundaryUnverified,
}

impl BlockingReason {
    /// Every blocking reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stale,
        Self::EngineUnavailable,
        Self::LatencySloBreached,
        Self::MissingProofCorpus,
        Self::MissingRollbackPath,
        Self::BoundaryUnverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stale => "stale",
            Self::EngineUnavailable => "engine_unavailable",
            Self::LatencySloBreached => "latency_slo_breached",
            Self::MissingProofCorpus => "missing_proof_corpus",
            Self::MissingRollbackPath => "missing_rollback_path",
            Self::BoundaryUnverified => "boundary_unverified",
        }
    }

    /// Highest maturity a row carrying this blocking reason may publish.
    ///
    /// A stale packet, unavailable engine, or breached latency SLO narrows to
    /// provisional; a missing scorecard/corpus, missing rollback path, or
    /// unverified boundary narrows all the way to underqualified.
    pub const fn maturity_ceiling(self) -> MaturityClass {
        match self {
            Self::Stale | Self::EngineUnavailable | Self::LatencySloBreached => {
                MaturityClass::Provisional
            }
            Self::MissingProofCorpus | Self::MissingRollbackPath | Self::BoundaryUnverified => {
                MaturityClass::Underqualified
            }
        }
    }

    /// Whether this reason reflects missing evidence rather than degraded state.
    pub const fn is_missing_evidence(self) -> bool {
        matches!(
            self,
            Self::MissingProofCorpus | Self::MissingRollbackPath | Self::BoundaryUnverified
        )
    }
}

/// The action the publication gate takes on a row relative to a full
/// certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingAction {
    /// No narrowing; the row publishes a full certification.
    None,
    /// Narrow the published claim to provisional/beta depth.
    NarrowToProvisional,
    /// Narrow the published claim to underqualified (no positive depth claim).
    NarrowToUnderqualified,
    /// Withhold the cell from publication entirely.
    WithholdFromPublication,
}

impl NarrowingAction {
    /// Every narrowing action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::NarrowToProvisional,
        Self::NarrowToUnderqualified,
        Self::WithholdFromPublication,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NarrowToProvisional => "narrow_to_provisional",
            Self::NarrowToUnderqualified => "narrow_to_underqualified",
            Self::WithholdFromPublication => "withhold_from_publication",
        }
    }

    /// The narrowing action implied by a published maturity.
    pub const fn for_published(maturity: MaturityClass) -> Self {
        match maturity {
            MaturityClass::Certified => Self::None,
            MaturityClass::Provisional => Self::NarrowToProvisional,
            MaturityClass::Underqualified => Self::NarrowToUnderqualified,
            MaturityClass::Unsupported => Self::WithholdFromPublication,
        }
    }
}

/// One maturity row for a (capability, profile) cell of the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaturityRow {
    /// Stable row id.
    pub row_id: String,
    /// Recovery capability this row certifies.
    pub capability: RecoveryCapability,
    /// Deployment profile this row certifies.
    pub deployment_profile: DeploymentProfile,
    /// Maturity the underlying lane assessed, before the publication gate.
    pub declared_maturity: MaturityClass,
    /// Maturity actually published after the gate narrows the row.
    ///
    /// Must equal [`MaturityRow::effective_maturity`]; validation rejects a row
    /// that publishes beyond what its proof supports.
    pub published_maturity: MaturityClass,
    /// Proof freshness relative to the cell's freshness SLO.
    pub evidence_freshness: EvidenceFreshness,
    /// Reversal class of the repair this cell performs.
    pub reversal_class: ReversalClass,
    /// Desktop/CLI/support parity carried by this cell.
    pub support_parity: SupportParity,
    /// Action the gate takes on this row; must equal the recomputed narrowing.
    pub narrowing_action: NarrowingAction,
    /// Reasons this cell cannot promote at full strength.
    #[serde(default)]
    pub blocking_reasons: Vec<BlockingReason>,
    /// Ref to the cell's feature scorecard.
    pub scorecard_ref: String,
    /// Ref to the cell's diagnosis-latency proof corpus.
    pub latency_corpus_ref: String,
    /// Ref to the cell's durable rollback path.
    pub rollback_ref: String,
    /// Ref to the cell's compatibility/downgrade story.
    pub compatibility_ref: String,
    /// Optional ref to the cell's admin/policy story, where one is relevant.
    #[serde(default)]
    pub admin_policy_ref: Option<String>,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl MaturityRow {
    /// Whether the row carries its own scorecard, latency corpus, and rollback
    /// path and is not flagged as missing proof, rollback, or boundary evidence.
    pub fn has_required_evidence(&self) -> bool {
        !self.scorecard_ref.trim().is_empty()
            && !self.latency_corpus_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
            && !self
                .blocking_reasons
                .iter()
                .any(|r| r.is_missing_evidence())
    }

    /// The maturity the publication gate permits this row to publish.
    ///
    /// Starts from [`MaturityRow::declared_maturity`] and lowers it to the weakest
    /// ceiling implied by the evidence freshness, every blocking reason, and any
    /// missing required evidence — so a stale, engine-unavailable, latency-breached,
    /// or evidence-missing row can never publish a full certification.
    pub fn effective_maturity(&self) -> MaturityClass {
        let mut ceiling = self
            .declared_maturity
            .min(self.evidence_freshness.maturity_ceiling());
        for reason in &self.blocking_reasons {
            ceiling = ceiling.min(reason.maturity_ceiling());
        }
        if !self.has_required_evidence() {
            ceiling = ceiling.min(MaturityClass::Underqualified);
        }
        ceiling
    }

    /// The narrowing action the gate must record for this row.
    pub fn required_narrowing(&self) -> NarrowingAction {
        NarrowingAction::for_published(self.effective_maturity())
    }

    /// Whether the row may publish a full certification.
    pub fn is_promotable(&self) -> bool {
        self.effective_maturity() == MaturityClass::Certified
    }

    /// Whether a mutating-repair cell carries a concrete reversal class.
    ///
    /// A guided-repair row may never publish `not_applicable`; read-only lanes
    /// (Project Doctor probes) use `not_applicable` as their canonical value.
    pub fn reversal_class_consistent(&self) -> bool {
        if self.capability.is_mutating_repair() {
            self.reversal_class != ReversalClass::NotApplicable
        } else {
            true
        }
    }

    /// Whether the stored published maturity and narrowing action agree with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_maturity == self.effective_maturity()
            && self.narrowing_action == self.required_narrowing()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DoctorRepairContainerMaturityMatrixSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Number of recovery capabilities.
    pub capability_count: usize,
    /// Number of deployment profiles.
    pub deployment_profile_count: usize,
    /// Rows published as certified.
    pub certified_rows: usize,
    /// Rows published as provisional.
    pub provisional_rows: usize,
    /// Rows published as underqualified.
    pub underqualified_rows: usize,
    /// Rows published as unsupported.
    pub unsupported_rows: usize,
    /// Rows that may publish a full certification.
    pub promotable_rows: usize,
    /// Rows the gate narrowed in any way.
    pub narrowed_rows: usize,
    /// Rows the gate withheld from publication.
    pub withheld_rows: usize,
    /// Rows with current proof freshness.
    pub current_freshness_rows: usize,
    /// Rows carrying at least one blocking reason.
    pub rows_with_blocking_reasons: usize,
}

/// A redaction-safe export row projected from a maturity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorRepairContainerMaturityMatrixExportRow {
    /// Row id.
    pub row_id: String,
    /// Capability token.
    pub capability: String,
    /// Deployment-profile token.
    pub deployment_profile: String,
    /// Declared maturity token.
    pub declared_maturity: String,
    /// Published maturity token.
    pub published_maturity: String,
    /// Evidence freshness token.
    pub evidence_freshness: String,
    /// Reversal class token.
    pub reversal_class: String,
    /// Support-parity token.
    pub support_parity: String,
    /// Narrowing action token.
    pub narrowing_action: String,
    /// Blocking reason tokens.
    pub blocking_reasons: Vec<String>,
    /// Feature scorecard ref.
    pub scorecard_ref: String,
    /// Diagnosis-latency corpus ref.
    pub latency_corpus_ref: String,
    /// Rollback path ref.
    pub rollback_ref: String,
    /// Compatibility/downgrade ref.
    pub compatibility_ref: String,
    /// Whether the row publishes a full certification.
    pub publication_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorRepairContainerMaturityMatrixExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<DoctorRepairContainerMaturityMatrixExportRow>,
    /// Whether every row's published maturity and narrowing agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows that may publish a full certification.
    pub promotable_count: usize,
    /// Rows the gate narrowed in any way.
    pub narrowed_count: usize,
    /// Rows the gate withheld from publication.
    pub withheld_count: usize,
}

/// The typed Doctor/repair/container maturity-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DoctorRepairContainerMaturityMatrix {
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
    /// Recovery capabilities the matrix claims; one row per capability and profile.
    pub capabilities: Vec<RecoveryCapability>,
    /// Closed deployment-profile vocabulary.
    pub deployment_profiles: Vec<DeploymentProfile>,
    /// Closed maturity-class vocabulary.
    pub maturity_classes: Vec<MaturityClass>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed reversal-class vocabulary.
    pub reversal_classes: Vec<ReversalClass>,
    /// Closed support-parity vocabulary.
    pub support_parities: Vec<SupportParity>,
    /// Closed blocking-reason vocabulary.
    pub blocking_reasons: Vec<BlockingReason>,
    /// Closed narrowing-action vocabulary.
    pub narrowing_actions: Vec<NarrowingAction>,
    /// Maturity rows, one per (capability, profile) cell.
    #[serde(default)]
    pub rows: Vec<MaturityRow>,
    /// Summary counts.
    pub summary: DoctorRepairContainerMaturityMatrixSummary,
}

impl DoctorRepairContainerMaturityMatrix {
    /// Returns the row for a (capability, profile) cell.
    pub fn row(
        &self,
        capability: RecoveryCapability,
        profile: DeploymentProfile,
    ) -> Option<&MaturityRow> {
        self.rows
            .iter()
            .find(|r| r.capability == capability && r.deployment_profile == profile)
    }

    /// Rows that may publish a full certification.
    pub fn promotable_rows(&self) -> impl Iterator<Item = &MaturityRow> {
        self.rows.iter().filter(|r| r.is_promotable())
    }

    /// Rows the gate narrowed in any way.
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &MaturityRow> {
        self.rows
            .iter()
            .filter(|r| r.required_narrowing() != NarrowingAction::None)
    }

    /// Rows the gate withheld from publication.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &MaturityRow> {
        self.rows
            .iter()
            .filter(|r| r.required_narrowing() == NarrowingAction::WithholdFromPublication)
    }

    /// Whether every row's stored published maturity and narrowing action agree
    /// with the recomputed gate decision.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> DoctorRepairContainerMaturityMatrixSummary {
        let count_published = |maturity: MaturityClass| {
            self.rows
                .iter()
                .filter(|r| r.published_maturity == maturity)
                .count()
        };
        DoctorRepairContainerMaturityMatrixSummary {
            total_rows: self.rows.len(),
            capability_count: self.capabilities.len(),
            deployment_profile_count: self.deployment_profiles.len(),
            certified_rows: count_published(MaturityClass::Certified),
            provisional_rows: count_published(MaturityClass::Provisional),
            underqualified_rows: count_published(MaturityClass::Underqualified),
            unsupported_rows: count_published(MaturityClass::Unsupported),
            promotable_rows: self.promotable_rows().count(),
            narrowed_rows: self.narrowed_rows().count(),
            withheld_rows: self.withheld_rows().count(),
            current_freshness_rows: self
                .rows
                .iter()
                .filter(|r| r.evidence_freshness.is_current())
                .count(),
            rows_with_blocking_reasons: self
                .rows
                .iter()
                .filter(|r| !r.blocking_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/help, support exports, and release/public-truth packets — render
    /// instead of restating recovery/container status text by hand.
    pub fn export_projection(&self) -> DoctorRepairContainerMaturityMatrixExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|row| DoctorRepairContainerMaturityMatrixExportRow {
                row_id: row.row_id.clone(),
                capability: row.capability.as_str().to_owned(),
                deployment_profile: row.deployment_profile.as_str().to_owned(),
                declared_maturity: row.declared_maturity.as_str().to_owned(),
                published_maturity: row.published_maturity.as_str().to_owned(),
                evidence_freshness: row.evidence_freshness.as_str().to_owned(),
                reversal_class: row.reversal_class.as_str().to_owned(),
                support_parity: row.support_parity.as_str().to_owned(),
                narrowing_action: row.narrowing_action.as_str().to_owned(),
                blocking_reasons: row
                    .blocking_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                scorecard_ref: row.scorecard_ref.clone(),
                latency_corpus_ref: row.latency_corpus_ref.clone(),
                rollback_ref: row.rollback_ref.clone(),
                compatibility_ref: row.compatibility_ref.clone(),
                publication_ready: row.is_promotable(),
                summary: format!(
                    "{} / {}: declared {}, published {} ({}), freshness {}, reversal {}",
                    row.capability.as_str(),
                    row.deployment_profile.as_str(),
                    row.declared_maturity.as_str(),
                    row.published_maturity.as_str(),
                    row.narrowing_action.as_str(),
                    row.evidence_freshness.as_str(),
                    row.reversal_class.as_str()
                ),
            })
            .collect();
        DoctorRepairContainerMaturityMatrixExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            promotable_count: self.promotable_rows().count(),
            narrowed_count: self.narrowed_rows().count(),
            withheld_count: self.withheld_rows().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<DoctorRepairContainerMaturityMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<RecoveryCapability> = self.capabilities.iter().copied().collect();

        let mut seen_rows = BTreeSet::new();
        let mut seen_cells = BTreeSet::new();
        for row in &self.rows {
            if !seen_rows.insert(row.row_id.clone()) {
                violations.push(
                    DoctorRepairContainerMaturityMatrixViolation::DuplicateRowId {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if !seen_cells.insert((row.capability, row.deployment_profile)) {
                violations.push(
                    DoctorRepairContainerMaturityMatrixViolation::DuplicateMatrixCell {
                        capability: row.capability.as_str(),
                        profile: row.deployment_profile.as_str(),
                    },
                );
            }
            if !claimed.contains(&row.capability) {
                violations.push(
                    DoctorRepairContainerMaturityMatrixViolation::UnclaimedCapabilityRow {
                        row_id: row.row_id.clone(),
                        capability: row.capability.as_str(),
                    },
                );
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed (capability, profile) cell must carry its own row, so a
        // cell never inherits trust from an adjacent one.
        for &capability in &self.capabilities {
            for &profile in &self.deployment_profiles {
                if !seen_cells.contains(&(capability, profile)) {
                    violations.push(
                        DoctorRepairContainerMaturityMatrixViolation::MissingMatrixCell {
                            capability: capability.as_str(),
                            profile: profile.as_str(),
                        },
                    );
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(DoctorRepairContainerMaturityMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<DoctorRepairContainerMaturityMatrixViolation>,
    ) {
        if self.schema_version != DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_SCHEMA_VERSION {
            violations.push(
                DoctorRepairContainerMaturityMatrixViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_RECORD_KIND {
            violations.push(
                DoctorRepairContainerMaturityMatrixViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(DoctorRepairContainerMaturityMatrixViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "capabilities",
                self.capabilities == RecoveryCapability::ALL.to_vec(),
            ),
            (
                "deployment_profiles",
                self.deployment_profiles == DeploymentProfile::ALL.to_vec(),
            ),
            (
                "maturity_classes",
                self.maturity_classes == MaturityClass::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "reversal_classes",
                self.reversal_classes == ReversalClass::ALL.to_vec(),
            ),
            (
                "support_parities",
                self.support_parities == SupportParity::ALL.to_vec(),
            ),
            (
                "blocking_reasons",
                self.blocking_reasons == BlockingReason::ALL.to_vec(),
            ),
            (
                "narrowing_actions",
                self.narrowing_actions == NarrowingAction::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    DoctorRepairContainerMaturityMatrixViolation::ClosedVocabularyMismatch {
                        field,
                    },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &MaturityRow,
        violations: &mut Vec<DoctorRepairContainerMaturityMatrixViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("scorecard_ref", &row.scorecard_ref),
            ("latency_corpus_ref", &row.latency_corpus_ref),
            ("rollback_ref", &row.rollback_ref),
            ("compatibility_ref", &row.compatibility_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(DoctorRepairContainerMaturityMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.blocking_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    DoctorRepairContainerMaturityMatrixViolation::DuplicateBlockingReason {
                        row_id: row.row_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // A mutating-repair cell must carry a concrete reversal class, so repair
        // safety can never widen through support folklore.
        if !row.reversal_class_consistent() {
            violations.push(
                DoctorRepairContainerMaturityMatrixViolation::RepairLaneMissingReversalClass {
                    row_id: row.row_id.clone(),
                },
            );
        }

        // The published maturity must equal the gate's recomputed decision, so a
        // row can never publish beyond what its freshness, blocking reasons, and
        // evidence support.
        let effective = row.effective_maturity();
        if row.published_maturity != effective {
            violations.push(
                DoctorRepairContainerMaturityMatrixViolation::OverstatedPublishedMaturity {
                    row_id: row.row_id.clone(),
                    published: row.published_maturity.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The recorded narrowing action must match the published maturity, so
        // release tooling proves underqualified rows narrow automatically.
        let required = row.required_narrowing();
        if row.narrowing_action != required {
            violations.push(
                DoctorRepairContainerMaturityMatrixViolation::NarrowingActionMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.narrowing_action.as_str(),
                    required: required.as_str(),
                },
            );
        }

        // A promotable row must be genuinely clean: current freshness and no
        // blocking reason. This is the non-inheritance guardrail.
        if row.is_promotable()
            && (!row.evidence_freshness.is_current() || !row.blocking_reasons.is_empty())
        {
            violations.push(
                DoctorRepairContainerMaturityMatrixViolation::PromotedRowNotClean {
                    row_id: row.row_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the Doctor/repair/container maturity-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorRepairContainerMaturityMatrixViolation {
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
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A (capability, profile) cell carries more than one row.
    DuplicateMatrixCell {
        /// Capability token.
        capability: &'static str,
        /// Profile token.
        profile: &'static str,
    },
    /// A claimed (capability, profile) cell has no row.
    MissingMatrixCell {
        /// Capability token.
        capability: &'static str,
        /// Profile token.
        profile: &'static str,
    },
    /// A row covers a capability the matrix does not claim.
    UnclaimedCapabilityRow {
        /// Row id.
        row_id: String,
        /// Capability token.
        capability: &'static str,
    },
    /// A row lists a blocking reason more than once.
    DuplicateBlockingReason {
        /// Row id.
        row_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A mutating-repair cell does not carry a concrete reversal class.
    RepairLaneMissingReversalClass {
        /// Row id.
        row_id: String,
    },
    /// A row publishes a maturity beyond what its proof supports.
    OverstatedPublishedMaturity {
        /// Row id.
        row_id: String,
        /// Published-maturity token.
        published: &'static str,
        /// Computed effective-maturity token.
        computed: &'static str,
    },
    /// A row's narrowing action disagrees with its published maturity.
    NarrowingActionMismatch {
        /// Row id.
        row_id: String,
        /// Declared narrowing token.
        declared: &'static str,
        /// Required narrowing token.
        required: &'static str,
    },
    /// A promotable row still carries a blocking reason or non-current freshness.
    PromotedRowNotClean {
        /// Row id.
        row_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for DoctorRepairContainerMaturityMatrixViolation {
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
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateMatrixCell {
                capability,
                profile,
            } => {
                write!(f, "duplicate matrix cell for {capability}/{profile}")
            }
            Self::MissingMatrixCell {
                capability,
                profile,
            } => {
                write!(f, "missing matrix cell for {capability}/{profile}")
            }
            Self::UnclaimedCapabilityRow { row_id, capability } => {
                write!(f, "row {row_id} covers unclaimed capability {capability}")
            }
            Self::DuplicateBlockingReason { row_id, reason } => {
                write!(f, "row {row_id} repeats blocking reason {reason}")
            }
            Self::RepairLaneMissingReversalClass { row_id } => {
                write!(
                    f,
                    "row {row_id} is a guided-repair cell but carries no reversal class"
                )
            }
            Self::OverstatedPublishedMaturity {
                row_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} publishes maturity {published} but the gate computes {computed}"
                )
            }
            Self::NarrowingActionMismatch {
                row_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {row_id} records narrowing {declared} but the gate requires {required}"
                )
            }
            Self::PromotedRowNotClean { row_id } => {
                write!(
                    f,
                    "row {row_id} is promotable but carries a blocking reason or non-current freshness"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for DoctorRepairContainerMaturityMatrixViolation {}

/// Loads the embedded Doctor/repair/container maturity-matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`DoctorRepairContainerMaturityMatrix`].
pub fn current_doctor_repair_container_maturity_matrix(
) -> Result<DoctorRepairContainerMaturityMatrix, serde_json::Error> {
    serde_json::from_str(DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_JSON)
}

#[cfg(test)]
mod tests;
