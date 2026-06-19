//! Typed M5 contract-health register: the enforcement layer that makes a
//! missing, stale, downgraded, or incompatible M5 contract package block the
//! same release and claim-publication paths as missing evidence or a stale
//! qualification row.
//!
//! Where the public-contract publication matrix records *whether* each M5
//! artifact family has published its contract forms, and the contract catalog is
//! the consuming index that joins each family to its lifecycle label and sample
//! gallery, this register is the *enforcement* layer on top of both. For every
//! published contract family it binds:
//!
//! - the CI gates ([`Gate`]) that guard each contract-package class — the
//!   schema/spec package, the example corpus, the validator coverage, the
//!   compatibility/migration report, and the release-packet linkage — each
//!   evaluated to a [`GateOutcome`] and a [`FreshnessState`] reused from the
//!   publication-matrix and release-candidate vocabularies,
//! - the exact release-artifact-graph linkage ([`GraphLinkage`]) — the release
//!   packet, the artifact-graph node, the build identity, and the contract
//!   [`PackageIdentity`] (its canonical schema/spec id and package version) — so
//!   one build identity proves the contract set the candidate shipped, and
//! - the shiproom [`Blocker`] decision those signals produce.
//!
//! Each [`ContractHealthRow::lifecycle_label`] equals the publication matrix's
//! effective published label, so a narrowed family narrows here automatically and
//! the register never advertises a greener label than the matrix. A
//! release-blocking family with a failing required contract gate sets the
//! register's promotion decision to [`BlockerDecision::Hold`], and the
//! mirror/offline publishability of a family follows the same gate outputs so
//! sovereign and self-hosted trains are not second-class citizens.
//!
//! The register is checked in at `artifacts/release/m5-contract-health.json` and
//! embedded here, so this typed consumer and the CI validator agree on every
//! family and gate without a cargo build in CI. The model is metadata-plus-state
//! only: every field is a typed state, an opaque repo-relative ref or URI, or a
//! copy/export-safe summary. It carries no credential bodies or raw provider
//! payloads.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported register schema version.
pub const M5_CONTRACT_HEALTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_CONTRACT_HEALTH_RECORD_KIND: &str = "m5_contract_health_register";

/// Stable register identifier.
pub const M5_CONTRACT_HEALTH_REGISTER_ID: &str = "m5_contract_health:v1";

/// Repo-relative path to the checked-in register.
pub const M5_CONTRACT_HEALTH_PATH: &str = "artifacts/release/m5-contract-health.json";

/// Embedded checked-in register JSON.
pub const M5_CONTRACT_HEALTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-contract-health.json"
));

/// The lifecycle/stability label a family publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabel {
    /// Long-term-stable.
    Lts,
    /// Stable.
    Stable,
    /// Beta.
    Beta,
    /// Preview.
    Preview,
    /// Withdrawn.
    Withdrawn,
}

impl LifecycleLabel {
    /// Every label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];

    /// True when the label publishes at or above the stable cutline.
    pub fn is_at_or_above_cutline(self) -> bool {
        matches!(self, Self::Lts | Self::Stable)
    }
}

/// The CI gate that guards one contract-package class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateKind {
    /// The schema/spec contract package (JSON Schema, WIT world, or OpenAPI spec).
    SchemaSpecPackage,
    /// The example payload corpus.
    ExampleCorpus,
    /// The validator-suite coverage.
    ValidatorCoverage,
    /// The compatibility / migration report.
    CompatibilityReport,
    /// The release-packet linkage to the artifact graph and build identity.
    ReleasePacketLinkage,
}

impl GateKind {
    /// Every gate kind, in evaluation order.
    pub const ALL: [Self; 5] = [
        Self::SchemaSpecPackage,
        Self::ExampleCorpus,
        Self::ValidatorCoverage,
        Self::CompatibilityReport,
        Self::ReleasePacketLinkage,
    ];
}

/// The outcome of evaluating one gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateOutcome {
    /// The guarded contract artifacts are published.
    Pass,
    /// A guarded artifact is partial; the family narrows.
    Downgrade,
    /// A guarded artifact is missing; the family is held.
    Fail,
}

impl GateOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 3] = [Self::Pass, Self::Downgrade, Self::Fail];
}

/// Evidence freshness, reused from the release-candidate matrix vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Current.
    Current,
    /// Due for refresh.
    DueForRefresh,
    /// Breached its freshness SLO.
    Breached,
    /// Missing.
    Missing,
}

impl FreshnessState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::DueForRefresh,
        Self::Breached,
        Self::Missing,
    ];
}

/// A family's overall contract-health state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthState {
    /// Every required gate passes.
    Healthy,
    /// A required gate downgraded, or the matrix narrowed the family.
    Narrowed,
    /// A release-blocking family has a failing required gate.
    Blocked,
}

impl HealthState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Healthy, Self::Narrowed, Self::Blocked];
}

/// The shiproom blocker decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerDecision {
    /// Release-clear.
    Clear,
    /// Promotion held.
    Hold,
}

impl BlockerDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 2] = [Self::Clear, Self::Hold];
}

/// A family's mirror/offline publishability, following the gate outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorParityState {
    /// Published to the mirror and current.
    Current,
    /// Published but stale.
    Stale,
    /// Not publishable to the mirror (a blocking gate failed).
    Unpublished,
    /// Local-only; mirror parity does not apply.
    NotApplicable,
}

impl MirrorParityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::Stale,
        Self::Unpublished,
        Self::NotApplicable,
    ];
}

/// The canonical contract-package identity kind for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageKind {
    /// A JSON Schema package.
    JsonSchema,
    /// An OpenAPI specification.
    OpenapiSpec,
    /// A WIT world package.
    WitWorld,
}

impl PackageKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::JsonSchema, Self::OpenapiSpec, Self::WitWorld];
}

/// The build identity that one candidate's contract set is proved against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildIdentity {
    /// Repo-relative ref to the build-identity artifact (resolved at review time).
    pub build_identity_ref: String,
    /// Symbolic release-candidate identifier.
    pub release_candidate_ref: String,
    /// Symbolic artifact-graph identifier.
    pub artifact_graph_ref: String,
    /// The pinned toolchain channel.
    pub toolchain_channel: String,
    /// Human-readable note.
    pub note: String,
}

/// The launch cutline, ingested verbatim from the publication matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchCutline {
    /// The cutline level.
    pub cutline_level: LifecycleLabel,
    /// Levels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Levels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Human-readable description.
    pub description: String,
}

/// One published CI gate descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateDescriptor {
    /// Stable gate id.
    pub gate_id: String,
    /// The gate kind.
    pub gate_kind: GateKind,
    /// Human-readable title.
    pub title: String,
    /// Human-readable description.
    pub description: String,
    /// The publication-requirement artifact kinds this gate guards.
    pub guards_artifact_kinds: Vec<String>,
    /// The gap reasons this gate raises.
    pub gap_reasons: Vec<String>,
    /// The remediation actions this gate recommends.
    pub remediation_actions: Vec<String>,
    /// The outcome a failing gate yields.
    pub fail_outcome: GateOutcome,
    /// Whether a failing gate holds promotion on a release-blocking family.
    pub blocks_when_release_blocking: bool,
    /// Repo-relative ref to the per-gate descriptor file under `ci/contracts/`.
    pub descriptor_ref: String,
}

/// The contract-package identity with its resolvable version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageIdentity {
    /// The contract-identity kind.
    pub identity_kind: PackageKind,
    /// The stable schema or spec identifier (a `$id` URI or a stable catalog id).
    pub schema_or_spec_id: String,
    /// Repo-relative ref to the schema or spec document.
    pub schema_or_spec_ref: String,
    /// The contract-package kind.
    pub package_kind: PackageKind,
    /// The resolvable package version.
    pub package_version: u32,
    /// The in-band version field the package carries.
    pub in_band_version_field: String,
}

/// The exact release-artifact-graph linkage for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphLinkage {
    /// Symbolic release-candidate identifier.
    pub release_candidate_ref: String,
    /// The release-packet entry this family rides.
    pub release_packet_ref: String,
    /// Repo-relative ref to the build-identity artifact.
    pub build_identity_ref: String,
    /// Symbolic artifact-graph node identifier.
    pub artifact_graph_node_ref: String,
    /// The family's mirror/offline publishability.
    pub mirror_parity: MirrorParityState,
    /// True when the contract set is offline-verifiable for this family.
    pub offline_verifiable: bool,
}

/// One evaluated CI gate for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gate {
    /// Stable gate id.
    pub gate_id: String,
    /// The gate kind.
    pub gate_kind: GateKind,
    /// Whether the gate is required for this family.
    pub required: bool,
    /// The evidence freshness.
    pub freshness: FreshnessState,
    /// The evaluated outcome.
    pub outcome: GateOutcome,
    /// Evidence refs the gate read.
    pub evidence_refs: Vec<String>,
    /// Human-readable detail.
    pub detail: String,
}

impl Gate {
    /// True when this gate fails and is required.
    pub fn is_required_failure(&self) -> bool {
        self.required && self.outcome == GateOutcome::Fail
    }

    /// True when this gate downgrades and is required.
    pub fn is_required_downgrade(&self) -> bool {
        self.required && self.outcome == GateOutcome::Downgrade
    }
}

/// The shiproom blocker decision for one family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blocker {
    /// The blocker decision.
    pub decision: BlockerDecision,
    /// The gate ids that are failing and required.
    pub blocking_gate_ids: Vec<String>,
    /// Whether the family needs a retest.
    pub retest_needed: bool,
    /// Active stale/gap reasons.
    pub stale_reasons: Vec<String>,
    /// Human-readable summary.
    pub summary: String,
}

/// One contract-health row: a family and its gates, linkage, and blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractHealthRow {
    /// Stable family id (links to the contract catalog and the matrix).
    pub family_id: String,
    /// Human-readable title.
    pub title: String,
    /// The contract form (matrix lexicon).
    pub contract_form: String,
    /// The owning crate or lane.
    pub owning_package: String,
    /// The lifecycle label the family is put forward at.
    pub claim_label: LifecycleLabel,
    /// The lifecycle label the matrix publishes after narrowing.
    pub published_label: LifecycleLabel,
    /// The lifecycle label this register publishes (equals the matrix's).
    pub lifecycle_label: LifecycleLabel,
    /// Whether the family narrows below its claim label.
    pub narrowed: bool,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The contract-package identity.
    pub package_identity: PackageIdentity,
    /// The release-artifact-graph linkage.
    pub graph_linkage: GraphLinkage,
    /// The evaluated gates (one per gate kind, in order).
    pub gates: Vec<Gate>,
    /// Active gap reasons (matrix lexicon).
    pub active_gap_reasons: Vec<String>,
    /// Remediation actions this family needs.
    pub remediation_actions: Vec<String>,
    /// The overall health state.
    pub health_state: HealthState,
    /// Ref to the contract-catalog entry.
    pub catalog_entry_ref: String,
    /// Ref to the publication-matrix row.
    pub matrix_row_ref: String,
    /// The shiproom blocker decision.
    pub blocker: Blocker,
}

impl ContractHealthRow {
    /// True when the family publishes at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        self.lifecycle_label.is_at_or_above_cutline()
    }

    /// Recomputes the health state from the gates, narrowing, and blocking flags.
    ///
    /// A release-blocking family with a failing required gate is `Blocked`; a
    /// family with a narrowed label or a downgraded/failing required gate is
    /// `Narrowed`; otherwise it is `Healthy`.
    pub fn computed_health(&self) -> HealthState {
        let any_required_fail = self.gates.iter().any(Gate::is_required_failure);
        let any_required_downgrade = self.gates.iter().any(Gate::is_required_downgrade);
        if self.release_blocking && any_required_fail {
            HealthState::Blocked
        } else if self.narrowed || any_required_downgrade || any_required_fail {
            HealthState::Narrowed
        } else {
            HealthState::Healthy
        }
    }

    /// The blocker decision implied by the computed health state.
    pub fn computed_decision(&self) -> BlockerDecision {
        if self.computed_health() == HealthState::Blocked {
            BlockerDecision::Hold
        } else {
            BlockerDecision::Clear
        }
    }
}

/// Top-level shiproom blocker summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blockers {
    /// The promotion decision.
    pub decision: BlockerDecision,
    /// Families holding promotion.
    pub blocking_family_ids: Vec<String>,
    /// Gate kinds failing on blocking families.
    pub blocking_gate_kinds: Vec<GateKind>,
    /// Families needing a retest.
    pub retest_needed_family_ids: Vec<String>,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Summary counts over the family set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ContractHealthSummary {
    /// Total families.
    pub total_families: usize,
    /// Release-blocking families.
    pub release_blocking_families: usize,
    /// Healthy families.
    pub healthy_families: usize,
    /// Narrowed families.
    pub narrowed_families: usize,
    /// Blocked families.
    pub blocked_families: usize,
    /// Families whose blocker decision is hold.
    pub families_held: usize,
    /// Families needing a retest.
    pub families_retest_needed: usize,
    /// Families that remain mirror/offline publishable.
    pub mirror_publishable_families: usize,
    /// Total gate evaluations.
    pub total_gates_evaluated: usize,
    /// Passing gate evaluations.
    pub gates_passing: usize,
    /// Downgrading gate evaluations.
    pub gates_downgrading: usize,
    /// Failing gate evaluations.
    pub gates_failing: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContractHealthViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

impl std::fmt::Display for M5ContractHealthViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.check_id, self.detail)
    }
}

/// One support/shiproom export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContractHealthExportRow {
    /// Stable family id.
    pub family_id: String,
    /// The lifecycle label the family publishes.
    pub lifecycle_label: LifecycleLabel,
    /// The overall health state.
    pub health_state: HealthState,
    /// The blocker decision.
    pub decision: BlockerDecision,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The contract-package kind.
    pub package_kind: PackageKind,
    /// The resolvable package version.
    pub package_version: u32,
    /// The mirror/offline publishability.
    pub mirror_parity: MirrorParityState,
}

/// Export projection for shiproom, support, and partner-review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContractHealthExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The top-level promotion decision.
    pub decision: BlockerDecision,
    /// Export rows.
    pub rows: Vec<M5ContractHealthExportRow>,
}

/// The typed M5 contract-health register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ContractHealthRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Narrative companion document.
    pub overview_page: String,
    /// Evidence/proof packet.
    pub evidence_page: String,
    /// Help-center page.
    pub help_page: String,
    /// Shiproom blocker dashboard.
    pub shiproom_dashboard_page: String,
    /// Ref to the CI gate manifest.
    pub gate_manifest_ref: String,
    /// Ref to the contract catalog.
    pub contract_catalog_ref: String,
    /// Ref to the publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// The build identity the contract set is proved against.
    pub build_identity: BuildIdentity,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed gate-kind vocabulary.
    pub gate_kinds: Vec<GateKind>,
    /// Closed gate-outcome vocabulary.
    pub gate_outcomes: Vec<GateOutcome>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessState>,
    /// Closed health-state vocabulary.
    pub health_states: Vec<HealthState>,
    /// Closed blocker-decision vocabulary.
    pub blocker_decisions: Vec<BlockerDecision>,
    /// Closed mirror-parity vocabulary.
    pub mirror_parity_states: Vec<MirrorParityState>,
    /// Closed gap-reason vocabulary (matrix lexicon).
    pub gap_reasons: Vec<String>,
    /// Closed remediation-action vocabulary (matrix lexicon).
    pub remediation_actions: Vec<String>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The published CI gate catalog.
    pub gate_catalog: Vec<GateDescriptor>,
    /// The contract-health rows.
    pub rows: Vec<ContractHealthRow>,
    /// The top-level shiproom blocker summary.
    pub blockers: Blockers,
    /// Summary counts.
    pub summary: M5ContractHealthSummary,
}

impl M5ContractHealthRegister {
    /// Returns the row registered for `family_id`.
    pub fn row(&self, family_id: &str) -> Option<&ContractHealthRow> {
        self.rows.iter().find(|r| r.family_id == family_id)
    }

    /// Resolves the lifecycle label, health state, and blocker decision for a
    /// family. This is the lookup shiproom, support export, and the in-product
    /// inspect surface share.
    pub fn resolve_health(
        &self,
        family_id: &str,
    ) -> Option<(LifecycleLabel, HealthState, BlockerDecision)> {
        self.row(family_id)
            .map(|r| (r.lifecycle_label, r.health_state, r.blocker.decision))
    }

    /// Families holding promotion.
    pub fn blocked_rows(&self) -> Vec<&ContractHealthRow> {
        self.rows
            .iter()
            .filter(|r| r.health_state == HealthState::Blocked)
            .collect()
    }

    /// True when the contract set holds promotion.
    pub fn holds_promotion(&self) -> bool {
        self.blockers.decision == BlockerDecision::Hold
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5ContractHealthSummary {
        let count = |f: &dyn Fn(&ContractHealthRow) -> bool| {
            self.rows.iter().filter(|r| f(r)).count()
        };
        M5ContractHealthSummary {
            total_families: self.rows.len(),
            release_blocking_families: count(&|r| r.release_blocking),
            healthy_families: count(&|r| r.health_state == HealthState::Healthy),
            narrowed_families: count(&|r| r.health_state == HealthState::Narrowed),
            blocked_families: count(&|r| r.health_state == HealthState::Blocked),
            families_held: count(&|r| r.blocker.decision == BlockerDecision::Hold),
            families_retest_needed: count(&|r| r.blocker.retest_needed),
            mirror_publishable_families: count(&|r| r.graph_linkage.offline_verifiable),
            total_gates_evaluated: self.rows.iter().map(|r| r.gates.len()).sum(),
            gates_passing: self
                .rows
                .iter()
                .flat_map(|r| &r.gates)
                .filter(|g| g.outcome == GateOutcome::Pass)
                .count(),
            gates_downgrading: self
                .rows
                .iter()
                .flat_map(|r| &r.gates)
                .filter(|g| g.outcome == GateOutcome::Downgrade)
                .count(),
            gates_failing: self
                .rows
                .iter()
                .flat_map(|r| &r.gates)
                .filter(|g| g.outcome == GateOutcome::Fail)
                .count(),
        }
    }

    /// Produces an export/inspect-safe projection downstream surfaces render
    /// instead of cloning register text.
    pub fn support_export_projection(&self) -> M5ContractHealthExportProjection {
        M5ContractHealthExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            decision: self.blockers.decision,
            rows: self
                .rows
                .iter()
                .map(|r| M5ContractHealthExportRow {
                    family_id: r.family_id.clone(),
                    lifecycle_label: r.lifecycle_label,
                    health_state: r.health_state,
                    decision: r.blocker.decision,
                    release_blocking: r.release_blocking,
                    package_kind: r.package_identity.package_kind,
                    package_version: r.package_identity.package_version,
                    mirror_parity: r.graph_linkage.mirror_parity,
                })
                .collect(),
        }
    }

    /// Validates the register's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in register
    /// returns no violations; each negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5ContractHealthViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5ContractHealthViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_CONTRACT_HEALTH_SCHEMA_VERSION {
            push(
                "register.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_CONTRACT_HEALTH_RECORD_KIND {
            push(
                "register.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.register_id != M5_CONTRACT_HEALTH_REGISTER_ID {
            push(
                "register.register_id",
                format!("unexpected register_id {}", self.register_id),
            );
        }

        if self.lifecycle_labels != LifecycleLabel::ALL {
            push("vocab.lifecycle_labels", "lifecycle_labels off the canonical list".into());
        }
        if self.gate_kinds != GateKind::ALL {
            push("vocab.gate_kinds", "gate_kinds off the canonical list".into());
        }
        if self.gate_outcomes != GateOutcome::ALL {
            push("vocab.gate_outcomes", "gate_outcomes off the canonical list".into());
        }
        if self.freshness_states != FreshnessState::ALL {
            push("vocab.freshness_states", "freshness_states off the canonical list".into());
        }
        if self.health_states != HealthState::ALL {
            push("vocab.health_states", "health_states off the canonical list".into());
        }
        if self.blocker_decisions != BlockerDecision::ALL {
            push("vocab.blocker_decisions", "blocker_decisions off the canonical list".into());
        }
        if self.mirror_parity_states != MirrorParityState::ALL {
            push("vocab.mirror_parity_states", "mirror_parity_states off the canonical list".into());
        }

        let catalog_kinds: Vec<GateKind> =
            self.gate_catalog.iter().map(|g| g.gate_kind).collect();
        if catalog_kinds != GateKind::ALL.to_vec() {
            push("gate_catalog.kinds", "gate_catalog kinds off the canonical list".into());
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.family_id.as_str()) {
                push(
                    "rows.duplicate_family_id",
                    format!("duplicate family_id {}", row.family_id),
                );
            }

            let gate_kinds: Vec<GateKind> = row.gates.iter().map(|g| g.gate_kind).collect();
            if gate_kinds != GateKind::ALL.to_vec() {
                push(
                    "rows.gate_coverage",
                    format!("{}: gates must be exactly the gate-kind set", row.family_id),
                );
            }

            let expected_health = row.computed_health();
            if row.health_state != expected_health {
                push(
                    "rows.health_state",
                    format!(
                        "{}: health_state disagrees with the gates",
                        row.family_id
                    ),
                );
            }

            let expected_decision = row.computed_decision();
            if row.blocker.decision != expected_decision {
                push(
                    "rows.blocker_decision",
                    format!(
                        "{}: blocker decision disagrees with the health state",
                        row.family_id
                    ),
                );
            }

            let parity = row.graph_linkage.mirror_parity;
            let offline = row.graph_linkage.offline_verifiable;
            if offline
                != matches!(parity, MirrorParityState::Current | MirrorParityState::NotApplicable)
            {
                push(
                    "rows.mirror_parity",
                    format!(
                        "{}: offline_verifiable disagrees with mirror_parity",
                        row.family_id
                    ),
                );
            }
            if expected_health == HealthState::Blocked && offline {
                push(
                    "rows.mirror_parity",
                    format!(
                        "{}: a blocked family must not be offline_verifiable",
                        row.family_id
                    ),
                );
            }
        }

        // Top-level blocker decision recomputed from the rows.
        let blocked_ids: Vec<String> = self
            .rows
            .iter()
            .filter(|r| r.computed_health() == HealthState::Blocked)
            .map(|r| r.family_id.clone())
            .collect();
        if self.blockers.blocking_family_ids != blocked_ids {
            push(
                "blockers.block",
                "blocking_family_ids disagree with the blocked rows".into(),
            );
        }
        let expected_top = if blocked_ids.is_empty() {
            BlockerDecision::Clear
        } else {
            BlockerDecision::Hold
        };
        if self.blockers.decision != expected_top {
            push(
                "blockers.decision",
                "top-level decision disagrees with the blocked rows".into(),
            );
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the rows".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in register into the typed model.
pub fn current_m5_contract_health_register(
) -> Result<M5ContractHealthRegister, serde_json::Error> {
    serde_json::from_str(M5_CONTRACT_HEALTH_JSON)
}

#[cfg(test)]
mod tests;
