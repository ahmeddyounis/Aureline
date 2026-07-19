//! Compatibility-forecast sheets and migration-assistant task rows — the per-subject forecast a user,
//! admin, or support reviewer reads *before restart or rollout widening* to see how a staged M5 update
//! will drift the surfaces Aureline qualifies, on top of the
//! [typed change-impact cards](crate::m5_change_impact_card).
//!
//! The change-impact cards answer "what will this update do to my workspace, profile, schema, caches,
//! extensions, remote helpers, and toolchain once I restart". This lane answers the narrower
//! lifecycle question the exit-gate anchor calls out: *which qualified subjects — certified archetypes,
//! extension SDK / manifest ranges, remote-agent skew, and public export / schema readers — will drift
//! out of compatibility on the stable / beta / preview / LTS lines, and what concrete migration tasks
//! clear that drift before a stable-facing surface breaks*.
//!
//! Each forecast [subject](CompatibilitySubject) gets one [forecast](SubjectForecast) carrying one
//! [line forecast](LineForecast) per [compatibility line](CompatibilityLine), so a subject's drift is
//! never collapsed into a single channel-agnostic verdict. Per line, a reviewer sees:
//!
//! - a [drift class](DriftClass) that deliberately distinguishes a compatible-within-window forecast
//!   from a [breaking drift](DriftClass::BreakingDrift);
//! - a [forecast confidence](ForecastConfidence) that labels unknown inputs and — the lane's
//!   guardrail — subjects *outside Aureline's claimed window* honestly: a breaking drift forecast made
//!   on speculative or out-of-window inputs is flagged for review, never raised as a hard failure
//!   (enforced by [`CompatibilityForecastSheet::validate`]).
//!
//! Every subject that drifts (narrows or holds) MUST carry at least one
//! [migration task row](MigrationTaskRow) — the actionable companion the migration assistant routes
//! the user through. Each row discloses its owner, affected scope, [auto-fix
//! availability](AutoFixAvailability), [due-before boundary](DueBoundary), [skip / waive
//! policy](SkipPolicy), [rollback guidance](RollbackGuidance), and the [actions](MigrationAction) —
//! pin, postpone, side-by-side, validator, repair — Aureline already offers. A task is suppressible
//! only with a recorded rationale where its policy requires one, enforced in validation.
//!
//! The [consumer surfaces](ForecastConsumer) — update center, migration assistant, release center,
//! admin console, support export — bind the subject families they read and *derive* their [review
//! readiness](ForecastReadiness) and gaps from the forecasts, so all of them read this one
//! [`CompatibilityForecastSheet`] packet rather than cloning drift fields locally.
//!
//! The packet is inspectable and serde-serializable; it carries metadata, refs, and message ids only
//! — no credential bodies or raw provider payloads — so the forecast is exportable and reviewable
//! outside the app without forcing an immediate restart.
//!
//! - Packet schema:
//!   [`schemas/release/m5-compatibility-forecast.schema.json`](../../../../../schemas/release/m5-compatibility-forecast.schema.json)
//! - Migration-task-row schema:
//!   [`schemas/release/m5-migration-task-row.schema.json`](../../../../../schemas/release/m5-migration-task-row.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-compatibility-forecast-contract.md`](../../../../../docs/release/m5-compatibility-forecast-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_compatibility_forecast_sheet, seeded_m5_compatibility_forecast_sheet_hold,
    seeded_m5_compatibility_forecast_sheet_out_of_window,
    seeded_m5_compatibility_forecast_sheet_review, M5_COMPATIBILITY_FORECAST_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The forecast reuses the update / support-lifecycle governance vocabularies for artifact class,
// channel, and deployment profile, and the descriptor / badge runtime's gate / status / signal
// vocabulary, so this forecast layer can never drift to a different vocabulary than the layers above.
use crate::m5_descriptor_badge::{ConsumerStatus, DescriptorGate, DescriptorSignal};
use crate::m5_update_lifecycle::{ArtifactClass, ChannelScope, DeploymentProfile};

/// Record-kind tag carried by [`CompatibilityForecastSheet`].
pub const M5_COMPATIBILITY_FORECAST_RECORD_KIND: &str = "m5_compatibility_forecast_sheet";

/// Schema version for the compatibility-forecast sheet packet.
pub const M5_COMPATIBILITY_FORECAST_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the forecast-sheet packet schema.
pub const M5_COMPATIBILITY_FORECAST_SCHEMA_REF: &str =
    "schemas/release/m5-compatibility-forecast.schema.json";

/// Repo-relative path of the migration-task-row schema.
pub const M5_MIGRATION_TASK_ROW_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the published forecast-sheet inventory.
pub const M5_COMPATIBILITY_FORECAST_REF: &str = "artifacts/release/m5-compatibility-forecast.json";

/// Repo-relative path of the release-grade migration-assistant parity proof.
pub const M5_COMPATIBILITY_FORECAST_PROOF_REF: &str =
    "artifacts/release/m5-migration-assistant-proof/compatibility-forecast.json";

/// Repo-relative path of the machine-readable per-task migration export.
pub const M5_MIGRATION_TASK_CSV_REF: &str = "artifacts/release/m5-migration-tasks.csv";

/// Repo-relative path of the forecast-sheet contract doc.
pub const M5_COMPATIBILITY_FORECAST_DOC_REF: &str =
    "docs/release/m5-compatibility-forecast-contract.md";

/// Repo-relative directory of the per-state forecast-sheet fixtures.
pub const M5_COMPATIBILITY_FORECAST_FIXTURE_DIR: &str = "fixtures/release/compatibility-forecast/";

/// Prefix every forecast message id carries so consumers can route it.
pub const M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX: &str = "release_compat_forecast.";

const REDACTION_CLASS: &str = "metadata_safe_default";

// ---------------------------------------------------------------------------
// Controlled vocabularies
// ---------------------------------------------------------------------------

/// One qualified subject family the lane forecasts compatibility drift for. The set is the union of
/// the subjects the exit-gate anchor names; each is forecast once so an update never collapses
/// unrelated subjects into a single generic row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilitySubject {
    /// A certified reference archetype.
    CertifiedArchetype,
    /// An installed extension's SDK version range.
    ExtensionSdkRange,
    /// An installed extension's manifest compatibility range.
    ExtensionManifestRange,
    /// Remote-agent / helper version skew.
    RemoteAgentSkew,
    /// A public export-format reader.
    PublicExportReader,
    /// A public schema / contract reader.
    PublicSchemaReader,
}

impl CompatibilitySubject {
    /// Every subject family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CertifiedArchetype,
        Self::ExtensionSdkRange,
        Self::ExtensionManifestRange,
        Self::RemoteAgentSkew,
        Self::PublicExportReader,
        Self::PublicSchemaReader,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedArchetype => "certified_archetype",
            Self::ExtensionSdkRange => "extension_sdk_range",
            Self::ExtensionManifestRange => "extension_manifest_range",
            Self::RemoteAgentSkew => "remote_agent_skew",
            Self::PublicExportReader => "public_export_reader",
            Self::PublicSchemaReader => "public_schema_reader",
        }
    }

    /// Human-facing label for the subject.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CertifiedArchetype => "Certified archetype",
            Self::ExtensionSdkRange => "Extension SDK range",
            Self::ExtensionManifestRange => "Extension manifest range",
            Self::RemoteAgentSkew => "Remote-agent skew",
            Self::PublicExportReader => "Public export reader",
            Self::PublicSchemaReader => "Public schema reader",
        }
    }

    /// The primary artifact class this subject forecasts drift on.
    pub const fn primary_artifact_class(self) -> ArtifactClass {
        match self {
            Self::CertifiedArchetype => ArtifactClass::WorkspaceState,
            Self::ExtensionSdkRange | Self::ExtensionManifestRange => ArtifactClass::ExtensionPacks,
            Self::RemoteAgentSkew => ArtifactClass::CoreRuntime,
            Self::PublicExportReader | Self::PublicSchemaReader => ArtifactClass::SchemaContracts,
        }
    }

    /// Accountable owner role for this subject's forecast.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::CertifiedArchetype => "certification_owner",
            Self::ExtensionSdkRange => "extension_sdk_owner",
            Self::ExtensionManifestRange => "extension_manifest_owner",
            Self::RemoteAgentSkew => "remote_helper_owner",
            Self::PublicExportReader => "export_contract_owner",
            Self::PublicSchemaReader => "schema_contract_owner",
        }
    }
}

/// One compatibility line the forecast covers. The set is the stable-facing line vocabulary the
/// exit-gate anchor names — `nightly` is excluded because the lane forecasts drift against the lines a
/// support window can claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityLine {
    /// The general-availability lane.
    Stable,
    /// The publicly announced pre-release lane.
    Beta,
    /// The gated pre-release lane.
    Preview,
    /// The long-term-support line.
    Lts,
}

impl CompatibilityLine {
    /// Every line, in declaration order.
    pub const ALL: [Self; 4] = [Self::Stable, Self::Beta, Self::Preview, Self::Lts];

    /// Stable token recorded in the packet; a subset of the frozen release-channel vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Lts => "lts",
        }
    }
}

/// The drift class a line forecast assigns. Declaration order is least→most severe; the vocabulary
/// deliberately separates a [`CompatibleWithinWindow`](Self::CompatibleWithinWindow) forecast from a
/// [`BreakingDrift`](Self::BreakingDrift) so a compatible update can never read like a break, and vice
/// versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftClass {
    /// No drift is forecast for this line.
    NoDrift,
    /// Compatible and within the supported window; no action.
    CompatibleWithinWindow,
    /// Compatible now, but a deprecation is scheduled the user should be warned of.
    DeprecationScheduled,
    /// A migration / range bump is required to stay compatible.
    MigrationRequired,
    /// A breaking drift: the subject breaks on this line without a migration.
    BreakingDrift,
}

impl DriftClass {
    /// Every drift class, least→most severe.
    pub const ALL: [Self; 5] = [
        Self::NoDrift,
        Self::CompatibleWithinWindow,
        Self::DeprecationScheduled,
        Self::MigrationRequired,
        Self::BreakingDrift,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDrift => "no_drift",
            Self::CompatibleWithinWindow => "compatible_within_window",
            Self::DeprecationScheduled => "deprecation_scheduled",
            Self::MigrationRequired => "migration_required",
            Self::BreakingDrift => "breaking_drift",
        }
    }

    /// True only for the drift classes that stay compatible (no migration required).
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::NoDrift | Self::CompatibleWithinWindow | Self::DeprecationScheduled
        )
    }

    /// True for the one drift class that is a hard break.
    pub const fn is_breaking(self) -> bool {
        matches!(self, Self::BreakingDrift)
    }

    /// True when this drift class requires a migration task to clear it.
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::MigrationRequired | Self::BreakingDrift)
    }

    /// The gate this drift class implies *assuming the forecast is certain*. The effective gate is this
    /// gate capped by the [forecast confidence](ForecastConfidence::gate_cap).
    pub const fn drift_gate(self) -> DescriptorGate {
        match self {
            Self::NoDrift | Self::CompatibleWithinWindow => DescriptorGate::Governed,
            Self::DeprecationScheduled | Self::MigrationRequired => DescriptorGate::Narrowed,
            Self::BreakingDrift => DescriptorGate::Blocked,
        }
    }
}

/// How confident the forecast is, given the inputs Aureline actually has. Declaration order is
/// best→worst. The lane's guardrail lives here: an [`Estimated`](Self::Estimated),
/// [`Unknown`](Self::Unknown), or [`OutsideClaimedWindow`](Self::OutsideClaimedWindow) forecast caps
/// the line's gate at narrowed, so speculation and unqualified subjects are labeled honestly and never
/// raised as a hard pre-rollout failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastConfidence {
    /// The subject is within Aureline's claimed window and the inputs are fully available.
    Qualified,
    /// Inputs are mostly available; the forecast is well-supported.
    Likely,
    /// Inputs are partial; the forecast is an estimate.
    Estimated,
    /// Inputs are unavailable; the drift cannot be forecast and is labeled as such.
    Unknown,
    /// The subject is outside Aureline's claimed compatibility window — coverage is not asserted, and
    /// the forecast is labeled rather than presented as authoritative.
    OutsideClaimedWindow,
    /// The line does not apply to this subject.
    NotApplicable,
}

impl ForecastConfidence {
    /// Every confidence level, best→worst.
    pub const ALL: [Self; 6] = [
        Self::Qualified,
        Self::Likely,
        Self::Estimated,
        Self::Unknown,
        Self::OutsideClaimedWindow,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Likely => "likely",
            Self::Estimated => "estimated",
            Self::Unknown => "unknown",
            Self::OutsideClaimedWindow => "outside_claimed_window",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the forecast rests on partial or absent inputs (estimated / unknown).
    pub const fn is_speculative(self) -> bool {
        matches!(self, Self::Estimated | Self::Unknown)
    }

    /// True when the subject is outside Aureline's claimed window.
    pub const fn is_outside_window(self) -> bool {
        matches!(self, Self::OutsideClaimedWindow)
    }

    /// True when this confidence forbids a hard failure: speculative or out-of-window inputs cap the
    /// gate at narrowed. This is the predicate the guardrail validation reads.
    pub const fn caps_below_blocked(self) -> bool {
        matches!(
            self,
            Self::Estimated | Self::Unknown | Self::OutsideClaimedWindow
        )
    }

    /// The most severe gate this confidence allows. The line's effective gate is the *less severe* of
    /// the drift gate and this cap, so speculative / out-of-window inputs cap at narrowed and a
    /// not-applicable line caps at governed.
    pub const fn gate_cap(self) -> DescriptorGate {
        match self {
            Self::Qualified | Self::Likely => DescriptorGate::Blocked,
            Self::Estimated | Self::Unknown | Self::OutsideClaimedWindow => {
                DescriptorGate::Narrowed
            }
            Self::NotApplicable => DescriptorGate::Governed,
        }
    }
}

/// The review readiness a subject or consumer resolves to, in pre-rollout language. A direct, one-to-one
/// reading of a [`DescriptorGate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastReadiness {
    /// No drift; clear to apply and widen the rollout.
    ClearToWiden,
    /// At least one narrowing drift; review before widening the rollout.
    ReviewBeforeWidening,
    /// At least one confirmed breaking drift; hold and resolve before widening or restart.
    HoldBeforeWidening,
}

impl ForecastReadiness {
    /// Every readiness, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ClearToWiden,
        Self::ReviewBeforeWidening,
        Self::HoldBeforeWidening,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearToWiden => "clear_to_widen",
            Self::ReviewBeforeWidening => "review_before_widening",
            Self::HoldBeforeWidening => "hold_before_widening",
        }
    }

    /// The readiness a gate resolves to.
    pub const fn from_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::ClearToWiden,
            DescriptorGate::Narrowed => Self::ReviewBeforeWidening,
            DescriptorGate::Blocked => Self::HoldBeforeWidening,
        }
    }
}

/// The class of migration a task row clears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationTaskClass {
    /// No migration is required.
    NoActionRequired,
    /// Re-validate a certified archetype against the target.
    ArchetypeRevalidation,
    /// Bump an extension's supported SDK range.
    ExtensionSdkRangeBump,
    /// Update an extension manifest's compatibility range.
    ExtensionManifestUpdate,
    /// Upgrade a remote helper / agent to clear skew.
    RemoteHelperUpgrade,
    /// Migrate a public export reader to the new format.
    ExportReaderMigration,
    /// Migrate a public schema reader to the new contract.
    SchemaReaderMigration,
}

impl MigrationTaskClass {
    /// Every task class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NoActionRequired,
        Self::ArchetypeRevalidation,
        Self::ExtensionSdkRangeBump,
        Self::ExtensionManifestUpdate,
        Self::RemoteHelperUpgrade,
        Self::ExportReaderMigration,
        Self::SchemaReaderMigration,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoActionRequired => "no_action_required",
            Self::ArchetypeRevalidation => "archetype_revalidation",
            Self::ExtensionSdkRangeBump => "extension_sdk_range_bump",
            Self::ExtensionManifestUpdate => "extension_manifest_update",
            Self::RemoteHelperUpgrade => "remote_helper_upgrade",
            Self::ExportReaderMigration => "export_reader_migration",
            Self::SchemaReaderMigration => "schema_reader_migration",
        }
    }
}

/// How much of a migration task Aureline can automate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoFixAvailability {
    /// No task, so auto-fix does not apply.
    NotApplicable,
    /// Aureline can apply the fix automatically.
    AutoFixAvailable,
    /// An assistant can drive the fix with user confirmation.
    AssistedFix,
    /// The user must perform manual steps.
    ManualOnly,
    /// An administrator must act.
    AdminRequired,
}

impl AutoFixAvailability {
    /// Every auto-fix level, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotApplicable,
        Self::AutoFixAvailable,
        Self::AssistedFix,
        Self::ManualOnly,
        Self::AdminRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::AutoFixAvailable => "auto_fix_available",
            Self::AssistedFix => "assisted_fix",
            Self::ManualOnly => "manual_only",
            Self::AdminRequired => "admin_required",
        }
    }

    /// True when Aureline can drive the fix without unaided manual steps.
    pub const fn is_automatable(self) -> bool {
        matches!(self, Self::AutoFixAvailable | Self::AssistedFix)
    }
}

/// The boundary a migration task must complete before. Ties the forecast to the exit-gate anchor:
/// drift must be cleared *before a stable-facing surface breaks*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DueBoundary {
    /// No task, so no boundary.
    NotRequired,
    /// Before the update is applied.
    BeforeApply,
    /// Before the restart that activates the update.
    BeforeRestart,
    /// Before the rollout is widened past its current ring.
    BeforeRolloutWidening,
    /// Before the next stable-line promotion.
    BeforeNextStableLine,
    /// Before the subject's end-of-support boundary.
    BeforeEndOfSupport,
}

impl DueBoundary {
    /// Every boundary, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotRequired,
        Self::BeforeApply,
        Self::BeforeRestart,
        Self::BeforeRolloutWidening,
        Self::BeforeNextStableLine,
        Self::BeforeEndOfSupport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::BeforeApply => "before_apply",
            Self::BeforeRestart => "before_restart",
            Self::BeforeRolloutWidening => "before_rollout_widening",
            Self::BeforeNextStableLine => "before_next_stable_line",
            Self::BeforeEndOfSupport => "before_end_of_support",
        }
    }
}

/// Whether a migration task can be skipped, and whether suppressing it needs a recorded rationale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipPolicy {
    /// The task is resolved automatically; no skip applies.
    AutoResolved,
    /// The task is optional; the user may skip it freely.
    OptionalRecommended,
    /// The task may be waived, but only with a recorded rationale.
    SkippableWithRationale,
    /// The task must be completed; it cannot be waived.
    NotSkippable,
}

impl SkipPolicy {
    /// Every skip policy, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AutoResolved,
        Self::OptionalRecommended,
        Self::SkippableWithRationale,
        Self::NotSkippable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoResolved => "auto_resolved",
            Self::OptionalRecommended => "optional_recommended",
            Self::SkippableWithRationale => "skippable_with_rationale",
            Self::NotSkippable => "not_skippable",
        }
    }

    /// True when suppressing the task requires a recorded rationale.
    pub const fn requires_recorded_rationale(self) -> bool {
        matches!(self, Self::SkippableWithRationale)
    }

    /// True when the task may be waived at all.
    pub const fn is_waivable(self) -> bool {
        matches!(
            self,
            Self::OptionalRecommended | Self::SkippableWithRationale
        )
    }
}

/// The rollback path a migration task discloses, so a reviewer always sees the recovery route. Kinds
/// are distinct so a row never implies a true version rollback when only a pin, a side-by-side
/// fallback, or a reinstall remains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackGuidance {
    /// No recovery path applies (nothing changes).
    NotApplicable,
    /// A true rollback to the prior version is supported.
    RollbackSupported,
    /// The current version can be pinned to defer the change.
    PinCurrentVersion,
    /// The prior version coexists; the user can fall back side-by-side.
    SideBySideFallback,
    /// Recovering the prior state requires a reinstall.
    ReinstallOnly,
    /// No rollback, pin, or fallback is available.
    NoRollback,
}

impl RollbackGuidance {
    /// Every rollback guidance, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotApplicable,
        Self::RollbackSupported,
        Self::PinCurrentVersion,
        Self::SideBySideFallback,
        Self::ReinstallOnly,
        Self::NoRollback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::RollbackSupported => "rollback_supported",
            Self::PinCurrentVersion => "pin_current_version",
            Self::SideBySideFallback => "side_by_side_fallback",
            Self::ReinstallOnly => "reinstall_only",
            Self::NoRollback => "no_rollback",
        }
    }

    /// True when the user has *some* way to defer or recover.
    pub const fn offers_recovery(self) -> bool {
        matches!(
            self,
            Self::RollbackSupported | Self::PinCurrentVersion | Self::SideBySideFallback
        )
    }
}

/// A pre-emptive action a migration task surfaces, drawn only from the mechanisms Aureline already has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationAction {
    /// Pin the current version to defer the change.
    Pin,
    /// Postpone the update.
    Postpone,
    /// Install the new version side-by-side with the current one.
    SideBySide,
    /// Run a compatibility validator against the subject.
    Validator,
    /// Run guided repair on the subject.
    Repair,
}

impl MigrationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pin,
        Self::Postpone,
        Self::SideBySide,
        Self::Validator,
        Self::Repair,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pin => "pin",
            Self::Postpone => "postpone",
            Self::SideBySide => "side_by_side",
            Self::Validator => "validator",
            Self::Repair => "repair",
        }
    }
}

/// The named cause of a consumer's review gap on one subject it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastGapKind {
    /// A read subject narrows on a confirmed drift; review before widening.
    ReviewBeforeWidening,
    /// A read subject narrows because its inputs are speculative; review, not a failure.
    ForecastInputUnknown,
    /// A read subject narrows because it is outside Aureline's claimed window; labeled, not a failure.
    OutsideClaimedWindow,
    /// A read subject is a confirmed breaking drift; resolve before widening.
    ResolveBeforeWidening,
    /// A subject family the consumer reads is not forecast in the sheet.
    SubjectNotForecast,
}

impl ForecastGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReviewBeforeWidening,
        Self::ForecastInputUnknown,
        Self::OutsideClaimedWindow,
        Self::ResolveBeforeWidening,
        Self::SubjectNotForecast,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewBeforeWidening => "review_before_widening",
            Self::ForecastInputUnknown => "forecast_input_unknown",
            Self::OutsideClaimedWindow => "outside_claimed_window",
            Self::ResolveBeforeWidening => "resolve_before_widening",
            Self::SubjectNotForecast => "subject_not_forecast",
        }
    }

    /// The gate this gap forces.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::ReviewBeforeWidening
            | Self::ForecastInputUnknown
            | Self::OutsideClaimedWindow => DescriptorGate::Narrowed,
            Self::ResolveBeforeWidening | Self::SubjectNotForecast => DescriptorGate::Blocked,
        }
    }
}

/// One claimed consumer surface that reads the compatibility-forecast sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastConsumer {
    /// The in-product update center's pre-restart surface.
    UpdateCenter,
    /// The migration assistant.
    MigrationAssistant,
    /// The release center / public-truth automation.
    ReleaseCenter,
    /// The admin console.
    AdminConsole,
    /// The support export.
    SupportExport,
}

impl ForecastConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UpdateCenter,
        Self::MigrationAssistant,
        Self::ReleaseCenter,
        Self::AdminConsole,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::MigrationAssistant => "migration_assistant",
            Self::ReleaseCenter => "release_center",
            Self::AdminConsole => "admin_console",
            Self::SupportExport => "support_export",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpdateCenter => "Update center",
            Self::MigrationAssistant => "Migration assistant",
            Self::ReleaseCenter => "Release center",
            Self::AdminConsole => "Admin console",
            Self::SupportExport => "Support export",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center_owner",
            Self::MigrationAssistant => "migration_assistant_owner",
            Self::ReleaseCenter => "release_center_owner",
            Self::AdminConsole => "admin_console_owner",
            Self::SupportExport => "support_export_owner",
        }
    }
}

/// The kind of inputs the forecast was generated from, labeled so partial coverage is honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastBasis {
    /// Forecast from release compatibility evidence only.
    ReleaseEvidenceOnly,
    /// Forecast from release evidence plus a local scan.
    ReleaseAndLocalScan,
    /// Forecast from a local scan only.
    LocalScanOnly,
    /// Forecast as a mirror-import preflight.
    MirrorImportPreflight,
    /// Forecast reconstructed for support review.
    SupportReconstruction,
}

impl ForecastBasis {
    /// Every basis, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseEvidenceOnly,
        Self::ReleaseAndLocalScan,
        Self::LocalScanOnly,
        Self::MirrorImportPreflight,
        Self::SupportReconstruction,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseEvidenceOnly => "release_evidence_only",
            Self::ReleaseAndLocalScan => "release_and_local_scan",
            Self::LocalScanOnly => "local_scan_only",
            Self::MirrorImportPreflight => "mirror_import_preflight",
            Self::SupportReconstruction => "support_reconstruction",
        }
    }
}

// ---------------------------------------------------------------------------
// Ranking helpers for deterministic ordering
// ---------------------------------------------------------------------------

fn subject_rank(s: CompatibilitySubject) -> usize {
    CompatibilitySubject::ALL
        .iter()
        .position(|x| *x == s)
        .unwrap_or(usize::MAX)
}

fn line_rank(l: CompatibilityLine) -> usize {
    CompatibilityLine::ALL
        .iter()
        .position(|x| *x == l)
        .unwrap_or(usize::MAX)
}

fn artifact_rank(c: ArtifactClass) -> usize {
    ArtifactClass::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn profile_rank(p: DeploymentProfile) -> usize {
    DeploymentProfile::ALL
        .iter()
        .position(|x| *x == p)
        .unwrap_or(usize::MAX)
}

fn action_rank(a: MigrationAction) -> usize {
    MigrationAction::ALL
        .iter()
        .position(|x| *x == a)
        .unwrap_or(usize::MAX)
}

fn consumer_rank(c: ForecastConsumer) -> usize {
    ForecastConsumer::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn gate_rank(g: DescriptorGate) -> u8 {
    match g {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

fn worst_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

/// Caps a gate at `cap`: returns the *less severe* of the two. This is how a speculative or
/// out-of-window confidence prevents a breaking-drift line from becoming a hard failure.
fn cap_gate(gate: DescriptorGate, cap: DescriptorGate) -> DescriptorGate {
    if gate_rank(gate) <= gate_rank(cap) {
        gate
    } else {
        cap
    }
}

fn status_for_gate(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

fn signal_for_gate(gate: DescriptorGate) -> DescriptorSignal {
    match gate {
        DescriptorGate::Governed => DescriptorSignal::Green,
        DescriptorGate::Narrowed => DescriptorSignal::Yellow,
        DescriptorGate::Blocked => DescriptorSignal::Red,
    }
}

/// The effective gate of one (drift, confidence) line forecast: the drift gate capped by the
/// confidence, so a speculative or out-of-window forecast can never become a hard failure.
fn line_gate(drift: DriftClass, confidence: ForecastConfidence) -> DescriptorGate {
    cap_gate(drift.drift_gate(), confidence.gate_cap())
}

// ---------------------------------------------------------------------------
// Line forecast
// ---------------------------------------------------------------------------

/// Builder input for [`LineForecast::new`].
#[derive(Debug, Clone)]
pub struct LineForecastInput {
    /// The line this forecast covers.
    pub line: CompatibilityLine,
    /// The forecast drift class.
    pub drift_class: DriftClass,
    /// The forecast confidence.
    pub confidence: ForecastConfidence,
    /// The supported version window's lower bound, if known.
    pub supported_from: Option<String>,
    /// The supported version window's upper bound, if known.
    pub supported_to: Option<String>,
}

/// The drift forecast for one [subject](CompatibilitySubject) on one [line](CompatibilityLine): the
/// drift class, the forecast confidence, the supported window, and the derived gate. The gate is the
/// [drift gate](DriftClass::drift_gate) *capped* by the [confidence](ForecastConfidence::gate_cap).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineForecast {
    /// The line.
    pub line: CompatibilityLine,
    /// The forecast drift class.
    pub drift_class: DriftClass,
    /// The forecast confidence.
    pub confidence: ForecastConfidence,
    /// True when the forecast rests on partial / absent inputs.
    pub speculative: bool,
    /// True when the subject is outside Aureline's claimed window on this line.
    pub outside_claimed_window: bool,
    /// The supported version window's lower bound.
    pub supported_from: Option<String>,
    /// The supported version window's upper bound.
    pub supported_to: Option<String>,
    /// Gate derived from the drift class capped by the confidence.
    pub gate: DescriptorGate,
    /// Readiness mirroring [`gate`](Self::gate).
    pub readiness: ForecastReadiness,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// Routable message id for this line's forecast.
    pub forecast_message_id: String,
}

impl LineForecast {
    /// Builds a line forecast from its inputs, deriving the gate, flags, and readiness.
    pub fn new(subject: CompatibilitySubject, input: LineForecastInput) -> Self {
        let gate = line_gate(input.drift_class, input.confidence);
        Self {
            line: input.line,
            drift_class: input.drift_class,
            confidence: input.confidence,
            speculative: input.confidence.is_speculative(),
            outside_claimed_window: input.confidence.is_outside_window(),
            supported_from: input.supported_from,
            supported_to: input.supported_to,
            gate,
            readiness: ForecastReadiness::from_gate(gate),
            signal: signal_for_gate(gate),
            forecast_message_id: format!(
                "{}subject.{}.line.{}.forecast",
                M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                subject.as_str(),
                input.line.as_str(),
            ),
        }
    }

    fn recompute(&mut self, subject: CompatibilitySubject) {
        let gate = line_gate(self.drift_class, self.confidence);
        self.speculative = self.confidence.is_speculative();
        self.outside_claimed_window = self.confidence.is_outside_window();
        self.gate = gate;
        self.readiness = ForecastReadiness::from_gate(gate);
        self.signal = signal_for_gate(gate);
        self.forecast_message_id = format!(
            "{}subject.{}.line.{}.forecast",
            M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
            subject.as_str(),
            self.line.as_str(),
        );
    }
}

// ---------------------------------------------------------------------------
// Subject forecast
// ---------------------------------------------------------------------------

/// Builder input for [`SubjectForecast::new`].
#[derive(Debug, Clone)]
pub struct SubjectForecastInput {
    /// The subject family this forecast covers.
    pub subject: CompatibilitySubject,
    /// A stable identifier for the concrete subject instance (e.g. an archetype id).
    pub subject_id: String,
    /// Whether Aureline claims a compatibility window for this subject.
    pub within_claimed_window: bool,
    /// The per-line drift forecasts.
    pub line_forecasts: Vec<LineForecast>,
    /// Artifact classes the drift affects (the primary class is always added).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// Deployment profiles the drift affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the forecast (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// The compatibility forecast for one [subject](CompatibilitySubject): the per-line drift forecasts,
/// the affected scope, and the derived worst-line verdict. A subject's worst-line gate decides whether
/// it is clear, narrowed, or held — and every narrowed / held subject MUST carry a migration task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectForecast {
    /// The subject family.
    pub subject: CompatibilitySubject,
    /// Human-facing subject label.
    pub subject_label: String,
    /// A stable identifier for the concrete subject instance.
    pub subject_id: String,
    /// The subject's primary artifact class.
    pub primary_artifact_class: ArtifactClass,
    /// Accountable owner role.
    pub owner_role: String,
    /// Whether Aureline claims a compatibility window for this subject.
    pub within_claimed_window: bool,
    /// The per-line drift forecasts, one per covered line.
    pub line_forecasts: Vec<LineForecast>,
    /// The union of artifact classes the drift affects (always includes the primary class).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// The deployment profiles the drift affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the forecast.
    pub evidence_refs: Vec<String>,
    /// True when any covered line rests on speculative inputs.
    pub speculative: bool,
    /// The worst gate across the covered lines.
    pub worst_gate: DescriptorGate,
    /// Review readiness mirroring [`worst_gate`](Self::worst_gate).
    pub readiness: ForecastReadiness,
    /// Coverage status mirroring [`worst_gate`](Self::worst_gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`worst_gate`](Self::worst_gate).
    pub signal: DescriptorSignal,
    /// True when at least one covered line is a confirmed breaking drift that must be resolved.
    pub requires_pre_rollout_resolution: bool,
    /// True when the subject narrows or holds and therefore MUST carry a migration task.
    pub requires_migration_task: bool,
    /// Set only when the subject is outside Aureline's claimed window: a routable message id naming the
    /// narrowed coverage, so it is labeled honestly rather than presented as authoritative.
    pub out_of_window_message_id: Option<String>,
    /// Routable message id for the subject's summary line.
    pub summary_message_id: String,
    /// Routable message id for the subject's detail.
    pub detail_message_id: String,
}

impl SubjectForecast {
    /// Builds a subject forecast from its inputs, deriving the worst-line gate, scope, and flags.
    pub fn new(input: SubjectForecastInput) -> Self {
        let subject = input.subject;
        let mut forecast = Self {
            subject,
            subject_label: subject.label().to_owned(),
            subject_id: input.subject_id,
            primary_artifact_class: subject.primary_artifact_class(),
            owner_role: subject.owner_role().to_owned(),
            within_claimed_window: input.within_claimed_window,
            line_forecasts: input.line_forecasts,
            affected_artifact_classes: input.affected_artifact_classes,
            affected_profiles: input.affected_profiles,
            evidence_refs: input.evidence_refs,
            speculative: false,
            worst_gate: DescriptorGate::Governed,
            readiness: ForecastReadiness::ClearToWiden,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            requires_pre_rollout_resolution: false,
            requires_migration_task: false,
            out_of_window_message_id: None,
            summary_message_id: format!(
                "{}subject.{}.summary",
                M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                subject.as_str(),
            ),
            detail_message_id: format!(
                "{}subject.{}.detail",
                M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                subject.as_str(),
            ),
        };
        forecast.recompute();
        forecast
    }

    /// Recomputes the disclosed scope and derived verdict from the line forecasts. The disclosed
    /// artifact classes are the union of the affected classes plus the primary class; the worst gate is
    /// the most severe line gate; the out-of-window message id is set when coverage is not claimed.
    pub fn recompute(&mut self) {
        let subject = self.subject;
        for line in &mut self.line_forecasts {
            line.recompute(subject);
        }
        self.line_forecasts.sort_by_key(|l| line_rank(l.line));

        let mut classes = vec![self.primary_artifact_class];
        classes.extend(self.affected_artifact_classes.iter().copied());
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        self.affected_artifact_classes = classes;

        let mut profiles = self.affected_profiles.clone();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        self.affected_profiles = profiles;

        let mut gate = DescriptorGate::Governed;
        for line in &self.line_forecasts {
            gate = worst_gate(gate, line.gate);
        }
        self.worst_gate = gate;
        self.readiness = ForecastReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_pre_rollout_resolution = gate == DescriptorGate::Blocked;
        self.requires_migration_task = gate != DescriptorGate::Governed;
        self.speculative = self.line_forecasts.iter().any(|l| l.speculative);

        self.out_of_window_message_id = if !self.within_claimed_window {
            Some(format!(
                "{}subject.{}.out_of_window",
                M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                subject.as_str(),
            ))
        } else {
            None
        };
    }

    /// The gap kind this subject contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<ForecastGapKind> {
        match self.worst_gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(if !self.within_claimed_window {
                ForecastGapKind::OutsideClaimedWindow
            } else if self.speculative {
                ForecastGapKind::ForecastInputUnknown
            } else {
                ForecastGapKind::ReviewBeforeWidening
            }),
            DescriptorGate::Blocked => Some(ForecastGapKind::ResolveBeforeWidening),
        }
    }
}

// ---------------------------------------------------------------------------
// Migration task row
// ---------------------------------------------------------------------------

/// A recorded waiver suppressing a migration task. When the task's [skip policy](SkipPolicy) requires
/// a rationale, the waiver MUST carry one — enforced in validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWaiver {
    /// Whether the task is waived.
    pub waived: bool,
    /// The recorded rationale, required when the policy demands one.
    pub rationale: Option<String>,
    /// The role that recorded the waiver.
    pub waived_by_role: Option<String>,
    /// Routable message id for the waiver.
    pub waiver_message_id: String,
}

/// Builder input for [`MigrationTaskRow::new`].
#[derive(Debug, Clone)]
pub struct MigrationTaskRowInput {
    /// A stable identifier for the task.
    pub task_id: String,
    /// The subject family this task clears drift for.
    pub subject: CompatibilitySubject,
    /// The migration class.
    pub task_class: MigrationTaskClass,
    /// The drift the task addresses (its worst line drift).
    pub addresses_drift: DriftClass,
    /// The confidence of the addressed drift.
    pub confidence: ForecastConfidence,
    /// Artifact classes the task affects (the subject's primary class is always added).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// Deployment profiles the task affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// The lines the task applies to.
    pub affected_lines: Vec<CompatibilityLine>,
    /// How much of the task Aureline can automate.
    pub auto_fix: AutoFixAvailability,
    /// The boundary the task must complete before.
    pub due_before: DueBoundary,
    /// Whether and how the task may be skipped.
    pub skip_policy: SkipPolicy,
    /// The rollback path the task discloses.
    pub rollback_guidance: RollbackGuidance,
    /// The pre-emptive actions the task surfaces.
    pub available_actions: Vec<MigrationAction>,
    /// An optional recorded waiver.
    pub waiver: Option<MigrationWaiver>,
    /// Opaque evidence refs backing the task (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// A typed migration-assistant task row: the actionable step that clears a subject's drift, with its
/// owner, affected scope, auto-fix availability, due-before boundary, skip / waive policy, rollback
/// guidance, and available actions. The row's gate is the addressed [drift gate](DriftClass::drift_gate)
/// capped by the [confidence](ForecastConfidence::gate_cap), so a task addressing a breaking drift on a
/// speculative or out-of-window subject is flagged for review, never raised as a hard failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationTaskRow {
    /// A stable identifier for the task.
    pub task_id: String,
    /// The subject family this task clears drift for.
    pub subject: CompatibilitySubject,
    /// Human-facing subject label.
    pub subject_label: String,
    /// The migration class.
    pub task_class: MigrationTaskClass,
    /// Accountable owner role.
    pub owner_role: String,
    /// The drift the task addresses.
    pub addresses_drift: DriftClass,
    /// The confidence of the addressed drift.
    pub confidence: ForecastConfidence,
    /// True when the task rests on speculative inputs.
    pub speculative: bool,
    /// The union of artifact classes the task affects (always includes the subject's primary class).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// The deployment profiles the task affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// The lines the task applies to.
    pub affected_lines: Vec<CompatibilityLine>,
    /// How much of the task Aureline can automate.
    pub auto_fix: AutoFixAvailability,
    /// The boundary the task must complete before.
    pub due_before: DueBoundary,
    /// Whether and how the task may be skipped.
    pub skip_policy: SkipPolicy,
    /// True when suppressing the task requires a recorded rationale.
    pub requires_recorded_rationale: bool,
    /// The rollback path the task discloses.
    pub rollback_guidance: RollbackGuidance,
    /// The pre-emptive actions the task surfaces.
    pub available_actions: Vec<MigrationAction>,
    /// An optional recorded waiver.
    pub waiver: Option<MigrationWaiver>,
    /// Opaque evidence refs backing the task.
    pub evidence_refs: Vec<String>,
    /// Gate derived from the addressed drift capped by the confidence.
    pub gate: DescriptorGate,
    /// Review readiness mirroring [`gate`](Self::gate).
    pub readiness: ForecastReadiness,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// Routable message id for the task's summary line.
    pub summary_message_id: String,
    /// Routable message id for the task's detail.
    pub detail_message_id: String,
}

impl MigrationTaskRow {
    /// Builds a task row from its inputs, deriving the gate, scope, and skip semantics.
    pub fn new(input: MigrationTaskRowInput) -> Self {
        let subject = input.subject;
        let mut row = Self {
            task_id: input.task_id,
            subject,
            subject_label: subject.label().to_owned(),
            task_class: input.task_class,
            owner_role: subject.owner_role().to_owned(),
            addresses_drift: input.addresses_drift,
            confidence: input.confidence,
            speculative: false,
            affected_artifact_classes: input.affected_artifact_classes,
            affected_profiles: input.affected_profiles,
            affected_lines: input.affected_lines,
            auto_fix: input.auto_fix,
            due_before: input.due_before,
            skip_policy: input.skip_policy,
            requires_recorded_rationale: false,
            rollback_guidance: input.rollback_guidance,
            available_actions: input.available_actions,
            waiver: input.waiver,
            evidence_refs: input.evidence_refs,
            gate: DescriptorGate::Governed,
            readiness: ForecastReadiness::ClearToWiden,
            signal: DescriptorSignal::Green,
            summary_message_id: String::new(),
            detail_message_id: String::new(),
        };
        row.recompute();
        row
    }

    /// Recomputes the disclosed scope, gate, and skip semantics from the task's inputs.
    pub fn recompute(&mut self) {
        let subject = self.subject;

        let mut classes = vec![subject.primary_artifact_class()];
        classes.extend(self.affected_artifact_classes.iter().copied());
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        self.affected_artifact_classes = classes;

        let mut profiles = self.affected_profiles.clone();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        self.affected_profiles = profiles;

        let mut lines = self.affected_lines.clone();
        lines.sort_by_key(|l| line_rank(*l));
        lines.dedup();
        self.affected_lines = lines;

        let mut actions = self.available_actions.clone();
        actions.sort_by_key(|a| action_rank(*a));
        actions.dedup();
        self.available_actions = actions;

        self.speculative = self.confidence.is_speculative();
        self.requires_recorded_rationale = self.skip_policy.requires_recorded_rationale();

        let gate = line_gate(self.addresses_drift, self.confidence);
        self.gate = gate;
        self.readiness = ForecastReadiness::from_gate(gate);
        self.signal = signal_for_gate(gate);

        self.summary_message_id = format!(
            "{}task.{}.{}.summary",
            M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
            subject.as_str(),
            self.task_class.as_str(),
        );
        self.detail_message_id = format!(
            "{}task.{}.{}.detail",
            M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
            subject.as_str(),
            self.task_class.as_str(),
        );
    }

    /// True when the task is currently waived.
    pub fn is_waived(&self) -> bool {
        self.waiver.as_ref().is_some_and(|w| w.waived)
    }

    /// The waiver violations this row carries, if any: an illegal waiver of a non-skippable task, or a
    /// rationale missing where the policy requires one.
    fn waiver_violation(&self) -> Option<CompatibilityForecastViolation> {
        let Some(waiver) = &self.waiver else {
            return None;
        };
        if !waiver.waived {
            return None;
        }
        if !self.skip_policy.is_waivable() {
            return Some(CompatibilityForecastViolation::IllegalWaiver);
        }
        if self.requires_recorded_rationale
            && waiver
                .rationale
                .as_ref()
                .map_or(true, |r| r.trim().is_empty())
        {
            return Some(CompatibilityForecastViolation::WaiverRationaleMissing);
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Consumer rows
// ---------------------------------------------------------------------------

/// A review gap a consumer carries for one subject it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastGap {
    /// The consumer that carries the gap.
    pub consumer: ForecastConsumer,
    /// The subject whose forecast caused the gap.
    pub subject: CompatibilitySubject,
    /// The subject's primary artifact class.
    pub artifact_class: ArtifactClass,
    /// The named cause of the gap.
    pub gap_kind: ForecastGapKind,
    /// Routable message id naming the cause.
    pub cause_message_id: String,
}

/// A consumer surface bound to the subject families it reads, with its review readiness, decision, and
/// gaps derived from those subjects' forecasts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastConsumerRow {
    /// The consumer surface.
    pub consumer: ForecastConsumer,
    /// Human-facing label.
    pub consumer_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The subject families this consumer reads.
    pub read_subjects: Vec<CompatibilitySubject>,
    /// The union of artifact classes disclosed across the read subjects.
    pub disclosed_artifact_classes: Vec<ArtifactClass>,
    /// The union of profiles across the read subjects.
    pub profiles: Vec<DeploymentProfile>,
    /// The derived review readiness.
    pub readiness: ForecastReadiness,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Gate decision.
    pub gate_decision: DescriptorGate,
    /// True when at least one read subject is a confirmed breaking drift.
    pub requires_pre_rollout_resolution: bool,
    /// Review gaps, one per (subject, cause).
    pub gaps: Vec<ForecastGap>,
    /// Routable status message id.
    pub status_message_id: String,
    /// Routable decision message id.
    pub decision_message_id: String,
}

impl ForecastConsumerRow {
    /// Builds a consumer row; the resolved unions, gaps, and verdict are recomputed against the
    /// packet's subjects when the packet is assembled.
    pub fn new(consumer: ForecastConsumer, read_subjects: &[CompatibilitySubject]) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_subjects: read_subjects.to_vec(),
            disclosed_artifact_classes: Vec::new(),
            profiles: Vec::new(),
            readiness: ForecastReadiness::ClearToWiden,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            requires_pre_rollout_resolution: false,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                consumer.as_str(),
            ),
            decision_message_id: format!(
                "{}consumer.{}.decision",
                M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                consumer.as_str(),
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's subjects, so a consumer's
    /// review verdict is always generated from the same checked-in forecasts rather than a
    /// hand-maintained status.
    pub fn recompute(&mut self, subjects: &[SubjectForecast]) {
        let mut read = self.read_subjects.clone();
        read.sort_by_key(|s| subject_rank(*s));
        read.dedup();
        self.read_subjects = read.clone();

        let forecast_for = |subject: CompatibilitySubject| -> Option<&SubjectForecast> {
            subjects.iter().find(|s| s.subject == subject)
        };

        let mut classes: Vec<ArtifactClass> = Vec::new();
        let mut profiles: Vec<DeploymentProfile> = Vec::new();
        let mut gaps: Vec<ForecastGap> = Vec::new();
        let consumer = self.consumer;
        for &subject in &read {
            match forecast_for(subject) {
                None => {
                    gaps.push(make_gap(
                        consumer,
                        subject,
                        ForecastGapKind::SubjectNotForecast,
                    ));
                }
                Some(forecast) => {
                    classes.extend(forecast.affected_artifact_classes.iter().copied());
                    profiles.extend(forecast.affected_profiles.iter().copied());
                    if let Some(kind) = forecast.gap_kind() {
                        gaps.push(make_gap(consumer, subject, kind));
                    }
                }
            }
        }
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        gaps.sort_by(|a, b| {
            subject_rank(a.subject)
                .cmp(&subject_rank(b.subject))
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        self.disclosed_artifact_classes = classes;
        self.profiles = profiles;
        self.gaps = gaps;

        let mut gate = DescriptorGate::Governed;
        for gap in &self.gaps {
            gate = worst_gate(gate, gap.gap_kind.gate());
        }
        self.gate_decision = gate;
        self.readiness = ForecastReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_pre_rollout_resolution = gate == DescriptorGate::Blocked;
    }

    /// True when the consumer reads every subject as clear to widen.
    pub fn is_clear(&self) -> bool {
        self.gate_decision == DescriptorGate::Governed
    }

    /// True when at least one read subject narrows the consumer to a review-recommended state.
    pub fn is_review(&self) -> bool {
        self.gate_decision == DescriptorGate::Narrowed
    }

    /// True when at least one read subject holds the consumer for resolution.
    pub fn is_hold(&self) -> bool {
        self.gate_decision == DescriptorGate::Blocked
    }
}

fn make_gap(
    consumer: ForecastConsumer,
    subject: CompatibilitySubject,
    kind: ForecastGapKind,
) -> ForecastGap {
    ForecastGap {
        consumer,
        subject,
        artifact_class: subject.primary_artifact_class(),
        gap_kind: kind,
        cause_message_id: format!(
            "{}consumer.{}.{}.{}.gap",
            M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
            consumer.as_str(),
            subject.as_str(),
            kind.as_str(),
        ),
    }
}

// ---------------------------------------------------------------------------
// Aggregate sub-objects
// ---------------------------------------------------------------------------

/// The staged update the forecast sheet covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastTarget {
    /// The channel the staged update is on.
    pub channel: ChannelScope,
    /// The compatibility lines the forecast covers.
    pub lines: Vec<CompatibilityLine>,
    /// The deployment profiles the staged update covers.
    pub profiles: Vec<DeploymentProfile>,
    /// The currently installed version.
    pub current_version: String,
    /// The version the staged update moves to.
    pub target_version: String,
    /// The basis of the forecast, labeled honestly.
    pub forecast_basis: ForecastBasis,
}

/// Disclosure flags asserting every claimed consumer ingests this one forecast sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastDisclosure {
    /// The update center consumes the sheet.
    pub update_center_consumes_sheet: bool,
    /// The migration assistant consumes the sheet.
    pub migration_assistant_consumes_sheet: bool,
    /// The release center consumes the sheet.
    pub release_center_consumes_sheet: bool,
    /// The admin console consumes the sheet.
    pub admin_console_consumes_sheet: bool,
    /// The support export consumes the sheet.
    pub support_export_consumes_sheet: bool,
}

impl ForecastDisclosure {
    fn canonical() -> Self {
        Self {
            update_center_consumes_sheet: true,
            migration_assistant_consumes_sheet: true,
            release_center_consumes_sheet: true,
            admin_console_consumes_sheet: true,
            support_export_consumes_sheet: true,
        }
    }

    /// True when every consumer is asserted to consume the sheet.
    pub fn all_consume(&self) -> bool {
        self.update_center_consumes_sheet
            && self.migration_assistant_consumes_sheet
            && self.release_center_consumes_sheet
            && self.admin_console_consumes_sheet
            && self.support_export_consumes_sheet
    }
}

/// Roll-up counts over the subjects, tasks, and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastCounts {
    /// Total subjects.
    pub total_subjects: u32,
    /// Subjects clear to widen (governed).
    pub clear_subjects: u32,
    /// Subjects needing review (narrowed).
    pub review_subjects: u32,
    /// Subjects held for resolution (blocked).
    pub hold_subjects: u32,
    /// Subjects forecast outside Aureline's claimed window.
    pub out_of_window_subjects: u32,
    /// Subjects whose forecast is speculative.
    pub speculative_subjects: u32,
    /// Total migration tasks.
    pub total_tasks: u32,
    /// Tasks Aureline can drive automatically (auto-fix or assisted).
    pub automatable_tasks: u32,
    /// Tasks requiring manual or admin steps.
    pub manual_tasks: u32,
    /// Tasks suppressible only with a recorded rationale.
    pub rationale_gated_tasks: u32,
    /// Tasks currently waived.
    pub waived_tasks: u32,
    /// Total consumers.
    pub total_consumers: u32,
    /// Consumers clear to widen.
    pub clear_consumers: u32,
    /// Consumers needing review.
    pub review_consumers: u32,
    /// Consumers held for resolution.
    pub hold_consumers: u32,
    /// Whether the sheet requires a pre-rollout resolution.
    pub requires_pre_rollout_resolution: bool,
}

/// The packet-level forecast-coverage honesty block: how much of the forecast is fully grounded vs.
/// speculative, out-of-window, or not-applicable, so partial coverage is disclosed rather than implied
/// complete. Counts are over line forecasts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastCoverage {
    /// Lines forecast with qualified / likely confidence.
    pub qualified_lines: u32,
    /// Lines forecast with estimated confidence.
    pub estimated_lines: u32,
    /// Lines with unknown inputs.
    pub unknown_lines: u32,
    /// Lines forecast outside Aureline's claimed window.
    pub outside_window_lines: u32,
    /// Lines the subject does not apply to.
    pub not_applicable_lines: u32,
    /// True when at least one line rests on speculative or out-of-window inputs.
    pub has_partial_coverage: bool,
}

/// The packet-level pre-rollout review gate aggregating the per-consumer decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastReleaseGate {
    /// Whether any consumer is held for resolution before rollout widening.
    pub requires_pre_rollout_resolution: bool,
    /// Tokens of the held consumers.
    pub hold_consumers: Vec<String>,
    /// Tokens of the review-recommended consumers.
    pub review_consumers: Vec<String>,
    /// Tokens of the clear consumers.
    pub clear_consumers: Vec<String>,
    /// Tokens of the subjects that contributed a gap.
    pub affected_subjects: Vec<String>,
    /// Routable gate message id.
    pub gate_message_id: String,
}

/// The frozen controlled vocabulary the forecast draws from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastVocabulary {
    /// Subject-family tokens.
    pub subjects: Vec<String>,
    /// Compatibility-line tokens.
    pub lines: Vec<String>,
    /// Drift-class tokens.
    pub drift_classes: Vec<String>,
    /// Forecast-confidence tokens.
    pub confidence_levels: Vec<String>,
    /// Review-readiness tokens.
    pub review_readiness: Vec<String>,
    /// Migration-task-class tokens.
    pub task_classes: Vec<String>,
    /// Auto-fix-availability tokens.
    pub auto_fix_levels: Vec<String>,
    /// Due-boundary tokens.
    pub due_boundaries: Vec<String>,
    /// Skip-policy tokens.
    pub skip_policies: Vec<String>,
    /// Rollback-guidance tokens.
    pub rollback_guidances: Vec<String>,
    /// Migration-action tokens.
    pub actions: Vec<String>,
    /// Artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Profile tokens.
    pub profiles: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Forecast-basis tokens.
    pub forecast_bases: Vec<String>,
}

impl ForecastVocabulary {
    /// The canonical frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            subjects: tokens(&CompatibilitySubject::ALL, |x| x.as_str()),
            lines: tokens(&CompatibilityLine::ALL, |x| x.as_str()),
            drift_classes: tokens(&DriftClass::ALL, |x| x.as_str()),
            confidence_levels: tokens(&ForecastConfidence::ALL, |x| x.as_str()),
            review_readiness: tokens(&ForecastReadiness::ALL, |x| x.as_str()),
            task_classes: tokens(&MigrationTaskClass::ALL, |x| x.as_str()),
            auto_fix_levels: tokens(&AutoFixAvailability::ALL, |x| x.as_str()),
            due_boundaries: tokens(&DueBoundary::ALL, |x| x.as_str()),
            skip_policies: tokens(&SkipPolicy::ALL, |x| x.as_str()),
            rollback_guidances: tokens(&RollbackGuidance::ALL, |x| x.as_str()),
            actions: tokens(&MigrationAction::ALL, |x| x.as_str()),
            artifact_classes: tokens(&ArtifactClass::ALL, |x| x.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |x| x.as_str()),
            consumers: tokens(&ForecastConsumer::ALL, |x| x.as_str()),
            gap_kinds: tokens(&ForecastGapKind::ALL, |x| x.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |x| x.as_str()),
            forecast_bases: tokens(&ForecastBasis::ALL, |x| x.as_str()),
        }
    }

    /// True when this vocabulary equals the canonical frozen vocabulary.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance flags every canonical forecast sheet asserts. They restate the acceptance bar so a
/// tampered packet that flips one to false fails [`CompatibilityForecastSheet::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastConformance {
    /// Every subject family is forecast exactly once.
    pub every_subject_forecast: bool,
    /// A drift class is disclosed on every line forecast.
    pub drift_class_disclosed_per_line: bool,
    /// Compatible forecasts are distinguished from breaking drift.
    pub compatible_distinguished_from_breaking: bool,
    /// Forecast confidence is labeled on every line.
    pub forecast_confidence_labelled: bool,
    /// Out-of-window subjects are labeled, never raised as a hard failure.
    pub out_of_window_labelled_not_failed: bool,
    /// Speculative inputs are labeled, never raised as a hard failure.
    pub speculative_inputs_labelled_not_failed: bool,
    /// Every narrowed / held subject carries a migration task.
    pub narrowed_subjects_have_migration_task: bool,
    /// Every migration task discloses owner, scope, auto-fix, due-before, and rollback.
    pub migration_tasks_actionable: bool,
    /// Waivers require a recorded rationale where the policy demands it.
    pub waivers_require_recorded_rationale: bool,
    /// Rollback guidance is disclosed on every migration task.
    pub rollback_guidance_disclosed: bool,
    /// The affected artifact-class / profile scope is disclosed on every subject.
    pub affected_scope_disclosed: bool,
    /// The forecast is computed and visible before rollout widening.
    pub visible_before_rollout_widening: bool,
    /// The sheet is exportable and reviewable outside the app.
    pub exportable_outside_app: bool,
    /// Every claimed consumer reads this one sheet.
    pub consumers_read_one_sheet: bool,
    /// Every consumer verdict is derived from the subjects, not hand-maintained.
    pub consumer_verdict_derived_from_subjects: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The export carries metadata and refs only — no credential bodies or raw payloads.
    pub export_carries_no_raw_material: bool,
}

impl ForecastConformance {
    fn canonical() -> Self {
        Self {
            every_subject_forecast: true,
            drift_class_disclosed_per_line: true,
            compatible_distinguished_from_breaking: true,
            forecast_confidence_labelled: true,
            out_of_window_labelled_not_failed: true,
            speculative_inputs_labelled_not_failed: true,
            narrowed_subjects_have_migration_task: true,
            migration_tasks_actionable: true,
            waivers_require_recorded_rationale: true,
            rollback_guidance_disclosed: true,
            affected_scope_disclosed: true,
            visible_before_rollout_widening: true,
            exportable_outside_app: true,
            consumers_read_one_sheet: true,
            consumer_verdict_derived_from_subjects: true,
            controlled_enums_frozen: true,
            export_carries_no_raw_material: true,
        }
    }

    /// True when every conformance flag holds.
    pub fn all_hold(&self) -> bool {
        *self == Self::canonical()
    }
}

// ---------------------------------------------------------------------------
// Render channel
// ---------------------------------------------------------------------------

/// The render channels the packet must serialize identically across.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForecastChannel {
    /// The desktop update center / migration assistant.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// The offline / exported review surface.
    OfflineExport,
}

// ---------------------------------------------------------------------------
// Validation violations
// ---------------------------------------------------------------------------

/// A reason a forecast sheet failed [`CompatibilityForecastSheet::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityForecastViolation {
    /// The record kind or schema version is wrong.
    HeaderDrift,
    /// A subject family is missing or forecast more than once.
    SubjectCoverageDrift,
    /// A subject's derived verdict, scope, or flags drifted.
    SubjectDerivationDrift,
    /// A speculative or out-of-window forecast was raised to a hard failure — the lane's guardrail.
    SpeculativeHardFailure,
    /// A migration task's derived gate, scope, or skip semantics drifted.
    MigrationTaskDerivationDrift,
    /// A narrowed / held subject has no migration task.
    MissingMigrationTask,
    /// A non-skippable task was waived.
    IllegalWaiver,
    /// A waiver of a rationale-gated task carries no recorded rationale.
    WaiverRationaleMissing,
    /// A consumer's derived verdict, unions, or gaps drifted.
    ConsumerVerdictDrift,
    /// The summary counts, coverage, or release gate drifted.
    SummaryDrift,
    /// The disclosure flags do not all assert consumption of the one sheet.
    DisclosureDrift,
    /// The controlled vocabulary drifted.
    VocabularyDrift,
    /// A conformance flag does not hold.
    ConformanceDrift,
    /// The export carried forbidden raw material.
    ForbiddenMaterial,
}

impl CompatibilityForecastViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderDrift => "header_drift",
            Self::SubjectCoverageDrift => "subject_coverage_drift",
            Self::SubjectDerivationDrift => "subject_derivation_drift",
            Self::SpeculativeHardFailure => "speculative_hard_failure",
            Self::MigrationTaskDerivationDrift => "migration_task_derivation_drift",
            Self::MissingMigrationTask => "missing_migration_task",
            Self::IllegalWaiver => "illegal_waiver",
            Self::WaiverRationaleMissing => "waiver_rationale_missing",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::DisclosureDrift => "disclosure_drift",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::ConformanceDrift => "conformance_drift",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Builder input for [`CompatibilityForecastSheet::new`].
#[derive(Debug, Clone)]
pub struct CompatibilityForecastSheetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The staged update the forecast covers.
    pub target: ForecastTarget,
    /// The per-subject forecasts.
    pub subjects: Vec<SubjectForecast>,
    /// The migration-assistant task rows.
    pub migration_tasks: Vec<MigrationTaskRow>,
    /// The claimed consumer rows.
    pub consumers: Vec<ForecastConsumerRow>,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable compatibility-forecast sheet the update center, migration
/// assistant, release center, admin console, and support export consume before restart or rollout
/// widening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityForecastSheet {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The staged update the forecast covers.
    pub target: ForecastTarget,
    /// The per-subject forecasts.
    pub subjects: Vec<SubjectForecast>,
    /// The subject-family tokens this sheet covers.
    pub subject_families: Vec<String>,
    /// The migration-assistant task rows.
    pub migration_tasks: Vec<MigrationTaskRow>,
    /// The consumer rows reading the forecasts.
    pub consumers: Vec<ForecastConsumerRow>,
    /// The consumer tokens, in canonical order.
    pub consumer_tokens: Vec<String>,
    /// Disclosure flags.
    pub disclosure: ForecastDisclosure,
    /// Roll-up counts.
    pub summary: ForecastCounts,
    /// Forecast-coverage honesty block.
    pub coverage: ForecastCoverage,
    /// Packet-level pre-rollout review gate.
    pub release_gate: ForecastReleaseGate,
    /// Controlled vocabulary.
    pub vocabulary: ForecastVocabulary,
    /// Conformance flags.
    pub conformance: ForecastConformance,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl CompatibilityForecastSheet {
    /// Builds a packet from the given subjects, tasks, and consumer rows, recomputing every derived
    /// field so the published packet is always generated from the same checked-in forecasts.
    pub fn new(input: CompatibilityForecastSheetInput) -> Self {
        let mut subjects = input.subjects;
        for subject in &mut subjects {
            subject.recompute();
        }
        subjects.sort_by_key(|s| subject_rank(s.subject));

        let mut migration_tasks = input.migration_tasks;
        for task in &mut migration_tasks {
            task.recompute();
        }
        migration_tasks.sort_by(|a, b| {
            subject_rank(a.subject)
                .cmp(&subject_rank(b.subject))
                .then(a.task_id.cmp(&b.task_id))
        });

        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&subjects);
        }
        consumers.sort_by_key(|c| consumer_rank(c.consumer));

        let mut target = input.target;
        target.lines.sort_by_key(|l| line_rank(*l));
        target.lines.dedup();
        target.profiles.sort_by_key(|p| profile_rank(*p));
        target.profiles.dedup();

        let summary = derive_counts(&subjects, &migration_tasks, &consumers);
        let coverage = derive_coverage(&subjects);
        let release_gate = derive_release_gate(&consumers);

        Self {
            record_kind: M5_COMPATIBILITY_FORECAST_RECORD_KIND.to_owned(),
            schema_version: M5_COMPATIBILITY_FORECAST_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            target,
            subject_families: tokens(&CompatibilitySubject::ALL, |x| x.as_str()),
            subjects,
            migration_tasks,
            consumer_tokens: tokens(&ForecastConsumer::ALL, |x| x.as_str()),
            consumers,
            disclosure: ForecastDisclosure::canonical(),
            summary,
            coverage,
            release_gate,
            vocabulary: ForecastVocabulary::canonical(),
            conformance: ForecastConformance::canonical(),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Looks up the forecast for a subject family.
    pub fn subject(&self, subject: CompatibilitySubject) -> Option<&SubjectForecast> {
        self.subjects.iter().find(|s| s.subject == subject)
    }

    /// The migration tasks bound to a subject family.
    pub fn tasks_for(&self, subject: CompatibilitySubject) -> Vec<&MigrationTaskRow> {
        self.migration_tasks
            .iter()
            .filter(|t| t.subject == subject)
            .collect()
    }

    /// Looks up the consumer row for a consumer.
    pub fn consumer(&self, consumer: ForecastConsumer) -> Option<&ForecastConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Whether the sheet requires a pre-rollout resolution.
    pub fn requires_pre_rollout_resolution(&self) -> bool {
        self.release_gate.requires_pre_rollout_resolution
    }

    /// Validates every derived field by recomputing it from the forecasts and comparing. Returns an
    /// empty vector when the packet is internally consistent.
    pub fn validate(&self) -> Vec<CompatibilityForecastViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPATIBILITY_FORECAST_RECORD_KIND
            || self.schema_version != M5_COMPATIBILITY_FORECAST_SCHEMA_VERSION
        {
            violations.push(CompatibilityForecastViolation::HeaderDrift);
        }

        // Every subject forecast exactly once.
        for subject in CompatibilitySubject::ALL {
            let count = self
                .subjects
                .iter()
                .filter(|s| s.subject == subject)
                .count();
            if count != 1 {
                violations.push(CompatibilityForecastViolation::SubjectCoverageDrift);
                break;
            }
        }

        for subject in &self.subjects {
            // Recompute the subject from its inputs and compare the derived verdict.
            let mut fresh = subject.clone();
            fresh.recompute();
            if fresh != *subject {
                violations.push(CompatibilityForecastViolation::SubjectDerivationDrift);
            }
            // Guardrail: a speculative / out-of-window forecast can never be a hard failure.
            for line in &subject.line_forecasts {
                if line.confidence.caps_below_blocked() && line.gate == DescriptorGate::Blocked {
                    violations.push(CompatibilityForecastViolation::SpeculativeHardFailure);
                }
            }
            // The primary class must always be disclosed.
            if !subject
                .affected_artifact_classes
                .contains(&subject.primary_artifact_class)
            {
                violations.push(CompatibilityForecastViolation::SubjectDerivationDrift);
            }
            // Every narrowed / held subject must carry a migration task.
            if subject.requires_migration_task
                && !self
                    .migration_tasks
                    .iter()
                    .any(|t| t.subject == subject.subject)
            {
                violations.push(CompatibilityForecastViolation::MissingMigrationTask);
            }
        }

        for task in &self.migration_tasks {
            let mut fresh = task.clone();
            fresh.recompute();
            if fresh != *task {
                violations.push(CompatibilityForecastViolation::MigrationTaskDerivationDrift);
            }
            if task.confidence.caps_below_blocked() && task.gate == DescriptorGate::Blocked {
                violations.push(CompatibilityForecastViolation::SpeculativeHardFailure);
            }
            if let Some(v) = task.waiver_violation() {
                violations.push(v);
            }
            // A task must bind to a forecast subject.
            if self.subject(task.subject).is_none() {
                violations.push(CompatibilityForecastViolation::MigrationTaskDerivationDrift);
            }
        }

        // Consumers: recompute and compare verdict, unions, and gaps.
        for consumer in &self.consumers {
            let mut fresh = ForecastConsumerRow::new(consumer.consumer, &consumer.read_subjects);
            fresh.recompute(&self.subjects);
            if fresh != *consumer {
                violations.push(CompatibilityForecastViolation::ConsumerVerdictDrift);
                break;
            }
        }

        if self.summary != derive_counts(&self.subjects, &self.migration_tasks, &self.consumers)
            || self.coverage != derive_coverage(&self.subjects)
            || self.release_gate != derive_release_gate(&self.consumers)
        {
            violations.push(CompatibilityForecastViolation::SummaryDrift);
        }

        if !self.disclosure.all_consume()
            || self.consumer_tokens != tokens(&ForecastConsumer::ALL, |x| x.as_str())
            || self.subject_families != tokens(&CompatibilitySubject::ALL, |x| x.as_str())
        {
            violations.push(CompatibilityForecastViolation::DisclosureDrift);
        }

        if !self.vocabulary.matches_canonical() {
            violations.push(CompatibilityForecastViolation::VocabularyDrift);
        }

        if !self.conformance.all_hold() {
            violations.push(CompatibilityForecastViolation::ConformanceDrift);
        }

        if contains_forbidden_material(self) {
            violations.push(CompatibilityForecastViolation::ForbiddenMaterial);
        }

        violations
    }

    /// The canonical export form: pretty JSON, identical across every render channel.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("forecast sheet serializes")
    }

    /// Renders the packet for a channel. Every channel produces byte-identical output.
    pub fn render_for_channel(&self, _channel: ForecastChannel) -> String {
        self.export_safe_json()
    }

    /// A compact Markdown summary of the forecasts and migration tasks, for export and review outside
    /// the app.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.report_label));
        out.push_str(&format!(
            "Update `{}` → `{}` on channel `{}` — {} subjects ({} review, {} hold), {} migration tasks, {} consumers.\n\n",
            self.target.current_version,
            self.target.target_version,
            self.target.channel.as_str(),
            self.summary.total_subjects,
            self.summary.review_subjects,
            self.summary.hold_subjects,
            self.summary.total_tasks,
            self.summary.total_consumers,
        ));
        if self.coverage.has_partial_coverage {
            out.push_str(&format!(
                "> Partial coverage: {} estimated, {} unknown-input, {} out-of-window line(s) labeled, not failed.\n\n",
                self.coverage.estimated_lines,
                self.coverage.unknown_lines,
                self.coverage.outside_window_lines,
            ));
        }
        out.push_str("## Compatibility forecasts\n\n");
        out.push_str(
            "| Subject | Claimed window | Worst readiness | Stable | Beta | Preview | LTS |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|\n");
        for s in &self.subjects {
            let drift = |line: CompatibilityLine| -> &str {
                s.line_forecasts
                    .iter()
                    .find(|l| l.line == line)
                    .map(|l| l.drift_class.as_str())
                    .unwrap_or("-")
            };
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                s.subject.as_str(),
                if s.within_claimed_window { "yes" } else { "no" },
                s.readiness.as_str(),
                drift(CompatibilityLine::Stable),
                drift(CompatibilityLine::Beta),
                drift(CompatibilityLine::Preview),
                drift(CompatibilityLine::Lts),
            ));
        }
        out.push_str("\n## Migration tasks\n\n");
        out.push_str("| Task | Subject | Class | Auto-fix | Due before | Skip policy | Rollback | Actions |\n");
        out.push_str("|---|---|---|---|---|---|---|---|\n");
        for t in &self.migration_tasks {
            let actions: Vec<&str> = t.available_actions.iter().map(|a| a.as_str()).collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                t.task_id,
                t.subject.as_str(),
                t.task_class.as_str(),
                t.auto_fix.as_str(),
                t.due_before.as_str(),
                t.skip_policy.as_str(),
                t.rollback_guidance.as_str(),
                actions.join(", "),
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}",
                c.consumer.as_str(),
                c.readiness.as_str(),
                c.gate_decision.as_str(),
            ));
            if c.gaps.is_empty() {
                out.push_str(")\n");
            } else {
                let gaps: Vec<String> = c
                    .gaps
                    .iter()
                    .map(|g| format!("{}:{}", g.subject.as_str(), g.gap_kind.as_str()))
                    .collect();
                out.push_str(&format!("; gap: {})\n", gaps.join(", ")));
            }
        }
        out
    }

    /// A machine-readable CSV of every migration task, for export and review outside the app.
    pub fn render_task_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "task_id,subject,task_class,owner_role,addresses_drift,confidence,auto_fix,due_before,skip_policy,requires_recorded_rationale,rollback_guidance,actions,waived,readiness\n",
        );
        for t in &self.migration_tasks {
            let actions: Vec<&str> = t.available_actions.iter().map(|a| a.as_str()).collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                t.task_id,
                t.subject.as_str(),
                t.task_class.as_str(),
                t.owner_role,
                t.addresses_drift.as_str(),
                t.confidence.as_str(),
                t.auto_fix.as_str(),
                t.due_before.as_str(),
                t.skip_policy.as_str(),
                t.requires_recorded_rationale,
                t.rollback_guidance.as_str(),
                actions.join("|"),
                t.is_waived(),
                t.readiness.as_str(),
            ));
        }
        out
    }
}

fn derive_counts(
    subjects: &[SubjectForecast],
    tasks: &[MigrationTaskRow],
    consumers: &[ForecastConsumerRow],
) -> ForecastCounts {
    let clear_subjects = subjects
        .iter()
        .filter(|s| s.worst_gate == DescriptorGate::Governed)
        .count() as u32;
    let review_subjects = subjects
        .iter()
        .filter(|s| s.worst_gate == DescriptorGate::Narrowed)
        .count() as u32;
    let hold_subjects = subjects
        .iter()
        .filter(|s| s.worst_gate == DescriptorGate::Blocked)
        .count() as u32;
    let clear_consumers = consumers.iter().filter(|c| c.is_clear()).count() as u32;
    let review_consumers = consumers.iter().filter(|c| c.is_review()).count() as u32;
    let hold_consumers = consumers.iter().filter(|c| c.is_hold()).count() as u32;
    ForecastCounts {
        total_subjects: subjects.len() as u32,
        clear_subjects,
        review_subjects,
        hold_subjects,
        out_of_window_subjects: subjects.iter().filter(|s| !s.within_claimed_window).count() as u32,
        speculative_subjects: subjects.iter().filter(|s| s.speculative).count() as u32,
        total_tasks: tasks.len() as u32,
        automatable_tasks: tasks.iter().filter(|t| t.auto_fix.is_automatable()).count() as u32,
        manual_tasks: tasks
            .iter()
            .filter(|t| {
                matches!(
                    t.auto_fix,
                    AutoFixAvailability::ManualOnly | AutoFixAvailability::AdminRequired
                )
            })
            .count() as u32,
        rationale_gated_tasks: tasks
            .iter()
            .filter(|t| t.requires_recorded_rationale)
            .count() as u32,
        waived_tasks: tasks.iter().filter(|t| t.is_waived()).count() as u32,
        total_consumers: consumers.len() as u32,
        clear_consumers,
        review_consumers,
        hold_consumers,
        requires_pre_rollout_resolution: hold_consumers > 0,
    }
}

fn derive_coverage(subjects: &[SubjectForecast]) -> ForecastCoverage {
    let lines = || subjects.iter().flat_map(|s| s.line_forecasts.iter());
    let qualified = lines()
        .filter(|l| {
            matches!(
                l.confidence,
                ForecastConfidence::Qualified | ForecastConfidence::Likely
            )
        })
        .count() as u32;
    let estimated = lines()
        .filter(|l| l.confidence == ForecastConfidence::Estimated)
        .count() as u32;
    let unknown = lines()
        .filter(|l| l.confidence == ForecastConfidence::Unknown)
        .count() as u32;
    let outside = lines()
        .filter(|l| l.confidence == ForecastConfidence::OutsideClaimedWindow)
        .count() as u32;
    let not_applicable = lines()
        .filter(|l| l.confidence == ForecastConfidence::NotApplicable)
        .count() as u32;
    ForecastCoverage {
        qualified_lines: qualified,
        estimated_lines: estimated,
        unknown_lines: unknown,
        outside_window_lines: outside,
        not_applicable_lines: not_applicable,
        has_partial_coverage: estimated > 0 || unknown > 0 || outside > 0,
    }
}

fn derive_release_gate(consumers: &[ForecastConsumerRow]) -> ForecastReleaseGate {
    let collect = |pred: fn(&ForecastConsumerRow) -> bool| -> Vec<String> {
        consumers
            .iter()
            .filter(|c| pred(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect()
    };
    let mut affected: Vec<CompatibilitySubject> = consumers
        .iter()
        .flat_map(|c| c.gaps.iter().map(|g| g.subject))
        .collect();
    affected.sort_by_key(|s| subject_rank(*s));
    affected.dedup();
    let hold = collect(ForecastConsumerRow::is_hold);
    ForecastReleaseGate {
        requires_pre_rollout_resolution: !hold.is_empty(),
        hold_consumers: hold,
        review_consumers: collect(ForecastConsumerRow::is_review),
        clear_consumers: collect(ForecastConsumerRow::is_clear),
        affected_subjects: affected.iter().map(|s| s.as_str().to_owned()).collect(),
        gate_message_id: format!(
            "{}release_gate",
            M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX
        ),
    }
}

/// Scans the export for forbidden raw material (credential bodies / raw provider payloads).
fn contains_forbidden_material(packet: &CompatibilityForecastSheet) -> bool {
    let json = serde_json::to_string(packet)
        .unwrap_or_default()
        .to_ascii_lowercase();
    const FORBIDDEN: [&str; 6] = [
        "bearer_token",
        "authorization:",
        "private_key",
        "begin rsa",
        "set-cookie",
        "client_secret",
    ];
    FORBIDDEN.iter().any(|needle| json.contains(needle))
}

/// Maps each variant of an `as_str`-bearing enum to its token, in declaration order.
fn tokens<T: Copy, const N: usize>(all: &[T; N], f: impl Fn(&T) -> &'static str) -> Vec<String> {
    all.iter().map(|x| f(x).to_owned()).collect()
}
