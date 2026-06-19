//! Typed consumer for the M5 extension-host WIT contract publication packet.
//!
//! Where the M5 public-contract matrix speaks for *which* contract forms the
//! `extension_host_wit_world` family must publish, this packet is the concrete
//! publication of that family's WIT contract form. It binds:
//!
//! - every reserved capability-world WIT package as a versioned
//!   [`WitPackage`] with a [`LifecycleLabel`], reader/writer posture, trust-state
//!   gating posture, permission-scope projection, predecessor/successor links,
//!   and a compatibility note,
//! - the host/guest [`NegotiationFixture`]s that prove the host behaviour for one
//!   real bridge-backed family across the four required
//!   [`NegotiationOutcome`]s (supported, downgraded, deprecated,
//!   unsupported-skew), and
//! - the [`CapabilityDiff`]s between published versions, with a typed
//!   [`ChangeClass`] and [`CompatibilityVerdict`].
//!
//! The packet is checked in at
//! `artifacts/contracts/m5-wit-contract-publication.json` and the four fixtures
//! at `fixtures/contracts/m5-wit-negotiation/<outcome>.json`; all are embedded
//! here via `include_str!`, so this consumer and the Python validator agree on
//! every package, fixture, and diff without a cargo build in CI. The model is
//! metadata-only: every field is a typed state or an opaque repo-relative ref or
//! world identity. It carries no raw component bytes, bridge-shim payloads,
//! signing-key material, or policy-bundle bytes.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Supported packet schema version.
pub const WIT_CONTRACT_PUBLICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const WIT_CONTRACT_PUBLICATION_RECORD_KIND: &str = "m5_wit_contract_publication";

/// Stable record-kind tag for a standalone negotiation fixture.
pub const WIT_NEGOTIATION_FIXTURE_RECORD_KIND: &str = "m5_wit_negotiation_fixture";

/// Repo-relative path to the checked-in packet.
pub const WIT_CONTRACT_PUBLICATION_PATH: &str =
    "artifacts/contracts/m5-wit-contract-publication.json";

/// Embedded checked-in packet JSON.
pub const WIT_CONTRACT_PUBLICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-wit-contract-publication.json"
));

const FIXTURE_SUPPORTED_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/contracts/m5-wit-negotiation/supported.json"
));
const FIXTURE_DOWNGRADED_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/contracts/m5-wit-negotiation/downgraded.json"
));
const FIXTURE_DEPRECATED_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/contracts/m5-wit-negotiation/deprecated.json"
));
const FIXTURE_UNSUPPORTED_SKEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/contracts/m5-wit-negotiation/unsupported_skew.json"
));

/// Loads the checked-in WIT contract publication packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked artifact does not match
/// [`WitContractPublicationPacket`].
pub fn current_wit_contract_publication() -> Result<WitContractPublicationPacket, serde_json::Error>
{
    serde_json::from_str(WIT_CONTRACT_PUBLICATION_JSON)
}

/// Loads the standalone negotiation fixture for an outcome.
///
/// # Errors
///
/// Returns a JSON parse error when the checked fixture does not match
/// [`NegotiationFixture`].
pub fn load_negotiation_fixture(
    outcome: NegotiationOutcome,
) -> Result<NegotiationFixture, serde_json::Error> {
    let raw = match outcome {
        NegotiationOutcome::Supported => FIXTURE_SUPPORTED_JSON,
        NegotiationOutcome::Downgraded => FIXTURE_DOWNGRADED_JSON,
        NegotiationOutcome::Deprecated => FIXTURE_DEPRECATED_JSON,
        NegotiationOutcome::UnsupportedSkew => FIXTURE_UNSUPPORTED_SKEW_JSON,
    };
    serde_json::from_str(raw)
}

/// Publication lifecycle label of a WIT package *version* (distinct from the
/// world slug's registry status).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabel {
    /// Stable, supported version.
    Stable,
    /// Beta version.
    Beta,
    /// Experimental version.
    Experimental,
    /// Deprecated version (superseded; still admitted with a notice).
    Deprecated,
    /// Retired version (no longer admitted).
    Retired,
}

impl LifecycleLabel {
    /// Every label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Beta,
        Self::Experimental,
        Self::Deprecated,
        Self::Retired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
        }
    }
}

/// Registry status of the underlying world slug (ADR-0019 retirement policy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryStatus {
    /// World slug is active.
    Active,
    /// World slug is deprecated.
    Deprecated,
    /// World slug is retired.
    Retired,
}

/// Reader/writer posture (reused from the M5 public-contract vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderWriterPosture {
    /// Read-only.
    ReaderOnly,
    /// Write-only.
    WriterOnly,
    /// Read and write.
    ReadWrite,
    /// Bidirectional interchange.
    BidirectionalInterchange,
}

/// Trust-state gating posture (ADR-0019).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStatePosture {
    /// Admitted (read-only, side-effect-free) under a restricted trust state.
    AdmittedInRestricted,
    /// Blocked entirely under a restricted trust state.
    BlockedInRestricted,
}

/// Per-form publication state (reused from the M5 public-contract vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationState {
    /// Published.
    Published,
    /// Partially published.
    Partial,
    /// Missing.
    Missing,
    /// Not applicable.
    NotApplicable,
}

/// Negotiation-outcome class proven by a fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegotiationOutcome {
    /// Full declared world set admitted.
    Supported,
    /// Some worlds narrowed out (capability narrowing).
    Downgraded,
    /// A deprecated world admitted with a successor notice.
    Deprecated,
    /// A world denied fail-closed because of ABI / vocabulary skew.
    UnsupportedSkew,
}

impl NegotiationOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Supported,
        Self::Downgraded,
        Self::Deprecated,
        Self::UnsupportedSkew,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Downgraded => "downgraded",
            Self::Deprecated => "deprecated",
            Self::UnsupportedSkew => "unsupported_skew",
        }
    }
}

/// Capability-diff change class, in escalating-impact order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeClass {
    /// Additive-minor (adds-only).
    AdditiveMinor,
    /// Deprecation of a version in favour of a successor.
    Deprecation,
    /// Breaking change (item removed or repurposed).
    BreakingMajor,
    /// Retirement of a version.
    Retirement,
}

/// Capability-diff compatibility verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityVerdict {
    /// Backward compatible.
    BackwardCompatible,
    /// Deprecated and superseded.
    DeprecatedSuperseded,
    /// Breaking.
    Breaking,
}

/// Guest action a diff requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuestAction {
    /// No action required.
    None,
    /// Upgrade recommended.
    UpgradeRecommended,
    /// Upgrade required.
    UpgradeRequired,
}

/// Trust state a fixture negotiates under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    /// Trusted workspace.
    Trusted,
    /// Restricted workspace.
    Restricted,
    /// Blocked workspace.
    Blocked,
}

/// ADR-0019 narrowing reason for a declared-but-not-admitted world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// Restricted trust state narrowed the world.
    WorkspaceTrustRestricted,
    /// Admin deny-list named the world.
    AdminPolicyDenyList,
    /// Admin permission floor narrowed a required scope.
    AdminPolicyPermissionFloor,
    /// Admin egress-host narrowing dropped a required host.
    AdminPolicyEgressHostNarrowing,
    /// The world's lifecycle row is degraded.
    CapabilityLifecycleDegraded,
    /// Host and guest disagree on the world-vocabulary version.
    WorldVocabularyVersionUnknown,
    /// Host ABI range does not overlap.
    HostAbiRangeMismatch,
    /// Guest ABI range is unsupported.
    GuestAbiRangeMismatch,
    /// A bridge profile does not carry the world.
    CompatibilityBridgeProfileUnbound,
    /// A declared budget exceeds the host ceiling.
    BudgetDeclarationUnacceptable,
}

/// ADR-0019 reason a declared world is not implemented at the host ABI range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedWorldReason {
    /// The world is retired.
    WorldRetired,
    /// The world is not shipped on this host.
    WorldNotShippedOnHost,
    /// The bridge refused the world.
    BridgeRefused,
    /// Host ABI range mismatch.
    HostAbiRangeMismatch,
    /// Guest ABI range mismatch.
    GuestAbiRangeMismatch,
    /// World-vocabulary version unknown.
    WorldVocabularyVersionUnknown,
}

impl UnsupportedWorldReason {
    /// Returns true for the reasons that represent an ABI / vocabulary skew.
    pub const fn is_skew(self) -> bool {
        matches!(
            self,
            Self::HostAbiRangeMismatch
                | Self::GuestAbiRangeMismatch
                | Self::WorldVocabularyVersionUnknown
        )
    }
}

/// One published versioned WIT package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitPackage {
    /// Package identity (`aureline:<slug>@<semver>`).
    pub package_identity: String,
    /// World slug.
    pub world_slug: String,
    /// World semver.
    pub world_semver: String,
    /// Repo-relative path to the backing `.wit` file.
    pub wit_package_ref: String,
    /// Lifecycle label of this version.
    pub lifecycle_label: LifecycleLabel,
    /// Registry status of the world slug.
    pub registry_status: RegistryStatus,
    /// Reader/writer posture.
    pub reader_writer_posture: ReaderWriterPosture,
    /// Trust-state gating posture.
    pub trust_state_gating_posture: TrustStatePosture,
    /// Permission-scope projection.
    pub permission_scope_projection: Vec<String>,
    /// Supported host families.
    pub supported_host_families: Vec<String>,
    /// ADR-0019 registry row ref.
    pub registry_row_ref: String,
    /// Predecessor package identity, if any.
    pub predecessor_package_ref: Option<String>,
    /// Successor package identity, if any.
    pub successor_package_ref: Option<String>,
    /// Publication state.
    pub publication_state: PublicationState,
    /// Export-safe compatibility note.
    pub compatibility_note: String,
}

/// One capability diff between two published versions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDiff {
    /// Stable diff id.
    pub diff_id: String,
    /// World slug.
    pub world_slug: String,
    /// From package identity.
    pub from_package_ref: String,
    /// To package identity.
    pub to_package_ref: String,
    /// From version.
    pub from_version: String,
    /// To version.
    pub to_version: String,
    /// Change class.
    pub change_class: ChangeClass,
    /// Compatibility verdict.
    pub compatibility_verdict: CompatibilityVerdict,
    /// Guest action required.
    pub guest_action_required: GuestAction,
    /// Added capabilities.
    pub added_capabilities: Vec<String>,
    /// Removed capabilities.
    pub removed_capabilities: Vec<String>,
    /// Changed capabilities.
    pub changed_capabilities: Vec<String>,
    /// Export-safe note.
    pub notes: String,
}

impl CapabilityDiff {
    /// Returns the semantic-invariant violation codes for this diff (empty when
    /// the diff conforms). Mirrors the Python `diff_issues`.
    pub fn issues(&self) -> Vec<&'static str> {
        let mut issues = Vec::new();
        match self.change_class {
            ChangeClass::AdditiveMinor => {
                if !self.removed_capabilities.is_empty() || !self.changed_capabilities.is_empty() {
                    issues.push("additive_minor_removed_or_changed");
                }
                if self.added_capabilities.is_empty() {
                    issues.push("additive_minor_without_additions");
                }
                if self.compatibility_verdict != CompatibilityVerdict::BackwardCompatible {
                    issues.push("additive_minor_not_backward_compatible");
                }
                if self.guest_action_required != GuestAction::None {
                    issues.push("additive_minor_requires_guest_action");
                }
            }
            ChangeClass::Deprecation => {
                if self.compatibility_verdict != CompatibilityVerdict::DeprecatedSuperseded {
                    issues.push("deprecation_wrong_verdict");
                }
                if self.to_package_ref.trim().is_empty() {
                    issues.push("deprecation_without_successor");
                }
            }
            ChangeClass::BreakingMajor | ChangeClass::Retirement => {
                if self.compatibility_verdict != CompatibilityVerdict::Breaking {
                    issues.push("breaking_wrong_verdict");
                }
                if self.guest_action_required != GuestAction::UpgradeRequired {
                    issues.push("breaking_without_required_upgrade");
                }
            }
        }
        issues
    }
}

/// A typed narrowing reason for one declared-but-not-admitted world.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowingReasonEntry {
    /// The narrowed world identity.
    pub world: String,
    /// The narrowing reason.
    pub reason: NarrowingReason,
    /// Human-legible repair affordance.
    pub repair_affordance_label: String,
}

/// A typed unsupported-world decision record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedWorldDecision {
    /// The declared world the host does not implement.
    pub declared_world_ref: String,
    /// The unsupported reason.
    pub unsupported_reason: UnsupportedWorldReason,
    /// A successor world the host carries, if any.
    pub successor_world_ref: Option<String>,
    /// Human-legible repair affordance.
    pub repair_affordance_label: String,
}

/// A typed deprecated-world notice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecatedWorldNotice {
    /// The deprecated world identity (still admitted).
    pub world: String,
    /// The successor world identity.
    pub successor_world_ref: String,
    /// Human-legible repair affordance.
    pub repair_affordance_label: String,
}

/// One host/guest negotiation fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegotiationFixture {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Negotiation outcome.
    pub outcome: NegotiationOutcome,
    /// Human-readable title.
    pub title: String,
    /// Opaque negotiation id (safe to log).
    pub negotiation_id: String,
    /// Ref to the manifest row.
    pub extension_identity_ref: String,
    /// Version declared at negotiation time.
    pub extension_version: String,
    /// Host contract family.
    pub host_contract_family: String,
    /// Host-side ABI range.
    pub host_abi_range: String,
    /// Guest-side ABI range.
    pub guest_abi_range: String,
    /// World-vocabulary version.
    pub world_vocabulary_version: u32,
    /// Trust state.
    pub trust_state: TrustState,
    /// Worlds the manifest declares.
    pub declared_capability_worlds: Vec<String>,
    /// Worlds the host offers for this session.
    pub offered_capability_worlds: Vec<String>,
    /// Worlds admitted.
    pub negotiated_capability_worlds: Vec<String>,
    /// Typed narrowing reasons.
    pub narrowing_reasons: Vec<NarrowingReasonEntry>,
    /// Typed unsupported-world decisions.
    pub unsupported_world_decisions: Vec<UnsupportedWorldDecision>,
    /// Typed deprecated-world notices.
    pub deprecated_world_notices: Vec<DeprecatedWorldNotice>,
    /// Whether the negotiation failed closed.
    pub fail_closed: bool,
    /// Whether guest authority was widened (must be false).
    pub guest_authority_widened: bool,
    /// Reserved audit events emitted.
    pub expected_audit_events: Vec<String>,
    /// Export-safe narrative.
    pub narrative: String,
}

impl NegotiationFixture {
    /// A negotiation fails closed when at least one declared world was narrowed
    /// or denied (rather than widened or silently dropped).
    pub fn derived_fail_closed(&self) -> bool {
        !self.narrowing_reasons.is_empty() || !self.unsupported_world_decisions.is_empty()
    }

    /// Returns the semantic-invariant violation codes for this fixture (empty
    /// when conforming). Mirrors the Python `fixture_issues`.
    pub fn issues(&self) -> Vec<String> {
        let mut issues: Vec<String> = Vec::new();
        let declared: BTreeSet<&str> = self
            .declared_capability_worlds
            .iter()
            .map(String::as_str)
            .collect();
        let offered: BTreeSet<&str> = self
            .offered_capability_worlds
            .iter()
            .map(String::as_str)
            .collect();
        let negotiated: BTreeSet<&str> = self
            .negotiated_capability_worlds
            .iter()
            .map(String::as_str)
            .collect();

        if !offered.is_subset(&declared) {
            issues.push("offered_not_subset_of_declared".to_string());
        }
        if !negotiated.is_subset(&offered) {
            issues.push("negotiated_not_subset_of_offered".to_string());
        }
        if !negotiated.is_subset(&declared) {
            issues.push("negotiated_widens_beyond_declared".to_string());
        }
        if self.guest_authority_widened {
            issues.push("guest_authority_widened".to_string());
        }

        let narrowed: BTreeSet<&str> = self
            .narrowing_reasons
            .iter()
            .map(|e| e.world.as_str())
            .collect();
        let unsupported: BTreeSet<&str> = self
            .unsupported_world_decisions
            .iter()
            .map(|e| e.declared_world_ref.as_str())
            .collect();

        for world in declared.difference(&negotiated) {
            if !narrowed.contains(world) && !unsupported.contains(world) {
                issues.push(format!("silent_drop:{world}"));
            }
        }
        for world in &narrowed {
            if !declared.contains(world) {
                issues.push(format!("narrowing_reason_undeclared:{world}"));
            }
            if negotiated.contains(world) {
                issues.push(format!("narrowed_world_still_negotiated:{world}"));
            }
        }
        for world in &unsupported {
            if !declared.contains(world) {
                issues.push(format!("unsupported_decision_undeclared:{world}"));
            }
            if negotiated.contains(world) {
                issues.push(format!("unsupported_world_still_negotiated:{world}"));
            }
        }

        for entry in &self.narrowing_reasons {
            if entry.repair_affordance_label.trim().is_empty() {
                issues.push(format!("narrowing_reason_missing_repair:{}", entry.world));
            }
        }
        for entry in &self.unsupported_world_decisions {
            if entry.repair_affordance_label.trim().is_empty() {
                issues.push(format!(
                    "unsupported_decision_missing_repair:{}",
                    entry.declared_world_ref
                ));
            }
        }
        for entry in &self.deprecated_world_notices {
            if entry.successor_world_ref.trim().is_empty() {
                issues.push(format!(
                    "deprecated_notice_missing_successor:{}",
                    entry.world
                ));
            }
            if entry.repair_affordance_label.trim().is_empty() {
                issues.push(format!("deprecated_notice_missing_repair:{}", entry.world));
            }
            if !negotiated.contains(entry.world.as_str()) {
                issues.push(format!("deprecated_world_not_admitted:{}", entry.world));
            }
        }

        if self.fail_closed != self.derived_fail_closed() {
            issues.push("fail_closed_mismatch".to_string());
        }

        match self.outcome {
            NegotiationOutcome::Supported => {
                if negotiated != declared {
                    issues.push("supported_did_not_admit_all".to_string());
                }
            }
            NegotiationOutcome::Downgraded => {
                if self.narrowing_reasons.is_empty() {
                    issues.push("downgraded_without_narrowing".to_string());
                }
                if negotiated == declared {
                    issues.push("downgraded_admitted_all".to_string());
                }
            }
            NegotiationOutcome::Deprecated => {
                if self.deprecated_world_notices.is_empty() {
                    issues.push("deprecated_without_notice".to_string());
                }
            }
            NegotiationOutcome::UnsupportedSkew => {
                if self.unsupported_world_decisions.is_empty() {
                    issues.push("unsupported_skew_without_decision".to_string());
                }
                if !self
                    .unsupported_world_decisions
                    .iter()
                    .any(|e| e.unsupported_reason.is_skew())
                {
                    issues.push("unsupported_skew_without_skew_reason".to_string());
                }
            }
        }

        issues
    }
}

/// Derived summary counts for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSummary {
    /// Total published package entries.
    pub package_count: usize,
    /// Entries with `publication_state == published`.
    pub published_package_count: usize,
    /// Entries with `lifecycle_label == deprecated`.
    pub deprecated_package_count: usize,
    /// Number of negotiation fixtures.
    pub negotiation_fixture_count: usize,
    /// Sorted outcomes covered by the fixtures.
    pub outcomes_covered: Vec<NegotiationOutcome>,
    /// Number of fixtures that fail closed.
    pub fail_closed_fixture_count: usize,
    /// Number of capability diffs.
    pub capability_diff_count: usize,
    /// Whether every fixture conforms.
    pub all_fixtures_conform: bool,
    /// Whether every diff conforms.
    pub all_diffs_conform: bool,
}

/// The canonical M5 extension-host WIT contract publication packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitContractPublicationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Publication status.
    pub status: String,
    /// Date the packet is valid as of.
    pub as_of: String,
    /// The M5 public-contract matrix family id this packet publishes.
    pub family_id: String,
    /// Narrative companion page.
    pub overview_page: String,
    /// Evidence record page.
    pub evidence_page: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// M5 public-contract matrix ref.
    pub contract_matrix_ref: String,
    /// M5 public-contract matrix row id.
    pub contract_matrix_row: String,
    /// ADR-0019 capability-world registry ref.
    pub capability_world_registry_ref: String,
    /// Host-negotiation schema ref.
    pub negotiation_schema_ref: String,
    /// ADR ref.
    pub adr_ref: String,
    /// Root WIT package ref.
    pub root_package_ref: String,
    /// WIT publication index ref.
    pub wit_index_ref: String,
    /// Capability-diff report ref.
    pub capability_diff_report_ref: String,
    /// Lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<String>,
    /// Registry-status vocabulary.
    pub registry_statuses: Vec<String>,
    /// Reader/writer posture vocabulary.
    pub reader_writer_postures: Vec<String>,
    /// Trust-state posture vocabulary.
    pub trust_state_postures: Vec<String>,
    /// Publication-state vocabulary.
    pub publication_states: Vec<String>,
    /// Negotiation-outcome vocabulary.
    pub negotiation_outcomes: Vec<String>,
    /// Change-class vocabulary.
    pub change_classes: Vec<String>,
    /// Compatibility-verdict vocabulary.
    pub compatibility_verdicts: Vec<String>,
    /// Guest-action vocabulary.
    pub guest_actions: Vec<String>,
    /// Narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<String>,
    /// Unsupported-world-reason vocabulary.
    pub unsupported_world_reasons: Vec<String>,
    /// Surfaces that consume this packet directly.
    pub consuming_surfaces: Vec<String>,
    /// The published versioned WIT packages.
    pub packages: Vec<WitPackage>,
    /// The capability diffs between published versions.
    pub capability_diffs: Vec<CapabilityDiff>,
    /// The host/guest negotiation fixtures.
    pub negotiation_fixtures: Vec<NegotiationFixture>,
    /// Derived summary.
    pub summary: PublicationSummary,
}

/// A typed structural / semantic violation found by [`WitContractPublicationPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitContractViolation {
    /// `record_kind` is not the expected tag.
    WrongRecordKind,
    /// `schema_version` is not supported.
    WrongSchemaVersion,
    /// Two packages share a package identity.
    DuplicatePackageIdentity(String),
    /// A predecessor / successor ref names an unpublished package.
    DanglingPackageRef {
        /// The package owning the ref.
        package: String,
        /// The unpublished reference.
        reference: String,
    },
    /// A required negotiation outcome is missing.
    MissingOutcome(NegotiationOutcome),
    /// A negotiation outcome appears more than once.
    DuplicateOutcome(NegotiationOutcome),
    /// A fixture failed a semantic invariant.
    FixtureIssue {
        /// The fixture outcome.
        outcome: NegotiationOutcome,
        /// The violation code.
        code: String,
    },
    /// A capability diff failed a semantic invariant.
    DiffIssue {
        /// The diff id.
        diff_id: String,
        /// The violation code.
        code: String,
    },
    /// The recorded summary disagrees with recomputation.
    SummaryMismatch,
    /// The packet has no capability diffs.
    MissingCapabilityDiff,
}

impl WitContractPublicationPacket {
    /// Recomputes the derived summary from the packages, fixtures, and diffs.
    pub fn computed_summary(&self) -> PublicationSummary {
        let mut outcomes: Vec<NegotiationOutcome> = self
            .negotiation_fixtures
            .iter()
            .map(|f| f.outcome)
            .collect();
        outcomes.sort_by_key(|o| o.as_str());
        outcomes.dedup();
        PublicationSummary {
            package_count: self.packages.len(),
            published_package_count: self
                .packages
                .iter()
                .filter(|p| p.publication_state == PublicationState::Published)
                .count(),
            deprecated_package_count: self
                .packages
                .iter()
                .filter(|p| p.lifecycle_label == LifecycleLabel::Deprecated)
                .count(),
            negotiation_fixture_count: self.negotiation_fixtures.len(),
            outcomes_covered: outcomes,
            fail_closed_fixture_count: self
                .negotiation_fixtures
                .iter()
                .filter(|f| f.fail_closed)
                .count(),
            capability_diff_count: self.capability_diffs.len(),
            all_fixtures_conform: self
                .negotiation_fixtures
                .iter()
                .all(|f| f.issues().is_empty()),
            all_diffs_conform: self.capability_diffs.iter().all(|d| d.issues().is_empty()),
        }
    }

    /// Returns every published package for a world slug.
    pub fn packages_for_slug(&self, slug: &str) -> Vec<&WitPackage> {
        self.packages
            .iter()
            .filter(|p| p.world_slug == slug)
            .collect()
    }

    /// Returns the packages published with a `deprecated` lifecycle label.
    pub fn deprecated_packages(&self) -> Vec<&WitPackage> {
        self.packages
            .iter()
            .filter(|p| p.lifecycle_label == LifecycleLabel::Deprecated)
            .collect()
    }

    /// Returns the fixture proving an outcome, if present.
    pub fn fixture_for_outcome(&self, outcome: NegotiationOutcome) -> Option<&NegotiationFixture> {
        self.negotiation_fixtures
            .iter()
            .find(|f| f.outcome == outcome)
    }

    /// Returns every capability diff touching a world slug.
    pub fn capability_diffs_for_slug(&self, slug: &str) -> Vec<&CapabilityDiff> {
        self.capability_diffs
            .iter()
            .filter(|d| d.world_slug == slug)
            .collect()
    }

    /// Validates structural and semantic invariants, returning every violation.
    pub fn validate(&self) -> Vec<WitContractViolation> {
        let mut violations = Vec::new();

        if self.record_kind != WIT_CONTRACT_PUBLICATION_RECORD_KIND {
            violations.push(WitContractViolation::WrongRecordKind);
        }
        if self.schema_version != WIT_CONTRACT_PUBLICATION_SCHEMA_VERSION {
            violations.push(WitContractViolation::WrongSchemaVersion);
        }

        // Unique package identities; non-dangling predecessor / successor refs.
        let identities: BTreeSet<&str> = self
            .packages
            .iter()
            .map(|p| p.package_identity.as_str())
            .collect();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for pkg in &self.packages {
            if !seen.insert(pkg.package_identity.as_str()) {
                violations.push(WitContractViolation::DuplicatePackageIdentity(
                    pkg.package_identity.clone(),
                ));
            }
            for reference in [&pkg.predecessor_package_ref, &pkg.successor_package_ref]
                .into_iter()
                .flatten()
            {
                if !identities.contains(reference.as_str()) {
                    violations.push(WitContractViolation::DanglingPackageRef {
                        package: pkg.package_identity.clone(),
                        reference: reference.clone(),
                    });
                }
            }
        }

        // Every required outcome covered exactly once.
        for outcome in NegotiationOutcome::ALL {
            let count = self
                .negotiation_fixtures
                .iter()
                .filter(|f| f.outcome == outcome)
                .count();
            if count == 0 {
                violations.push(WitContractViolation::MissingOutcome(outcome));
            } else if count > 1 {
                violations.push(WitContractViolation::DuplicateOutcome(outcome));
            }
        }

        // Per-fixture and per-diff semantic invariants.
        for fixture in &self.negotiation_fixtures {
            for code in fixture.issues() {
                violations.push(WitContractViolation::FixtureIssue {
                    outcome: fixture.outcome,
                    code,
                });
            }
        }
        if self.capability_diffs.is_empty() {
            violations.push(WitContractViolation::MissingCapabilityDiff);
        }
        for diff in &self.capability_diffs {
            for code in diff.issues() {
                violations.push(WitContractViolation::DiffIssue {
                    diff_id: diff.diff_id.clone(),
                    code: code.to_string(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(WitContractViolation::SummaryMismatch);
        }

        violations
    }

    /// Projects a metadata-safe support / docs / SDK export row set.
    pub fn support_export_projection(&self) -> WitContractSupportExport {
        WitContractSupportExport {
            packet_id: self.packet_id.clone(),
            family_id: self.family_id.clone(),
            as_of: self.as_of.clone(),
            schema_ref: self.schema_ref.clone(),
            capability_diff_report_ref: self.capability_diff_report_ref.clone(),
            packages: self
                .packages
                .iter()
                .map(|p| WitContractSupportRow {
                    package_identity: p.package_identity.clone(),
                    lifecycle_label: p.lifecycle_label,
                    publication_state: p.publication_state,
                    successor_package_ref: p.successor_package_ref.clone(),
                    compatibility_note: p.compatibility_note.clone(),
                })
                .collect(),
            outcomes_covered: self.summary.outcomes_covered.clone(),
            fail_closed_fixture_count: self.summary.fail_closed_fixture_count,
        }
    }
}

/// Metadata-safe support / docs / SDK export of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitContractSupportExport {
    /// Stable packet id.
    pub packet_id: String,
    /// The published family id.
    pub family_id: String,
    /// Date the packet is valid as of.
    pub as_of: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Capability-diff report ref.
    pub capability_diff_report_ref: String,
    /// One row per published package.
    pub packages: Vec<WitContractSupportRow>,
    /// Outcomes proven by the fixtures.
    pub outcomes_covered: Vec<NegotiationOutcome>,
    /// Number of fixtures that fail closed.
    pub fail_closed_fixture_count: usize,
}

/// One support-export row for a published package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitContractSupportRow {
    /// Package identity.
    pub package_identity: String,
    /// Lifecycle label.
    pub lifecycle_label: LifecycleLabel,
    /// Publication state.
    pub publication_state: PublicationState,
    /// Successor package identity, if any.
    pub successor_package_ref: Option<String>,
    /// Export-safe compatibility note.
    pub compatibility_note: String,
}
