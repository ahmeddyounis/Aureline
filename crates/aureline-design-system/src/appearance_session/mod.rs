//! Appearance-session beta contract projection.
//!
//! This module composes the design-side appearance-session schemas, theme
//! package manifest, token-overlay fixture, imported-theme mapping report, and
//! OS live-change matrix into one release-review packet. It does not render a
//! theme; it proves the appearance object model is versioned, checkpointed,
//! recoverable, and honest about live apply versus reload or confirm posture.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use aureline_ui::density::DensityClass;
use aureline_ui::themes::{
    first_party_theme_package_manifest, imported_theme_mapping_report_with_warnings,
    AccessibilityPostureClass, AppearanceAxis, AppearanceSessionRecord,
    ImportedThemeParityReadiness, LiveFollowSystemPolicyRecord, LiveUpdateClass,
};
use aureline_ui::tokens::ThemeClass;
use serde::{Deserialize, Serialize};

use crate::{
    DesignSystemFinding, GateStateClass, DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
    DESIGN_SYSTEM_FINDING_RECORD_KIND,
};

const STEADY_SESSION_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/appearance_session_cases/steady_state_follow_system_dark_signal.yaml"
));
const LIVE_POLICY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/appearance_session_cases/live_follow_system_policy_default_profile.yaml"
));
const TOKEN_OVERLAY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/appearance_session_cases/workspace_overlay_overridden_deprecated_unmapped.yaml"
));
const IMPORT_REPORT_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/theme_support_cases/imported_translated_theme_mapping_report_with_warnings.yaml"
));
const LIVE_CHANGE_MATRIX_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/design/appearance_live_change_matrix.yaml"
));

/// Record kind for [`AppearanceSessionBetaContract`].
pub const APPEARANCE_SESSION_BETA_RECORD_KIND: &str = "appearance_session_beta_contract_record";

/// Shared contract reference for appearance runtime beta packets.
pub const APPEARANCE_SESSION_BETA_SHARED_CONTRACT_REF: &str =
    "appearance:theme_package_session_overlay_import:v1";

/// Error emitted while assembling a seeded appearance-session beta packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppearanceSessionContractError {
    /// One of the embedded YAML fixtures failed to parse.
    ParseFailed {
        artifact_ref: String,
        detail: String,
    },
    /// A lower-level contract loader rejected the embedded fixture.
    InvalidFixture {
        artifact_ref: String,
        detail: String,
    },
}

impl fmt::Display for AppearanceSessionContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFailed {
                artifact_ref,
                detail,
            } => write!(f, "failed to parse {artifact_ref}: {detail}"),
            Self::InvalidFixture {
                artifact_ref,
                detail,
            } => write!(f, "invalid {artifact_ref}: {detail}"),
        }
    }
}

impl std::error::Error for AppearanceSessionContractError {}

/// Package-level coverage published by the active first-party theme package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePackageContractSummary {
    /// Design-side fixture or artifact backing the package summary.
    pub fixture_ref: String,
    /// Package manifest schema consumed by this summary.
    pub schema_ref: String,
    /// Theme-package manifest reference used by support and audit rows.
    pub theme_package_manifest_ref: String,
    /// Theme-package revision reference used by support and audit rows.
    pub theme_package_revision_ref: String,
    /// Canonical package id.
    pub package_id: String,
    /// Human-readable package version label.
    pub package_version_label: String,
    /// Supported theme classes declared by the package.
    pub supported_theme_classes: Vec<String>,
    /// Supported density classes declared by the package.
    pub supported_density_classes: Vec<String>,
    /// Supported motion postures declared by the package.
    pub supported_motion_postures: Vec<String>,
    /// Minimum text contrast target by theme class.
    pub minimum_text_contrast_targets: BTreeMap<String, f32>,
    /// True when dark, light, and both high-contrast modes are present.
    pub first_party_four_mode_floor_complete: bool,
}

/// Live follow-system policy coverage for the active appearance session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveFollowPolicySummary {
    /// Policy fixture reference.
    pub fixture_ref: String,
    /// Policy schema reference.
    pub schema_ref: String,
    /// Stable policy id.
    pub live_follow_system_policy_id: String,
    /// Appearance session cited by the policy.
    pub appearance_session_ref: String,
    /// Axes with live, no-review apply.
    pub live_no_review_axes: Vec<String>,
    /// Axes with live checkpointed apply.
    pub live_checkpointed_axes: Vec<String>,
    /// Axes held for explicit user confirmation.
    pub confirm_required_axes: Vec<String>,
    /// Axes blocked by policy.
    pub policy_blocked_axes: Vec<String>,
    /// True when all required appearance axes have one policy row.
    pub all_required_axes_declared: bool,
    /// True when checkpoint and confirm booleans match live class semantics.
    pub checkpoint_and_confirm_rules_valid: bool,
}

/// Token-overlay state summary proving unsupported slots survive round trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenOverlayContractSummary {
    /// Token-overlay fixture reference.
    pub fixture_ref: String,
    /// Token-overlay schema reference.
    pub schema_ref: String,
    /// Stable token-overlay id.
    pub token_overlay_id: String,
    /// Appearance session that owns this overlay.
    pub appearance_session_ref: String,
    /// Effective overlay scope.
    pub overlay_scope: String,
    /// Effective validation state.
    pub validation_state: String,
    /// Inherited token count.
    pub inherited_count: u32,
    /// Overridden token count.
    pub overridden_count: u32,
    /// Deprecated token count.
    pub deprecated_count: u32,
    /// Unmapped token count.
    pub unmapped_count: u32,
    /// True when summary counts match the visible per-token rows.
    pub summary_counts_match_entries: bool,
    /// True when deprecated entries carry replacement refs.
    pub deprecated_replacements_visible: bool,
    /// True when unmapped entries end in inert fallback-chain steps.
    pub unmapped_entries_preserved_inert: bool,
    /// True when fallback-chain rows are reviewable.
    pub fallback_chain_visible: bool,
}

/// Imported-theme mapping report coverage and parity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeImportMappingContractSummary {
    /// Mapping-report fixture reference.
    pub fixture_ref: String,
    /// Mapping-report schema reference.
    pub schema_ref: String,
    /// Stable mapping report id.
    pub report_id: String,
    /// Source ecosystem named by the importer.
    pub source_ecosystem: String,
    /// Source tool version named by the importer.
    pub source_tool_version: String,
    /// Target theme classes covered by the import.
    pub target_theme_classes: Vec<String>,
    /// Number of translated source slots.
    pub translated_slot_count: u32,
    /// Number of substituted fallback slots.
    pub substituted_with_fallback_count: u32,
    /// Number of unsupported source slots.
    pub unsupported_slot_count: u32,
    /// Number of unresolved source slots.
    pub unresolved_mapping_count: u32,
    /// Number of blocked protected-cue honesty rows.
    pub blocked_honesty_count: u32,
    /// Number of deprecated replacement rows.
    pub deprecated_replacement_count: u32,
    /// Syntax-token coverage percent.
    pub syntax_coverage_percent: u32,
    /// Syntax scopes still unresolved.
    pub syntax_unresolved_scope_count: u32,
    /// Rollback checkpoint reference.
    pub appearance_checkpoint_ref: String,
    /// Rollback handle reference.
    pub rollback_ref: String,
    /// User-facing theme-import report ref.
    pub linked_ux_import_report_ref: String,
    /// Readiness for imported-theme parity claims.
    pub parity_readiness: ImportedThemeParityClass,
    /// True when all protected cue rows are present and passed.
    pub protected_cues_preserved: bool,
}

/// Imported-theme parity readiness projected into JSON fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedThemeParityClass {
    /// The report can claim full parity.
    FullParityClaimable,
    /// The report is usable with visible gaps.
    PartialWithVisibleGaps,
    /// The report blocks durable apply or parity claims.
    Blocked,
}

impl From<ImportedThemeParityReadiness> for ImportedThemeParityClass {
    fn from(value: ImportedThemeParityReadiness) -> Self {
        match value {
            ImportedThemeParityReadiness::FullParityClaimable => Self::FullParityClaimable,
            ImportedThemeParityReadiness::PartialWithVisibleGaps => Self::PartialWithVisibleGaps,
            ImportedThemeParityReadiness::Blocked => Self::Blocked,
        }
    }
}

/// Live OS appearance-change coverage projected from the platform matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveOsAppearanceChangeSummary {
    /// Matrix artifact reference.
    pub matrix_ref: String,
    /// Matrix schema version.
    pub matrix_schema_version: u32,
    /// Claimed desktop profile count in the matrix.
    pub claimed_profile_count: usize,
    /// Total matrix row count.
    pub row_count: usize,
    /// OS-triggered axes covered by the matrix.
    pub axes_covered: Vec<String>,
    /// Shell axes that apply live with no checkpoint.
    pub shell_live_axes: Vec<String>,
    /// Shell axes that apply live behind a checkpoint.
    pub shell_checkpointed_axes: Vec<String>,
    /// Shell axes held for explicit confirmation.
    pub shell_confirm_required_axes: Vec<String>,
    /// Rows requiring embedded or extension surface reload.
    pub embedded_reload_required_rows: usize,
    /// Rows requiring full application restart.
    pub full_restart_required_rows: usize,
    /// True when every reload/restart row requires disclosure.
    pub all_reload_or_restart_rows_disclosed: bool,
    /// True when each claimed profile has all shell OS axes covered.
    pub all_claimed_profiles_cover_shell_axes: bool,
    /// True when forced-colors rows use live, checkpointed, or disclosed reload posture.
    pub forced_colors_rows_explicit: bool,
}

/// One protected cue family that the appearance contract may not collapse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedCueContractRow {
    /// Protected cue class.
    pub protected_cue_class: String,
    /// True when non-color cues are required.
    pub non_color_cues_required: bool,
    /// True when high-contrast mode preserves the cue.
    pub preserved_in_high_contrast: bool,
    /// True when forced-colors mode preserves the cue.
    pub preserved_in_forced_colors: bool,
}

/// Release-review packet for beta appearance-session runtime contracts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppearanceSessionBetaContract {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared appearance contract reference.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Source documents and artifacts governing this packet.
    pub source_refs: Vec<String>,
    /// Machine-readable schemas consumed by this packet.
    pub schema_refs: Vec<String>,
    /// Runtime consumers that must consume this contract family.
    pub runtime_consumer_refs: Vec<String>,
    /// Active first-party package coverage summary.
    pub theme_package: ThemePackageContractSummary,
    /// In-effect appearance session record.
    pub appearance_session: AppearanceSessionRecord,
    /// Live follow-system policy cited by the session.
    pub live_follow_system_policy: LiveFollowSystemPolicyRecord,
    /// Policy summary for quick release review.
    pub live_follow_system_summary: LiveFollowPolicySummary,
    /// Token-overlay summary proving unknown slots are not dropped.
    pub token_overlay: TokenOverlayContractSummary,
    /// Imported-theme mapping summary proving parity claims cite a report.
    pub import_mapping_report: ThemeImportMappingContractSummary,
    /// OS live-change and reload/restart posture summary.
    pub live_os_change_matrix: LiveOsAppearanceChangeSummary,
    /// Protected cue classes that must survive theme/package/import changes.
    pub protected_cue_preservation: Vec<ProtectedCueContractRow>,
    /// Aggregate findings emitted by the validator.
    pub findings: Vec<DesignSystemFinding>,
    /// Gate state resolved from findings and disclosed imported-theme gaps.
    pub gate_state: GateStateClass,
    /// True when raw token values, raw screenshots, paths, URLs, and user content are excluded.
    pub raw_private_material_excluded: bool,
}

/// Builds the seeded appearance-session beta contract packet.
pub fn try_seeded_appearance_session_beta_contract(
) -> Result<AppearanceSessionBetaContract, AppearanceSessionContractError> {
    let appearance_session: AppearanceSessionRecord = parse_yaml(
        STEADY_SESSION_YAML,
        "fixtures/design/appearance_session_cases/steady_state_follow_system_dark_signal.yaml",
    )?;
    let live_follow_system_policy: LiveFollowSystemPolicyRecord = parse_yaml(
        LIVE_POLICY_YAML,
        "fixtures/design/appearance_session_cases/live_follow_system_policy_default_profile.yaml",
    )?;
    let token_overlay_doc: TokenOverlayDoc = parse_yaml(
        TOKEN_OVERLAY_YAML,
        "fixtures/design/appearance_session_cases/workspace_overlay_overridden_deprecated_unmapped.yaml",
    )?;
    let import_mapping_doc: ThemeImportMappingReportDoc = parse_yaml(
        IMPORT_REPORT_YAML,
        "fixtures/design/theme_support_cases/imported_translated_theme_mapping_report_with_warnings.yaml",
    )?;
    let live_change_matrix_doc: LiveChangeMatrixDoc = parse_yaml(
        LIVE_CHANGE_MATRIX_YAML,
        "artifacts/design/appearance_live_change_matrix.yaml",
    )?;

    let theme_package = theme_package_summary()?;
    let live_follow_system_summary = live_follow_policy_summary(&live_follow_system_policy);
    let token_overlay = token_overlay_doc.summary();
    let import_mapping_report = import_mapping_summary(&import_mapping_doc)?;
    let live_os_change_matrix = live_os_change_summary(&live_change_matrix_doc);
    let protected_cue_preservation = protected_cue_rows();

    let mut packet = AppearanceSessionBetaContract {
        record_kind: APPEARANCE_SESSION_BETA_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        shared_contract_ref: APPEARANCE_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
        packet_id: "appearance-session-beta:theme-import-live-change".to_owned(),
        source_refs: vec![
            ".t2/docs/Aureline_Technical_Design_Document.md#7.1.17".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#9.47".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#8.9".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md#12.7".to_owned(),
            "docs/design/appearance_session_contract.md".to_owned(),
            "docs/design/theme_package_manifest_contract.md".to_owned(),
            "docs/design/theme_support_and_inheritance_contract.md".to_owned(),
            "docs/design/os_appearance_change_matrix.md".to_owned(),
            "docs/ux/appearance_import_and_checkpoint_contract.md".to_owned(),
        ],
        schema_refs: vec![
            "schemas/design/theme_package_manifest.schema.json".to_owned(),
            "schemas/design/appearance_session.schema.json".to_owned(),
            "schemas/design/token_overlay.schema.json".to_owned(),
            "schemas/design/theme_import_mapping_report.schema.json".to_owned(),
            "schemas/ux/theme_package_manifest.schema.json".to_owned(),
            "schemas/ux/appearance_checkpoint.schema.json".to_owned(),
            "schemas/ux/theme_import_report.schema.json".to_owned(),
        ],
        runtime_consumer_refs: vec![
            "crates/aureline-ui/src/themes/session.rs".to_owned(),
            "crates/aureline-ui/src/themes/package.rs".to_owned(),
            "crates/aureline-ui/src/themes/import_review.rs".to_owned(),
            "crates/aureline-shell/src/bootstrap/native_shell.rs".to_owned(),
            "crates/aureline-settings/src/lib.rs".to_owned(),
            "crates/aureline-install/src/lib.rs".to_owned(),
            "crates/aureline-design-system/src/appearance_session/mod.rs".to_owned(),
        ],
        theme_package,
        appearance_session,
        live_follow_system_policy,
        live_follow_system_summary,
        token_overlay,
        import_mapping_report,
        live_os_change_matrix,
        protected_cue_preservation,
        findings: Vec::new(),
        gate_state: GateStateClass::Pass,
        raw_private_material_excluded: true,
    };

    let findings = audit_appearance_session_beta_contract(&packet);
    packet.gate_state = if !findings.is_empty() {
        GateStateClass::Block
    } else if packet.import_mapping_report.parity_readiness
        == ImportedThemeParityClass::PartialWithVisibleGaps
    {
        GateStateClass::PassWithDisclosedGap
    } else {
        GateStateClass::Pass
    };
    packet.findings = findings;
    Ok(packet)
}

/// Builds the seeded appearance-session beta contract packet.
pub fn seeded_appearance_session_beta_contract() -> AppearanceSessionBetaContract {
    try_seeded_appearance_session_beta_contract()
        .expect("seeded appearance-session beta contract must parse")
}

/// Validates an appearance-session beta contract packet.
pub fn validate_appearance_session_beta_contract(
    packet: &AppearanceSessionBetaContract,
) -> Result<(), Vec<DesignSystemFinding>> {
    let mut findings = packet.findings.clone();
    findings.extend(audit_appearance_session_beta_contract(packet));
    if findings.is_empty()
        && matches!(
            packet.gate_state,
            GateStateClass::Pass | GateStateClass::PassWithDisclosedGap
        )
    {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Audits an appearance-session beta contract packet and returns all findings.
pub fn audit_appearance_session_beta_contract(
    packet: &AppearanceSessionBetaContract,
) -> Vec<DesignSystemFinding> {
    let mut findings = Vec::new();
    if packet.record_kind != APPEARANCE_SESSION_BETA_RECORD_KIND {
        findings.push(finding(
            "appearance_beta.record_kind.invalid",
            "record_kind",
            "appearance-session beta record kind is invalid",
        ));
    }
    if packet.schema_version != DESIGN_SYSTEM_BETA_SCHEMA_VERSION {
        findings.push(finding(
            "appearance_beta.schema_version.invalid",
            "schema_version",
            "appearance-session beta schema version is unsupported",
        ));
    }
    if !packet.theme_package.first_party_four_mode_floor_complete {
        findings.push(finding(
            "appearance_beta.theme_package.four_mode_floor_missing",
            "theme_package.supported_theme_classes",
            "first-party theme package does not cover dark, light, and both high-contrast rows",
        ));
    }
    if packet.appearance_session.appearance_session_schema_version
        != DESIGN_SYSTEM_BETA_SCHEMA_VERSION
    {
        findings.push(finding(
            "appearance_beta.session.schema_version.invalid",
            "appearance_session.appearance_session_schema_version",
            "appearance session schema version is unsupported",
        ));
    }
    if packet
        .appearance_session
        .active_theme_package_ref
        .is_empty()
        || packet
            .appearance_session
            .active_theme_revision_ref
            .is_empty()
    {
        findings.push(finding(
            "appearance_beta.session.theme_ref_missing",
            "appearance_session.active_theme_package_ref",
            "appearance session must cite an active package and revision",
        ));
    }
    if packet.appearance_session.live_follow_system_policy_ref
        != packet
            .live_follow_system_policy
            .live_follow_system_policy_id
    {
        findings.push(finding(
            "appearance_beta.session.policy_ref_mismatch",
            "appearance_session.live_follow_system_policy_ref",
            "appearance session must cite the emitted live follow-system policy",
        ));
    }
    if !packet.live_follow_system_summary.all_required_axes_declared {
        findings.push(finding(
            "appearance_beta.live_policy.axis_missing",
            "live_follow_system_policy.axes",
            "live follow-system policy must declare every appearance axis",
        ));
    }
    if !packet
        .live_follow_system_summary
        .checkpoint_and_confirm_rules_valid
    {
        findings.push(finding(
            "appearance_beta.live_policy.checkpoint_rules_invalid",
            "live_follow_system_policy.axes",
            "live follow-system policy checkpoint and confirm booleans do not match live classes",
        ));
    }
    if !packet.token_overlay.summary_counts_match_entries {
        findings.push(finding(
            "appearance_beta.token_overlay.summary_count_mismatch",
            "token_overlay.summary_counts",
            "token-overlay summary counts must match visible entries",
        ));
    }
    if !packet.token_overlay.deprecated_replacements_visible {
        findings.push(finding(
            "appearance_beta.token_overlay.deprecated_replacement_missing",
            "token_overlay.entries",
            "deprecated overlay entries must carry replacement refs",
        ));
    }
    if !packet.token_overlay.unmapped_entries_preserved_inert {
        findings.push(finding(
            "appearance_beta.token_overlay.unmapped_not_inert",
            "token_overlay.fallback_chain",
            "unmapped overlay entries must survive as inert fallback-chain rows",
        ));
    }
    if packet.import_mapping_report.rollback_ref.is_empty()
        || packet
            .import_mapping_report
            .appearance_checkpoint_ref
            .is_empty()
    {
        findings.push(finding(
            "appearance_beta.import_mapping.rollback_missing",
            "import_mapping_report.rollback_ref",
            "import mapping report must cite both checkpoint and rollback refs",
        ));
    }
    if !packet.import_mapping_report.protected_cues_preserved {
        findings.push(finding(
            "appearance_beta.import_mapping.protected_cue_failed",
            "import_mapping_report.protected_cue_honesty_checks",
            "import mapping report must preserve trust, policy, severity, and source-integrity cues",
        ));
    }
    if !packet
        .live_os_change_matrix
        .all_claimed_profiles_cover_shell_axes
    {
        findings.push(finding(
            "appearance_beta.live_os.shell_axis_missing",
            "live_os_change_matrix.rows",
            "each claimed desktop profile must cover shell OS appearance axes",
        ));
    }
    if !packet
        .live_os_change_matrix
        .all_reload_or_restart_rows_disclosed
    {
        findings.push(finding(
            "appearance_beta.live_os.reload_disclosure_missing",
            "live_os_change_matrix.rows",
            "reload or restart appearance rows must require disclosure",
        ));
    }
    if !packet.live_os_change_matrix.forced_colors_rows_explicit {
        findings.push(finding(
            "appearance_beta.live_os.forced_colors_not_explicit",
            "live_os_change_matrix.rows",
            "forced-colors rows must either checkpoint live apply or disclose reload posture",
        ));
    }
    if packet.protected_cue_preservation.len() < 4
        || packet.protected_cue_preservation.iter().any(|row| {
            !row.non_color_cues_required
                || !row.preserved_in_high_contrast
                || !row.preserved_in_forced_colors
        })
    {
        findings.push(finding(
            "appearance_beta.protected_cues.incomplete",
            "protected_cue_preservation",
            "appearance contract must preserve protected cues outside color alone",
        ));
    }
    if !packet.raw_private_material_excluded {
        findings.push(finding(
            "appearance_beta.raw_private_material_included",
            "raw_private_material_excluded",
            "appearance packet must not embed raw token values, screenshots, paths, URLs, or user content",
        ));
    }
    findings
}

fn parse_yaml<T: for<'de> Deserialize<'de>>(
    body: &str,
    artifact_ref: &str,
) -> Result<T, AppearanceSessionContractError> {
    serde_yaml::from_str(body).map_err(|err| AppearanceSessionContractError::ParseFailed {
        artifact_ref: artifact_ref.to_owned(),
        detail: err.to_string(),
    })
}

fn theme_package_summary() -> Result<ThemePackageContractSummary, AppearanceSessionContractError> {
    let manifest = first_party_theme_package_manifest().map_err(|err| {
        AppearanceSessionContractError::InvalidFixture {
            artifact_ref:
                "fixtures/design/theme_package_cases/first_party_default_theme_manifest.yaml"
                    .to_owned(),
            detail: err.to_string(),
        }
    })?;
    let theme_classes = [
        ThemeClass::DarkReference,
        ThemeClass::LightParity,
        ThemeClass::HighContrastDark,
        ThemeClass::HighContrastLight,
    ];
    let density_classes = [
        DensityClass::Compact,
        DensityClass::Standard,
        DensityClass::Comfortable,
    ];
    let motion_postures = [
        AccessibilityPostureClass::MotionStandard,
        AccessibilityPostureClass::MotionReduced,
        AccessibilityPostureClass::MotionLowMotion,
        AccessibilityPostureClass::MotionPowerSaver,
        AccessibilityPostureClass::MotionCriticalHotPath,
    ];
    let supported_theme_classes: Vec<_> = theme_classes
        .iter()
        .copied()
        .filter(|theme| manifest.supports_theme_class(*theme))
        .map(|theme| theme.token().to_owned())
        .collect();
    let supported_density_classes: Vec<_> = density_classes
        .iter()
        .copied()
        .filter(|density| manifest.supports_density_class(*density))
        .map(|density| density.token().to_owned())
        .collect();
    let supported_motion_postures: Vec<_> = motion_postures
        .iter()
        .copied()
        .filter(|posture| manifest.supports_motion_posture(*posture))
        .map(|posture| posture.token().to_owned())
        .collect();
    let minimum_text_contrast_targets = theme_classes
        .iter()
        .copied()
        .filter_map(|theme| {
            manifest
                .minimum_text_contrast_target(theme)
                .map(|target| (theme.token().to_owned(), target))
        })
        .collect();

    Ok(ThemePackageContractSummary {
        fixture_ref: "fixtures/design/theme_package_cases/first_party_default_theme_manifest.yaml"
            .to_owned(),
        schema_ref: "schemas/design/theme_package_manifest.schema.json".to_owned(),
        theme_package_manifest_ref: manifest.theme_package_manifest_ref().to_owned(),
        theme_package_revision_ref: manifest.theme_package_revision_ref().to_owned(),
        package_id: manifest.package_id().to_owned(),
        package_version_label: manifest.package_version_label().to_owned(),
        supported_theme_classes,
        supported_density_classes,
        supported_motion_postures,
        minimum_text_contrast_targets,
        first_party_four_mode_floor_complete: theme_classes
            .iter()
            .all(|theme| manifest.supports_theme_class(*theme)),
    })
}

fn live_follow_policy_summary(policy: &LiveFollowSystemPolicyRecord) -> LiveFollowPolicySummary {
    let mut seen = BTreeSet::new();
    let mut live_no_review_axes = Vec::new();
    let mut live_checkpointed_axes = Vec::new();
    let mut confirm_required_axes = Vec::new();
    let mut policy_blocked_axes = Vec::new();
    let mut checkpoint_and_confirm_rules_valid = true;

    for row in &policy.axes {
        seen.insert(axis_token(row.axis));
        match row.live_update_class {
            LiveUpdateClass::LiveApplyNoReview => {
                live_no_review_axes.push(axis_token(row.axis).to_owned());
                if row.requires_checkpoint || row.requires_user_confirm {
                    checkpoint_and_confirm_rules_valid = false;
                }
            }
            LiveUpdateClass::LiveApplyWithRevertableCheckpoint => {
                live_checkpointed_axes.push(axis_token(row.axis).to_owned());
                if !row.requires_checkpoint || row.requires_user_confirm {
                    checkpoint_and_confirm_rules_valid = false;
                }
            }
            LiveUpdateClass::ConfirmReviewRequired => {
                confirm_required_axes.push(axis_token(row.axis).to_owned());
                if !row.requires_checkpoint || !row.requires_user_confirm {
                    checkpoint_and_confirm_rules_valid = false;
                }
            }
            LiveUpdateClass::PolicyBlocked => {
                policy_blocked_axes.push(axis_token(row.axis).to_owned());
            }
        }
    }

    LiveFollowPolicySummary {
        fixture_ref:
            "fixtures/design/appearance_session_cases/live_follow_system_policy_default_profile.yaml"
                .to_owned(),
        schema_ref: "schemas/design/appearance_session.schema.json".to_owned(),
        live_follow_system_policy_id: policy.live_follow_system_policy_id.clone(),
        appearance_session_ref: policy.appearance_session_ref.clone(),
        live_no_review_axes,
        live_checkpointed_axes,
        confirm_required_axes,
        policy_blocked_axes,
        all_required_axes_declared: required_live_axes()
            .iter()
            .all(|axis| seen.contains(axis)),
        checkpoint_and_confirm_rules_valid,
    }
}

fn import_mapping_summary(
    doc: &ThemeImportMappingReportDoc,
) -> Result<ThemeImportMappingContractSummary, AppearanceSessionContractError> {
    let report = imported_theme_mapping_report_with_warnings().map_err(|err| {
        AppearanceSessionContractError::InvalidFixture {
            artifact_ref:
                "fixtures/design/theme_support_cases/imported_translated_theme_mapping_report_with_warnings.yaml"
                    .to_owned(),
            detail: err.to_string(),
        }
    })?;
    let summary = report.summary();
    let protected_cues_preserved = protected_cues_passed(&doc.protected_cue_honesty_checks);
    Ok(ThemeImportMappingContractSummary {
        fixture_ref:
            "fixtures/design/theme_support_cases/imported_translated_theme_mapping_report_with_warnings.yaml"
                .to_owned(),
        schema_ref: "schemas/design/theme_import_mapping_report.schema.json".to_owned(),
        report_id: doc.report_id.clone(),
        source_ecosystem: doc.source_tool.source_ecosystem.clone(),
        source_tool_version: doc.source_tool.source_tool_version.clone(),
        target_theme_classes: doc
            .target_theme_classes
            .iter()
            .map(|theme| theme.token().to_owned())
            .collect(),
        translated_slot_count: summary.translated_slot_count,
        substituted_with_fallback_count: summary.substituted_with_fallback_count,
        unsupported_slot_count: summary.unsupported_slot_count,
        unresolved_mapping_count: summary.unresolved_mapping_count,
        blocked_honesty_count: summary.blocked_honesty_count,
        deprecated_replacement_count: summary.deprecated_replacement_count,
        syntax_coverage_percent: doc.syntax_token_coverage.coverage_percent,
        syntax_unresolved_scope_count: doc.syntax_token_coverage.unresolved_scope_count,
        appearance_checkpoint_ref: report.appearance_checkpoint_ref().to_owned(),
        rollback_ref: doc.rollback_path.rollback_ref.clone().unwrap_or_default(),
        linked_ux_import_report_ref: doc.linked_ux_import_report_ref.clone(),
        parity_readiness: report.parity_readiness().into(),
        protected_cues_preserved,
    })
}

fn live_os_change_summary(doc: &LiveChangeMatrixDoc) -> LiveOsAppearanceChangeSummary {
    let axes_covered: BTreeSet<_> = doc.rows.iter().map(|row| row.axis.as_str()).collect();
    let claimed_profiles: BTreeSet<_> = doc
        .rows
        .iter()
        .map(|row| row.desktop_profile_id.as_str())
        .collect();

    let shell_rows: Vec<_> = doc
        .rows
        .iter()
        .filter(|row| row.surface_family_class == "shell_and_first_party_dialogs")
        .collect();
    let shell_live_axes = unique_axes_for_apply_path(&shell_rows, "apply_live");
    let shell_checkpointed_axes =
        unique_axes_for_apply_path(&shell_rows, "apply_live_checkpointed");
    let shell_confirm_required_axes =
        unique_axes_for_apply_path(&shell_rows, "hold_pending_confirm");
    let embedded_reload_required_rows = doc
        .rows
        .iter()
        .filter(|row| {
            row.surface_family_class == "embedded_or_extension_surface"
                && row.apply_path_class == "surface_reload_required"
        })
        .count();
    let full_restart_required_rows = doc
        .rows
        .iter()
        .filter(|row| row.apply_path_class == "full_restart_required")
        .count();
    let all_reload_or_restart_rows_disclosed = doc.rows.iter().all(|row| {
        !matches!(
            row.apply_path_class.as_str(),
            "surface_reload_required" | "full_restart_required"
        ) || row.disclosure_required
    });
    let all_claimed_profiles_cover_shell_axes = claimed_profiles.iter().all(|profile| {
        required_os_axes().iter().all(|axis| {
            shell_rows
                .iter()
                .any(|row| row.desktop_profile_id == **profile && row.axis == **axis)
        })
    });
    let forced_colors_rows_explicit = doc
        .rows
        .iter()
        .filter(|row| row.os_signal_class == "os_forced_colors_signal")
        .all(|row| {
            row.apply_path_class == "apply_live"
                || row.apply_path_class == "apply_live_checkpointed"
                || (row.apply_path_class == "surface_reload_required" && row.disclosure_required)
        });

    LiveOsAppearanceChangeSummary {
        matrix_ref: "artifacts/design/appearance_live_change_matrix.yaml".to_owned(),
        matrix_schema_version: doc.appearance_live_change_matrix_schema_version,
        claimed_profile_count: claimed_profiles.len(),
        row_count: doc.rows.len(),
        axes_covered: axes_covered.into_iter().map(str::to_owned).collect(),
        shell_live_axes,
        shell_checkpointed_axes,
        shell_confirm_required_axes,
        embedded_reload_required_rows,
        full_restart_required_rows,
        all_reload_or_restart_rows_disclosed,
        all_claimed_profiles_cover_shell_axes,
        forced_colors_rows_explicit,
    }
}

fn unique_axes_for_apply_path(rows: &[&LiveChangeMatrixRowDoc], apply_path: &str) -> Vec<String> {
    rows.iter()
        .filter(|row| row.apply_path_class == apply_path)
        .map(|row| row.axis.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn protected_cue_rows() -> Vec<ProtectedCueContractRow> {
    ["trust", "policy_lock", "severity", "source_integrity"]
        .iter()
        .map(|cue| ProtectedCueContractRow {
            protected_cue_class: (*cue).to_owned(),
            non_color_cues_required: true,
            preserved_in_high_contrast: true,
            preserved_in_forced_colors: true,
        })
        .collect()
}

fn protected_cues_passed(rows: &[ProtectedCueHonestyCheckDoc]) -> bool {
    let required = ["trust", "policy_lock", "severity", "source_integrity"];
    required.iter().all(|cue| {
        rows.iter().any(|row| {
            row.protected_cue_class == *cue
                && row.color_alone_prohibited
                && row.preserved_in_high_contrast
                && row.preserved_in_forced_colors
                && row.result_state == "passed"
                && !row.non_color_cues.is_empty()
        })
    })
}

fn required_live_axes() -> &'static [&'static str] {
    &[
        "mode_theme_class",
        "contrast_mode",
        "accent_source",
        "density_class",
        "text_scale",
        "reduced_motion_posture",
        "follow_system_posture",
    ]
}

fn required_os_axes() -> &'static [&'static str] {
    &[
        "mode_theme_class",
        "contrast_mode",
        "accent_source",
        "text_scale",
        "reduced_motion_posture",
    ]
}

fn axis_token(axis: AppearanceAxis) -> &'static str {
    match axis {
        AppearanceAxis::ModeThemeClass => "mode_theme_class",
        AppearanceAxis::ContrastMode => "contrast_mode",
        AppearanceAxis::AccentSource => "accent_source",
        AppearanceAxis::DensityClass => "density_class",
        AppearanceAxis::TextScale => "text_scale",
        AppearanceAxis::ReducedMotionPosture => "reduced_motion_posture",
        AppearanceAxis::FollowSystemPosture => "follow_system_posture",
    }
}

fn finding(
    check_id: &str,
    field: impl Into<String>,
    note: impl Into<String>,
) -> DesignSystemFinding {
    let field = field.into();
    DesignSystemFinding {
        record_kind: DESIGN_SYSTEM_FINDING_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        finding_id: format!(
            "design-system:finding:{check_id}:{}",
            crate::stable_field(&field)
        ),
        severity: crate::FindingSeverity::Error,
        check_id: check_id.to_owned(),
        surface_class: None,
        field,
        note: note.into(),
    }
}

#[derive(Debug, Clone, Deserialize)]
struct TokenOverlayDoc {
    token_overlay_id: String,
    appearance_session_ref: String,
    overlay_scope: String,
    validation_state: String,
    entries: Vec<TokenOverlayEntryDoc>,
    summary_counts: TokenOverlaySummaryCountsDoc,
    fallback_chain: Vec<TokenOverlayFallbackStepDoc>,
}

impl TokenOverlayDoc {
    fn summary(&self) -> TokenOverlayContractSummary {
        TokenOverlayContractSummary {
            fixture_ref:
                "fixtures/design/appearance_session_cases/workspace_overlay_overridden_deprecated_unmapped.yaml"
                    .to_owned(),
            schema_ref: "schemas/design/token_overlay.schema.json".to_owned(),
            token_overlay_id: self.token_overlay_id.clone(),
            appearance_session_ref: self.appearance_session_ref.clone(),
            overlay_scope: self.overlay_scope.clone(),
            validation_state: self.validation_state.clone(),
            inherited_count: self.summary_counts.inherited_count,
            overridden_count: self.summary_counts.overridden_count,
            deprecated_count: self.summary_counts.deprecated_count,
            unmapped_count: self.summary_counts.unmapped_count,
            summary_counts_match_entries: self.summary_counts_match_entries(),
            deprecated_replacements_visible: self.deprecated_replacements_visible(),
            unmapped_entries_preserved_inert: self.unmapped_entries_preserved_inert(),
            fallback_chain_visible: !self.fallback_chain.is_empty()
                && self.entries.iter().all(|entry| {
                    entry
                        .fallback_chain_ref
                        .as_deref()
                        .is_some_and(|step_ref| {
                            self.fallback_chain
                                .iter()
                                .any(|step| step.step_id == step_ref)
                        })
                }),
        }
    }

    fn summary_counts_match_entries(&self) -> bool {
        count_entries(&self.entries, "inherited") == self.summary_counts.inherited_count
            && count_entries(&self.entries, "overridden") == self.summary_counts.overridden_count
            && count_entries(&self.entries, "deprecated") == self.summary_counts.deprecated_count
            && count_entries(&self.entries, "unmapped") == self.summary_counts.unmapped_count
    }

    fn deprecated_replacements_visible(&self) -> bool {
        self.entries
            .iter()
            .filter(|entry| entry.value_state_class == "deprecated")
            .all(|entry| entry.deprecated_replacement_ref.is_some())
    }

    fn unmapped_entries_preserved_inert(&self) -> bool {
        self.entries
            .iter()
            .filter(|entry| entry.value_state_class == "unmapped")
            .all(|entry| {
                entry.unmapped_source_slot_ref.is_some()
                    && entry
                        .fallback_chain_ref
                        .as_deref()
                        .and_then(|step_ref| {
                            self.fallback_chain
                                .iter()
                                .find(|step| step.step_id == step_ref)
                        })
                        .is_some_and(|step| {
                            step.step_kind == "inert_placeholder"
                                && step.target_token_ref.is_none()
                                && !step.applied
                        })
            })
    }
}

fn count_entries(entries: &[TokenOverlayEntryDoc], value_state: &str) -> u32 {
    entries
        .iter()
        .filter(|entry| entry.value_state_class == value_state)
        .count() as u32
}

#[derive(Debug, Clone, Deserialize)]
struct TokenOverlayEntryDoc {
    value_state_class: String,
    fallback_chain_ref: Option<String>,
    deprecated_replacement_ref: Option<String>,
    unmapped_source_slot_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenOverlaySummaryCountsDoc {
    inherited_count: u32,
    overridden_count: u32,
    deprecated_count: u32,
    unmapped_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenOverlayFallbackStepDoc {
    step_id: String,
    step_kind: String,
    target_token_ref: Option<String>,
    applied: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ThemeImportMappingReportDoc {
    report_id: String,
    source_tool: SourceToolDoc,
    target_theme_classes: Vec<ThemeClass>,
    syntax_token_coverage: SyntaxTokenCoverageDoc,
    protected_cue_honesty_checks: Vec<ProtectedCueHonestyCheckDoc>,
    rollback_path: RollbackPathDoc,
    linked_ux_import_report_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SourceToolDoc {
    source_ecosystem: String,
    source_tool_version: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SyntaxTokenCoverageDoc {
    unresolved_scope_count: u32,
    coverage_percent: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct ProtectedCueHonestyCheckDoc {
    protected_cue_class: String,
    non_color_cues: Vec<String>,
    color_alone_prohibited: bool,
    preserved_in_high_contrast: bool,
    preserved_in_forced_colors: bool,
    result_state: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RollbackPathDoc {
    rollback_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct LiveChangeMatrixDoc {
    appearance_live_change_matrix_schema_version: u32,
    rows: Vec<LiveChangeMatrixRowDoc>,
}

#[derive(Debug, Clone, Deserialize)]
struct LiveChangeMatrixRowDoc {
    desktop_profile_id: String,
    surface_family_class: String,
    axis: String,
    os_signal_class: String,
    apply_path_class: String,
    disclosure_required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_appearance_session_packet_validates_with_disclosed_import_gap() {
        let packet = seeded_appearance_session_beta_contract();
        validate_appearance_session_beta_contract(&packet).expect("appearance packet validates");
        assert_eq!(packet.gate_state, GateStateClass::PassWithDisclosedGap);
        assert_eq!(
            packet.import_mapping_report.parity_readiness,
            ImportedThemeParityClass::PartialWithVisibleGaps
        );
        assert!(packet.token_overlay.unmapped_entries_preserved_inert);
        assert!(
            packet
                .live_os_change_matrix
                .all_reload_or_restart_rows_disclosed
        );
    }

    #[test]
    fn audit_flags_missing_reload_disclosure() {
        let mut packet = seeded_appearance_session_beta_contract();
        packet
            .live_os_change_matrix
            .all_reload_or_restart_rows_disclosed = false;
        let findings = audit_appearance_session_beta_contract(&packet);
        assert!(
            findings
                .iter()
                .any(|finding| finding.check_id
                    == "appearance_beta.live_os.reload_disclosure_missing")
        );
    }

    #[test]
    fn audit_flags_dropped_unmapped_overlay_entry() {
        let mut packet = seeded_appearance_session_beta_contract();
        packet.token_overlay.unmapped_entries_preserved_inert = false;
        let findings = audit_appearance_session_beta_contract(&packet);
        assert!(findings
            .iter()
            .any(|finding| finding.check_id == "appearance_beta.token_overlay.unmapped_not_inert"));
    }
}
