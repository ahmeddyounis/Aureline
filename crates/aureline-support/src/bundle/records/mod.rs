//! Records-governance, hold-awareness, and chain-of-custody projection
//! for support-bundle artifacts.
//!
//! This module mints the typed [`RecordsGovernancePacket`] that travels with
//! a support-bundle preview row so reviewers see, for every exported
//! artifact, the same governance truth the record-class registry pins:
//!
//! - which [`ArtifactClass`] the row falls into (local-only, managed-copy,
//!   held, queued-for-delete, or export-only);
//! - the current [`HoldState`] plus any [`HoldClass`] that gates the row;
//! - the [`RetentionOwnerClass`] and the local/managed owner refs that
//!   inherit from the registry row;
//! - a [`ChainOfCustodyEvent`] timeline that survives export and later
//!   escalation;
//! - and a [`DestructionCaveatClass`] that records why a deletion is
//!   delayed, partial, or impossible rather than implying success.
//!
//! The packet mirrors the boundary schema at
//! [`/schemas/support/record_class.schema.json`] and the registry rows
//! parsed by [`aureline_records`]. It is metadata-only: it cites refs,
//! ids, counts, and closed-vocabulary tokens, never raw payloads,
//! credentials, or hold justification bytes.
//!
//! Fixtures under `/fixtures/support/records_governance/` exercise every
//! [`ArtifactClass`] so support, product-boundary, and CLI/headless
//! review surfaces share one record set instead of reinventing a local
//! "deleted/held/exported" string.

use std::collections::BTreeSet;

use aureline_records::{
    current_registry, HoldEligibility, RecordClassId, RecordClassRow, RecordClassScope,
    RecordRegistryError,
};
use serde::{Deserialize, Serialize};

use super::manifest::SizeEstimate;
use super::preview::{PreviewItemSeed, SupportBundlePreviewBuilder};
use super::vocabulary::{ActionabilityImpactClass, DiagnosticDataClass, HighRiskContentClass};

/// Stable record-kind tag carried on every governance packet record.
pub const RECORDS_GOVERNANCE_PACKET_RECORD_KIND: &str = "records_governance_packet_record";

/// Frozen schema version for the records-governance packet record.
pub const RECORDS_GOVERNANCE_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative boundary schema path.
pub const RECORDS_GOVERNANCE_PACKET_SCHEMA_REF: &str = "schemas/support/record_class.schema.json";

/// Reviewer doc ref quoted alongside every emitted packet.
pub const RECORDS_GOVERNANCE_PACKET_DOC_REF: &str = "docs/support/m3/records_governance_beta.md";

/// Required redaction class for every governance packet.
pub const RECORDS_GOVERNANCE_PACKET_REDACTION_CLASS: &str = "metadata_safe_default";

/// Support-pack item id used when a governance packet is queued onto a
/// support-bundle preview as its own row.
pub const SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET: &str = "support.item.records_governance_packet";

/// Closed artifact-class vocabulary for one governance row.
///
/// Mirrors the acceptance row in the spec: every queued artifact resolves
/// to exactly one of these classes so support, admin, and CLI surfaces
/// share one taxonomy rather than re-deriving "is it gone" from prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    /// The artifact lives only on the local device and was never
    /// mirrored, exported, or handed off.
    LocalOnly,
    /// A managed-service copy exists alongside (or instead of) the local
    /// copy; the local row may be a cache, a preview, or the
    /// authoritative source depending on the registry row.
    ManagedCopy,
    /// At least one hold is active; the destructive lifecycle for this
    /// row is blocked until the hold clears.
    Held,
    /// A delete request has been accepted but has not completed; the
    /// row carries delete-state truth instead of pretending it is gone.
    QueuedForDelete,
    /// The artifact only ever existed as a generated export packet
    /// (e.g. usage export, offboarding exit packet, destruction
    /// receipt). There is no durable in-product row to keep.
    ExportOnly,
}

impl ArtifactClass {
    /// Stable snake-case token used in manifests, exports, and tests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCopy => "managed_copy",
            Self::Held => "held",
            Self::QueuedForDelete => "queued_for_delete",
            Self::ExportOnly => "export_only",
        }
    }

    /// Reviewer-facing label shown next to the artifact-class chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::ManagedCopy => "Managed copy retained",
            Self::Held => "Held — destructive actions blocked",
            Self::QueuedForDelete => "Queued for delete — not yet gone",
            Self::ExportOnly => "Export packet — no durable in-product row",
        }
    }
}

/// Closed hold-class vocabulary. Mirrors the hold_classes set on the
/// alpha record-class registry rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldClass {
    AdministrativeLegal,
    SupportInvestigation,
    RetentionMinimum,
    ExportPending,
}

impl HoldClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdministrativeLegal => "administrative_legal",
            Self::SupportInvestigation => "support_investigation",
            Self::RetentionMinimum => "retention_minimum",
            Self::ExportPending => "export_pending",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "administrative_legal" => Some(Self::AdministrativeLegal),
            "support_investigation" => Some(Self::SupportInvestigation),
            "retention_minimum" => Some(Self::RetentionMinimum),
            "export_pending" => Some(Self::ExportPending),
            _ => None,
        }
    }
}

/// Closed hold-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldState {
    /// No hold applies to this row.
    None,
    /// One or more holds gate the destructive lifecycle for this row.
    OnHold,
    /// A hold was lifted but downstream destructive steps have not yet
    /// completed; the row is still distinct from a clean "no hold" state.
    ReleasePending,
}

impl HoldState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OnHold => "on_hold",
            Self::ReleasePending => "release_pending",
        }
    }
}

/// Closed retention-owner vocabulary. Inherits from the registry row's
/// `retention.local_owner_ref` and `retention.managed_owner_ref`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionOwnerClass {
    /// The local user owns retention. No managed mirror exists.
    LocalUser,
    /// An operator/admin owns retention on the managed side.
    OperatorAdmin,
    /// A support/export pipeline owns retention until packet expiry.
    SupportExport,
    /// A governance-packets surface owns retention (e.g. destruction
    /// receipts, signed governance evidence).
    GovernancePackets,
    /// Local and managed copies have distinct owners.
    Mixed,
}

impl RetentionOwnerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::OperatorAdmin => "operator_admin",
            Self::SupportExport => "support_export",
            Self::GovernancePackets => "governance_packets",
            Self::Mixed => "mixed",
        }
    }
}

/// Closed destruction-caveat vocabulary describing why a deletion is
/// delayed, partial, or impossible. `None` means a deletion (if it ran)
/// completed without retained subsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestructionCaveatClass {
    /// No caveat applies. A delete completed cleanly, or none has been
    /// requested.
    None,
    /// A destruction receipt is retained as append-only metadata even
    /// after the payload is gone.
    ReceiptRetained,
    /// A managed retained subset remains after a managed-side delete.
    RetainedSubsetRemains,
    /// One or more holds block terminal completion for this row.
    HoldBlocksCompletion,
    /// A legal hold prevents any destructive action.
    LegalHoldPrevents,
    /// A retention-minimum floor keeps at least one managed or audit
    /// subset.
    RetentionMinimumApplies,
    /// A locally-exported copy remains under user/device control even
    /// after a managed-side delete.
    ExportedLocalCopyRemains,
    /// The artifact lives outside the platform's destructive scope
    /// (e.g. third-party sinks).
    OutsidePlatformScope,
    /// A provider backlog (sync, mirror, hosted intake) delays terminal
    /// completion.
    ProviderBacklog,
}

impl DestructionCaveatClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ReceiptRetained => "receipt_retained",
            Self::RetainedSubsetRemains => "retained_subset_remains",
            Self::HoldBlocksCompletion => "hold_blocks_completion",
            Self::LegalHoldPrevents => "legal_hold_prevents",
            Self::RetentionMinimumApplies => "retention_minimum_applies",
            Self::ExportedLocalCopyRemains => "exported_local_copy_remains",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::ProviderBacklog => "provider_backlog",
        }
    }
}

/// Closed custody-actor vocabulary for chain-of-custody events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyActorClass {
    LocalUser,
    AdminInitiated,
    SupportAgent,
    AutomatedRetentionJob,
    ExportPipeline,
    OffboardingJob,
    GovernancePackets,
}

impl CustodyActorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::AdminInitiated => "admin_initiated",
            Self::SupportAgent => "support_agent",
            Self::AutomatedRetentionJob => "automated_retention_job",
            Self::ExportPipeline => "export_pipeline",
            Self::OffboardingJob => "offboarding_job",
            Self::GovernancePackets => "governance_packets",
        }
    }
}

/// Closed custody-action vocabulary for chain-of-custody events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyActionClass {
    Created,
    PackagedForExport,
    ExportedLocally,
    MirroredToManaged,
    PlacedOnHold,
    HoldReleased,
    DeleteRequested,
    DeleteCompleted,
    ReceiptIssued,
    AccessHandedOff,
    ImportedFromHandoff,
}

impl CustodyActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::PackagedForExport => "packaged_for_export",
            Self::ExportedLocally => "exported_locally",
            Self::MirroredToManaged => "mirrored_to_managed",
            Self::PlacedOnHold => "placed_on_hold",
            Self::HoldReleased => "hold_released",
            Self::DeleteRequested => "delete_requested",
            Self::DeleteCompleted => "delete_completed",
            Self::ReceiptIssued => "receipt_issued",
            Self::AccessHandedOff => "access_handed_off",
            Self::ImportedFromHandoff => "imported_from_handoff",
        }
    }
}

/// Closed custody-location vocabulary. Mirrors the
/// remaining-location-class taxonomy used by retention/delete records,
/// narrowed to the subset support exports must distinguish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyLocationClass {
    LocalDeviceOnly,
    LocalExportCopy,
    ManagedArchiveActive,
    ManagedArchiveHeld,
    ManagedArchivePolicyRetained,
    ManagedArchiveReplicatedPendingPurge,
    DestructionReceiptOnly,
    ImportSourceOrigin,
    OutsidePlatformScope,
    NoRemainingLocation,
}

impl CustodyLocationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::LocalExportCopy => "local_export_copy",
            Self::ManagedArchiveActive => "managed_archive_active",
            Self::ManagedArchiveHeld => "managed_archive_held",
            Self::ManagedArchivePolicyRetained => "managed_archive_policy_retained",
            Self::ManagedArchiveReplicatedPendingPurge => {
                "managed_archive_replicated_pending_purge"
            }
            Self::DestructionReceiptOnly => "destruction_receipt_only",
            Self::ImportSourceOrigin => "import_source_origin",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::NoRemainingLocation => "no_remaining_location",
        }
    }
}

/// One chain-of-custody event for an artifact. Survives export verbatim
/// so a later escalation reads the same history the local preview
/// already showed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainOfCustodyEvent {
    /// Stable event id; unique within a single packet.
    pub event_id: String,
    /// Sequence number within the chain (0-based; strictly increasing).
    pub sequence: u32,
    /// Who performed the action.
    pub actor_class: CustodyActorClass,
    /// Opaque actor ref (operator id, automation id, or
    /// `local_user_anonymous` for the local user). Never a raw email,
    /// token, or username body.
    pub actor_ref: String,
    /// What happened.
    pub action_class: CustodyActionClass,
    /// RFC 3339 UTC timestamp recorded by the event source. Reviewers
    /// reading exports later use this to reconstruct the timeline.
    pub occurred_at: String,
    /// Where the artifact lived (or moved to) after the action.
    pub location_class: CustodyLocationClass,
    /// Opaque evidence ref (audit row id, packet ref, journal id) that
    /// proves the event. May be empty when the event source does not
    /// emit a separate evidence row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Redaction-safe one-line note. No raw payloads or hold
    /// justification bytes.
    pub note: String,
}

/// Caller-supplied inputs the evaluator validates and turns into a
/// [`RecordsGovernancePacket`].
#[derive(Debug, Clone)]
pub struct RecordsGovernanceInputs {
    /// Stable packet id. Must start with `records_governance_packet:`.
    pub packet_id: String,
    /// Stable artifact id this packet describes.
    pub artifact_id: String,
    /// Reviewer-visible title for the artifact.
    pub title: String,
    /// Record class governing the artifact's lifecycle.
    pub record_class_id: RecordClassId,
    /// Artifact-class derived (or asserted) by the caller. The
    /// evaluator cross-checks the value against the active hold set and
    /// destruction caveat so the chip cannot lie.
    pub artifact_class: ArtifactClass,
    /// Current hold state.
    pub hold_state: HoldState,
    /// Active hold classes (deduplicated by the evaluator).
    pub hold_classes: Vec<HoldClass>,
    /// Retention-owner class.
    pub retention_owner_class: RetentionOwnerClass,
    /// Opaque local-owner ref copied verbatim into the packet.
    pub local_owner_ref: String,
    /// Opaque managed-owner ref copied verbatim into the packet.
    pub managed_owner_ref: String,
    /// Destruction-caveat class.
    pub destruction_caveat_class: DestructionCaveatClass,
    /// Reviewer-safe caveat note. Must be non-empty when
    /// `destruction_caveat_class` is not `None`.
    pub destruction_caveat_note: String,
    /// Whether a downloaded export copy remains under user/device
    /// control even after a managed-side delete.
    pub exported_copy_remains_local: bool,
    /// Chain-of-custody events ordered oldest-first. The evaluator
    /// validates monotonic `sequence` values and unique `event_id`s.
    pub chain_of_custody: Vec<ChainOfCustodyEvent>,
    /// Optional support-pack item id this packet is bound to when
    /// projected into a support-bundle preview row.
    pub support_pack_item_id: Option<String>,
}

/// Typed records-governance packet. Mirrors
/// `/schemas/support/record_class.schema.json#records_governance_packet_record`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordsGovernancePacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub artifact_id: String,
    pub title: String,
    pub record_class_id: RecordClassId,
    pub record_class_token: String,
    pub registry_row_id: String,
    pub registry_ref: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub class_scope: RecordClassScope,
    pub artifact_class: ArtifactClass,
    pub hold_state: HoldState,
    pub hold_classes: Vec<HoldClass>,
    pub retention_owner_class: RetentionOwnerClass,
    pub local_owner_ref: String,
    pub managed_owner_ref: String,
    pub destruction_caveat_class: DestructionCaveatClass,
    pub destruction_caveat_note: String,
    pub exported_copy_remains_local: bool,
    pub chain_of_custody: Vec<ChainOfCustodyEvent>,
    pub redaction_class: String,
    pub raw_content_exported: bool,
    /// Reviewer-visible summary sentence quoting the closed tokens.
    pub reviewer_summary: String,
}

impl RecordsGovernancePacket {
    /// True when at least one hold is active for this row.
    pub fn is_held(&self) -> bool {
        matches!(self.hold_state, HoldState::OnHold) && !self.hold_classes.is_empty()
    }

    /// True when the chain of custody records a destructive lifecycle
    /// step that has not yet completed.
    pub fn is_queued_for_delete(&self) -> bool {
        matches!(self.artifact_class, ArtifactClass::QueuedForDelete)
    }

    /// True when the destruction caveat is non-trivial. Drives the
    /// honesty marker on the support-bundle preview.
    pub fn has_destruction_caveat(&self) -> bool {
        !matches!(self.destruction_caveat_class, DestructionCaveatClass::None)
    }

    /// Count of chain-of-custody events recorded so far.
    pub fn custody_event_count(&self) -> usize {
        self.chain_of_custody.len()
    }
}

/// Errors raised by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordsGovernanceError {
    /// The packet id did not start with `records_governance_packet:`.
    PacketIdPrefixMissing { packet_id: String },
    /// The artifact id was empty.
    ArtifactIdEmpty,
    /// The reviewer-visible title was empty.
    TitleEmpty,
    /// The record class is not registered in the active record-class
    /// registry.
    RecordClassUnregistered { record_class_id: RecordClassId },
    /// The record-class registry could not be parsed.
    RecordRegistry(RecordRegistryError),
    /// The asserted artifact class disagrees with the hold state or
    /// destruction caveat.
    ArtifactClassInconsistent {
        asserted: ArtifactClass,
        expected: ArtifactClass,
        reason: &'static str,
    },
    /// A hold class was supplied that the registry row does not allow.
    HoldClassNotAllowed {
        record_class_id: RecordClassId,
        hold_class: HoldClass,
    },
    /// The hold state is `on_hold` but no hold classes were supplied.
    HoldStateMissingHoldClass,
    /// The hold state is `none` but hold classes were supplied.
    HoldStateNoneWithHoldClass,
    /// The record class is ineligible to be held under the registry.
    HoldNotEligible { record_class_id: RecordClassId },
    /// The destruction caveat is non-trivial but the note was empty.
    CaveatNoteEmpty {
        destruction_caveat_class: DestructionCaveatClass,
    },
    /// The chain-of-custody list was empty.
    ChainOfCustodyEmpty,
    /// A chain-of-custody event had a non-monotonic sequence number.
    ChainOfCustodySequenceNotMonotonic { event_id: String, sequence: u32 },
    /// A chain-of-custody event repeated an `event_id`.
    ChainOfCustodyDuplicateEventId { event_id: String },
    /// A chain-of-custody event was missing a required string.
    ChainOfCustodyEventFieldEmpty {
        event_id: String,
        field: &'static str,
    },
    /// The support-pack item id did not match the `support.item.` prefix.
    SupportPackItemIdPrefixMissing { support_pack_item_id: String },
}

impl std::fmt::Display for RecordsGovernanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketIdPrefixMissing { packet_id } => write!(
                f,
                "records_governance packet_id must start with 'records_governance_packet:' (got {packet_id})"
            ),
            Self::ArtifactIdEmpty => write!(f, "records_governance artifact_id must not be empty"),
            Self::TitleEmpty => write!(f, "records_governance title must not be empty"),
            Self::RecordClassUnregistered { record_class_id } => write!(
                f,
                "records_governance record_class_id {record_class_id} is not registered in the active record-class registry"
            ),
            Self::RecordRegistry(err) => {
                write!(f, "records_governance record-registry error: {err}")
            }
            Self::ArtifactClassInconsistent {
                asserted,
                expected,
                reason,
            } => write!(
                f,
                "records_governance artifact_class {} is inconsistent with derived class {}: {reason}",
                asserted.as_str(),
                expected.as_str()
            ),
            Self::HoldClassNotAllowed {
                record_class_id,
                hold_class,
            } => write!(
                f,
                "records_governance hold_class {} is not allowed for record class {record_class_id}",
                hold_class.as_str()
            ),
            Self::HoldStateMissingHoldClass => write!(
                f,
                "records_governance hold_state=on_hold requires at least one hold_class"
            ),
            Self::HoldStateNoneWithHoldClass => write!(
                f,
                "records_governance hold_state=none must not supply hold_classes"
            ),
            Self::HoldNotEligible { record_class_id } => write!(
                f,
                "records_governance record class {record_class_id} is not hold-eligible under the registry"
            ),
            Self::CaveatNoteEmpty {
                destruction_caveat_class,
            } => write!(
                f,
                "records_governance destruction_caveat_class {} requires a non-empty destruction_caveat_note",
                destruction_caveat_class.as_str()
            ),
            Self::ChainOfCustodyEmpty => {
                write!(f, "records_governance chain_of_custody must not be empty")
            }
            Self::ChainOfCustodySequenceNotMonotonic { event_id, sequence } => write!(
                f,
                "records_governance chain_of_custody event {event_id} has non-monotonic sequence {sequence}"
            ),
            Self::ChainOfCustodyDuplicateEventId { event_id } => write!(
                f,
                "records_governance chain_of_custody event_id {event_id} appears more than once"
            ),
            Self::ChainOfCustodyEventFieldEmpty { event_id, field } => write!(
                f,
                "records_governance chain_of_custody event {event_id} has empty {field}"
            ),
            Self::SupportPackItemIdPrefixMissing {
                support_pack_item_id,
            } => write!(
                f,
                "records_governance support_pack_item_id must start with 'support.item.' (got {support_pack_item_id})"
            ),
        }
    }
}

impl std::error::Error for RecordsGovernanceError {}

impl From<RecordRegistryError> for RecordsGovernanceError {
    fn from(err: RecordRegistryError) -> Self {
        Self::RecordRegistry(err)
    }
}

/// Evaluate caller-supplied governance inputs into a typed
/// [`RecordsGovernancePacket`]. Validates every field against the active
/// record-class registry so the emitted packet cannot drift from the
/// governed lifecycle.
pub fn evaluate_records_governance_packet(
    inputs: RecordsGovernanceInputs,
) -> Result<RecordsGovernancePacket, RecordsGovernanceError> {
    if !inputs.packet_id.starts_with("records_governance_packet:") {
        return Err(RecordsGovernanceError::PacketIdPrefixMissing {
            packet_id: inputs.packet_id.clone(),
        });
    }
    if inputs.artifact_id.trim().is_empty() {
        return Err(RecordsGovernanceError::ArtifactIdEmpty);
    }
    if inputs.title.trim().is_empty() {
        return Err(RecordsGovernanceError::TitleEmpty);
    }
    if let Some(support_pack_item_id) = inputs.support_pack_item_id.as_ref() {
        if !support_pack_item_id.starts_with("support.item.") {
            return Err(RecordsGovernanceError::SupportPackItemIdPrefixMissing {
                support_pack_item_id: support_pack_item_id.clone(),
            });
        }
    }

    let registry = current_registry()?;
    let row = registry
        .row(inputs.record_class_id)
        .ok_or(RecordsGovernanceError::RecordClassUnregistered {
            record_class_id: inputs.record_class_id,
        })?;

    // Hold-state shape validation. Eligibility is checked first so a
    // hold-ineligible class never reports a less-precise "hold class
    // not allowed" error when the underlying problem is that the row
    // cannot be held at all.
    match inputs.hold_state {
        HoldState::OnHold | HoldState::ReleasePending => {
            if !row.hold_semantics.eligible.as_bool() {
                return Err(RecordsGovernanceError::HoldNotEligible {
                    record_class_id: inputs.record_class_id,
                });
            }
        }
        HoldState::None => {
            if !inputs.hold_classes.is_empty() {
                return Err(RecordsGovernanceError::HoldStateNoneWithHoldClass);
            }
        }
    }

    let allowed_hold_classes = allowed_hold_classes_for_row(row);
    let mut hold_classes_set = BTreeSet::new();
    for hold_class in &inputs.hold_classes {
        if !allowed_hold_classes.contains(hold_class) {
            return Err(RecordsGovernanceError::HoldClassNotAllowed {
                record_class_id: inputs.record_class_id,
                hold_class: *hold_class,
            });
        }
        hold_classes_set.insert(*hold_class);
    }
    let hold_classes: Vec<HoldClass> = hold_classes_set.into_iter().collect();

    if matches!(inputs.hold_state, HoldState::OnHold) && hold_classes.is_empty() {
        return Err(RecordsGovernanceError::HoldStateMissingHoldClass);
    }

    // Destruction caveat note must be present whenever the class is
    // non-trivial.
    if !matches!(
        inputs.destruction_caveat_class,
        DestructionCaveatClass::None
    ) && inputs.destruction_caveat_note.trim().is_empty()
    {
        return Err(RecordsGovernanceError::CaveatNoteEmpty {
            destruction_caveat_class: inputs.destruction_caveat_class,
        });
    }

    // Chain-of-custody validation.
    if inputs.chain_of_custody.is_empty() {
        return Err(RecordsGovernanceError::ChainOfCustodyEmpty);
    }
    let mut seen_ids: BTreeSet<String> = BTreeSet::new();
    let mut last_sequence: Option<u32> = None;
    for event in &inputs.chain_of_custody {
        if event.event_id.trim().is_empty() {
            return Err(RecordsGovernanceError::ChainOfCustodyEventFieldEmpty {
                event_id: event.event_id.clone(),
                field: "event_id",
            });
        }
        if event.actor_ref.trim().is_empty() {
            return Err(RecordsGovernanceError::ChainOfCustodyEventFieldEmpty {
                event_id: event.event_id.clone(),
                field: "actor_ref",
            });
        }
        if event.occurred_at.trim().is_empty() {
            return Err(RecordsGovernanceError::ChainOfCustodyEventFieldEmpty {
                event_id: event.event_id.clone(),
                field: "occurred_at",
            });
        }
        if event.note.trim().is_empty() {
            return Err(RecordsGovernanceError::ChainOfCustodyEventFieldEmpty {
                event_id: event.event_id.clone(),
                field: "note",
            });
        }
        if !seen_ids.insert(event.event_id.clone()) {
            return Err(RecordsGovernanceError::ChainOfCustodyDuplicateEventId {
                event_id: event.event_id.clone(),
            });
        }
        if let Some(prev) = last_sequence {
            if event.sequence <= prev {
                return Err(RecordsGovernanceError::ChainOfCustodySequenceNotMonotonic {
                    event_id: event.event_id.clone(),
                    sequence: event.sequence,
                });
            }
        }
        last_sequence = Some(event.sequence);
    }

    // Cross-check artifact class with hold + custody truth.
    let derived_class = derive_artifact_class(
        inputs.hold_state,
        &hold_classes,
        &inputs.chain_of_custody,
        inputs.destruction_caveat_class,
        row,
    );
    if derived_class != inputs.artifact_class {
        return Err(RecordsGovernanceError::ArtifactClassInconsistent {
            asserted: inputs.artifact_class,
            expected: derived_class,
            reason: derived_class_reason(derived_class),
        });
    }

    let reviewer_summary = format!(
        "Artifact class {}; hold state {} ({} hold class{}); retention owner {}; destruction caveat {}; \
         {} custody event{}; exported_copy_remains_local={}.",
        inputs.artifact_class.as_str(),
        inputs.hold_state.as_str(),
        hold_classes.len(),
        if hold_classes.len() == 1 { "" } else { "es" },
        inputs.retention_owner_class.as_str(),
        inputs.destruction_caveat_class.as_str(),
        inputs.chain_of_custody.len(),
        if inputs.chain_of_custody.len() == 1 {
            ""
        } else {
            "s"
        },
        inputs.exported_copy_remains_local
    );

    Ok(RecordsGovernancePacket {
        schema_version: RECORDS_GOVERNANCE_PACKET_SCHEMA_VERSION,
        record_kind: RECORDS_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        packet_id: inputs.packet_id,
        artifact_id: inputs.artifact_id,
        title: inputs.title,
        record_class_id: inputs.record_class_id,
        record_class_token: inputs.record_class_id.as_str().to_owned(),
        registry_row_id: row.row_id.clone(),
        registry_ref: aureline_records::RECORD_CLASS_REGISTRY_ALPHA_PATH.to_owned(),
        schema_ref: RECORDS_GOVERNANCE_PACKET_SCHEMA_REF.to_owned(),
        doc_ref: RECORDS_GOVERNANCE_PACKET_DOC_REF.to_owned(),
        class_scope: row.class_scope,
        artifact_class: inputs.artifact_class,
        hold_state: inputs.hold_state,
        hold_classes,
        retention_owner_class: inputs.retention_owner_class,
        local_owner_ref: inputs.local_owner_ref,
        managed_owner_ref: inputs.managed_owner_ref,
        destruction_caveat_class: inputs.destruction_caveat_class,
        destruction_caveat_note: inputs.destruction_caveat_note,
        exported_copy_remains_local: inputs.exported_copy_remains_local,
        chain_of_custody: inputs.chain_of_custody,
        redaction_class: RECORDS_GOVERNANCE_PACKET_REDACTION_CLASS.to_owned(),
        raw_content_exported: false,
        reviewer_summary,
    })
}

fn allowed_hold_classes_for_row(row: &RecordClassRow) -> BTreeSet<HoldClass> {
    let mut set = BTreeSet::new();
    if matches!(row.hold_semantics.eligible, HoldEligibility::Ineligible) {
        return set;
    }
    for token in &row.hold_semantics.hold_classes {
        if let Some(class) = HoldClass::parse(token) {
            set.insert(class);
        }
    }
    set
}

fn derive_artifact_class(
    hold_state: HoldState,
    hold_classes: &[HoldClass],
    chain: &[ChainOfCustodyEvent],
    caveat: DestructionCaveatClass,
    row: &RecordClassRow,
) -> ArtifactClass {
    // Held trumps everything: the row's destructive lifecycle is blocked.
    if matches!(hold_state, HoldState::OnHold) && !hold_classes.is_empty() {
        return ArtifactClass::Held;
    }

    let has_delete_requested = chain
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::DeleteRequested));
    let has_delete_completed = chain
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::DeleteCompleted));

    if has_delete_requested && !has_delete_completed {
        return ArtifactClass::QueuedForDelete;
    }

    // Export-only classes: the class scope is export_packet or receipt
    // — there's no durable in-product row, only the generated packet.
    if matches!(
        row.class_scope,
        RecordClassScope::ExportPacket | RecordClassScope::Receipt
    ) {
        return ArtifactClass::ExportOnly;
    }

    // Managed-copy posture: a managed copy exists (or is the
    // authoritative source) and the destructive caveat names a managed
    // retained subset / receipt / replicated copy.
    let managed_evident = matches!(
        row.class_scope,
        RecordClassScope::ManagedCopy | RecordClassScope::SupportBundle
    ) || matches!(
        caveat,
        DestructionCaveatClass::RetainedSubsetRemains
            | DestructionCaveatClass::ReceiptRetained
            | DestructionCaveatClass::ProviderBacklog
    ) || chain
        .iter()
        .any(|event| matches!(event.action_class, CustodyActionClass::MirroredToManaged));

    if managed_evident {
        return ArtifactClass::ManagedCopy;
    }

    ArtifactClass::LocalOnly
}

fn derived_class_reason(derived: ArtifactClass) -> &'static str {
    match derived {
        ArtifactClass::Held => "an active hold gates destructive lifecycle steps",
        ArtifactClass::QueuedForDelete => {
            "a delete was requested but not completed in the custody chain"
        }
        ArtifactClass::ExportOnly => {
            "the registry class scope is export_packet or receipt with no durable in-product row"
        }
        ArtifactClass::ManagedCopy => {
            "the registry scope, destruction caveat, or custody chain indicates a managed copy"
        }
        ArtifactClass::LocalOnly => {
            "no managed copy, hold, or pending delete is recorded for this row"
        }
    }
}

/// Build the preview-row seed that carries a [`RecordsGovernancePacket`]
/// into a support-bundle preview as metadata-only governance evidence.
pub fn records_governance_preview_item_seed(packet: &RecordsGovernancePacket) -> PreviewItemSeed {
    let support_pack_item_id = SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET.to_owned();
    let manifest_path_ref = format!("preview_items[].{}", packet.record_kind);
    let bundle_member_path_ref = Some(format!(
        "manifest/records_governance/{}.json",
        safe_member_name(&packet.artifact_id)
    ));
    let mut source_refs: Vec<String> = vec![
        packet.schema_ref.clone(),
        packet.doc_ref.clone(),
        packet.registry_ref.clone(),
    ];
    if !packet.registry_row_id.is_empty() {
        source_refs.push(packet.registry_row_id.clone());
    }

    let bytes_estimate = serde_json::to_vec(packet)
        .map(|bytes| bytes.len() as u64)
        .ok();
    let display_label = bytes_estimate
        .map(|bytes| format!("{bytes} B"))
        .unwrap_or_else(|| "unknown until export".to_owned());

    let impact_class = if matches!(packet.artifact_class, ArtifactClass::Held)
        || packet.has_destruction_caveat()
    {
        ActionabilityImpactClass::High
    } else {
        ActionabilityImpactClass::Medium
    };

    let impact_summary = format!(
        "Records-governance row for {} ({}); hold state {}; destruction caveat {}.",
        packet.title,
        packet.record_class_token,
        packet.hold_state.as_str(),
        packet.destruction_caveat_class.as_str(),
    );

    let notes = format!(
        "Metadata-only records-governance packet. {} Quotes registry row {} and schema {}.",
        packet.reviewer_summary, packet.registry_row_id, packet.schema_ref,
    );

    PreviewItemSeed {
        support_pack_item_id,
        title: format!("Records governance — {}", packet.title),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".to_owned(),
        artifact_kind_class: "records_governance_packet".to_owned(),
        manifest_path_ref,
        bundle_member_path_ref,
        source_refs,
        size_estimate: SizeEstimate {
            estimated_bytes: bytes_estimate,
            confidence_class: if bytes_estimate.is_some() {
                "exact".to_owned()
            } else {
                "unknown".to_owned()
            },
            display_label,
            size_source_class: if bytes_estimate.is_some() {
                "precomputed_manifest".to_owned()
            } else {
                "unknown_until_export".to_owned()
            },
        },
        impact_class,
        impact_summary,
        notes,
    }
}

/// Queue a records-governance row on a support-bundle preview builder.
pub fn add_records_governance_preview_item<'a>(
    builder: &'a mut SupportBundlePreviewBuilder,
    packet: &RecordsGovernancePacket,
) -> &'a mut SupportBundlePreviewBuilder {
    builder.add_item(records_governance_preview_item_seed(packet))
}

fn safe_member_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_only_inputs() -> RecordsGovernanceInputs {
        RecordsGovernanceInputs {
            packet_id: "records_governance_packet:local_only:0001".into(),
            artifact_id: "artifact:workspace_state:0001".into(),
            title: "Local workspace state".into(),
            record_class_id: RecordClassId::DurableWorkspaceState,
            artifact_class: ArtifactClass::LocalOnly,
            hold_state: HoldState::None,
            hold_classes: Vec::new(),
            retention_owner_class: RetentionOwnerClass::LocalUser,
            local_owner_ref: "local_user".into(),
            managed_owner_ref: "local_user".into(),
            destruction_caveat_class: DestructionCaveatClass::None,
            destruction_caveat_note: String::new(),
            exported_copy_remains_local: false,
            chain_of_custody: vec![ChainOfCustodyEvent {
                event_id: "evt:0001:created".into(),
                sequence: 0,
                actor_class: CustodyActorClass::LocalUser,
                actor_ref: "local_user_anonymous".into(),
                action_class: CustodyActionClass::Created,
                occurred_at: "2026-05-15T08:00:00Z".into(),
                location_class: CustodyLocationClass::LocalDeviceOnly,
                evidence_ref: None,
                note: "Workspace state captured locally for support preview.".into(),
            }],
            support_pack_item_id: Some(SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET.into()),
        }
    }

    #[test]
    fn local_only_packet_evaluates() {
        let packet =
            evaluate_records_governance_packet(local_only_inputs()).expect("local-only packet");
        assert_eq!(packet.artifact_class, ArtifactClass::LocalOnly);
        assert_eq!(packet.hold_state, HoldState::None);
        assert!(!packet.has_destruction_caveat());
        assert_eq!(packet.custody_event_count(), 1);
        assert_eq!(packet.redaction_class, "metadata_safe_default");
        assert!(!packet.raw_content_exported);
    }

    #[test]
    fn rejects_packet_id_without_prefix() {
        let mut inputs = local_only_inputs();
        inputs.packet_id = "not_a_records_packet:0001".into();
        let err = evaluate_records_governance_packet(inputs).expect_err("prefix is enforced");
        assert!(matches!(
            err,
            RecordsGovernanceError::PacketIdPrefixMissing { .. }
        ));
    }

    #[test]
    fn rejects_inconsistent_artifact_class() {
        let mut inputs = local_only_inputs();
        inputs.artifact_class = ArtifactClass::Held;
        let err = evaluate_records_governance_packet(inputs)
            .expect_err("held without hold state is rejected");
        assert!(matches!(
            err,
            RecordsGovernanceError::ArtifactClassInconsistent { .. }
        ));
    }

    #[test]
    fn rejects_hold_class_not_allowed_by_registry() {
        let mut inputs = local_only_inputs();
        inputs.record_class_id = RecordClassId::SupportBundleArchive;
        inputs.artifact_class = ArtifactClass::Held;
        inputs.hold_state = HoldState::OnHold;
        inputs.hold_classes = vec![HoldClass::SupportInvestigation];
        let packet = evaluate_records_governance_packet(inputs).expect("registry-allowed hold");
        assert_eq!(packet.artifact_class, ArtifactClass::Held);

        let mut bad = local_only_inputs();
        bad.artifact_class = ArtifactClass::Held;
        bad.hold_state = HoldState::OnHold;
        bad.hold_classes = vec![HoldClass::SupportInvestigation];
        let err = evaluate_records_governance_packet(bad)
            .expect_err("durable_workspace_state is not hold-eligible");
        assert!(matches!(
            err,
            RecordsGovernanceError::HoldNotEligible { .. }
        ));
    }

    #[test]
    fn rejects_non_monotonic_chain_of_custody() {
        let mut inputs = local_only_inputs();
        inputs.chain_of_custody.push(ChainOfCustodyEvent {
            event_id: "evt:0002:created".into(),
            sequence: 0,
            actor_class: CustodyActorClass::LocalUser,
            actor_ref: "local_user_anonymous".into(),
            action_class: CustodyActionClass::PackagedForExport,
            occurred_at: "2026-05-15T08:05:00Z".into(),
            location_class: CustodyLocationClass::LocalExportCopy,
            evidence_ref: None,
            note: "Second event with stale sequence.".into(),
        });
        let err = evaluate_records_governance_packet(inputs)
            .expect_err("non-monotonic sequence is rejected");
        assert!(matches!(
            err,
            RecordsGovernanceError::ChainOfCustodySequenceNotMonotonic { .. }
        ));
    }

    #[test]
    fn queued_for_delete_packet_evaluates() {
        let mut inputs = local_only_inputs();
        inputs.packet_id = "records_governance_packet:queued_for_delete:0001".into();
        inputs.record_class_id = RecordClassId::SupportBundleArchive;
        inputs.artifact_class = ArtifactClass::QueuedForDelete;
        inputs.destruction_caveat_class = DestructionCaveatClass::ProviderBacklog;
        inputs.destruction_caveat_note =
            "Managed-side delete waiting on a provider purge backlog.".into();
        inputs.retention_owner_class = RetentionOwnerClass::Mixed;
        inputs.local_owner_ref = "local_user".into();
        inputs.managed_owner_ref = "support_export".into();
        inputs.chain_of_custody = vec![
            ChainOfCustodyEvent {
                event_id: "evt:0001:created".into(),
                sequence: 0,
                actor_class: CustodyActorClass::LocalUser,
                actor_ref: "local_user_anonymous".into(),
                action_class: CustodyActionClass::Created,
                occurred_at: "2026-05-15T08:00:00Z".into(),
                location_class: CustodyLocationClass::LocalDeviceOnly,
                evidence_ref: None,
                note: "Support bundle minted locally.".into(),
            },
            ChainOfCustodyEvent {
                event_id: "evt:0002:mirrored".into(),
                sequence: 1,
                actor_class: CustodyActorClass::ExportPipeline,
                actor_ref: "support_export".into(),
                action_class: CustodyActionClass::MirroredToManaged,
                occurred_at: "2026-05-15T08:05:00Z".into(),
                location_class: CustodyLocationClass::ManagedArchiveActive,
                evidence_ref: Some("packet:support_intake:0001".into()),
                note: "Managed support copy retained.".into(),
            },
            ChainOfCustodyEvent {
                event_id: "evt:0003:delete_requested".into(),
                sequence: 2,
                actor_class: CustodyActorClass::SupportAgent,
                actor_ref: "support_agent_anonymous".into(),
                action_class: CustodyActionClass::DeleteRequested,
                occurred_at: "2026-05-15T09:00:00Z".into(),
                location_class: CustodyLocationClass::ManagedArchiveReplicatedPendingPurge,
                evidence_ref: Some("packet:delete_request:0001".into()),
                note: "Managed delete queued; replicated copy purge pending.".into(),
            },
        ];
        let packet =
            evaluate_records_governance_packet(inputs).expect("queued-for-delete packet evaluates");
        assert_eq!(packet.artifact_class, ArtifactClass::QueuedForDelete);
        assert!(packet.has_destruction_caveat());
        assert_eq!(packet.custody_event_count(), 3);
    }

    #[test]
    fn preview_seed_carries_metadata_only_class() {
        let packet =
            evaluate_records_governance_packet(local_only_inputs()).expect("packet evaluates");
        let seed = records_governance_preview_item_seed(&packet);
        assert_eq!(seed.data_class, DiagnosticDataClass::MetadataOnly);
        assert_eq!(seed.high_risk_content_class, HighRiskContentClass::NotApplicable);
        assert_eq!(
            seed.support_pack_item_id,
            SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET
        );
        assert!(seed
            .source_refs
            .iter()
            .any(|r| r == "schemas/support/record_class.schema.json"));
    }
}
