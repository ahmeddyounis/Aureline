//! Versioned, machine-readable design-system foundation packages for the claimed M5 surfaces.
//!
//! Where [`crate::m5_design_system_contract`] freezes the *governance matrix* — the inventory
//! of governed objects, their owners, and the coverage gate — this module ships the actual
//! *content* those objects reference: a versioned [`M5FoundationPackage`] that carries one
//! [`M5FoundationFamily`] per governed foundation kind (color, spacing, typography, icon,
//! density, motion, contrast, and component-state), each with its own
//! [version field](M5FoundationFamily::family_version) and a list of [entries](M5FoundationEntry).
//!
//! The package is the single source the density, reduced-motion, power-saving, and
//! high-contrast rows read from, so those rows cannot drift by surface family: the
//! [`M5FoundationPackage::density_tokens`], [`M5FoundationPackage::motion_postures`],
//! [`M5FoundationPackage::contrast_tokens`], and [`M5FoundationPackage::state_tokens`] accessors
//! resolve the same vocabulary [`aureline_ui`] and [`crate::CanonicalStateClass`] publish, and
//! the validator rejects a package whose density / motion / contrast / state rows fall out of
//! step with that canonical vocabulary.
//!
//! Every entry declares an explicit [support state](M5SupportState): a `deprecated` or
//! `unsupported` entry stays in the package and carries its [downgrade](M5EntryDowngrade)
//! target and reason, so unsupported or deprecated tokens remain inspectable and explicitly
//! downgraded instead of being silently dropped. That guarantee carries through the three
//! lifecycle operations this module supports:
//!
//! - **export / import** — [`M5FoundationPackage::export_safe_json`] mints deterministic JSON
//!   and [`M5FoundationPackage::from_json`] reads it back, validating on import.
//! - **diff** — [`M5FoundationPackage::diff`] produces a [`M5FoundationPackageDiff`] that names
//!   added, removed, changed, and downgraded entries per family; removed and downgraded entries
//!   are retained in the diff (with their last support state), never dropped.
//! - **release-packet inclusion** — [`M5FoundationPackage::release_packet`] projects a
//!   [`M5FoundationPackageReleasePacket`] whose `downgraded_entries` block enumerates every
//!   deprecated or unsupported entry for the release record.
//!
//! The records are metadata-only truth packets: they carry semantic token *references* and
//! posture/class *tokens*, never raw color values, credential bodies, or provider payloads.
//!
//! - Schema:
//!   [`schemas/design-system/m5-foundation-package.schema.json`](../../../../../schemas/design-system/m5-foundation-package.schema.json)
//! - Doc:
//!   [`docs/design-system/m5-foundation-package.md`](../../../../../docs/design-system/m5-foundation-package.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_foundation_package, seeded_m5_foundation_package_next, M5_FOUNDATION_PACKAGE_ID,
    M5_FOUNDATION_PACKAGE_NEXT_VERSION, M5_FOUNDATION_PACKAGE_VERSION,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_ui::density::DensityClass;
use aureline_ui::themes::AccessibilityPostureClass;
use aureline_ui::tokens::ThemeClass;

use crate::CanonicalStateClass;

/// Record-kind tag carried by [`M5FoundationPackage`].
pub const M5_FOUNDATION_PACKAGE_RECORD_KIND: &str = "m5_design_system_foundation_package";

/// Record-kind tag carried by [`M5FoundationPackageDiff`].
pub const M5_FOUNDATION_PACKAGE_DIFF_RECORD_KIND: &str = "m5_design_system_foundation_package_diff";

/// Record-kind tag carried by [`M5FoundationPackageReleasePacket`].
pub const M5_FOUNDATION_PACKAGE_RELEASE_RECORD_KIND: &str =
    "m5_design_system_foundation_package_release";

/// Schema version shared by the foundation-package records.
pub const M5_FOUNDATION_PACKAGE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the foundation-package boundary schema.
pub const M5_FOUNDATION_PACKAGE_SCHEMA_REF: &str =
    "schemas/design-system/m5-foundation-package.schema.json";

/// Repo-relative path of the foundation-package contract doc.
pub const M5_FOUNDATION_PACKAGE_DOC_REF: &str = "docs/design-system/m5-foundation-package.md";

/// Repo-relative path of the release-grade foundation-package proof packet — the proof lane
/// that blocks drift for the package.
pub const M5_FOUNDATION_PACKAGE_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/foundation-package-release.json";

/// Release packet that keeps the foundation package current (shared with the contract matrix).
pub const M5_FOUNDATION_PACKAGE_RELEASE_PACKET_REF: &str =
    "evidence:m5-design-system-release-packet";

/// Repo-relative path of the checked-in package fixture directory.
pub const M5_FOUNDATION_PACKAGE_DIR: &str = "fixtures/ui/m5-foundation-package/";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_FOUNDATION_MESSAGE_ID_PREFIX: &str = "design_system_foundation.";

/// One governed foundation family kind the package versions and ships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FoundationFamilyKind {
    /// Semantic color token family.
    Color,
    /// Spacing-scale token family.
    Spacing,
    /// Typography token family.
    Typography,
    /// Icon token family.
    Icon,
    /// Density-class family (compact / standard / comfortable).
    Density,
    /// Motion-posture family (standard / reduced / low-motion / power-saver / critical hot path).
    Motion,
    /// Contrast/theme-class family (dark / light / high-contrast variants).
    Contrast,
    /// Controlled component-state family (empty / loading / pending / … / completed).
    ComponentState,
}

impl M5FoundationFamilyKind {
    /// Every family kind, in declaration order. The package must publish one family per kind.
    pub const ALL: [Self; 8] = [
        Self::Color,
        Self::Spacing,
        Self::Typography,
        Self::Icon,
        Self::Density,
        Self::Motion,
        Self::Contrast,
        Self::ComponentState,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Color => "color",
            Self::Spacing => "spacing",
            Self::Typography => "typography",
            Self::Icon => "icon",
            Self::Density => "density",
            Self::Motion => "motion",
            Self::Contrast => "contrast",
            Self::ComponentState => "component_state",
        }
    }
}

/// Support state of one foundation entry. Unsupported and deprecated entries stay published and
/// carry a [downgrade](M5EntryDowngrade); they are never silently dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportState {
    /// The entry resolves and is fully supported.
    Supported,
    /// The entry still resolves but is on a removal path; it carries a downgrade replacement.
    Deprecated,
    /// The entry no longer resolves on the target surfaces; it carries a downgrade fallback.
    Unsupported,
}

impl M5SupportState {
    /// Every support state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Supported, Self::Deprecated, Self::Unsupported];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Deprecated => "deprecated",
            Self::Unsupported => "unsupported",
        }
    }

    /// True when the entry is fully supported (and therefore carries no downgrade).
    pub const fn is_supported(self) -> bool {
        matches!(self, Self::Supported)
    }

    /// Restrictiveness rank used to detect a downgrade transition (supported least, unsupported
    /// most).
    const fn rank(self) -> u8 {
        match self {
            Self::Supported => 0,
            Self::Deprecated => 1,
            Self::Unsupported => 2,
        }
    }
}

/// Explicit downgrade carried by a deprecated or unsupported entry, so the entry stays
/// inspectable rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryDowngrade {
    /// The replacement or fallback entry id / token consumers should resolve instead.
    pub downgraded_to: String,
    /// Stable message id naming the downgrade reason; prefixed [`M5_FOUNDATION_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Package version at which the entry was downgraded.
    pub since_package_version: String,
}

/// One entry in a foundation family: a named semantic token reference with an explicit support
/// state and, when not fully supported, a downgrade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationEntry {
    /// Stable entry id, unique within its family.
    pub entry_id: String,
    /// Human-readable entry name.
    pub display_name: String,
    /// Semantic token reference or posture/class token this entry resolves to (metadata only).
    pub value_token: String,
    /// Support state of the entry.
    pub support_state: M5SupportState,
    /// Downgrade target and reason; present iff the entry is not fully supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade: Option<M5EntryDowngrade>,
}

impl M5FoundationEntry {
    /// Builds a fully supported entry (no downgrade).
    pub fn supported(entry_id: &str, display_name: &str, value_token: &str) -> Self {
        Self {
            entry_id: entry_id.to_owned(),
            display_name: display_name.to_owned(),
            value_token: value_token.to_owned(),
            support_state: M5SupportState::Supported,
            downgrade: None,
        }
    }

    /// Builds a downgraded (deprecated or unsupported) entry.
    pub fn downgraded(
        entry_id: &str,
        display_name: &str,
        value_token: &str,
        support_state: M5SupportState,
        downgrade: M5EntryDowngrade,
    ) -> Self {
        Self {
            entry_id: entry_id.to_owned(),
            display_name: display_name.to_owned(),
            value_token: value_token.to_owned(),
            support_state,
            downgrade: Some(downgrade),
        }
    }

    /// True when the entry is deprecated or unsupported (and so carries a downgrade).
    pub fn is_downgraded(&self) -> bool {
        !self.support_state.is_supported()
    }
}

/// One governed foundation family: a versioned, named list of entries for a single foundation
/// kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationFamily {
    /// The governed family kind.
    pub family_kind: M5FoundationFamilyKind,
    /// Stable family id, unique within the package.
    pub family_id: String,
    /// Human-readable family name.
    pub display_name: String,
    /// Monotonic family version; bumps when this family's entries change.
    pub family_version: u32,
    /// The family's entries.
    pub entries: Vec<M5FoundationEntry>,
}

impl M5FoundationFamily {
    /// Finds an entry by id.
    pub fn entry(&self, entry_id: &str) -> Option<&M5FoundationEntry> {
        self.entries.iter().find(|e| e.entry_id == entry_id)
    }

    /// The value tokens of every entry, in declared order.
    pub fn value_tokens(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|e| e.value_token.as_str())
            .collect()
    }
}

/// A versioned, machine-readable design-system foundation package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationPackage {
    /// Record kind; must equal [`M5_FOUNDATION_PACKAGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FOUNDATION_PACKAGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable package id.
    pub package_id: String,
    /// Package version (semver `MAJOR.MINOR.PATCH`).
    pub package_version: String,
    /// Owner role accountable for the package.
    pub owner_role: String,
    /// The governed foundation families (one per [`M5FoundationFamilyKind`]).
    pub families: Vec<M5FoundationFamily>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Repo-relative release packet that keeps the package current.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable summary message id; prefixed [`M5_FOUNDATION_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5FoundationPackage {
    /// Finds the family for a kind.
    pub fn family(&self, kind: M5FoundationFamilyKind) -> Option<&M5FoundationFamily> {
        self.families.iter().find(|f| f.family_kind == kind)
    }

    /// The value tokens of a family kind's entries, in declared order. Returns an empty vector
    /// when the family is absent.
    fn family_value_tokens(&self, kind: M5FoundationFamilyKind) -> Vec<&str> {
        self.family(kind)
            .map(M5FoundationFamily::value_tokens)
            .unwrap_or_default()
    }

    /// The density-class tokens the package publishes — the single source the density rows read
    /// from.
    pub fn density_tokens(&self) -> Vec<&str> {
        self.family_value_tokens(M5FoundationFamilyKind::Density)
    }

    /// The motion-posture tokens the package publishes — the single source the reduced-motion
    /// and power-saving rows read from.
    pub fn motion_postures(&self) -> Vec<&str> {
        self.family_value_tokens(M5FoundationFamilyKind::Motion)
    }

    /// The contrast/theme-class tokens the package publishes — the single source the
    /// high-contrast rows read from.
    pub fn contrast_tokens(&self) -> Vec<&str> {
        self.family_value_tokens(M5FoundationFamilyKind::Contrast)
    }

    /// The high-contrast subset of the contrast tokens.
    pub fn high_contrast_tokens(&self) -> Vec<&str> {
        self.contrast_tokens()
            .into_iter()
            .filter(|t| t.contains("high_contrast"))
            .collect()
    }

    /// The controlled component-state tokens the package publishes.
    pub fn state_tokens(&self) -> Vec<&str> {
        self.family_value_tokens(M5FoundationFamilyKind::ComponentState)
    }

    /// The motion entry carrying the reduced-motion posture, if published.
    pub fn reduced_motion_entry(&self) -> Option<&M5FoundationEntry> {
        self.motion_entry_with_token(AccessibilityPostureClass::MotionReduced.token())
    }

    /// The motion entry carrying the power-saving posture, if published.
    pub fn power_saving_entry(&self) -> Option<&M5FoundationEntry> {
        self.motion_entry_with_token(AccessibilityPostureClass::MotionPowerSaver.token())
    }

    fn motion_entry_with_token(&self, token: &str) -> Option<&M5FoundationEntry> {
        self.family(M5FoundationFamilyKind::Motion)?
            .entries
            .iter()
            .find(|e| e.value_token == token)
    }

    /// Every deprecated or unsupported entry across all families, paired with its family kind —
    /// the inspectable, explicitly downgraded inventory.
    pub fn downgraded_entries(&self) -> Vec<(M5FoundationFamilyKind, &M5FoundationEntry)> {
        let mut out: Vec<(M5FoundationFamilyKind, &M5FoundationEntry)> = self
            .families
            .iter()
            .flat_map(|f| {
                f.entries
                    .iter()
                    .filter(|e| e.is_downgraded())
                    .map(move |e| (f.family_kind, e))
            })
            .collect();
        out.sort_by(|a, b| {
            a.0.as_str()
                .cmp(b.0.as_str())
                .then(a.1.entry_id.cmp(&b.1.entry_id))
        });
        out
    }

    /// Total entry count across all families.
    pub fn total_entries(&self) -> usize {
        self.families.iter().map(|f| f.entries.len()).sum()
    }

    /// Validates the package invariants, returning the violations (empty when valid).
    pub fn validate(&self) -> Vec<M5FoundationPackageViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FOUNDATION_PACKAGE_RECORD_KIND {
            violations.push(M5FoundationPackageViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FOUNDATION_PACKAGE_SCHEMA_VERSION {
            violations.push(M5FoundationPackageViolation::WrongSchemaVersion);
        }
        if self.package_id.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.proof_lane_ref.trim().is_empty()
            || self.release_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FoundationPackageViolation::MissingIdentity);
        }
        if !is_semver(&self.package_version) {
            violations.push(M5FoundationPackageViolation::BadPackageVersion);
        }
        if !self
            .summary_message_id
            .starts_with(M5_FOUNDATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5FoundationPackageViolation::MessageIdPrefixMissing);
        }

        for required in [
            M5_FOUNDATION_PACKAGE_SCHEMA_REF,
            M5_FOUNDATION_PACKAGE_DOC_REF,
            M5_FOUNDATION_PACKAGE_PROOF_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.push(M5FoundationPackageViolation::MissingSourceContracts);
                break;
            }
        }

        validate_families(self, &mut violations);
        validate_canonical_rows(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 foundation package serializes"),
        ) {
            violations.push(M5FoundationPackageViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// True when the package validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON for the package.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 foundation package serializes")
    }

    /// Imports a package from JSON. The caller validates the returned package with
    /// [`Self::validate`].
    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    /// Diffs this package (the *from* side) against `to` (the *next* side), naming every added,
    /// removed, changed, and downgraded entry per family. Removed and downgraded entries are
    /// retained in the diff with their last support state, never dropped.
    pub fn diff(&self, to: &M5FoundationPackage) -> M5FoundationPackageDiff {
        let mut family_diffs = Vec::new();
        for kind in M5FoundationFamilyKind::ALL {
            if let Some(diff) = family_diff(kind, self.family(kind), to.family(kind)) {
                family_diffs.push(diff);
            }
        }

        let added: u32 = family_diffs
            .iter()
            .map(|f| f.added_entries.len() as u32)
            .sum();
        let removed: u32 = family_diffs
            .iter()
            .map(|f| f.removed_entries.len() as u32)
            .sum();
        let changed: u32 = family_diffs
            .iter()
            .map(|f| f.changed_entries.len() as u32)
            .sum();
        let downgraded: u32 = family_diffs
            .iter()
            .map(|f| f.downgraded_entries.len() as u32)
            .sum();

        M5FoundationPackageDiff {
            record_kind: M5_FOUNDATION_PACKAGE_DIFF_RECORD_KIND.to_owned(),
            schema_version: M5_FOUNDATION_PACKAGE_SCHEMA_VERSION,
            from_package_id: self.package_id.clone(),
            to_package_id: to.package_id.clone(),
            from_version: self.package_version.clone(),
            to_version: to.package_version.clone(),
            family_diffs,
            added_entry_count: added,
            removed_entry_count: removed,
            changed_entry_count: changed,
            downgraded_entry_count: downgraded,
            // The diff retains every removed and downgraded entry below, so unsupported and
            // downgraded-state information is never lost across versions.
            retains_unsupported_and_downgraded: true,
            summary_message_id: format!(
                "{}diff.{}.{}",
                M5_FOUNDATION_MESSAGE_ID_PREFIX, self.package_version, to.package_version
            ),
        }
    }

    /// Projects the release-packet inclusion: per-family support counts plus the full
    /// downgraded-entry inventory, so a release record preserves unsupported and deprecated
    /// entries rather than dropping them.
    pub fn release_packet(&self) -> M5FoundationPackageReleasePacket {
        let family_summaries: Vec<M5FoundationFamilySummary> = self
            .families
            .iter()
            .map(|f| {
                let count_state = |state: M5SupportState| {
                    f.entries
                        .iter()
                        .filter(|e| e.support_state == state)
                        .count() as u32
                };
                M5FoundationFamilySummary {
                    family_kind: f.family_kind,
                    family_id: f.family_id.clone(),
                    family_version: f.family_version,
                    entry_count: f.entries.len() as u32,
                    supported_count: count_state(M5SupportState::Supported),
                    deprecated_count: count_state(M5SupportState::Deprecated),
                    unsupported_count: count_state(M5SupportState::Unsupported),
                }
            })
            .collect();

        let downgraded_entries: Vec<M5DowngradedEntryRecord> = self
            .downgraded_entries()
            .into_iter()
            .map(|(kind, entry)| M5DowngradedEntryRecord {
                family_kind: kind,
                entry_id: entry.entry_id.clone(),
                value_token: entry.value_token.clone(),
                support_state: entry.support_state,
                downgrade: entry
                    .downgrade
                    .clone()
                    .expect("downgraded entry carries a downgrade"),
            })
            .collect();

        let total_entries = self.total_entries() as u32;
        let total_downgraded = downgraded_entries.len() as u32;

        M5FoundationPackageReleasePacket {
            record_kind: M5_FOUNDATION_PACKAGE_RELEASE_RECORD_KIND.to_owned(),
            schema_version: M5_FOUNDATION_PACKAGE_SCHEMA_VERSION,
            package_id: self.package_id.clone(),
            package_version: self.package_version.clone(),
            family_summaries,
            downgraded_entries,
            total_entries,
            total_supported: total_entries - total_downgraded,
            total_downgraded,
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.release",
                M5_FOUNDATION_MESSAGE_ID_PREFIX, self.package_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Reads and validates the checked-in canonical package fixture.
pub fn current_stable_m5_foundation_package(
) -> Result<M5FoundationPackage, M5FoundationPackageArtifactError> {
    let package: M5FoundationPackage = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-foundation-package/foundation-package.json"
    )))
    .map_err(M5FoundationPackageArtifactError::Parse)?;
    let violations = package.validate();
    if violations.is_empty() {
        Ok(package)
    } else {
        Err(M5FoundationPackageArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading a checked-in package export.
#[derive(Debug)]
pub enum M5FoundationPackageArtifactError {
    /// The export failed to parse.
    Parse(serde_json::Error),
    /// The export failed validation.
    Validation(Vec<M5FoundationPackageViolation>),
}

impl fmt::Display for M5FoundationPackageArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "m5 foundation package parse failed: {error}"),
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 foundation package failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5FoundationPackageArtifactError {}

/// Validation failures emitted by [`M5FoundationPackage::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FoundationPackageViolation {
    /// Package record kind is wrong.
    WrongRecordKind,
    /// Package schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The package version is not `MAJOR.MINOR.PATCH`.
    BadPackageVersion,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A governed family kind has no published family.
    RequiredFamilyKindMissing,
    /// Two families share a kind.
    DuplicateFamilyKind,
    /// Two families share an id.
    DuplicateFamilyId,
    /// A family is incomplete (empty id/name, zero version, or no entries).
    FamilyIncomplete,
    /// Two entries within a family share an id.
    DuplicateEntryId,
    /// An entry is incomplete (empty id, name, or value token).
    EntryIncomplete,
    /// An entry's support state and downgrade disagree (supported with a downgrade, or
    /// deprecated/unsupported without one), or a downgrade field is missing.
    DowngradeInconsistent,
    /// The density family does not publish exactly the canonical density-class tokens.
    DensityRowsIncomplete,
    /// The motion family does not publish all canonical motion-posture tokens.
    MotionRowsIncomplete,
    /// The contrast family does not publish all canonical theme-class tokens.
    ContrastRowsIncomplete,
    /// The component-state family does not publish all canonical controlled-state tokens.
    StateFamilyIncomplete,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5FoundationPackageViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BadPackageVersion => "bad_package_version",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyKindMissing => "required_family_kind_missing",
            Self::DuplicateFamilyKind => "duplicate_family_kind",
            Self::DuplicateFamilyId => "duplicate_family_id",
            Self::FamilyIncomplete => "family_incomplete",
            Self::DuplicateEntryId => "duplicate_entry_id",
            Self::EntryIncomplete => "entry_incomplete",
            Self::DowngradeInconsistent => "downgrade_inconsistent",
            Self::DensityRowsIncomplete => "density_rows_incomplete",
            Self::MotionRowsIncomplete => "motion_rows_incomplete",
            Self::ContrastRowsIncomplete => "contrast_rows_incomplete",
            Self::StateFamilyIncomplete => "state_family_incomplete",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Diff records.
// ---------------------------------------------------------------------------

/// Deterministic diff of two foundation packages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationPackageDiff {
    /// Record kind; must equal [`M5_FOUNDATION_PACKAGE_DIFF_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The *from* (older) package id.
    pub from_package_id: String,
    /// The *to* (newer) package id.
    pub to_package_id: String,
    /// The *from* package version.
    pub from_version: String,
    /// The *to* package version.
    pub to_version: String,
    /// Per-family diffs, sorted by family-kind token. A family appears only when it changed.
    pub family_diffs: Vec<M5FoundationFamilyDiff>,
    /// Total entries added across families.
    pub added_entry_count: u32,
    /// Total entries removed across families.
    pub removed_entry_count: u32,
    /// Total entries changed across families.
    pub changed_entry_count: u32,
    /// Total entries downgraded across families.
    pub downgraded_entry_count: u32,
    /// Always true: the diff retains removed and downgraded entries with their last support
    /// state, so unsupported and downgraded-state information is never lost.
    pub retains_unsupported_and_downgraded: bool,
    /// Stable message id; prefixed [`M5_FOUNDATION_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5FoundationPackageDiff {
    /// Deterministic export-safe JSON for the diff.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 foundation package diff serializes")
    }

    /// True when the two packages are identical (no family changed).
    pub fn is_empty(&self) -> bool {
        self.family_diffs.is_empty()
    }
}

/// One family's diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationFamilyDiff {
    /// The governed family kind.
    pub family_kind: M5FoundationFamilyKind,
    /// Family version on the *from* side (`None` when the family was added).
    pub from_version: Option<u32>,
    /// Family version on the *to* side (`None` when the family was removed).
    pub to_version: Option<u32>,
    /// Entry ids added (present in *to*, absent in *from*), sorted.
    pub added_entries: Vec<String>,
    /// Entries removed (present in *from*, absent in *to*), retained with their last support
    /// state, sorted by entry id.
    pub removed_entries: Vec<M5RemovedEntry>,
    /// Entries whose value token or support state changed, sorted by entry id.
    pub changed_entries: Vec<M5ChangedEntry>,
    /// Entry ids whose support state moved toward less support, sorted.
    pub downgraded_entries: Vec<String>,
}

/// A removed entry, retained in the diff with its last support state so it is never silently
/// dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RemovedEntry {
    /// The removed entry id.
    pub entry_id: String,
    /// Its support state on the *from* side.
    pub last_support_state: M5SupportState,
    /// Its downgrade on the *from* side, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_downgrade: Option<M5EntryDowngrade>,
}

/// A changed entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangedEntry {
    /// The entry id.
    pub entry_id: String,
    /// True when the value token changed.
    pub value_changed: bool,
    /// Support state on the *from* side.
    pub support_from: M5SupportState,
    /// Support state on the *to* side.
    pub support_to: M5SupportState,
}

// ---------------------------------------------------------------------------
// Release-packet records.
// ---------------------------------------------------------------------------

/// Release-packet projection of a foundation package: per-family support counts plus the full
/// downgraded-entry inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationPackageReleasePacket {
    /// Record kind; must equal [`M5_FOUNDATION_PACKAGE_RELEASE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The package id this release record projects.
    pub package_id: String,
    /// The package version.
    pub package_version: String,
    /// Per-family support summaries, in package order.
    pub family_summaries: Vec<M5FoundationFamilySummary>,
    /// Every deprecated or unsupported entry, preserved for the release record.
    pub downgraded_entries: Vec<M5DowngradedEntryRecord>,
    /// Total entries across all families.
    pub total_entries: u32,
    /// Total fully supported entries.
    pub total_supported: u32,
    /// Total deprecated or unsupported entries.
    pub total_downgraded: u32,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_FOUNDATION_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5FoundationPackageReleasePacket {
    /// Deterministic export-safe JSON for the release packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 foundation package release packet serializes")
    }
}

/// One family's support summary inside a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationFamilySummary {
    /// The governed family kind.
    pub family_kind: M5FoundationFamilyKind,
    /// The family id.
    pub family_id: String,
    /// The family version.
    pub family_version: u32,
    /// Total entries in the family.
    pub entry_count: u32,
    /// Supported entries.
    pub supported_count: u32,
    /// Deprecated entries.
    pub deprecated_count: u32,
    /// Unsupported entries.
    pub unsupported_count: u32,
}

/// One downgraded entry preserved in a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DowngradedEntryRecord {
    /// The family kind the entry belongs to.
    pub family_kind: M5FoundationFamilyKind,
    /// The entry id.
    pub entry_id: String,
    /// The entry's value token.
    pub value_token: String,
    /// The entry's support state (deprecated or unsupported).
    pub support_state: M5SupportState,
    /// The downgrade target and reason.
    pub downgrade: M5EntryDowngrade,
}

// ---------------------------------------------------------------------------
// Validation helpers.
// ---------------------------------------------------------------------------

fn validate_families(
    package: &M5FoundationPackage,
    violations: &mut Vec<M5FoundationPackageViolation>,
) {
    let present: BTreeSet<M5FoundationFamilyKind> =
        package.families.iter().map(|f| f.family_kind).collect();
    for required in M5FoundationFamilyKind::ALL {
        if !present.contains(&required) {
            violations.push(M5FoundationPackageViolation::RequiredFamilyKindMissing);
            break;
        }
    }
    if present.len() != package.families.len() {
        violations.push(M5FoundationPackageViolation::DuplicateFamilyKind);
    }

    let mut seen_family_ids: BTreeSet<&str> = BTreeSet::new();
    for family in &package.families {
        if !seen_family_ids.insert(family.family_id.as_str()) {
            violations.push(M5FoundationPackageViolation::DuplicateFamilyId);
        }
        if family.family_id.trim().is_empty()
            || family.display_name.trim().is_empty()
            || family.family_version == 0
            || family.entries.is_empty()
        {
            violations.push(M5FoundationPackageViolation::FamilyIncomplete);
        }

        let mut seen_entry_ids: BTreeSet<&str> = BTreeSet::new();
        for entry in &family.entries {
            if !seen_entry_ids.insert(entry.entry_id.as_str()) {
                violations.push(M5FoundationPackageViolation::DuplicateEntryId);
            }
            if entry.entry_id.trim().is_empty()
                || entry.display_name.trim().is_empty()
                || entry.value_token.trim().is_empty()
            {
                violations.push(M5FoundationPackageViolation::EntryIncomplete);
            }
            validate_entry_downgrade(entry, violations);
        }
    }
}

fn validate_entry_downgrade(
    entry: &M5FoundationEntry,
    violations: &mut Vec<M5FoundationPackageViolation>,
) {
    match (&entry.support_state, &entry.downgrade) {
        // A fully supported entry must not carry a downgrade.
        (M5SupportState::Supported, Some(_)) => {
            violations.push(M5FoundationPackageViolation::DowngradeInconsistent);
        }
        // A deprecated or unsupported entry must carry a complete downgrade.
        (M5SupportState::Deprecated | M5SupportState::Unsupported, None) => {
            violations.push(M5FoundationPackageViolation::DowngradeInconsistent);
        }
        (M5SupportState::Deprecated | M5SupportState::Unsupported, Some(downgrade)) => {
            if downgrade.downgraded_to.trim().is_empty()
                || !downgrade
                    .reason_message_id
                    .starts_with(M5_FOUNDATION_MESSAGE_ID_PREFIX)
                || !is_semver(&downgrade.since_package_version)
            {
                violations.push(M5FoundationPackageViolation::DowngradeInconsistent);
            }
        }
        (M5SupportState::Supported, None) => {}
    }
}

/// Validates that the density, motion, contrast, and component-state families read from the
/// same canonical vocabulary [`aureline_ui`] and [`CanonicalStateClass`] publish, so those rows
/// cannot drift by surface family.
fn validate_canonical_rows(
    package: &M5FoundationPackage,
    violations: &mut Vec<M5FoundationPackageViolation>,
) {
    let value_set = |kind: M5FoundationFamilyKind| -> BTreeSet<String> {
        package
            .family(kind)
            .map(|f| f.entries.iter().map(|e| e.value_token.clone()).collect())
            .unwrap_or_default()
    };

    if value_set(M5FoundationFamilyKind::Density) != canonical_density_tokens() {
        violations.push(M5FoundationPackageViolation::DensityRowsIncomplete);
    }
    // Motion and contrast may add entries beyond the canonical set, but must cover all of it.
    if !canonical_motion_tokens().is_subset(&value_set(M5FoundationFamilyKind::Motion)) {
        violations.push(M5FoundationPackageViolation::MotionRowsIncomplete);
    }
    if !canonical_contrast_tokens().is_subset(&value_set(M5FoundationFamilyKind::Contrast)) {
        violations.push(M5FoundationPackageViolation::ContrastRowsIncomplete);
    }
    if value_set(M5FoundationFamilyKind::ComponentState) != canonical_state_tokens() {
        violations.push(M5FoundationPackageViolation::StateFamilyIncomplete);
    }
}

fn family_diff(
    kind: M5FoundationFamilyKind,
    from: Option<&M5FoundationFamily>,
    to: Option<&M5FoundationFamily>,
) -> Option<M5FoundationFamilyDiff> {
    let from_version = from.map(|f| f.family_version);
    let to_version = to.map(|f| f.family_version);
    let from_entries = from.map(|f| f.entries.as_slice()).unwrap_or_default();
    let to_entries = to.map(|f| f.entries.as_slice()).unwrap_or_default();

    let mut added_entries: Vec<String> = to_entries
        .iter()
        .filter(|e| !from_entries.iter().any(|o| o.entry_id == e.entry_id))
        .map(|e| e.entry_id.clone())
        .collect();
    added_entries.sort();

    let mut removed_entries: Vec<M5RemovedEntry> = from_entries
        .iter()
        .filter(|e| !to_entries.iter().any(|n| n.entry_id == e.entry_id))
        .map(|e| M5RemovedEntry {
            entry_id: e.entry_id.clone(),
            last_support_state: e.support_state,
            last_downgrade: e.downgrade.clone(),
        })
        .collect();
    removed_entries.sort_by(|a, b| a.entry_id.cmp(&b.entry_id));

    let mut changed_entries: Vec<M5ChangedEntry> = Vec::new();
    let mut downgraded_entries: Vec<String> = Vec::new();
    for new_entry in to_entries {
        if let Some(old_entry) = from_entries
            .iter()
            .find(|o| o.entry_id == new_entry.entry_id)
        {
            let value_changed = old_entry.value_token != new_entry.value_token;
            let support_changed = old_entry.support_state != new_entry.support_state;
            if value_changed || support_changed {
                changed_entries.push(M5ChangedEntry {
                    entry_id: new_entry.entry_id.clone(),
                    value_changed,
                    support_from: old_entry.support_state,
                    support_to: new_entry.support_state,
                });
            }
            if new_entry.support_state.rank() > old_entry.support_state.rank() {
                downgraded_entries.push(new_entry.entry_id.clone());
            }
        }
    }
    changed_entries.sort_by(|a, b| a.entry_id.cmp(&b.entry_id));
    downgraded_entries.sort();

    let version_changed = from_version != to_version;
    if !version_changed
        && added_entries.is_empty()
        && removed_entries.is_empty()
        && changed_entries.is_empty()
    {
        return None;
    }

    Some(M5FoundationFamilyDiff {
        family_kind: kind,
        from_version,
        to_version,
        added_entries,
        removed_entries,
        changed_entries,
        downgraded_entries,
    })
}

// ---------------------------------------------------------------------------
// Canonical vocabulary helpers (the single source the rows reconcile to).
// ---------------------------------------------------------------------------

fn canonical_density_tokens() -> BTreeSet<String> {
    [
        DensityClass::Compact,
        DensityClass::Standard,
        DensityClass::Comfortable,
    ]
    .iter()
    .map(|v| v.token().to_owned())
    .collect()
}

fn canonical_motion_tokens() -> BTreeSet<String> {
    [
        AccessibilityPostureClass::MotionStandard,
        AccessibilityPostureClass::MotionReduced,
        AccessibilityPostureClass::MotionLowMotion,
        AccessibilityPostureClass::MotionPowerSaver,
        AccessibilityPostureClass::MotionCriticalHotPath,
    ]
    .iter()
    .map(|v| v.token().to_owned())
    .collect()
}

fn canonical_contrast_tokens() -> BTreeSet<String> {
    [
        ThemeClass::DarkReference,
        ThemeClass::LightParity,
        ThemeClass::HighContrastDark,
        ThemeClass::HighContrastLight,
    ]
    .iter()
    .map(|v| v.token().to_owned())
    .collect()
}

fn canonical_state_tokens() -> BTreeSet<String> {
    CanonicalStateClass::required()
        .iter()
        .map(|v| v.as_str().to_owned())
        .collect()
}

/// True when `value` is a `MAJOR.MINOR.PATCH` numeric semver.
fn is_semver(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material (credential
/// bodies, raw provider payloads). Foundation packages are metadata-only by construction; this
/// is a defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
