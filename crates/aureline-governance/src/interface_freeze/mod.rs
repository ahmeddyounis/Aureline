//! Typed interface-freeze register.
//!
//! The Beta interface freeze decides, per governed interface surface, whether
//! the contract is still `open`, `soft_frozen` (additive change expected), or
//! `hard_frozen` (locked at a recorded version). Until now that posture was
//! implicit in scattered version constants. This module promotes it into a
//! single typed register: one [`InterfaceFreezeRow`] per governed surface,
//! carrying its freeze state, the version it was frozen at, the current
//! version, the exception classes permitted for it, and any recorded
//! exceptions that authorized a change.
//!
//! The register is checked in at
//! `artifacts/governance/interface_freeze_register_beta.json` and embedded
//! here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI.
//!
//! Rows whose [`VersionSource`] is [`VersionSource::GovernedSchemaRegistry`]
//! are cross-checked against the governed schema-family registry
//! ([`crate::schema_registry`]): the register may not invent a version, drop a
//! governed family, or rename a frozen schema reference. Declared rows pin a
//! serialization surface (settings, portable state, extension manifest) that is
//! not itself a governed-registry family.
//!
//! Freeze state, surface class, and exception class are closed enums. A row
//! that carries a token outside the closed vocabulary fails to deserialize
//! rather than passing silently. A `hard_frozen` row may not move past its
//! frozen version without a recorded exception drawn from its allowed classes.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::schema_registry::GovernedSchemaRegistry;

/// Supported register schema version.
pub const INTERFACE_FREEZE_REGISTER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const INTERFACE_FREEZE_REGISTER_RECORD_KIND: &str = "interface_freeze_register";

/// Repo-relative path to the checked-in register.
pub const INTERFACE_FREEZE_REGISTER_PATH: &str =
    "artifacts/governance/interface_freeze_register_beta.json";

/// Embedded checked-in register JSON.
pub const INTERFACE_FREEZE_REGISTER_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/interface_freeze_register_beta.json"
));

/// Freeze posture of one governed interface surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreezeState {
    /// The surface may still change freely; it is not part of the Beta freeze.
    Open,
    /// Additive, backward-compatible change is expected and does not need an
    /// exception; breaking change is discouraged and reviewed.
    SoftFrozen,
    /// The surface is locked at its frozen version; any change requires a
    /// recorded exception from the row's allowed classes.
    HardFrozen,
}

impl FreezeState {
    /// Every freeze state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Open, Self::SoftFrozen, Self::HardFrozen];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::SoftFrozen => "soft_frozen",
            Self::HardFrozen => "hard_frozen",
        }
    }
}

/// Broad interface surface a freeze row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// CLI and headless machine-readable output.
    CliHeadless,
    /// Settings and portable-state serialization.
    SettingsPortableState,
    /// Extension SDK result schemas and extension manifests.
    ExtensionSdkManifest,
    /// Governed export/telemetry/diagnostic packet families.
    GovernedExportPacket,
}

impl SurfaceClass {
    /// Every surface class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CliHeadless,
        Self::SettingsPortableState,
        Self::ExtensionSdkManifest,
        Self::GovernedExportPacket,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CliHeadless => "cli_headless",
            Self::SettingsPortableState => "settings_portable_state",
            Self::ExtensionSdkManifest => "extension_sdk_manifest",
            Self::GovernedExportPacket => "governed_export_packet",
        }
    }
}

/// Class of change that may be permitted on a frozen surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreezeExceptionClass {
    /// A purely additive, backward-compatible change.
    AdditiveBackwardCompatible,
    /// A change required to correct a security defect.
    SecurityFix,
    /// A change required to correct an incorrect contract.
    DefectCorrection,
    /// An approved, coordinated breaking change with a migration path.
    CoordinatedBreakingChange,
}

impl FreezeExceptionClass {
    /// Every exception class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AdditiveBackwardCompatible,
        Self::SecurityFix,
        Self::DefectCorrection,
        Self::CoordinatedBreakingChange,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdditiveBackwardCompatible => "additive_backward_compatible",
            Self::SecurityFix => "security_fix",
            Self::DefectCorrection => "defect_correction",
            Self::CoordinatedBreakingChange => "coordinated_breaking_change",
        }
    }
}

/// Where a row's authoritative current version comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionSource {
    /// The row mirrors a governed schema-family in the schema registry; its
    /// current version is cross-checked against that registry.
    GovernedSchemaRegistry,
    /// The row pins a serialization surface that is not a governed-registry
    /// family; its current version is declared in the register itself.
    Declared,
}

impl VersionSource {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GovernedSchemaRegistry => "governed_schema_registry",
            Self::Declared => "declared",
        }
    }
}

/// One recorded exception that authorized a change to a frozen surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordedFreezeException {
    /// Class of the authorized change.
    pub exception_class: FreezeExceptionClass,
    /// Version the surface moved from.
    pub from_version: u32,
    /// Version the surface moved to.
    pub to_version: u32,
    /// Reviewable reason the exception was granted.
    pub rationale: String,
    /// Authority reference (exception packet id, ADR, or decision ref).
    pub authority_ref: String,
}

/// One governed interface-surface freeze row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceFreezeRow {
    /// Stable surface id. Governed rows reuse the schema-registry `schema_id`.
    pub schema_id: String,
    /// Human-readable title.
    pub title: String,
    /// Broad interface surface this row governs.
    pub surface_class: SurfaceClass,
    /// Where the authoritative current version comes from.
    pub version_source: VersionSource,
    /// Repo-relative schema file this surface is defined by.
    pub schema_ref: String,
    /// Freeze posture for this surface.
    pub freeze_state: FreezeState,
    /// Version the surface was frozen at.
    pub frozen_at_version: u32,
    /// Current version of the surface.
    pub current_version: u32,
    /// Exception classes permitted for this surface.
    pub allowed_exception_classes: Vec<FreezeExceptionClass>,
    /// Recorded exceptions that authorized observed changes.
    pub recorded_exceptions: Vec<RecordedFreezeException>,
    /// Owning team or role.
    pub owner_ref: String,
    /// Reviewable reason this surface carries this freeze state.
    pub rationale: String,
}

impl InterfaceFreezeRow {
    /// True when the surface's current version has moved past the frozen one.
    pub fn has_changed(&self) -> bool {
        self.current_version != self.frozen_at_version
    }

    /// True when a recorded exception, with an allowed class, authorizes the
    /// move to the current version.
    pub fn change_is_authorized(&self) -> bool {
        self.recorded_exceptions.iter().any(|exception| {
            exception.to_version == self.current_version
                && self
                    .allowed_exception_classes
                    .contains(&exception.exception_class)
        })
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceFreezeSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Rows whose freeze state is `open`.
    pub open_rows: usize,
    /// Rows whose freeze state is `soft_frozen`.
    pub soft_frozen_rows: usize,
    /// Rows whose freeze state is `hard_frozen`.
    pub hard_frozen_rows: usize,
    /// Rows whose version source is the governed schema registry.
    pub governed_schema_rows: usize,
    /// Rows whose version source is declared in the register.
    pub declared_rows: usize,
    /// Total recorded exceptions across all rows.
    pub recorded_exception_count: usize,
}

/// The typed interface-freeze register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceFreezeRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Governed schema-family registry this register cross-checks against.
    pub governed_schema_registry_ref: String,
    /// Closed surface-class vocabulary.
    pub surface_classes: Vec<SurfaceClass>,
    /// Closed freeze-state vocabulary.
    pub freeze_state_classes: Vec<FreezeState>,
    /// Closed exception-class vocabulary.
    pub exception_classes: Vec<FreezeExceptionClass>,
    /// Freeze rows.
    pub rows: Vec<InterfaceFreezeRow>,
    /// Summary counts.
    pub summary: InterfaceFreezeSummary,
}

impl InterfaceFreezeRegister {
    /// Returns the row registered for `schema_id`.
    pub fn row(&self, schema_id: &str) -> Option<&InterfaceFreezeRow> {
        self.rows.iter().find(|row| row.schema_id == schema_id)
    }

    /// Returns the hard-frozen rows.
    pub fn hard_frozen_rows(&self) -> Vec<&InterfaceFreezeRow> {
        self.rows
            .iter()
            .filter(|row| row.freeze_state == FreezeState::HardFrozen)
            .collect()
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> InterfaceFreezeSummary {
        InterfaceFreezeSummary {
            total_rows: self.rows.len(),
            open_rows: self.count_state(FreezeState::Open),
            soft_frozen_rows: self.count_state(FreezeState::SoftFrozen),
            hard_frozen_rows: self.count_state(FreezeState::HardFrozen),
            governed_schema_rows: self
                .rows
                .iter()
                .filter(|row| row.version_source == VersionSource::GovernedSchemaRegistry)
                .count(),
            declared_rows: self
                .rows
                .iter()
                .filter(|row| row.version_source == VersionSource::Declared)
                .count(),
            recorded_exception_count: self
                .rows
                .iter()
                .map(|row| row.recorded_exceptions.len())
                .sum(),
        }
    }

    fn count_state(&self, state: FreezeState) -> usize {
        self.rows
            .iter()
            .filter(|row| row.freeze_state == state)
            .count()
    }

    /// Validates the register against the governed schema registry, returning
    /// every violation found.
    pub fn validate_against(
        &self,
        schema_registry: &GovernedSchemaRegistry,
    ) -> Vec<InterfaceFreezeViolation> {
        let mut violations = Vec::new();

        if self.schema_version != INTERFACE_FREEZE_REGISTER_SCHEMA_VERSION {
            violations.push(InterfaceFreezeViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != INTERFACE_FREEZE_REGISTER_RECORD_KIND {
            violations.push(InterfaceFreezeViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.governed_schema_registry_ref != crate::schema_registry::GOVERNED_SCHEMA_REGISTRY_PATH
        {
            violations.push(InterfaceFreezeViolation::ClosedVocabularyMismatch {
                field: "governed_schema_registry_ref",
            });
        }
        if self.surface_classes != SurfaceClass::ALL.to_vec() {
            violations.push(InterfaceFreezeViolation::ClosedVocabularyMismatch {
                field: "surface_classes",
            });
        }
        if self.freeze_state_classes != FreezeState::ALL.to_vec() {
            violations.push(InterfaceFreezeViolation::ClosedVocabularyMismatch {
                field: "freeze_state_classes",
            });
        }
        if self.exception_classes != FreezeExceptionClass::ALL.to_vec() {
            violations.push(InterfaceFreezeViolation::ClosedVocabularyMismatch {
                field: "exception_classes",
            });
        }
        if self.rows.is_empty() {
            violations.push(InterfaceFreezeViolation::EmptyRegister);
        }

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            for (field, value) in [
                ("schema_id", &row.schema_id),
                ("title", &row.title),
                ("schema_ref", &row.schema_ref),
                ("owner_ref", &row.owner_ref),
                ("rationale", &row.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(InterfaceFreezeViolation::EmptyField {
                        schema_id: row.schema_id.clone(),
                        field_name: field,
                    });
                }
            }

            if !seen.insert(row.schema_id.clone()) {
                violations.push(InterfaceFreezeViolation::DuplicateSchemaId {
                    schema_id: row.schema_id.clone(),
                });
            }

            if row.frozen_at_version == 0 || row.current_version == 0 {
                violations.push(InterfaceFreezeViolation::EmptyField {
                    schema_id: row.schema_id.clone(),
                    field_name: "version",
                });
            }
            if row.current_version < row.frozen_at_version {
                violations.push(InterfaceFreezeViolation::VersionWentBackwards {
                    schema_id: row.schema_id.clone(),
                    frozen_at_version: row.frozen_at_version,
                    current_version: row.current_version,
                });
            }

            self.validate_exceptions(row, &mut violations);
            self.validate_against_registry(row, schema_registry, &mut violations);

            // Acceptance core: a hard-frozen surface may not move past its
            // frozen version without a recorded exception drawn from its
            // allowed classes.
            if row.freeze_state == FreezeState::HardFrozen
                && row.has_changed()
                && !row.change_is_authorized()
            {
                violations.push(InterfaceFreezeViolation::HardFreezeChangedWithoutException {
                    schema_id: row.schema_id.clone(),
                    frozen_at_version: row.frozen_at_version,
                    current_version: row.current_version,
                });
            }
        }

        // Acceptance core: every governed schema family must declare a freeze
        // state through a governed-source row.
        for governed in &schema_registry.rows {
            let covered = self.rows.iter().any(|row| {
                row.version_source == VersionSource::GovernedSchemaRegistry
                    && row.schema_id == governed.schema_id
            });
            if !covered {
                violations.push(InterfaceFreezeViolation::MissingFreezeState {
                    schema_id: governed.schema_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(InterfaceFreezeViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_exceptions(
        &self,
        row: &InterfaceFreezeRow,
        violations: &mut Vec<InterfaceFreezeViolation>,
    ) {
        for exception in &row.recorded_exceptions {
            if !row
                .allowed_exception_classes
                .contains(&exception.exception_class)
            {
                violations.push(InterfaceFreezeViolation::ExceptionClassNotAllowed {
                    schema_id: row.schema_id.clone(),
                    exception_class: exception.exception_class,
                });
            }
            if exception.from_version >= exception.to_version
                || exception.from_version < row.frozen_at_version
                || exception.to_version > row.current_version
            {
                violations.push(InterfaceFreezeViolation::ExceptionVersionRange {
                    schema_id: row.schema_id.clone(),
                });
            }
        }
    }

    fn validate_against_registry(
        &self,
        row: &InterfaceFreezeRow,
        schema_registry: &GovernedSchemaRegistry,
        violations: &mut Vec<InterfaceFreezeViolation>,
    ) {
        if row.version_source != VersionSource::GovernedSchemaRegistry {
            return;
        }
        match schema_registry.row(&row.schema_id) {
            None => violations.push(InterfaceFreezeViolation::UnknownGovernedSchema {
                schema_id: row.schema_id.clone(),
            }),
            Some(governed) => {
                if row.current_version != governed.schema_version {
                    violations.push(InterfaceFreezeViolation::RegistryVersionMismatch {
                        schema_id: row.schema_id.clone(),
                        register_version: row.current_version,
                        registry_version: governed.schema_version,
                    });
                }
                if row.schema_ref != governed.schema_ref {
                    violations.push(InterfaceFreezeViolation::RegistrySchemaRefMismatch {
                        schema_id: row.schema_id.clone(),
                    });
                }
            }
        }
    }
}

/// A validation violation for the interface-freeze register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceFreezeViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned reference is not the canonical value.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no rows.
    EmptyRegister,
    /// A schema id appears more than once.
    DuplicateSchemaId {
        /// Duplicate schema id.
        schema_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Row id.
        schema_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A current version is older than the frozen version.
    VersionWentBackwards {
        /// Row id.
        schema_id: String,
        /// Frozen version.
        frozen_at_version: u32,
        /// Current version.
        current_version: u32,
    },
    /// A governed-source row names a schema absent from the governed registry.
    UnknownGovernedSchema {
        /// Row id.
        schema_id: String,
    },
    /// A governed-source row's current version disagrees with the registry.
    RegistryVersionMismatch {
        /// Row id.
        schema_id: String,
        /// Version recorded in the freeze register.
        register_version: u32,
        /// Version recorded in the governed schema registry.
        registry_version: u32,
    },
    /// A governed-source row's schema reference disagrees with the registry.
    RegistrySchemaRefMismatch {
        /// Row id.
        schema_id: String,
    },
    /// A recorded exception uses a class not allowed for the row.
    ExceptionClassNotAllowed {
        /// Row id.
        schema_id: String,
        /// Offending exception class.
        exception_class: FreezeExceptionClass,
    },
    /// A recorded exception's version range is invalid for the row.
    ExceptionVersionRange {
        /// Row id.
        schema_id: String,
    },
    /// A hard-frozen surface changed without a recorded, allowed exception.
    HardFreezeChangedWithoutException {
        /// Row id.
        schema_id: String,
        /// Frozen version.
        frozen_at_version: u32,
        /// Current version.
        current_version: u32,
    },
    /// A governed schema family has no freeze state in the register.
    MissingFreezeState {
        /// Uncovered governed schema family id.
        schema_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for InterfaceFreezeViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::DuplicateSchemaId { schema_id } => {
                write!(f, "duplicate freeze row id {schema_id}")
            }
            Self::EmptyField {
                schema_id,
                field_name,
            } => write!(f, "freeze row {schema_id} has empty field {field_name}"),
            Self::VersionWentBackwards {
                schema_id,
                frozen_at_version,
                current_version,
            } => write!(
                f,
                "freeze row {schema_id} current version {current_version} is older than frozen version {frozen_at_version}"
            ),
            Self::UnknownGovernedSchema { schema_id } => write!(
                f,
                "freeze row {schema_id} names a schema absent from the governed registry"
            ),
            Self::RegistryVersionMismatch {
                schema_id,
                register_version,
                registry_version,
            } => write!(
                f,
                "freeze row {schema_id} records version {register_version} but the governed registry says {registry_version}"
            ),
            Self::RegistrySchemaRefMismatch { schema_id } => write!(
                f,
                "freeze row {schema_id} schema_ref disagrees with the governed registry"
            ),
            Self::ExceptionClassNotAllowed {
                schema_id,
                exception_class,
            } => write!(
                f,
                "freeze row {schema_id} records an exception of class {} which it does not allow",
                exception_class.as_str()
            ),
            Self::ExceptionVersionRange { schema_id } => write!(
                f,
                "freeze row {schema_id} has a recorded exception with an invalid version range"
            ),
            Self::HardFreezeChangedWithoutException {
                schema_id,
                frozen_at_version,
                current_version,
            } => write!(
                f,
                "hard-frozen row {schema_id} changed from version {frozen_at_version} to {current_version} without a recorded exception"
            ),
            Self::MissingFreezeState { schema_id } => write!(
                f,
                "governed schema family {schema_id} has no freeze state in the register"
            ),
            Self::SummaryMismatch => write!(f, "register summary counts disagree with the rows"),
        }
    }
}

impl Error for InterfaceFreezeViolation {}

/// Loads the embedded interface-freeze register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`InterfaceFreezeRegister`] — including when a row carries a freeze state,
/// surface class, exception class, or version source outside the closed
/// vocabularies.
pub fn current_interface_freeze_register() -> Result<InterfaceFreezeRegister, serde_json::Error> {
    serde_json::from_str(INTERFACE_FREEZE_REGISTER_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_registry::load_default_schema_registry;

    fn registry() -> GovernedSchemaRegistry {
        load_default_schema_registry().expect("schema registry loads")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let register = current_interface_freeze_register().expect("register parses");
        assert_eq!(
            register.schema_version,
            INTERFACE_FREEZE_REGISTER_SCHEMA_VERSION
        );
        assert_eq!(register.record_kind, INTERFACE_FREEZE_REGISTER_RECORD_KIND);
        assert_eq!(register.validate_against(&registry()), Vec::new());
        assert!(!register.rows.is_empty());
    }

    #[test]
    fn every_governed_family_has_a_freeze_state() {
        let register = current_interface_freeze_register().expect("register parses");
        let registry = registry();
        for governed in &registry.rows {
            let row = register
                .row(&governed.schema_id)
                .unwrap_or_else(|| panic!("no freeze row for {}", governed.schema_id));
            assert_eq!(row.version_source, VersionSource::GovernedSchemaRegistry);
            assert!(FreezeState::ALL.contains(&row.freeze_state), "{}", row.schema_id);
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let register = current_interface_freeze_register().expect("register parses");
        assert_eq!(register.summary, register.computed_summary());
        assert_eq!(
            register.summary.open_rows
                + register.summary.soft_frozen_rows
                + register.summary.hard_frozen_rows,
            register.rows.len()
        );
    }

    #[test]
    fn validate_flags_a_governed_family_without_a_freeze_state() {
        let mut register = current_interface_freeze_register().expect("register parses");
        let registry = registry();
        // Drop the first governed-source row: the governed family it covered now
        // has no declared freeze state and must be flagged.
        let index = register
            .rows
            .iter()
            .position(|row| row.version_source == VersionSource::GovernedSchemaRegistry)
            .expect("a governed-source row exists");
        let dropped = register.rows.remove(index);
        register.summary = register.computed_summary();
        assert!(register.validate_against(&registry).contains(
            &InterfaceFreezeViolation::MissingFreezeState {
                schema_id: dropped.schema_id,
            }
        ));
    }

    #[test]
    fn validate_flags_a_hard_frozen_change_without_an_exception() {
        let mut register = current_interface_freeze_register().expect("register parses");
        let registry = registry();
        // Bump a declared hard-frozen surface past its frozen version with no
        // recorded exception: a frozen contract changed silently.
        let row = register
            .rows
            .iter_mut()
            .find(|row| {
                row.freeze_state == FreezeState::HardFrozen
                    && row.version_source == VersionSource::Declared
            })
            .expect("a declared hard-frozen row exists");
        let schema_id = row.schema_id.clone();
        row.current_version = row.frozen_at_version + 1;
        register.summary = register.computed_summary();
        assert!(register.validate_against(&registry).contains(
            &InterfaceFreezeViolation::HardFreezeChangedWithoutException {
                schema_id,
                frozen_at_version: 1,
                current_version: 2,
            }
        ));
    }

    #[test]
    fn a_recorded_exception_authorizes_a_hard_frozen_change() {
        let mut register = current_interface_freeze_register().expect("register parses");
        let registry = registry();
        let row = register
            .rows
            .iter_mut()
            .find(|row| {
                row.freeze_state == FreezeState::HardFrozen
                    && row.version_source == VersionSource::Declared
            })
            .expect("a declared hard-frozen row exists");
        let allowed = row.allowed_exception_classes[0];
        row.current_version = row.frozen_at_version + 1;
        row.recorded_exceptions.push(RecordedFreezeException {
            exception_class: allowed,
            from_version: row.frozen_at_version,
            to_version: row.current_version,
            rationale: "Coordinated change for the test.".to_owned(),
            authority_ref: "decision.test".to_owned(),
        });
        register.summary = register.computed_summary();
        assert_eq!(register.validate_against(&registry), Vec::new());
    }

    #[test]
    fn validate_flags_a_disallowed_exception_class() {
        let mut register = current_interface_freeze_register().expect("register parses");
        let registry = registry();
        let row = &mut register.rows[0];
        // Choose an exception class the row does not allow.
        let disallowed = FreezeExceptionClass::ALL
            .into_iter()
            .find(|class| !row.allowed_exception_classes.contains(class))
            .expect("a disallowed class exists for the first row");
        let schema_id = row.schema_id.clone();
        row.current_version = row.frozen_at_version + 1;
        row.recorded_exceptions.push(RecordedFreezeException {
            exception_class: disallowed,
            from_version: row.frozen_at_version,
            to_version: row.current_version,
            rationale: "Disallowed class for the test.".to_owned(),
            authority_ref: "decision.test".to_owned(),
        });
        register.summary = register.computed_summary();
        assert!(register.validate_against(&registry).contains(
            &InterfaceFreezeViolation::ExceptionClassNotAllowed {
                schema_id,
                exception_class: disallowed,
            }
        ));
    }
}
