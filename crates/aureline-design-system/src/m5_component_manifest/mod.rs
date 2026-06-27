//! Versioned, machine-readable component-contract manifests for the launch-critical M5
//! component families.
//!
//! Where [`crate::m5_design_system_contract`] freezes the *governance matrix* — which design-system
//! objects exist and whether each claimed surface maps them — and
//! [`crate::m5_foundation_package`] ships the versioned *foundations* (tokens, density, motion,
//! contrast, and the controlled state vocabulary), this module ships the durable *component
//! contracts* the M5 depth surfaces reuse: a versioned [`M5ComponentManifestPackage`] carrying one
//! [`M5ComponentManifest`] per launch-critical [component kind](M5ComponentKind) — placeholder
//! cards, state blocks, review sheets, job rows, boundary bars, form controls, and dense
//! collection primitives.
//!
//! Each manifest is the single, cite-able contract engineering, QA, docs, and the extension SDK
//! reuse instead of reading shell code or screenshots. It records:
//!
//! - **anatomy** — the named [parts](M5AnatomyPart) of the component and their roles, marking the
//!   parts that are required.
//! - **states** — the [mandatory and optional](M5ComponentStates) controlled-state families the
//!   component renders, drawn from the same [`CanonicalStateClass`] vocabulary the foundation
//!   package publishes, so mandatory vs. optional is explicit rather than implied by a screenshot.
//! - **labels** — the governed [label](M5ComponentLabel) message ids the component announces.
//! - **commands** — the [commands](M5ComponentCommand) the component offers, each with its label
//!   message id and key chord.
//! - **keyboard model** — the [key chords](M5KeyBinding) and the actions they trigger.
//! - **accessibility** — the [role, screen-reader label rule, focus-order rule, and
//!   notes](M5AccessibilityContract).
//! - **token dependencies** — the foundation token references the component renders from.
//! - **extension guidance** — the [consumption rules](M5ExtensionGuidance) an extension author
//!   reads, so extensions point at this manifest instead of copying shell behavior.
//!
//! Every manifest carries versioned [lifecycle and owner metadata](M5ComponentLifecycle) — an
//! owner role, a lifecycle state, a monotonic `manifest_version`, and the package version it was
//! introduced in — so design QA, support exports, and release packets can all point at the same
//! contract and detect drift.
//!
//! The records are metadata-only truth packets: they carry semantic token *references* and message
//! *ids*, never raw color values, credential bodies, or provider payloads.
//!
//! - Schema:
//!   [`schemas/design-system/m5-component-manifest.schema.json`](../../../../../schemas/design-system/m5-component-manifest.schema.json)
//! - Doc:
//!   [`docs/design-system/m5-component-manifest.md`](../../../../../docs/design-system/m5-component-manifest.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_component_manifest_package, M5_COMPONENT_MANIFEST_PACKAGE_ID,
    M5_COMPONENT_MANIFEST_PACKAGE_VERSION,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::CanonicalStateClass;

/// Record-kind tag carried by [`M5ComponentManifestPackage`].
pub const M5_COMPONENT_MANIFEST_PACKAGE_RECORD_KIND: &str =
    "m5_design_system_component_manifest_package";

/// Record-kind tag carried by [`M5ComponentManifestReleasePacket`].
pub const M5_COMPONENT_MANIFEST_RELEASE_RECORD_KIND: &str =
    "m5_design_system_component_manifest_release";

/// Schema version shared by the component-manifest records.
pub const M5_COMPONENT_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the component-manifest boundary schema.
pub const M5_COMPONENT_MANIFEST_SCHEMA_REF: &str =
    "schemas/design-system/m5-component-manifest.schema.json";

/// Repo-relative path of the component-manifest contract doc.
pub const M5_COMPONENT_MANIFEST_DOC_REF: &str = "docs/design-system/m5-component-manifest.md";

/// Repo-relative path of the release-grade component-manifest proof packet — the proof lane that
/// blocks drift for the package.
pub const M5_COMPONENT_MANIFEST_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/component-manifest-release.json";

/// Release packet that keeps the component manifests current (shared with the foundation package
/// and contract matrix).
pub const M5_COMPONENT_MANIFEST_RELEASE_PACKET_REF: &str =
    "evidence:m5-design-system-release-packet";

/// Repo-relative directory of the checked-in manifest fixtures.
pub const M5_COMPONENT_MANIFEST_DIR: &str = "fixtures/ui/m5-component-gallery/";

/// Repo-relative extension-SDK guidance an extension author reads to consume a manifest.
pub const M5_COMPONENT_EXTENSION_GUIDANCE_REF: &str =
    "docs/sdk/extension-ui-component-contracts.md";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_COMPONENT_MESSAGE_ID_PREFIX: &str = "design_system_component.";

/// One launch-critical M5 component family the package publishes a manifest for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentKind {
    /// An empty-but-ready surface that offers a useful next route.
    PlaceholderCard,
    /// A block that renders a controlled state with a title, detail, and recovery action.
    StateBlock,
    /// A review surface that stages a decision and records its outcome.
    ReviewSheet,
    /// A row in a dense job / activity collection.
    JobRow,
    /// An embedded-surface boundary indicator that names route, trust, and capability.
    BoundaryBar,
    /// A labelled input control with validation and submission semantics.
    FormControl,
    /// A dense, virtualizable collection primitive (tree / table / log / list).
    DenseCollection,
}

impl M5ComponentKind {
    /// Every component kind, in declaration order. The package must publish one manifest per kind.
    pub const ALL: [Self; 7] = [
        Self::PlaceholderCard,
        Self::StateBlock,
        Self::ReviewSheet,
        Self::JobRow,
        Self::BoundaryBar,
        Self::FormControl,
        Self::DenseCollection,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaceholderCard => "placeholder_card",
            Self::StateBlock => "state_block",
            Self::ReviewSheet => "review_sheet",
            Self::JobRow => "job_row",
            Self::BoundaryBar => "boundary_bar",
            Self::FormControl => "form_control",
            Self::DenseCollection => "dense_collection",
        }
    }
}

/// Lifecycle state of a published component manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleState {
    /// In active design; shape may still change.
    Experimental,
    /// Shape is settling and consumable, ahead of a stable commitment.
    Preview,
    /// Committed contract; changes are versioned and reviewed.
    Stable,
    /// On a removal path; consumers should migrate.
    Deprecated,
}

impl M5LifecycleState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Experimental => "experimental",
            Self::Preview => "preview",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Versioned lifecycle and owner metadata for one component manifest, so design QA, support
/// exports, and release packets point at the same contract revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentLifecycle {
    /// Owner role accountable for the manifest.
    pub owner_role: String,
    /// Lifecycle state of the manifest.
    pub lifecycle_state: M5LifecycleState,
    /// Monotonic manifest version; bumps when this manifest's contract changes.
    pub manifest_version: u32,
    /// Package version (semver) the manifest was introduced in.
    pub introduced_in_package_version: String,
}

/// One named part of a component's anatomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnatomyPart {
    /// Stable part id, unique within the manifest.
    pub part_id: String,
    /// The part's role.
    pub role: String,
    /// True when the part is always present.
    pub required: bool,
}

impl M5AnatomyPart {
    /// Builds a required anatomy part.
    pub fn required(part_id: &str, role: &str) -> Self {
        Self {
            part_id: part_id.to_owned(),
            role: role.to_owned(),
            required: true,
        }
    }

    /// Builds an optional anatomy part.
    pub fn optional(part_id: &str, role: &str) -> Self {
        Self {
            part_id: part_id.to_owned(),
            role: role.to_owned(),
            required: false,
        }
    }
}

/// The controlled-state families a component renders, split into mandatory and optional. Together
/// they classify every [`CanonicalStateClass`], so mandatory vs. optional is explicit rather than
/// implied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentStates {
    /// States the component MUST render.
    pub mandatory: Vec<CanonicalStateClass>,
    /// States the component MAY render.
    pub optional: Vec<CanonicalStateClass>,
}

impl M5ComponentStates {
    /// Builds a state set from the mandatory and optional slices.
    pub fn new(mandatory: &[CanonicalStateClass], optional: &[CanonicalStateClass]) -> Self {
        Self {
            mandatory: mandatory.to_vec(),
            optional: optional.to_vec(),
        }
    }
}

/// One governed label a component announces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentLabel {
    /// Stable label id, unique within the manifest.
    pub label_id: String,
    /// Governed message id; prefixed [`M5_COMPONENT_MESSAGE_ID_PREFIX`].
    pub message_id: String,
    /// What the label is for.
    pub purpose: String,
}

impl M5ComponentLabel {
    /// Builds a label whose message id is derived from the component id and label id.
    fn new(component_id: &str, label_id: &str, purpose: &str) -> Self {
        Self {
            label_id: label_id.to_owned(),
            message_id: format!(
                "{}{}.label.{}",
                M5_COMPONENT_MESSAGE_ID_PREFIX, component_id, label_id
            ),
            purpose: purpose.to_owned(),
        }
    }
}

/// One command a component offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentCommand {
    /// Stable command id, unique within the manifest.
    pub command_id: String,
    /// Governed label message id; prefixed [`M5_COMPONENT_MESSAGE_ID_PREFIX`].
    pub label_message_id: String,
    /// Default key chord that invokes the command.
    pub keys: String,
}

impl M5ComponentCommand {
    /// Builds a command whose label message id is derived from the component id and command id.
    fn new(component_id: &str, command_id: &str, keys: &str) -> Self {
        Self {
            command_id: command_id.to_owned(),
            label_message_id: format!(
                "{}{}.command.{}",
                M5_COMPONENT_MESSAGE_ID_PREFIX, component_id, command_id
            ),
            keys: keys.to_owned(),
        }
    }
}

/// One key chord and the action it triggers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5KeyBinding {
    /// The key chord.
    pub keys: String,
    /// The action the chord triggers.
    pub action: String,
}

impl M5KeyBinding {
    /// Builds a key binding.
    pub fn new(keys: &str, action: &str) -> Self {
        Self {
            keys: keys.to_owned(),
            action: action.to_owned(),
        }
    }
}

/// A component's accessibility contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AccessibilityContract {
    /// Assistive-technology role the component exposes.
    pub role: String,
    /// Rule for the screen-reader label the component announces.
    pub screen_reader_label_rule: String,
    /// Rule for the component's focus order.
    pub focus_order_rule: String,
    /// Additional accessibility obligations engineering and QA must honor.
    pub notes: Vec<String>,
}

/// Extension-author consumption rules for a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExtensionGuidance {
    /// Repo-relative extension-SDK guidance ref.
    pub guidance_ref: String,
    /// The rules an extension author must honor to extend or reuse the component.
    pub consumption_rules: Vec<String>,
}

/// One launch-critical component's contract manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentManifest {
    /// The governed component kind.
    pub component_kind: M5ComponentKind,
    /// Stable component id, unique within the package.
    pub component_id: String,
    /// Human-readable component name.
    pub display_name: String,
    /// Versioned lifecycle and owner metadata.
    pub lifecycle: M5ComponentLifecycle,
    /// The named parts of the component and their roles.
    pub anatomy: Vec<M5AnatomyPart>,
    /// The mandatory and optional controlled-state families the component renders.
    pub states: M5ComponentStates,
    /// The governed labels the component announces.
    pub labels: Vec<M5ComponentLabel>,
    /// The commands the component offers.
    pub commands: Vec<M5ComponentCommand>,
    /// The component's keyboard model.
    pub keyboard: Vec<M5KeyBinding>,
    /// The component's accessibility contract.
    pub accessibility: M5AccessibilityContract,
    /// The foundation token references the component renders from.
    pub token_dependencies: Vec<String>,
    /// Extension-author consumption rules.
    pub extension_guidance: M5ExtensionGuidance,
    /// Stable summary message id; prefixed [`M5_COMPONENT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5ComponentManifest {
    /// The anatomy parts that are always present.
    pub fn required_parts(&self) -> Vec<&M5AnatomyPart> {
        self.anatomy.iter().filter(|p| p.required).collect()
    }
}

/// A versioned, machine-readable package of launch-critical component manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentManifestPackage {
    /// Record kind; must equal [`M5_COMPONENT_MANIFEST_PACKAGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPONENT_MANIFEST_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable package id.
    pub package_id: String,
    /// Package version (semver `MAJOR.MINOR.PATCH`).
    pub package_version: String,
    /// Owner role accountable for the package.
    pub owner_role: String,
    /// The governed component manifests (one per [`M5ComponentKind`]).
    pub manifests: Vec<M5ComponentManifest>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Repo-relative release packet that keeps the package current.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable summary message id; prefixed [`M5_COMPONENT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5ComponentManifestPackage {
    /// Finds the manifest for a component kind.
    pub fn manifest(&self, kind: M5ComponentKind) -> Option<&M5ComponentManifest> {
        self.manifests.iter().find(|m| m.component_kind == kind)
    }

    /// Total manifest count.
    pub fn total_manifests(&self) -> usize {
        self.manifests.len()
    }

    /// Validates the package invariants, returning the violations (empty when valid).
    pub fn validate(&self) -> Vec<M5ComponentManifestViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPONENT_MANIFEST_PACKAGE_RECORD_KIND {
            violations.push(M5ComponentManifestViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPONENT_MANIFEST_SCHEMA_VERSION {
            violations.push(M5ComponentManifestViolation::WrongSchemaVersion);
        }
        if self.package_id.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.proof_lane_ref.trim().is_empty()
            || self.release_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ComponentManifestViolation::MissingIdentity);
        }
        if !is_semver(&self.package_version) {
            violations.push(M5ComponentManifestViolation::BadPackageVersion);
        }
        if !self
            .summary_message_id
            .starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX)
        {
            violations.push(M5ComponentManifestViolation::MessageIdPrefixMissing);
        }

        for required in [
            M5_COMPONENT_MANIFEST_SCHEMA_REF,
            M5_COMPONENT_MANIFEST_DOC_REF,
            M5_COMPONENT_MANIFEST_PROOF_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.push(M5ComponentManifestViolation::MissingSourceContracts);
                break;
            }
        }

        validate_manifest_set(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 component manifest package serializes"),
        ) {
            violations.push(M5ComponentManifestViolation::RawBoundaryMaterialInExport);
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
        serde_json::to_string_pretty(self).expect("m5 component manifest package serializes")
    }

    /// Imports a package from JSON. The caller validates the returned package with
    /// [`Self::validate`].
    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    /// Projects the release-packet inclusion: per-manifest lifecycle and shape summaries, so a
    /// release record names the contract revision QA and support exports cite.
    pub fn release_packet(&self) -> M5ComponentManifestReleasePacket {
        let manifest_summaries: Vec<M5ComponentManifestSummary> = self
            .manifests
            .iter()
            .map(|m| M5ComponentManifestSummary {
                component_kind: m.component_kind,
                component_id: m.component_id.clone(),
                lifecycle_state: m.lifecycle.lifecycle_state,
                manifest_version: m.lifecycle.manifest_version,
                anatomy_part_count: m.anatomy.len() as u32,
                mandatory_state_count: m.states.mandatory.len() as u32,
                command_count: m.commands.len() as u32,
                keyboard_binding_count: m.keyboard.len() as u32,
                token_dependency_count: m.token_dependencies.len() as u32,
            })
            .collect();

        M5ComponentManifestReleasePacket {
            record_kind: M5_COMPONENT_MANIFEST_RELEASE_RECORD_KIND.to_owned(),
            schema_version: M5_COMPONENT_MANIFEST_SCHEMA_VERSION,
            package_id: self.package_id.clone(),
            package_version: self.package_version.clone(),
            total_manifests: self.total_manifests() as u32,
            manifest_summaries,
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.release",
                M5_COMPONENT_MESSAGE_ID_PREFIX, self.package_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Reads and validates the checked-in canonical manifest package fixture.
pub fn current_stable_m5_component_manifest_package(
) -> Result<M5ComponentManifestPackage, M5ComponentManifestArtifactError> {
    let package: M5ComponentManifestPackage = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-component-gallery/component-manifest-package.json"
    )))
    .map_err(M5ComponentManifestArtifactError::Parse)?;
    let violations = package.validate();
    if violations.is_empty() {
        Ok(package)
    } else {
        Err(M5ComponentManifestArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading a checked-in manifest-package export.
#[derive(Debug)]
pub enum M5ComponentManifestArtifactError {
    /// The export failed to parse.
    Parse(serde_json::Error),
    /// The export failed validation.
    Validation(Vec<M5ComponentManifestViolation>),
}

impl fmt::Display for M5ComponentManifestArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(
                    formatter,
                    "m5 component manifest package parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 component manifest package failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5ComponentManifestArtifactError {}

/// Validation failures emitted by [`M5ComponentManifestPackage::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ComponentManifestViolation {
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
    /// A governed component kind has no published manifest.
    RequiredComponentKindMissing,
    /// Two manifests share a kind.
    DuplicateComponentKind,
    /// Two manifests share a component id.
    DuplicateComponentId,
    /// A manifest is missing an identity field (component id, display name, or summary id).
    ManifestIncomplete,
    /// A manifest's lifecycle metadata is incomplete (empty owner, zero version, or bad introduced
    /// version).
    LifecycleIncomplete,
    /// A manifest's anatomy is empty, has duplicate part ids, or declares no required part.
    AnatomyIncomplete,
    /// A manifest's mandatory / optional states do not classify exactly the canonical state set, or
    /// overlap.
    StatesIncomplete,
    /// A manifest's labels are empty, have duplicate ids, or carry an unprefixed message id.
    LabelsIncomplete,
    /// A manifest's commands are empty, have duplicate ids, carry an unprefixed message id, or have
    /// an empty key chord.
    CommandsIncomplete,
    /// A manifest's keyboard model is empty or has an incomplete binding.
    KeyboardIncomplete,
    /// A manifest's accessibility contract is incomplete (empty role / rule, or no notes).
    AccessibilityIncomplete,
    /// A manifest declares no token dependencies, or one is empty.
    TokenDependenciesIncomplete,
    /// A manifest's extension guidance is incomplete (empty ref or no consumption rules).
    ExtensionGuidanceIncomplete,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ComponentManifestViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BadPackageVersion => "bad_package_version",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentKindMissing => "required_component_kind_missing",
            Self::DuplicateComponentKind => "duplicate_component_kind",
            Self::DuplicateComponentId => "duplicate_component_id",
            Self::ManifestIncomplete => "manifest_incomplete",
            Self::LifecycleIncomplete => "lifecycle_incomplete",
            Self::AnatomyIncomplete => "anatomy_incomplete",
            Self::StatesIncomplete => "states_incomplete",
            Self::LabelsIncomplete => "labels_incomplete",
            Self::CommandsIncomplete => "commands_incomplete",
            Self::KeyboardIncomplete => "keyboard_incomplete",
            Self::AccessibilityIncomplete => "accessibility_incomplete",
            Self::TokenDependenciesIncomplete => "token_dependencies_incomplete",
            Self::ExtensionGuidanceIncomplete => "extension_guidance_incomplete",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Release-packet records.
// ---------------------------------------------------------------------------

/// Release-packet projection of a component-manifest package: one lifecycle and shape summary per
/// manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentManifestReleasePacket {
    /// Record kind; must equal [`M5_COMPONENT_MANIFEST_RELEASE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The package id this release record projects.
    pub package_id: String,
    /// The package version.
    pub package_version: String,
    /// Total manifests across the package.
    pub total_manifests: u32,
    /// Per-manifest lifecycle and shape summaries, in package order.
    pub manifest_summaries: Vec<M5ComponentManifestSummary>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_COMPONENT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5ComponentManifestReleasePacket {
    /// Deterministic export-safe JSON for the release packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 component manifest release packet serializes")
    }
}

/// One manifest's lifecycle and shape summary inside a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentManifestSummary {
    /// The governed component kind.
    pub component_kind: M5ComponentKind,
    /// The component id.
    pub component_id: String,
    /// The manifest's lifecycle state.
    pub lifecycle_state: M5LifecycleState,
    /// The manifest version.
    pub manifest_version: u32,
    /// Anatomy part count.
    pub anatomy_part_count: u32,
    /// Mandatory state count.
    pub mandatory_state_count: u32,
    /// Command count.
    pub command_count: u32,
    /// Keyboard binding count.
    pub keyboard_binding_count: u32,
    /// Token dependency count.
    pub token_dependency_count: u32,
}

// ---------------------------------------------------------------------------
// Validation helpers.
// ---------------------------------------------------------------------------

fn validate_manifest_set(
    package: &M5ComponentManifestPackage,
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    let present: BTreeSet<M5ComponentKind> =
        package.manifests.iter().map(|m| m.component_kind).collect();
    for required in M5ComponentKind::ALL {
        if !present.contains(&required) {
            violations.push(M5ComponentManifestViolation::RequiredComponentKindMissing);
            break;
        }
    }
    if present.len() != package.manifests.len() {
        violations.push(M5ComponentManifestViolation::DuplicateComponentKind);
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for manifest in &package.manifests {
        if !seen_ids.insert(manifest.component_id.as_str()) {
            violations.push(M5ComponentManifestViolation::DuplicateComponentId);
        }
        validate_manifest(manifest, violations);
    }
}

fn validate_manifest(
    manifest: &M5ComponentManifest,
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    if manifest.component_id.trim().is_empty()
        || manifest.display_name.trim().is_empty()
        || manifest.summary_message_id.trim().is_empty()
    {
        violations.push(M5ComponentManifestViolation::ManifestIncomplete);
    }
    if !manifest
        .summary_message_id
        .starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX)
    {
        violations.push(M5ComponentManifestViolation::MessageIdPrefixMissing);
    }

    validate_lifecycle(&manifest.lifecycle, violations);
    validate_anatomy(&manifest.anatomy, violations);
    validate_states(&manifest.states, violations);
    validate_labels(&manifest.labels, violations);
    validate_commands(&manifest.commands, violations);
    validate_keyboard(&manifest.keyboard, violations);
    validate_accessibility(&manifest.accessibility, violations);
    validate_token_dependencies(&manifest.token_dependencies, violations);
    validate_extension_guidance(&manifest.extension_guidance, violations);
}

fn validate_lifecycle(
    lifecycle: &M5ComponentLifecycle,
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    if lifecycle.owner_role.trim().is_empty()
        || lifecycle.manifest_version == 0
        || !is_semver(&lifecycle.introduced_in_package_version)
    {
        violations.push(M5ComponentManifestViolation::LifecycleIncomplete);
    }
}

fn validate_anatomy(anatomy: &[M5AnatomyPart], violations: &mut Vec<M5ComponentManifestViolation>) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut duplicate = false;
    let mut incomplete = anatomy.is_empty();
    for part in anatomy {
        if !seen.insert(part.part_id.as_str()) {
            duplicate = true;
        }
        if part.part_id.trim().is_empty() || part.role.trim().is_empty() {
            incomplete = true;
        }
    }
    if incomplete || duplicate || !anatomy.iter().any(|p| p.required) {
        violations.push(M5ComponentManifestViolation::AnatomyIncomplete);
    }
}

fn validate_states(states: &M5ComponentStates, violations: &mut Vec<M5ComponentManifestViolation>) {
    let mandatory: BTreeSet<CanonicalStateClass> = states.mandatory.iter().copied().collect();
    let optional: BTreeSet<CanonicalStateClass> = states.optional.iter().copied().collect();
    let canonical: BTreeSet<CanonicalStateClass> =
        CanonicalStateClass::required().iter().copied().collect();

    let no_duplicates =
        mandatory.len() == states.mandatory.len() && optional.len() == states.optional.len();
    let disjoint = mandatory.is_disjoint(&optional);
    let union: BTreeSet<CanonicalStateClass> = mandatory.union(&optional).copied().collect();

    if states.mandatory.is_empty() || !no_duplicates || !disjoint || union != canonical {
        violations.push(M5ComponentManifestViolation::StatesIncomplete);
    }
}

fn validate_labels(
    labels: &[M5ComponentLabel],
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut bad = labels.is_empty();
    for label in labels {
        if !seen.insert(label.label_id.as_str()) {
            bad = true;
        }
        if label.label_id.trim().is_empty()
            || label.purpose.trim().is_empty()
            || !label.message_id.starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX)
        {
            bad = true;
        }
    }
    if bad {
        violations.push(M5ComponentManifestViolation::LabelsIncomplete);
    }
}

fn validate_commands(
    commands: &[M5ComponentCommand],
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut bad = commands.is_empty();
    for command in commands {
        if !seen.insert(command.command_id.as_str()) {
            bad = true;
        }
        if command.command_id.trim().is_empty()
            || command.keys.trim().is_empty()
            || !command
                .label_message_id
                .starts_with(M5_COMPONENT_MESSAGE_ID_PREFIX)
        {
            bad = true;
        }
    }
    if bad {
        violations.push(M5ComponentManifestViolation::CommandsIncomplete);
    }
}

fn validate_keyboard(
    keyboard: &[M5KeyBinding],
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    let bad = keyboard.is_empty()
        || keyboard
            .iter()
            .any(|b| b.keys.trim().is_empty() || b.action.trim().is_empty());
    if bad {
        violations.push(M5ComponentManifestViolation::KeyboardIncomplete);
    }
}

fn validate_accessibility(
    accessibility: &M5AccessibilityContract,
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    if accessibility.role.trim().is_empty()
        || accessibility.screen_reader_label_rule.trim().is_empty()
        || accessibility.focus_order_rule.trim().is_empty()
        || accessibility.notes.is_empty()
        || accessibility.notes.iter().any(|n| n.trim().is_empty())
    {
        violations.push(M5ComponentManifestViolation::AccessibilityIncomplete);
    }
}

fn validate_token_dependencies(
    tokens: &[String],
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    if tokens.is_empty() || tokens.iter().any(|t| t.trim().is_empty()) {
        violations.push(M5ComponentManifestViolation::TokenDependenciesIncomplete);
    }
}

fn validate_extension_guidance(
    guidance: &M5ExtensionGuidance,
    violations: &mut Vec<M5ComponentManifestViolation>,
) {
    if guidance.guidance_ref.trim().is_empty()
        || guidance.consumption_rules.is_empty()
        || guidance
            .consumption_rules
            .iter()
            .any(|r| r.trim().is_empty())
    {
        violations.push(M5ComponentManifestViolation::ExtensionGuidanceIncomplete);
    }
}

/// True when `value` is a `MAJOR.MINOR.PATCH` numeric semver.
fn is_semver(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material (credential bodies,
/// raw provider payloads). Component manifests are metadata-only by construction; this is a
/// defense-in-depth scan over the serialized export.
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
