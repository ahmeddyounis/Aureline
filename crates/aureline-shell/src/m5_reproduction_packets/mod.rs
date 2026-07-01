//! Reproduction-packet builders with preview-before-share redaction for the M5
//! help, support, and community-handoff surfaces.
//!
//! This module is the in-product producer of the durable [`ReproductionPacket`]
//! a user reviews *before* a report leaves the machine. A packet preserves enough
//! context to make a report actionable — the exact originating surface, the
//! [`ObjectAnchor`] of the locus of concern, and a set of redaction-safe
//! [`IncludedContextItem`] diagnostics/artifacts — while showing, field by field,
//! exactly what will and will not be shared:
//!
//! - **Preview before share.** Every packet carries a [`RedactionPreviewRow`] for
//!   each captured sensitive field — local paths, usernames, hostnames, tokens,
//!   extension inventories, deployment profile, and linked diagnostics/artifacts.
//!   Each row states the [`RedactionActionClass`] Aureline proposes
//!   (`default_action`) and the one the user picked (`chosen_action`). A user may
//!   only *tighten* a row, never loosen it, so the preview can never read safer
//!   than what is actually shared.
//! - **Secrets never leave.** A [`RedactableFieldClass::Token`] row is always
//!   [`RedactionActionClass::RemovedEntirely`], and every packet asserts that raw
//!   bearer tokens, hidden approvals, and unmanaged capture data are excluded —
//!   they are never collected or exported just because they appear in a local log.
//! - **Creation is separate from submission.** A packet is built and previewed by
//!   one of three distinct [`PacketFlowClass`] flows — save-local, copy-summary,
//!   or submit-later — and `auto_submit_on_create_allowed` is always `false`, so
//!   building a packet never silently uploads a support bundle.
//! - **Saved packets survive offline.** A save-local or submit-later packet sets
//!   `offline_reusable = true` and obeys a local [`DataExitBoundary`], so a
//!   blocked or offline handoff degrades to a labeled local artifact instead of
//!   dead-ending.
//!
//! The [`DataExitBoundary`] and redaction-posture vocabulary are reused from the
//! About/help/community destination contract so a packet declares the same
//! versioned, redaction-safe boundary the community-handoff routes
//! ([`crate::m5_community_handoff_targets`]) and the M3 repro-packet preview
//! already publish; the user never has to infer scope from a raw payload.
//!
//! Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
//! tokens, and raw secret material never cross this boundary; the records carry
//! opaque refs, controlled-vocabulary tokens, and bounded reviewable sentences
//! only.
//!
//! The boundary schema is
//! [`schemas/help/m5-reproduction-packet.schema.json`](../../../../schemas/help/m5-reproduction-packet.schema.json).
//! The contract doc is
//! [`docs/help/m5_reproduction_packets_contract.md`](../../../../docs/help/m5_reproduction_packets_contract.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_reproduction_packet_set, seeded_save_local_offline_draft_packet,
    seeded_tokens_and_approvals_removed_packet, M5_REPRODUCTION_PACKET_SET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::public_truth::DataExitBoundary;

/// Stable record-kind tag carried by [`ReproductionPacket`].
pub const REPRODUCTION_PACKET_RECORD_KIND: &str = "reproduction_packet_record";

/// Stable record-kind tag carried by [`M5ReproductionPacketSet`].
pub const M5_REPRODUCTION_PACKET_SET_RECORD_KIND: &str = "m5_reproduction_packet_set";

/// Schema version for a single reproduction packet.
pub const REPRODUCTION_PACKET_SCHEMA_VERSION: u32 = 1;

/// Schema version for the bundled packet set.
pub const M5_REPRODUCTION_PACKET_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this producer projects.
pub const M5_REPRODUCTION_PACKET_SCHEMA_REF: &str =
    "schemas/help/m5-reproduction-packet.schema.json";

/// Repo-relative path of the contract doc all records point at.
pub const M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF: &str =
    "docs/help/m5_reproduction_packets_contract.md";

/// Repo-relative path of the M3 repro-packet preview contract this lane builds
/// on.
pub const M5_REPRODUCTION_PACKET_PREVIEW_BASE_REF: &str =
    "schemas/public/repro_packet_preview.schema.json";

/// Repo-relative path of the community-handoff target contract a packet feeds
/// before a public/community/support route opens.
pub const M5_REPRODUCTION_PACKET_HANDOFF_TARGET_REF: &str =
    "schemas/help/m5-handoff-target.schema.json";

/// Repo-relative path of the frozen M5 public-handoff matrix that governs
/// whether this lane may publish a packet to a public route.
pub const M5_REPRODUCTION_PACKET_PUBLIC_MATRIX_REF: &str =
    "schemas/help/m5-public-handoff-matrix.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REPRODUCTION_PACKET_ARTIFACT_REF: &str =
    "artifacts/help/m5-reproduction-packet-proof/packet_set.json";

/// The originating surface a reproduction packet is anchored to, so the
/// recipient can reproduce the same locus of concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginatingSurfaceClass {
    /// A documentation pane the report is about.
    DocsPane,
    /// A trust / security warning surface.
    TrustWarning,
    /// An update / install screen.
    UpdateScreen,
    /// A workflow bundle (saved automation / task bundle).
    WorkflowBundle,
    /// Any other originating surface not enumerated above.
    OtherSurface,
}

impl OriginatingSurfaceClass {
    /// Every originating surface class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsPane,
        Self::TrustWarning,
        Self::UpdateScreen,
        Self::WorkflowBundle,
        Self::OtherSurface,
    ];

    /// Stable token recorded on the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPane => "docs_pane",
            Self::TrustWarning => "trust_warning",
            Self::UpdateScreen => "update_screen",
            Self::WorkflowBundle => "workflow_bundle",
            Self::OtherSurface => "other_surface",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsPane => "Docs pane",
            Self::TrustWarning => "Trust warning",
            Self::UpdateScreen => "Update screen",
            Self::WorkflowBundle => "Workflow bundle",
            Self::OtherSurface => "Other surface",
        }
    }
}

/// The closed set of sensitive field kinds a packet redacts before share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactableFieldClass {
    /// A local filesystem path.
    LocalPath,
    /// An operating-system username.
    Username,
    /// A machine hostname.
    Hostname,
    /// A bearer token, key, or other secret credential.
    Token,
    /// The installed-extension inventory.
    ExtensionInventory,
    /// The deployment / hosting profile.
    DeploymentProfile,
    /// A linked diagnostic or artifact (log, trace, report).
    LinkedDiagnostic,
}

impl RedactableFieldClass {
    /// Every redactable field class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalPath,
        Self::Username,
        Self::Hostname,
        Self::Token,
        Self::ExtensionInventory,
        Self::DeploymentProfile,
        Self::LinkedDiagnostic,
    ];

    /// Stable token recorded on the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPath => "local_path",
            Self::Username => "username",
            Self::Hostname => "hostname",
            Self::Token => "token",
            Self::ExtensionInventory => "extension_inventory",
            Self::DeploymentProfile => "deployment_profile",
            Self::LinkedDiagnostic => "linked_diagnostic",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalPath => "Local path",
            Self::Username => "Username",
            Self::Hostname => "Hostname",
            Self::Token => "Token / secret",
            Self::ExtensionInventory => "Extension inventory",
            Self::DeploymentProfile => "Deployment profile",
            Self::LinkedDiagnostic => "Linked diagnostic / artifact",
        }
    }

    /// The closed set of redaction actions this field class may take. An action
    /// outside this set is denied so a sensitive field can never be exported in
    /// a raw form.
    pub fn allows_action(self, action: RedactionActionClass) -> bool {
        use RedactionActionClass as A;
        match self {
            // Tokens and secrets are always removed — never exported in any form.
            Self::Token => matches!(action, A::RemovedEntirely),
            // Identifiers are never kept raw: placeholder, generalized class,
            // opaque object ref, or removed.
            Self::LocalPath | Self::Username | Self::Hostname => matches!(
                action,
                A::RedactedPlaceholder
                    | A::GeneralizedClass
                    | A::IncludedAsObjectRef
                    | A::RemovedEntirely
            ),
            // The deployment profile is carried as a class label only, or removed.
            Self::DeploymentProfile => matches!(action, A::GeneralizedClass | A::RemovedEntirely),
            // The extension inventory is carried as a ref or a generalized count,
            // never a raw list of names and paths.
            Self::ExtensionInventory => matches!(
                action,
                A::IncludedAsObjectRef | A::GeneralizedClass | A::RemovedEntirely
            ),
            // Linked diagnostics/artifacts are carried as opaque object refs, or
            // removed.
            Self::LinkedDiagnostic => matches!(action, A::IncludedAsObjectRef | A::RemovedEntirely),
        }
    }

    /// True when this field must always be removed and may never be exported in
    /// any form, regardless of user choice.
    pub const fn is_always_removed(self) -> bool {
        matches!(self, Self::Token)
    }
}

/// What the preview does to one captured sensitive field. Every action keeps the
/// raw value out of the export; they differ only in how much labeled,
/// redaction-safe context survives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionActionClass {
    /// The field is dropped entirely; nothing about it leaves the product.
    RemovedEntirely,
    /// The raw value is replaced with a fixed placeholder (e.g. `<project-root>`).
    RedactedPlaceholder,
    /// The raw value is replaced with a class / generalized label only.
    GeneralizedClass,
    /// The raw value is removed and an opaque object ref is kept so the report
    /// stays actionable.
    IncludedAsObjectRef,
}

impl RedactionActionClass {
    /// Every redaction action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RemovedEntirely,
        Self::RedactedPlaceholder,
        Self::GeneralizedClass,
        Self::IncludedAsObjectRef,
    ];

    /// Stable token recorded on the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemovedEntirely => "removed_entirely",
            Self::RedactedPlaceholder => "redacted_placeholder",
            Self::GeneralizedClass => "generalized_class",
            Self::IncludedAsObjectRef => "included_as_object_ref",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RemovedEntirely => "Removed entirely",
            Self::RedactedPlaceholder => "Redacted placeholder",
            Self::GeneralizedClass => "Generalized class",
            Self::IncludedAsObjectRef => "Included as object ref",
        }
    }

    /// How much labeled context this action lets survive, used to enforce that a
    /// user may only tighten a row. Higher means more context leaves.
    pub const fn exposure_level(self) -> u8 {
        match self {
            Self::RemovedEntirely => 0,
            Self::RedactedPlaceholder => 1,
            Self::GeneralizedClass => 2,
            Self::IncludedAsObjectRef => 3,
        }
    }
}

/// The closed redaction-posture vocabulary applied to a whole packet, reused
/// from the M3 repro-packet preview contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPostureClass {
    /// Fully redacted and safe for a world-readable target.
    FullyRedactedPublicSafe,
    /// Redacted and scoped to a private support channel.
    RedactedSupportScoped,
    /// Restricted to a private security disclosure channel.
    SecurityChannelOnly,
    /// Only redaction-safe metadata and object refs, no payload bodies.
    MetadataRefsOnly,
}

impl RedactionPostureClass {
    /// Every redaction posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyRedactedPublicSafe,
        Self::RedactedSupportScoped,
        Self::SecurityChannelOnly,
        Self::MetadataRefsOnly,
    ];

    /// Stable token recorded on the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyRedactedPublicSafe => "fully_redacted_public_safe",
            Self::RedactedSupportScoped => "redacted_support_scoped",
            Self::SecurityChannelOnly => "security_channel_only",
            Self::MetadataRefsOnly => "metadata_refs_only",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullyRedactedPublicSafe => "Fully redacted, public-safe",
            Self::RedactedSupportScoped => "Redacted, support-scoped",
            Self::SecurityChannelOnly => "Security channel only",
            Self::MetadataRefsOnly => "Metadata refs only",
        }
    }

    /// True when this posture is safe to share to a world-readable target.
    pub const fn is_world_readable_safe(self) -> bool {
        matches!(self, Self::FullyRedactedPublicSafe | Self::MetadataRefsOnly)
    }

    /// Whether this posture permits the given data-exit boundary.
    pub fn allows_data_exit(self, data_exit: DataExitBoundary) -> bool {
        use DataExitBoundary as D;
        match self {
            Self::FullyRedactedPublicSafe => matches!(
                data_exit,
                D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs | D::ProposalRefsOnly
            ),
            Self::MetadataRefsOnly => {
                matches!(
                    data_exit,
                    D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs
                )
            }
            Self::RedactedSupportScoped => matches!(
                data_exit,
                D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs | D::RedactedSupportPacket
            ),
            Self::SecurityChannelOnly => {
                matches!(
                    data_exit,
                    D::NoPayloadLeavesProduct | D::SecurityPayloadsOnly
                )
            }
        }
    }
}

/// The three distinct flows a reproduction packet supports. Packet creation and
/// submission stay separate: no flow auto-submits at build time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketFlowClass {
    /// Save the packet to a local artifact that never leaves the product.
    SaveLocal,
    /// Copy a redaction-safe text summary to the clipboard.
    CopySummary,
    /// Stage the packet for later submission, separate from creation.
    SubmitLater,
}

impl PacketFlowClass {
    /// Every flow, in declaration order.
    pub const ALL: [Self; 3] = [Self::SaveLocal, Self::CopySummary, Self::SubmitLater];

    /// Stable token recorded on the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SaveLocal => "save_local",
            Self::CopySummary => "copy_summary",
            Self::SubmitLater => "submit_later",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SaveLocal => "Save local",
            Self::CopySummary => "Copy summary",
            Self::SubmitLater => "Submit later",
        }
    }

    /// True when sharing through this flow moves redaction-safe data off the
    /// machine (clipboard or a later submission).
    pub const fn leaves_product_on_share(self) -> bool {
        matches!(self, Self::CopySummary | Self::SubmitLater)
    }

    /// True when this flow keeps a packet reusable offline after creation.
    pub const fn keeps_offline_copy(self) -> bool {
        matches!(self, Self::SaveLocal | Self::SubmitLater)
    }

    /// Whether this flow permits the given data-exit boundary.
    pub fn allows_data_exit(self, data_exit: DataExitBoundary) -> bool {
        use DataExitBoundary as D;
        match self {
            // A saved packet never leaves the product.
            Self::SaveLocal => matches!(data_exit, D::NoPayloadLeavesProduct),
            // A copied summary carries only redaction-safe metadata.
            Self::CopySummary => {
                matches!(
                    data_exit,
                    D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs
                )
            }
            // A staged submission may later carry any redaction-safe boundary.
            Self::SubmitLater => matches!(
                data_exit,
                D::NoPayloadLeavesProduct
                    | D::MetadataSafeObjectRefs
                    | D::ProposalRefsOnly
                    | D::RedactedSupportPacket
                    | D::SecurityPayloadsOnly
            ),
        }
    }
}

/// The closed set of redaction-safe diagnostics/artifacts a packet may carry, by
/// opaque ref, so the report is actionable without raw payload bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncludedContextClass {
    /// The build identity (version, channel, commit ref).
    BuildIdentity,
    /// A redaction-safe environment capsule (class facts only).
    EnvironmentCapsule,
    /// A redacted tail of a relevant log.
    RedactedLogTail,
    /// A sanitized configuration snapshot.
    SanitizedConfigSnapshot,
    /// Free-text reproduction steps.
    ReproStepsText,
    /// The anchor / object identity ref.
    AnchorObjectRef,
    /// A performance trace ref.
    PerformanceTrace,
}

impl IncludedContextClass {
    /// Stable token recorded on the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::EnvironmentCapsule => "environment_capsule",
            Self::RedactedLogTail => "redacted_log_tail",
            Self::SanitizedConfigSnapshot => "sanitized_config_snapshot",
            Self::ReproStepsText => "repro_steps_text",
            Self::AnchorObjectRef => "anchor_object_ref",
            Self::PerformanceTrace => "performance_trace",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BuildIdentity => "Build identity",
            Self::EnvironmentCapsule => "Environment capsule",
            Self::RedactedLogTail => "Redacted log tail",
            Self::SanitizedConfigSnapshot => "Sanitized config snapshot",
            Self::ReproStepsText => "Repro steps text",
            Self::AnchorObjectRef => "Anchor object ref",
            Self::PerformanceTrace => "Performance trace",
        }
    }
}

/// The exact anchor / object identity a packet is about, so the report names a
/// precise locus rather than a fuzzy description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectAnchor {
    /// Opaque ref of the originating anchor (surface, position, selection).
    pub anchor_ref: String,
    /// Opaque ref of the object the report is about.
    pub object_ref: String,
    /// Reviewer-facing anchor label.
    pub anchor_label: String,
}

/// One row of the redaction preview: a captured sensitive field, the action
/// Aureline proposes, and the action the user picked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionPreviewRow {
    /// The kind of sensitive field this row redacts.
    pub field_class: RedactableFieldClass,
    /// The redaction action Aureline proposes for this field.
    pub default_action: RedactionActionClass,
    /// The redaction action the user chose; may tighten the default, never loosen
    /// it.
    pub chosen_action: RedactionActionClass,
    /// Whether this field cannot be loosened below its default (always true for
    /// tokens and secrets).
    pub mandatory_redaction: bool,
    /// A bounded reviewable label showing what the recipient will see for this
    /// field (e.g. `<project-root>`), never the raw value.
    pub redacted_preview_label: String,
    /// A bounded reviewable sentence describing the redaction.
    pub field_summary: String,
}

impl RedactionPreviewRow {
    fn validate(&self, packet_id: &str) -> Result<(), ReproductionPacketError> {
        if !self.field_class.allows_action(self.default_action) {
            return Err(ReproductionPacketError::FieldActionNotAllowed {
                packet_id: packet_id.to_owned(),
                field: self.field_class,
                action: self.default_action,
            });
        }
        if !self.field_class.allows_action(self.chosen_action) {
            return Err(ReproductionPacketError::FieldActionNotAllowed {
                packet_id: packet_id.to_owned(),
                field: self.field_class,
                action: self.chosen_action,
            });
        }
        // The user may only tighten a row: the chosen action must not expose more
        // than the proposed default.
        if self.chosen_action.exposure_level() > self.default_action.exposure_level() {
            return Err(ReproductionPacketError::ChosenLoosensRedaction {
                packet_id: packet_id.to_owned(),
                field: self.field_class,
            });
        }
        // A field that must always be removed is removed by both default and
        // chosen action and is flagged mandatory.
        if self.field_class.is_always_removed()
            && (self.default_action != RedactionActionClass::RemovedEntirely
                || self.chosen_action != RedactionActionClass::RemovedEntirely
                || !self.mandatory_redaction)
        {
            return Err(ReproductionPacketError::MandatoryFieldNotRemoved {
                packet_id: packet_id.to_owned(),
                field: self.field_class,
            });
        }
        if non_empty(&self.redacted_preview_label).is_none()
            || non_empty(&self.field_summary).is_none()
        {
            return Err(ReproductionPacketError::EmptyRequiredField {
                record_id: packet_id.to_owned(),
                field: "redaction_preview_row",
            });
        }
        Ok(())
    }
}

/// One redaction-safe diagnostic/artifact carried by a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncludedContextItem {
    /// The kind of context this item carries.
    pub context_class: IncludedContextClass,
    /// Opaque ref of the included context.
    pub context_ref: String,
    /// Whether redaction has been applied to this item; always true.
    pub redaction_applied: bool,
    /// A bounded reviewable sentence describing the item.
    pub item_summary: String,
}

impl IncludedContextItem {
    fn validate(&self, packet_id: &str) -> Result<(), ReproductionPacketError> {
        if !ref_is_opaque(&self.context_ref) {
            return Err(ReproductionPacketError::RawRefLeak {
                record_id: packet_id.to_owned(),
                field: "included_context.context_ref",
            });
        }
        if !self.redaction_applied {
            return Err(ReproductionPacketError::ContextNotRedacted {
                packet_id: packet_id.to_owned(),
            });
        }
        if non_empty(&self.item_summary).is_none() {
            return Err(ReproductionPacketError::EmptyRequiredField {
                record_id: packet_id.to_owned(),
                field: "included_context.item_summary",
            });
        }
        Ok(())
    }
}

/// One reproduction packet a user previews before a report leaves the machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproductionPacket {
    /// Schema version for this packet shape.
    pub reproduction_packet_schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable packet id; prefixed `reproduction_packet:`.
    pub packet_id: String,
    /// The originating surface this packet is anchored to.
    pub originating_surface: OriginatingSurfaceClass,
    /// The exact object anchor of the locus of concern.
    pub object_anchor: ObjectAnchor,
    /// The flow used to build and review this packet.
    pub flow: PacketFlowClass,
    /// The redaction posture applied to the whole packet.
    pub redaction_posture: RedactionPostureClass,
    /// The data-exit boundary the packet obeys.
    pub data_exit_boundary: DataExitBoundary,
    /// The redaction preview: one row per captured sensitive field.
    pub redaction_preview: Vec<RedactionPreviewRow>,
    /// The redaction-safe diagnostics/artifacts the packet carries.
    pub included_context: Vec<IncludedContextItem>,
    /// Whether the user confirmed the preview before any share.
    pub preview_confirmed_before_share: bool,
    /// Whether the packet stays reusable offline after creation.
    pub offline_reusable: bool,
    /// Whether building this packet may auto-submit it; always false so creation
    /// and submission stay separate.
    pub auto_submit_on_create_allowed: bool,
    /// Whether raw bearer tokens / secrets are excluded; always true.
    pub raw_secrets_excluded: bool,
    /// Whether raw screenshots are excluded; always true.
    pub raw_screenshots_excluded: bool,
    /// Whether hidden approvals are excluded; always true.
    pub hidden_approvals_excluded: bool,
    /// Whether unmanaged capture data is excluded; always true.
    pub unmanaged_capture_excluded: bool,
    /// Reviewer-facing headline label.
    pub headline_label: String,
    /// A bounded reviewable sentence summarizing the packet.
    pub packet_summary: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
    /// Optional reviewer note.
    pub notes: Option<String>,
}

impl ReproductionPacket {
    /// Validate the packet against the reproduction-packet contract.
    pub fn validate(&self) -> Result<(), ReproductionPacketError> {
        if self.reproduction_packet_schema_version != REPRODUCTION_PACKET_SCHEMA_VERSION {
            return Err(ReproductionPacketError::WrongPacketSchemaVersion {
                packet_id: self.packet_id.clone(),
                actual: self.reproduction_packet_schema_version,
            });
        }
        if self.record_kind != REPRODUCTION_PACKET_RECORD_KIND {
            return Err(ReproductionPacketError::WrongPacketRecordKind {
                packet_id: self.packet_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.packet_id.starts_with("reproduction_packet:") {
            return Err(ReproductionPacketError::MalformedPacketId {
                packet_id: self.packet_id.clone(),
            });
        }
        if self.contract_doc_ref != M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF {
            return Err(ReproductionPacketError::WrongContractDocRef {
                record_id: self.packet_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("packet_summary", &self.packet_summary),
            (
                "object_anchor.anchor_label",
                &self.object_anchor.anchor_label,
            ),
        ] {
            if non_empty(value).is_none() {
                return Err(ReproductionPacketError::EmptyRequiredField {
                    record_id: self.packet_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.object_anchor.anchor_ref)
            || !ref_is_opaque(&self.object_anchor.object_ref)
        {
            return Err(ReproductionPacketError::RawRefLeak {
                record_id: self.packet_id.clone(),
                field: "object_anchor",
            });
        }

        // Guardrail: raw secrets, screenshots, hidden approvals, and unmanaged
        // capture are never collected or exported.
        if !self.raw_secrets_excluded
            || !self.raw_screenshots_excluded
            || !self.hidden_approvals_excluded
            || !self.unmanaged_capture_excluded
        {
            return Err(ReproductionPacketError::GuardrailExclusionMissing {
                packet_id: self.packet_id.clone(),
            });
        }
        // Out-of-scope guardrail: packet creation never auto-submits.
        if self.auto_submit_on_create_allowed {
            return Err(ReproductionPacketError::AutoSubmitOnCreate {
                packet_id: self.packet_id.clone(),
            });
        }

        // The redaction preview names at least one field and every row is valid.
        if self.redaction_preview.is_empty() {
            return Err(ReproductionPacketError::EmptyRedactionPreview {
                packet_id: self.packet_id.clone(),
            });
        }
        let mut seen_fields: BTreeSet<RedactableFieldClass> = BTreeSet::new();
        for row in &self.redaction_preview {
            row.validate(&self.packet_id)?;
            if !seen_fields.insert(row.field_class) {
                return Err(ReproductionPacketError::DuplicateRedactionField {
                    packet_id: self.packet_id.clone(),
                    field: row.field_class,
                });
            }
        }

        // Included context is redaction-safe and ref-only.
        for item in &self.included_context {
            item.validate(&self.packet_id)?;
        }

        // Posture and flow each pin the data-exit boundary.
        if !self
            .redaction_posture
            .allows_data_exit(self.data_exit_boundary)
        {
            return Err(ReproductionPacketError::PostureDataExitMismatch {
                packet_id: self.packet_id.clone(),
                posture: self.redaction_posture,
                data_exit: self.data_exit_boundary,
            });
        }
        if !self.flow.allows_data_exit(self.data_exit_boundary) {
            return Err(ReproductionPacketError::FlowDataExitMismatch {
                packet_id: self.packet_id.clone(),
                flow: self.flow,
                data_exit: self.data_exit_boundary,
            });
        }

        // A saved or staged packet stays reusable offline.
        if self.flow.keeps_offline_copy() && !self.offline_reusable {
            return Err(ReproductionPacketError::OfflineCopyNotReusable {
                packet_id: self.packet_id.clone(),
                flow: self.flow,
            });
        }

        // Anything that leaves the product must be previewed and confirmed first.
        let leaves = self.flow.leaves_product_on_share()
            || self.data_exit_boundary != DataExitBoundary::NoPayloadLeavesProduct;
        if leaves && !self.preview_confirmed_before_share {
            return Err(ReproductionPacketError::ShareBeforePreviewConfirmed {
                packet_id: self.packet_id.clone(),
            });
        }

        Ok(())
    }

    /// Render a deterministic, redaction-safe copy summary — the text a
    /// copy-summary flow places on the clipboard, and a reviewer-facing preview
    /// for every flow. Stable for the same input snapshot.
    pub fn render_copy_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("[{}] {}\n", self.packet_id, self.headline_label));
        out.push_str(&format!(
            "    surface: {} | anchor: {} (object={})\n",
            self.originating_surface.as_str(),
            self.object_anchor.anchor_ref,
            self.object_anchor.object_ref,
        ));
        out.push_str(&format!(
            "    flow={} posture={} data_exit={} preview_confirmed={}\n",
            self.flow.as_str(),
            self.redaction_posture.as_str(),
            self.data_exit_boundary.as_str(),
            self.preview_confirmed_before_share,
        ));
        out.push_str("    redaction preview:\n");
        for row in &self.redaction_preview {
            out.push_str(&format!(
                "      - {} -> {} ({})\n",
                row.field_class.as_str(),
                row.chosen_action.as_str(),
                row.redacted_preview_label,
            ));
        }
        for item in &self.included_context {
            out.push_str(&format!(
                "    included: {} ({})\n",
                item.context_class.as_str(),
                item.context_ref,
            ));
        }
        out
    }
}

/// A bundled set of reproduction packets, one per originating surface, checked in
/// as the canonical M5 source for repro-sharing and capture-boundary truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReproductionPacketSet {
    /// Schema version for the packet-set shape.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable id for the packet set.
    pub packet_set_id: String,
    /// Reviewer-facing label for the packet set.
    pub packet_set_label: String,
    /// One packet per originating surface.
    pub packets: Vec<ReproductionPacket>,
    /// Source contracts this set binds to by id.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token covering the export boundary.
    pub redaction_class_token: String,
    /// Opaque mint timestamp ref.
    pub minted_at: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
}

impl M5ReproductionPacketSet {
    /// Validate the packet set: every packet validates, every originating
    /// surface and flow is represented, every redactable field class is covered
    /// somewhere, no two packets share an id, and the source contracts are
    /// present.
    pub fn validate(&self) -> Result<(), ReproductionPacketError> {
        if self.schema_version != M5_REPRODUCTION_PACKET_SET_SCHEMA_VERSION {
            return Err(ReproductionPacketError::WrongSetSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_REPRODUCTION_PACKET_SET_RECORD_KIND {
            return Err(ReproductionPacketError::WrongSetRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if non_empty(&self.packet_set_id).is_none()
            || non_empty(&self.packet_set_label).is_none()
            || non_empty(&self.redaction_class_token).is_none()
            || non_empty(&self.minted_at).is_none()
        {
            return Err(ReproductionPacketError::SetIdentityIncomplete);
        }
        if self.contract_doc_ref != M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF {
            return Err(ReproductionPacketError::WrongContractDocRef {
                record_id: self.packet_set_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for packet in &self.packets {
            packet.validate()?;
            if !seen.insert(packet.packet_id.as_str()) {
                return Err(ReproductionPacketError::DuplicatePacketId {
                    packet_id: packet.packet_id.clone(),
                });
            }
        }

        // Every originating surface is named exactly once.
        for surface in OriginatingSurfaceClass::ALL {
            if !self
                .packets
                .iter()
                .any(|p| p.originating_surface == surface)
            {
                return Err(ReproductionPacketError::SurfaceMissing { surface });
            }
        }

        // Every flow is exercised by some packet.
        for flow in PacketFlowClass::ALL {
            if !self.packets.iter().any(|p| p.flow == flow) {
                return Err(ReproductionPacketError::FlowMissing { flow });
            }
        }

        // Every redactable field class is covered by some packet's preview, so a
        // sensitive field never slips through unmodeled.
        for field in RedactableFieldClass::ALL {
            let covered = self
                .packets
                .iter()
                .any(|p| p.redaction_preview.iter().any(|r| r.field_class == field));
            if !covered {
                return Err(ReproductionPacketError::FieldClassUncovered { field });
            }
        }

        // Source contracts bound by id.
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            M5_REPRODUCTION_PACKET_SCHEMA_REF,
            M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF,
            M5_REPRODUCTION_PACKET_PREVIEW_BASE_REF,
            M5_REPRODUCTION_PACKET_HANDOFF_TARGET_REF,
            M5_REPRODUCTION_PACKET_PUBLIC_MATRIX_REF,
        ] {
            if !refs.contains(required) {
                return Err(ReproductionPacketError::MissingSourceContracts);
            }
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("reproduction packet set serializes"),
        ) {
            return Err(ReproductionPacketError::RawMaterialInExport);
        }

        Ok(())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("reproduction packet set serializes")
    }

    /// Deterministic, machine-readable CSV: one row per packet, naming its
    /// originating surface, flow, redaction posture, data-exit boundary,
    /// redaction-row count, and the offline/preview gates.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "packet,originating_surface,flow,redaction_posture,data_exit_boundary,redaction_rows,preview_confirmed,offline_reusable\n",
        );
        for packet in &self.packets {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                packet.packet_id,
                packet.originating_surface.as_str(),
                packet.flow.as_str(),
                packet.redaction_posture.as_str(),
                packet.data_exit_boundary.as_str(),
                packet.redaction_preview.len(),
                packet.preview_confirmed_before_share,
                packet.offline_reusable,
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 reproduction-packet review\n\n");
        out.push_str(&format!("Packet set: `{}`\n\n", self.packet_set_id));
        out.push_str(
            "| Packet | Surface | Flow | Posture | Data exit | Preview confirmed? | Offline reusable? |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for packet in &self.packets {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | `{}` | {} | {} |\n",
                packet.packet_id,
                packet.originating_surface.label(),
                packet.flow.label(),
                packet.redaction_posture.label(),
                packet.data_exit_boundary.as_str(),
                packet.preview_confirmed_before_share,
                packet.offline_reusable,
            ));
        }
        out.push('\n');
        out.push_str(
            "Every packet previews each sensitive field before share, tokens are always removed, ",
        );
        out.push_str("and packet creation never auto-submits — building a packet is separate from sending it.\n");
        out
    }
}

/// True when a ref is an opaque token rather than a raw URL, email, or blank.
fn ref_is_opaque(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed == value
        && !trimmed.contains("://")
        && !trimmed.contains('@')
        && !trimmed.contains(char::is_whitespace)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Closed validation-error vocabulary for the reproduction-packet contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproductionPacketError {
    WrongPacketSchemaVersion {
        packet_id: String,
        actual: u32,
    },
    WrongPacketRecordKind {
        packet_id: String,
        actual: String,
    },
    MalformedPacketId {
        packet_id: String,
    },
    GuardrailExclusionMissing {
        packet_id: String,
    },
    AutoSubmitOnCreate {
        packet_id: String,
    },
    EmptyRedactionPreview {
        packet_id: String,
    },
    DuplicateRedactionField {
        packet_id: String,
        field: RedactableFieldClass,
    },
    FieldActionNotAllowed {
        packet_id: String,
        field: RedactableFieldClass,
        action: RedactionActionClass,
    },
    ChosenLoosensRedaction {
        packet_id: String,
        field: RedactableFieldClass,
    },
    MandatoryFieldNotRemoved {
        packet_id: String,
        field: RedactableFieldClass,
    },
    ContextNotRedacted {
        packet_id: String,
    },
    PostureDataExitMismatch {
        packet_id: String,
        posture: RedactionPostureClass,
        data_exit: DataExitBoundary,
    },
    FlowDataExitMismatch {
        packet_id: String,
        flow: PacketFlowClass,
        data_exit: DataExitBoundary,
    },
    OfflineCopyNotReusable {
        packet_id: String,
        flow: PacketFlowClass,
    },
    ShareBeforePreviewConfirmed {
        packet_id: String,
    },
    WrongSetSchemaVersion {
        actual: u32,
    },
    WrongSetRecordKind {
        actual: String,
    },
    SetIdentityIncomplete,
    DuplicatePacketId {
        packet_id: String,
    },
    SurfaceMissing {
        surface: OriginatingSurfaceClass,
    },
    FlowMissing {
        flow: PacketFlowClass,
    },
    FieldClassUncovered {
        field: RedactableFieldClass,
    },
    MissingSourceContracts,
    RawMaterialInExport,
    WrongContractDocRef {
        record_id: String,
        actual: String,
    },
    EmptyRequiredField {
        record_id: String,
        field: &'static str,
    },
    RawRefLeak {
        record_id: String,
        field: &'static str,
    },
}

impl fmt::Display for ReproductionPacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongPacketSchemaVersion { packet_id, actual } => write!(
                f,
                "packet {packet_id} has unsupported reproduction_packet_schema_version {actual}"
            ),
            Self::WrongPacketRecordKind { packet_id, actual } => {
                write!(f, "packet {packet_id} has unsupported record kind {actual}")
            }
            Self::MalformedPacketId { packet_id } => {
                write!(f, "packet id {packet_id} must start with reproduction_packet:")
            }
            Self::GuardrailExclusionMissing { packet_id } => write!(
                f,
                "packet {packet_id} must exclude raw secrets, screenshots, hidden approvals, and unmanaged capture"
            ),
            Self::AutoSubmitOnCreate { packet_id } => write!(
                f,
                "packet {packet_id} must not auto-submit on create; creation and submission stay separate"
            ),
            Self::EmptyRedactionPreview { packet_id } => {
                write!(f, "packet {packet_id} must preview at least one sensitive field")
            }
            Self::DuplicateRedactionField { packet_id, field } => write!(
                f,
                "packet {packet_id} repeats redaction field {}",
                field.as_str()
            ),
            Self::FieldActionNotAllowed {
                packet_id,
                field,
                action,
            } => write!(
                f,
                "packet {packet_id} field {} cannot take redaction action {}",
                field.as_str(),
                action.as_str()
            ),
            Self::ChosenLoosensRedaction { packet_id, field } => write!(
                f,
                "packet {packet_id} field {} chosen action loosens the proposed redaction",
                field.as_str()
            ),
            Self::MandatoryFieldNotRemoved { packet_id, field } => write!(
                f,
                "packet {packet_id} field {} must always be removed entirely",
                field.as_str()
            ),
            Self::ContextNotRedacted { packet_id } => {
                write!(f, "packet {packet_id} carries an unredacted context item")
            }
            Self::PostureDataExitMismatch {
                packet_id,
                posture,
                data_exit,
            } => write!(
                f,
                "packet {packet_id} posture {} cannot use data exit {}",
                posture.as_str(),
                data_exit.as_str()
            ),
            Self::FlowDataExitMismatch {
                packet_id,
                flow,
                data_exit,
            } => write!(
                f,
                "packet {packet_id} flow {} cannot use data exit {}",
                flow.as_str(),
                data_exit.as_str()
            ),
            Self::OfflineCopyNotReusable { packet_id, flow } => write!(
                f,
                "packet {packet_id} flow {} must stay reusable offline",
                flow.as_str()
            ),
            Self::ShareBeforePreviewConfirmed { packet_id } => write!(
                f,
                "packet {packet_id} shares before the preview was confirmed"
            ),
            Self::WrongSetSchemaVersion { actual } => {
                write!(f, "packet set has unsupported schema_version {actual}")
            }
            Self::WrongSetRecordKind { actual } => {
                write!(f, "packet set has unsupported record kind {actual}")
            }
            Self::SetIdentityIncomplete => {
                write!(f, "packet set is missing required identity fields")
            }
            Self::DuplicatePacketId { packet_id } => {
                write!(f, "packet set has duplicate packet id {packet_id}")
            }
            Self::SurfaceMissing { surface } => {
                write!(f, "packet set is missing surface {}", surface.as_str())
            }
            Self::FlowMissing { flow } => {
                write!(f, "packet set is missing flow {}", flow.as_str())
            }
            Self::FieldClassUncovered { field } => write!(
                f,
                "packet set never previews redaction field {}",
                field.as_str()
            ),
            Self::MissingSourceContracts => {
                write!(f, "packet set is missing a required source contract ref")
            }
            Self::RawMaterialInExport => {
                write!(f, "packet set export carries forbidden raw material")
            }
            Self::WrongContractDocRef { record_id, actual } => {
                write!(f, "record {record_id} cites wrong contract doc {actual}")
            }
            Self::EmptyRequiredField { record_id, field } => {
                write!(f, "record {record_id} is missing required field {field}")
            }
            Self::RawRefLeak { record_id, field } => write!(
                f,
                "record {record_id} field {field} contains a raw URL, email, or whitespace; opaque refs only"
            ),
        }
    }
}

impl Error for ReproductionPacketError {}

/// Reads and validates the checked-in stable reproduction packet set.
pub fn current_stable_m5_reproduction_packet_set() -> Result<M5ReproductionPacketSet, Box<dyn Error>>
{
    let set: M5ReproductionPacketSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-reproduction-packet-proof/packet_set.json"
    )))?;
    set.validate()?;
    Ok(set)
}
