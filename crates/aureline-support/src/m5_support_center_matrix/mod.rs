//! Canonical M5 Support Center matrix: the single contract that makes the Support Center an explicit
//! product surface instead of a scatter of hidden pages.
//!
//! The Support Center is where a blocked user recovers — Project Doctor, Safe mode, extension
//! bisect, the performance / language / index / AI-usage / crash / network / artifacts inspectors,
//! issue-report and crash-intake, and the support-bundle export preview. Each of those surfaces had
//! grown its own row-local wording for *which inspectors it reuses*, *which support data classes it
//! touches*, *how it redacts on export*, and *which export modes it offers*. This packet replaces
//! that scatter with one machine-readable matrix: one [`SupportModuleRow`] per
//! [`SupportModule`], each binding the module to the shared inspector vocabulary
//! ([`Inspector`]), the export-risk data classes ([`DataClass`]), a redaction default
//! ([`RedactionDefault`]), and the local-save / team-share / formal-support export modes
//! ([`ExportMode`]) it offers.
//!
//! The readiness gate is non-inheriting and fail-closed. Each module declares the readiness it
//! claims ([`SupportModuleRow::declared_readiness`]) and records three independent inputs — how
//! fresh its evidence is ([`EvidenceFreshness`]), how available each bound inspector is
//! ([`InspectorAvailability`]), and whether export consent is granted for each offered export mode
//! ([`ConsentState`]). The published readiness ([`SupportModuleRow::effective_readiness`]) is the
//! weakest ceiling those inputs imply, so stale evidence, a degraded or unavailable inspector, or an
//! ungranted/blocked consent all narrow or withhold the module automatically rather than leaving a
//! page green by inertia. A module that declared a stronger claim than the gate permits has its
//! published readiness lowered, its [`SupportModuleDowngradeReason`]s and
//! [`SupportModuleDowngradePath`] recomputed, and its [`ModulePublication`] decision recomputed; all
//! are validated against the gate so a downgrade can never be asserted or hidden by hand.
//!
//! Support data classes stay visible and redaction-safe. A module that touches [`DataClass::HighRisk`]
//! material must default to [`RedactionDefault::ExcludedAlways`], and any module that offers a
//! sharing export mode (team-share or formal-support) must reuse the [`Inspector::ExportConsent`]
//! descriptor, so no surface can mint a private "safe to share" path. The local-save mode is always
//! a first-class peer of the share/upload modes.
//!
//! Because every required consumer surface — desktop shell, CLI/headless, Help/About, shiproom, and
//! formal-support handoff — binds to this one packet via a [`MatrixConsumerBinding`] that must ingest
//! it, preserve its published readiness and recovery paths, and narrow with it, a module narrowed
//! here cannot stay authoritative on a desktop page, a CLI report, a Help/About claim, a shiproom
//! row, or a formal-support handoff. Each binding is stamped with the active scope snapshot so
//! support and evidence packets can reconstruct the scope the matrix answered.
//!
//! The packet is checked in at `artifacts/support/m5/m5-support-center-matrix.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, or an opaque ref, and it carries
//! no credential bodies, raw provider payloads, live authority handles, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 Support Center matrix schema version.
pub const M5_SUPPORT_CENTER_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SUPPORT_CENTER_MATRIX_RECORD_KIND: &str = "m5_support_center_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_SUPPORT_CENTER_MATRIX_PATH: &str =
    "artifacts/support/m5/m5-support-center-matrix.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SUPPORT_CENTER_MATRIX_SCHEMA_REF: &str =
    "schemas/support/m5-support-center-matrix.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SUPPORT_CENTER_MATRIX_DOC_REF: &str = "docs/help/support/m5-support-center-matrix.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_SUPPORT_CENTER_MATRIX_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-support-center-matrix.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SUPPORT_CENTER_MATRIX_FIXTURE_DIR: &str =
    "fixtures/support/m5/m5-support-center-matrix";

/// Repo-relative path to the shiproom review packet that renders this matrix.
pub const M5_SUPPORT_CENTER_MATRIX_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-support-center-review-packet/support_center_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_SUPPORT_CENTER_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-support-center-matrix.json"
));

/// One Support Center module the matrix governs.
///
/// These are the recovery surfaces a blocked user reaches for; the matrix keeps them distinct so a
/// module's inspectors, data classes, redaction default, and export modes are never qualified as
/// another's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportModule {
    /// Project Doctor: probe-backed findings, finding codes, and guided repair entry.
    Doctor,
    /// Safe mode: the narrowed runtime profile with retained capabilities.
    SafeMode,
    /// Extension bisect / suspect-runtime quarantine.
    Bisect,
    /// Performance inspector: timeline, hot spots, and budget review.
    Performance,
    /// Language-service inspector: server state, capabilities, and restarts.
    Language,
    /// Index inspector: index health, rebuild state, and coverage.
    Index,
    /// AI-usage inspector: model, token, and policy usage review.
    AiUsage,
    /// Crash inspector: crash store, exact-build, and symbolication review.
    Crash,
    /// Network inspector: route origin, exposure, and reachability review.
    Network,
    /// Artifacts inspector: build/release artifact graph and provenance review.
    Artifacts,
    /// Issue-report and crash-intake routing.
    IssueReportCrashIntake,
    /// Support-bundle export preview with redaction manifest.
    SupportBundleExportPreview,
}

impl SupportModule {
    /// Every Support Center module, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Doctor,
        Self::SafeMode,
        Self::Bisect,
        Self::Performance,
        Self::Language,
        Self::Index,
        Self::AiUsage,
        Self::Crash,
        Self::Network,
        Self::Artifacts,
        Self::IssueReportCrashIntake,
        Self::SupportBundleExportPreview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Doctor => "doctor",
            Self::SafeMode => "safe_mode",
            Self::Bisect => "bisect",
            Self::Performance => "performance",
            Self::Language => "language",
            Self::Index => "index",
            Self::AiUsage => "ai_usage",
            Self::Crash => "crash",
            Self::Network => "network",
            Self::Artifacts => "artifacts",
            Self::IssueReportCrashIntake => "issue_report_crash_intake",
            Self::SupportBundleExportPreview => "support_bundle_export_preview",
        }
    }
}

/// The readiness the matrix publishes for a Support Center module, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleReadiness {
    /// The module is fully available and actionable.
    Operational,
    /// The module is available but limited — a degraded inspector or aging evidence narrows it.
    Degraded,
    /// The module can only inspect read-only; actions or exports are not offered.
    InspectOnly,
    /// The module is withheld; it cannot be offered as a Support Center surface.
    Unavailable,
}

impl ModuleReadiness {
    /// Every readiness class, highest to lowest.
    pub const ALL: [Self; 4] = [
        Self::Operational,
        Self::Degraded,
        Self::InspectOnly,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Degraded => "degraded",
            Self::InspectOnly => "inspect_only",
            Self::Unavailable => "unavailable",
        }
    }

    /// Rank for the fail-closed gate; higher is more capable.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Operational => 3,
            Self::Degraded => 2,
            Self::InspectOnly => 1,
            Self::Unavailable => 0,
        }
    }
}

/// The weaker (lower-rank) of two readiness classes.
fn weaker(a: ModuleReadiness, b: ModuleReadiness) -> ModuleReadiness {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// How fresh the evidence backing a module is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The evidence is current.
    Current,
    /// The evidence is aging but in tolerance; caps at degraded.
    Aging,
    /// The evidence is expired; caps at inspect-only.
    Expired,
    /// The evidence is missing; caps at withheld.
    Missing,
}

impl EvidenceFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Aging, Self::Expired, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// Highest readiness this freshness state permits a module to publish.
    pub const fn readiness_ceiling(self) -> ModuleReadiness {
        match self {
            Self::Current => ModuleReadiness::Operational,
            Self::Aging => ModuleReadiness::Degraded,
            Self::Expired => ModuleReadiness::InspectOnly,
            Self::Missing => ModuleReadiness::Unavailable,
        }
    }

    /// Whether this state raises the [`SupportModuleDowngradeReason::EvidenceStale`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// The one canonical inspector / descriptor vocabulary every module binds.
///
/// This is the spec's "one canonical vocabulary": environment status, precedence inspection,
/// crash-intake, install/advisory state, credential state, and export consent. A module declares
/// which of these it reuses rather than minting its own descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Inspector {
    /// Environment status strip: the execution context and why it won.
    EnvironmentStatus,
    /// Precedence inspector: which config/policy layer won and what it shadowed.
    PrecedenceInspector,
    /// Crash-intake descriptor: crash envelope, exact-build, and symbolication routing.
    CrashIntake,
    /// Install / advisory state: install mode, channel, and active advisories.
    InstallAdvisoryState,
    /// Credential-state descriptor: credential posture without secret bodies.
    CredentialState,
    /// Export-consent descriptor: redaction manifest and data-class consent.
    ExportConsent,
}

impl Inspector {
    /// Every inspector descriptor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EnvironmentStatus,
        Self::PrecedenceInspector,
        Self::CrashIntake,
        Self::InstallAdvisoryState,
        Self::CredentialState,
        Self::ExportConsent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentStatus => "environment_status",
            Self::PrecedenceInspector => "precedence_inspector",
            Self::CrashIntake => "crash_intake",
            Self::InstallAdvisoryState => "install_advisory_state",
            Self::CredentialState => "credential_state",
            Self::ExportConsent => "export_consent",
        }
    }
}

/// How available a bound inspector is for a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorAvailability {
    /// The inspector is wired and current.
    Available,
    /// The inspector is wired but degraded; caps the module at degraded.
    Degraded,
    /// The inspector is missing; caps the module at withheld.
    Unavailable,
}

impl InspectorAvailability {
    /// Every availability state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Available, Self::Degraded, Self::Unavailable];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }

    /// Highest readiness this availability permits a module to publish.
    pub const fn readiness_ceiling(self) -> ModuleReadiness {
        match self {
            Self::Available => ModuleReadiness::Operational,
            Self::Degraded => ModuleReadiness::Degraded,
            Self::Unavailable => ModuleReadiness::Unavailable,
        }
    }
}

/// A support data class crossing a Support Center boundary.
///
/// Reuses the frozen export-risk vocabulary at `schemas/support/data_risk_class.schema.json` so the
/// Support Center never mints local synonyms for "safe to export".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataClass {
    /// Metadata only: ids, counts, states, and timestamps.
    MetadataOnly,
    /// Environment-adjacent: platform, channel, and topology descriptors.
    EnvironmentAdjacent,
    /// Code-adjacent: file paths, symbol names, and code references.
    CodeAdjacent,
    /// High-risk: secret-bearing, credential, or transcript material.
    HighRisk,
}

impl DataClass {
    /// Every data class, in declaration order (low to high risk).
    pub const ALL: [Self; 4] = [
        Self::MetadataOnly,
        Self::EnvironmentAdjacent,
        Self::CodeAdjacent,
        Self::HighRisk,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::CodeAdjacent => "code_adjacent",
            Self::HighRisk => "high_risk",
        }
    }
}

/// The default redaction / inclusion posture a module applies on export.
///
/// Reuses the evidence-inclusion vocabulary shared by the export-review and support-bundle lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionDefault {
    /// Only metadata is embedded by default.
    EmbeddedMetadataOnly,
    /// Material is embedded by opaque reference, not by body.
    EmbeddedByReference,
    /// Material is retained local-only and never leaves the machine by default.
    RetainedLocalOnly,
    /// Material is excluded by default but may be added with explicit consent.
    ExcludedByDefault,
    /// Material is excluded always; no consent can include it.
    ExcludedAlways,
}

impl RedactionDefault {
    /// Every redaction default, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EmbeddedMetadataOnly,
        Self::EmbeddedByReference,
        Self::RetainedLocalOnly,
        Self::ExcludedByDefault,
        Self::ExcludedAlways,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedMetadataOnly => "embedded_metadata_only",
            Self::EmbeddedByReference => "embedded_by_reference",
            Self::RetainedLocalOnly => "retained_local_only",
            Self::ExcludedByDefault => "excluded_by_default",
            Self::ExcludedAlways => "excluded_always",
        }
    }

    /// Whether this default keeps high-risk material out of any export by default.
    ///
    /// Only [`RedactionDefault::ExcludedAlways`] is strong enough for a module that touches
    /// high-risk data: it is the one posture no consent can override.
    pub const fn excludes_high_risk(self) -> bool {
        matches!(self, Self::ExcludedAlways)
    }
}

/// An export mode a Support Center module may offer.
///
/// The three modes the spec names; local-save is always a first-class peer of the share/upload modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportMode {
    /// Save the export locally for the user's own review; no sharing.
    LocalSave,
    /// Share the export with a team or managed admin.
    TeamShare,
    /// Hand the export to a formal / vendor support case.
    FormalSupport,
}

impl ExportMode {
    /// Every export mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::LocalSave, Self::TeamShare, Self::FormalSupport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSave => "local_save",
            Self::TeamShare => "team_share",
            Self::FormalSupport => "formal_support",
        }
    }

    /// Whether this mode shares the export off the local machine.
    pub const fn is_sharing(self) -> bool {
        matches!(self, Self::TeamShare | Self::FormalSupport)
    }
}

/// Whether export consent is granted for an offered export mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentState {
    /// Consent is granted; the export mode is actionable.
    Granted,
    /// Consent is required but not yet granted; caps the module at degraded.
    RequiredNotGranted,
    /// Consent is blocked by policy or data class; caps the module at inspect-only.
    Blocked,
}

impl ConsentState {
    /// Every consent state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Granted, Self::RequiredNotGranted, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::RequiredNotGranted => "required_not_granted",
            Self::Blocked => "blocked",
        }
    }

    /// Highest readiness this consent state permits a module to publish.
    pub const fn readiness_ceiling(self) -> ModuleReadiness {
        match self {
            Self::Granted => ModuleReadiness::Operational,
            Self::RequiredNotGranted => ModuleReadiness::Degraded,
            Self::Blocked => ModuleReadiness::InspectOnly,
        }
    }
}

/// A headline reason the readiness gate narrows a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportModuleDowngradeReason {
    /// The module's evidence is aging, expired, or missing.
    EvidenceStale,
    /// A bound inspector is degraded.
    InspectorDegraded,
    /// A bound inspector is unavailable.
    InspectorUnavailable,
    /// Export consent is required-but-ungranted or blocked for an offered mode.
    ConsentUnsatisfied,
}

impl SupportModuleDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::EvidenceStale,
        Self::InspectorDegraded,
        Self::InspectorUnavailable,
        Self::ConsentUnsatisfied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceStale => "evidence_stale",
            Self::InspectorDegraded => "inspector_degraded",
            Self::InspectorUnavailable => "inspector_unavailable",
            Self::ConsentUnsatisfied => "consent_unsatisfied",
        }
    }
}

/// The exact recovery path surfaced when a module is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportModuleDowngradePath {
    /// Refresh the aging, expired, or missing evidence.
    RefreshEvidence,
    /// Restore the degraded or unavailable inspector.
    RestoreInspector,
    /// Resolve the ungranted or blocked export consent.
    ResolveConsent,
    /// Withhold the module from the Support Center.
    WithholdModule,
    /// No downgrade is needed; only valid when the module publishes cleanly.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl SupportModuleDowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RefreshEvidence,
        Self::RestoreInspector,
        Self::ResolveConsent,
        Self::WithholdModule,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshEvidence => "refresh_evidence",
            Self::RestoreInspector => "restore_inspector",
            Self::ResolveConsent => "resolve_consent",
            Self::WithholdModule => "withhold_module",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the module owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// The publication decision the gate publishes for a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModulePublication {
    /// The module is offered at its declared readiness; nothing narrowed it.
    Published,
    /// The gate narrowed the module below its declared readiness.
    Narrowed,
    /// The module is withheld from the Support Center entirely.
    Withheld,
}

impl ModulePublication {
    /// Every publication decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::Published, Self::Narrowed, Self::Withheld];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the gate narrowed or withheld the module.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Published)
    }
}

/// A downstream surface that must ingest this matrix and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixConsumerSurface {
    /// The desktop shell's Support Center pages.
    DesktopShell,
    /// CLI / headless support output.
    CliHeadless,
    /// Help/About and service-health surfaces.
    HelpAbout,
    /// Shiproom claim packet.
    Shiproom,
    /// Formal-support / vendor handoff.
    FormalSupportHandoff,
}

impl MatrixConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopShell,
        Self::CliHeadless,
        Self::HelpAbout,
        Self::Shiproom,
        Self::FormalSupportHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopShell => "desktop_shell",
            Self::CliHeadless => "cli_headless",
            Self::HelpAbout => "help_about",
            Self::Shiproom => "shiproom",
            Self::FormalSupportHandoff => "formal_support_handoff",
        }
    }
}

/// One bound inspector for a module, with how available it is and the descriptor it binds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectorBinding {
    /// Inspector descriptor this binding reuses.
    pub inspector: Inspector,
    /// How available the inspector is for the module.
    pub availability: InspectorAvailability,
    /// Ref to the canonical descriptor source this binding reuses.
    pub descriptor_ref: String,
    /// Capture timestamp for the availability check.
    pub checked_at: String,
}

impl InspectorBinding {
    /// Whether the binding carries the non-empty descriptor ref and timestamp it requires.
    pub fn is_well_formed(&self) -> bool {
        !self.descriptor_ref.trim().is_empty() && !self.checked_at.trim().is_empty()
    }
}

/// One offered export mode for a module, with its consent state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportModeBinding {
    /// Export mode this binding offers.
    pub mode: ExportMode,
    /// Whether export consent is granted for the mode.
    pub consent: ConsentState,
}

/// One Support Center module row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportModuleRow {
    /// Stable module-row id.
    pub row_id: String,
    /// Support Center module this row governs.
    pub module: SupportModule,
    /// Owner accountable for the module's evidence and conformance.
    pub owner: String,
    /// How fresh the module's evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// Inspectors this module reuses; at least one.
    #[serde(default)]
    pub inspectors: Vec<InspectorBinding>,
    /// Support data classes this module touches.
    #[serde(default)]
    pub data_classes: Vec<DataClass>,
    /// Default redaction posture the module applies on export.
    pub redaction_default: RedactionDefault,
    /// Export modes the module offers, with consent state.
    #[serde(default)]
    pub export_modes: Vec<ExportModeBinding>,
    /// Readiness the module's own evidence claims, before the gate.
    pub declared_readiness: ModuleReadiness,
    /// Readiness actually published after the gate narrows the module.
    ///
    /// Must equal [`SupportModuleRow::effective_readiness`].
    pub published_readiness: ModuleReadiness,
    /// Publication decision the gate publishes; must equal the recomputed decision.
    pub module_publication: ModulePublication,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<SupportModuleDowngradeReason>,
    /// Recovery path surfaced when the module is narrowed or withheld.
    pub downgrade_path: SupportModuleDowngradePath,
    /// Actions or capabilities the module still offers; empty when withheld.
    #[serde(default)]
    pub offered_actions: Vec<String>,
    /// Caveats attached to the published module.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the module.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the conformance suite backing the module.
    pub conformance_ref: String,
    /// Ref to the module's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the matrix answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Ref to the machine-readable matrix receipt.
    pub matrix_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl SupportModuleRow {
    /// The readiness the module's own evidence asserted, before gate narrowing.
    pub fn capability_floor(&self) -> ModuleReadiness {
        self.declared_readiness
    }

    /// Highest readiness the evidence freshness permits.
    pub fn freshness_ceiling(&self) -> ModuleReadiness {
        self.evidence_freshness.readiness_ceiling()
    }

    /// Highest readiness the bound inspectors permit, the weakest across every inspector.
    pub fn inspector_ceiling(&self) -> ModuleReadiness {
        let mut ceiling = ModuleReadiness::Operational;
        for binding in &self.inspectors {
            ceiling = weaker(ceiling, binding.availability.readiness_ceiling());
        }
        ceiling
    }

    /// Highest readiness the export consent permits, the weakest across every offered mode.
    pub fn consent_ceiling(&self) -> ModuleReadiness {
        let mut ceiling = ModuleReadiness::Operational;
        for binding in &self.export_modes {
            ceiling = weaker(ceiling, binding.consent.readiness_ceiling());
        }
        ceiling
    }

    /// The readiness the gate permits this module to publish.
    ///
    /// Lowers the declared readiness to the weakest ceiling implied by the evidence freshness, the
    /// inspector availability, and the export consent, so stale evidence, a degraded or unavailable
    /// inspector, or an ungranted/blocked consent can never publish a fuller claim than the inputs
    /// support.
    pub fn effective_readiness(&self) -> ModuleReadiness {
        let mut readiness = self.capability_floor();
        readiness = weaker(readiness, self.freshness_ceiling());
        readiness = weaker(readiness, self.inspector_ceiling());
        readiness = weaker(readiness, self.consent_ceiling());
        readiness
    }

    /// Whether any bound inspector is degraded.
    pub fn has_degraded_inspector(&self) -> bool {
        self.inspectors
            .iter()
            .any(|b| b.availability == InspectorAvailability::Degraded)
    }

    /// Whether any bound inspector is unavailable.
    pub fn has_unavailable_inspector(&self) -> bool {
        self.inspectors
            .iter()
            .any(|b| b.availability == InspectorAvailability::Unavailable)
    }

    /// Whether any offered export mode lacks granted consent.
    pub fn has_unsatisfied_consent(&self) -> bool {
        self.export_modes
            .iter()
            .any(|b| b.consent != ConsentState::Granted)
    }

    /// Whether the module offers any sharing (off-machine) export mode.
    pub fn offers_sharing_export(&self) -> bool {
        self.export_modes.iter().any(|b| b.mode.is_sharing())
    }

    /// Whether the module touches any high-risk data class.
    pub fn touches_high_risk(&self) -> bool {
        self.data_classes.contains(&DataClass::HighRisk)
    }

    /// Whether the module reuses the given inspector descriptor.
    pub fn reuses_inspector(&self, inspector: Inspector) -> bool {
        self.inspectors.iter().any(|b| b.inspector == inspector)
    }

    /// The headline downgrade reasons recomputed from the module's observed states.
    ///
    /// A reason is raised only when its input narrows the module below its declared readiness, so a
    /// module designed as inspect-only is not reported as "downgraded" for being inspect-only.
    pub fn computed_downgrade_reasons(&self) -> Vec<SupportModuleDowngradeReason> {
        let declared = self.declared_readiness.rank();
        let mut reasons = Vec::new();
        if self.freshness_ceiling().rank() < declared {
            reasons.push(SupportModuleDowngradeReason::EvidenceStale);
        }
        if self.has_degraded_inspector()
            && InspectorAvailability::Degraded.readiness_ceiling().rank() < declared
        {
            reasons.push(SupportModuleDowngradeReason::InspectorDegraded);
        }
        if self.has_unavailable_inspector()
            && InspectorAvailability::Unavailable
                .readiness_ceiling()
                .rank()
                < declared
        {
            reasons.push(SupportModuleDowngradeReason::InspectorUnavailable);
        }
        if self.consent_ceiling().rank() < declared {
            reasons.push(SupportModuleDowngradeReason::ConsentUnsatisfied);
        }
        reasons
    }

    /// The recovery path the gate must record, derived from the module's observed states.
    ///
    /// Ordered by severity: a withheld module points at a withhold, an inspector problem at an
    /// inspector restore, a consent problem at a consent resolution, and stale evidence at a refresh.
    pub fn computed_downgrade_path(&self) -> SupportModuleDowngradePath {
        let reasons = self.computed_downgrade_reasons();
        if self.effective_readiness() == ModuleReadiness::Unavailable {
            SupportModuleDowngradePath::WithholdModule
        } else if reasons.contains(&SupportModuleDowngradeReason::InspectorDegraded)
            || reasons.contains(&SupportModuleDowngradeReason::InspectorUnavailable)
        {
            SupportModuleDowngradePath::RestoreInspector
        } else if reasons.contains(&SupportModuleDowngradeReason::ConsentUnsatisfied) {
            SupportModuleDowngradePath::ResolveConsent
        } else if reasons.contains(&SupportModuleDowngradeReason::EvidenceStale) {
            SupportModuleDowngradePath::RefreshEvidence
        } else {
            SupportModuleDowngradePath::NoneNeeded
        }
    }

    /// The publication decision the gate must record, derived from the module's observed states.
    pub fn computed_publication(&self) -> ModulePublication {
        if self.effective_readiness() == ModuleReadiness::Unavailable {
            ModulePublication::Withheld
        } else if self.is_downgraded() || !self.computed_downgrade_reasons().is_empty() {
            ModulePublication::Narrowed
        } else {
            ModulePublication::Published
        }
    }

    /// Whether the module is offered cleanly at its declared readiness.
    pub fn is_published(&self) -> bool {
        self.computed_publication() == ModulePublication::Published
    }

    /// Whether the gate narrowed the published readiness below what the module declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_readiness().rank() < self.capability_floor().rank()
    }

    /// Whether the module carries its own non-empty conformance, evidence, scope, and receipt refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
            && !self.matrix_receipt_ref.trim().is_empty()
    }

    /// Whether the recorded readiness, decision, reasons, and path all agree with the gate.
    pub fn gate_consistent(&self) -> bool {
        self.published_readiness == self.effective_readiness()
            && self.module_publication == self.computed_publication()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.downgrade_path == self.computed_downgrade_path()
    }
}

/// One binding wiring a downstream surface to this matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: MatrixConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Matrix packet id this surface ingests.
    pub matrix_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this matrix rather than a parallel sheet.
    pub ingests_matrix: bool,
    /// True when the surface preserves the published readiness verbatim.
    pub preserves_published_readiness: bool,
    /// True when the surface preserves the recovery paths verbatim.
    pub preserves_recovery_paths: bool,
    /// True when the surface narrows automatically as modules are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl MatrixConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.matrix_packet_id_ref == packet_id
            && self.ingests_matrix
            && self.preserves_published_readiness
            && self.preserves_recovery_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportCenterMatrixSummary {
    /// Total module rows.
    pub total_modules: usize,
    /// Modules offered at their declared readiness.
    pub published_modules: usize,
    /// Modules the gate narrowed.
    pub narrowed_modules: usize,
    /// Modules the gate withheld.
    pub withheld_modules: usize,
    /// Modules whose published readiness was narrowed below their declared readiness.
    pub downgraded_modules: usize,
    /// Modules carrying at least one downgrade reason.
    pub modules_with_downgrade_reasons: usize,
    /// Modules whose evidence is aging, expired, or missing.
    pub stale_evidence_modules: usize,
    /// Modules with at least one degraded or unavailable inspector.
    pub modules_with_imperfect_inspectors: usize,
    /// Modules that offer at least one export mode.
    pub exportable_modules: usize,
}

/// A redaction-safe export row projected from a module row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportCenterMatrixExportRow {
    /// Module-row id.
    pub row_id: String,
    /// Module token.
    pub module: String,
    /// Owner accountable for the module.
    pub owner: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Inspector tokens this module reuses.
    pub inspectors: Vec<String>,
    /// Data-class tokens this module touches.
    pub data_classes: Vec<String>,
    /// Redaction-default token.
    pub redaction_default: String,
    /// Export-mode tokens this module offers.
    pub export_modes: Vec<String>,
    /// Declared-readiness token.
    pub declared_readiness: String,
    /// Published-readiness token.
    pub published_readiness: String,
    /// Publication token.
    pub module_publication: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Actions the module still offers.
    pub offered_actions: Vec<String>,
    /// Caveats attached to the module.
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Scope snapshot the matrix answered.
    pub scope_snapshot_ref: String,
    /// Matrix-receipt ref.
    pub matrix_receipt_ref: String,
    /// Whether the module publishes cleanly.
    pub published: bool,
    /// Whether the published readiness was narrowed below the declared readiness.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the matrix — the canonical Support Center index downstream
/// surfaces render instead of restating each module's readiness by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportCenterMatrixExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5SupportCenterMatrixExportRow>,
    /// Whether every module's published readiness and decision agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Modules offered cleanly.
    pub published_count: usize,
    /// Modules the gate narrowed.
    pub narrowed_count: usize,
    /// Modules the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 Support Center matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportCenterMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed Support Center module vocabulary.
    pub modules: Vec<SupportModule>,
    /// Closed readiness vocabulary.
    pub readiness_labels: Vec<ModuleReadiness>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed inspector-descriptor vocabulary.
    pub inspectors: Vec<Inspector>,
    /// Closed inspector-availability vocabulary.
    pub inspector_availabilities: Vec<InspectorAvailability>,
    /// Closed data-class vocabulary.
    pub data_classes: Vec<DataClass>,
    /// Closed redaction-default vocabulary.
    pub redaction_defaults: Vec<RedactionDefault>,
    /// Closed export-mode vocabulary.
    pub export_modes: Vec<ExportMode>,
    /// Closed consent-state vocabulary.
    pub consent_states: Vec<ConsentState>,
    /// Closed publication vocabulary.
    pub publications: Vec<ModulePublication>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<SupportModuleDowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<SupportModuleDowngradeReason>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<MatrixConsumerSurface>,
    /// Module rows, one per module.
    #[serde(default)]
    pub rows: Vec<SupportModuleRow>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<MatrixConsumerBinding>,
    /// Summary counts.
    pub summary: M5SupportCenterMatrixSummary,
}

impl M5SupportCenterMatrix {
    /// Returns the row for the given module.
    pub fn row_for(&self, module: SupportModule) -> Option<&SupportModuleRow> {
        self.rows.iter().find(|r| r.module == module)
    }

    /// Returns the row with the given id.
    pub fn row(&self, row_id: &str) -> Option<&SupportModuleRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Modules offered cleanly.
    pub fn published_rows(&self) -> impl Iterator<Item = &SupportModuleRow> {
        self.rows.iter().filter(|r| r.is_published())
    }

    /// Modules the gate auto-narrowed.
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &SupportModuleRow> {
        self.rows
            .iter()
            .filter(|r| r.computed_publication() == ModulePublication::Narrowed)
    }

    /// Modules the gate withheld entirely.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &SupportModuleRow> {
        self.rows
            .iter()
            .filter(|r| r.computed_publication() == ModulePublication::Withheld)
    }

    /// Whether a consumer binding preserves this matrix for the given surface.
    pub fn has_binding_for(&self, surface: MatrixConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every module's recorded readiness, decision, reasons, and path agree with the gate.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5SupportCenterMatrixSummary {
        let count_publication = |publication: ModulePublication| {
            self.rows
                .iter()
                .filter(|r| r.module_publication == publication)
                .count()
        };
        M5SupportCenterMatrixSummary {
            total_modules: self.rows.len(),
            published_modules: count_publication(ModulePublication::Published),
            narrowed_modules: count_publication(ModulePublication::Narrowed),
            withheld_modules: count_publication(ModulePublication::Withheld),
            downgraded_modules: self.rows.iter().filter(|r| r.is_downgraded()).count(),
            modules_with_downgrade_reasons: self
                .rows
                .iter()
                .filter(|r| !r.downgrade_reasons.is_empty())
                .count(),
            stale_evidence_modules: self
                .rows
                .iter()
                .filter(|r| r.evidence_freshness.is_stale_trigger())
                .count(),
            modules_with_imperfect_inspectors: self
                .rows
                .iter()
                .filter(|r| r.has_degraded_inspector() || r.has_unavailable_inspector())
                .count(),
            exportable_modules: self
                .rows
                .iter()
                .filter(|r| !r.export_modes.is_empty())
                .count(),
        }
    }

    /// Produces the Support Center index downstream surfaces — desktop shell, CLI/headless,
    /// Help/About, shiproom, and formal-support handoff — render instead of restating each module's
    /// readiness by hand.
    pub fn export_projection(&self) -> M5SupportCenterMatrixExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|r| M5SupportCenterMatrixExportRow {
                row_id: r.row_id.clone(),
                module: r.module.as_str().to_owned(),
                owner: r.owner.clone(),
                evidence_freshness: r.evidence_freshness.as_str().to_owned(),
                inspectors: r
                    .inspectors
                    .iter()
                    .map(|b| b.inspector.as_str().to_owned())
                    .collect(),
                data_classes: r
                    .data_classes
                    .iter()
                    .map(|d| d.as_str().to_owned())
                    .collect(),
                redaction_default: r.redaction_default.as_str().to_owned(),
                export_modes: r
                    .export_modes
                    .iter()
                    .map(|b| b.mode.as_str().to_owned())
                    .collect(),
                declared_readiness: r.declared_readiness.as_str().to_owned(),
                published_readiness: r.published_readiness.as_str().to_owned(),
                module_publication: r.module_publication.as_str().to_owned(),
                downgrade_reasons: r
                    .downgrade_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                downgrade_path: r.downgrade_path.as_str().to_owned(),
                offered_actions: r.offered_actions.clone(),
                caveats: r.caveats.clone(),
                stale_or_missing_fields: r.stale_or_missing_fields.clone(),
                scope_snapshot_ref: r.scope_snapshot_ref.clone(),
                matrix_receipt_ref: r.matrix_receipt_ref.clone(),
                published: r.is_published(),
                downgraded: r.is_downgraded(),
                summary: format!(
                    "{}: evidence {}, declared {}, published {} ({}), recovery {}",
                    r.module.as_str(),
                    r.evidence_freshness.as_str(),
                    r.declared_readiness.as_str(),
                    r.published_readiness.as_str(),
                    r.module_publication.as_str(),
                    r.downgrade_path.as_str()
                ),
            })
            .collect();
        M5SupportCenterMatrixExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            published_count: self.published_rows().count(),
            narrowed_count: self.narrowed_rows().count(),
            withheld_count: self.withheld_rows().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact Support Center matrix.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SupportCenterMatrixSupportExport {
        M5SupportCenterMatrixSupportExport {
            record_kind: M5_SUPPORT_CENTER_MATRIX_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_CENTER_MATRIX_SCHEMA_VERSION,
            export_id: export_id.into(),
            matrix_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            matrix: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5SupportCenterMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut covered_modules = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5SupportCenterMatrixViolation::DuplicateModuleRow {
                    row_id: row.row_id.clone(),
                });
            }
            if !covered_modules.insert(row.module) {
                violations.push(M5SupportCenterMatrixViolation::DuplicateModule {
                    module: row.module.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every Support Center module must carry exactly one row, so no module inherits a posture
        // from an adjacent one and no claimed module is missing.
        for module in SupportModule::ALL {
            if !covered_modules.contains(&module) {
                violations.push(M5SupportCenterMatrixViolation::MissingModule {
                    module: module.as_str(),
                });
            }
        }

        // Every required consumer surface must bind to this packet and narrow with it, so a narrowed
        // module cannot stay green on a downstream surface by inertia.
        for surface in MatrixConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5SupportCenterMatrixViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5SupportCenterMatrixViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SupportCenterMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SupportCenterMatrixViolation>) {
        if self.schema_version != M5_SUPPORT_CENTER_MATRIX_SCHEMA_VERSION {
            violations.push(M5SupportCenterMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_SUPPORT_CENTER_MATRIX_RECORD_KIND {
            violations.push(M5SupportCenterMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportCenterMatrixViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("modules", self.modules == SupportModule::ALL.to_vec()),
            (
                "readiness_labels",
                self.readiness_labels == ModuleReadiness::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            ("inspectors", self.inspectors == Inspector::ALL.to_vec()),
            (
                "inspector_availabilities",
                self.inspector_availabilities == InspectorAvailability::ALL.to_vec(),
            ),
            ("data_classes", self.data_classes == DataClass::ALL.to_vec()),
            (
                "redaction_defaults",
                self.redaction_defaults == RedactionDefault::ALL.to_vec(),
            ),
            (
                "export_modes",
                self.export_modes == ExportMode::ALL.to_vec(),
            ),
            (
                "consent_states",
                self.consent_states == ConsentState::ALL.to_vec(),
            ),
            (
                "publications",
                self.publications == ModulePublication::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == SupportModuleDowngradePath::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == SupportModuleDowngradeReason::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == MatrixConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5SupportCenterMatrixViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &SupportModuleRow,
        violations: &mut Vec<M5SupportCenterMatrixViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("owner", &row.owner),
            ("conformance_ref", &row.conformance_ref),
            ("evidence_ref", &row.evidence_ref),
            ("scope_snapshot_ref", &row.scope_snapshot_ref),
            ("matrix_receipt_ref", &row.matrix_receipt_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportCenterMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every module must reuse at least one inspector, and each binding must carry its descriptor
        // ref and timestamp, so a module never claims a posture without binding the shared vocabulary.
        if row.inspectors.is_empty() {
            violations.push(M5SupportCenterMatrixViolation::NoInspectors {
                row_id: row.row_id.clone(),
            });
        }
        let mut seen_inspectors = BTreeSet::new();
        for binding in &row.inspectors {
            if !seen_inspectors.insert(binding.inspector) {
                violations.push(M5SupportCenterMatrixViolation::DuplicateInspector {
                    row_id: row.row_id.clone(),
                    inspector: binding.inspector.as_str(),
                });
            }
            if !binding.is_well_formed() {
                violations.push(M5SupportCenterMatrixViolation::InspectorBindingIncomplete {
                    row_id: row.row_id.clone(),
                    inspector: binding.inspector.as_str(),
                });
            }
        }

        let mut seen_modes = BTreeSet::new();
        for binding in &row.export_modes {
            if !seen_modes.insert(binding.mode) {
                violations.push(M5SupportCenterMatrixViolation::DuplicateExportMode {
                    row_id: row.row_id.clone(),
                    mode: binding.mode.as_str(),
                });
            }
        }

        let mut seen_classes = BTreeSet::new();
        for class in &row.data_classes {
            if !seen_classes.insert(*class) {
                violations.push(M5SupportCenterMatrixViolation::DuplicateDataClass {
                    row_id: row.row_id.clone(),
                    data_class: class.as_str(),
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5SupportCenterMatrixViolation::DuplicateDowngradeReason {
                    row_id: row.row_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published readiness must equal the gate's recomputed ceiling, so a stale, degraded, or
        // unconsented module can never read as operational.
        let effective = row.effective_readiness();
        if row.published_readiness != effective {
            violations.push(M5SupportCenterMatrixViolation::OverstatedReadiness {
                row_id: row.row_id.clone(),
                published: row.published_readiness.as_str(),
                computed: effective.as_str(),
            });
        }

        let required_publication = row.computed_publication();
        if row.module_publication != required_publication {
            violations.push(M5SupportCenterMatrixViolation::PublicationMismatch {
                row_id: row.row_id.clone(),
                declared: row.module_publication.as_str(),
                required: required_publication.as_str(),
            });
        }

        let computed_reasons = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed_reasons {
            violations.push(M5SupportCenterMatrixViolation::DowngradeReasonsMismatch {
                row_id: row.row_id.clone(),
            });
        }

        let computed_path = row.computed_downgrade_path();
        if row.downgrade_path != computed_path {
            violations.push(M5SupportCenterMatrixViolation::DowngradePathMismatch {
                row_id: row.row_id.clone(),
                declared: row.downgrade_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // A narrowed or withheld module must offer a real recovery path, list a caveat, and name what
        // is stale, so a degraded module never drops its recovery semantics or hides why it narrowed.
        if row.module_publication.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(M5SupportCenterMatrixViolation::MissingDowngradePath {
                    row_id: row.row_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5SupportCenterMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5SupportCenterMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A withheld module offers nothing; a still-offered module must name at least one action.
        if row.module_publication == ModulePublication::Withheld {
            if !row.offered_actions.is_empty() {
                violations.push(
                    M5SupportCenterMatrixViolation::WithheldModuleOffersActions {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        } else if row.offered_actions.is_empty() {
            violations.push(M5SupportCenterMatrixViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "offered_actions",
            });
        }

        // High-risk data must be excluded-always: no consent can include secret-bearing material in
        // a Support Center export.
        if row.touches_high_risk() && !row.redaction_default.excludes_high_risk() {
            violations.push(M5SupportCenterMatrixViolation::HighRiskNotExcluded {
                row_id: row.row_id.clone(),
                redaction_default: row.redaction_default.as_str(),
            });
        }

        // A module that shares off-machine must reuse the export-consent descriptor, so the consent
        // surface is always bound where it matters.
        if row.offers_sharing_export() && !row.reuses_inspector(Inspector::ExportConsent) {
            violations.push(
                M5SupportCenterMatrixViolation::SharingWithoutConsentInspector {
                    row_id: row.row_id.clone(),
                },
            );
        }

        // A module that touches data classes but offers no export mode must still be local-first
        // explainable; the matrix never requires an export. But a module that publishes a clean
        // operational claim must be genuinely whole: current evidence, every inspector available,
        // every consent granted, declared operational, and nothing narrowing it.
        if effective == ModuleReadiness::Operational
            && (row.declared_readiness != ModuleReadiness::Operational
                || row.evidence_freshness != EvidenceFreshness::Current
                || row.inspector_ceiling() != ModuleReadiness::Operational
                || row.consent_ceiling() != ModuleReadiness::Operational
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered())
        {
            violations.push(M5SupportCenterMatrixViolation::PublishedModuleNotWhole {
                row_id: row.row_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 Support Center matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SupportCenterMatrixViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A module-row id appears more than once.
    DuplicateModuleRow {
        /// Duplicate row id.
        row_id: String,
    },
    /// A module appears in more than one row.
    DuplicateModule {
        /// Module token.
        module: &'static str,
    },
    /// A Support Center module has no row.
    MissingModule {
        /// Module token.
        module: &'static str,
    },
    /// A module reuses no inspector.
    NoInspectors {
        /// Row id.
        row_id: String,
    },
    /// A module reuses the same inspector more than once.
    DuplicateInspector {
        /// Row id.
        row_id: String,
        /// Inspector token.
        inspector: &'static str,
    },
    /// An inspector binding is missing its descriptor ref or timestamp.
    InspectorBindingIncomplete {
        /// Row id.
        row_id: String,
        /// Inspector token.
        inspector: &'static str,
    },
    /// A module offers the same export mode more than once.
    DuplicateExportMode {
        /// Row id.
        row_id: String,
        /// Export-mode token.
        mode: &'static str,
    },
    /// A module lists the same data class more than once.
    DuplicateDataClass {
        /// Row id.
        row_id: String,
        /// Data-class token.
        data_class: &'static str,
    },
    /// A module lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Row id.
        row_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A module publishes a readiness beyond what the gate computes.
    OverstatedReadiness {
        /// Row id.
        row_id: String,
        /// Published readiness token.
        published: &'static str,
        /// Computed effective readiness token.
        computed: &'static str,
    },
    /// A module's publication decision disagrees with the gate.
    PublicationMismatch {
        /// Row id.
        row_id: String,
        /// Declared publication token.
        declared: &'static str,
        /// Required publication token.
        required: &'static str,
    },
    /// A module's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        row_id: String,
    },
    /// A module's downgrade path disagrees with the recomputed path.
    DowngradePathMismatch {
        /// Row id.
        row_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A narrowed or withheld module offers no recovery path.
    MissingDowngradePath {
        /// Row id.
        row_id: String,
    },
    /// A withheld module still offers actions.
    WithheldModuleOffersActions {
        /// Row id.
        row_id: String,
    },
    /// A module touches high-risk data but does not exclude it always.
    HighRiskNotExcluded {
        /// Row id.
        row_id: String,
        /// Redaction-default token.
        redaction_default: &'static str,
    },
    /// A module shares off-machine without reusing the export-consent descriptor.
    SharingWithoutConsentInspector {
        /// Row id.
        row_id: String,
    },
    /// A module publishes a clean operational claim but narrows a state or carries a reason.
    PublishedModuleNotWhole {
        /// Row id.
        row_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints matrix truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5SupportCenterMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateModuleRow { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::DuplicateModule { module } => write!(f, "module {module} has more than one row"),
            Self::MissingModule { module } => write!(f, "missing row for module {module}"),
            Self::NoInspectors { row_id } => write!(f, "row {row_id} reuses no inspector"),
            Self::DuplicateInspector { row_id, inspector } => {
                write!(f, "row {row_id} reuses inspector {inspector} more than once")
            }
            Self::InspectorBindingIncomplete { row_id, inspector } => {
                write!(
                    f,
                    "row {row_id} inspector {inspector} is missing its descriptor ref or timestamp"
                )
            }
            Self::DuplicateExportMode { row_id, mode } => {
                write!(f, "row {row_id} offers export mode {mode} more than once")
            }
            Self::DuplicateDataClass { row_id, data_class } => {
                write!(f, "row {row_id} lists data class {data_class} more than once")
            }
            Self::DuplicateDowngradeReason { row_id, reason } => {
                write!(f, "row {row_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedReadiness {
                row_id,
                published,
                computed,
            } => write!(
                f,
                "row {row_id} publishes readiness {published} but the gate computes {computed}"
            ),
            Self::PublicationMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records publication {declared} but the gate requires {required}"
            ),
            Self::DowngradeReasonsMismatch { row_id } => {
                write!(f, "row {row_id} downgrade reasons disagree with the gate")
            }
            Self::DowngradePathMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::MissingDowngradePath { row_id } => {
                write!(
                    f,
                    "row {row_id} is narrowed or withheld but offers no recovery path"
                )
            }
            Self::WithheldModuleOffersActions { row_id } => {
                write!(f, "row {row_id} is withheld but still offers actions")
            }
            Self::HighRiskNotExcluded {
                row_id,
                redaction_default,
            } => write!(
                f,
                "row {row_id} touches high-risk data but defaults to {redaction_default}, not excluded_always"
            ),
            Self::SharingWithoutConsentInspector { row_id } => {
                write!(
                    f,
                    "row {row_id} offers a sharing export mode without reusing export_consent"
                )
            }
            Self::PublishedModuleNotWhole { row_id } => {
                write!(
                    f,
                    "row {row_id} publishes operational but narrows a state or carries a downgrade reason"
                )
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve matrix truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for M5SupportCenterMatrixViolation {}

/// Stable record-kind tag for [`M5SupportCenterMatrixSupportExport`].
pub const M5_SUPPORT_CENTER_MATRIX_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_support_center_matrix_support_export";

/// Support-export wrapper preserving the matrix verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportCenterMatrixSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub matrix_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact matrix preserved by the export.
    pub matrix: M5SupportCenterMatrix,
}

impl M5SupportCenterMatrixSupportExport {
    /// Whether the export preserves the same packet id and a clean matrix.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_SUPPORT_CENTER_MATRIX_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_SUPPORT_CENTER_MATRIX_SCHEMA_VERSION
            && self.matrix_packet_id_ref == self.matrix.packet_id
            && self.raw_private_material_excluded
            && self.matrix.validate().is_empty()
    }
}

/// Loads the embedded M5 Support Center matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SupportCenterMatrix`].
pub fn current_m5_support_center_matrix() -> Result<M5SupportCenterMatrix, serde_json::Error> {
    serde_json::from_str(M5_SUPPORT_CENTER_MATRIX_JSON)
}

#[cfg(test)]
mod tests;
