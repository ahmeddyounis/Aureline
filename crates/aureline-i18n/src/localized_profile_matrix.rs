//! Localized-profile matrix, surface inventory, and fallback-chain truth.
//!
//! This module freezes the localization delivery model into one inspectable
//! register: which localized profiles exist, which product surfaces they cover,
//! the fallback chain each profile walks, and the proof each claim rests on. It
//! answers, per surface, whether the active locale renders localized text, falls
//! back to source language only, or is explicitly not localized.
//!
//! Claims cannot outrun their evidence. A profile that intends to claim
//! localized support is narrowed automatically to source-language fallback when
//! a required surface lacks a compatible pack or current proof. The narrowed
//! posture is derived from the rows, so release, Help/About, diagnostics, and
//! claim-narrowing tooling ingest one truth instead of cloning status prose.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    LocalePackValidationFinding, MachineOutputLocaleClass, GENERATED_AT, SOURCE_LANGUAGE_LOCALE,
};

/// Schema version for the localized-profile matrix packet.
pub const LOCALIZED_PROFILE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LocalizedProfileMatrixPacket`].
pub const LOCALIZED_PROFILE_MATRIX_RECORD_KIND: &str = "localized_profile_matrix_packet";

/// Stable packet id for the seeded localized-profile matrix.
pub const LOCALIZED_PROFILE_MATRIX_PACKET_ID: &str =
    "i18n:localized-profile-matrix:surface-inventory:v1";

/// Fixture path for the seeded localized-profile matrix packet.
pub const LOCALIZED_PROFILE_MATRIX_FIXTURE_REF: &str =
    "fixtures/i18n/m5-surface-inventory/manifest.json";

/// Localization posture for one product surface under one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceLocalizationState {
    /// The surface renders localized text for the requested locale.
    Localized,
    /// The surface falls back to source language while disclosing the fallback.
    SourceLanguageFallbackOnly,
    /// The surface makes no localization claim and is rendered in source language by design.
    NotLocalized,
}

/// Roll-up claim posture for one localized profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileClaimClass {
    /// The profile claims stable localized support across its covered surfaces.
    ClaimedLocalized,
    /// The profile is a non-source locale currently served by source-language fallback.
    SourceLanguageFallbackOnly,
    /// The profile is explicitly not localized.
    NotLocalized,
}

/// Compatibility posture for the locale pack backing a coverage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackCompatibilityState {
    /// Pack is present, signed, and compatible with the target build.
    Compatible,
    /// Pack is present but incompatible with the target build.
    Incompatible,
    /// Pack is present but its compatibility or signature is unverified.
    Unverified,
    /// No pack is available for the surface and locale.
    Missing,
    /// No pack is required because the surface is not localized.
    NotApplicable,
}

impl PackCompatibilityState {
    /// Reports whether the pack may back a localized claim.
    fn may_localize(self) -> bool {
        matches!(self, PackCompatibilityState::Compatible)
    }
}

/// Freshness posture for the proof backing a coverage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessState {
    /// Proof is current for the target build.
    Current,
    /// Proof exists but is stale against the target build.
    Stale,
    /// Proof is missing.
    Missing,
    /// Proof is not required because the surface is not localized.
    NotRequired,
}

impl EvidenceFreshnessState {
    /// Reports whether the proof may back a localized claim.
    fn may_localize(self) -> bool {
        matches!(self, EvidenceFreshnessState::Current)
    }
}

/// Reason a localized claim narrowed to source-language fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimNarrowReason {
    /// The backing locale pack is missing.
    PackMissing,
    /// The backing locale pack is incompatible with the target build.
    PackIncompatible,
    /// The backing locale pack compatibility or signature is unverified.
    PackUnverified,
    /// Required proof is missing.
    EvidenceMissing,
    /// Required proof is stale.
    EvidenceStale,
}

/// Promotion state for a coverage row, release gate, or matrix summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixGateState {
    /// The row holds its claim with current, compatible proof.
    Green,
    /// The row narrowed its localized claim to source-language fallback.
    Narrowed,
    /// The row blocks the claim.
    Blocked,
}

/// Localizable product surface family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalizableSurfaceFamily {
    /// Shell chrome: title bars, status areas, switcher, and panel labels.
    ShellChrome,
    /// Command palette, menus, and command labels.
    CommandPalette,
    /// Help, docs, and docs-browser prose.
    HelpAndDocs,
    /// CLI usage and Doctor human help.
    CliAndDoctor,
    /// Notifications and toasts.
    Notifications,
    /// Extension-contributed UI inside an extension namespace.
    ExtensionContributedUi,
    /// Companion and browser-handoff surfaces.
    CompanionHandoff,
    /// Notebook editing and execution surfaces.
    NotebookTooling,
    /// Data and API tooling surfaces.
    DataAndApiTooling,
    /// Guided learning, tours, and exercises.
    GuidedLearning,
    /// Support, recovery, and Doctor remediation flows.
    SupportFlows,
    /// Release center and About surfaces.
    ReleaseAndAbout,
}

/// Machine-stable identifier kind a surface must preserve across locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableElementKind {
    /// Canonical command id.
    CommandId,
    /// Canonical command verb.
    CanonicalVerb,
    /// Keybinding path token.
    KeybindingPath,
    /// CLI subcommand name.
    CliSubcommandName,
    /// CLI flag literal.
    CliFlagLiteral,
    /// JSON output key.
    JsonKey,
    /// Schema id.
    SchemaId,
    /// Setting id.
    SettingId,
    /// Policy id.
    PolicyId,
    /// Telemetry key.
    TelemetryKey,
    /// Error code or finding id.
    ErrorCodeOrFindingId,
    /// Docs anchor id.
    DocsAnchorId,
    /// Citation anchor id.
    CitationAnchorId,
    /// Recovery route or URL id.
    RecoveryRouteId,
    /// Notification id.
    NotificationId,
    /// Extension namespace id.
    ExtensionNamespaceId,
    /// Host-owned stable identifier.
    HostStableIdentifier,
}

/// Downstream consumer that must ingest the matrix instead of cloning status text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerKind {
    /// Release center gating of localized profiles.
    ReleaseCenter,
    /// Help and About localized-scope disclosure.
    HelpAbout,
    /// Diagnostics and Doctor localization reporting.
    Diagnostics,
    /// Claim-narrowing tooling.
    ClaimNarrowing,
    /// Support export projection.
    SupportExport,
    /// Docs-browser surface inventory consumption.
    DocsBrowser,
}

/// One row of the frozen localizable surface inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceInventoryRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Human title of the surface.
    pub title: String,
    /// Surface family.
    pub surface_family: LocalizableSurfaceFamily,
    /// Locale pack that owns translation for this surface.
    pub owning_pack_ref: String,
    /// Machine-stable identifier kinds the surface preserves across locales.
    pub stable_element_kinds: Vec<StableElementKind>,
    /// Human description of what may localize on this surface.
    pub localizable_element_summary: String,
    /// Same-surface source-language route.
    pub source_language_route_ref: String,
    /// Machine-output localization posture for the surface.
    pub machine_output_locale_class: MachineOutputLocaleClass,
}

/// One localized profile with its fallback chain and roll-up claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedProfileRow {
    /// Stable profile id.
    pub profile_id: String,
    /// Human title of the profile.
    pub title: String,
    /// Requested locale for the profile.
    pub requested_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Primary locale pack backing the profile.
    pub primary_pack_ref: String,
    /// Supporting locale packs (docs overlay, CLI, extension).
    pub supporting_pack_refs: Vec<String>,
    /// Claim the profile intends to make before narrowing.
    pub intended_claim_class: ProfileClaimClass,
    /// Effective claim after narrowing against evidence and pack compatibility.
    pub claim_class: ProfileClaimClass,
    /// Whether the effective claim narrowed below the intended claim.
    pub narrowed: bool,
    /// Reason for narrowing, when narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_reason: Option<ClaimNarrowReason>,
    /// Same-surface source-language route for the profile.
    pub source_language_route_ref: String,
    /// Whether Settings exposes this profile.
    pub visible_in_settings: bool,
    /// Whether diagnostics exposes this profile.
    pub visible_in_diagnostics: bool,
    /// Whether support export exposes this profile.
    pub visible_in_support_export: bool,
    /// Whether Help/About exposes this profile.
    pub visible_in_help_about: bool,
    /// Whether missing or narrowed localization keeps local product use available.
    pub non_blocking_core_use: bool,
}

/// One profile-by-surface coverage cell of the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSurfaceCoverageRow {
    /// Stable coverage row id.
    pub row_id: String,
    /// Profile this cell belongs to.
    pub profile_id_ref: String,
    /// Surface this cell covers.
    pub surface_id_ref: String,
    /// Localization state the profile intends for this surface.
    pub claimed_localization_state: SurfaceLocalizationState,
    /// Locale pack backing this cell.
    pub pack_ref: String,
    /// Pack compatibility posture for this cell.
    pub pack_compatibility: PackCompatibilityState,
    /// Proof refs backing this cell.
    pub evidence_refs: Vec<String>,
    /// Freshness of the proof backing this cell.
    pub evidence_freshness: EvidenceFreshnessState,
    /// Whether this cell gates the profile's localized claim.
    pub required_for_claim: bool,
    /// Effective localization state after narrowing.
    pub effective_localization_state: SurfaceLocalizationState,
    /// Whether this cell narrowed below its claimed state.
    pub narrowed: bool,
    /// Reason for narrowing, when narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_reason: Option<ClaimNarrowReason>,
    /// Gate state derived for this cell.
    pub gate_state: MatrixGateState,
}

/// Binding describing how a downstream surface consumes this matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumptionBindingRow {
    /// Consumer kind.
    pub consumer_kind: ConsumerKind,
    /// Consumer crate or surface ref.
    pub consumer_ref: String,
    /// Human description of what the consumer ingests.
    pub ingests_summary: String,
    /// Packet fields the consumer reads.
    pub consumed_fields: Vec<String>,
}

/// Release-gate proof row for the localized-profile matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileReleaseGateRow {
    /// Stable row id.
    pub row_id: String,
    /// Proof kind such as `surface_inventory_frozen` or `claim_auto_narrowing`.
    pub proof_kind: String,
    /// Exact command for local verification.
    pub command: String,
    /// Fixture refs consumed by the proof.
    pub fixture_refs: Vec<String>,
    /// Artifact refs produced or reviewed by the proof.
    pub artifact_refs: Vec<String>,
    /// Whether the row gates claimed localized profiles.
    pub required_for_claimed_profiles: bool,
    /// Gate state for this proof.
    pub gate_state: MatrixGateState,
}

/// Summary posture for the localized-profile matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedProfileMatrixSummary {
    /// Number of inventoried surfaces.
    pub total_surfaces: usize,
    /// Number of profiles.
    pub total_profiles: usize,
    /// Number of profiles claiming localized support.
    pub claimed_localized_profiles: usize,
    /// Number of profiles served by source-language fallback only.
    pub source_language_fallback_profiles: usize,
    /// Number of explicitly non-localized profiles.
    pub not_localized_profiles: usize,
    /// Number of profiles whose claim narrowed below their intent.
    pub narrowed_profiles: usize,
    /// Number of coverage cells rendering localized text.
    pub localized_surface_cells: usize,
    /// Number of coverage cells served by source-language fallback only.
    pub source_language_fallback_cells: usize,
    /// Number of explicitly non-localized coverage cells.
    pub not_localized_cells: usize,
    /// Number of coverage cells that narrowed below their claimed state.
    pub narrowed_cells: usize,
    /// Number of blocked rows.
    pub blocked_rows: usize,
    /// Overall promotion state.
    pub promotion_state: MatrixGateState,
}

/// Localized-profile matrix, surface inventory, and fallback-chain truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedProfileMatrixPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contracts that govern this packet.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this packet.
    pub runtime_consumer_refs: Vec<String>,
    /// Frozen localizable surface inventory.
    pub surface_inventory: Vec<SurfaceInventoryRow>,
    /// Localized profiles with fallback chains and roll-up claims.
    pub localized_profiles: Vec<LocalizedProfileRow>,
    /// Profile-by-surface coverage cells.
    pub profile_surface_coverage: Vec<ProfileSurfaceCoverageRow>,
    /// Downstream consumption bindings.
    pub consumption_bindings: Vec<ConsumptionBindingRow>,
    /// Release-gated proof rows.
    pub release_gate_rows: Vec<ProfileReleaseGateRow>,
    /// Summary posture derived from the rows.
    pub summary: LocalizedProfileMatrixSummary,
}

impl LocalizedProfileMatrixPacket {
    /// Validates the matrix, surface inventory, fallback chains, and narrowing.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != LOCALIZED_PROFILE_MATRIX_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "localized profile matrix record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALIZED_PROFILE_MATRIX_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "localized profile matrix schema_version is unsupported",
            ));
        }

        let surface_ids = validate_surface_inventory(&self.surface_inventory, &mut findings);
        let profile_ids = validate_profiles(&self.localized_profiles, &mut findings);
        validate_coverage(
            &self.profile_surface_coverage,
            &surface_ids,
            &profile_ids,
            &mut findings,
        );
        validate_profile_narrowing(self, &mut findings);
        validate_consumption_bindings(&self.consumption_bindings, &mut findings);
        validate_release_gates(&self.release_gate_rows, &mut findings);
        validate_summary(self, &mut findings);

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Returns the effective localization state for a profile-by-surface pair.
    ///
    /// This is the field downstream tooling reads to answer whether a surface is
    /// localized, source-language fallback only, or not localized for a profile.
    pub fn effective_state(
        &self,
        profile_id: &str,
        surface_id: &str,
    ) -> Option<SurfaceLocalizationState> {
        self.profile_surface_coverage
            .iter()
            .find(|row| row.profile_id_ref == profile_id && row.surface_id_ref == surface_id)
            .map(|row| row.effective_localization_state)
    }
}

/// Derives the effective state, narrowing, reason, and gate for a coverage cell.
fn derive_cell(
    claimed: SurfaceLocalizationState,
    pack: PackCompatibilityState,
    freshness: EvidenceFreshnessState,
) -> (
    SurfaceLocalizationState,
    bool,
    Option<ClaimNarrowReason>,
    MatrixGateState,
) {
    match claimed {
        SurfaceLocalizationState::NotLocalized => (
            SurfaceLocalizationState::NotLocalized,
            false,
            None,
            MatrixGateState::Green,
        ),
        SurfaceLocalizationState::SourceLanguageFallbackOnly => (
            SurfaceLocalizationState::SourceLanguageFallbackOnly,
            false,
            None,
            MatrixGateState::Green,
        ),
        SurfaceLocalizationState::Localized => {
            if pack.may_localize() && freshness.may_localize() {
                (
                    SurfaceLocalizationState::Localized,
                    false,
                    None,
                    MatrixGateState::Green,
                )
            } else {
                (
                    SurfaceLocalizationState::SourceLanguageFallbackOnly,
                    true,
                    Some(narrow_reason(pack, freshness)),
                    MatrixGateState::Narrowed,
                )
            }
        }
    }
}

/// Picks the dominant narrowing reason, preferring pack faults over evidence faults.
fn narrow_reason(
    pack: PackCompatibilityState,
    freshness: EvidenceFreshnessState,
) -> ClaimNarrowReason {
    match pack {
        PackCompatibilityState::Missing => ClaimNarrowReason::PackMissing,
        PackCompatibilityState::Incompatible => ClaimNarrowReason::PackIncompatible,
        PackCompatibilityState::Unverified => ClaimNarrowReason::PackUnverified,
        PackCompatibilityState::Compatible | PackCompatibilityState::NotApplicable => {
            match freshness {
                EvidenceFreshnessState::Missing => ClaimNarrowReason::EvidenceMissing,
                EvidenceFreshnessState::Stale => ClaimNarrowReason::EvidenceStale,
                EvidenceFreshnessState::Current | EvidenceFreshnessState::NotRequired => {
                    ClaimNarrowReason::EvidenceMissing
                }
            }
        }
    }
}

/// Derives the effective claim, narrowing, and reason for a profile from its cells.
fn derive_profile_claim(
    intended: ProfileClaimClass,
    cells: &[&ProfileSurfaceCoverageRow],
) -> (ProfileClaimClass, bool, Option<ClaimNarrowReason>) {
    let required_localized: Vec<&&ProfileSurfaceCoverageRow> = cells
        .iter()
        .filter(|cell| {
            cell.claimed_localization_state == SurfaceLocalizationState::Localized
                && cell.required_for_claim
        })
        .collect();
    let any_localized_effective = cells
        .iter()
        .any(|cell| cell.effective_localization_state == SurfaceLocalizationState::Localized);
    let any_fallback_effective = cells.iter().any(|cell| {
        cell.effective_localization_state == SurfaceLocalizationState::SourceLanguageFallbackOnly
    });

    let derived = if !required_localized.is_empty()
        && required_localized
            .iter()
            .all(|cell| cell.effective_localization_state == SurfaceLocalizationState::Localized)
    {
        ProfileClaimClass::ClaimedLocalized
    } else if any_localized_effective || any_fallback_effective {
        ProfileClaimClass::SourceLanguageFallbackOnly
    } else {
        ProfileClaimClass::NotLocalized
    };

    let narrowed = derived != intended;
    let reason = if narrowed {
        dominant_profile_reason(cells)
    } else {
        None
    };
    (derived, narrowed, reason)
}

/// Picks the dominant narrowing reason across a profile's narrowed cells.
fn dominant_profile_reason(cells: &[&ProfileSurfaceCoverageRow]) -> Option<ClaimNarrowReason> {
    const PRIORITY: [ClaimNarrowReason; 5] = [
        ClaimNarrowReason::PackMissing,
        ClaimNarrowReason::PackIncompatible,
        ClaimNarrowReason::PackUnverified,
        ClaimNarrowReason::EvidenceMissing,
        ClaimNarrowReason::EvidenceStale,
    ];
    let observed: BTreeSet<ClaimNarrowReason> =
        cells.iter().filter_map(|cell| cell.narrow_reason).collect();
    PRIORITY
        .into_iter()
        .find(|reason| observed.contains(reason))
}

/// Returns the seeded localized-profile matrix packet.
pub fn seeded_localized_profile_matrix_packet() -> LocalizedProfileMatrixPacket {
    let surface_inventory = seeded_surface_inventory();
    let profile_surface_coverage = seeded_coverage();
    let localized_profiles = seeded_profiles(&profile_surface_coverage);
    let consumption_bindings = seeded_consumption_bindings();
    let release_gate_rows = seeded_release_gates();
    let summary = derive_summary(
        &surface_inventory,
        &localized_profiles,
        &profile_surface_coverage,
        &release_gate_rows,
    );

    LocalizedProfileMatrixPacket {
        record_kind: LOCALIZED_PROFILE_MATRIX_RECORD_KIND.to_owned(),
        schema_version: LOCALIZED_PROFILE_MATRIX_SCHEMA_VERSION,
        packet_id: LOCALIZED_PROFILE_MATRIX_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_contract_refs: BTreeMap::from([
            (
                "architecture_localization".to_owned(),
                ".t2/docs/Aureline_Technical_Architecture_Document.md#23.3.1".to_owned(),
            ),
            (
                "architecture_verification_lanes".to_owned(),
                ".t2/docs/Aureline_Technical_Architecture_Document.md#27.23".to_owned(),
            ),
            (
                "localization_governance_matrix".to_owned(),
                ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-df".to_owned(),
            ),
            (
                "locale_surface_matrix".to_owned(),
                "artifacts/i18n/locale_surface_matrix.yaml".to_owned(),
            ),
            (
                "stable_locale_lifecycle_parity".to_owned(),
                "fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json"
                    .to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            "crates/aureline-i18n".to_owned(),
            "crates/aureline-shell".to_owned(),
            "crates/aureline-docs".to_owned(),
            "crates/aureline-cli".to_owned(),
            "crates/aureline-extensions".to_owned(),
            "crates/aureline-companion".to_owned(),
            "crates/aureline-support".to_owned(),
            "crates/aureline-release".to_owned(),
        ],
        surface_inventory,
        localized_profiles,
        profile_surface_coverage,
        consumption_bindings,
        release_gate_rows,
        summary,
    }
}

struct SurfaceSeed {
    surface_id: &'static str,
    title: &'static str,
    surface_family: LocalizableSurfaceFamily,
    owning_pack_ref: &'static str,
    stable_element_kinds: &'static [StableElementKind],
    localizable_element_summary: &'static str,
    source_language_route_ref: &'static str,
    machine_output_locale_class: MachineOutputLocaleClass,
}

fn seeded_surface_inventory() -> Vec<SurfaceInventoryRow> {
    use LocalizableSurfaceFamily as F;
    use MachineOutputLocaleClass as M;
    use StableElementKind as S;

    const SEEDS: &[SurfaceSeed] = &[
        SurfaceSeed {
            surface_id: "surface:shell:chrome",
            title: "Shell chrome",
            surface_family: F::ShellChrome,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[S::CommandId, S::TelemetryKey, S::PolicyId],
            localizable_element_summary: "Window chrome, status areas, and switcher labels",
            source_language_route_ref: "route:shell:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:command:palette",
            title: "Command palette and menus",
            surface_family: F::CommandPalette,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[
                S::CommandId,
                S::CanonicalVerb,
                S::KeybindingPath,
                S::TelemetryKey,
            ],
            localizable_element_summary: "Command labels, synonyms, and palette chrome",
            source_language_route_ref: "route:command:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:help:docs",
            title: "Help and docs",
            surface_family: F::HelpAndDocs,
            owning_pack_ref: "locale-pack:core:docs-overlay",
            stable_element_kinds: &[
                S::DocsAnchorId,
                S::CitationAnchorId,
                S::CommandId,
                S::RecoveryRouteId,
            ],
            localizable_element_summary: "Docs prose, headings, and help cards",
            source_language_route_ref: "route:docs:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:cli:doctor",
            title: "CLI help and Doctor",
            surface_family: F::CliAndDoctor,
            owning_pack_ref: "locale-pack:core:cli",
            stable_element_kinds: &[
                S::CliSubcommandName,
                S::CliFlagLiteral,
                S::JsonKey,
                S::SchemaId,
                S::ErrorCodeOrFindingId,
            ],
            localizable_element_summary: "Usage prose, finding explanations, and next actions",
            source_language_route_ref: "flag:--locale-neutral",
            machine_output_locale_class: M::LocaleNeutralWithTranslatedHumanField,
        },
        SurfaceSeed {
            surface_id: "surface:notifications:center",
            title: "Notifications",
            surface_family: F::Notifications,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[S::NotificationId, S::CommandId, S::TelemetryKey],
            localizable_element_summary: "Toasts, banners, and notification body prose",
            source_language_route_ref: "route:notifications:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:extension:contributed-ui",
            title: "Extension-contributed UI",
            surface_family: F::ExtensionContributedUi,
            owning_pack_ref: "locale-pack:extension:contributed",
            stable_element_kinds: &[
                S::ExtensionNamespaceId,
                S::HostStableIdentifier,
                S::CommandId,
            ],
            localizable_element_summary:
                "Extension-owned labels and help inside the extension namespace",
            source_language_route_ref: "route:extension:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:companion:handoff",
            title: "Companion and browser handoff",
            surface_family: F::CompanionHandoff,
            owning_pack_ref: "locale-pack:companion:overlay",
            stable_element_kinds: &[S::HostStableIdentifier, S::RecoveryRouteId, S::CommandId],
            localizable_element_summary: "Companion prompts and browser-handoff guidance",
            source_language_route_ref: "route:companion:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:notebook:tooling",
            title: "Notebook tooling",
            surface_family: F::NotebookTooling,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[S::CommandId, S::SchemaId, S::TelemetryKey],
            localizable_element_summary: "Notebook editing prompts and execution status prose",
            source_language_route_ref: "route:notebook:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:data:api-tooling",
            title: "Data and API tooling",
            surface_family: F::DataAndApiTooling,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[
                S::CommandId,
                S::JsonKey,
                S::SchemaId,
                S::ErrorCodeOrFindingId,
            ],
            localizable_element_summary: "Data and API tool prose, labels, and error explanations",
            source_language_route_ref: "route:data:source-language:open",
            machine_output_locale_class: M::LocaleNeutralWithTranslatedHumanField,
        },
        SurfaceSeed {
            surface_id: "surface:learning:guided",
            title: "Guided learning and tours",
            surface_family: F::GuidedLearning,
            owning_pack_ref: "locale-pack:core:docs-overlay",
            stable_element_kinds: &[S::DocsAnchorId, S::CitationAnchorId, S::CommandId],
            localizable_element_summary: "Tour steps, exercise prompts, and instructional copy",
            source_language_route_ref: "route:learning:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:support:flows",
            title: "Support and recovery flows",
            surface_family: F::SupportFlows,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[S::ErrorCodeOrFindingId, S::RecoveryRouteId, S::TelemetryKey],
            localizable_element_summary: "Support guidance, remediation steps, and recovery prose",
            source_language_route_ref: "route:support:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
        SurfaceSeed {
            surface_id: "surface:release:about",
            title: "Release center and About",
            surface_family: F::ReleaseAndAbout,
            owning_pack_ref: "locale-pack:core:product-ui",
            stable_element_kinds: &[S::DocsAnchorId, S::PolicyId, S::TelemetryKey],
            localizable_element_summary: "Release notes summaries and About-pane disclosures",
            source_language_route_ref: "route:release:source-language:open",
            machine_output_locale_class: M::LocaleNativeHumanOnly,
        },
    ];

    SEEDS
        .iter()
        .map(|seed| SurfaceInventoryRow {
            surface_id: seed.surface_id.to_owned(),
            title: seed.title.to_owned(),
            surface_family: seed.surface_family,
            owning_pack_ref: seed.owning_pack_ref.to_owned(),
            stable_element_kinds: seed.stable_element_kinds.to_vec(),
            localizable_element_summary: seed.localizable_element_summary.to_owned(),
            source_language_route_ref: seed.source_language_route_ref.to_owned(),
            machine_output_locale_class: seed.machine_output_locale_class,
        })
        .collect()
}

struct CoverageSeed {
    row_id: &'static str,
    profile_id_ref: &'static str,
    surface_id_ref: &'static str,
    claimed: SurfaceLocalizationState,
    pack_ref: &'static str,
    pack_compatibility: PackCompatibilityState,
    evidence_refs: &'static [&'static str],
    evidence_freshness: EvidenceFreshnessState,
    required_for_claim: bool,
}

fn coverage_row(seed: &CoverageSeed) -> ProfileSurfaceCoverageRow {
    let (effective, narrowed, reason, gate) = derive_cell(
        seed.claimed,
        seed.pack_compatibility,
        seed.evidence_freshness,
    );
    ProfileSurfaceCoverageRow {
        row_id: seed.row_id.to_owned(),
        profile_id_ref: seed.profile_id_ref.to_owned(),
        surface_id_ref: seed.surface_id_ref.to_owned(),
        claimed_localization_state: seed.claimed,
        pack_ref: seed.pack_ref.to_owned(),
        pack_compatibility: seed.pack_compatibility,
        evidence_refs: seed.evidence_refs.iter().map(|r| (*r).to_owned()).collect(),
        evidence_freshness: seed.evidence_freshness,
        required_for_claim: seed.required_for_claim,
        effective_localization_state: effective,
        narrowed,
        narrow_reason: reason,
        gate_state: gate,
    }
}

fn seeded_coverage() -> Vec<ProfileSurfaceCoverageRow> {
    use EvidenceFreshnessState as E;
    use PackCompatibilityState as P;
    use SurfaceLocalizationState as L;

    const ES_PACK: &str = "locale-pack:core:es-mx:stable";
    const PT_PACK: &str = "locale-pack:community:pt-br:reviewed";
    const JA_PACK: &str = "locale-pack:core:ja-jp:missing";
    const FR_PACK: &str = "locale-pack:core:fr-fr:absent";

    const ES_PROOF: &[&str] = &[
        "fixtures/i18n/m5-surface-inventory/manifest.json",
        "fixtures/i18n/locale_surface_examples/shell_commands_and_palette_localized_label_stable_ids.yaml",
    ];
    const PT_PROOF: &[&str] = &["fixtures/i18n/m3/locale_fallback/manifest.json"];

    // The flagship profile covers every inventoried surface; two newer
    // surfaces are intentionally source-language fallback (partial coverage).
    let es_surfaces: [(&str, &str, L, P, E, bool); 12] = [
        (
            "cov:es-mx:shell",
            "surface:shell:chrome",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:command",
            "surface:command:palette",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:help",
            "surface:help:docs",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:cli",
            "surface:cli:doctor",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:notifications",
            "surface:notifications:center",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:extension",
            "surface:extension:contributed-ui",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:companion",
            "surface:companion:handoff",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:notebook",
            "surface:notebook:tooling",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:learning",
            "surface:learning:guided",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:support",
            "surface:support:flows",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:es-mx:data",
            "surface:data:api-tooling",
            L::SourceLanguageFallbackOnly,
            P::Compatible,
            E::NotRequired,
            false,
        ),
        (
            "cov:es-mx:release",
            "surface:release:about",
            L::SourceLanguageFallbackOnly,
            P::Compatible,
            E::NotRequired,
            false,
        ),
    ];

    // The community profile intends localized support, but stale proof on the
    // command surface narrows the profile to source-language fallback.
    let pt_surfaces: [(&str, &str, L, P, E, bool); 4] = [
        (
            "cov:pt-br:shell",
            "surface:shell:chrome",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:pt-br:command",
            "surface:command:palette",
            L::Localized,
            P::Compatible,
            E::Stale,
            true,
        ),
        (
            "cov:pt-br:help",
            "surface:help:docs",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
        (
            "cov:pt-br:cli",
            "surface:cli:doctor",
            L::Localized,
            P::Compatible,
            E::Current,
            true,
        ),
    ];

    // The missing-pack profile intends localized support, but the absent pack
    // narrows every covered surface to source-language fallback.
    let ja_surfaces: [(&str, &str, L, P, E, bool); 4] = [
        (
            "cov:ja-jp:shell",
            "surface:shell:chrome",
            L::Localized,
            P::Missing,
            E::Missing,
            true,
        ),
        (
            "cov:ja-jp:command",
            "surface:command:palette",
            L::Localized,
            P::Missing,
            E::Missing,
            true,
        ),
        (
            "cov:ja-jp:help",
            "surface:help:docs",
            L::Localized,
            P::Missing,
            E::Missing,
            true,
        ),
        (
            "cov:ja-jp:cli",
            "surface:cli:doctor",
            L::Localized,
            P::Missing,
            E::Missing,
            true,
        ),
    ];

    // The explicitly non-localized profile makes no localized claim.
    let fr_surfaces: [(&str, &str, L, P, E, bool); 4] = [
        (
            "cov:fr-fr:shell",
            "surface:shell:chrome",
            L::NotLocalized,
            P::NotApplicable,
            E::NotRequired,
            false,
        ),
        (
            "cov:fr-fr:command",
            "surface:command:palette",
            L::NotLocalized,
            P::NotApplicable,
            E::NotRequired,
            false,
        ),
        (
            "cov:fr-fr:help",
            "surface:help:docs",
            L::NotLocalized,
            P::NotApplicable,
            E::NotRequired,
            false,
        ),
        (
            "cov:fr-fr:cli",
            "surface:cli:doctor",
            L::NotLocalized,
            P::NotApplicable,
            E::NotRequired,
            false,
        ),
    ];

    let mut rows = Vec::new();
    for (row_id, surface, claimed, pack, freshness, required) in es_surfaces {
        rows.push(coverage_row(&CoverageSeed {
            row_id,
            profile_id_ref: "profile:es-MX:desktop",
            surface_id_ref: surface,
            claimed,
            pack_ref: ES_PACK,
            pack_compatibility: pack,
            evidence_refs: if claimed == L::Localized {
                ES_PROOF
            } else {
                &[]
            },
            evidence_freshness: freshness,
            required_for_claim: required,
        }));
    }
    for (row_id, surface, claimed, pack, freshness, required) in pt_surfaces {
        rows.push(coverage_row(&CoverageSeed {
            row_id,
            profile_id_ref: "profile:pt-BR:community",
            surface_id_ref: surface,
            claimed,
            pack_ref: PT_PACK,
            pack_compatibility: pack,
            evidence_refs: if freshness == E::Missing {
                &[]
            } else {
                PT_PROOF
            },
            evidence_freshness: freshness,
            required_for_claim: required,
        }));
    }
    for (row_id, surface, claimed, pack, freshness, required) in ja_surfaces {
        rows.push(coverage_row(&CoverageSeed {
            row_id,
            profile_id_ref: "profile:ja-JP:desktop",
            surface_id_ref: surface,
            claimed,
            pack_ref: JA_PACK,
            pack_compatibility: pack,
            evidence_refs: &[],
            evidence_freshness: freshness,
            required_for_claim: required,
        }));
    }
    for (row_id, surface, claimed, pack, freshness, required) in fr_surfaces {
        rows.push(coverage_row(&CoverageSeed {
            row_id,
            profile_id_ref: "profile:fr-FR:not-localized",
            surface_id_ref: surface,
            claimed,
            pack_ref: FR_PACK,
            pack_compatibility: pack,
            evidence_refs: &[],
            evidence_freshness: freshness,
            required_for_claim: required,
        }));
    }
    rows
}

struct ProfileSeed {
    profile_id: &'static str,
    title: &'static str,
    requested_locale: &'static str,
    fallback_chain: &'static [&'static str],
    primary_pack_ref: &'static str,
    supporting_pack_refs: &'static [&'static str],
    intended_claim_class: ProfileClaimClass,
    source_language_route_ref: &'static str,
}

fn seeded_profiles(coverage: &[ProfileSurfaceCoverageRow]) -> Vec<LocalizedProfileRow> {
    const SEEDS: &[ProfileSeed] = &[
        ProfileSeed {
            profile_id: "profile:es-MX:desktop",
            title: "Spanish (Mexico) desktop",
            requested_locale: "es-MX",
            fallback_chain: &["es-MX", "es", "en-US"],
            primary_pack_ref: "locale-pack:core:es-mx:stable",
            supporting_pack_refs: &[
                "locale-pack:core:docs-overlay:es-mx",
                "locale-pack:core:cli:es-mx",
            ],
            intended_claim_class: ProfileClaimClass::ClaimedLocalized,
            source_language_route_ref: "route:profile:source-language:open",
        },
        ProfileSeed {
            profile_id: "profile:pt-BR:community",
            title: "Portuguese (Brazil) community",
            requested_locale: "pt-BR",
            fallback_chain: &["pt-BR", "pt", "en-US"],
            primary_pack_ref: "locale-pack:community:pt-br:reviewed",
            supporting_pack_refs: &["locale-pack:core:docs-overlay:pt-br"],
            intended_claim_class: ProfileClaimClass::ClaimedLocalized,
            source_language_route_ref: "route:profile:source-language:open",
        },
        ProfileSeed {
            profile_id: "profile:ja-JP:desktop",
            title: "Japanese (Japan) desktop",
            requested_locale: "ja-JP",
            fallback_chain: &["ja-JP", "ja", "en-US"],
            primary_pack_ref: "locale-pack:core:ja-jp:missing",
            supporting_pack_refs: &[],
            intended_claim_class: ProfileClaimClass::ClaimedLocalized,
            source_language_route_ref: "route:profile:source-language:open",
        },
        ProfileSeed {
            profile_id: "profile:fr-FR:not-localized",
            title: "French (France) not localized",
            requested_locale: "fr-FR",
            fallback_chain: &["fr-FR", "fr", "en-US"],
            primary_pack_ref: "locale-pack:core:fr-fr:absent",
            supporting_pack_refs: &[],
            intended_claim_class: ProfileClaimClass::NotLocalized,
            source_language_route_ref: "route:profile:source-language:open",
        },
    ];

    SEEDS
        .iter()
        .map(|seed| {
            let cells: Vec<&ProfileSurfaceCoverageRow> = coverage
                .iter()
                .filter(|row| row.profile_id_ref == seed.profile_id)
                .collect();
            let (claim_class, narrowed, narrow_reason) =
                derive_profile_claim(seed.intended_claim_class, &cells);
            LocalizedProfileRow {
                profile_id: seed.profile_id.to_owned(),
                title: seed.title.to_owned(),
                requested_locale: seed.requested_locale.to_owned(),
                source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
                fallback_chain: seed
                    .fallback_chain
                    .iter()
                    .map(|l| (*l).to_owned())
                    .collect(),
                primary_pack_ref: seed.primary_pack_ref.to_owned(),
                supporting_pack_refs: seed
                    .supporting_pack_refs
                    .iter()
                    .map(|p| (*p).to_owned())
                    .collect(),
                intended_claim_class: seed.intended_claim_class,
                claim_class,
                narrowed,
                narrow_reason,
                source_language_route_ref: seed.source_language_route_ref.to_owned(),
                visible_in_settings: true,
                visible_in_diagnostics: true,
                visible_in_support_export: true,
                visible_in_help_about: true,
                non_blocking_core_use: true,
            }
        })
        .collect()
}

fn seeded_consumption_bindings() -> Vec<ConsumptionBindingRow> {
    vec![
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::ReleaseCenter,
            consumer_ref: "crates/aureline-release".to_owned(),
            ingests_summary: "Gates localized-profile promotion on green coverage and proof."
                .to_owned(),
            consumed_fields: strings(&["summary", "localized_profiles", "release_gate_rows"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::HelpAbout,
            consumer_ref: "crates/aureline-shell".to_owned(),
            ingests_summary: "Discloses which surfaces are localized, fallback, or not localized."
                .to_owned(),
            consumed_fields: strings(&[
                "surface_inventory",
                "localized_profiles",
                "profile_surface_coverage",
            ]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::Diagnostics,
            consumer_ref: "crates/aureline-cli".to_owned(),
            ingests_summary: "Reports effective localization state and fallback chains in Doctor."
                .to_owned(),
            consumed_fields: strings(&["profile_surface_coverage", "localized_profiles"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::ClaimNarrowing,
            consumer_ref: "crates/aureline-i18n".to_owned(),
            ingests_summary:
                "Narrows localized claims when pack compatibility or proof is missing.".to_owned(),
            consumed_fields: strings(&[
                "localized_profiles",
                "profile_surface_coverage",
                "summary",
            ]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::SupportExport,
            consumer_ref: "crates/aureline-support".to_owned(),
            ingests_summary:
                "Projects localized-profile posture into metadata-only support export.".to_owned(),
            consumed_fields: strings(&["localized_profiles", "summary"]),
        },
        ConsumptionBindingRow {
            consumer_kind: ConsumerKind::DocsBrowser,
            consumer_ref: "crates/aureline-docs".to_owned(),
            ingests_summary: "Reads the surface inventory and owning packs for docs surfaces."
                .to_owned(),
            consumed_fields: strings(&["surface_inventory"]),
        },
    ]
}

fn seeded_release_gates() -> Vec<ProfileReleaseGateRow> {
    let command = "cargo test -p aureline-i18n --test localized_profile_matrix --locked";
    [
        (
            "release-gate:surface-inventory-frozen",
            "surface_inventory_frozen",
        ),
        (
            "release-gate:profile-fallback-chains",
            "profile_fallback_chains",
        ),
        ("release-gate:claim-auto-narrowing", "claim_auto_narrowing"),
        ("release-gate:pack-compatibility", "pack_compatibility"),
        ("release-gate:evidence-freshness", "evidence_freshness"),
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
        fixture_refs: vec![LOCALIZED_PROFILE_MATRIX_FIXTURE_REF.to_owned()],
        artifact_refs: vec![
            "artifacts/i18n/m5-localized-profile-matrix.md".to_owned(),
            "docs/i18n/m5-localization-scope.md".to_owned(),
        ],
        required_for_claimed_profiles: true,
        gate_state: MatrixGateState::Green,
    })
    .collect()
}

fn derive_summary(
    surfaces: &[SurfaceInventoryRow],
    profiles: &[LocalizedProfileRow],
    coverage: &[ProfileSurfaceCoverageRow],
    gates: &[ProfileReleaseGateRow],
) -> LocalizedProfileMatrixSummary {
    let claimed_localized_profiles = profiles
        .iter()
        .filter(|p| p.claim_class == ProfileClaimClass::ClaimedLocalized)
        .count();
    let source_language_fallback_profiles = profiles
        .iter()
        .filter(|p| p.claim_class == ProfileClaimClass::SourceLanguageFallbackOnly)
        .count();
    let not_localized_profiles = profiles
        .iter()
        .filter(|p| p.claim_class == ProfileClaimClass::NotLocalized)
        .count();
    let narrowed_profiles = profiles.iter().filter(|p| p.narrowed).count();

    let localized_surface_cells = coverage
        .iter()
        .filter(|c| c.effective_localization_state == SurfaceLocalizationState::Localized)
        .count();
    let source_language_fallback_cells = coverage
        .iter()
        .filter(|c| {
            c.effective_localization_state == SurfaceLocalizationState::SourceLanguageFallbackOnly
        })
        .count();
    let not_localized_cells = coverage
        .iter()
        .filter(|c| c.effective_localization_state == SurfaceLocalizationState::NotLocalized)
        .count();
    let narrowed_cells = coverage.iter().filter(|c| c.narrowed).count();

    let blocked_rows = coverage
        .iter()
        .filter(|c| c.gate_state == MatrixGateState::Blocked)
        .count()
        + gates
            .iter()
            .filter(|g| g.gate_state == MatrixGateState::Blocked)
            .count();

    let promotion_state = if blocked_rows == 0 {
        MatrixGateState::Green
    } else {
        MatrixGateState::Blocked
    };

    LocalizedProfileMatrixSummary {
        total_surfaces: surfaces.len(),
        total_profiles: profiles.len(),
        claimed_localized_profiles,
        source_language_fallback_profiles,
        not_localized_profiles,
        narrowed_profiles,
        localized_surface_cells,
        source_language_fallback_cells,
        not_localized_cells,
        narrowed_cells,
        blocked_rows,
        promotion_state,
    }
}

fn validate_surface_inventory(
    surfaces: &[SurfaceInventoryRow],
    findings: &mut Vec<LocalePackValidationFinding>,
) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut families = BTreeSet::new();
    for surface in surfaces {
        if !ids.insert(surface.surface_id.clone()) {
            findings.push(LocalePackValidationFinding::new(
                surface.surface_id.clone(),
                "duplicate surface id",
            ));
        }
        families.insert(surface.surface_family);
        if surface.stable_element_kinds.is_empty()
            || surface.owning_pack_ref.trim().is_empty()
            || surface.source_language_route_ref.trim().is_empty()
            || surface.title.trim().is_empty()
        {
            findings.push(LocalePackValidationFinding::new(
                surface.surface_id.clone(),
                "surface row must cite owning pack, stable elements, and a source-language route",
            ));
        }
    }

    for required in [
        LocalizableSurfaceFamily::ShellChrome,
        LocalizableSurfaceFamily::CommandPalette,
        LocalizableSurfaceFamily::HelpAndDocs,
        LocalizableSurfaceFamily::CliAndDoctor,
        LocalizableSurfaceFamily::Notifications,
        LocalizableSurfaceFamily::ExtensionContributedUi,
        LocalizableSurfaceFamily::CompanionHandoff,
    ] {
        if !families.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALIZED_PROFILE_MATRIX_PACKET_ID,
                format!("surface inventory is missing {required:?}"),
            ));
        }
    }
    ids
}

fn validate_profiles(
    profiles: &[LocalizedProfileRow],
    findings: &mut Vec<LocalePackValidationFinding>,
) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for profile in profiles {
        if !ids.insert(profile.profile_id.clone()) {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "duplicate profile id",
            ));
        }
        classes.insert(profile.claim_class);
        if profile.fallback_chain.first() != Some(&profile.requested_locale)
            || profile.fallback_chain.last() != Some(&profile.source_language_locale)
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile fallback chain must run requested locale to source language",
            ));
        }
        if !profile.visible_in_settings
            || !profile.visible_in_diagnostics
            || !profile.visible_in_support_export
            || !profile.visible_in_help_about
            || profile.source_language_route_ref.trim().is_empty()
            || !profile.non_blocking_core_use
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile must be inspectable, source-language reachable, and non-blocking",
            ));
        }
        if profile.narrowed == (profile.narrow_reason.is_none()) {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "narrowed profiles must cite a reason and unnarrowed profiles must not",
            ));
        }
        if profile.claim_class == ProfileClaimClass::ClaimedLocalized && profile.narrowed {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "a claimed localized profile must not be narrowed",
            ));
        }
    }

    for required in [
        ProfileClaimClass::ClaimedLocalized,
        ProfileClaimClass::SourceLanguageFallbackOnly,
        ProfileClaimClass::NotLocalized,
    ] {
        if !classes.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALIZED_PROFILE_MATRIX_PACKET_ID,
                format!("profiles are missing claim class {required:?}"),
            ));
        }
    }
    ids
}

fn validate_coverage(
    coverage: &[ProfileSurfaceCoverageRow],
    surface_ids: &BTreeSet<String>,
    profile_ids: &BTreeSet<String>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut row_ids = BTreeSet::new();
    for row in coverage {
        if !row_ids.insert(row.row_id.clone()) {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "duplicate coverage row id",
            ));
        }
        if !profile_ids.contains(&row.profile_id_ref) {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "coverage row references an unknown profile",
            ));
        }
        if !surface_ids.contains(&row.surface_id_ref) {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "coverage row references an unknown surface",
            ));
        }

        let (effective, narrowed, reason, gate) = derive_cell(
            row.claimed_localization_state,
            row.pack_compatibility,
            row.evidence_freshness,
        );
        if row.effective_localization_state != effective
            || row.narrowed != narrowed
            || row.narrow_reason != reason
            || row.gate_state != gate
        {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "coverage row derived state drifted from claim, pack, and evidence",
            ));
        }
        if row.claimed_localization_state == SurfaceLocalizationState::Localized
            && row.evidence_refs.is_empty()
            && !row.narrowed
        {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "a localized coverage row must cite proof",
            ));
        }
    }
}

fn validate_profile_narrowing(
    packet: &LocalizedProfileMatrixPacket,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    for profile in &packet.localized_profiles {
        let cells: Vec<&ProfileSurfaceCoverageRow> = packet
            .profile_surface_coverage
            .iter()
            .filter(|row| row.profile_id_ref == profile.profile_id)
            .collect();
        if cells.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile has no coverage rows",
            ));
            continue;
        }
        let (claim_class, narrowed, reason) =
            derive_profile_claim(profile.intended_claim_class, &cells);
        if profile.claim_class != claim_class
            || profile.narrowed != narrowed
            || profile.narrow_reason != reason
        {
            findings.push(LocalePackValidationFinding::new(
                profile.profile_id.clone(),
                "profile claim drifted from its coverage rows",
            ));
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
    ] {
        if !kinds.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALIZED_PROFILE_MATRIX_PACKET_ID,
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
        "surface_inventory_frozen",
        "profile_fallback_chains",
        "claim_auto_narrowing",
        "pack_compatibility",
        "evidence_freshness",
        "downstream_consumption",
    ] {
        if !proof_kinds.contains(required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALIZED_PROFILE_MATRIX_PACKET_ID,
                format!("release gates are missing {required}"),
            ));
        }
    }
}

fn validate_summary(
    packet: &LocalizedProfileMatrixPacket,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let expected = derive_summary(
        &packet.surface_inventory,
        &packet.localized_profiles,
        &packet.profile_surface_coverage,
        &packet.release_gate_rows,
    );
    if packet.summary != expected {
        findings.push(LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "localized profile matrix summary drifted from row state",
        ));
    }
    if packet.summary.blocked_rows != 0 || packet.summary.promotion_state != MatrixGateState::Green
    {
        findings.push(LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "localized profile matrix contains blocked rows",
        ));
    }
    if packet.summary.claimed_localized_profiles == 0 {
        findings.push(LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "localized profile matrix must claim at least one localized profile",
        ));
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_owned()).collect()
}
