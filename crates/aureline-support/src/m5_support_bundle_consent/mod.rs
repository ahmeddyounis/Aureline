//! Support-bundle consent sheets that show what an export would include, exclude, and policy-lock by
//! data class before any packet leaves the machine.
//!
//! Where the support-bundle manifest owns *what a bundle would contain* and the redaction defaults own
//! *how each class is handled*, this packet governs *how the export is consented to so nothing leaves
//! the machine opaquely*. It is a registry of consent sheets, one per export scenario worth reviewing,
//! each carrying the included / excluded / policy-locked counts by data class, the visible schema
//! version, the retention note, the destination class, the class-safe redaction toggles, and — equal in
//! prominence to any upload or formal-support send — the local-save path. It reuses the support-bundle
//! truth by reference: every sheet carries a `source_of_truth_ref` projecting from the existing
//! support-bundle manifest and redaction profile rather than re-deriving any bundle of its own.
//!
//! The readiness analogue here is a fail-closed **consent gate**. The guardrail the source set treats
//! as core support UX is that a consent sheet must never present a clean "ready to export" affordance
//! that hides a policy lock, a silent redaction override, a stale schema, or — above all — content that
//! cannot safely leave the machine, and must never make the local-save or no-export path look secondary
//! to upload. Each sheet therefore publishes a [`ConsentPresentation`] that is the weaker of two
//! ceilings: its [`ConsentStatus`] ceiling (a clean, send-safe sheet presents [`ConsentPresentation::ReviewReady`];
//! a policy lock or a redaction override narrows it to [`ConsentPresentation::NarrowedReview`]; content
//! that cannot leave the machine caps it at [`ConsentPresentation::SendBlocked`]) and its
//! [`SchemaFreshness`] ceiling (a current schema presents transparently; a stale one narrows it). A sheet
//! can never claim a cleaner presentation than its inputs support, and a stricter rule still holds:
//! local-save must always be at least as prominent as every send path, and secret-bearing classes stay
//! excluded by default and unexportable, both enforced as hard invariants rather than soft downgrades.
//!
//! Every sheet always carries its one-step `explain_entrypoint_ref` — the inspectable "Review what this
//! export includes" answer — and its `cli_object_ref`, the CLI / headless equivalent, so the same
//! consent answer is reachable from the active Support Center, the CLI / headless export review, and the
//! formal-support handoff. Every required consumer surface binds to this one registry via a
//! [`ConsentConsumerBinding`] that must ingest it, preserve its consent vocabulary and object ids, keep
//! local-save first-class, and narrow with it, so desktop, CLI / headless, support export, and formal
//! support handoff share one consent grammar.
//!
//! The packet is checked in at `artifacts/support/m5/m5-support-bundle-consent.json` and embedded here.
//! It is metadata-only: every field is a typed state, a count, or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, clipboard history, or secret-bearing payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported support-bundle-consent schema version.
pub const M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SUPPORT_BUNDLE_CONSENT_RECORD_KIND: &str = "m5_support_bundle_consent";

/// Repo-relative path to the checked-in packet.
pub const M5_SUPPORT_BUNDLE_CONSENT_PATH: &str =
    "artifacts/support/m5/m5-support-bundle-consent.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_REF: &str =
    "schemas/support/m5-support-bundle-consent.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SUPPORT_BUNDLE_CONSENT_DOC_REF: &str =
    "docs/help/support/m5-support-bundle-consent.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_SUPPORT_BUNDLE_CONSENT_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-support-bundle-consent.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SUPPORT_BUNDLE_CONSENT_FIXTURE_DIR: &str =
    "fixtures/support/m5/m5-support-bundle-consent";

/// Repo-relative path to the shiproom review packet that renders this registry.
pub const M5_SUPPORT_BUNDLE_CONSENT_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-support-bundle-consent-review-packet/support_bundle_consent_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_SUPPORT_BUNDLE_CONSENT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-support-bundle-consent.json"
));

/// A diagnostic data class an export is reviewed against.
///
/// Mirrors `diagnostic_data_class` in the support-bundle boundary schemas so the consent sheet, the
/// manifest, and the export writer resolve to the same token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentDataClass {
    /// Build ids, version, policy fingerprints, summary counters.
    MetadataOnly,
    /// Toolchain versions, target classes, route summaries.
    EnvironmentAdjacent,
    /// Filenames, stack traces, snippets, command-argument summaries.
    CodeAdjacent,
    /// Secret-bearing material, raw dumps, full transcripts.
    HighRisk,
}

impl ConsentDataClass {
    /// Every data class, in declaration order (least to most sensitive).
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

    /// Whether this class carries secret-bearing material that stays excluded by default and unexportable.
    pub const fn is_secret_bearing(self) -> bool {
        matches!(self, Self::HighRisk)
    }
}

/// The destination an export would be sent to.
///
/// Mirrors `delivery_path_class` in `schemas/support/escalation_packet.schema.json` so the consent
/// sheet, the reopen manifest, and the escalation packet share one destination vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentDestinationClass {
    /// A local-only review or save; the packet never leaves the machine.
    LocalOnlyReview,
    /// A handoff attached to a vendor support case.
    VendorCaseHandoff,
    /// A user-initiated upload to a hosted intake.
    UserInitiatedUpload,
    /// A handoff to a managed / administrator support channel.
    ManagedAdminHandoff,
    /// A private security-disclosure channel.
    PrivateSecurityChannel,
}

impl ConsentDestinationClass {
    /// Every destination class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnlyReview,
        Self::VendorCaseHandoff,
        Self::UserInitiatedUpload,
        Self::ManagedAdminHandoff,
        Self::PrivateSecurityChannel,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyReview => "local_only_review",
            Self::VendorCaseHandoff => "vendor_case_handoff",
            Self::UserInitiatedUpload => "user_initiated_upload",
            Self::ManagedAdminHandoff => "managed_admin_handoff",
            Self::PrivateSecurityChannel => "private_security_channel",
        }
    }

    /// Whether the packet stays on the machine (a local save / review) rather than being sent.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnlyReview)
    }

    /// Whether selecting this destination causes the packet to leave the machine.
    pub const fn leaves_machine(self) -> bool {
        !self.is_local_only()
    }
}

/// How prominent an export path is in the consent sheet.
///
/// The gate enforces that the local-save path is never less prominent than any send path, so an
/// upload-first surface can never make local-save or no-export look secondary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathProminence {
    /// The path is presented as the primary, default affordance.
    Primary,
    /// The path is presented co-equal to the other paths.
    CoEqual,
    /// The path is presented as a secondary affordance.
    Secondary,
}

impl PathProminence {
    /// Every prominence level, most prominent first.
    pub const ALL: [Self; 3] = [Self::Primary, Self::CoEqual, Self::Secondary];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::CoEqual => "co_equal",
            Self::Secondary => "secondary",
        }
    }

    /// Prominence rank; higher is more prominent. Used to prove local-save is never out-shouted by send.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Primary => 2,
            Self::CoEqual => 1,
            Self::Secondary => 0,
        }
    }
}

/// How long the export's destination retains the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Not retained anywhere off the machine; a local save / review only.
    NotRetainedLocalOnly,
    /// Retained for the life of a single support case, then purged.
    ShortLivedCaseAttachment,
    /// Retained under the standard support-data retention window.
    StandardSupportRetention,
    /// Retained under an extended compliance hold.
    ExtendedComplianceHold,
    /// Retained under a vendor-defined policy named in the retention note.
    VendorDefinedRetention,
}

impl RetentionClass {
    /// Every retention class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotRetainedLocalOnly,
        Self::ShortLivedCaseAttachment,
        Self::StandardSupportRetention,
        Self::ExtendedComplianceHold,
        Self::VendorDefinedRetention,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRetainedLocalOnly => "not_retained_local_only",
            Self::ShortLivedCaseAttachment => "short_lived_case_attachment",
            Self::StandardSupportRetention => "standard_support_retention",
            Self::ExtendedComplianceHold => "extended_compliance_hold",
            Self::VendorDefinedRetention => "vendor_defined_retention",
        }
    }
}

/// Whether the visible bundle schema version is current or stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaFreshness {
    /// The bundle schema version matches the current support-bundle schema.
    Current,
    /// The bundle schema version is older than the current one; warn before relying on or sending it.
    Stale,
}

impl SchemaFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Current, Self::Stale];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }

    /// Whether the schema is stale.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }

    /// Highest presentation this freshness permits.
    pub const fn presentation_ceiling(self) -> ConsentPresentation {
        match self {
            Self::Current => ConsentPresentation::ReviewReady,
            Self::Stale => ConsentPresentation::NarrowedReview,
        }
    }
}

/// The overall consent disposition of a sheet — the headline reason it is or is not cleanly reviewable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStatus {
    /// The export is reviewable and send-safe: nothing policy-locked, nothing overridden, nothing unsafe.
    ReviewReady,
    /// A policy lock excludes one or more classes; the locked content is shown as excluded, not hidden.
    PolicyNarrowed,
    /// A class-safe redaction toggle was changed from its default; surfaced so the override is not silent.
    RedactionAdjusted,
    /// Content selected for a send destination cannot safely leave the machine; the send is blocked.
    SendBlocked,
}

impl ConsentStatus {
    /// Every consent status, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewReady,
        Self::PolicyNarrowed,
        Self::RedactionAdjusted,
        Self::SendBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewReady => "review_ready",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::RedactionAdjusted => "redaction_adjusted",
            Self::SendBlocked => "send_blocked",
        }
    }

    /// Highest presentation this status permits.
    pub const fn presentation_ceiling(self) -> ConsentPresentation {
        match self {
            Self::ReviewReady => ConsentPresentation::ReviewReady,
            Self::PolicyNarrowed | Self::RedactionAdjusted => ConsentPresentation::NarrowedReview,
            Self::SendBlocked => ConsentPresentation::SendBlocked,
        }
    }

    /// Whether the consent status itself needs the user to act, beyond a stale-schema warning.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::ReviewReady)
    }

    /// Whether this status names blockers the user must reconcile before sending.
    pub const fn requires_blockers(self) -> bool {
        matches!(self, Self::PolicyNarrowed | Self::SendBlocked)
    }
}

/// The presentation the consent gate publishes for a sheet, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentPresentation {
    /// The export is fully reviewable and send-safe; what is included, excluded, and retained is clear.
    ReviewReady,
    /// The sheet is shown but narrowed: a policy lock, a redaction override, or a stale schema needs
    /// attention. What is included and excluded stays visible, and local-save stays first-class.
    NarrowedReview,
    /// A send destination is selected but the content cannot safely leave the machine; the sheet warns
    /// and blocks the send before any packet leaves.
    SendBlocked,
}

impl ConsentPresentation {
    /// Every presentation, highest to lowest.
    pub const ALL: [Self; 3] = [Self::ReviewReady, Self::NarrowedReview, Self::SendBlocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewReady => "review_ready",
            Self::NarrowedReview => "narrowed_review",
            Self::SendBlocked => "send_blocked",
        }
    }

    /// Rank for the fail-closed gate; higher is more permissive.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ReviewReady => 2,
            Self::NarrowedReview => 1,
            Self::SendBlocked => 0,
        }
    }

    /// Whether the gate narrowed or blocked the sheet below a fully reviewable, send-safe export.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::ReviewReady)
    }

    /// Whether the sheet must warn and block before a packet leaves the machine.
    pub const fn warns_before_send(self) -> bool {
        matches!(self, Self::SendBlocked)
    }
}

/// The weaker (lower-rank) of two presentations.
fn weaker(a: ConsentPresentation, b: ConsentPresentation) -> ConsentPresentation {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// A headline reason the consent gate narrows or blocks a sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentDowngradeReason {
    /// A policy lock excludes one or more classes from the export.
    DestinationPolicyLocked,
    /// A class-safe redaction toggle was changed from its default.
    RedactionOverrideApplied,
    /// Content selected for a send destination cannot safely leave the machine.
    ExportBlockedUnsafeContent,
    /// The visible bundle schema version is stale relative to the current schema.
    StaleSchemaWarning,
}

impl ConsentDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DestinationPolicyLocked,
        Self::RedactionOverrideApplied,
        Self::ExportBlockedUnsafeContent,
        Self::StaleSchemaWarning,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DestinationPolicyLocked => "destination_policy_locked",
            Self::RedactionOverrideApplied => "redaction_override_applied",
            Self::ExportBlockedUnsafeContent => "export_blocked_unsafe_content",
            Self::StaleSchemaWarning => "stale_schema_warning",
        }
    }
}

/// How a data class is handled in the export, before and after any user toggle.
///
/// Mirrors `redaction_state` in the support-bundle boundary schemas so the consent sheet and the
/// manifest resolve to the same redaction vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionMode {
    /// Metadata only; no redaction needed.
    NotRequiredMetadata,
    /// A redacted summary is exported.
    RedactedSummary,
    /// A sanitized snapshot is exported.
    SanitizedSnapshot,
    /// Retained on the machine only; never exported.
    RetainedLocalOnly,
    /// Omitted, awaiting an explicit opt-in.
    OmittedPendingOptIn,
    /// Prohibited; never exported under any choice.
    Prohibited,
    /// Locked by policy; the user cannot change the handling.
    PolicyLocked,
}

impl RedactionMode {
    /// Every redaction mode, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotRequiredMetadata,
        Self::RedactedSummary,
        Self::SanitizedSnapshot,
        Self::RetainedLocalOnly,
        Self::OmittedPendingOptIn,
        Self::Prohibited,
        Self::PolicyLocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredMetadata => "not_required_metadata",
            Self::RedactedSummary => "redacted_summary",
            Self::SanitizedSnapshot => "sanitized_snapshot",
            Self::RetainedLocalOnly => "retained_local_only",
            Self::OmittedPendingOptIn => "omitted_pending_opt_in",
            Self::Prohibited => "prohibited",
            Self::PolicyLocked => "policy_locked",
        }
    }

    /// Whether content handled this way may be exported off the machine.
    ///
    /// Only metadata, a redacted summary, or a sanitized snapshot may leave the machine; retained,
    /// omitted, prohibited, and policy-locked content may not.
    pub const fn is_exportable_off_machine(self) -> bool {
        matches!(
            self,
            Self::NotRequiredMetadata | Self::RedactedSummary | Self::SanitizedSnapshot
        )
    }
}

/// How a data class is included or excluded by default in the export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultInclusion {
    /// Included by default (the user can deselect or redact it further).
    IncludedByDefault,
    /// Excluded by default; the user may opt in only where policy allows.
    ExcludedByDefault,
    /// Never exportable under any choice.
    NonExportable,
}

impl DefaultInclusion {
    /// Every default-inclusion state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::IncludedByDefault,
        Self::ExcludedByDefault,
        Self::NonExportable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedByDefault => "included_by_default",
            Self::ExcludedByDefault => "excluded_by_default",
            Self::NonExportable => "non_exportable",
        }
    }

    /// Whether content is excluded (or unexportable) by default rather than included.
    pub const fn is_excluded_by_default(self) -> bool {
        !matches!(self, Self::IncludedByDefault)
    }
}

/// A downstream surface that must ingest this registry and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentConsumerSurface {
    /// The desktop Support Center's export-review surface.
    SupportCenter,
    /// The CLI / headless export-review path.
    CliHeadless,
    /// The formal support-handoff packet.
    FormalSupportHandoff,
    /// The support export of the consent review itself.
    SupportExport,
}

impl ConsentConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::SupportCenter,
        Self::CliHeadless,
        Self::FormalSupportHandoff,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportCenter => "support_center",
            Self::CliHeadless => "cli_headless",
            Self::FormalSupportHandoff => "formal_support_handoff",
            Self::SupportExport => "support_export",
        }
    }
}

/// A class-safe redaction toggle the consent sheet offers (or locks) for a data class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedactionToggle {
    /// Whether the user may change the handling of this class.
    pub available: bool,
    /// The handling currently applied.
    pub current_mode: RedactionMode,
    /// The default handling for this class.
    pub default_mode: RedactionMode,
    /// The modes the user may select, where policy allows.
    #[serde(default)]
    pub allowed_modes: Vec<RedactionMode>,
    /// Whether the handling is locked by policy.
    pub policy_locked: bool,
}

impl RedactionToggle {
    /// Whether the current handling differs from the default — a user-applied override.
    pub fn is_override_applied(&self) -> bool {
        self.available && self.current_mode != self.default_mode
    }

    /// Whether the toggle's modes are internally consistent.
    ///
    /// The allowed set must be non-empty and contain both the current and default modes; an
    /// unavailable or policy-locked toggle may offer only its current, default-equal mode, so a
    /// non-adjustable class can never hide a second option.
    pub fn is_well_formed(&self) -> bool {
        if self.allowed_modes.is_empty()
            || !self.allowed_modes.contains(&self.current_mode)
            || !self.allowed_modes.contains(&self.default_mode)
        {
            return false;
        }
        if !self.available || self.policy_locked {
            return self.allowed_modes == vec![self.current_mode]
                && self.current_mode == self.default_mode;
        }
        true
    }
}

/// One data-class row in a consent sheet: the included / excluded / policy-locked counts and the
/// class-safe redaction toggle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsentClassRow {
    /// The data class this row covers.
    pub data_class: ConsentDataClass,
    /// Number of bundle sections of this class that would be included.
    pub included_count: usize,
    /// Number of bundle sections of this class that are excluded (deselected, omitted, or unavailable).
    pub excluded_count: usize,
    /// Number of bundle sections of this class that are locked out by policy.
    pub policy_locked_count: usize,
    /// How this class is included or excluded by default.
    pub default_inclusion: DefaultInclusion,
    /// The class-safe redaction toggle for this class.
    pub redaction_toggle: RedactionToggle,
    /// Reviewer-facing note for this class.
    pub note: String,
}

impl ConsentClassRow {
    /// Total sections of this class across every disposition.
    pub fn total(&self) -> usize {
        self.included_count + self.excluded_count + self.policy_locked_count
    }

    /// Whether this class would put content into the export.
    pub fn is_included(&self) -> bool {
        self.included_count > 0
    }

    /// Whether the row carries its non-empty note and a well-formed toggle.
    pub fn is_well_formed(&self) -> bool {
        !self.note.trim().is_empty() && self.redaction_toggle.is_well_formed()
    }
}

/// One export-destination option offered by a consent sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsentDestination {
    /// The destination class.
    pub destination_class: ConsentDestinationClass,
    /// How prominent this path is in the sheet.
    pub prominence: PathProminence,
    /// Whether this path is offered for the current sheet.
    pub enabled: bool,
    /// Whether this path is the one selected for this sheet.
    pub selected: bool,
    /// Whether selecting this path causes the packet to leave the machine; must match the class.
    pub leaves_machine: bool,
    /// Human-readable label (e.g. "Save bundle locally").
    pub label: String,
    /// Ref to the destination's wiring.
    pub destination_ref: String,
}

impl ConsentDestination {
    /// Whether this is a local-save / local-only-review path.
    pub fn is_local_save(&self) -> bool {
        self.destination_class.is_local_only()
    }

    /// Whether the row's `leaves_machine` flag matches its destination class.
    pub fn leaves_machine_consistent(&self) -> bool {
        self.leaves_machine == self.destination_class.leaves_machine()
    }

    /// Whether the destination carries its non-empty label and ref.
    pub fn is_well_formed(&self) -> bool {
        !self.label.trim().is_empty() && !self.destination_ref.trim().is_empty()
    }
}

/// One consent sheet: what a single export scenario would include, exclude, and policy-lock, plus the
/// destination, retention note, schema version, redaction toggles, and local-save parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportBundleConsentSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Human-readable label for the sheet (e.g. "Upload to vendor case").
    pub title: String,
    /// The visible bundle schema version (e.g. "support-bundle v1").
    pub schema_version_label: String,
    /// The current bundle schema version, for comparison; equal to the visible one when not stale.
    pub current_schema_version_label: String,
    /// Whether the visible schema version is current or stale.
    pub schema_freshness: SchemaFreshness,
    /// How long the destination retains the packet.
    pub retention_class: RetentionClass,
    /// Reviewer-facing retention note.
    pub retention_note: String,
    /// The destination class selected for this sheet; must equal the selected destination's class.
    pub selected_destination_class: ConsentDestinationClass,
    /// Overall consent disposition; must equal the recomputed status.
    pub consent_status: ConsentStatus,
    /// Presentation actually published after the gate; must equal the recomputed decision.
    pub presentation: ConsentPresentation,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<ConsentDowngradeReason>,
    /// Attestation that the local-save path is at least as prominent as every send path; must equal the
    /// recomputed parity.
    pub local_save_first_class: bool,
    /// True when the sheet warns and blocks before a packet leaves the machine; required iff send-blocked.
    pub blocked_before_send: bool,
    /// Attestation that no raw secret bodies, clipboard history, or raw payloads are carried; always true.
    pub raw_material_excluded: bool,
    /// One row per data class; all four classes are required exactly once.
    #[serde(default)]
    pub class_rows: Vec<ConsentClassRow>,
    /// The export-destination options; at least one enabled local-save path is required.
    #[serde(default)]
    pub destinations: Vec<ConsentDestination>,
    /// Caveats attached to a narrowed or blocked sheet.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// The blockers the user must reconcile before sending (policy locks, unsafe content).
    #[serde(default)]
    pub blockers: Vec<String>,
    /// Ref to the support-bundle manifest / redaction profile this sheet projects.
    pub source_of_truth_ref: String,
    /// One-step "Review what this export includes" entrypoint; always present.
    pub explain_entrypoint_ref: String,
    /// The equivalent CLI / headless object id; always present.
    pub cli_object_ref: String,
    /// Ref to the conformance suite backing the sheet.
    pub conformance_ref: String,
    /// Ref to the sheet's supporting evidence.
    pub evidence_ref: String,
    /// Ref to the machine-readable consent receipt.
    pub consent_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl SupportBundleConsentSheet {
    /// The row for the given data class, if present.
    pub fn class_row(&self, data_class: ConsentDataClass) -> Option<&ConsentClassRow> {
        self.class_rows.iter().find(|r| r.data_class == data_class)
    }

    /// Total included sections across every class.
    pub fn included_total(&self) -> usize {
        self.class_rows.iter().map(|r| r.included_count).sum()
    }

    /// Total excluded sections across every class.
    pub fn excluded_total(&self) -> usize {
        self.class_rows.iter().map(|r| r.excluded_count).sum()
    }

    /// Total policy-locked sections across every class.
    pub fn policy_locked_total(&self) -> usize {
        self.class_rows.iter().map(|r| r.policy_locked_count).sum()
    }

    /// Whether any class carries a user-applied redaction override.
    pub fn has_redaction_override(&self) -> bool {
        self.class_rows
            .iter()
            .any(|r| r.redaction_toggle.is_override_applied())
    }

    /// The selected destination, if exactly one is declared.
    pub fn selected_destination(&self) -> Option<&ConsentDestination> {
        let mut selected = self.destinations.iter().filter(|d| d.selected);
        let first = selected.next()?;
        if selected.next().is_some() {
            None
        } else {
            Some(first)
        }
    }

    /// The enabled local-save destinations.
    pub fn local_save_destinations(&self) -> impl Iterator<Item = &ConsentDestination> {
        self.destinations
            .iter()
            .filter(|d| d.is_local_save() && d.enabled)
    }

    /// The enabled send (leaves-machine) destinations.
    pub fn send_destinations(&self) -> impl Iterator<Item = &ConsentDestination> {
        self.destinations
            .iter()
            .filter(|d| d.leaves_machine && d.enabled)
    }

    /// The highest send-path prominence rank, or `0` when no send path is enabled.
    pub fn max_send_prominence_rank(&self) -> u8 {
        self.send_destinations()
            .map(|d| d.prominence.rank())
            .max()
            .unwrap_or(0)
    }

    /// Whether an enabled local-save path exists and is at least as prominent as every send path.
    pub fn local_save_is_first_class(&self) -> bool {
        let max_local = self
            .local_save_destinations()
            .map(|d| d.prominence.rank())
            .max();
        match max_local {
            Some(local_rank) => local_rank >= self.max_send_prominence_rank(),
            None => false,
        }
    }

    /// Whether the content selected for the chosen destination cannot safely leave the machine.
    ///
    /// Only relevant when the selected destination leaves the machine: any included class that is
    /// secret-bearing, or whose current redaction handling is not exportable off the machine, makes the
    /// send unsafe.
    pub fn send_unsafe(&self) -> bool {
        match self.selected_destination() {
            Some(dest) if dest.leaves_machine => self.class_rows.iter().any(|r| {
                r.is_included()
                    && (r.data_class.is_secret_bearing()
                        || !r.redaction_toggle.current_mode.is_exportable_off_machine())
            }),
            _ => false,
        }
    }

    /// The consent status recomputed from the sheet's observed states.
    ///
    /// Unsafe send dominates a policy lock, which dominates a redaction override; a clean sheet is
    /// review-ready.
    pub fn computed_status(&self) -> ConsentStatus {
        if self.send_unsafe() {
            ConsentStatus::SendBlocked
        } else if self.policy_locked_total() > 0 {
            ConsentStatus::PolicyNarrowed
        } else if self.has_redaction_override() {
            ConsentStatus::RedactionAdjusted
        } else {
            ConsentStatus::ReviewReady
        }
    }

    /// Highest presentation the consent status permits.
    pub fn status_ceiling(&self) -> ConsentPresentation {
        self.computed_status().presentation_ceiling()
    }

    /// Highest presentation the schema freshness permits.
    pub fn schema_ceiling(&self) -> ConsentPresentation {
        self.schema_freshness.presentation_ceiling()
    }

    /// The presentation the gate permits this sheet to publish.
    ///
    /// Lowers the clean baseline to the weaker of the status ceiling and the schema-freshness ceiling,
    /// so a policy lock, a redaction override, unsafe content, or a stale schema can never present a
    /// fuller claim than the inputs support.
    pub fn effective_presentation(&self) -> ConsentPresentation {
        weaker(self.status_ceiling(), self.schema_ceiling())
    }

    /// The headline downgrade reasons recomputed from the sheet's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<ConsentDowngradeReason> {
        ConsentDowngradeReason::ALL
            .into_iter()
            .filter(|reason| match reason {
                ConsentDowngradeReason::DestinationPolicyLocked => self.policy_locked_total() > 0,
                ConsentDowngradeReason::RedactionOverrideApplied => self.has_redaction_override(),
                ConsentDowngradeReason::ExportBlockedUnsafeContent => self.send_unsafe(),
                ConsentDowngradeReason::StaleSchemaWarning => self.schema_freshness.is_stale(),
            })
            .collect()
    }

    /// Whether the sheet presents a fully reviewable, send-safe export.
    pub fn is_review_ready(&self) -> bool {
        self.effective_presentation() == ConsentPresentation::ReviewReady
    }

    /// Whether the sheet carries its own non-empty one-step explain and CLI-equivalent refs.
    pub fn has_one_step_explainability(&self) -> bool {
        !self.explain_entrypoint_ref.trim().is_empty() && !self.cli_object_ref.trim().is_empty()
    }

    /// Whether the recorded status, presentation, reasons, parity, and blocked flag agree with the gate.
    pub fn gate_consistent(&self) -> bool {
        let effective = self.effective_presentation();
        self.consent_status == self.computed_status()
            && self.presentation == effective
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.local_save_first_class == self.local_save_is_first_class()
            && self.blocked_before_send == effective.warns_before_send()
    }
}

/// One binding wiring a downstream surface to this registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsentConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: ConsentConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet id this surface ingests.
    pub packet_id_ref: String,
    /// True when the surface ingests this registry rather than a parallel list.
    pub ingests_registry: bool,
    /// True when the surface preserves the consent vocabulary verbatim.
    pub preserves_consent_vocabulary: bool,
    /// True when the surface preserves the sheet and CLI object ids rather than reminting them.
    pub preserves_object_ids: bool,
    /// True when the surface keeps the local-save path at least as prominent as every send path.
    pub local_save_first_class: bool,
    /// True when the surface narrows automatically as sheets are narrowed or blocked.
    pub narrows_on_downgrade: bool,
    /// True when raw secret, clipboard, or payload material is excluded from the binding.
    pub raw_material_excluded: bool,
}

impl ConsentConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.ingests_registry
            && self.preserves_consent_vocabulary
            && self.preserves_object_ids
            && self.local_save_first_class
            && self.narrows_on_downgrade
            && self.raw_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportBundleConsentSummary {
    /// Total consent sheets.
    pub total_sheets: usize,
    /// Sheets that present a fully reviewable, send-safe export.
    pub review_ready_sheets: usize,
    /// Sheets the gate narrowed.
    pub narrowed_sheets: usize,
    /// Sheets the gate blocked from sending.
    pub send_blocked_sheets: usize,
    /// Sheets carrying at least one policy-locked class.
    pub sheets_with_policy_locks: usize,
    /// Sheets carrying at least one user-applied redaction override.
    pub sheets_with_redaction_overrides: usize,
    /// Sheets warning on a stale schema version.
    pub stale_schema_sheets: usize,
    /// Sheets that keep the local-save path first-class; equals total when the gate passes.
    pub local_save_first_class_sheets: usize,
    /// Total included sections across all sheets.
    pub total_included: usize,
    /// Total excluded sections across all sheets.
    pub total_excluded: usize,
    /// Total policy-locked sections across all sheets.
    pub total_policy_locked: usize,
}

/// A redaction-safe export row projected from a consent sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportBundleConsentExportRow {
    /// Sheet id.
    pub sheet_id: String,
    /// Visible schema version label.
    pub schema_version_label: String,
    /// Schema-freshness token.
    pub schema_freshness: String,
    /// Retention-class token.
    pub retention_class: String,
    /// Selected-destination-class token.
    pub selected_destination_class: String,
    /// Consent-status token.
    pub consent_status: String,
    /// Published-presentation token.
    pub presentation: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Whether the local-save path stays first-class.
    pub local_save_first_class: bool,
    /// Whether the sheet warns and blocks before sending.
    pub blocked_before_send: bool,
    /// Included sections in this sheet.
    pub included_total: usize,
    /// Excluded sections in this sheet.
    pub excluded_total: usize,
    /// Policy-locked sections in this sheet.
    pub policy_locked_total: usize,
    /// One-step explain entrypoint ref.
    pub explain_entrypoint_ref: String,
    /// CLI / headless equivalent object id.
    pub cli_object_ref: String,
    /// Source-of-truth ref.
    pub source_of_truth_ref: String,
    /// Consent-receipt ref.
    pub consent_receipt_ref: String,
    /// Whether the sheet presents as review-ready.
    pub review_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the registry — the canonical consent index downstream
/// surfaces render instead of restating each export scenario by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportBundleConsentExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5SupportBundleConsentExportRow>,
    /// Whether every sheet's published presentation and decision agree with the gate.
    pub all_sheets_gate_consistent: bool,
    /// Whether every sheet keeps the local-save path first-class.
    pub all_local_save_first_class: bool,
    /// Sheets that present as review-ready.
    pub review_ready_count: usize,
    /// Sheets the gate narrowed.
    pub narrowed_count: usize,
    /// Sheets the gate blocked from sending.
    pub send_blocked_count: usize,
}

/// The typed support-bundle-consent registry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SupportBundleConsent {
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
    /// Closed data-class vocabulary.
    pub data_classes: Vec<ConsentDataClass>,
    /// Closed destination-class vocabulary.
    pub destination_classes: Vec<ConsentDestinationClass>,
    /// Closed path-prominence vocabulary.
    pub path_prominences: Vec<PathProminence>,
    /// Closed retention-class vocabulary.
    pub retention_classes: Vec<RetentionClass>,
    /// Closed schema-freshness vocabulary.
    pub schema_freshnesses: Vec<SchemaFreshness>,
    /// Closed consent-status vocabulary.
    pub consent_statuses: Vec<ConsentStatus>,
    /// Closed presentation vocabulary.
    pub presentations: Vec<ConsentPresentation>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<ConsentDowngradeReason>,
    /// Closed redaction-mode vocabulary.
    pub redaction_modes: Vec<RedactionMode>,
    /// Closed default-inclusion vocabulary.
    pub default_inclusions: Vec<DefaultInclusion>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ConsentConsumerSurface>,
    /// Consent sheets, one per export scenario worth reviewing.
    #[serde(default)]
    pub sheets: Vec<SupportBundleConsentSheet>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<ConsentConsumerBinding>,
    /// Summary counts.
    pub summary: M5SupportBundleConsentSummary,
}

impl M5SupportBundleConsent {
    /// Returns the sheet with the given id.
    pub fn sheet(&self, sheet_id: &str) -> Option<&SupportBundleConsentSheet> {
        self.sheets.iter().find(|s| s.sheet_id == sheet_id)
    }

    /// Sheets that present as review-ready.
    pub fn review_ready_sheets(&self) -> impl Iterator<Item = &SupportBundleConsentSheet> {
        self.sheets
            .iter()
            .filter(|s| s.effective_presentation() == ConsentPresentation::ReviewReady)
    }

    /// Sheets the gate narrowed.
    pub fn narrowed_sheets(&self) -> impl Iterator<Item = &SupportBundleConsentSheet> {
        self.sheets
            .iter()
            .filter(|s| s.effective_presentation() == ConsentPresentation::NarrowedReview)
    }

    /// Sheets the gate blocked from sending.
    pub fn send_blocked_sheets(&self) -> impl Iterator<Item = &SupportBundleConsentSheet> {
        self.sheets
            .iter()
            .filter(|s| s.effective_presentation() == ConsentPresentation::SendBlocked)
    }

    /// Whether a consumer binding preserves this registry for the given surface.
    pub fn has_binding_for(&self, surface: ConsentConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every sheet's recorded decision agrees with the gate.
    pub fn all_sheets_gate_consistent(&self) -> bool {
        self.sheets
            .iter()
            .all(SupportBundleConsentSheet::gate_consistent)
    }

    /// Whether every sheet keeps the local-save path first-class.
    pub fn all_local_save_first_class(&self) -> bool {
        self.sheets
            .iter()
            .all(SupportBundleConsentSheet::local_save_is_first_class)
    }

    /// Recomputes the summary block from the sheets.
    pub fn computed_summary(&self) -> M5SupportBundleConsentSummary {
        let count_presentation = |decision: ConsentPresentation| {
            self.sheets
                .iter()
                .filter(|s| s.effective_presentation() == decision)
                .count()
        };
        let mut total_included = 0usize;
        let mut total_excluded = 0usize;
        let mut total_policy_locked = 0usize;
        for sheet in &self.sheets {
            total_included += sheet.included_total();
            total_excluded += sheet.excluded_total();
            total_policy_locked += sheet.policy_locked_total();
        }
        M5SupportBundleConsentSummary {
            total_sheets: self.sheets.len(),
            review_ready_sheets: count_presentation(ConsentPresentation::ReviewReady),
            narrowed_sheets: count_presentation(ConsentPresentation::NarrowedReview),
            send_blocked_sheets: count_presentation(ConsentPresentation::SendBlocked),
            sheets_with_policy_locks: self
                .sheets
                .iter()
                .filter(|s| s.policy_locked_total() > 0)
                .count(),
            sheets_with_redaction_overrides: self
                .sheets
                .iter()
                .filter(|s| s.has_redaction_override())
                .count(),
            stale_schema_sheets: self
                .sheets
                .iter()
                .filter(|s| s.schema_freshness.is_stale())
                .count(),
            local_save_first_class_sheets: self
                .sheets
                .iter()
                .filter(|s| s.local_save_is_first_class())
                .count(),
            total_included,
            total_excluded,
            total_policy_locked,
        }
    }

    /// Produces the consent index downstream surfaces render instead of restating each export scenario
    /// by hand.
    pub fn export_projection(&self) -> M5SupportBundleConsentExportProjection {
        let rows = self
            .sheets
            .iter()
            .map(|s| M5SupportBundleConsentExportRow {
                sheet_id: s.sheet_id.clone(),
                schema_version_label: s.schema_version_label.clone(),
                schema_freshness: s.schema_freshness.as_str().to_owned(),
                retention_class: s.retention_class.as_str().to_owned(),
                selected_destination_class: s.selected_destination_class.as_str().to_owned(),
                consent_status: s.consent_status.as_str().to_owned(),
                presentation: s.presentation.as_str().to_owned(),
                downgrade_reasons: s
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                local_save_first_class: s.local_save_first_class,
                blocked_before_send: s.blocked_before_send,
                included_total: s.included_total(),
                excluded_total: s.excluded_total(),
                policy_locked_total: s.policy_locked_total(),
                explain_entrypoint_ref: s.explain_entrypoint_ref.clone(),
                cli_object_ref: s.cli_object_ref.clone(),
                source_of_truth_ref: s.source_of_truth_ref.clone(),
                consent_receipt_ref: s.consent_receipt_ref.clone(),
                review_ready: s.is_review_ready(),
                summary: format!(
                    "{}: {} included / {} excluded / {} policy-locked, destination {}, presentation {}",
                    s.sheet_id,
                    s.included_total(),
                    s.excluded_total(),
                    s.policy_locked_total(),
                    s.selected_destination_class.as_str(),
                    s.presentation.as_str()
                ),
            })
            .collect();
        M5SupportBundleConsentExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_sheets_gate_consistent: self.all_sheets_gate_consistent(),
            all_local_save_first_class: self.all_local_save_first_class(),
            review_ready_count: self.review_ready_sheets().count(),
            narrowed_count: self.narrowed_sheets().count(),
            send_blocked_count: self.send_blocked_sheets().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact consent registry.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SupportBundleConsentSupportExport {
        M5SupportBundleConsentSupportExport {
            record_kind: M5_SUPPORT_BUNDLE_CONSENT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_material_excluded: true,
            registry: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5SupportBundleConsentViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        for sheet in &self.sheets {
            if !seen_ids.insert(sheet.sheet_id.clone()) {
                violations.push(M5SupportBundleConsentViolation::DuplicateSheet {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            self.validate_sheet(sheet, &mut violations);
        }

        for surface in ConsentConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5SupportBundleConsentViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5SupportBundleConsentViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SupportBundleConsentViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SupportBundleConsentViolation>) {
        if self.schema_version != M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_VERSION {
            violations.push(M5SupportBundleConsentViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_SUPPORT_BUNDLE_CONSENT_RECORD_KIND {
            violations.push(M5SupportBundleConsentViolation::UnsupportedRecordKind {
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
                violations.push(M5SupportBundleConsentViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "data_classes",
                self.data_classes == ConsentDataClass::ALL.to_vec(),
            ),
            (
                "destination_classes",
                self.destination_classes == ConsentDestinationClass::ALL.to_vec(),
            ),
            (
                "path_prominences",
                self.path_prominences == PathProminence::ALL.to_vec(),
            ),
            (
                "retention_classes",
                self.retention_classes == RetentionClass::ALL.to_vec(),
            ),
            (
                "schema_freshnesses",
                self.schema_freshnesses == SchemaFreshness::ALL.to_vec(),
            ),
            (
                "consent_statuses",
                self.consent_statuses == ConsentStatus::ALL.to_vec(),
            ),
            (
                "presentations",
                self.presentations == ConsentPresentation::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == ConsentDowngradeReason::ALL.to_vec(),
            ),
            (
                "redaction_modes",
                self.redaction_modes == RedactionMode::ALL.to_vec(),
            ),
            (
                "default_inclusions",
                self.default_inclusions == DefaultInclusion::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == ConsentConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5SupportBundleConsentViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_sheet(
        &self,
        sheet: &SupportBundleConsentSheet,
        violations: &mut Vec<M5SupportBundleConsentViolation>,
    ) {
        for (field, value) in [
            ("sheet_id", &sheet.sheet_id),
            ("title", &sheet.title),
            ("schema_version_label", &sheet.schema_version_label),
            (
                "current_schema_version_label",
                &sheet.current_schema_version_label,
            ),
            ("retention_note", &sheet.retention_note),
            ("source_of_truth_ref", &sheet.source_of_truth_ref),
            ("explain_entrypoint_ref", &sheet.explain_entrypoint_ref),
            ("cli_object_ref", &sheet.cli_object_ref),
            ("conformance_ref", &sheet.conformance_ref),
            ("evidence_ref", &sheet.evidence_ref),
            ("consent_receipt_ref", &sheet.consent_receipt_ref),
            ("note", &sheet.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SupportBundleConsentViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every sheet must carry its one-step "Review what this export includes" entry and its
        // CLI / headless equivalent, so consent is answerable from the Support Center and the CLI.
        if !sheet.has_one_step_explainability() {
            violations.push(
                M5SupportBundleConsentViolation::MissingOneStepExplainability {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }

        // No raw secret bodies, clipboard history, or raw payloads may be carried, ever.
        if !sheet.raw_material_excluded {
            violations.push(M5SupportBundleConsentViolation::RawMaterialNotExcluded {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        // A stale schema must name the current version it lags; a current one must match it.
        match sheet.schema_freshness {
            SchemaFreshness::Current => {
                if sheet.schema_version_label != sheet.current_schema_version_label {
                    violations.push(M5SupportBundleConsentViolation::SchemaFreshnessMismatch {
                        sheet_id: sheet.sheet_id.clone(),
                    });
                }
            }
            SchemaFreshness::Stale => {
                if sheet.schema_version_label == sheet.current_schema_version_label {
                    violations.push(M5SupportBundleConsentViolation::SchemaFreshnessMismatch {
                        sheet_id: sheet.sheet_id.clone(),
                    });
                }
            }
        }

        self.validate_class_rows(sheet, violations);
        self.validate_destinations(sheet, violations);
        self.validate_local_save_parity(sheet, violations);
        self.validate_gate(sheet, violations);
    }

    fn validate_class_rows(
        &self,
        sheet: &SupportBundleConsentSheet,
        violations: &mut Vec<M5SupportBundleConsentViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for row in &sheet.class_rows {
            if !seen.insert(row.data_class) {
                violations.push(M5SupportBundleConsentViolation::DuplicateDataClass {
                    sheet_id: sheet.sheet_id.clone(),
                    data_class: row.data_class.as_str(),
                });
            }
            if !row.is_well_formed() {
                violations.push(M5SupportBundleConsentViolation::ClassRowIncomplete {
                    sheet_id: sheet.sheet_id.clone(),
                    data_class: row.data_class.as_str(),
                });
            }

            // Secret-bearing classes stay excluded by default and never present an off-machine-exportable
            // toggle — they are kept out by default and usually non-exportable.
            if row.data_class.is_secret_bearing() {
                if !row.default_inclusion.is_excluded_by_default() {
                    violations.push(
                        M5SupportBundleConsentViolation::SecretBearingIncludedByDefault {
                            sheet_id: sheet.sheet_id.clone(),
                        },
                    );
                }
                if row
                    .redaction_toggle
                    .allowed_modes
                    .iter()
                    .any(|m| m.is_exportable_off_machine())
                {
                    violations.push(
                        M5SupportBundleConsentViolation::SecretBearingExportableToggle {
                            sheet_id: sheet.sheet_id.clone(),
                        },
                    );
                }
            }
        }

        // Every data class must be present exactly once so the included / excluded / policy-locked
        // counts are complete by class.
        for data_class in ConsentDataClass::ALL {
            if !seen.contains(&data_class) {
                violations.push(M5SupportBundleConsentViolation::MissingDataClass {
                    sheet_id: sheet.sheet_id.clone(),
                    data_class: data_class.as_str(),
                });
            }
        }
    }

    fn validate_destinations(
        &self,
        sheet: &SupportBundleConsentSheet,
        violations: &mut Vec<M5SupportBundleConsentViolation>,
    ) {
        for dest in &sheet.destinations {
            if !dest.is_well_formed() {
                violations.push(M5SupportBundleConsentViolation::DestinationIncomplete {
                    sheet_id: sheet.sheet_id.clone(),
                    destination_class: dest.destination_class.as_str(),
                });
            }
            if !dest.leaves_machine_consistent() {
                violations.push(
                    M5SupportBundleConsentViolation::DestinationLeavesMachineMismatch {
                        sheet_id: sheet.sheet_id.clone(),
                        destination_class: dest.destination_class.as_str(),
                    },
                );
            }
        }

        // Exactly one destination is selected, and the sheet's selected-destination class must match it.
        let selected: Vec<&ConsentDestination> =
            sheet.destinations.iter().filter(|d| d.selected).collect();
        match selected.as_slice() {
            [] => violations.push(M5SupportBundleConsentViolation::NoSelectedDestination {
                sheet_id: sheet.sheet_id.clone(),
            }),
            [one] => {
                if one.destination_class != sheet.selected_destination_class {
                    violations.push(
                        M5SupportBundleConsentViolation::SelectedDestinationMismatch {
                            sheet_id: sheet.sheet_id.clone(),
                        },
                    );
                }
                if !one.enabled {
                    violations.push(
                        M5SupportBundleConsentViolation::SelectedDestinationDisabled {
                            sheet_id: sheet.sheet_id.clone(),
                        },
                    );
                }
            }
            _ => violations.push(
                M5SupportBundleConsentViolation::MultipleSelectedDestinations {
                    sheet_id: sheet.sheet_id.clone(),
                },
            ),
        }
    }

    fn validate_local_save_parity(
        &self,
        sheet: &SupportBundleConsentSheet,
        violations: &mut Vec<M5SupportBundleConsentViolation>,
    ) {
        // A local-save path is always offered and enabled, so no-export / local review is never hidden.
        if sheet.local_save_destinations().next().is_none() {
            violations.push(M5SupportBundleConsentViolation::NoLocalSaveDestination {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        // The local-save path is at least as prominent as every send path; an upload-first sheet that
        // out-shouts local-save fails the gate.
        if !sheet.local_save_is_first_class() {
            violations.push(M5SupportBundleConsentViolation::LocalSaveNotFirstClass {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }

    fn validate_gate(
        &self,
        sheet: &SupportBundleConsentSheet,
        violations: &mut Vec<M5SupportBundleConsentViolation>,
    ) {
        // The recorded consent status must equal the status recomputed from the counts, toggles, and
        // destination, so a policy lock, override, or unsafe send can never be misclassified.
        let computed_status = sheet.computed_status();
        if sheet.consent_status != computed_status {
            violations.push(M5SupportBundleConsentViolation::ConsentStatusMismatch {
                sheet_id: sheet.sheet_id.clone(),
                declared: sheet.consent_status.as_str(),
                computed: computed_status.as_str(),
            });
        }

        // The published presentation must equal the gate's recomputed decision, so a narrowed or
        // blocked export can never read as a clean "ready to export" sheet.
        let effective = sheet.effective_presentation();
        if sheet.presentation != effective {
            violations.push(M5SupportBundleConsentViolation::OverstatedPresentation {
                sheet_id: sheet.sheet_id.clone(),
                published: sheet.presentation.as_str(),
                computed: effective.as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &sheet.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5SupportBundleConsentViolation::DuplicateDowngradeReason {
                    sheet_id: sheet.sheet_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }
        if sheet.downgrade_reasons != sheet.computed_downgrade_reasons() {
            violations.push(M5SupportBundleConsentViolation::DowngradeReasonsMismatch {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        // The local-save-first-class attestation must equal the recomputed parity.
        if sheet.local_save_first_class != sheet.local_save_is_first_class() {
            violations.push(
                M5SupportBundleConsentViolation::LocalSaveAttestationMismatch {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }

        // A send-blocked sheet must warn before any packet leaves; a non-blocked one must not claim it.
        if sheet.blocked_before_send != effective.warns_before_send() {
            violations.push(M5SupportBundleConsentViolation::BlockedBeforeSendMismatch {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        // A narrowed or blocked sheet always carries a caveat naming why it is not cleanly reviewable.
        if effective.requires_attention() && sheet.caveats.is_empty() {
            violations.push(M5SupportBundleConsentViolation::EmptyField {
                id: sheet.sheet_id.clone(),
                field_name: "caveats",
            });
        }

        // A policy-locked or send-blocked sheet always names the blockers the user must reconcile.
        if computed_status.requires_blockers() && sheet.blockers.is_empty() {
            violations.push(M5SupportBundleConsentViolation::EmptyField {
                id: sheet.sheet_id.clone(),
                field_name: "blockers",
            });
        }

        // Secret-bearing content can never reach a send destination on a sheet that is not blocked.
        if let Some(dest) = sheet.selected_destination() {
            if dest.leaves_machine
                && effective != ConsentPresentation::SendBlocked
                && sheet
                    .class_rows
                    .iter()
                    .any(|r| r.data_class.is_secret_bearing() && r.is_included())
            {
                violations.push(M5SupportBundleConsentViolation::SecretBearingExported {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
        }

        // A review-ready sheet must be genuinely whole: a clean status, a current schema, nothing
        // flagging it, and local-save first-class.
        if effective == ConsentPresentation::ReviewReady
            && (computed_status != ConsentStatus::ReviewReady
                || sheet.schema_freshness.is_stale()
                || !sheet.downgrade_reasons.is_empty()
                || !sheet.caveats.is_empty()
                || !sheet.blockers.is_empty()
                || sheet.blocked_before_send
                || !sheet.local_save_first_class
                || sheet.policy_locked_total() > 0
                || sheet.has_redaction_override())
        {
            violations.push(M5SupportBundleConsentViolation::ReviewReadySheetNotWhole {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }
}

/// A validation violation for the support-bundle-consent registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SupportBundleConsentViolation {
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
        /// Sheet or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A sheet id appears more than once.
    DuplicateSheet {
        /// Duplicate sheet id.
        sheet_id: String,
    },
    /// A sheet is missing its one-step explain entry or CLI-equivalent object id.
    MissingOneStepExplainability {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet does not attest that raw secret, clipboard, or payload material is excluded.
    RawMaterialNotExcluded {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's schema-freshness state disagrees with its visible-versus-current version labels.
    SchemaFreshnessMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet is missing a data-class row.
    MissingDataClass {
        /// Sheet id.
        sheet_id: String,
        /// Data-class token.
        data_class: &'static str,
    },
    /// A sheet lists a data-class row more than once.
    DuplicateDataClass {
        /// Sheet id.
        sheet_id: String,
        /// Data-class token.
        data_class: &'static str,
    },
    /// A class row is missing its note or carries a malformed toggle.
    ClassRowIncomplete {
        /// Sheet id.
        sheet_id: String,
        /// Data-class token.
        data_class: &'static str,
    },
    /// A secret-bearing class is included by default rather than excluded.
    SecretBearingIncludedByDefault {
        /// Sheet id.
        sheet_id: String,
    },
    /// A secret-bearing class offers an off-machine-exportable redaction toggle.
    SecretBearingExportableToggle {
        /// Sheet id.
        sheet_id: String,
    },
    /// A destination is missing its label or ref.
    DestinationIncomplete {
        /// Sheet id.
        sheet_id: String,
        /// Destination-class token.
        destination_class: &'static str,
    },
    /// A destination's `leaves_machine` flag disagrees with its class.
    DestinationLeavesMachineMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Destination-class token.
        destination_class: &'static str,
    },
    /// A sheet names no selected destination.
    NoSelectedDestination {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet names more than one selected destination.
    MultipleSelectedDestinations {
        /// Sheet id.
        sheet_id: String,
    },
    /// The selected-destination class disagrees with the selected destination.
    SelectedDestinationMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// The selected destination is not enabled.
    SelectedDestinationDisabled {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet offers no enabled local-save path.
    NoLocalSaveDestination {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's local-save path is less prominent than a send path.
    LocalSaveNotFirstClass {
        /// Sheet id.
        sheet_id: String,
    },
    /// The recorded consent status disagrees with the recomputed status.
    ConsentStatusMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Declared status token.
        declared: &'static str,
        /// Computed status token.
        computed: &'static str,
    },
    /// A sheet publishes a presentation cleaner than the gate computes.
    OverstatedPresentation {
        /// Sheet id.
        sheet_id: String,
        /// Published presentation token.
        published: &'static str,
        /// Computed effective presentation token.
        computed: &'static str,
    },
    /// A sheet lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Sheet id.
        sheet_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A sheet's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's local-save-first-class attestation disagrees with the recomputed parity.
    LocalSaveAttestationMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's blocked-before-send flag disagrees with the gate.
    BlockedBeforeSendMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A secret-bearing class would reach a send destination on a sheet that is not blocked.
    SecretBearingExported {
        /// Sheet id.
        sheet_id: String,
    },
    /// A review-ready sheet flags a state or carries a reason.
    ReviewReadySheetNotWhole {
        /// Sheet id.
        sheet_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints registry truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the sheets.
    SummaryMismatch,
}

impl fmt::Display for M5SupportBundleConsentViolation {
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
            Self::DuplicateSheet { sheet_id } => write!(f, "duplicate sheet id {sheet_id}"),
            Self::MissingOneStepExplainability { sheet_id } => write!(
                f,
                "sheet {sheet_id} is missing its one-step explain entry or CLI-equivalent object id"
            ),
            Self::RawMaterialNotExcluded { sheet_id } => write!(
                f,
                "sheet {sheet_id} does not attest raw secret/clipboard/payload material is excluded"
            ),
            Self::SchemaFreshnessMismatch { sheet_id } => write!(
                f,
                "sheet {sheet_id} schema-freshness state disagrees with its version labels"
            ),
            Self::MissingDataClass {
                sheet_id,
                data_class,
            } => write!(f, "sheet {sheet_id} is missing data class {data_class}"),
            Self::DuplicateDataClass {
                sheet_id,
                data_class,
            } => write!(f, "sheet {sheet_id} lists data class {data_class} more than once"),
            Self::ClassRowIncomplete {
                sheet_id,
                data_class,
            } => write!(
                f,
                "sheet {sheet_id} class {data_class} is missing its note or carries a malformed toggle"
            ),
            Self::SecretBearingIncludedByDefault { sheet_id } => write!(
                f,
                "sheet {sheet_id} includes a secret-bearing class by default"
            ),
            Self::SecretBearingExportableToggle { sheet_id } => write!(
                f,
                "sheet {sheet_id} offers an off-machine-exportable toggle for a secret-bearing class"
            ),
            Self::DestinationIncomplete {
                sheet_id,
                destination_class,
            } => write!(
                f,
                "sheet {sheet_id} destination {destination_class} is missing its label or ref"
            ),
            Self::DestinationLeavesMachineMismatch {
                sheet_id,
                destination_class,
            } => write!(
                f,
                "sheet {sheet_id} destination {destination_class} leaves-machine flag disagrees with its class"
            ),
            Self::NoSelectedDestination { sheet_id } => {
                write!(f, "sheet {sheet_id} names no selected destination")
            }
            Self::MultipleSelectedDestinations { sheet_id } => {
                write!(f, "sheet {sheet_id} names more than one selected destination")
            }
            Self::SelectedDestinationMismatch { sheet_id } => write!(
                f,
                "sheet {sheet_id} selected-destination class disagrees with the selected destination"
            ),
            Self::SelectedDestinationDisabled { sheet_id } => {
                write!(f, "sheet {sheet_id} selected destination is not enabled")
            }
            Self::NoLocalSaveDestination { sheet_id } => {
                write!(f, "sheet {sheet_id} offers no enabled local-save path")
            }
            Self::LocalSaveNotFirstClass { sheet_id } => write!(
                f,
                "sheet {sheet_id} local-save path is less prominent than a send path"
            ),
            Self::ConsentStatusMismatch {
                sheet_id,
                declared,
                computed,
            } => write!(
                f,
                "sheet {sheet_id} records consent status {declared} but the gate computes {computed}"
            ),
            Self::OverstatedPresentation {
                sheet_id,
                published,
                computed,
            } => write!(
                f,
                "sheet {sheet_id} publishes presentation {published} but the gate computes {computed}"
            ),
            Self::DuplicateDowngradeReason { sheet_id, reason } => {
                write!(f, "sheet {sheet_id} repeats downgrade reason {reason}")
            }
            Self::DowngradeReasonsMismatch { sheet_id } => {
                write!(f, "sheet {sheet_id} downgrade reasons disagree with the gate")
            }
            Self::LocalSaveAttestationMismatch { sheet_id } => write!(
                f,
                "sheet {sheet_id} local-save-first-class attestation disagrees with the recomputed parity"
            ),
            Self::BlockedBeforeSendMismatch { sheet_id } => write!(
                f,
                "sheet {sheet_id} blocked-before-send flag disagrees with the gate"
            ),
            Self::SecretBearingExported { sheet_id } => write!(
                f,
                "sheet {sheet_id} would send a secret-bearing class without blocking"
            ),
            Self::ReviewReadySheetNotWhole { sheet_id } => write!(
                f,
                "sheet {sheet_id} presents as review-ready but flags a state or carries a reason"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve registry truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the sheets"),
        }
    }
}

impl Error for M5SupportBundleConsentViolation {}

/// Stable record-kind tag for [`M5SupportBundleConsentSupportExport`].
pub const M5_SUPPORT_BUNDLE_CONSENT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_support_bundle_consent_support_export";

/// Support-export wrapper preserving the registry verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportBundleConsentSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw secret, clipboard, or payload material is excluded.
    pub raw_material_excluded: bool,
    /// Exact registry preserved by the export.
    pub registry: M5SupportBundleConsent,
}

impl M5SupportBundleConsentSupportExport {
    /// Whether the export preserves the same packet id and a clean registry.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_SUPPORT_BUNDLE_CONSENT_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_VERSION
            && self.packet_id_ref == self.registry.packet_id
            && self.raw_material_excluded
            && self.registry.validate().is_empty()
    }
}

/// Loads the embedded support-bundle-consent registry packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SupportBundleConsent`].
pub fn current_m5_support_bundle_consent() -> Result<M5SupportBundleConsent, serde_json::Error> {
    serde_json::from_str(M5_SUPPORT_BUNDLE_CONSENT_JSON)
}

#[cfg(test)]
mod tests;
