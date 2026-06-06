//! Stable locale-pack lifecycle and translated-surface parity contract.
//!
//! This module owns the stable localization claim packet. It turns locale
//! packs, fallback-chain state, stable message identifiers, translated docs /
//! tour / auth / help / CLI parity, and release-gated i18n proofs into one
//! validation surface that release, support, settings, and docs consumers can
//! ingest without cloning status prose.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    DegradedLocalizationState, LocaleFallbackOriginClass, LocalePackDistributionClass,
    LocalePackSignatureState, LocalePackSourceClass, MachineOutputLocaleClass,
    MessageSurfaceFamily, StableMessageIdentityRefs, VersionMatchState, GENERATED_AT,
    SOURCE_LANGUAGE_LOCALE, TARGET_BUILD,
};

/// Schema version for the stable locale lifecycle and parity packet.
pub const STABLE_LOCALE_LIFECYCLE_PARITY_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`StableLocaleLifecycleParityPacket`].
pub const STABLE_LOCALE_LIFECYCLE_PARITY_RECORD_KIND: &str =
    "stable_locale_lifecycle_parity_packet";

/// Stable packet id for the seeded stable locale lifecycle and parity contract.
pub const STABLE_LOCALE_LIFECYCLE_PARITY_PACKET_ID: &str =
    "locale-pack:stable:lifecycle-translated-surface-parity:v1";

/// Fixture path for the seeded stable locale lifecycle and parity packet.
pub const STABLE_LOCALE_LIFECYCLE_PARITY_FIXTURE_REF: &str =
    "fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json";

/// Claim posture for one localized row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalizationClaimClass {
    /// Row claims full stable localized support.
    StableLocalized,
    /// Row is intentionally downgraded to source language with visible state.
    DegradedSourceLanguageOnly,
    /// Row does not make a localized support claim.
    NotClaimed,
}

/// Promotion state for a stable localization claim gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimGateState {
    /// All blocking proof rows are green for the claim.
    Green,
    /// The row remains usable only after narrowing the localized claim.
    Narrowed,
    /// The row blocks the claim.
    Blocked,
}

/// Lifecycle state for a locale-pack artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleLifecycleState {
    /// Built-in source-language pack is current.
    SourceLanguageBuiltInCurrent,
    /// First-party locale pack is signed, current, and compatible.
    FirstPartySignedCompatible,
    /// Mirrored first-party pack is signed, current, and compatible.
    MirrorSignedCompatible,
    /// Reviewed community pack is signed and compatible within its window.
    ReviewedCommunitySignedCompatible,
    /// Signature failure forced source-language rendering.
    SignatureFailedSourceLanguageOnly,
    /// Version skew forced source-language rendering.
    VersionSkewSourceLanguageOnly,
    /// Missing pack forced source-language rendering.
    MissingPackSourceLanguageOnly,
}

/// Product surface covered by translated parity validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslatedSurfaceKind {
    /// Translated documentation page or docs pane.
    Docs,
    /// Guided onboarding or product tour.
    GuidedTour,
    /// Authentication or recovery copy.
    AuthRecovery,
    /// Help or glossary card.
    HelpGlossaryCard,
    /// Human-readable CLI help or Doctor text.
    CliHumanHelp,
}

/// Result state for translated-surface parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslatedSurfaceParityResult {
    /// Translated row preserves all stable truth.
    Passed,
    /// Row falls back to source language and preserves all stable truth.
    SourceLanguageFallbackPassed,
    /// Row failed parity and must not be claimed localized.
    Blocked,
}

/// Lifecycle and compatibility window for one locale pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackLifecycleWindow {
    /// Stable window id.
    pub window_id: String,
    /// Stable pack id.
    pub pack_ref: String,
    /// Pack source class.
    pub source_class: LocalePackSourceClass,
    /// Pack distribution class.
    pub distribution_class: LocalePackDistributionClass,
    /// Requested locale this window can satisfy.
    pub requested_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Minimum compatible build identity.
    pub compatibility_min_build_ref: String,
    /// Maximum compatible build identity.
    pub compatibility_max_build_ref: String,
    /// Signature state observed for the pack.
    pub signature_state: LocalePackSignatureState,
    /// Version match state observed for the pack.
    pub version_match_state: VersionMatchState,
    /// Lifecycle state derived from signature, source, mirror, and version.
    pub lifecycle_state: LocaleLifecycleState,
    /// Mirror receipts that prove the artifact can be mirrored.
    pub mirror_receipt_refs: Vec<String>,
    /// Offline bundle ref when the pack supports air-gapped import.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_bundle_ref: Option<String>,
    /// Reviewed-community decision row, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_community_decision_ref: Option<String>,
    /// Rollback target for the pack.
    pub rollback_ref: String,
    /// Whether this window backs a claimed stable localized row.
    pub backs_claimed_localized_row: bool,
}

/// Stable message-id proof for one translated message family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMessageIdProofRow {
    /// Stable row id.
    pub row_id: String,
    /// Stable message id.
    pub message_id: String,
    /// Surface family where the message renders.
    pub surface_family: MessageSurfaceFamily,
    /// Stable non-prose refs bound to the message.
    pub stable_refs: StableMessageIdentityRefs,
    /// Locale packs that may translate the message.
    pub translation_pack_refs: Vec<String>,
    /// Whether command ids remain stable across translation.
    pub command_id_stable: bool,
    /// Whether schema or diagnostic ids remain stable across translation.
    pub schema_or_diagnostic_id_stable: bool,
    /// Whether semantic action ids remain stable across translation.
    pub semantic_action_id_stable: bool,
    /// Whether machine identifiers remain locale-neutral.
    pub machine_identifier_locale_neutral: bool,
    /// Whether behavior routes by localized prose.
    pub routed_by_localized_prose: bool,
}

/// Inspectable fallback-chain truth for one localized or downgraded row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackChainTruthRow {
    /// Stable row id.
    pub row_id: String,
    /// Claim posture for this row.
    pub claim_class: LocalizationClaimClass,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale that produced rendered text.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Fallback origin class.
    pub fallback_origin_class: LocaleFallbackOriginClass,
    /// Degraded localization state.
    pub degraded_localization_state: DegradedLocalizationState,
    /// Whether Settings exposes this fallback row.
    pub visible_in_settings: bool,
    /// Whether diagnostics exposes this fallback row.
    pub visible_in_diagnostics: bool,
    /// Whether support export exposes this fallback row.
    pub visible_in_support_export: bool,
    /// Source-language route available on the same surface.
    pub open_in_source_language_route_ref: String,
    /// Whether missing or blocked localization keeps local product use available.
    pub non_blocking_core_use: bool,
}

/// Translated docs, tour, auth, help, or CLI parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedSurfaceParityRow {
    /// Stable row id.
    pub row_id: String,
    /// Claim posture for this translated row.
    pub claim_class: LocalizationClaimClass,
    /// Surface kind under parity validation.
    pub surface_kind: TranslatedSurfaceKind,
    /// Stable message ids rendered by this row.
    pub message_id_refs: Vec<String>,
    /// Stable command ids preserved by this row.
    pub command_id_refs: Vec<String>,
    /// Citation anchors preserved by this row.
    pub citation_anchor_refs: Vec<String>,
    /// Keyboard paths preserved by this row.
    pub keyboard_path_refs: Vec<String>,
    /// Screen-reader label refs preserved by this row.
    pub screen_reader_label_refs: Vec<String>,
    /// Recovery route refs preserved by this row.
    pub recovery_route_refs: Vec<String>,
    /// Same-surface source-language route.
    pub source_language_route_ref: String,
    /// Whether CLI machine keys remain locale-neutral.
    pub cli_machine_keys_locale_neutral: bool,
    /// Whether human prose is allowed to localize on this row.
    pub human_help_may_localize: bool,
    /// Machine-output localization posture.
    pub machine_output_locale_class: MachineOutputLocaleClass,
    /// Fallback state refs consulted by this row.
    pub fallback_state_refs: Vec<String>,
    /// Accessibility and parity fixtures backing this row.
    pub accessibility_fixture_refs: Vec<String>,
    /// Result of translated parity validation.
    pub parity_result: TranslatedSurfaceParityResult,
}

/// Release-gate proof row for localization validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseGateProofRow {
    /// Stable row id.
    pub row_id: String,
    /// Proof kind such as `pseudoloc`, `rtl`, or `locale_pack_signing`.
    pub proof_kind: String,
    /// Exact command for local verification.
    pub command: String,
    /// Fixture refs consumed by the proof.
    pub fixture_refs: Vec<String>,
    /// Artifact refs produced or reviewed by the proof.
    pub artifact_refs: Vec<String>,
    /// Whether the row gates claimed stable localized rows.
    pub required_for_claimed_localized_rows: bool,
    /// Gate state for this proof.
    pub gate_state: ClaimGateState,
}

/// Summary of stable localization claim posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackParitySummary {
    /// Number of rows that claim stable localized support.
    pub claimed_localized_rows: usize,
    /// Number of claimed rows with green gate state.
    pub green_claimed_localized_rows: usize,
    /// Number of rows intentionally downgraded to source language.
    pub degraded_source_language_rows: usize,
    /// Number of blocked rows.
    pub blocked_rows: usize,
    /// Overall promotion state for claimed localized rows.
    pub promotion_state: ClaimGateState,
}

/// Stable locale-pack lifecycle and translated-surface parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLocaleLifecycleParityPacket {
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
    /// Runtime consumers that must ingest this packet.
    pub runtime_consumer_refs: Vec<String>,
    /// Locale-pack lifecycle windows.
    pub lifecycle_windows: Vec<LocalePackLifecycleWindow>,
    /// Stable message-id proof rows.
    pub stable_message_id_rows: Vec<StableMessageIdProofRow>,
    /// Inspectable fallback-chain truth rows.
    pub fallback_truth_rows: Vec<FallbackChainTruthRow>,
    /// Translated-surface parity rows.
    pub translated_surface_rows: Vec<TranslatedSurfaceParityRow>,
    /// Release-gated proof rows.
    pub release_gate_rows: Vec<ReleaseGateProofRow>,
    /// Summary posture derived from the rows.
    pub summary: LocalePackParitySummary,
}

impl StableLocaleLifecycleParityPacket {
    /// Validates stable locale lifecycle, fallback, and translated parity invariants.
    pub fn validate(&self) -> Result<(), Vec<crate::LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != STABLE_LOCALE_LIFECYCLE_PARITY_RECORD_KIND {
            findings.push(crate::LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "stable locale lifecycle packet record_kind is unsupported",
            ));
        }
        if self.schema_version != STABLE_LOCALE_LIFECYCLE_PARITY_SCHEMA_VERSION {
            findings.push(crate::LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "stable locale lifecycle packet schema_version is unsupported",
            ));
        }

        validate_lifecycle_windows(&self.lifecycle_windows, &mut findings);
        validate_stable_message_rows(&self.stable_message_id_rows, &mut findings);
        validate_fallback_truth_rows(&self.fallback_truth_rows, &mut findings);
        validate_translated_surface_rows(&self.translated_surface_rows, &mut findings);
        validate_release_gate_rows(&self.release_gate_rows, &mut findings);
        validate_summary(self, &mut findings);

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Returns the seeded stable locale lifecycle and translated-surface parity packet.
pub fn seeded_stable_locale_lifecycle_parity_packet() -> StableLocaleLifecycleParityPacket {
    let lifecycle_windows = seeded_lifecycle_windows();
    let stable_message_id_rows = seeded_message_id_rows();
    let fallback_truth_rows = seeded_fallback_truth_rows();
    let translated_surface_rows = seeded_translated_surface_rows();
    let release_gate_rows = seeded_release_gate_rows();
    let summary = derive_summary(
        &fallback_truth_rows,
        &translated_surface_rows,
        &release_gate_rows,
    );

    StableLocaleLifecycleParityPacket {
        record_kind: STABLE_LOCALE_LIFECYCLE_PARITY_RECORD_KIND.to_owned(),
        schema_version: STABLE_LOCALE_LIFECYCLE_PARITY_SCHEMA_VERSION,
        packet_id: STABLE_LOCALE_LIFECYCLE_PARITY_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_contract_refs: BTreeMap::from([
            (
                "architecture_locale_governance".to_owned(),
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
                "dense_i18n_corpus".to_owned(),
                "fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json".to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            "crates/aureline-i18n".to_owned(),
            "crates/aureline-docs".to_owned(),
            "crates/aureline-cli".to_owned(),
            "crates/aureline-auth".to_owned(),
            "crates/aureline-shell".to_owned(),
            "crates/aureline-support".to_owned(),
        ],
        lifecycle_windows,
        stable_message_id_rows,
        fallback_truth_rows,
        translated_surface_rows,
        release_gate_rows,
        summary,
    }
}

fn seeded_lifecycle_windows() -> Vec<LocalePackLifecycleWindow> {
    vec![
        LocalePackLifecycleWindow {
            window_id: "locale-window:source:en-us:stable".to_owned(),
            pack_ref: "locale-pack:core:source:en-us".to_owned(),
            source_class: LocalePackSourceClass::FirstPartySourceLanguage,
            distribution_class: LocalePackDistributionClass::BuiltInWithProduct,
            requested_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            compatibility_min_build_ref: "build:aureline:0.0.0-stable.2026.05.01".to_owned(),
            compatibility_max_build_ref: TARGET_BUILD.to_owned(),
            signature_state: LocalePackSignatureState::NotApplicableBuiltIn,
            version_match_state: VersionMatchState::ExactBuildMatch,
            lifecycle_state: LocaleLifecycleState::SourceLanguageBuiltInCurrent,
            mirror_receipt_refs: vec!["mirror-receipt:core:source:en-us".to_owned()],
            offline_bundle_ref: Some("offline-import:core:source:en-us".to_owned()),
            reviewed_community_decision_ref: None,
            rollback_ref: "rollback:locale-pack:core:source:en-us:last-known-good".to_owned(),
            backs_claimed_localized_row: false,
        },
        LocalePackLifecycleWindow {
            window_id: "locale-window:first-party:es-mx:stable".to_owned(),
            pack_ref: "locale-pack:core:es-mx:stable".to_owned(),
            source_class: LocalePackSourceClass::FirstPartyLocalePack,
            distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
            requested_locale: "es-MX".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            compatibility_min_build_ref: "build:aureline:0.0.0-stable.2026.05.01".to_owned(),
            compatibility_max_build_ref: TARGET_BUILD.to_owned(),
            signature_state: LocalePackSignatureState::SignedVerified,
            version_match_state: VersionMatchState::ExactBuildMatch,
            lifecycle_state: LocaleLifecycleState::MirrorSignedCompatible,
            mirror_receipt_refs: vec![
                "mirror-receipt:official:locale-pack:core:es-mx:stable".to_owned(),
                "mirror-receipt:airgap:locale-pack:core:es-mx:stable".to_owned(),
            ],
            offline_bundle_ref: Some(
                "offline-import:locale-pack:core:es-mx:stable-bundle-01".to_owned(),
            ),
            reviewed_community_decision_ref: None,
            rollback_ref: "rollback:locale-pack:core:es-mx:stable-last-known-good".to_owned(),
            backs_claimed_localized_row: true,
        },
        LocalePackLifecycleWindow {
            window_id: "locale-window:community:pt-br:reviewed".to_owned(),
            pack_ref: "locale-pack:community:pt-br:reviewed".to_owned(),
            source_class: LocalePackSourceClass::ReviewedCommunityPack,
            distribution_class: LocalePackDistributionClass::CommunitySuppliedPack,
            requested_locale: "pt-BR".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            compatibility_min_build_ref: "build:aureline:0.0.0-stable.2026.05.01".to_owned(),
            compatibility_max_build_ref: TARGET_BUILD.to_owned(),
            signature_state: LocalePackSignatureState::SignedVerified,
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            lifecycle_state: LocaleLifecycleState::ReviewedCommunitySignedCompatible,
            mirror_receipt_refs: vec!["mirror-receipt:community-reviewed:pt-br:stable".to_owned()],
            offline_bundle_ref: Some(
                "offline-import:locale-pack:community:pt-br:reviewed-bundle-01".to_owned(),
            ),
            reviewed_community_decision_ref: Some(
                "review:locale-pack:community:pt-br:stable-compat-window".to_owned(),
            ),
            rollback_ref: "rollback:locale-pack:community:pt-br:reviewed-last-known-good"
                .to_owned(),
            backs_claimed_localized_row: false,
        },
        LocalePackLifecycleWindow {
            window_id: "locale-window:blocked:ja-jp:missing".to_owned(),
            pack_ref: "locale-pack:core:ja-jp:missing".to_owned(),
            source_class: LocalePackSourceClass::FirstPartyLocalePack,
            distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
            requested_locale: "ja-JP".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            compatibility_min_build_ref: "build:aureline:0.0.0-stable.2026.05.01".to_owned(),
            compatibility_max_build_ref: TARGET_BUILD.to_owned(),
            signature_state: LocalePackSignatureState::SignedUnverified,
            version_match_state: VersionMatchState::UnknownTargetBuild,
            lifecycle_state: LocaleLifecycleState::MissingPackSourceLanguageOnly,
            mirror_receipt_refs: Vec::new(),
            offline_bundle_ref: None,
            reviewed_community_decision_ref: None,
            rollback_ref: "rollback:locale-pack:core:ja-jp:source-language".to_owned(),
            backs_claimed_localized_row: false,
        },
    ]
}

fn seeded_message_id_rows() -> Vec<StableMessageIdProofRow> {
    vec![
        StableMessageIdProofRow {
            row_id: "message-proof:command:open-folder".to_owned(),
            message_id: "msg:shell:palette:open-folder:title".to_owned(),
            surface_family: MessageSurfaceFamily::CommandLabel,
            stable_refs: StableMessageIdentityRefs {
                command_id_ref: Some("cmd:core:open_folder".to_owned()),
                semantic_action_id_ref: Some("action:workspace.open_folder".to_owned()),
                telemetry_key_ref: Some("telemetry:command_palette.command_invoked".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            translation_pack_refs: vec![
                "locale-pack:core:es-mx:stable".to_owned(),
                "locale-pack:community:pt-br:reviewed".to_owned(),
            ],
            command_id_stable: true,
            schema_or_diagnostic_id_stable: true,
            semantic_action_id_stable: true,
            machine_identifier_locale_neutral: true,
            routed_by_localized_prose: false,
        },
        StableMessageIdProofRow {
            row_id: "message-proof:docs:onboarding-open-folder".to_owned(),
            message_id: "msg:docs:onboarding:open-folder:summary".to_owned(),
            surface_family: MessageSurfaceFamily::DocsTourOrAuthText,
            stable_refs: StableMessageIdentityRefs {
                command_id_ref: Some("cmd:core:open_folder".to_owned()),
                semantic_action_id_ref: Some("action:onboarding.open_folder".to_owned()),
                docs_pack_key_ref: Some(
                    "docs-pack:onboarding:first-useful-work:open-folder".to_owned(),
                ),
                ..StableMessageIdentityRefs::default()
            },
            translation_pack_refs: vec!["locale-pack:core:es-mx:stable".to_owned()],
            command_id_stable: true,
            schema_or_diagnostic_id_stable: true,
            semantic_action_id_stable: true,
            machine_identifier_locale_neutral: true,
            routed_by_localized_prose: false,
        },
        StableMessageIdProofRow {
            row_id: "message-proof:auth:recovery".to_owned(),
            message_id: "msg:auth:recovery:source-language-fallback".to_owned(),
            surface_family: MessageSurfaceFamily::PolicyLegalOrRecoveryText,
            stable_refs: StableMessageIdentityRefs {
                semantic_action_id_ref: Some(
                    "action:auth.recovery.open_source_language".to_owned(),
                ),
                policy_name_ref: Some("policy.locale.source_language_fallback_required".to_owned()),
                docs_pack_key_ref: Some("docs-pack:recovery:source-language-fallback".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            translation_pack_refs: vec![
                "locale-pack:core:es-mx:stable".to_owned(),
                "locale-pack:community:pt-br:reviewed".to_owned(),
            ],
            command_id_stable: true,
            schema_or_diagnostic_id_stable: true,
            semantic_action_id_stable: true,
            machine_identifier_locale_neutral: true,
            routed_by_localized_prose: false,
        },
        StableMessageIdProofRow {
            row_id: "message-proof:cli:doctor-schema-drift".to_owned(),
            message_id: "msg:doctor:profile-schema-drift:human".to_owned(),
            surface_family: MessageSurfaceFamily::CliHelpText,
            stable_refs: StableMessageIdentityRefs {
                diagnostic_id_ref: Some("doctor.finding.profile.schema_drift".to_owned()),
                schema_id_ref: Some("schemas/diagnostics/diagnostic_record.schema.json".to_owned()),
                telemetry_key_ref: Some("telemetry:doctor.finding_emitted".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            translation_pack_refs: vec!["locale-pack:core:es-mx:stable".to_owned()],
            command_id_stable: true,
            schema_or_diagnostic_id_stable: true,
            semantic_action_id_stable: true,
            machine_identifier_locale_neutral: true,
            routed_by_localized_prose: false,
        },
    ]
}

fn seeded_fallback_truth_rows() -> Vec<FallbackChainTruthRow> {
    vec![
        FallbackChainTruthRow {
            row_id: "fallback-truth:es-mx:claimed:stable".to_owned(),
            claim_class: LocalizationClaimClass::StableLocalized,
            requested_locale: "es-MX".to_owned(),
            effective_locale: "es-MX".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "es-MX".to_owned(),
                "es".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin_class: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_localization_state: DegradedLocalizationState::FullyLocalized,
            visible_in_settings: true,
            visible_in_diagnostics: true,
            visible_in_support_export: true,
            open_in_source_language_route_ref: "route:docs:source-language:open".to_owned(),
            non_blocking_core_use: true,
        },
        FallbackChainTruthRow {
            row_id: "fallback-truth:pt-br:community:source-language".to_owned(),
            claim_class: LocalizationClaimClass::DegradedSourceLanguageOnly,
            requested_locale: "pt-BR".to_owned(),
            effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "pt-BR".to_owned(),
                "pt".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin_class: LocaleFallbackOriginClass::SourceLanguageFallback,
            degraded_localization_state: DegradedLocalizationState::GlossaryOnlyLocalized,
            visible_in_settings: true,
            visible_in_diagnostics: true,
            visible_in_support_export: true,
            open_in_source_language_route_ref: "route:docs:source-language:open".to_owned(),
            non_blocking_core_use: true,
        },
        FallbackChainTruthRow {
            row_id: "fallback-truth:ja-jp:missing-pack:source-language".to_owned(),
            claim_class: LocalizationClaimClass::DegradedSourceLanguageOnly,
            requested_locale: "ja-JP".to_owned(),
            effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "ja-JP".to_owned(),
                "ja".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin_class: LocaleFallbackOriginClass::PackMissingSourceLanguageOnly,
            degraded_localization_state: DegradedLocalizationState::FailedPackSourceLanguageOnly,
            visible_in_settings: true,
            visible_in_diagnostics: true,
            visible_in_support_export: true,
            open_in_source_language_route_ref: "route:docs:source-language:open".to_owned(),
            non_blocking_core_use: true,
        },
        FallbackChainTruthRow {
            row_id: "fallback-truth:de-de:signature-failed:source-language".to_owned(),
            claim_class: LocalizationClaimClass::DegradedSourceLanguageOnly,
            requested_locale: "de-DE".to_owned(),
            effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "de-DE".to_owned(),
                "de".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin_class: LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly,
            degraded_localization_state: DegradedLocalizationState::FailedPackSourceLanguageOnly,
            visible_in_settings: true,
            visible_in_diagnostics: true,
            visible_in_support_export: true,
            open_in_source_language_route_ref: "route:docs:source-language:open".to_owned(),
            non_blocking_core_use: true,
        },
    ]
}

fn seeded_translated_surface_rows() -> Vec<TranslatedSurfaceParityRow> {
    vec![
        translated_surface_row(TranslatedSurfaceSeed {
            row_id: "surface-parity:docs:open-folder",
            claim_class: LocalizationClaimClass::StableLocalized,
            surface_kind: TranslatedSurfaceKind::Docs,
            message_id_refs: vec!["msg:docs:onboarding:open-folder:summary"],
            command_id_refs: vec!["cmd:core:open_folder"],
            citation_anchor_refs: vec!["citation:docs:onboarding:first-useful-work#open-folder"],
            keyboard_path_refs: vec!["keyboard:path:palette.open_folder"],
            screen_reader_label_refs: vec!["sr:docs:onboarding:open-folder:title"],
            recovery_route_refs: vec!["route:recovery:open-folder-fallback"],
            source_language_route_ref: "route:docs:source-language:open",
            fallback_state_refs: vec!["fallback-truth:es-mx:claimed:stable"],
            accessibility_fixture_refs: vec![
                "fixtures/i18n/locale_surface_examples/docs_tour_localized_text_citation_parity.yaml",
                "fixtures/accessibility/representation_review_cases/locale_pack_partial_fallback.yaml",
            ],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
        }),
        translated_surface_row(TranslatedSurfaceSeed {
            row_id: "surface-parity:tour:open-folder",
            claim_class: LocalizationClaimClass::StableLocalized,
            surface_kind: TranslatedSurfaceKind::GuidedTour,
            message_id_refs: vec!["msg:docs:onboarding:open-folder:summary"],
            command_id_refs: vec!["cmd:core:open_folder"],
            citation_anchor_refs: vec!["citation:tour:first-run#open-folder"],
            keyboard_path_refs: vec!["keyboard:path:tour.open_folder"],
            screen_reader_label_refs: vec!["sr:tour:first-run:open-folder"],
            recovery_route_refs: vec!["route:recovery:open-folder-fallback"],
            source_language_route_ref: "route:tour:source-language:open",
            fallback_state_refs: vec!["fallback-truth:es-mx:claimed:stable"],
            accessibility_fixture_refs: vec![
                "fixtures/i18n/locale_surface_examples/docs_tour_localized_text_citation_parity.yaml",
                "fixtures/ux/onboarding_help_search_alpha/source_language_fallback.json",
            ],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
        }),
        translated_surface_row(TranslatedSurfaceSeed {
            row_id: "surface-parity:auth:recovery-source",
            claim_class: LocalizationClaimClass::StableLocalized,
            surface_kind: TranslatedSurfaceKind::AuthRecovery,
            message_id_refs: vec!["msg:auth:recovery:source-language-fallback"],
            command_id_refs: Vec::new(),
            citation_anchor_refs: vec!["citation:auth:system-browser#device-code-fallback"],
            keyboard_path_refs: vec!["keyboard:path:auth.device_code_fallback"],
            screen_reader_label_refs: vec!["sr:auth:recovery:source-language-fallback"],
            recovery_route_refs: vec![
                "route:auth:device-code-fallback",
                "route:auth:system-browser-retry",
            ],
            source_language_route_ref: "route:auth:source-language:open",
            fallback_state_refs: vec!["fallback-truth:es-mx:claimed:stable"],
            accessibility_fixture_refs: vec![
                "fixtures/web/browser_surface_cases/auth_handoff_system_browser_device_code_fallback.yaml",
                "fixtures/ux/open_in_browser_fallback_cases/auth_confirmation_device_code_fallback_offered.yaml",
            ],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
        }),
        translated_surface_row(TranslatedSurfaceSeed {
            row_id: "surface-parity:help:glossary-source-fallback",
            claim_class: LocalizationClaimClass::DegradedSourceLanguageOnly,
            surface_kind: TranslatedSurfaceKind::HelpGlossaryCard,
            message_id_refs: vec!["msg:docs:onboarding:open-folder:summary"],
            command_id_refs: vec!["cmd:core:open_folder"],
            citation_anchor_refs: vec!["citation:help:glossary#open-folder"],
            keyboard_path_refs: vec!["keyboard:path:help.open_source_language"],
            screen_reader_label_refs: vec!["sr:help:locale-fallback:source-language"],
            recovery_route_refs: vec!["route:docs:source-language:open"],
            source_language_route_ref: "route:help:source-language:open",
            fallback_state_refs: vec!["fallback-truth:pt-br:community:source-language"],
            accessibility_fixture_refs: vec![
                "fixtures/i18n/m3/locale_fallback/help_about_projection.json",
                "fixtures/i18n/m3/locale_fallback/support_export.json",
            ],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
        })
        .with_result(TranslatedSurfaceParityResult::SourceLanguageFallbackPassed),
        translated_surface_row(TranslatedSurfaceSeed {
            row_id: "surface-parity:cli:doctor-help",
            claim_class: LocalizationClaimClass::StableLocalized,
            surface_kind: TranslatedSurfaceKind::CliHumanHelp,
            message_id_refs: vec!["msg:doctor:profile-schema-drift:human"],
            command_id_refs: vec!["cmd:doctor:profile"],
            citation_anchor_refs: vec!["citation:cli:doctor#profile-schema-drift"],
            keyboard_path_refs: vec!["keyboard:path:terminal.copy_help"],
            screen_reader_label_refs: vec!["sr:cli:doctor:profile-schema-drift"],
            recovery_route_refs: vec!["route:support:doctor-profile-schema-drift"],
            source_language_route_ref: "flag:--locale-neutral",
            fallback_state_refs: vec!["fallback-truth:es-mx:claimed:stable"],
            accessibility_fixture_refs: vec![
                "fixtures/i18n/locale_surface_examples/cli_help_localized_prose_stable_flags_and_json_keys.yaml",
                "fixtures/accessibility/ime_and_text_cases/mixed_direction_technical_strings.yaml",
            ],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField,
        }),
    ]
}

fn seeded_release_gate_rows() -> Vec<ReleaseGateProofRow> {
    let command = "cargo test -p aureline-i18n --test stable_locale_lifecycle_translated_surface_parity --locked";
    vec![
        release_gate(
            "release-gate:locale-pack-signing",
            "locale_pack_signing",
            command,
        ),
        release_gate(
            "release-gate:fallback-chain",
            "fallback_chain_truth",
            command,
        ),
        release_gate(
            "release-gate:stable-message-ids",
            "stable_message_ids",
            command,
        ),
        release_gate(
            "release-gate:translated-surface-parity",
            "translated_surface_parity",
            command,
        ),
        release_gate(
            "release-gate:pseudoloc-text-expansion",
            "pseudoloc_text_expansion",
            command,
        ),
        release_gate(
            "release-gate:rtl-bidi-ime-font",
            "rtl_bidi_ime_font_fallback",
            command,
        ),
    ]
}

struct TranslatedSurfaceSeed<'a> {
    row_id: &'a str,
    claim_class: LocalizationClaimClass,
    surface_kind: TranslatedSurfaceKind,
    message_id_refs: Vec<&'a str>,
    command_id_refs: Vec<&'a str>,
    citation_anchor_refs: Vec<&'a str>,
    keyboard_path_refs: Vec<&'a str>,
    screen_reader_label_refs: Vec<&'a str>,
    recovery_route_refs: Vec<&'a str>,
    source_language_route_ref: &'a str,
    fallback_state_refs: Vec<&'a str>,
    accessibility_fixture_refs: Vec<&'a str>,
    machine_output_locale_class: MachineOutputLocaleClass,
}

fn translated_surface_row(seed: TranslatedSurfaceSeed<'_>) -> TranslatedSurfaceParityRow {
    TranslatedSurfaceParityRow {
        row_id: seed.row_id.to_owned(),
        claim_class: seed.claim_class,
        surface_kind: seed.surface_kind,
        message_id_refs: strings(seed.message_id_refs),
        command_id_refs: strings(seed.command_id_refs),
        citation_anchor_refs: strings(seed.citation_anchor_refs),
        keyboard_path_refs: strings(seed.keyboard_path_refs),
        screen_reader_label_refs: strings(seed.screen_reader_label_refs),
        recovery_route_refs: strings(seed.recovery_route_refs),
        source_language_route_ref: seed.source_language_route_ref.to_owned(),
        cli_machine_keys_locale_neutral: true,
        human_help_may_localize: true,
        machine_output_locale_class: seed.machine_output_locale_class,
        fallback_state_refs: strings(seed.fallback_state_refs),
        accessibility_fixture_refs: strings(seed.accessibility_fixture_refs),
        parity_result: TranslatedSurfaceParityResult::Passed,
    }
}

impl TranslatedSurfaceParityRow {
    fn with_result(mut self, parity_result: TranslatedSurfaceParityResult) -> Self {
        self.parity_result = parity_result;
        self
    }
}

fn release_gate(row_id: &str, proof_kind: &str, command: &str) -> ReleaseGateProofRow {
    ReleaseGateProofRow {
        row_id: row_id.to_owned(),
        proof_kind: proof_kind.to_owned(),
        command: command.to_owned(),
        fixture_refs: vec![
            STABLE_LOCALE_LIFECYCLE_PARITY_FIXTURE_REF.to_owned(),
            "fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json".to_owned(),
        ],
        artifact_refs: vec![
            "artifacts/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md"
                .to_owned(),
            "docs/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md"
                .to_owned(),
        ],
        required_for_claimed_localized_rows: true,
        gate_state: ClaimGateState::Green,
    }
}

fn derive_summary(
    fallback_rows: &[FallbackChainTruthRow],
    surface_rows: &[TranslatedSurfaceParityRow],
    gate_rows: &[ReleaseGateProofRow],
) -> LocalePackParitySummary {
    let claimed_localized_rows = fallback_rows
        .iter()
        .filter(|row| row.claim_class == LocalizationClaimClass::StableLocalized)
        .count()
        + surface_rows
            .iter()
            .filter(|row| row.claim_class == LocalizationClaimClass::StableLocalized)
            .count();
    let required_gates_green = gate_rows
        .iter()
        .filter(|row| row.required_for_claimed_localized_rows)
        .all(|row| row.gate_state == ClaimGateState::Green);
    let green_claimed_localized_rows = if required_gates_green {
        claimed_localized_rows
    } else {
        0
    };
    let degraded_source_language_rows = fallback_rows
        .iter()
        .filter(|row| row.claim_class == LocalizationClaimClass::DegradedSourceLanguageOnly)
        .count()
        + surface_rows
            .iter()
            .filter(|row| row.claim_class == LocalizationClaimClass::DegradedSourceLanguageOnly)
            .count();
    let blocked_rows = surface_rows
        .iter()
        .filter(|row| row.parity_result == TranslatedSurfaceParityResult::Blocked)
        .count()
        + gate_rows
            .iter()
            .filter(|row| row.gate_state == ClaimGateState::Blocked)
            .count();

    LocalePackParitySummary {
        claimed_localized_rows,
        green_claimed_localized_rows,
        degraded_source_language_rows,
        blocked_rows,
        promotion_state: if blocked_rows == 0 && required_gates_green {
            ClaimGateState::Green
        } else if blocked_rows == 0 {
            ClaimGateState::Narrowed
        } else {
            ClaimGateState::Blocked
        },
    }
}

fn validate_lifecycle_windows(
    windows: &[LocalePackLifecycleWindow],
    findings: &mut Vec<crate::LocalePackValidationFinding>,
) {
    let mut ids = BTreeSet::new();
    let mut source_classes = BTreeSet::new();

    for window in windows {
        if !ids.insert(window.window_id.as_str()) {
            findings.push(crate::LocalePackValidationFinding::new(
                window.window_id.clone(),
                "duplicate lifecycle window id",
            ));
        }
        source_classes.insert(window.source_class);
        if window.rollback_ref.trim().is_empty()
            || window.compatibility_min_build_ref.trim().is_empty()
            || window.compatibility_max_build_ref.trim().is_empty()
        {
            findings.push(crate::LocalePackValidationFinding::new(
                window.window_id.clone(),
                "lifecycle window must cite compatibility window and rollback ref",
            ));
        }
        if window.backs_claimed_localized_row
            && (!window.signature_state.may_render() || !window.version_match_state.may_render())
        {
            findings.push(crate::LocalePackValidationFinding::new(
                window.window_id.clone(),
                "claimed localized row requires renderable signature and compatible version",
            ));
        }
        if matches!(
            window.source_class,
            LocalePackSourceClass::FirstPartyLocalePack
                | LocalePackSourceClass::ReviewedCommunityPack
        ) && !matches!(
            window.lifecycle_state,
            LocaleLifecycleState::MissingPackSourceLanguageOnly
                | LocaleLifecycleState::SignatureFailedSourceLanguageOnly
        ) && window.mirror_receipt_refs.is_empty()
            && window.offline_bundle_ref.is_none()
        {
            findings.push(crate::LocalePackValidationFinding::new(
                window.window_id.clone(),
                "non-source locale pack must preserve mirror or offline metadata",
            ));
        }
        if window.source_class == LocalePackSourceClass::ReviewedCommunityPack
            && window.reviewed_community_decision_ref.is_none()
        {
            findings.push(crate::LocalePackValidationFinding::new(
                window.window_id.clone(),
                "reviewed community pack must cite review decision",
            ));
        }
    }

    for required in [
        LocalePackSourceClass::FirstPartySourceLanguage,
        LocalePackSourceClass::FirstPartyLocalePack,
        LocalePackSourceClass::ReviewedCommunityPack,
    ] {
        if !source_classes.contains(&required) {
            findings.push(crate::LocalePackValidationFinding::new(
                STABLE_LOCALE_LIFECYCLE_PARITY_PACKET_ID,
                format!("lifecycle windows are missing {required:?}"),
            ));
        }
    }
}

fn validate_stable_message_rows(
    rows: &[StableMessageIdProofRow],
    findings: &mut Vec<crate::LocalePackValidationFinding>,
) {
    let mut ids = BTreeSet::new();
    for row in rows {
        if !ids.insert(row.message_id.as_str()) {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "duplicate stable message id",
            ));
        }
        if !row.stable_refs.has_anchor()
            || row.translation_pack_refs.is_empty()
            || !row.command_id_stable
            || !row.schema_or_diagnostic_id_stable
            || !row.semantic_action_id_stable
            || !row.machine_identifier_locale_neutral
            || row.routed_by_localized_prose
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "message row must preserve stable ids and never route by localized prose",
            ));
        }
    }
}

fn validate_fallback_truth_rows(
    rows: &[FallbackChainTruthRow],
    findings: &mut Vec<crate::LocalePackValidationFinding>,
) {
    let mut ids = BTreeSet::new();
    for row in rows {
        if !ids.insert(row.row_id.as_str()) {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "duplicate fallback truth row id",
            ));
        }
        if row.fallback_chain.first() != Some(&row.requested_locale)
            || row.fallback_chain.last() != Some(&row.source_language_locale)
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "fallback chain must be requested locale to source language",
            ));
        }
        if !row.visible_in_settings
            || !row.visible_in_diagnostics
            || !row.visible_in_support_export
            || row.open_in_source_language_route_ref.trim().is_empty()
            || !row.non_blocking_core_use
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "fallback truth must be visible, source-language reachable, and non-blocking",
            ));
        }
    }
}

fn validate_translated_surface_rows(
    rows: &[TranslatedSurfaceParityRow],
    findings: &mut Vec<crate::LocalePackValidationFinding>,
) {
    let mut kinds = BTreeSet::new();
    for row in rows {
        kinds.insert(row.surface_kind);
        if row.message_id_refs.is_empty()
            || row.keyboard_path_refs.is_empty()
            || row.screen_reader_label_refs.is_empty()
            || row.recovery_route_refs.is_empty()
            || row.source_language_route_ref.trim().is_empty()
            || row.fallback_state_refs.is_empty()
            || row.accessibility_fixture_refs.is_empty()
            || row.parity_result == TranslatedSurfaceParityResult::Blocked
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "translated surface row must preserve parity, accessibility, fallback, and source-language routes",
            ));
        }
        if matches!(
            row.surface_kind,
            TranslatedSurfaceKind::Docs | TranslatedSurfaceKind::GuidedTour
        ) && (row.command_id_refs.is_empty() || row.citation_anchor_refs.is_empty())
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "docs and tour rows must preserve command ids and citations",
            ));
        }
        if row.surface_kind == TranslatedSurfaceKind::CliHumanHelp
            && (!row.cli_machine_keys_locale_neutral
                || row.machine_output_locale_class
                    != MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField)
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "CLI human help must keep machine keys locale-neutral",
            ));
        }
    }

    for required in [
        TranslatedSurfaceKind::Docs,
        TranslatedSurfaceKind::GuidedTour,
        TranslatedSurfaceKind::AuthRecovery,
        TranslatedSurfaceKind::HelpGlossaryCard,
        TranslatedSurfaceKind::CliHumanHelp,
    ] {
        if !kinds.contains(&required) {
            findings.push(crate::LocalePackValidationFinding::new(
                STABLE_LOCALE_LIFECYCLE_PARITY_PACKET_ID,
                format!("translated surface rows are missing {required:?}"),
            ));
        }
    }
}

fn validate_release_gate_rows(
    rows: &[ReleaseGateProofRow],
    findings: &mut Vec<crate::LocalePackValidationFinding>,
) {
    let mut proof_kinds = BTreeSet::new();
    for row in rows {
        proof_kinds.insert(row.proof_kind.as_str());
        if !row.required_for_claimed_localized_rows
            || row.gate_state != ClaimGateState::Green
            || row.command.trim().is_empty()
            || row.fixture_refs.is_empty()
            || row.artifact_refs.is_empty()
        {
            findings.push(crate::LocalePackValidationFinding::new(
                row.row_id.clone(),
                "release gate row must be green and proof-backed for claimed localized rows",
            ));
        }
    }

    for required in [
        "locale_pack_signing",
        "fallback_chain_truth",
        "stable_message_ids",
        "translated_surface_parity",
        "pseudoloc_text_expansion",
        "rtl_bidi_ime_font_fallback",
    ] {
        if !proof_kinds.contains(required) {
            findings.push(crate::LocalePackValidationFinding::new(
                STABLE_LOCALE_LIFECYCLE_PARITY_PACKET_ID,
                format!("release gates are missing {required}"),
            ));
        }
    }
}

fn validate_summary(
    packet: &StableLocaleLifecycleParityPacket,
    findings: &mut Vec<crate::LocalePackValidationFinding>,
) {
    let expected = derive_summary(
        &packet.fallback_truth_rows,
        &packet.translated_surface_rows,
        &packet.release_gate_rows,
    );
    if packet.summary != expected {
        findings.push(crate::LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "stable locale lifecycle summary drifted from row state",
        ));
    }
    if packet.summary.claimed_localized_rows != packet.summary.green_claimed_localized_rows {
        findings.push(crate::LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "every claimed localized row must be green",
        ));
    }
    if packet.summary.blocked_rows != 0 || packet.summary.promotion_state != ClaimGateState::Green {
        findings.push(crate::LocalePackValidationFinding::new(
            packet.packet_id.clone(),
            "stable locale lifecycle packet contains blocked rows",
        ));
    }
}

fn strings(values: Vec<&str>) -> Vec<String> {
    values.into_iter().map(str::to_owned).collect()
}
