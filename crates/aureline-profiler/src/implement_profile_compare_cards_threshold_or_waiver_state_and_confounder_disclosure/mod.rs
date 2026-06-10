//! Implement profile-compare cards, threshold or waiver state, and confounder disclosure.
//!
//! This module materializes the typed records that keep profile-comparison surfaces
//! honest about what is being compared, whether the comparison is within threshold,
//! whether an active waiver covers a threshold breach, and what confounders may
//! weaken the comparison claim. The records and closed vocabularies here mirror the
//! boundary schema at
//! `/schemas/perf/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and comparison axes
//! already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`ProfileCompareCardRow`] record that binds left and right capture refs,
//!   comparison kind, threshold state ref, and confounder refs so comparison cards
//!   never silently compare incomparable evidence;
//! - the [`ThresholdStateRow`] record that carries metric family, threshold value,
//!   current value, threshold state, and visual-bar truth so users see whether a
//!   comparison is within bounds, in warning, breached, waived, or provisional;
//! - the [`WaiverStateRow`] record that carries threshold ref, waiver status,
//!   waiver cause, expiry proximity, and honest expiry labels so waived breaches
//!   are never mistaken for clean passes;
//! - the [`ConfounderDisclosureRow`] record that carries confounder kind, severity,
//!   blocking status, disclosure text, and mitigation hint so every comparison
//!   surface narrows its claim automatically when mapping fidelity, baseline
//!   comparability, or artifact identity are weak;
//! - the [`ProfileCompareQualificationPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every profile-compare qualification packet carried by
/// this module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const PROFILE_COMPARE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ProfileCompareQualificationPacket`].
pub const PROFILE_COMPARE_QUALIFICATION_RECORD_KIND: &str =
    "implement_profile_compare_cards_threshold_or_waiver_state_and_confounder_disclosure";

/// Repo-relative path to the checked-in profile-compare qualification packet JSON.
pub const PROFILE_COMPARE_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json";

/// Embedded checked-in qualification packet JSON.
pub const PROFILE_COMPARE_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json"
));

/// Qualification label shown on promoted profile-compare surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileCompareQualificationLabel {
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

impl ProfileCompareQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Profile-compare surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileCompareSurfaceKind {
    /// Side-by-side profile-compare card.
    ProfileCompareCard,
    /// Threshold inspector showing metric against bounds.
    ThresholdInspector,
    /// Waiver badge or banner.
    WaiverBadge,
    /// Confounder disclosure panel.
    ConfounderDisclosurePanel,
    /// Export review surface for comparison evidence.
    ExportReview,
    /// Support export surface for comparison evidence.
    SupportExport,
}

/// Threshold state for a profile comparison metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdState {
    /// Current value is within the threshold bound.
    Within,
    /// Current value is inside the warning band.
    Warning,
    /// Current value breaches the threshold.
    Breach,
    /// Breach is covered by an active waiver.
    Waived,
    /// Threshold is provisional and not yet hardened.
    Provisional,
}

impl ThresholdState {
    /// Returns true when the state allows the comparison to proceed without
    /// blocking.
    pub const fn allows_comparison(self) -> bool {
        matches!(self, Self::Within | Self::Warning | Self::Waived | Self::Provisional)
    }

    /// Returns true when the state represents a breach (waived or not).
    pub const fn is_breach(self) -> bool {
        matches!(self, Self::Breach | Self::Waived)
    }

    /// Returns true when the state should show a warning or breach label.
    pub const fn shows_alert(self) -> bool {
        matches!(self, Self::Warning | Self::Breach | Self::Waived | Self::Provisional)
    }
}

/// Waiver status for a threshold breach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaiverStatus {
    /// Waiver is active and covers the breach.
    Active,
    /// Waiver has expired; the breach is no longer covered.
    Expired,
    /// Waiver is pending review or approval.
    Pending,
    /// Waiver has been retired after mitigation.
    Retired,
}

impl WaiverStatus {
    /// Returns true when the waiver is currently covering a breach.
    pub const fn is_covering(self) -> bool {
        matches!(self, Self::Active)
    }
}

/// Severity of a confounder disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfounderSeverity {
    /// Confounder makes the comparison unreliable or invalid.
    Critical,
    /// Confounder materially weakens the comparison claim.
    Major,
    /// Confounder is present but has limited impact.
    Minor,
    /// Confounder is informational only.
    Info,
}

impl ConfounderSeverity {
    /// Returns true when the confounder blocks a stable comparison claim.
    pub const fn blocks_stable_claim(self) -> bool {
        matches!(self, Self::Critical | Self::Major)
    }
}

/// Kind of confounder that may weaken a profile comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfounderKind {
    /// Left and right builds differ.
    BuildMismatch,
    /// Environment has drifted between captures.
    EnvironmentDrift,
    /// Capture modes are not comparable.
    CaptureModeDiff,
    /// Mapping quality degraded on one side.
    MappingQualityDegraded,
    /// Sample size is too small for reliable comparison.
    SampleSizeInsufficient,
    /// Thermal throttling affected one capture.
    ThermalThrottle,
    /// Clock skew between capture devices.
    ClockSkew,
    /// Other confounder named in the disclosure text.
    Other,
}

/// Kind of profile comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonKind {
    /// Side-by-side overlay or split view.
    SideBySide,
    /// Delta view showing numeric or structural differences.
    Delta,
    /// Trend view showing change over time.
    Trend,
    /// Baseline versus current capture.
    BaselineVsCurrent,
}

/// One profile-compare card row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCompareCardRow {
    /// Stable card row id.
    pub card_id: String,
    /// Human-readable title.
    pub title: String,
    /// Left-side capture session ref.
    pub left_capture_ref: String,
    /// Right-side capture session ref.
    pub right_capture_ref: String,
    /// Comparison kind.
    pub comparison_kind: ComparisonKind,
    /// Threshold state ref.
    pub threshold_state_ref: String,
    /// Confounder disclosure refs.
    #[serde(default)]
    pub confounder_refs: Vec<String>,
    /// True when the card shows confounder disclosures honestly.
    pub shows_confounders: bool,
    /// True when the card is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One threshold-state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThresholdStateRow {
    /// Stable threshold row id.
    pub threshold_id: String,
    /// Human-readable title.
    pub title: String,
    /// Metric family label.
    pub metric_family: String,
    /// Threshold value in percent or basis points.
    pub threshold_value_pct: i64,
    /// Current measured value in percent or basis points.
    pub current_value_pct: i64,
    /// Threshold state.
    pub threshold_state: ThresholdState,
    /// True when the threshold bar is visible.
    pub shows_threshold_bar: bool,
    /// True when the threshold is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One waiver-state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaiverStateRow {
    /// Stable waiver row id.
    pub waiver_id: String,
    /// Human-readable title.
    pub title: String,
    /// Threshold ref being waived.
    pub threshold_ref: String,
    /// Waiver status.
    pub waiver_status: WaiverStatus,
    /// Waiver cause label.
    pub waiver_cause: String,
    /// Expiry proximity label.
    pub expiry_proximity: String,
    /// True when the waiver shows expiry information.
    pub shows_expiry: bool,
    /// True when the waiver is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One confounder-disclosure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfounderDisclosureRow {
    /// Stable disclosure row id.
    pub disclosure_id: String,
    /// Human-readable title.
    pub title: String,
    /// Confounder kind.
    pub confounder_kind: ConfounderKind,
    /// Severity level.
    pub severity: ConfounderSeverity,
    /// True when the confounder blocks a stable comparison claim.
    pub is_blocking: bool,
    /// Disclosure text shown to the user.
    pub disclosure_text: String,
    /// Mitigation hint or workaround suggestion.
    pub mitigation_hint: String,
    /// True when the disclosure is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCompareQualificationProof {
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
pub struct ProfileCompareQualificationSummary {
    /// Total number of profile-compare card rows.
    pub profile_compare_card_count: usize,
    /// Total number of threshold-state rows.
    pub threshold_state_count: usize,
    /// Total number of waiver-state rows.
    pub waiver_state_count: usize,
    /// Total number of confounder-disclosure rows.
    pub confounder_disclosure_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of threshold states that are breaches.
    pub breach_count: usize,
    /// Number of breaches covered by active waivers.
    pub waived_breach_count: usize,
    /// Number of blocking confounders.
    pub blocking_confounder_count: usize,
}

/// Guard set for a profile-compare surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCompareSurfaceGuardSet {
    /// Profile-compare card is visible.
    pub compare_card_visible: bool,
    /// Threshold inspector is visible.
    pub threshold_inspector_visible: bool,
    /// Waiver badge is visible.
    pub waiver_badge_visible: bool,
    /// Confounder disclosure panel is visible.
    pub confounder_disclosure_visible: bool,
    /// Capture identity (left and right) is visible.
    pub capture_identity_visible: bool,
    /// Comparison basis is visible.
    pub comparison_basis_visible: bool,
    /// Threshold bar is visible.
    pub threshold_bar_visible: bool,
    /// Waiver expiry is visible.
    pub waiver_expiry_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
    /// Mapping quality is visible.
    pub mapping_quality_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCompareSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: ProfileCompareSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: ProfileCompareQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: ProfileCompareQualificationProof,
    /// Guard set.
    pub guards: ProfileCompareSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in profile-compare qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCompareQualificationPacket {
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
    pub surfaces: Vec<ProfileCompareSurfaceQualificationRow>,
    /// Profile-compare card rows.
    pub compare_cards: Vec<ProfileCompareCardRow>,
    /// Threshold-state rows.
    pub threshold_states: Vec<ThresholdStateRow>,
    /// Waiver-state rows.
    pub waiver_states: Vec<WaiverStateRow>,
    /// Confounder-disclosure rows.
    pub confounder_disclosures: Vec<ConfounderDisclosureRow>,
    /// Summary.
    pub summary: ProfileCompareQualificationSummary,
}

impl ProfileCompareQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> ProfileCompareQualificationSummary {
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
        let breach_count = self
            .threshold_states
            .iter()
            .filter(|t| t.threshold_state.is_breach())
            .count();
        let waived_breach_count = self
            .threshold_states
            .iter()
            .filter(|t| matches!(t.threshold_state, ThresholdState::Waived))
            .count();
        let blocking_confounder_count = self
            .confounder_disclosures
            .iter()
            .filter(|c| c.is_blocking || c.severity.blocks_stable_claim())
            .count();

        ProfileCompareQualificationSummary {
            profile_compare_card_count: self.compare_cards.len(),
            threshold_state_count: self.threshold_states.len(),
            waiver_state_count: self.waiver_states.len(),
            confounder_disclosure_count: self.confounder_disclosures.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            breach_count,
            waived_breach_count,
            blocking_confounder_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<ProfileCompareQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROFILE_COMPARE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ProfileCompareQualificationViolation::SchemaVersion {
                expected: PROFILE_COMPARE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != PROFILE_COMPARE_QUALIFICATION_RECORD_KIND {
            violations.push(ProfileCompareQualificationViolation::RecordKind {
                expected: PROFILE_COMPARE_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(ProfileCompareQualificationViolation::DuplicateId {
                    kind: ProfileCompareQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.compare_card_visible
                    || !surface.guards.threshold_inspector_visible
                    || !surface.guards.waiver_badge_visible
                    || !surface.guards.confounder_disclosure_visible
                    || !surface.guards.capture_identity_visible
                    || !surface.guards.comparison_basis_visible
                    || !surface.guards.threshold_bar_visible
                    || !surface.guards.waiver_expiry_visible
                    || !surface.guards.mapping_quality_visible)
            {
                violations.push(ProfileCompareQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut card_ids = BTreeSet::new();
        for card in &self.compare_cards {
            if !card_ids.insert(card.card_id.clone()) {
                violations.push(ProfileCompareQualificationViolation::DuplicateId {
                    kind: ProfileCompareQualificationViolationKind::CompareCard,
                    id: card.card_id.clone(),
                });
            }
            if card.card_id.trim().is_empty()
                || card.title.trim().is_empty()
                || card.left_capture_ref.trim().is_empty()
                || card.right_capture_ref.trim().is_empty()
                || card.threshold_state_ref.trim().is_empty()
            {
                violations.push(ProfileCompareQualificationViolation::IncompleteCompareCard {
                    card_id: card.card_id.clone(),
                });
            }
            if !card.shows_confounders {
                violations.push(
                    ProfileCompareQualificationViolation::CompareCardMissingConfounders {
                        card_id: card.card_id.clone(),
                    },
                );
            }
        }

        let mut threshold_ids = BTreeSet::new();
        for threshold in &self.threshold_states {
            if !threshold_ids.insert(threshold.threshold_id.clone()) {
                violations.push(ProfileCompareQualificationViolation::DuplicateId {
                    kind: ProfileCompareQualificationViolationKind::ThresholdState,
                    id: threshold.threshold_id.clone(),
                });
            }
            if threshold.threshold_id.trim().is_empty()
                || threshold.title.trim().is_empty()
                || threshold.metric_family.trim().is_empty()
            {
                violations.push(ProfileCompareQualificationViolation::IncompleteThresholdState {
                    threshold_id: threshold.threshold_id.clone(),
                });
            }
            if !threshold.shows_threshold_bar {
                violations.push(
                    ProfileCompareQualificationViolation::ThresholdStateMissingBar {
                        threshold_id: threshold.threshold_id.clone(),
                    },
                );
            }
        }

        let mut waiver_ids = BTreeSet::new();
        for waiver in &self.waiver_states {
            if !waiver_ids.insert(waiver.waiver_id.clone()) {
                violations.push(ProfileCompareQualificationViolation::DuplicateId {
                    kind: ProfileCompareQualificationViolationKind::WaiverState,
                    id: waiver.waiver_id.clone(),
                });
            }
            if waiver.waiver_id.trim().is_empty()
                || waiver.title.trim().is_empty()
                || waiver.threshold_ref.trim().is_empty()
                || waiver.waiver_cause.trim().is_empty()
                || waiver.expiry_proximity.trim().is_empty()
            {
                violations.push(ProfileCompareQualificationViolation::IncompleteWaiverState {
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.shows_expiry {
                violations.push(ProfileCompareQualificationViolation::WaiverStateMissingExpiry {
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }

        let mut disclosure_ids = BTreeSet::new();
        for disclosure in &self.confounder_disclosures {
            if !disclosure_ids.insert(disclosure.disclosure_id.clone()) {
                violations.push(ProfileCompareQualificationViolation::DuplicateId {
                    kind: ProfileCompareQualificationViolationKind::ConfounderDisclosure,
                    id: disclosure.disclosure_id.clone(),
                });
            }
            if disclosure.disclosure_id.trim().is_empty()
                || disclosure.title.trim().is_empty()
                || disclosure.disclosure_text.trim().is_empty()
                || disclosure.mitigation_hint.trim().is_empty()
            {
                violations.push(
                    ProfileCompareQualificationViolation::IncompleteConfounderDisclosure {
                        disclosure_id: disclosure.disclosure_id.clone(),
                    },
                );
            }
            if disclosure.is_blocking != disclosure.severity.blocks_stable_claim() {
                violations.push(
                    ProfileCompareQualificationViolation::ConfounderDisclosureBlockingMismatch {
                        disclosure_id: disclosure.disclosure_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every compare card must point to a known threshold state.
        let threshold_id_set: BTreeSet<String> =
            self.threshold_states.iter().map(|t| t.threshold_id.clone()).collect();
        for card in &self.compare_cards {
            if !threshold_id_set.contains(&card.threshold_state_ref) {
                violations.push(
                    ProfileCompareQualificationViolation::CompareCardThresholdRefUnknown {
                        card_id: card.card_id.clone(),
                        threshold_ref: card.threshold_state_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every waiver must point to a known threshold state.
        for waiver in &self.waiver_states {
            if !threshold_id_set.contains(&waiver.threshold_ref) {
                violations.push(
                    ProfileCompareQualificationViolation::WaiverStateThresholdRefUnknown {
                        waiver_id: waiver.waiver_id.clone(),
                        threshold_ref: waiver.threshold_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every compare card confounder ref must point to a known disclosure.
        let disclosure_id_set: BTreeSet<String> = self
            .confounder_disclosures
            .iter()
            .map(|d| d.disclosure_id.clone())
            .collect();
        for card in &self.compare_cards {
            for confounder_ref in &card.confounder_refs {
                if !disclosure_id_set.contains(confounder_ref) {
                    violations.push(
                        ProfileCompareQualificationViolation::CompareCardConfounderRefUnknown {
                            card_id: card.card_id.clone(),
                            confounder_ref: confounder_ref.clone(),
                        },
                    );
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ProfileCompareQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in profile-compare qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_profile_compare_qualification(
) -> Result<ProfileCompareQualificationPacket, serde_json::Error> {
    serde_json::from_str(PROFILE_COMPARE_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileCompareQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Profile-compare card rows.
    CompareCard,
    /// Threshold-state rows.
    ThresholdState,
    /// Waiver-state rows.
    WaiverState,
    /// Confounder-disclosure rows.
    ConfounderDisclosure,
}

impl fmt::Display for ProfileCompareQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::CompareCard => write!(f, "compare_card"),
            Self::ThresholdState => write!(f, "threshold_state"),
            Self::WaiverState => write!(f, "waiver_state"),
            Self::ConfounderDisclosure => write!(f, "confounder_disclosure"),
        }
    }
}

/// Validation failure for profile-compare qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileCompareQualificationViolation {
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
        kind: ProfileCompareQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A compare-card row is incomplete.
    IncompleteCompareCard {
        /// Card id.
        card_id: String,
    },
    /// A compare-card row must show confounders.
    CompareCardMissingConfounders {
        /// Card id.
        card_id: String,
    },
    /// A threshold-state row is incomplete.
    IncompleteThresholdState {
        /// Threshold id.
        threshold_id: String,
    },
    /// A threshold-state row must show its threshold bar.
    ThresholdStateMissingBar {
        /// Threshold id.
        threshold_id: String,
    },
    /// A waiver-state row is incomplete.
    IncompleteWaiverState {
        /// Waiver id.
        waiver_id: String,
    },
    /// A waiver-state row must show expiry information.
    WaiverStateMissingExpiry {
        /// Waiver id.
        waiver_id: String,
    },
    /// A confounder-disclosure row is incomplete.
    IncompleteConfounderDisclosure {
        /// Disclosure id.
        disclosure_id: String,
    },
    /// A confounder-disclosure row has mismatched blocking and severity.
    ConfounderDisclosureBlockingMismatch {
        /// Disclosure id.
        disclosure_id: String,
    },
    /// A compare card references an unknown threshold state.
    CompareCardThresholdRefUnknown {
        /// Card id.
        card_id: String,
        /// Unknown threshold ref.
        threshold_ref: String,
    },
    /// A waiver state references an unknown threshold state.
    WaiverStateThresholdRefUnknown {
        /// Waiver id.
        waiver_id: String,
        /// Unknown threshold ref.
        threshold_ref: String,
    },
    /// A compare card references an unknown confounder disclosure.
    CompareCardConfounderRefUnknown {
        /// Card id.
        card_id: String,
        /// Unknown confounder ref.
        confounder_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for ProfileCompareQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
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
            Self::IncompleteCompareCard { card_id } => {
                write!(f, "incomplete compare-card row: {card_id}")
            }
            Self::CompareCardMissingConfounders { card_id } => {
                write!(
                    f,
                    "compare-card row {card_id} must show confounders"
                )
            }
            Self::IncompleteThresholdState { threshold_id } => {
                write!(f, "incomplete threshold-state row: {threshold_id}")
            }
            Self::ThresholdStateMissingBar { threshold_id } => {
                write!(
                    f,
                    "threshold-state row {threshold_id} must show threshold bar"
                )
            }
            Self::IncompleteWaiverState { waiver_id } => {
                write!(f, "incomplete waiver-state row: {waiver_id}")
            }
            Self::WaiverStateMissingExpiry { waiver_id } => {
                write!(
                    f,
                    "waiver-state row {waiver_id} must show expiry information"
                )
            }
            Self::IncompleteConfounderDisclosure { disclosure_id } => {
                write!(f, "incomplete confounder-disclosure row: {disclosure_id}")
            }
            Self::ConfounderDisclosureBlockingMismatch { disclosure_id } => {
                write!(
                    f,
                    "confounder-disclosure row {disclosure_id} has mismatched blocking and severity"
                )
            }
            Self::CompareCardThresholdRefUnknown {
                card_id,
                threshold_ref,
            } => {
                write!(
                    f,
                    "compare card {card_id} references unknown threshold state {threshold_ref}"
                )
            }
            Self::WaiverStateThresholdRefUnknown {
                waiver_id,
                threshold_ref,
            } => {
                write!(
                    f,
                    "waiver state {waiver_id} references unknown threshold state {threshold_ref}"
                )
            }
            Self::CompareCardConfounderRefUnknown {
                card_id,
                confounder_ref,
            } => {
                write!(
                    f,
                    "compare card {card_id} references unknown confounder disclosure {confounder_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for ProfileCompareQualificationViolation {}
