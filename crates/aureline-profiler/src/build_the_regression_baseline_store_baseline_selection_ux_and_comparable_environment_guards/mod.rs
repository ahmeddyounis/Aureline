//! Regression baseline store, baseline selection UX, and comparable-environment guards.
//!
//! This module materializes the typed records that keep profiler regression surfaces
//! honest about what baseline is being compared, how comparable the environments really
//! are, and what guards narrow the claim when comparability is weak. The records and
//! closed vocabularies here mirror the boundary schema at
//! `/schemas/perf/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and environment-identity
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`BaselineStoreRow`] record that binds baseline identity, build identity,
//!   environment fingerprint, capture mode, storage location, freshness state, and
//!   provenance so users always know what baseline they are comparing against;
//! - the [`BaselineSelectionUxRow`] record that carries selection kind, comparison
//!   basis label, baseline ref, current environment fingerprint ref, environment match
//!   state, and honest mismatch or stale warnings so the selection surface never
//!   silently compares incomparable evidence;
//! - the [`ComparableEnvironmentGuardRow`] record that carries the exact guard criteria
//!   used to decide whether two environments are comparable, including build identity,
//!   architecture, OS version, runtime version, capture mode, mapping quality, and
//!   freshness policy so comparison claims narrow automatically when guards fail;
//! - the [`EnvironmentFingerprintRow`] record that carries the normalized environment
//!   identity used for comparability checks;
//! - the [`RegressionBaselineQualificationPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every regression-baseline qualification packet carried
/// by this module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const REGRESSION_BASELINE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RegressionBaselineQualificationPacket`].
pub const REGRESSION_BASELINE_QUALIFICATION_RECORD_KIND: &str =
    "build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards";

/// Repo-relative path to the checked-in regression-baseline qualification packet JSON.
pub const REGRESSION_BASELINE_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.json";

/// Embedded checked-in qualification packet JSON.
pub const REGRESSION_BASELINE_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.json"
));

/// Qualification label shown on promoted regression-baseline surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionBaselineQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl RegressionBaselineQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Regression-baseline surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionBaselineSurfaceKind {
    /// Baseline store surface (list, import, or manage baselines).
    BaselineStore,
    /// Baseline selection UX surface (picker or list).
    BaselineSelectionUx,
    /// Comparison report surface showing baseline versus current.
    ComparisonReport,
    /// Environment-guard inspector surface.
    EnvironmentGuardInspector,
    /// Export review surface for regression evidence.
    ExportReview,
    /// Support export surface for regression evidence.
    SupportExport,
}

/// Freshness state of a stored baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineFreshness {
    /// Baseline is current and within freshness policy.
    Current,
    /// Baseline is older than freshness policy but still present.
    Stale,
    /// Baseline has passed retention expiry.
    Expired,
    /// Baseline is missing or was deleted.
    Missing,
    /// Baseline was imported and may have different provenance.
    Imported,
    /// Baseline integrity is unverified.
    Unverified,
}

impl BaselineFreshness {
    /// Returns true when the baseline is considered usable for comparison.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Current | Self::Stale | Self::Imported)
    }
}

/// Kind of baseline selection UX element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineSelectionKind {
    /// Picker dropdown for choosing a baseline.
    Picker,
    /// Scrollable list of available baselines.
    List,
    /// Recently-used baseline quick-select.
    Recent,
    /// Pinned or starred baseline.
    Pinned,
}

/// Environment match state shown on the selection UX.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentMatchState {
    /// Environments are fully comparable.
    Comparable,
    /// Environments are mostly comparable with minor differences.
    Partial,
    /// Environments differ in a way that may affect comparison validity.
    Mismatch,
    /// Baseline environment is unknown or unverified.
    Unknown,
    /// Baseline is too old to be confidently comparable.
    Stale,
}

impl EnvironmentMatchState {
    /// Returns true when the state allows a comparison to proceed with a warning.
    pub const fn allows_comparison_with_warning(self) -> bool {
        matches!(self, Self::Comparable | Self::Partial | Self::Stale)
    }

    /// Returns true when the state should show a mismatch or stale warning.
    pub const fn shows_warning(self) -> bool {
        matches!(self, Self::Partial | Self::Mismatch | Self::Stale)
    }
}

/// One baseline-store row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineStoreRow {
    /// Stable baseline row id.
    pub baseline_id: String,
    /// Human-readable title.
    pub title: String,
    /// Exact build identity ref at the time the baseline was captured.
    pub exact_build_identity_ref: String,
    /// Environment fingerprint ref for comparability checks.
    pub environment_fingerprint_ref: String,
    /// Capture-mode descriptor ref.
    pub capture_mode_ref: String,
    /// Storage-location truth ref.
    pub storage_location_ref: String,
    /// Baseline freshness state.
    pub freshness: BaselineFreshness,
    /// Provenance chain refs.
    #[serde(default)]
    pub provenance_refs: Vec<String>,
    /// True when the baseline is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the baseline shows its freshness state honestly.
    pub shows_freshness_state: bool,
    /// True when the baseline shows its build identity.
    pub shows_build_identity: bool,
}

/// One baseline-selection UX row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineSelectionUxRow {
    /// Stable selection row id.
    pub selection_id: String,
    /// Human-readable title.
    pub title: String,
    /// Selection kind.
    pub selection_kind: BaselineSelectionKind,
    /// Comparison basis label shown to the user.
    pub comparison_basis_label: String,
    /// Baseline ref selected or offered.
    pub baseline_ref: String,
    /// Current environment fingerprint ref.
    pub current_environment_fingerprint_ref: String,
    /// Environment match state.
    pub environment_match_state: EnvironmentMatchState,
    /// True when the selection shows a mismatch warning.
    pub shows_mismatch_warning: bool,
    /// True when the selection shows a stale baseline warning.
    pub shows_stale_warning: bool,
    /// True when the selection is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One comparable-environment guard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparableEnvironmentGuardRow {
    /// Stable guard row id.
    pub guard_id: String,
    /// Human-readable title.
    pub title: String,
    /// True when exact build identity must match.
    pub exact_build_identity_required: bool,
    /// True when architecture must match.
    pub architecture_required: bool,
    /// True when OS and version must match.
    pub os_version_required: bool,
    /// True when runtime version must match.
    pub runtime_version_required: bool,
    /// True when capture mode must be compatible.
    pub capture_mode_compatible_required: bool,
    /// True when mapping quality must be compatible.
    pub mapping_quality_compatible_required: bool,
    /// Freshness policy label (e.g. "seven_days", "thirty_days").
    pub freshness_policy: String,
    /// True when the guard row is enforced in the promoted build.
    pub enforced_in_build: bool,
    /// True when the guard shows which criteria passed or failed.
    pub shows_criteria_breakdown: bool,
    /// True when the guard narrows the comparison claim on mismatch.
    pub narrows_claim_on_mismatch: bool,
}

/// One environment fingerprint row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentFingerprintRow {
    /// Stable fingerprint row id.
    pub fingerprint_id: String,
    /// Human-readable title.
    pub title: String,
    /// Architecture label.
    pub architecture: String,
    /// OS name and version.
    pub os_version: String,
    /// Runtime version.
    pub runtime_version: String,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Extra normalized traits for comparability.
    #[serde(default)]
    pub extra_traits: Vec<String>,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionBaselineQualificationProof {
    /// Packet id.
    pub packet_id: String,
    /// Packet ref path.
    pub packet_ref: String,
    /// Proof index ref path.
    pub proof_index_ref: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionBaselineQualificationSummary {
    /// Total number of baseline-store rows.
    pub baseline_store_count: usize,
    /// Total number of baseline-selection UX rows.
    pub baseline_selection_ux_count: usize,
    /// Total number of comparable-environment guard rows.
    pub comparable_environment_guard_count: usize,
    /// Total number of environment fingerprint rows.
    pub environment_fingerprint_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of baselines that are usable for comparison.
    pub usable_baseline_count: usize,
    /// Number of guard rows that narrow claims on mismatch.
    pub narrowing_guard_count: usize,
}

/// Guard set for a regression-baseline surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionBaselineSurfaceGuardSet {
    /// Baseline identity is visible.
    pub baseline_identity_visible: bool,
    /// Build identity is visible.
    pub build_identity_visible: bool,
    /// Environment fingerprint is visible.
    pub environment_fingerprint_visible: bool,
    /// Capture mode is visible.
    pub capture_mode_visible: bool,
    /// Storage location is visible.
    pub storage_location_visible: bool,
    /// Freshness state is visible.
    pub freshness_state_visible: bool,
    /// Comparison basis label is visible.
    pub comparison_basis_visible: bool,
    /// Environment match state is visible.
    pub environment_match_visible: bool,
    /// Mismatch or stale warning is visible when applicable.
    pub mismatch_warning_visible: bool,
    /// Guard criteria breakdown is visible.
    pub guard_criteria_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionBaselineSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: RegressionBaselineSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: RegressionBaselineQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: RegressionBaselineQualificationProof,
    /// Guard set.
    pub guards: RegressionBaselineSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in regression-baseline qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionBaselineQualificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// As-of timestamp.
    pub as_of: String,
    /// Release doc ref.
    pub release_doc_ref: String,
    /// Help doc ref.
    pub help_doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Surface qualification rows.
    pub surfaces: Vec<RegressionBaselineSurfaceQualificationRow>,
    /// Baseline store rows.
    pub baseline_stores: Vec<BaselineStoreRow>,
    /// Baseline selection UX rows.
    pub baseline_selection_uxs: Vec<BaselineSelectionUxRow>,
    /// Comparable environment guard rows.
    pub comparable_environment_guards: Vec<ComparableEnvironmentGuardRow>,
    /// Environment fingerprint rows.
    pub environment_fingerprints: Vec<EnvironmentFingerprintRow>,
    /// Summary.
    pub summary: RegressionBaselineQualificationSummary,
}

impl RegressionBaselineQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> RegressionBaselineQualificationSummary {
        let stable_count = self
            .surfaces
            .iter()
            .filter(|s| s.claim_label.is_stable())
            .count();
        let below_stable_count = self.surfaces.len().saturating_sub(stable_count);
        let all_below_stable_have_disclosure = self
            .surfaces
            .iter()
            .filter(|s| !s.claim_label.is_stable())
            .all(|s| !s.rationale.is_empty());
        let usable_baseline_count = self
            .baseline_stores
            .iter()
            .filter(|b| b.freshness.is_usable())
            .count();
        let narrowing_guard_count = self
            .comparable_environment_guards
            .iter()
            .filter(|g| g.narrows_claim_on_mismatch)
            .count();

        RegressionBaselineQualificationSummary {
            baseline_store_count: self.baseline_stores.len(),
            baseline_selection_ux_count: self.baseline_selection_uxs.len(),
            comparable_environment_guard_count: self.comparable_environment_guards.len(),
            environment_fingerprint_count: self.environment_fingerprints.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            usable_baseline_count,
            narrowing_guard_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<RegressionBaselineQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REGRESSION_BASELINE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(RegressionBaselineQualificationViolation::SchemaVersion {
                expected: REGRESSION_BASELINE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != REGRESSION_BASELINE_QUALIFICATION_RECORD_KIND {
            violations.push(RegressionBaselineQualificationViolation::RecordKind {
                expected: REGRESSION_BASELINE_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(RegressionBaselineQualificationViolation::DuplicateId {
                    kind: RegressionBaselineQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.baseline_identity_visible
                    || !surface.guards.build_identity_visible
                    || !surface.guards.environment_fingerprint_visible
                    || !surface.guards.capture_mode_visible
                    || !surface.guards.storage_location_visible
                    || !surface.guards.freshness_state_visible
                    || !surface.guards.comparison_basis_visible
                    || !surface.guards.environment_match_visible
                    || !surface.guards.mismatch_warning_visible
                    || !surface.guards.guard_criteria_visible)
            {
                violations.push(RegressionBaselineQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut baseline_ids = BTreeSet::new();
        for baseline in &self.baseline_stores {
            if !baseline_ids.insert(baseline.baseline_id.clone()) {
                violations.push(RegressionBaselineQualificationViolation::DuplicateId {
                    kind: RegressionBaselineQualificationViolationKind::BaselineStore,
                    id: baseline.baseline_id.clone(),
                });
            }
            if baseline.baseline_id.trim().is_empty()
                || baseline.title.trim().is_empty()
                || baseline.exact_build_identity_ref.trim().is_empty()
                || baseline.environment_fingerprint_ref.trim().is_empty()
                || baseline.capture_mode_ref.trim().is_empty()
                || baseline.storage_location_ref.trim().is_empty()
            {
                violations.push(RegressionBaselineQualificationViolation::IncompleteBaselineStore {
                    baseline_id: baseline.baseline_id.clone(),
                });
            }
            if !baseline.shows_freshness_state || !baseline.shows_build_identity {
                violations.push(
                    RegressionBaselineQualificationViolation::BaselineStoreMissingTruthLabels {
                        baseline_id: baseline.baseline_id.clone(),
                    },
                );
            }
        }

        let mut selection_ids = BTreeSet::new();
        for selection in &self.baseline_selection_uxs {
            if !selection_ids.insert(selection.selection_id.clone()) {
                violations.push(RegressionBaselineQualificationViolation::DuplicateId {
                    kind: RegressionBaselineQualificationViolationKind::BaselineSelectionUx,
                    id: selection.selection_id.clone(),
                });
            }
            if selection.selection_id.trim().is_empty()
                || selection.title.trim().is_empty()
                || selection.comparison_basis_label.trim().is_empty()
                || selection.baseline_ref.trim().is_empty()
                || selection.current_environment_fingerprint_ref.trim().is_empty()
            {
                violations.push(
                    RegressionBaselineQualificationViolation::IncompleteBaselineSelectionUx {
                        selection_id: selection.selection_id.clone(),
                    },
                );
            }
            if selection.environment_match_state.shows_warning()
                && (!selection.shows_mismatch_warning && !selection.shows_stale_warning)
            {
                violations.push(
                    RegressionBaselineQualificationViolation::BaselineSelectionUxMissingWarning {
                        selection_id: selection.selection_id.clone(),
                    },
                );
            }
        }

        let mut guard_ids = BTreeSet::new();
        for guard in &self.comparable_environment_guards {
            if !guard_ids.insert(guard.guard_id.clone()) {
                violations.push(RegressionBaselineQualificationViolation::DuplicateId {
                    kind: RegressionBaselineQualificationViolationKind::ComparableEnvironmentGuard,
                    id: guard.guard_id.clone(),
                });
            }
            if guard.guard_id.trim().is_empty()
                || guard.title.trim().is_empty()
                || guard.freshness_policy.trim().is_empty()
            {
                violations.push(
                    RegressionBaselineQualificationViolation::IncompleteComparableEnvironmentGuard {
                        guard_id: guard.guard_id.clone(),
                    },
                );
            }
            if !guard.shows_criteria_breakdown || !guard.narrows_claim_on_mismatch {
                violations.push(
                    RegressionBaselineQualificationViolation::ComparableEnvironmentGuardMissingBehavior {
                        guard_id: guard.guard_id.clone(),
                    },
                );
            }
        }

        let mut fingerprint_ids = BTreeSet::new();
        for fingerprint in &self.environment_fingerprints {
            if !fingerprint_ids.insert(fingerprint.fingerprint_id.clone()) {
                violations.push(RegressionBaselineQualificationViolation::DuplicateId {
                    kind: RegressionBaselineQualificationViolationKind::EnvironmentFingerprint,
                    id: fingerprint.fingerprint_id.clone(),
                });
            }
            if fingerprint.fingerprint_id.trim().is_empty()
                || fingerprint.title.trim().is_empty()
                || fingerprint.architecture.trim().is_empty()
                || fingerprint.os_version.trim().is_empty()
                || fingerprint.runtime_version.trim().is_empty()
                || fingerprint.build_identity_ref.trim().is_empty()
            {
                violations.push(
                    RegressionBaselineQualificationViolation::IncompleteEnvironmentFingerprint {
                        fingerprint_id: fingerprint.fingerprint_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every baseline store must point to a known environment fingerprint.
        let fingerprint_id_set: BTreeSet<String> = self
            .environment_fingerprints
            .iter()
            .map(|f| f.fingerprint_id.clone())
            .collect();
        for baseline in &self.baseline_stores {
            if !fingerprint_id_set.contains(&baseline.environment_fingerprint_ref) {
                violations.push(
                    RegressionBaselineQualificationViolation::BaselineStoreFingerprintRefUnknown {
                        baseline_id: baseline.baseline_id.clone(),
                        fingerprint_ref: baseline.environment_fingerprint_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every baseline selection UX must point to a known baseline.
        let baseline_id_set: BTreeSet<String> = self
            .baseline_stores
            .iter()
            .map(|b| b.baseline_id.clone())
            .collect();
        for selection in &self.baseline_selection_uxs {
            if !baseline_id_set.contains(&selection.baseline_ref) {
                violations.push(
                    RegressionBaselineQualificationViolation::BaselineSelectionUxBaselineRefUnknown {
                        selection_id: selection.selection_id.clone(),
                        baseline_ref: selection.baseline_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every baseline selection UX must point to a known environment fingerprint.
        for selection in &self.baseline_selection_uxs {
            if !fingerprint_id_set.contains(&selection.current_environment_fingerprint_ref) {
                violations.push(
                    RegressionBaselineQualificationViolation::BaselineSelectionUxFingerprintRefUnknown {
                        selection_id: selection.selection_id.clone(),
                        fingerprint_ref: selection.current_environment_fingerprint_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(RegressionBaselineQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in regression-baseline qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_regression_baseline_qualification(
) -> Result<RegressionBaselineQualificationPacket, serde_json::Error> {
    serde_json::from_str(REGRESSION_BASELINE_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionBaselineQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Baseline-store rows.
    BaselineStore,
    /// Baseline-selection UX rows.
    BaselineSelectionUx,
    /// Comparable-environment guard rows.
    ComparableEnvironmentGuard,
    /// Environment-fingerprint rows.
    EnvironmentFingerprint,
}

impl fmt::Display for RegressionBaselineQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::BaselineStore => write!(f, "baseline_store"),
            Self::BaselineSelectionUx => write!(f, "baseline_selection_ux"),
            Self::ComparableEnvironmentGuard => write!(f, "comparable_environment_guard"),
            Self::EnvironmentFingerprint => write!(f, "environment_fingerprint"),
        }
    }
}

/// Validation failure for regression-baseline qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegressionBaselineQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Record kind does not match the model.
    RecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// IDs must be unique inside an object family.
    DuplicateId {
        /// Kind of object family.
        kind: RegressionBaselineQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A baseline-store row is incomplete.
    IncompleteBaselineStore {
        /// Baseline id.
        baseline_id: String,
    },
    /// A baseline-store row must show freshness state and build identity.
    BaselineStoreMissingTruthLabels {
        /// Baseline id.
        baseline_id: String,
    },
    /// A baseline-selection UX row is incomplete.
    IncompleteBaselineSelectionUx {
        /// Selection id.
        selection_id: String,
    },
    /// A baseline-selection UX row must show a warning when the environment match state warns.
    BaselineSelectionUxMissingWarning {
        /// Selection id.
        selection_id: String,
    },
    /// A comparable-environment guard row is incomplete.
    IncompleteComparableEnvironmentGuard {
        /// Guard id.
        guard_id: String,
    },
    /// A comparable-environment guard row must show criteria breakdown and narrow on mismatch.
    ComparableEnvironmentGuardMissingBehavior {
        /// Guard id.
        guard_id: String,
    },
    /// An environment-fingerprint row is incomplete.
    IncompleteEnvironmentFingerprint {
        /// Fingerprint id.
        fingerprint_id: String,
    },
    /// A baseline store references an unknown environment fingerprint.
    BaselineStoreFingerprintRefUnknown {
        /// Baseline id.
        baseline_id: String,
        /// Unknown fingerprint ref.
        fingerprint_ref: String,
    },
    /// A baseline selection UX references an unknown baseline.
    BaselineSelectionUxBaselineRefUnknown {
        /// Selection id.
        selection_id: String,
        /// Unknown baseline ref.
        baseline_ref: String,
    },
    /// A baseline selection UX references an unknown environment fingerprint.
    BaselineSelectionUxFingerprintRefUnknown {
        /// Selection id.
        selection_id: String,
        /// Unknown fingerprint ref.
        fingerprint_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for RegressionBaselineQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => {
                write!(f, "duplicate {kind} id: {id}")
            }
            Self::IncompleteGuardSet { surface_id } => {
                write!(
                    f,
                    "surface {surface_id} claims stable but guard set is incomplete"
                )
            }
            Self::IncompleteBaselineStore { baseline_id } => {
                write!(f, "incomplete baseline-store row: {baseline_id}")
            }
            Self::BaselineStoreMissingTruthLabels { baseline_id } => {
                write!(
                    f,
                    "baseline-store row {baseline_id} must show freshness state and build identity"
                )
            }
            Self::IncompleteBaselineSelectionUx { selection_id } => {
                write!(f, "incomplete baseline-selection UX row: {selection_id}")
            }
            Self::BaselineSelectionUxMissingWarning { selection_id } => {
                write!(
                    f,
                    "baseline-selection UX row {selection_id} must show a mismatch or stale warning when environment match state warns"
                )
            }
            Self::IncompleteComparableEnvironmentGuard { guard_id } => {
                write!(f, "incomplete comparable-environment guard row: {guard_id}")
            }
            Self::ComparableEnvironmentGuardMissingBehavior { guard_id } => {
                write!(
                    f,
                    "comparable-environment guard row {guard_id} must show criteria breakdown and narrow on mismatch"
                )
            }
            Self::IncompleteEnvironmentFingerprint { fingerprint_id } => {
                write!(f, "incomplete environment-fingerprint row: {fingerprint_id}")
            }
            Self::BaselineStoreFingerprintRefUnknown {
                baseline_id,
                fingerprint_ref,
            } => {
                write!(
                    f,
                    "baseline store {baseline_id} references unknown environment fingerprint {fingerprint_ref}"
                )
            }
            Self::BaselineSelectionUxBaselineRefUnknown {
                selection_id,
                baseline_ref,
            } => {
                write!(
                    f,
                    "baseline selection UX {selection_id} references unknown baseline {baseline_ref}"
                )
            }
            Self::BaselineSelectionUxFingerprintRefUnknown {
                selection_id,
                fingerprint_ref,
            } => {
                write!(
                    f,
                    "baseline selection UX {selection_id} references unknown environment fingerprint {fingerprint_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for RegressionBaselineQualificationViolation {}
