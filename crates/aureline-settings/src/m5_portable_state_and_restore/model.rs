//! Governed portable-state export/import and restore-provenance model.
//!
//! This module is the settings-lane contract for the portability of M5-owned
//! artifacts. It does not implement an export engine, a transport, or a restore
//! mutator; it defines the canonical record those surfaces must emit so a
//! portable-state package, a restore preview, a support export, or a docs/help
//! surface explains the same thing: which M5 artifact classes are portable,
//! redacted, machine-local, or restore-only; what schema-migration label a
//! restored card honestly carries; and which live dependency, extension, or
//! remote target is missing and therefore shown as a visible placeholder rather
//! than silently dropped.
//!
//! The gate is fail-closed. A restore card whose supporting dependency, runtime,
//! or schema row is missing or downgraded can never claim [`MigrationLabel::Exact`]
//! fidelity, and secret material or live authority handles can never be carried
//! as [`PortabilityDisposition::Portable`]. Both are build-time invariants, so a
//! dishonest package cannot be constructed in the first place.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for portable-state restore certification records.
pub const M5_PORTABLE_STATE_RESTORE_RECORD_KIND: &str =
    "m5_portable_state_restore_certification_record";

/// Schema version for [`M5PortableStateRestoreCertification`] records.
pub const M5_PORTABLE_STATE_RESTORE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by desktop, CLI, support, export, and docs.
pub const M5_PORTABLE_STATE_RESTORE_SHARED_CONTRACT_REF: &str =
    "settings:m5_portable_state_and_restore:v1";

const MAX_REF_CHARS: usize = 240;
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Returns true when `reference` is a non-empty canonical object ref.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    !class.is_empty() && !ident.is_empty()
}

/// An M5-owned artifact class that a portable-state package may carry.
///
/// These are the durable artifact classes new M5 desktop, marketplace, companion,
/// sync, and managed surfaces produce. Each one carries an explicit
/// [`PortabilityDisposition`] so an export never silently mixes a portable
/// document with a machine-local cache or a live token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableArtifactClass {
    /// Selected scalar and structured settings chosen for the package.
    SelectedSettings,
    /// Profile definitions and profile-scoped overlays.
    Profiles,
    /// Workflow or bundle manifests describing how a workspace is assembled.
    Manifests,
    /// Bundle and extension selections (by reference, not by payload).
    BundleSelections,
    /// Docs packs and help content packaged for portability.
    DocsPacks,
    /// Evidence references such as audit, certification, or proof pointers.
    EvidenceReferences,
}

impl PortableArtifactClass {
    /// Returns the canonical token for this artifact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedSettings => "selected_settings",
            Self::Profiles => "profiles",
            Self::Manifests => "manifests",
            Self::BundleSelections => "bundle_selections",
            Self::DocsPacks => "docs_packs",
            Self::EvidenceReferences => "evidence_references",
        }
    }

    /// Every artifact class the contract requires a package to classify.
    pub const REQUIRED: [Self; 6] = [
        Self::SelectedSettings,
        Self::Profiles,
        Self::Manifests,
        Self::BundleSelections,
        Self::DocsPacks,
        Self::EvidenceReferences,
    ];
}

/// What a portable-state package does with one artifact class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityDisposition {
    /// Carried in full; the class round-trips through export/import.
    Portable,
    /// Carried as reference or metadata only; sensitive bodies are stripped.
    Redacted,
    /// Re-derivable on import but not exported as a frozen body.
    RestoreOnly,
    /// Never serialized into the package; remains on the originating machine.
    MachineLocal,
}

impl PortabilityDisposition {
    /// Returns the canonical token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::Redacted => "redacted",
            Self::RestoreOnly => "restore_only",
            Self::MachineLocal => "machine_local",
        }
    }

    /// Returns true when the class leaves the originating machine in any form.
    pub const fn crosses_machine_boundary(self) -> bool {
        matches!(self, Self::Portable | Self::Redacted)
    }
}

/// Why an artifact class is redacted or held machine-local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExclusionReason {
    /// Raw tokens, passkeys, private keys, or similar secret material.
    SecretMaterial,
    /// Live authority handles such as tickets, sessions, or open sockets.
    LiveAuthorityHandle,
    /// Machine-unique trust anchors that cannot be transplanted.
    MachineUniqueTrustAnchor,
    /// Volatile machine-local state such as caches or indexes.
    VolatileMachineState,
}

impl ExclusionReason {
    /// Returns the canonical token for this exclusion reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecretMaterial => "secret_material",
            Self::LiveAuthorityHandle => "live_authority_handle",
            Self::MachineUniqueTrustAnchor => "machine_unique_trust_anchor",
            Self::VolatileMachineState => "volatile_machine_state",
        }
    }

    /// Returns true when raw material under this reason must never be serialized.
    pub const fn forbids_serialization(self) -> bool {
        matches!(
            self,
            Self::SecretMaterial | Self::LiveAuthorityHandle | Self::MachineUniqueTrustAnchor
        )
    }
}

/// Schema-migration and restore-provenance label for a restored artifact.
///
/// Labels are ordered by fidelity from [`Self::Exact`] (best) to
/// [`Self::EvidenceOnly`] (weakest). A package can never label a card stronger
/// than what its dependencies and schema actually back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationLabel {
    /// Same schema, all dependencies present; the artifact restores exactly.
    Exact,
    /// A forward/backward compatible schema migration was applied.
    Compatible,
    /// Only the layout or structure is restored; some values could not map.
    LayoutOnly,
    /// Content is recovered as editable drafts, not authoritative state.
    RecoveredDrafts,
    /// Only evidence/reference pointers are restored, not live content.
    EvidenceOnly,
}

impl MigrationLabel {
    /// Returns the canonical token for this label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// Fidelity rank where `0` is exact and higher values are weaker.
    pub const fn fidelity_rank(self) -> u8 {
        match self {
            Self::Exact => 0,
            Self::Compatible => 1,
            Self::LayoutOnly => 2,
            Self::RecoveredDrafts => 3,
            Self::EvidenceOnly => 4,
        }
    }

    /// Returns true when this label claims exact, lossless restore.
    pub const fn implies_exact_fidelity(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Every migration label, strongest fidelity first.
    pub const ALL: [Self; 5] = [
        Self::Exact,
        Self::Compatible,
        Self::LayoutOnly,
        Self::RecoveredDrafts,
        Self::EvidenceOnly,
    ];
}

/// A dependency that is unavailable when an M5 package is imported or reopened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingDependencyKind {
    /// A required extension or bundle is not installed.
    MissingExtension,
    /// A referenced remote target (repo, sync endpoint, service) is unreachable.
    MissingRemoteTarget,
    /// The importing client does not support the referenced capability.
    UnsupportedClient,
}

impl MissingDependencyKind {
    /// Returns the canonical token for this dependency kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingExtension => "missing_extension",
            Self::MissingRemoteTarget => "missing_remote_target",
            Self::UnsupportedClient => "unsupported_client",
        }
    }
}

/// Source surface that must render the same package and restore-provenance truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Desktop settings export/import and restore UI.
    DesktopSettings,
    /// CLI or headless inspect/export command.
    CliInspect,
    /// Support bundle or support-center export.
    SupportExport,
    /// Help or docs surface.
    HelpDocs,
    /// Admin docs and managed-fleet surfaces.
    AdminDocs,
}

impl SurfaceClass {
    /// Required surface set for parity.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopSettings,
        Self::CliInspect,
        Self::SupportExport,
        Self::HelpDocs,
        Self::AdminDocs,
    ];
}

/// One row in the portable-state package class table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortablePackageClassRow {
    /// Artifact class being classified.
    pub artifact_class: PortableArtifactClass,
    /// Disposition the package applies to this class.
    pub disposition: PortabilityDisposition,
    /// Canonical ref to the included content or its reference manifest.
    pub content_ref: String,
    /// Exclusion reason when the class is redacted or machine-local.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusion_reason: Option<ExclusionReason>,
    /// Human-readable rationale shown to the user or admin.
    pub rationale: String,
}

/// A visible placeholder standing in for a missing dependency on restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingDependencyPlaceholder {
    /// Dependency kind that is unavailable.
    pub kind: MissingDependencyKind,
    /// Artifact class whose restore is affected.
    pub affected_artifact_class: PortableArtifactClass,
    /// Canonical ref to the placeholder card rendered in the restored layout.
    pub placeholder_ref: String,
    /// Whether the placeholder is visible in the restored layout/packet.
    pub visible_in_layout: bool,
    /// Whether the affected surface was silently dropped (must be false).
    pub silently_dropped: bool,
    /// Recovery hint, e.g. install the extension or reconnect the target.
    pub recovery_hint: String,
}

/// A restore-provenance card rendered before an M5 package is applied/reopened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceCard {
    /// Stable card id.
    pub card_id: String,
    /// Canonical ref to the source package being restored.
    pub source_package_ref: String,
    /// Artifact class this card restores.
    pub artifact_class: PortableArtifactClass,
    /// Honest schema-migration / fidelity label for this restore.
    pub migration_label: MigrationLabel,
    /// Source schema version recorded in the package.
    pub source_schema_version: String,
    /// Target schema version of the importing client.
    pub target_schema_version: String,
    /// Canonical ref to the integrity hash or manifest covering the body.
    pub integrity_ref: String,
    /// Canonical ref to the rollback checkpoint created before overwrite.
    pub rollback_checkpoint_ref: String,
    /// Canonical ref to the sidecar holding unmappable values, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmappable_sidecar_ref: Option<String>,
    /// Missing-dependency placeholders attached to this card.
    pub placeholders: Vec<MissingDependencyPlaceholder>,
    /// Whether the card can be inspected before apply without mutation.
    pub previewable_before_apply: bool,
    /// Whether applying the card overwrites current local state.
    pub overwrites_local_state: bool,
}

impl RestoreProvenanceCard {
    /// Returns true when this card has at least one missing dependency.
    pub fn has_missing_dependencies(&self) -> bool {
        !self.placeholders.is_empty()
    }

    /// Returns true when the source and target schema versions match.
    pub fn schema_versions_match(&self) -> bool {
        self.source_schema_version == self.target_schema_version
    }
}

/// One surface-parity row for package and restore-provenance truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes this shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface shows per-class disposition.
    pub shows_disposition: bool,
    /// Whether the surface shows the migration label.
    pub shows_migration_label: bool,
    /// Whether the surface shows missing-dependency placeholders.
    pub shows_placeholders: bool,
    /// Whether the surface shows the rollback checkpoint.
    pub shows_rollback_checkpoint: bool,
}

/// Derived pillar verdicts for the portable-state restore contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableRestorePillars {
    /// Every required artifact class is classified with a disposition.
    pub package_classes_complete: bool,
    /// Secrets, live handles, and machine-unique anchors never cross as portable.
    pub secret_boundary_held: bool,
    /// Every restore card carries an honest migration label.
    pub provenance_labeled: bool,
    /// Missing dependencies are visible placeholders, never silently dropped.
    pub placeholders_visible: bool,
    /// Overwriting restores are previewable and rollback-checkpointed.
    pub restore_checkpointed: bool,
    /// All required surfaces render the same package and provenance truth.
    pub surface_truth_complete: bool,
}

/// Reason a record is narrowed below the exact-fidelity claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// One or more artifact classes are unclassified.
    PackageClassesIncomplete,
    /// A forbidden class would cross the machine boundary as portable.
    SecretBoundaryFailed,
    /// A restore card is missing an honest migration label.
    ProvenanceUnlabeled,
    /// A missing dependency is not a visible placeholder.
    PlaceholderDropped,
    /// An overwriting restore lacks a preview or rollback checkpoint.
    RestoreNotCheckpointed,
    /// At least one card restores below exact fidelity.
    FidelityBelowExact,
    /// One or more surfaces omit required package or provenance truth.
    SurfaceTruthIncomplete,
}

/// Public claim class derived from the portable-state restore evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableRestoreClaim {
    /// Every card restores exactly and every pillar holds.
    ExactRestore,
    /// Restore is sound but at least one card is below exact fidelity.
    DegradedRestore,
    /// A structural pillar failed; the package is not safely restorable as-is.
    Unsupported,
}

/// Derived fidelity verdict for the whole package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableRestoreQualification {
    /// Derived claim class.
    pub claim_class: PortableRestoreClaim,
    /// Weakest migration label implied across all restore cards.
    pub effective_fidelity_ceiling: MigrationLabel,
    /// Whether the package qualifies for an exact-fidelity claim.
    pub qualifies_exact: bool,
    /// Named narrowing reasons.
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// Input used to build a [`M5PortableStateRestoreCertification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PortableStateRestoreInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Portable-state package class rows.
    pub package_classes: Vec<PortablePackageClassRow>,
    /// Restore provenance cards.
    pub restore_cards: Vec<RestoreProvenanceCard>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
}

/// Canonical portable-state export/import and restore-provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableStateRestoreCertification {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Portable-state package class rows.
    pub package_classes: Vec<PortablePackageClassRow>,
    /// Restore provenance cards.
    pub restore_cards: Vec<RestoreProvenanceCard>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// Migration labels covered by the restore cards.
    pub migration_label_coverage: Vec<MigrationLabel>,
    /// Missing-dependency kinds surfaced as placeholders.
    pub missing_dependency_coverage: Vec<MissingDependencyKind>,
    /// Derived pillar verdicts.
    pub pillars: PortableRestorePillars,
    /// Derived fidelity qualification.
    pub fidelity_qualification: PortableRestoreQualification,
}

/// Reasons a portable-state restore certification cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A required artifact class is unclassified.
    MissingArtifactClass {
        /// The artifact class with no package class row.
        class: PortableArtifactClass,
    },
    /// An artifact class is classified more than once.
    DuplicateArtifactClass {
        /// The duplicated artifact class.
        class: PortableArtifactClass,
    },
    /// A canonical ref field is invalid.
    NonCanonicalRef {
        /// The field carrying the invalid ref.
        field: &'static str,
        /// The offending value.
        value: String,
    },
    /// A class carried as portable names a serialization-forbidden exclusion.
    SecretCarriedAsPortable {
        /// The artifact class that would leak.
        class: PortableArtifactClass,
        /// The forbidden exclusion reason it declared.
        reason: ExclusionReason,
    },
    /// A redacted or machine-local class is missing its exclusion reason.
    MissingExclusionReason {
        /// The class without an exclusion reason.
        class: PortableArtifactClass,
    },
    /// A card claims exact fidelity while carrying a missing dependency.
    ExactClaimWithMissingDependency {
        /// The dishonest card id.
        card_id: String,
    },
    /// A card claims exact fidelity across a schema-version mismatch.
    ExactClaimAcrossSchemaMismatch {
        /// The dishonest card id.
        card_id: String,
    },
    /// A required surface row is missing.
    MissingSurface {
        /// The missing surface.
        surface: SurfaceClass,
    },
    /// No restore cards were supplied.
    NoRestoreCards,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingArtifactClass { class } => {
                write!(f, "missing artifact class `{}`", class.as_str())
            }
            Self::DuplicateArtifactClass { class } => {
                write!(f, "duplicated artifact class `{}`", class.as_str())
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::SecretCarriedAsPortable { class, reason } => write!(
                f,
                "class `{}` cannot be portable while declaring `{}`",
                class.as_str(),
                reason.as_str()
            ),
            Self::MissingExclusionReason { class } => write!(
                f,
                "redacted/machine-local class `{}` requires an exclusion reason",
                class.as_str()
            ),
            Self::ExactClaimWithMissingDependency { card_id } => write!(
                f,
                "card `{card_id}` claims exact fidelity with a missing dependency"
            ),
            Self::ExactClaimAcrossSchemaMismatch { card_id } => write!(
                f,
                "card `{card_id}` claims exact fidelity across a schema-version mismatch"
            ),
            Self::MissingSurface { surface } => write!(f, "missing surface `{surface:?}`"),
            Self::NoRestoreCards => write!(f, "at least one restore card is required"),
        }
    }
}

impl std::error::Error for BuildError {}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_owned(),
        })
    }
}

impl M5PortableStateRestoreCertification {
    /// Builds a derived certification record from raw evidence rows.
    ///
    /// Returns a [`BuildError`] when a structural invariant or a fail-closed
    /// guardrail is violated, so a package that would leak secrets or overstate
    /// restore fidelity cannot be constructed.
    pub fn build(mut input: M5PortableStateRestoreInput) -> Result<Self, BuildError> {
        if input.restore_cards.is_empty() {
            return Err(BuildError::NoRestoreCards);
        }

        // Package class table: one row per required class, no duplicates, with
        // refs and exclusion reasons validated, and the secret boundary enforced.
        let mut seen_classes = BTreeSet::new();
        for row in &input.package_classes {
            if !seen_classes.insert(row.artifact_class) {
                return Err(BuildError::DuplicateArtifactClass {
                    class: row.artifact_class,
                });
            }
            require_ref("package_classes.content_ref", &row.content_ref)?;
            match row.disposition {
                PortabilityDisposition::Portable => {
                    if let Some(reason) = row.exclusion_reason {
                        if reason.forbids_serialization() {
                            return Err(BuildError::SecretCarriedAsPortable {
                                class: row.artifact_class,
                                reason,
                            });
                        }
                    }
                }
                PortabilityDisposition::Redacted | PortabilityDisposition::MachineLocal => {
                    if row.exclusion_reason.is_none() {
                        return Err(BuildError::MissingExclusionReason {
                            class: row.artifact_class,
                        });
                    }
                }
                PortabilityDisposition::RestoreOnly => {}
            }
        }
        for class in PortableArtifactClass::REQUIRED {
            if !seen_classes.contains(&class) {
                return Err(BuildError::MissingArtifactClass { class });
            }
        }

        // Restore cards: refs valid, and exact-fidelity claims are fail-closed.
        for card in &input.restore_cards {
            require_ref("restore_cards.source_package_ref", &card.source_package_ref)?;
            require_ref("restore_cards.integrity_ref", &card.integrity_ref)?;
            if !card.rollback_checkpoint_ref.is_empty() {
                require_ref(
                    "restore_cards.rollback_checkpoint_ref",
                    &card.rollback_checkpoint_ref,
                )?;
            }
            if let Some(sidecar) = &card.unmappable_sidecar_ref {
                require_ref("restore_cards.unmappable_sidecar_ref", sidecar)?;
            }
            for placeholder in &card.placeholders {
                require_ref(
                    "restore_cards.placeholders.placeholder_ref",
                    &placeholder.placeholder_ref,
                )?;
            }
            if card.migration_label.implies_exact_fidelity() {
                if card.has_missing_dependencies() {
                    return Err(BuildError::ExactClaimWithMissingDependency {
                        card_id: card.card_id.clone(),
                    });
                }
                if !card.schema_versions_match() {
                    return Err(BuildError::ExactClaimAcrossSchemaMismatch {
                        card_id: card.card_id.clone(),
                    });
                }
            }
        }

        let present_surfaces: BTreeSet<SurfaceClass> = input
            .surface_truth
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for surface in SurfaceClass::REQUIRED {
            if !present_surfaces.contains(&surface) {
                return Err(BuildError::MissingSurface { surface });
            }
        }

        input.package_classes.sort_by_key(|row| row.artifact_class);
        input
            .restore_cards
            .sort_by(|a, b| a.card_id.cmp(&b.card_id));
        input.surface_truth.sort_by_key(|row| row.surface_class);

        let migration_label_coverage: Vec<MigrationLabel> = input
            .restore_cards
            .iter()
            .map(|card| card.migration_label)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let missing_dependency_coverage: Vec<MissingDependencyKind> = input
            .restore_cards
            .iter()
            .flat_map(|card| card.placeholders.iter().map(|p| p.kind))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        // The package classifies every required class and never carries a
        // serialization-forbidden class across the machine boundary.
        let package_classes_complete = PortableArtifactClass::REQUIRED
            .iter()
            .all(|class| seen_classes.contains(class))
            && input
                .package_classes
                .iter()
                .all(|row| !row.rationale.trim().is_empty());

        let secret_boundary_held = input.package_classes.iter().all(|row| {
            !(row.disposition.crosses_machine_boundary()
                && row
                    .exclusion_reason
                    .is_some_and(ExclusionReason::forbids_serialization)
                && row.disposition == PortabilityDisposition::Portable)
        });

        let provenance_labeled = input
            .restore_cards
            .iter()
            .all(|card| !card.source_schema_version.trim().is_empty());

        let placeholders_visible = input.restore_cards.iter().all(|card| {
            card.placeholders
                .iter()
                .all(|p| p.visible_in_layout && !p.silently_dropped)
        });

        let restore_checkpointed = input.restore_cards.iter().all(|card| {
            card.previewable_before_apply
                && (!card.overwrites_local_state
                    || is_canonical_object_ref(&card.rollback_checkpoint_ref))
        });

        let surface_truth_complete = input.surface_truth.iter().all(|row| {
            row.consumes_shared_record
                && row.shows_disposition
                && row.shows_migration_label
                && row.shows_placeholders
                && row.shows_rollback_checkpoint
        });

        let effective_fidelity_ceiling = input
            .restore_cards
            .iter()
            .map(|card| card.migration_label)
            .max_by_key(|label| label.fidelity_rank())
            .unwrap_or(MigrationLabel::Exact);

        let pillars = PortableRestorePillars {
            package_classes_complete,
            secret_boundary_held,
            provenance_labeled,
            placeholders_visible,
            restore_checkpointed,
            surface_truth_complete,
        };

        let mut narrowing_reasons = Vec::new();
        if !pillars.package_classes_complete {
            narrowing_reasons.push(NarrowingReason::PackageClassesIncomplete);
        }
        if !pillars.secret_boundary_held {
            narrowing_reasons.push(NarrowingReason::SecretBoundaryFailed);
        }
        if !pillars.provenance_labeled {
            narrowing_reasons.push(NarrowingReason::ProvenanceUnlabeled);
        }
        if !pillars.placeholders_visible {
            narrowing_reasons.push(NarrowingReason::PlaceholderDropped);
        }
        if !pillars.restore_checkpointed {
            narrowing_reasons.push(NarrowingReason::RestoreNotCheckpointed);
        }
        if !effective_fidelity_ceiling.implies_exact_fidelity() {
            narrowing_reasons.push(NarrowingReason::FidelityBelowExact);
        }
        if !pillars.surface_truth_complete {
            narrowing_reasons.push(NarrowingReason::SurfaceTruthIncomplete);
        }

        let structural_ok = pillars.package_classes_complete
            && pillars.secret_boundary_held
            && pillars.provenance_labeled
            && pillars.placeholders_visible
            && pillars.restore_checkpointed
            && pillars.surface_truth_complete;

        let qualifies_exact = structural_ok && effective_fidelity_ceiling.implies_exact_fidelity();

        let claim_class = if !structural_ok {
            PortableRestoreClaim::Unsupported
        } else if qualifies_exact {
            PortableRestoreClaim::ExactRestore
        } else {
            PortableRestoreClaim::DegradedRestore
        };

        let fidelity_qualification = PortableRestoreQualification {
            claim_class,
            effective_fidelity_ceiling,
            qualifies_exact,
            narrowing_reasons,
        };

        Ok(Self {
            record_kind: M5_PORTABLE_STATE_RESTORE_RECORD_KIND.to_owned(),
            schema_version: M5_PORTABLE_STATE_RESTORE_SCHEMA_VERSION,
            shared_contract_ref: M5_PORTABLE_STATE_RESTORE_SHARED_CONTRACT_REF.to_owned(),
            record_id: input.record_id,
            as_of: input.as_of,
            summary: input.summary,
            package_classes: input.package_classes,
            restore_cards: input.restore_cards,
            surface_truth: input.surface_truth,
            migration_label_coverage,
            missing_dependency_coverage,
            pillars,
            fidelity_qualification,
        })
    }

    /// Renders a compact, export-safe support summary from the shared record.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("record_id: {}", self.record_id),
            format!("claim: {:?}", self.fidelity_qualification.claim_class),
            format!(
                "fidelity_ceiling: {}",
                self.fidelity_qualification
                    .effective_fidelity_ceiling
                    .as_str()
            ),
            format!("package_classes: {}", self.package_classes.len()),
            format!("restore_cards: {}", self.restore_cards.len()),
            format!(
                "missing_dependencies: {}",
                self.missing_dependency_coverage.len()
            ),
        ]
    }
}
