//! Consolidated localized-profile qualification and locale-bearing claim status.
//!
//! This module certifies each claimed localized M5 profile against the distinct
//! localization evidence lanes a stable-profile claim depends on — pseudoloc,
//! text expansion, RTL/bidi, IME composition, translated-help parity, and
//! locale-pack compatibility. It joins those lanes (each produced by a sibling
//! truth packet) into one inspectable claim-status packet, so release, Help/About,
//! diagnostics, support export, and claim-narrowing tooling read one source of
//! truth instead of re-reviewing localization status by hand.
//!
//! Claims cannot outrun their evidence. A profile that intends to claim localized
//! support is auto-narrowed to source-language fallback the moment any required
//! evidence lane goes stale or missing, and is blocked from staying green when a
//! lane is actively failing. Every narrowed or blocked profile publishes explicit
//! known limits and downgrade reasons, naming the lane that narrowed the claim and
//! the surfaces that fall back to source language.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::localized_catalog::TextDirection;
use crate::localized_profile_matrix::{
    ConsumerKind, ConsumptionBindingRow, LocalizableSurfaceFamily, MatrixGateState,
    ProfileClaimClass, ProfileReleaseGateRow,
};
use crate::{
    LocalePackValidationFinding, GENERATED_AT, LOCALE_PACK_COMPATIBILITY_REPORT_FIXTURE_REF,
    LOCALE_PACK_COMPATIBILITY_REPORT_ID, M5_DENSE_SURFACE_LAB_FIXTURE_REF,
    M5_DENSE_SURFACE_LAB_PACKET_ID, M5_TRANSLATED_HELP_PARITY_FIXTURE_REF,
    M5_TRANSLATED_HELP_PARITY_REPORT_ID, SOURCE_LANGUAGE_LOCALE, TARGET_BUILD,
};

/// Schema version for the localized claim-status packet.
pub const LOCALIZED_CLAIM_STATUS_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LocalizedClaimStatusPacket`].
pub const LOCALIZED_CLAIM_STATUS_RECORD_KIND: &str = "localized_claim_status_packet";

/// Stable packet id for the seeded localized claim-status packet.
pub const LOCALIZED_CLAIM_STATUS_PACKET_ID: &str =
    "i18n:m5-localized-profile-qualification:claim-status:v1";

/// Fixture path for the seeded localized claim-status packet.
pub const LOCALIZED_CLAIM_STATUS_FIXTURE_REF: &str =
    "fixtures/i18n/m5-localized-profile-qualification/claim_status.json";

/// Release channel the qualification packet defends.
const RELEASE_CHANNEL: &str = "stable";

/// Evidence lane a localized stable-profile claim must satisfy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationLaneKind {
    /// Pseudolocalization accent wrapping and clip detection.
    Pseudolocalization,
    /// Long translated strings against an explicit expansion budget.
    TextExpansion,
    /// Right-to-left chrome mirroring and bidi technical-token handling.
    RtlBidi,
    /// IME preedit, candidate, and commit behavior under dense churn.
    ImeComposition,
    /// Translated docs, help, tour, citation, and policy parity.
    TranslatedHelpParity,
    /// Locale-pack signature and target-build compatibility.
    LocalePackCompatibility,
}

impl QualificationLaneKind {
    /// Returns every evidence lane a claimed localized profile must satisfy.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Pseudolocalization,
            Self::TextExpansion,
            Self::RtlBidi,
            Self::ImeComposition,
            Self::TranslatedHelpParity,
            Self::LocalePackCompatibility,
        ]
    }

    /// Returns the upstream truth packet id that produces this lane's evidence.
    pub fn evidence_packet_id(self) -> &'static str {
        match self {
            Self::Pseudolocalization
            | Self::TextExpansion
            | Self::RtlBidi
            | Self::ImeComposition => M5_DENSE_SURFACE_LAB_PACKET_ID,
            Self::TranslatedHelpParity => M5_TRANSLATED_HELP_PARITY_REPORT_ID,
            Self::LocalePackCompatibility => LOCALE_PACK_COMPATIBILITY_REPORT_ID,
        }
    }

    /// Returns the upstream fixture that backs this lane's evidence.
    pub fn evidence_fixture_ref(self) -> &'static str {
        match self {
            Self::Pseudolocalization
            | Self::TextExpansion
            | Self::RtlBidi
            | Self::ImeComposition => M5_DENSE_SURFACE_LAB_FIXTURE_REF,
            Self::TranslatedHelpParity => M5_TRANSLATED_HELP_PARITY_FIXTURE_REF,
            Self::LocalePackCompatibility => LOCALE_PACK_COMPATIBILITY_REPORT_FIXTURE_REF,
        }
    }

    /// Returns the stable token used in derived ids for this lane.
    fn token(self) -> &'static str {
        match self {
            Self::Pseudolocalization => "pseudolocalization",
            Self::TextExpansion => "text_expansion",
            Self::RtlBidi => "rtl_bidi",
            Self::ImeComposition => "ime_composition",
            Self::TranslatedHelpParity => "translated_help_parity",
            Self::LocalePackCompatibility => "locale_pack_compatibility",
        }
    }

    /// Returns the surface families that fall back to source language when this lane narrows.
    fn affected_surface_families(self) -> Vec<LocalizableSurfaceFamily> {
        use LocalizableSurfaceFamily as F;
        match self {
            Self::Pseudolocalization | Self::TextExpansion => vec![
                F::ShellChrome,
                F::CommandPalette,
                F::Notifications,
                F::NotebookTooling,
                F::DataAndApiTooling,
            ],
            Self::RtlBidi => vec![
                F::ShellChrome,
                F::CommandPalette,
                F::HelpAndDocs,
                F::NotebookTooling,
            ],
            Self::ImeComposition => {
                vec![F::CommandPalette, F::NotebookTooling, F::DataAndApiTooling]
            }
            Self::TranslatedHelpParity => vec![
                F::HelpAndDocs,
                F::GuidedLearning,
                F::SupportFlows,
                F::ReleaseAndAbout,
            ],
            Self::LocalePackCompatibility => vec![
                F::ShellChrome,
                F::CommandPalette,
                F::HelpAndDocs,
                F::CliAndDoctor,
                F::ExtensionContributedUi,
            ],
        }
    }
}

/// Freshness and pass posture for one evidence lane on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneEvidenceState {
    /// Proof is current for the target build and the lane passed.
    CurrentPassing,
    /// Proof is current for the target build but the lane failed.
    CurrentFailing,
    /// Proof exists but is stale against the target build.
    Stale,
    /// Proof is missing for the lane.
    Missing,
    /// A bounded, expiring waiver currently covers the lane.
    WaivedBounded,
}

impl LaneEvidenceState {
    /// Reports whether the lane currently backs a localized claim.
    fn satisfies_claim(self) -> bool {
        matches!(self, Self::CurrentPassing | Self::WaivedBounded)
    }

    /// Reports whether the lane is actively failing and must block promotion.
    fn is_failing(self) -> bool {
        matches!(self, Self::CurrentFailing)
    }

    /// Maps an unsatisfied lane to the cause it narrows or blocks the claim with.
    fn narrow_cause(self) -> Option<LaneNarrowCause> {
        match self {
            Self::CurrentPassing | Self::WaivedBounded => None,
            Self::CurrentFailing => Some(LaneNarrowCause::EvidenceFailing),
            Self::Stale => Some(LaneNarrowCause::EvidenceStale),
            Self::Missing => Some(LaneNarrowCause::EvidenceMissing),
        }
    }
}

/// Why a localized claim narrowed or blocked on one evidence lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneNarrowCause {
    /// The lane's current evidence is failing.
    EvidenceFailing,
    /// The lane's evidence is stale against the target build.
    EvidenceStale,
    /// The lane's evidence is missing.
    EvidenceMissing,
}

impl LaneNarrowCause {
    /// Returns the gate state a lane with this cause forces on its profile.
    fn gate_state(self) -> MatrixGateState {
        match self {
            Self::EvidenceFailing => MatrixGateState::Blocked,
            Self::EvidenceStale | Self::EvidenceMissing => MatrixGateState::Narrowed,
        }
    }
}

/// One evidence lane evaluated for one localized profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLaneResult {
    /// Evidence lane this result covers.
    pub lane_kind: QualificationLaneKind,
    /// Freshness and pass posture observed for the lane.
    pub evidence_state: LaneEvidenceState,
    /// Upstream truth packet id that produced this lane's evidence.
    pub evidence_packet_ref: String,
    /// Upstream fixture that backs this lane's evidence.
    pub evidence_fixture_ref: String,
    /// Whether this lane gates the profile's localized claim.
    pub required_for_claim: bool,
    /// Export-safe detail; never carries raw translated bodies.
    pub detail: String,
    /// Bounded waiver ref when the lane is waived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Narrowing cause derived from the evidence state, when unsatisfied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_cause: Option<LaneNarrowCause>,
}

/// Published known limit and downgrade reason for a narrowed or blocked profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitRow {
    /// Stable known-limit id.
    pub limit_id: String,
    /// Profile this limit applies to.
    pub profile_id_ref: String,
    /// Evidence lane that narrowed or blocked the claim.
    pub lane_kind: QualificationLaneKind,
    /// Cause for the downgrade.
    pub cause: LaneNarrowCause,
    /// Gate state this limit forces on the profile.
    pub gate_state: MatrixGateState,
    /// Export-safe human summary of the limit and downgrade reason.
    pub summary: String,
    /// Surface families that fall back to source language under this limit.
    pub affected_surface_families: Vec<LocalizableSurfaceFamily>,
    /// Same-surface source-language route that stays available.
    pub source_language_route_ref: String,
    /// Surfaces that publish this known limit and downgrade reason.
    pub published_to: Vec<ConsumerKind>,
}

/// Per-profile localized claim status derived from the evidence lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedProfileClaimRow {
    /// Stable profile id.
    pub profile_id: String,
    /// Human title of the profile.
    pub title: String,
    /// Requested locale for the profile.
    pub requested_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Writing direction the profile renders.
    pub text_direction: TextDirection,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Primary locale pack backing the profile.
    pub primary_pack_ref: String,
    /// Claim the profile intends to make before qualification.
    pub intended_claim_class: ProfileClaimClass,
    /// Effective claim after auto-narrowing against the evidence lanes.
    pub effective_claim_class: ProfileClaimClass,
    /// Gate state derived from the evidence lanes.
    pub gate_state: MatrixGateState,
    /// Whether the effective claim narrowed below the intended claim.
    pub narrowed: bool,
    /// Whether the profile blocks promotion.
    pub blocks_promotion: bool,
    /// Evidence lanes evaluated for the profile.
    pub lane_results: Vec<ProfileLaneResult>,
    /// Number of lanes that satisfy the claim.
    pub satisfied_lane_count: usize,
    /// Number of lanes that are stale or missing.
    pub stale_or_missing_lane_count: usize,
    /// Number of lanes that are actively failing.
    pub failing_lane_count: usize,
    /// Number of lanes under a bounded waiver.
    pub waived_lane_count: usize,
    /// Evidence lanes that narrowed or blocked the claim.
    pub affected_lane_kinds: Vec<QualificationLaneKind>,
    /// Same-surface source-language route for the profile.
    pub source_language_route_ref: String,
    /// Whether Settings exposes this profile.
    pub visible_in_settings: bool,
    /// Whether Help/About exposes this profile.
    pub visible_in_help_about: bool,
    /// Whether the release center exposes this profile.
    pub visible_in_release_center: bool,
    /// Whether diagnostics exposes this profile.
    pub visible_in_diagnostics: bool,
    /// Whether support export exposes this profile.
    pub visible_in_support_export: bool,
    /// Whether missing or narrowed localization keeps local product use available.
    pub non_blocking_core_use: bool,
}

/// Summary posture for the localized claim-status packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedClaimStatusSummary {
    /// Number of qualified profiles.
    pub total_profiles: usize,
    /// Number of profiles holding a green localized claim.
    pub claimed_localized_profiles: usize,
    /// Number of profiles narrowed to source-language fallback.
    pub narrowed_profiles: usize,
    /// Number of profiles blocked from claiming localized support.
    pub blocked_profiles: usize,
    /// Number of explicitly non-localized profiles.
    pub not_localized_profiles: usize,
    /// Number of evidence-lane results evaluated.
    pub total_lane_results: usize,
    /// Number of lane results that satisfy a claim.
    pub satisfied_lane_results: usize,
    /// Number of lane results that are stale or missing.
    pub stale_or_missing_lane_results: usize,
    /// Number of lane results that are actively failing.
    pub failing_lane_results: usize,
    /// Number of lane results under a bounded waiver.
    pub waived_lane_results: usize,
    /// Number of published known limits and downgrade reasons.
    pub published_known_limits: usize,
    /// Overall promotion state.
    pub promotion_state: MatrixGateState,
}

/// Localized claim-status qualification packet for claimed M5 profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedClaimStatusPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Release channel the packet defends.
    pub release_channel: String,
    /// Target build identity the packet qualifies.
    pub target_build_identity_ref: String,
    /// Source contracts that govern this packet.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this packet.
    pub runtime_consumer_refs: Vec<String>,
    /// Evidence lane kind to upstream truth packet id.
    pub evidence_lane_refs: BTreeMap<String, String>,
    /// Per-profile localized claim status.
    pub claimed_profiles: Vec<LocalizedProfileClaimRow>,
    /// Published known limits and downgrade reasons.
    pub known_limits: Vec<KnownLimitRow>,
    /// Downstream consumption bindings.
    pub consumption_bindings: Vec<ConsumptionBindingRow>,
    /// Release-gated proof rows.
    pub release_gate_rows: Vec<ProfileReleaseGateRow>,
    /// Summary posture derived from the rows.
    pub summary: LocalizedClaimStatusSummary,
}

impl LocalizedClaimStatusPacket {
    /// Validates lane coverage, derived gates, known limits, and the summary.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != LOCALIZED_CLAIM_STATUS_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "localized claim status record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALIZED_CLAIM_STATUS_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "localized claim status schema_version is unsupported",
            ));
        }
        if self.packet_id != LOCALIZED_CLAIM_STATUS_PACKET_ID {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "localized claim status packet id drifted",
            ));
        }

        validate_profiles(self, &mut findings);
        validate_known_limits(self, &mut findings);
        validate_evidence_lane_refs(self, &mut findings);
        validate_consumption_bindings(&self.consumption_bindings, &mut findings);
        validate_release_gates(&self.release_gate_rows, &mut findings);
        validate_summary(self, &mut findings);

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Returns the per-profile claim status for a profile id, if present.
    pub fn profile(&self, profile_id: &str) -> Option<&LocalizedProfileClaimRow> {
        self.claimed_profiles
            .iter()
            .find(|row| row.profile_id == profile_id)
    }

    /// Returns the known limits published for a profile id.
    pub fn known_limits_for(&self, profile_id: &str) -> Vec<&KnownLimitRow> {
        self.known_limits
            .iter()
            .filter(|row| row.profile_id_ref == profile_id)
            .collect()
    }

    /// Returns a re-derived packet with one profile's lane state changed.
    ///
    /// This is how callers prove that a previously green claim can no longer stay
    /// green once its evidence expires or fails: flip a lane to
    /// [`LaneEvidenceState::Stale`] and the profile narrows; flip it to
    /// [`LaneEvidenceState::CurrentFailing`] and the profile blocks.
    pub fn with_lane_state(
        &self,
        profile_id: &str,
        lane_kind: QualificationLaneKind,
        new_state: LaneEvidenceState,
    ) -> Self {
        let mut profiles = self.claimed_profiles.clone();
        if let Some(profile) = profiles.iter_mut().find(|p| p.profile_id == profile_id) {
            if let Some(lane) = profile
                .lane_results
                .iter_mut()
                .find(|l| l.lane_kind == lane_kind)
            {
                lane.evidence_state = new_state;
                lane.waiver_ref = matches!(new_state, LaneEvidenceState::WaivedBounded)
                    .then(|| format!("waiver:{}:{}", short_locale(profile_id), lane_kind.token()));
            }
        }
        assemble(profiles)
    }
}

/// Returns the source-language text direction helper for a locale tag.
fn text_direction(locale: &str) -> TextDirection {
    TextDirection::for_locale(locale)
}

/// Trailing locale segment of a profile id, used to keep derived ids readable.
fn short_locale(profile_id: &str) -> String {
    profile_id
        .split(':')
        .nth(2)
        .unwrap_or(profile_id)
        .to_owned()
}

/// Recomputes one profile's per-lane causes, counts, gate, and effective claim.
fn finalize_profile(profile: &mut LocalizedProfileClaimRow) {
    let mut satisfied = 0;
    let mut stale_or_missing = 0;
    let mut failing = 0;
    let mut waived = 0;
    let mut affected: Vec<QualificationLaneKind> = Vec::new();
    let mut any_required_failing = false;
    let mut any_required_stale_missing = false;

    for lane in &mut profile.lane_results {
        lane.evidence_packet_ref = lane.lane_kind.evidence_packet_id().to_owned();
        lane.evidence_fixture_ref = lane.lane_kind.evidence_fixture_ref().to_owned();
        lane.narrow_cause = lane.evidence_state.narrow_cause();

        match lane.evidence_state {
            LaneEvidenceState::CurrentPassing => satisfied += 1,
            LaneEvidenceState::WaivedBounded => {
                satisfied += 1;
                waived += 1;
            }
            LaneEvidenceState::Stale | LaneEvidenceState::Missing => stale_or_missing += 1,
            LaneEvidenceState::CurrentFailing => failing += 1,
        }

        if lane.narrow_cause.is_some() {
            affected.push(lane.lane_kind);
            if lane.required_for_claim {
                if lane.evidence_state.is_failing() {
                    any_required_failing = true;
                } else {
                    any_required_stale_missing = true;
                }
            }
        }
    }

    affected.sort();
    affected.dedup();
    profile.affected_lane_kinds = affected;
    profile.satisfied_lane_count = satisfied;
    profile.stale_or_missing_lane_count = stale_or_missing;
    profile.failing_lane_count = failing;
    profile.waived_lane_count = waived;

    let (effective, gate, narrowed, blocks) = match profile.intended_claim_class {
        ProfileClaimClass::NotLocalized => (
            ProfileClaimClass::NotLocalized,
            MatrixGateState::Green,
            false,
            false,
        ),
        ProfileClaimClass::SourceLanguageFallbackOnly => (
            ProfileClaimClass::SourceLanguageFallbackOnly,
            MatrixGateState::Green,
            false,
            false,
        ),
        ProfileClaimClass::ClaimedLocalized => {
            if any_required_failing {
                (
                    ProfileClaimClass::SourceLanguageFallbackOnly,
                    MatrixGateState::Blocked,
                    true,
                    true,
                )
            } else if any_required_stale_missing {
                (
                    ProfileClaimClass::SourceLanguageFallbackOnly,
                    MatrixGateState::Narrowed,
                    true,
                    false,
                )
            } else {
                (
                    ProfileClaimClass::ClaimedLocalized,
                    MatrixGateState::Green,
                    false,
                    false,
                )
            }
        }
    };

    profile.effective_claim_class = effective;
    profile.gate_state = gate;
    profile.narrowed = narrowed;
    profile.blocks_promotion = blocks;
}

/// Builds the published known limits for every narrowed or blocked required lane.
fn build_known_limits(profiles: &[LocalizedProfileClaimRow]) -> Vec<KnownLimitRow> {
    let mut rows = Vec::new();
    for profile in profiles {
        if profile.intended_claim_class != ProfileClaimClass::ClaimedLocalized {
            continue;
        }
        for lane in &profile.lane_results {
            if !lane.required_for_claim {
                continue;
            }
            let Some(cause) = lane.narrow_cause else {
                continue;
            };
            rows.push(KnownLimitRow {
                limit_id: format!(
                    "known-limit:{}:{}",
                    short_locale(&profile.profile_id),
                    lane.lane_kind.token()
                ),
                profile_id_ref: profile.profile_id.clone(),
                lane_kind: lane.lane_kind,
                cause,
                gate_state: cause.gate_state(),
                summary: known_limit_summary(&profile.requested_locale, lane.lane_kind, cause),
                affected_surface_families: lane.lane_kind.affected_surface_families(),
                source_language_route_ref: profile.source_language_route_ref.clone(),
                published_to: vec![
                    ConsumerKind::HelpAbout,
                    ConsumerKind::ReleaseCenter,
                    ConsumerKind::Diagnostics,
                    ConsumerKind::SupportExport,
                ],
            });
        }
    }
    rows
}

/// Renders an export-safe known-limit summary; never embeds raw translated bodies.
fn known_limit_summary(
    locale: &str,
    lane: QualificationLaneKind,
    cause: LaneNarrowCause,
) -> String {
    let lane_phrase = match lane {
        QualificationLaneKind::Pseudolocalization => "pseudolocalization expansion",
        QualificationLaneKind::TextExpansion => "text-expansion budget",
        QualificationLaneKind::RtlBidi => "RTL and bidi rendering",
        QualificationLaneKind::ImeComposition => "IME composition",
        QualificationLaneKind::TranslatedHelpParity => "translated help and docs parity",
        QualificationLaneKind::LocalePackCompatibility => "locale-pack compatibility",
    };
    let cause_phrase = match cause {
        LaneNarrowCause::EvidenceFailing => "is failing",
        LaneNarrowCause::EvidenceStale => "is stale against the target build",
        LaneNarrowCause::EvidenceMissing => "is missing",
    };
    let outcome = match cause.gate_state() {
        MatrixGateState::Blocked => {
            "the localized claim is blocked from promotion until the lane passes again"
        }
        _ => "the localized claim is narrowed to source-language fallback on the affected surfaces",
    };
    format!("{locale}: {lane_phrase} evidence {cause_phrase}; {outcome}.")
}

/// Derives the summary posture from the profiles and known limits.
fn derive_summary(
    profiles: &[LocalizedProfileClaimRow],
    known_limits: &[KnownLimitRow],
) -> LocalizedClaimStatusSummary {
    let claimed_localized_profiles = profiles
        .iter()
        .filter(|p| p.effective_claim_class == ProfileClaimClass::ClaimedLocalized)
        .count();
    let narrowed_profiles = profiles
        .iter()
        .filter(|p| p.gate_state == MatrixGateState::Narrowed)
        .count();
    let blocked_profiles = profiles
        .iter()
        .filter(|p| p.gate_state == MatrixGateState::Blocked)
        .count();
    let not_localized_profiles = profiles
        .iter()
        .filter(|p| p.effective_claim_class == ProfileClaimClass::NotLocalized)
        .count();

    let lanes = || profiles.iter().flat_map(|p| p.lane_results.iter());
    let total_lane_results = lanes().count();
    let satisfied_lane_results = lanes()
        .filter(|l| l.evidence_state.satisfies_claim())
        .count();
    let stale_or_missing_lane_results = lanes()
        .filter(|l| {
            matches!(
                l.evidence_state,
                LaneEvidenceState::Stale | LaneEvidenceState::Missing
            )
        })
        .count();
    let failing_lane_results = lanes().filter(|l| l.evidence_state.is_failing()).count();
    let waived_lane_results = lanes()
        .filter(|l| matches!(l.evidence_state, LaneEvidenceState::WaivedBounded))
        .count();

    let promotion_state = if blocked_profiles == 0 {
        MatrixGateState::Green
    } else {
        MatrixGateState::Blocked
    };

    LocalizedClaimStatusSummary {
        total_profiles: profiles.len(),
        claimed_localized_profiles,
        narrowed_profiles,
        blocked_profiles,
        not_localized_profiles,
        total_lane_results,
        satisfied_lane_results,
        stale_or_missing_lane_results,
        failing_lane_results,
        waived_lane_results,
        published_known_limits: known_limits.len(),
        promotion_state,
    }
}

/// Assembles a packet from profile rows, deriving every gate, limit, and summary.
fn assemble(mut profiles: Vec<LocalizedProfileClaimRow>) -> LocalizedClaimStatusPacket {
    for profile in &mut profiles {
        finalize_profile(profile);
    }
    let known_limits = build_known_limits(&profiles);
    let summary = derive_summary(&profiles, &known_limits);

    LocalizedClaimStatusPacket {
        record_kind: LOCALIZED_CLAIM_STATUS_RECORD_KIND.to_owned(),
        schema_version: LOCALIZED_CLAIM_STATUS_SCHEMA_VERSION,
        packet_id: LOCALIZED_CLAIM_STATUS_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        release_channel: RELEASE_CHANNEL.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        source_contract_refs: seeded_source_contract_refs(),
        runtime_consumer_refs: seeded_runtime_consumer_refs(),
        evidence_lane_refs: seeded_evidence_lane_refs(),
        claimed_profiles: profiles,
        known_limits,
        consumption_bindings: seeded_consumption_bindings(),
        release_gate_rows: seeded_release_gates(),
        summary,
    }
}

/// Returns the seeded localized claim-status qualification packet.
pub fn seeded_localized_claim_status_packet() -> LocalizedClaimStatusPacket {
    assemble(seeded_profiles())
}

struct LaneSeed {
    lane_kind: QualificationLaneKind,
    evidence_state: LaneEvidenceState,
    detail: &'static str,
}

struct ProfileSeed {
    profile_id: &'static str,
    title: &'static str,
    requested_locale: &'static str,
    fallback_chain: &'static [&'static str],
    primary_pack_ref: &'static str,
    lanes: &'static [LaneSeed],
}

fn seeded_profiles() -> Vec<LocalizedProfileClaimRow> {
    use LaneEvidenceState as S;
    use QualificationLaneKind as L;

    // The flagship profile holds every evidence lane current and passing.
    const ES_LANES: &[LaneSeed] = &[
        LaneSeed {
            lane_kind: L::Pseudolocalization,
            evidence_state: S::CurrentPassing,
            detail: "Dense-surface pseudoloc passes for es-MX with no clipping or overflow.",
        },
        LaneSeed {
            lane_kind: L::TextExpansion,
            evidence_state: S::CurrentPassing,
            detail: "Translated strings stay within the declared expansion budget.",
        },
        LaneSeed {
            lane_kind: L::RtlBidi,
            evidence_state: S::CurrentPassing,
            detail: "LTR profile; directional chrome and literal tokens render correctly.",
        },
        LaneSeed {
            lane_kind: L::ImeComposition,
            evidence_state: S::CurrentPassing,
            detail: "IME composition is never silently committed, cancelled, or occluded.",
        },
        LaneSeed {
            lane_kind: L::TranslatedHelpParity,
            evidence_state: S::CurrentPassing,
            detail: "Translated help, tours, and citations preserve stable anchors.",
        },
        LaneSeed {
            lane_kind: L::LocalePackCompatibility,
            evidence_state: S::CurrentPassing,
            detail: "Locale pack is signed and compatible with the target build.",
        },
    ];

    // The Japanese profile is narrowed: translated-help parity proof is stale.
    const JA_LANES: &[LaneSeed] = &[
        LaneSeed {
            lane_kind: L::Pseudolocalization,
            evidence_state: S::CurrentPassing,
            detail: "Dense-surface pseudoloc passes for ja-JP.",
        },
        LaneSeed {
            lane_kind: L::TextExpansion,
            evidence_state: S::CurrentPassing,
            detail: "CJK strings stay within the declared expansion budget.",
        },
        LaneSeed {
            lane_kind: L::RtlBidi,
            evidence_state: S::CurrentPassing,
            detail: "LTR profile; directional chrome and literal tokens render correctly.",
        },
        LaneSeed {
            lane_kind: L::ImeComposition,
            evidence_state: S::CurrentPassing,
            detail: "Japanese IME composition is preserved across dense churn.",
        },
        LaneSeed {
            lane_kind: L::TranslatedHelpParity,
            evidence_state: S::Stale,
            detail: "Translated help and docs parity proof is stale against the target build.",
        },
        LaneSeed {
            lane_kind: L::LocalePackCompatibility,
            evidence_state: S::CurrentPassing,
            detail: "Locale pack is signed and compatible with the target build.",
        },
    ];

    // The Arabic profile is narrowed: RTL/bidi rendering proof is stale.
    const AR_LANES: &[LaneSeed] = &[
        LaneSeed {
            lane_kind: L::Pseudolocalization,
            evidence_state: S::CurrentPassing,
            detail: "Dense-surface pseudoloc passes for ar-SA.",
        },
        LaneSeed {
            lane_kind: L::TextExpansion,
            evidence_state: S::CurrentPassing,
            detail: "Translated strings stay within the declared expansion budget.",
        },
        LaneSeed {
            lane_kind: L::RtlBidi,
            evidence_state: S::Stale,
            detail: "RTL and bidi mirroring proof is stale against the target build.",
        },
        LaneSeed {
            lane_kind: L::ImeComposition,
            evidence_state: S::CurrentPassing,
            detail: "IME composition is preserved across dense churn.",
        },
        LaneSeed {
            lane_kind: L::TranslatedHelpParity,
            evidence_state: S::CurrentPassing,
            detail: "Translated help, tours, and citations preserve stable anchors.",
        },
        LaneSeed {
            lane_kind: L::LocalePackCompatibility,
            evidence_state: S::CurrentPassing,
            detail: "Locale pack is signed and compatible with the target build.",
        },
    ];

    const SEEDS: &[ProfileSeed] = &[
        ProfileSeed {
            profile_id: "profile:m5:es-MX:desktop",
            title: "Spanish (Mexico) desktop",
            requested_locale: "es-MX",
            fallback_chain: &["es-MX", "es", "en-US"],
            primary_pack_ref: "locale-pack:core:es-mx:stable",
            lanes: ES_LANES,
        },
        ProfileSeed {
            profile_id: "profile:m5:ja-JP:desktop",
            title: "Japanese (Japan) desktop",
            requested_locale: "ja-JP",
            fallback_chain: &["ja-JP", "ja", "en-US"],
            primary_pack_ref: "locale-pack:core:ja-jp:stable",
            lanes: JA_LANES,
        },
        ProfileSeed {
            profile_id: "profile:m5:ar-SA:desktop",
            title: "Arabic (Saudi Arabia) desktop",
            requested_locale: "ar-SA",
            fallback_chain: &["ar-SA", "ar", "en-US"],
            primary_pack_ref: "locale-pack:core:ar-sa:stable",
            lanes: AR_LANES,
        },
    ];

    SEEDS
        .iter()
        .map(|seed| {
            let lane_results = seed
                .lanes
                .iter()
                .map(|lane| ProfileLaneResult {
                    lane_kind: lane.lane_kind,
                    evidence_state: lane.evidence_state,
                    evidence_packet_ref: lane.lane_kind.evidence_packet_id().to_owned(),
                    evidence_fixture_ref: lane.lane_kind.evidence_fixture_ref().to_owned(),
                    required_for_claim: true,
                    detail: lane.detail.to_owned(),
                    waiver_ref: None,
                    narrow_cause: None,
                })
                .collect();
            LocalizedProfileClaimRow {
                profile_id: seed.profile_id.to_owned(),
                title: seed.title.to_owned(),
                requested_locale: seed.requested_locale.to_owned(),
                source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
                text_direction: text_direction(seed.requested_locale),
                fallback_chain: seed
                    .fallback_chain
                    .iter()
                    .map(|l| (*l).to_owned())
                    .collect(),
                primary_pack_ref: seed.primary_pack_ref.to_owned(),
                intended_claim_class: ProfileClaimClass::ClaimedLocalized,
                // Derived fields are filled by `finalize_profile`.
                effective_claim_class: ProfileClaimClass::ClaimedLocalized,
                gate_state: MatrixGateState::Green,
                narrowed: false,
                blocks_promotion: false,
                lane_results,
                satisfied_lane_count: 0,
                stale_or_missing_lane_count: 0,
                failing_lane_count: 0,
                waived_lane_count: 0,
                affected_lane_kinds: Vec::new(),
                source_language_route_ref: "route:profile:source-language:open".to_owned(),
                visible_in_settings: true,
                visible_in_help_about: true,
                visible_in_release_center: true,
                visible_in_diagnostics: true,
                visible_in_support_export: true,
                non_blocking_core_use: true,
            }
        })
        .collect()
}

fn seeded_source_contract_refs() -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "architecture_localization".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#23.3.1".to_owned(),
        ),
        (
            "architecture_verification_lanes".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#27.23".to_owned(),
        ),
        (
            "locale_pack_lifecycle".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-df".to_owned(),
        ),
        (
            "design_localization_governance".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#8.10".to_owned(),
        ),
        (
            "localized_profile_matrix".to_owned(),
            "fixtures/i18n/m5-surface-inventory/manifest.json".to_owned(),
        ),
    ])
}

fn seeded_runtime_consumer_refs() -> Vec<String> {
    strings(&[
        "crates/aureline-i18n",
        "crates/aureline-release",
        "crates/aureline-shell",
        "crates/aureline-cli",
        "crates/aureline-doctor",
        "crates/aureline-support",
        "crates/aureline-docs",
    ])
}

fn seeded_evidence_lane_refs() -> BTreeMap<String, String> {
    QualificationLaneKind::all()
        .into_iter()
        .map(|lane| (serde_lane_key(lane), lane.evidence_packet_id().to_owned()))
        .collect()
}

/// Snake-case key for a lane kind, matching its serde representation.
fn serde_lane_key(lane: QualificationLaneKind) -> String {
    lane.token().to_owned()
}

fn seeded_consumption_bindings() -> Vec<ConsumptionBindingRow> {
    vec![
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::ReleaseCenter,
            consumer_ref: "crates/aureline-release".to_owned(),
            ingests_summary:
                "Gates localized-profile promotion and ingests known limits and downgrade reasons."
                    .to_owned(),
            consumed_fields: strings(&["summary", "claimed_profiles", "known_limits"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::HelpAbout,
            consumer_ref: "crates/aureline-shell".to_owned(),
            ingests_summary:
                "Discloses localized-claim status, known limits, and downgrade reasons in About."
                    .to_owned(),
            consumed_fields: strings(&["claimed_profiles", "known_limits"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::Diagnostics,
            consumer_ref: "crates/aureline-doctor".to_owned(),
            ingests_summary:
                "Reports effective localized claim and the evidence lane that narrowed it."
                    .to_owned(),
            consumed_fields: strings(&["claimed_profiles", "known_limits"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::ClaimNarrowing,
            consumer_ref: "crates/aureline-i18n".to_owned(),
            ingests_summary:
                "Auto-narrows locale-bearing claims when a required evidence lane is stale or failing."
                    .to_owned(),
            consumed_fields: strings(&["claimed_profiles", "summary"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::SupportExport,
            consumer_ref: "crates/aureline-support".to_owned(),
            ingests_summary:
                "Projects localized-claim status and known limits into metadata-only support export."
                    .to_owned(),
            consumed_fields: strings(&["claimed_profiles", "known_limits", "summary"]),
        },
    ]
}

fn seeded_release_gates() -> Vec<ProfileReleaseGateRow> {
    let command = "cargo test -p aureline-i18n --test localized_profile_qualification --locked";
    [
        (
            "release-gate:per-profile-qualification",
            "per_profile_qualification",
        ),
        ("release-gate:claim-auto-narrowing", "claim_auto_narrowing"),
        (
            "release-gate:known-limits-published",
            "known_limits_published",
        ),
        (
            "release-gate:evidence-lane-freshness",
            "evidence_lane_freshness",
        ),
        (
            "release-gate:downstream-consumption",
            "downstream_consumption",
        ),
    ]
    .into_iter()
    .map(|(row_id, proof_kind)| ProfileReleaseGateRow {
        row_id: row_id.to_owned(),
        proof_kind: proof_kind.to_owned(),
        command: command.to_owned(),
        fixture_refs: vec![LOCALIZED_CLAIM_STATUS_FIXTURE_REF.to_owned()],
        artifact_refs: vec![
            "artifacts/i18n/m5-localized-profile-qualification/report.md".to_owned(),
            "docs/m5/localized-profile-known-limits.md".to_owned(),
        ],
        required_for_claimed_profiles: true,
        gate_state: MatrixGateState::Green,
    })
    .collect()
}

fn validate_profiles(
    packet: &LocalizedClaimStatusPacket,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut ids = BTreeSet::new();
    let required_lanes: BTreeSet<QualificationLaneKind> =
        QualificationLaneKind::all().into_iter().collect();

    for profile in &packet.claimed_profiles {
        if !ids.insert(profile.profile_id.clone()) {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "duplicate profile id",
            ));
        }

        // Re-derive the profile from its lanes and require stored fields to match.
        let mut recomputed = profile.clone();
        finalize_profile(&mut recomputed);
        if recomputed.effective_claim_class != profile.effective_claim_class
            || recomputed.gate_state != profile.gate_state
            || recomputed.narrowed != profile.narrowed
            || recomputed.blocks_promotion != profile.blocks_promotion
            || recomputed.satisfied_lane_count != profile.satisfied_lane_count
            || recomputed.stale_or_missing_lane_count != profile.stale_or_missing_lane_count
            || recomputed.failing_lane_count != profile.failing_lane_count
            || recomputed.waived_lane_count != profile.waived_lane_count
            || recomputed.affected_lane_kinds != profile.affected_lane_kinds
            || recomputed.lane_results != profile.lane_results
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile claim drifted from its evidence lanes",
            ));
        }

        let present: BTreeSet<QualificationLaneKind> =
            profile.lane_results.iter().map(|l| l.lane_kind).collect();
        if !required_lanes.is_subset(&present) {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile is missing a required evidence lane",
            ));
        }

        if profile.fallback_chain.first() != Some(&profile.requested_locale)
            || profile.fallback_chain.last() != Some(&profile.source_language_locale)
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile fallback chain must run requested locale to source language",
            ));
        }

        if !profile.visible_in_settings
            || !profile.visible_in_help_about
            || !profile.visible_in_release_center
            || !profile.visible_in_diagnostics
            || !profile.visible_in_support_export
            || profile.source_language_route_ref.trim().is_empty()
            || !profile.non_blocking_core_use
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile must be inspectable, source-language reachable, and non-blocking",
            ));
        }

        if profile.effective_claim_class == ProfileClaimClass::ClaimedLocalized
            && (profile.narrowed || profile.blocks_promotion)
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "a profile cannot stay claimed localized while narrowed or blocked",
            ));
        }
    }
}

fn validate_known_limits(
    packet: &LocalizedClaimStatusPacket,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let expected = build_known_limits(&packet.claimed_profiles);
    if packet.known_limits != expected {
        findings.push(LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "known limits drifted from the narrowed and blocked evidence lanes",
        ));
    }

    for limit in &packet.known_limits {
        if packet.profile(&limit.profile_id_ref).is_none() {
            findings.push(LocalePackValidationFinding::new(
                limit.limit_id.clone(),
                "known limit references an unknown profile",
            ));
        }
        if limit.gate_state != limit.cause.gate_state() {
            findings.push(LocalePackValidationFinding::new(
                limit.limit_id.clone(),
                "known limit gate state does not match its downgrade cause",
            ));
        }
        if limit.affected_surface_families.is_empty()
            || limit.summary.trim().is_empty()
            || limit.source_language_route_ref.trim().is_empty()
        {
            findings.push(LocalePackValidationFinding::new(
                limit.limit_id.clone(),
                "known limit must cite affected surfaces, a summary, and a source-language route",
            ));
        }
        if !limit.published_to.contains(&ConsumerKind::HelpAbout)
            || !limit.published_to.contains(&ConsumerKind::ReleaseCenter)
        {
            findings.push(LocalePackValidationFinding::new(
                limit.limit_id.clone(),
                "known limit must be published to Help/About and the release center",
            ));
        }
    }
}

fn validate_evidence_lane_refs(
    packet: &LocalizedClaimStatusPacket,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    for lane in QualificationLaneKind::all() {
        match packet.evidence_lane_refs.get(&serde_lane_key(lane)) {
            Some(packet_ref) if packet_ref == lane.evidence_packet_id() => {}
            _ => findings.push(LocalePackValidationFinding::new(
                packet.packet_id.clone(),
                "evidence lane refs must bind every lane to its upstream truth packet",
            )),
        }
    }
}

fn validate_consumption_bindings(
    bindings: &[ConsumptionBindingRow],
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut kinds = BTreeSet::new();
    for binding in bindings {
        kinds.insert(binding.consumer_kind);
        if binding.consumer_ref.trim().is_empty()
            || binding.ingests_summary.trim().is_empty()
            || binding.consumed_fields.is_empty()
        {
            findings.push(LocalePackValidationFinding::new(
                binding.consumer_ref.clone(),
                "consumption binding must cite a consumer, summary, and consumed fields",
            ));
        }
    }

    for required in [
        ConsumerKind::ReleaseCenter,
        ConsumerKind::HelpAbout,
        ConsumerKind::Diagnostics,
        ConsumerKind::ClaimNarrowing,
        ConsumerKind::SupportExport,
    ] {
        if !kinds.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALIZED_CLAIM_STATUS_PACKET_ID,
                format!("consumption bindings are missing {required:?}"),
            ));
        }
    }
}

fn validate_release_gates(
    gates: &[ProfileReleaseGateRow],
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut proof_kinds = BTreeSet::new();
    for gate in gates {
        proof_kinds.insert(gate.proof_kind.clone());
        if !gate.required_for_claimed_profiles
            || gate.gate_state != MatrixGateState::Green
            || gate.command.trim().is_empty()
            || gate.fixture_refs.is_empty()
            || gate.artifact_refs.is_empty()
        {
            findings.push(LocalePackValidationFinding::new(
                gate.row_id.clone(),
                "release gate row must be green and proof-backed for claimed profiles",
            ));
        }
    }

    for required in [
        "per_profile_qualification",
        "claim_auto_narrowing",
        "known_limits_published",
        "evidence_lane_freshness",
        "downstream_consumption",
    ] {
        if !proof_kinds.contains(required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALIZED_CLAIM_STATUS_PACKET_ID,
                format!("release gates are missing {required}"),
            ));
        }
    }
}

fn validate_summary(
    packet: &LocalizedClaimStatusPacket,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let expected = derive_summary(&packet.claimed_profiles, &packet.known_limits);
    if packet.summary != expected {
        findings.push(LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "localized claim status summary drifted from row state",
        ));
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_owned()).collect()
}

#[cfg(test)]
mod tests;
