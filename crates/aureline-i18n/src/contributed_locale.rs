//! Extension-contributed and companion locale-pack support and host-stable
//! label protection.
//!
//! The first-party delivery lane in [`crate::locale_pack_delivery`] governs how
//! the *product's own* locale packs ship and degrade. This module governs the
//! packs Aureline does **not** author: locale packs contributed by extensions
//! and overlays contributed by companion surfaces. It answers three questions
//! that the first-party lane cannot:
//!
//! - **How a contributed pack declares itself.** Each
//!   [`ContributedLocaleManifest`] names its owner, whether it is an extension
//!   pack or a companion overlay, the locales it covers, the compatibility build
//!   range it targets, its fallback behavior, and the surface families and
//!   namespace it owns translations for.
//! - **What a contributed pack may never touch.** Trust, policy, capability, and
//!   lifecycle vocabulary is host-owned. Each [`HostStableLabelGuard`] reserves a
//!   label class and a namespace prefix so a contributed pack can *render* those
//!   labels but never *replace* them. A manifest that asks to override them, or
//!   that claims a reserved namespace, fails validation.
//! - **How a contributed surface degrades truthfully.** A
//!   [`ContributedLocaleSupportRow`] resolves each manifest against the active
//!   build into an explicit apply-or-degrade decision with a
//!   [`ContributedDegradeReason`], discloses missing support on a claimed
//!   localized profile, and attributes the row to its
//!   [`LocalizationIssueSourceClass`] so support can tell a first-party problem
//!   from an extension or companion one.
//!
//! The central invariant matches the delivery lane: a contributed pack that is
//! unsigned, skewed, or simply absent does not leave a half-localized surface
//! with mixed-language trust vocabulary. It degrades fully to host
//! source-language behavior with a recorded reason, while host-stable labels
//! stay canonical regardless of what the surrounding extension strings do.
//!
//! Raw translated bodies, signing keys, and credentials never cross this
//! boundary: manifests carry refs and digests, not payloads.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    CompatibilityBuildRange, DegradedLocalizationState, ExtensionLocaleSupportMode,
    LocalePackSignatureState, LocalePackSourceClass, LocalePackValidationFinding,
    MessageSurfaceFamily, PackApplicationDecision, VersionMatchState, GENERATED_AT,
    LOCALE_PACK_BETA_CONTRACT_ID, SOURCE_LANGUAGE_LOCALE, TARGET_BUILD,
};

/// Schema version for the contributed-locale support records.
pub const CONTRIBUTED_LOCALE_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`ContributedLocaleSupportReport`].
pub const CONTRIBUTED_LOCALE_SUPPORT_REPORT_RECORD_KIND: &str = "contributed_locale_support_report";

/// Record kind for [`ContributedLocaleManifest`].
pub const CONTRIBUTED_LOCALE_MANIFEST_RECORD_KIND: &str = "contributed_locale_manifest_record";

/// Stable id for the seeded contributed-locale support report.
pub const CONTRIBUTED_LOCALE_SUPPORT_REPORT_ID: &str =
    "i18n:m5-extension-companion-locale-support:v1";

/// Fixture path for the seeded contributed-locale support report.
pub const CONTRIBUTED_LOCALE_SUPPORT_REPORT_FIXTURE_REF: &str =
    "fixtures/i18n/extension-companion-pack-compat/support_report.json";

/// Stable id of the first-party compatibility report this packet joins against.
///
/// Support tooling reads both packets: contributed rows here, first-party rows
/// in the delivery lane, so a localization issue can be attributed across all
/// three source classes.
pub const FIRST_PARTY_COMPATIBILITY_REPORT_REF: &str = "i18n:m5-locale-pack-compatibility:v1";

/// Same-surface route that always reaches host source-language truth.
const SOURCE_LANGUAGE_ROUTE: &str = "route:i18n:source-language:open";

/// Lowest build identity the seeded contributed packs target.
const CONTRIBUTED_MIN_BUILD: &str = "build:aureline:0.0.0-beta.2026.05.01";

/// Highest build identity the seeded contributed packs target.
const CONTRIBUTED_MAX_BUILD: &str = "build:aureline:0.0.0-beta.2026.06.30";

/// Whether a contributed pack is owned by an extension or a companion surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributedPackOwnerClass {
    /// Extension-owned locale pack contributing strings inside an extension namespace.
    ExtensionOwnedPack,
    /// Companion-surface overlay pack, intentionally narrower than desktop scope.
    CompanionOverlayPack,
}

impl ContributedPackOwnerClass {
    /// Returns the source class support uses to route an issue on this row.
    pub const fn issue_source(self) -> LocalizationIssueSourceClass {
        match self {
            Self::ExtensionOwnedPack => LocalizationIssueSourceClass::ExtensionPack,
            Self::CompanionOverlayPack => LocalizationIssueSourceClass::CompanionOverlay,
        }
    }
}

/// Source attribution for a localization issue, so support can route it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalizationIssueSourceClass {
    /// Issue originates in a first-party host pack (tracked in the delivery lane).
    FirstPartyPack,
    /// Issue originates in an extension-contributed pack.
    ExtensionPack,
    /// Issue originates in a companion overlay.
    CompanionOverlay,
}

/// Host-owned label category that a contributed pack may render but never replace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostStableLabelClass {
    /// Trust-state and trust-boundary vocabulary.
    TrustLabel,
    /// Policy and governance vocabulary.
    PolicyLabel,
    /// Capability and permission vocabulary.
    CapabilityLabel,
    /// Lifecycle-state vocabulary (enabled, disabled, quarantined, revoked).
    LifecycleLabel,
}

/// Every host-stable label class, in stable order.
pub const ALL_HOST_STABLE_LABEL_CLASSES: [HostStableLabelClass; 4] = [
    HostStableLabelClass::TrustLabel,
    HostStableLabelClass::PolicyLabel,
    HostStableLabelClass::CapabilityLabel,
    HostStableLabelClass::LifecycleLabel,
];

/// Why a contributed surface degrades rather than rendering localized text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributedDegradeReason {
    /// Surface is localized; no degradation occurred.
    NotDegraded,
    /// No contributed pack ships the requested locale.
    NoContributedPackForLocale,
    /// Contributed pack is blocked by signature failure or unaccepted unsigned state.
    PackBlockedSignatureFailure,
    /// Active build falls outside the pack's declared compatibility build range.
    PackBuildOutsideCompatibilityRange,
    /// Companion scope is intentionally narrower than desktop scope.
    CompanionScopeNarrowerThanDesktop,
    /// Policy disabled the locale for this surface.
    PolicyDisabledLocale,
}

impl ContributedDegradeReason {
    /// Returns true when this reason represents an actual degradation.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::NotDegraded)
    }

    /// Returns true when the degradation reflects a defect rather than a
    /// deliberately narrower companion scope.
    ///
    /// A narrower companion is the documented design, not a missing-support
    /// defect, so it never counts against a claimed localized profile.
    pub const fn is_support_defect(self) -> bool {
        matches!(
            self,
            Self::NoContributedPackForLocale
                | Self::PackBlockedSignatureFailure
                | Self::PackBuildOutsideCompatibilityRange
                | Self::PolicyDisabledLocale
        )
    }
}

/// Host-stable label guard: a label class held canonical and read-only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostStableLabelGuard {
    /// Protected label class.
    pub label_class: HostStableLabelClass,
    /// Host catalog ref that owns the canonical vocabulary for this class.
    pub host_catalog_ref: String,
    /// Stable message-id namespace prefix reserved for the host.
    pub reserved_namespace_prefix: String,
    /// Always true: contributed packs may reference but never override this class.
    pub contributed_override_forbidden: bool,
    /// Export-safe label.
    pub presentation_label: String,
}

/// One extension- or companion-owned contributed locale manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributedLocaleManifest {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Owning extension or companion-surface id.
    pub owner_id: String,
    /// Owner namespace ref.
    pub owner_namespace_ref: String,
    /// Whether this is an extension pack or a companion overlay.
    pub owner_class: ContributedPackOwnerClass,
    /// Declared locale-support mode.
    pub support_mode: ExtensionLocaleSupportMode,
    /// Locale-pack ref when the manifest ships one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale_pack_ref: Option<String>,
    /// Source-language locale for owner strings.
    pub source_language_locale: String,
    /// Fallback locale when the overlay is missing or blocked.
    pub fallback_locale: String,
    /// Locales the manifest claims to cover.
    pub coverage_locales: Vec<String>,
    /// Compatibility build range the pack declares.
    pub compatibility_build_range: CompatibilityBuildRange,
    /// Surface families the manifest owns translations for.
    pub owned_surface_families: Vec<MessageSurfaceFamily>,
    /// Reserved namespace prefix the manifest may write into.
    pub owned_namespace_prefix: String,
    /// Host-stable label classes the manifest renders read-only from the host.
    pub host_stable_labels_referenced: Vec<HostStableLabelClass>,
    /// Must remain false: contributed packs cannot override host-stable labels.
    pub may_override_host_stable_labels: bool,
    /// Pack source class.
    pub source_class: LocalePackSourceClass,
    /// Pack signature state.
    pub signature_state: LocalePackSignatureState,
    /// Whether the surface must disclose its localization posture.
    pub visible_disclosure_required: bool,
    /// For companion overlays: whether scope is intentionally narrower than desktop.
    pub companion_scope_narrower_than_desktop: bool,
}

impl ContributedLocaleManifest {
    /// Returns true when the manifest ships its own locale pack.
    pub fn ships_pack(&self) -> bool {
        matches!(
            self.support_mode,
            ExtensionLocaleSupportMode::ShipsOwnLocalePack
                | ExtensionLocaleSupportMode::ShipsCompanionLocalePack
        )
    }
}

/// Resolved per-surface support and degradation row for one contributed manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributedLocaleSupportRow {
    /// Stable row id.
    pub row_id: String,
    /// Manifest this row resolves.
    pub manifest_id: String,
    /// Owning extension or companion-surface id.
    pub owner_id: String,
    /// Whether this is an extension pack or a companion overlay.
    pub owner_class: ContributedPackOwnerClass,
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produces rendered text after evaluation.
    pub effective_locale: String,
    /// Whether the active build is inside the pack's compatibility range.
    pub target_build_in_compatibility_range: bool,
    /// Observed signature state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state.
    pub version_match_state: VersionMatchState,
    /// Apply-or-degrade decision.
    pub application_decision: PackApplicationDecision,
    /// Reason a degrade occurred, when applicable.
    pub degrade_reason: ContributedDegradeReason,
    /// Localized rendering state after fallback resolution.
    pub degraded_localization_state: DegradedLocalizationState,
    /// True when the surface lacks localized support on a claimed localized profile.
    pub missing_support_on_claimed_profile: bool,
    /// Source attribution for any localization issue on this row.
    pub issue_source_class: LocalizationIssueSourceClass,
    /// Host-stable label classes that stayed canonical for this row.
    pub host_stable_labels_preserved: Vec<HostStableLabelClass>,
    /// Whether this row backs a claimed localized profile.
    pub claimed_localized_profile: bool,
    /// Same-surface host source-language route.
    pub open_in_source_language_route_ref: String,
    /// Export-safe label.
    pub presentation_label: String,
}

impl ContributedLocaleSupportRow {
    /// Returns true when the row degraded fully to host source language.
    pub fn degraded_to_source_language(&self) -> bool {
        self.application_decision == PackApplicationDecision::DegradeToSourceLanguageOnly
    }

    /// Returns true when the row sits in a defined, non-ambiguous state.
    ///
    /// A degraded row carries a non-`NotDegraded` reason and never claims a
    /// localized profile; an applied row carries `NotDegraded`. There is no
    /// undefined half-localized middle, and host-stable labels are preserved
    /// either way.
    pub fn is_resolved(&self) -> bool {
        if self.host_stable_labels_preserved.is_empty() {
            return false;
        }
        if self.degraded_to_source_language() {
            self.degrade_reason.is_degraded() && !self.claimed_localized_profile
        } else {
            self.degrade_reason == ContributedDegradeReason::NotDegraded
        }
    }
}

/// Summary for the contributed-locale support report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributedLocaleSupportSummary {
    /// Total contributed manifests in the report.
    pub total_manifests: usize,
    /// Extension-owned manifests.
    pub extension_manifests: usize,
    /// Companion-overlay manifests.
    pub companion_manifests: usize,
    /// Rows that applied their translations.
    pub applied_rows: usize,
    /// Rows that degraded to host source language.
    pub degraded_rows: usize,
    /// Rows missing localized support on a claimed localized profile.
    pub missing_support_rows: usize,
    /// Host-stable label classes held canonical.
    pub host_stable_label_classes_protected: usize,
    /// True when no manifest overrides host-stable labels and every degrade is
    /// disclosed with a non-`NotDegraded` reason and never claims a profile.
    pub guardrail_clean: bool,
}

/// Top-level inspectable contributed-locale support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributedLocaleSupportReport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contract id this packet derives from.
    pub source_contract_id: String,
    /// First-party compatibility report this packet joins against.
    pub first_party_compatibility_report_ref: String,
    /// Active build identity the report was evaluated against.
    pub target_build_identity_ref: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Host-stable label guards held canonical for all contributed surfaces.
    pub host_stable_label_guards: Vec<HostStableLabelGuard>,
    /// Contributed locale manifests.
    pub manifests: Vec<ContributedLocaleManifest>,
    /// Resolved per-surface support rows.
    pub support_rows: Vec<ContributedLocaleSupportRow>,
    /// Report summary.
    pub summary: ContributedLocaleSupportSummary,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

impl ContributedLocaleSupportReport {
    /// Returns the manifest with the given id, when present.
    pub fn manifest(&self, manifest_id: &str) -> Option<&ContributedLocaleManifest> {
        self.manifests
            .iter()
            .find(|manifest| manifest.manifest_id == manifest_id)
    }

    /// Returns the support row with the given id, when present.
    pub fn row(&self, row_id: &str) -> Option<&ContributedLocaleSupportRow> {
        self.support_rows.iter().find(|row| row.row_id == row_id)
    }

    /// Returns the host-stable label guard for a label class, when present.
    pub fn guard(&self, label_class: HostStableLabelClass) -> Option<&HostStableLabelGuard> {
        self.host_stable_label_guards
            .iter()
            .find(|guard| guard.label_class == label_class)
    }

    /// Returns true when every support row resolves to a defined state.
    pub fn all_states_resolved(&self) -> bool {
        self.support_rows
            .iter()
            .all(ContributedLocaleSupportRow::is_resolved)
    }

    /// Validates the report's structure and host-protection invariants.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != CONTRIBUTED_LOCALE_SUPPORT_REPORT_RECORD_KIND {
            findings.push(finding(
                &self.report_id,
                "report record_kind is not canonical",
            ));
        }
        if self.schema_version != CONTRIBUTED_LOCALE_SCHEMA_VERSION {
            findings.push(finding(
                &self.report_id,
                "report schema_version is not canonical",
            ));
        }
        if !self.raw_translated_body_omitted {
            findings.push(finding(
                &self.report_id,
                "report must omit raw translated body text",
            ));
        }

        validate_host_guards(&self.host_stable_label_guards, &mut findings);
        let reserved_prefixes: BTreeSet<&str> = self
            .host_stable_label_guards
            .iter()
            .map(|guard| guard.reserved_namespace_prefix.as_str())
            .collect();

        let manifest_ids: BTreeSet<&str> = self
            .manifests
            .iter()
            .map(|manifest| manifest.manifest_id.as_str())
            .collect();
        for manifest in &self.manifests {
            validate_manifest(manifest, &reserved_prefixes, &mut findings);
        }

        for row in &self.support_rows {
            validate_row(row, &manifest_ids, &mut findings);
        }

        validate_summary(self, &mut findings);

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Inputs that decide whether a contributed surface applies or degrades.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContributedEvaluationInput {
    /// Whether the surface is an extension pack or a companion overlay.
    pub owner_class: ContributedPackOwnerClass,
    /// Whether a pack ships the requested locale.
    pub pack_present_for_locale: bool,
    /// Whether the active build is inside the pack's compatibility range.
    pub target_build_in_compatibility_range: bool,
    /// Observed signature state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state.
    pub version_match_state: VersionMatchState,
    /// Whether policy permits the locale on this surface.
    pub policy_locale_enabled: bool,
    /// For companion overlays: whether scope is intentionally narrower than desktop.
    pub companion_scope_narrower_than_desktop: bool,
}

/// Decides whether a contributed surface applies translations or degrades.
///
/// The decision is conservative in exactly the same spirit as
/// [`crate::decide_application`]: anything unsigned, skewed, or absent degrades
/// fully to host source language with a recorded reason. A deliberately narrower
/// companion overlay is reported as such rather than as a pack defect.
pub fn decide_contributed_support(
    input: &ContributedEvaluationInput,
) -> (PackApplicationDecision, ContributedDegradeReason) {
    use ContributedDegradeReason as Reason;
    use PackApplicationDecision::{
        ApplyLocalizedWithDisclosedMissingKeys, DegradeToSourceLanguageOnly,
    };

    if !input.policy_locale_enabled {
        return (DegradeToSourceLanguageOnly, Reason::PolicyDisabledLocale);
    }
    if input.owner_class == ContributedPackOwnerClass::CompanionOverlayPack
        && input.companion_scope_narrower_than_desktop
    {
        return (
            DegradeToSourceLanguageOnly,
            Reason::CompanionScopeNarrowerThanDesktop,
        );
    }
    if !input.pack_present_for_locale {
        return (
            DegradeToSourceLanguageOnly,
            Reason::NoContributedPackForLocale,
        );
    }
    if !input.signature_state.may_render() {
        return (
            DegradeToSourceLanguageOnly,
            Reason::PackBlockedSignatureFailure,
        );
    }
    if !input.target_build_in_compatibility_range || !input.version_match_state.may_render() {
        return (
            DegradeToSourceLanguageOnly,
            Reason::PackBuildOutsideCompatibilityRange,
        );
    }
    (ApplyLocalizedWithDisclosedMissingKeys, Reason::NotDegraded)
}

fn finding(row_ref: &str, message: &str) -> LocalePackValidationFinding {
    LocalePackValidationFinding {
        row_ref: row_ref.to_owned(),
        message: message.to_owned(),
    }
}

fn validate_host_guards(
    guards: &[HostStableLabelGuard],
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut seen = BTreeSet::new();
    for guard in guards {
        if !guard.contributed_override_forbidden {
            findings.push(finding(
                &guard.host_catalog_ref,
                "host-stable label guard must forbid contributed override",
            ));
        }
        if guard.reserved_namespace_prefix.trim().is_empty() {
            findings.push(finding(
                &guard.host_catalog_ref,
                "host-stable label guard must reserve a namespace prefix",
            ));
        }
        seen.insert(guard.label_class);
    }
    for class in ALL_HOST_STABLE_LABEL_CLASSES {
        if !seen.contains(&class) {
            findings.push(finding(
                CONTRIBUTED_LOCALE_SUPPORT_REPORT_ID,
                "report must guard every host-stable label class",
            ));
        }
    }
}

fn validate_manifest(
    manifest: &ContributedLocaleManifest,
    reserved_prefixes: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if manifest.record_kind != CONTRIBUTED_LOCALE_MANIFEST_RECORD_KIND {
        findings.push(finding(
            &manifest.manifest_id,
            "manifest record_kind is not canonical",
        ));
    }
    if manifest.schema_version != CONTRIBUTED_LOCALE_SCHEMA_VERSION {
        findings.push(finding(
            &manifest.manifest_id,
            "manifest schema_version is not canonical",
        ));
    }
    if manifest.may_override_host_stable_labels {
        findings.push(finding(
            &manifest.manifest_id,
            "contributed manifest must not override host-stable labels",
        ));
    }
    if manifest
        .owned_surface_families
        .contains(&MessageSurfaceFamily::PolicyLegalOrRecoveryText)
    {
        findings.push(finding(
            &manifest.manifest_id,
            "contributed manifest must not own policy, legal, or recovery text",
        ));
    }
    for prefix in reserved_prefixes {
        if manifest.owned_namespace_prefix.starts_with(prefix)
            || prefix.starts_with(manifest.owned_namespace_prefix.as_str())
        {
            findings.push(finding(
                &manifest.manifest_id,
                "contributed manifest namespace collides with a reserved host prefix",
            ));
        }
    }
    if manifest.ships_pack() && manifest.locale_pack_ref.is_none() {
        findings.push(finding(
            &manifest.manifest_id,
            "manifest that ships a pack must name its locale pack",
        ));
    }
    if manifest
        .compatibility_build_range
        .min_build_identity_ref
        .trim()
        .is_empty()
        || manifest
            .compatibility_build_range
            .max_build_identity_ref
            .trim()
            .is_empty()
    {
        findings.push(finding(
            &manifest.manifest_id,
            "manifest must declare a compatibility build range",
        ));
    }
    let is_companion = manifest.owner_class == ContributedPackOwnerClass::CompanionOverlayPack;
    let declares_companion =
        manifest.support_mode == ExtensionLocaleSupportMode::ShipsCompanionLocalePack;
    if is_companion != declares_companion
        && manifest.support_mode != ExtensionLocaleSupportMode::SourceLanguageOnly
        && manifest.support_mode != ExtensionLocaleSupportMode::InheritsHostLocale
    {
        findings.push(finding(
            &manifest.manifest_id,
            "companion owner class and companion support mode must agree",
        ));
    }
    if !manifest.visible_disclosure_required {
        findings.push(finding(
            &manifest.manifest_id,
            "contributed manifest must disclose its localization posture",
        ));
    }
}

fn validate_row(
    row: &ContributedLocaleSupportRow,
    manifest_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if !manifest_ids.contains(row.manifest_id.as_str()) {
        findings.push(finding(
            &row.row_id,
            "support row references an unknown manifest",
        ));
    }
    if row.host_stable_labels_preserved.is_empty() {
        findings.push(finding(
            &row.row_id,
            "support row must keep host-stable labels canonical",
        ));
    }
    if row.issue_source_class != row.owner_class.issue_source() {
        findings.push(finding(
            &row.row_id,
            "issue source class must match the row owner class",
        ));
    }
    let degraded = row.degraded_to_source_language();
    if degraded && !row.degrade_reason.is_degraded() {
        findings.push(finding(
            &row.row_id,
            "degraded row must carry a non-NotDegraded reason",
        ));
    }
    if !degraded && row.degrade_reason != ContributedDegradeReason::NotDegraded {
        findings.push(finding(
            &row.row_id,
            "applied row must carry the NotDegraded reason",
        ));
    }
    if degraded && row.degraded_localization_state == DegradedLocalizationState::FullyLocalized {
        findings.push(finding(
            &row.row_id,
            "degraded row must not report a fully localized state",
        ));
    }
    if row.claimed_localized_profile && degraded {
        findings.push(finding(
            &row.row_id,
            "a degraded row must never claim a localized profile",
        ));
    }
    // A real support defect on a claimed profile must be disclosed; a narrower
    // companion is the documented design and never counts as missing support.
    if row.missing_support_on_claimed_profile && !row.degrade_reason.is_support_defect() {
        findings.push(finding(
            &row.row_id,
            "missing-support disclosure requires a support-defect degrade reason",
        ));
    }
    if row.owner_class == ContributedPackOwnerClass::CompanionOverlayPack
        && row.missing_support_on_claimed_profile
        && row.degrade_reason == ContributedDegradeReason::CompanionScopeNarrowerThanDesktop
    {
        findings.push(finding(
            &row.row_id,
            "narrower companion scope must not be reported as missing support",
        ));
    }
    if row.open_in_source_language_route_ref.trim().is_empty() {
        findings.push(finding(
            &row.row_id,
            "support row must expose a source-language escape route",
        ));
    }
}

fn validate_summary(
    report: &ContributedLocaleSupportReport,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let expected = derive_summary(report);
    if report.summary != expected {
        findings.push(finding(
            &report.report_id,
            "summary does not match derived counts",
        ));
    }
}

fn derive_summary(report: &ContributedLocaleSupportReport) -> ContributedLocaleSupportSummary {
    let extension_manifests = report
        .manifests
        .iter()
        .filter(|m| m.owner_class == ContributedPackOwnerClass::ExtensionOwnedPack)
        .count();
    let companion_manifests = report
        .manifests
        .iter()
        .filter(|m| m.owner_class == ContributedPackOwnerClass::CompanionOverlayPack)
        .count();
    let degraded_rows = report
        .support_rows
        .iter()
        .filter(|r| r.degraded_to_source_language())
        .count();
    let applied_rows = report.support_rows.len() - degraded_rows;
    let missing_support_rows = report
        .support_rows
        .iter()
        .filter(|r| r.missing_support_on_claimed_profile)
        .count();
    let no_override = report
        .manifests
        .iter()
        .all(|m| !m.may_override_host_stable_labels);
    let degrades_disclosed = report.support_rows.iter().all(|r| {
        !r.degraded_to_source_language()
            || (r.degrade_reason.is_degraded() && !r.claimed_localized_profile)
    });
    ContributedLocaleSupportSummary {
        total_manifests: report.manifests.len(),
        extension_manifests,
        companion_manifests,
        applied_rows,
        degraded_rows,
        missing_support_rows,
        host_stable_label_classes_protected: report.host_stable_label_guards.len(),
        guardrail_clean: no_override && degrades_disclosed,
    }
}

fn build_range() -> CompatibilityBuildRange {
    CompatibilityBuildRange {
        min_build_identity_ref: CONTRIBUTED_MIN_BUILD.to_owned(),
        max_build_identity_ref: CONTRIBUTED_MAX_BUILD.to_owned(),
    }
}

fn out_of_range() -> CompatibilityBuildRange {
    CompatibilityBuildRange {
        min_build_identity_ref: "build:aureline:0.0.0-beta.2026.07.01".to_owned(),
        max_build_identity_ref: "build:aureline:0.0.0-beta.2026.08.01".to_owned(),
    }
}

fn host_stable_label_guards() -> Vec<HostStableLabelGuard> {
    vec![
        HostStableLabelGuard {
            label_class: HostStableLabelClass::TrustLabel,
            host_catalog_ref: "i18n:host:trust-vocabulary:v1".to_owned(),
            reserved_namespace_prefix: "host.trust.".to_owned(),
            contributed_override_forbidden: true,
            presentation_label: "Trust labels are host-controlled".to_owned(),
        },
        HostStableLabelGuard {
            label_class: HostStableLabelClass::PolicyLabel,
            host_catalog_ref: "i18n:host:policy-vocabulary:v1".to_owned(),
            reserved_namespace_prefix: "host.policy.".to_owned(),
            contributed_override_forbidden: true,
            presentation_label: "Policy labels are host-controlled".to_owned(),
        },
        HostStableLabelGuard {
            label_class: HostStableLabelClass::CapabilityLabel,
            host_catalog_ref: "i18n:host:capability-vocabulary:v1".to_owned(),
            reserved_namespace_prefix: "host.capability.".to_owned(),
            contributed_override_forbidden: true,
            presentation_label: "Capability labels are host-controlled".to_owned(),
        },
        HostStableLabelGuard {
            label_class: HostStableLabelClass::LifecycleLabel,
            host_catalog_ref: "i18n:host:lifecycle-vocabulary:v1".to_owned(),
            reserved_namespace_prefix: "host.lifecycle.".to_owned(),
            contributed_override_forbidden: true,
            presentation_label: "Lifecycle labels are host-controlled".to_owned(),
        },
    ]
}

#[allow(clippy::too_many_arguments)]
fn manifest(
    manifest_id: &str,
    owner_id: &str,
    owner_class: ContributedPackOwnerClass,
    support_mode: ExtensionLocaleSupportMode,
    locale_pack_ref: Option<&str>,
    coverage_locales: &[&str],
    compatibility_build_range: CompatibilityBuildRange,
    owned_surface_families: &[MessageSurfaceFamily],
    owned_namespace_prefix: &str,
    source_class: LocalePackSourceClass,
    signature_state: LocalePackSignatureState,
    companion_scope_narrower_than_desktop: bool,
) -> ContributedLocaleManifest {
    ContributedLocaleManifest {
        record_kind: CONTRIBUTED_LOCALE_MANIFEST_RECORD_KIND.to_owned(),
        schema_version: CONTRIBUTED_LOCALE_SCHEMA_VERSION,
        manifest_id: manifest_id.to_owned(),
        owner_id: owner_id.to_owned(),
        owner_namespace_ref: format!("{owner_id}:namespace"),
        owner_class,
        support_mode,
        locale_pack_ref: locale_pack_ref.map(str::to_owned),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        fallback_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        coverage_locales: coverage_locales.iter().map(|l| (*l).to_owned()).collect(),
        compatibility_build_range,
        owned_surface_families: owned_surface_families.to_vec(),
        owned_namespace_prefix: owned_namespace_prefix.to_owned(),
        host_stable_labels_referenced: ALL_HOST_STABLE_LABEL_CLASSES.to_vec(),
        may_override_host_stable_labels: false,
        source_class,
        signature_state,
        visible_disclosure_required: true,
        companion_scope_narrower_than_desktop,
    }
}

#[allow(clippy::too_many_arguments)]
fn support_row(
    row_id: &str,
    manifest: &ContributedLocaleManifest,
    requested_locale: &str,
    input: ContributedEvaluationInput,
    claimed_localized_profile: bool,
    missing_support_on_claimed_profile: bool,
    degraded_localization_state: DegradedLocalizationState,
    presentation_label: &str,
) -> ContributedLocaleSupportRow {
    let (application_decision, degrade_reason) = decide_contributed_support(&input);
    let effective_locale = if application_decision.applies() {
        requested_locale.to_owned()
    } else {
        SOURCE_LANGUAGE_LOCALE.to_owned()
    };
    ContributedLocaleSupportRow {
        row_id: row_id.to_owned(),
        manifest_id: manifest.manifest_id.clone(),
        owner_id: manifest.owner_id.clone(),
        owner_class: manifest.owner_class,
        requested_locale: requested_locale.to_owned(),
        effective_locale,
        target_build_in_compatibility_range: input.target_build_in_compatibility_range,
        signature_state: input.signature_state,
        version_match_state: input.version_match_state,
        application_decision,
        degrade_reason,
        degraded_localization_state,
        missing_support_on_claimed_profile,
        issue_source_class: manifest.owner_class.issue_source(),
        host_stable_labels_preserved: ALL_HOST_STABLE_LABEL_CLASSES.to_vec(),
        claimed_localized_profile,
        open_in_source_language_route_ref: SOURCE_LANGUAGE_ROUTE.to_owned(),
        presentation_label: presentation_label.to_owned(),
    }
}

/// Returns the seeded contributed-locale support report.
///
/// The seed exercises every degrade reason across both owner classes: a clean
/// extension pack, a signature-blocked extension pack, a source-only extension,
/// a build-skewed extension pack, a companion overlay that applies for its
/// covered scope, and a companion overlay that is deliberately narrower than
/// desktop. Host-stable labels stay canonical on every row.
pub fn seeded_contributed_locale_support_report() -> ContributedLocaleSupportReport {
    let manifests = vec![
        manifest(
            "contributed-locale:ext:notebook-charts:fr-fr",
            "ext:notebook-charts",
            ContributedPackOwnerClass::ExtensionOwnedPack,
            ExtensionLocaleSupportMode::ShipsOwnLocalePack,
            Some("locale-pack:extension:notebook-charts:fr-fr"),
            &["fr-FR"],
            build_range(),
            &[
                MessageSurfaceFamily::ExtensionContributedUi,
                MessageSurfaceFamily::CommandLabel,
            ],
            "ext.notebook-charts.",
            LocalePackSourceClass::ExtensionOwnedPack,
            LocalePackSignatureState::SignedVerified,
            false,
        ),
        manifest(
            "contributed-locale:ext:docs-helper:de-de",
            "ext:docs-helper",
            ContributedPackOwnerClass::ExtensionOwnedPack,
            ExtensionLocaleSupportMode::ShipsOwnLocalePack,
            Some("locale-pack:extension:docs-helper:de-de"),
            &["de-DE"],
            build_range(),
            &[MessageSurfaceFamily::ExtensionContributedUi],
            "ext.docs-helper.",
            LocalePackSourceClass::ExtensionOwnedPack,
            LocalePackSignatureState::SignatureFailedBlocked,
            false,
        ),
        manifest(
            "contributed-locale:ext:legacy-runner:source-only",
            "ext:legacy-runner",
            ContributedPackOwnerClass::ExtensionOwnedPack,
            ExtensionLocaleSupportMode::SourceLanguageOnly,
            None,
            &[],
            build_range(),
            &[MessageSurfaceFamily::ExtensionContributedUi],
            "ext.legacy-runner.",
            LocalePackSourceClass::ExtensionOwnedPack,
            LocalePackSignatureState::NotApplicableBuiltIn,
            false,
        ),
        manifest(
            "contributed-locale:ext:profiler-views:es-mx",
            "ext:profiler-views",
            ContributedPackOwnerClass::ExtensionOwnedPack,
            ExtensionLocaleSupportMode::ShipsOwnLocalePack,
            Some("locale-pack:extension:profiler-views:es-mx"),
            &["es-MX"],
            out_of_range(),
            &[MessageSurfaceFamily::ExtensionContributedUi],
            "ext.profiler-views.",
            LocalePackSourceClass::ExtensionOwnedPack,
            LocalePackSignatureState::SignedVerified,
            false,
        ),
        manifest(
            "contributed-locale:companion:browser-handoff:fr-fr",
            "companion:browser-handoff",
            ContributedPackOwnerClass::CompanionOverlayPack,
            ExtensionLocaleSupportMode::ShipsCompanionLocalePack,
            Some("locale-pack:companion:browser-handoff:fr-fr"),
            &["fr-FR"],
            build_range(),
            &[MessageSurfaceFamily::ShellChrome],
            "companion.browser-handoff.",
            LocalePackSourceClass::FirstPartyLocalePack,
            LocalePackSignatureState::SignedVerified,
            false,
        ),
        manifest(
            "contributed-locale:companion:browser-handoff:ja-jp",
            "companion:browser-handoff",
            ContributedPackOwnerClass::CompanionOverlayPack,
            ExtensionLocaleSupportMode::ShipsCompanionLocalePack,
            Some("locale-pack:companion:browser-handoff:ja-jp"),
            &["ja-JP"],
            build_range(),
            &[MessageSurfaceFamily::ShellChrome],
            "companion.browser-handoff.",
            LocalePackSourceClass::FirstPartyLocalePack,
            LocalePackSignatureState::SignedVerified,
            true,
        ),
    ];

    let support_rows = vec![
        support_row(
            "contributed-support:ext:notebook-charts:fr-fr",
            &manifests[0],
            "fr-FR",
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::ExtensionOwnedPack,
                pack_present_for_locale: true,
                target_build_in_compatibility_range: true,
                signature_state: LocalePackSignatureState::SignedVerified,
                version_match_state: VersionMatchState::ExactBuildMatch,
                policy_locale_enabled: true,
                companion_scope_narrower_than_desktop: false,
            },
            true,
            false,
            DegradedLocalizationState::FullyLocalized,
            "Notebook charts: localized (fr-FR)",
        ),
        support_row(
            "contributed-support:ext:docs-helper:de-de",
            &manifests[1],
            "de-DE",
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::ExtensionOwnedPack,
                pack_present_for_locale: true,
                target_build_in_compatibility_range: true,
                signature_state: LocalePackSignatureState::SignatureFailedBlocked,
                version_match_state: VersionMatchState::ExactBuildMatch,
                policy_locale_enabled: true,
                companion_scope_narrower_than_desktop: false,
            },
            false,
            true,
            DegradedLocalizationState::FailedPackSourceLanguageOnly,
            "Docs helper: signature failed, source language (de-DE)",
        ),
        support_row(
            "contributed-support:ext:legacy-runner:ja-jp",
            &manifests[2],
            "ja-JP",
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::ExtensionOwnedPack,
                pack_present_for_locale: false,
                target_build_in_compatibility_range: true,
                signature_state: LocalePackSignatureState::NotApplicableBuiltIn,
                version_match_state: VersionMatchState::ExactBuildMatch,
                policy_locale_enabled: true,
                companion_scope_narrower_than_desktop: false,
            },
            false,
            true,
            DegradedLocalizationState::FailedPackSourceLanguageOnly,
            "Legacy runner: source language only (ja-JP)",
        ),
        support_row(
            "contributed-support:ext:profiler-views:es-mx",
            &manifests[3],
            "es-MX",
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::ExtensionOwnedPack,
                pack_present_for_locale: true,
                target_build_in_compatibility_range: false,
                signature_state: LocalePackSignatureState::SignedVerified,
                version_match_state: VersionMatchState::IncompatibleDriftDetected,
                policy_locale_enabled: true,
                companion_scope_narrower_than_desktop: false,
            },
            false,
            true,
            DegradedLocalizationState::FailedPackSourceLanguageOnly,
            "Profiler views: build out of range, source language (es-MX)",
        ),
        support_row(
            "contributed-support:companion:browser-handoff:fr-fr",
            &manifests[4],
            "fr-FR",
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::CompanionOverlayPack,
                pack_present_for_locale: true,
                target_build_in_compatibility_range: true,
                signature_state: LocalePackSignatureState::SignedVerified,
                version_match_state: VersionMatchState::ExactBuildMatch,
                policy_locale_enabled: true,
                companion_scope_narrower_than_desktop: false,
            },
            false,
            false,
            DegradedLocalizationState::PartialTranslationDisclosed,
            "Companion handoff: localized for covered scope (fr-FR)",
        ),
        support_row(
            "contributed-support:companion:browser-handoff:ja-jp",
            &manifests[5],
            "ja-JP",
            ContributedEvaluationInput {
                owner_class: ContributedPackOwnerClass::CompanionOverlayPack,
                pack_present_for_locale: true,
                target_build_in_compatibility_range: true,
                signature_state: LocalePackSignatureState::SignedVerified,
                version_match_state: VersionMatchState::ExactBuildMatch,
                policy_locale_enabled: true,
                companion_scope_narrower_than_desktop: true,
            },
            false,
            false,
            DegradedLocalizationState::MixedLocaleStrictSeparation,
            "Companion handoff: narrower than desktop, disclosed (ja-JP)",
        ),
    ];

    let host_stable_label_guards = host_stable_label_guards();
    let mut report = ContributedLocaleSupportReport {
        record_kind: CONTRIBUTED_LOCALE_SUPPORT_REPORT_RECORD_KIND.to_owned(),
        schema_version: CONTRIBUTED_LOCALE_SCHEMA_VERSION,
        report_id: CONTRIBUTED_LOCALE_SUPPORT_REPORT_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_contract_id: LOCALE_PACK_BETA_CONTRACT_ID.to_owned(),
        first_party_compatibility_report_ref: FIRST_PARTY_COMPATIBILITY_REPORT_REF.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        host_stable_label_guards,
        manifests,
        support_rows,
        summary: ContributedLocaleSupportSummary {
            total_manifests: 0,
            extension_manifests: 0,
            companion_manifests: 0,
            applied_rows: 0,
            degraded_rows: 0,
            missing_support_rows: 0,
            host_stable_label_classes_protected: 0,
            guardrail_clean: false,
        },
        raw_translated_body_omitted: true,
    };
    report.summary = derive_summary(&report);
    report
}

#[cfg(test)]
mod tests;
