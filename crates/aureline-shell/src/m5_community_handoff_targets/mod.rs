//! Community-handoff target review sheets for the M5 issue, security-disclosure,
//! docs-feedback, RFC/discussion, community-support, and official-support routes.
//!
//! This module is the in-product producer of the durable
//! [`CommunityHandoffTargetSheet`] that the help, support, ecosystem, and
//! reporting surfaces render *before* a browser or handoff target opens. Each
//! sheet answers, for one outbound route, the questions a user must be able to
//! settle before their context leaves Aureline:
//!
//! - **Who will receive this, and how visible will it be?** Every sheet pins a
//!   typed [`DestinationTrustClass`] (`Official public`, `Official
//!   authenticated`, `Community`, `Private / security`, `Local only`), a typed
//!   [`VisibilityBoundaryClass`], a typed [`AuthExpectationClass`], and a
//!   reviewable data-exit note bound to a [`DataExitBoundary`] so the recipient
//!   and the visibility boundary are named, not inferred from a link color.
//! - **Is this a guaranteed product commitment?** A [`CommitmentHonesty`] block
//!   states whether the route is an official supported commitment or a
//!   best-effort community / public-forum / private-security path, so support
//!   and community links never masquerade as guarantees.
//! - **What survives if the handoff is blocked?** Every sheet carries a
//!   [`LocalSafeFallback`] whose destination never leaves the product, so a
//!   blocked, offline, or unsupported-profile route degrades to a labeled local
//!   path instead of dead-ending.
//!
//! The five-class destination/trust vocabulary and the build-context export
//! vocabulary are reused from [`crate::public_truth`] and the About/help/
//! community destination contract so the issue/report/disclosure lanes carry the
//! same versioned, redaction-safe export the About and community surfaces
//! already publish — the user never has to infer scope from a raw URL. Official
//! and community routes stay distinguishable because the trust class is always
//! explicit and reused verbatim across in-product surfaces and exported issue/
//! support packets.
//!
//! Two acceptance invariants are enforced structurally:
//!
//! - **No accidental public coercion.** A route may only target a trust class
//!   from its allowed set, world-readable routes require prior review and may
//!   never auto-open from a critical alert, and private/security routes always
//!   carry an explicit unsupported-profile disclosure plus a local-safe
//!   fallback.
//! - **Object anchors and issue-template support are preserved.** When Aureline
//!   can hand off richer context, the sheet names the exact [`ObjectAnchor`] and
//!   the [`IssueTemplateSupport`] block rather than a fuzzy description.
//!
//! Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
//! tokens, and raw secret material never cross this boundary; the records carry
//! opaque refs, controlled-vocabulary tokens, and bounded reviewable sentences
//! only.
//!
//! The boundary schema is
//! [`schemas/help/m5-handoff-target.schema.json`](../../../../schemas/help/m5-handoff-target.schema.json).
//! The contract doc is
//! [`docs/help/m5_community_handoff_targets_contract.md`](../../../../docs/help/m5_community_handoff_targets_contract.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_community_support_sheet_no_commitment, seeded_m5_community_handoff_target_sheet_set,
    seeded_security_disclosure_sheet_unsupported_profile, M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::public_truth::{BuildContextExport, BuildContextExportClass, DataExitBoundary};

/// Stable record-kind tag carried by [`CommunityHandoffTargetSheet`].
pub const COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND: &str =
    "community_handoff_target_sheet_record";

/// Stable record-kind tag carried by [`M5CommunityHandoffTargetSheetSet`].
pub const M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_RECORD_KIND: &str =
    "m5_community_handoff_target_sheet_set";

/// Schema version for a single target review sheet.
pub const COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION: u32 = 1;

/// Schema version for the bundled sheet set.
pub const M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this producer projects.
pub const M5_COMMUNITY_HANDOFF_TARGET_SCHEMA_REF: &str =
    "schemas/help/m5-handoff-target.schema.json";

/// Repo-relative path of the contract doc all records point at.
pub const M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF: &str =
    "docs/help/m5_community_handoff_targets_contract.md";

/// Repo-relative path of the community-handoff packet contract this lane mirrors.
pub const M5_COMMUNITY_HANDOFF_PACKET_CONTRACT_REF: &str =
    "schemas/help/community-handoff-packet.schema.json";

/// Repo-relative path of the M3 handoff-target review contract this lane builds
/// on.
pub const M5_COMMUNITY_HANDOFF_TARGET_REVIEW_BASE_REF: &str =
    "schemas/public/handoff_target_review.schema.json";

/// Repo-relative path of the frozen M5 public-handoff matrix that governs
/// whether this lane may publish a route claim.
pub const M5_COMMUNITY_HANDOFF_PUBLIC_MATRIX_REF: &str =
    "schemas/help/m5-public-handoff-matrix.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMMUNITY_HANDOFF_TARGET_ARTIFACT_REF: &str =
    "artifacts/help/m5-community-handoff-proof/target_set.json";

/// The six governed outbound community-handoff routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunityHandoffRouteClass {
    /// File a public issue against the project tracker.
    PublicIssue,
    /// Privately disclose a security finding.
    SecurityDisclosure,
    /// Send feedback or a correction about the documentation.
    DocsFeedback,
    /// Open or join an RFC / design discussion.
    RfcDiscussion,
    /// Ask the community for help on a public forum / discussion board.
    CommunitySupport,
    /// Open an official, authenticated support intake.
    OfficialSupport,
}

impl CommunityHandoffRouteClass {
    /// Every governed route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublicIssue,
        Self::SecurityDisclosure,
        Self::DocsFeedback,
        Self::RfcDiscussion,
        Self::CommunitySupport,
        Self::OfficialSupport,
    ];

    /// Stable token recorded on the sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicIssue => "public_issue",
            Self::SecurityDisclosure => "security_disclosure",
            Self::DocsFeedback => "docs_feedback",
            Self::RfcDiscussion => "rfc_discussion",
            Self::CommunitySupport => "community_support",
            Self::OfficialSupport => "official_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PublicIssue => "Public issue",
            Self::SecurityDisclosure => "Security disclosure",
            Self::DocsFeedback => "Docs feedback",
            Self::RfcDiscussion => "RFC / discussion",
            Self::CommunitySupport => "Community support",
            Self::OfficialSupport => "Official support",
        }
    }

    /// The closed set of destination trust classes a route may target. A trust
    /// class outside this set is denied so the routes never blur together or
    /// coerce the user into a public target by accident.
    pub fn allows_trust(self, trust: DestinationTrustClass) -> bool {
        use DestinationTrustClass as T;
        match self {
            Self::PublicIssue | Self::DocsFeedback | Self::RfcDiscussion => {
                matches!(trust, T::OfficialPublic | T::Community)
            }
            Self::CommunitySupport => matches!(trust, T::Community),
            Self::SecurityDisclosure => matches!(trust, T::PrivateSecurity),
            Self::OfficialSupport => matches!(trust, T::OfficialAuthenticated),
        }
    }
}

/// The five-class destination/trust vocabulary, reused verbatim across in-product
/// surfaces and exported issue/support packets so official and community routes
/// stay distinguishable everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationTrustClass {
    /// Official, first-party, world-readable destination (e.g. the project
    /// issue tracker).
    OfficialPublic,
    /// Official, first-party destination reached behind an authenticated plane
    /// (e.g. an official support intake).
    OfficialAuthenticated,
    /// Community-run destination (forum, discussion board, chat) that is not an
    /// official commitment.
    Community,
    /// Private destination for security disclosure handled confidentially.
    PrivateSecurity,
    /// Local-only destination: the packet is drafted and previewed but never
    /// leaves the product.
    LocalOnly,
}

impl DestinationTrustClass {
    /// Every trust class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OfficialPublic,
        Self::OfficialAuthenticated,
        Self::Community,
        Self::PrivateSecurity,
        Self::LocalOnly,
    ];

    /// Stable token recorded on the sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialAuthenticated => "official_authenticated",
            Self::Community => "community",
            Self::PrivateSecurity => "private_security",
            Self::LocalOnly => "local_only",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OfficialPublic => "Official public",
            Self::OfficialAuthenticated => "Official authenticated",
            Self::Community => "Community",
            Self::PrivateSecurity => "Private / security",
            Self::LocalOnly => "Local only",
        }
    }

    /// True when the destination is an official first-party channel.
    pub const fn is_official(self) -> bool {
        matches!(self, Self::OfficialPublic | Self::OfficialAuthenticated)
    }

    /// True when the destination is community-run.
    pub const fn is_community(self) -> bool {
        matches!(self, Self::Community)
    }

    /// True when sharing to this destination makes the report world-readable.
    pub const fn is_world_readable(self) -> bool {
        matches!(self, Self::OfficialPublic | Self::Community)
    }
}

/// The visibility boundary that applies once the report leaves the product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityBoundaryClass {
    /// World-readable to anyone, including search engines.
    WorldReadablePublic,
    /// Visible only inside the authenticated official support plane.
    OfficialAccountVisible,
    /// Visible to the community on the forum / discussion board.
    CommunityVisible,
    /// Confined to a private security channel.
    PrivateSecurityChannel,
    /// Never leaves the product.
    LocalNeverLeaves,
}

impl VisibilityBoundaryClass {
    /// Stable token recorded on the sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorldReadablePublic => "world_readable_public",
            Self::OfficialAccountVisible => "official_account_visible",
            Self::CommunityVisible => "community_visible",
            Self::PrivateSecurityChannel => "private_security_channel",
            Self::LocalNeverLeaves => "local_never_leaves",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorldReadablePublic => "World-readable public",
            Self::OfficialAccountVisible => "Official account visible",
            Self::CommunityVisible => "Community visible",
            Self::PrivateSecurityChannel => "Private security channel",
            Self::LocalNeverLeaves => "Local, never leaves",
        }
    }

    /// Whether this visibility boundary is consistent with the trust class.
    pub fn allowed_for_trust(self, trust: DestinationTrustClass) -> bool {
        use DestinationTrustClass as T;
        match trust {
            T::OfficialPublic => matches!(self, Self::WorldReadablePublic),
            T::Community => matches!(self, Self::CommunityVisible | Self::WorldReadablePublic),
            T::OfficialAuthenticated => matches!(self, Self::OfficialAccountVisible),
            T::PrivateSecurity => matches!(self, Self::PrivateSecurityChannel),
            T::LocalOnly => matches!(self, Self::LocalNeverLeaves),
        }
    }
}

/// What authentication the user should expect before the handoff target opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthExpectationClass {
    /// No account needed to browse / read.
    NoAccountNeeded,
    /// An official Aureline account is required.
    OfficialAccountRequired,
    /// A community-platform account is typically required to post.
    CommunityAccountTypical,
    /// A security-channel credential / verified channel is required.
    SecurityChannelCredential,
    /// No network and no account: the draft stays local.
    LocalNoNetwork,
}

impl AuthExpectationClass {
    /// Stable token recorded on the sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAccountNeeded => "no_account_needed",
            Self::OfficialAccountRequired => "official_account_required",
            Self::CommunityAccountTypical => "community_account_typical",
            Self::SecurityChannelCredential => "security_channel_credential",
            Self::LocalNoNetwork => "local_no_network",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoAccountNeeded => "No account needed",
            Self::OfficialAccountRequired => "Official account required",
            Self::CommunityAccountTypical => "Community account typical",
            Self::SecurityChannelCredential => "Security channel credential",
            Self::LocalNoNetwork => "Local, no network",
        }
    }

    /// Whether this auth expectation is consistent with the trust class.
    pub fn allowed_for_trust(self, trust: DestinationTrustClass) -> bool {
        use DestinationTrustClass as T;
        match trust {
            T::OfficialPublic | T::Community => {
                matches!(self, Self::NoAccountNeeded | Self::CommunityAccountTypical)
            }
            T::OfficialAuthenticated => matches!(self, Self::OfficialAccountRequired),
            T::PrivateSecurity => {
                matches!(
                    self,
                    Self::SecurityChannelCredential | Self::NoAccountNeeded
                )
            }
            T::LocalOnly => matches!(self, Self::LocalNoNetwork),
        }
    }
}

/// Whether a route is a guaranteed product commitment or a best-effort path, so
/// support and community links never masquerade as guarantees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentClass {
    /// An official, supported commitment from the project / vendor.
    OfficialSupportedCommitment,
    /// Best-effort help from the community; no guarantee.
    BestEffortCommunity,
    /// A public forum with no commitment of a response.
    NoCommitmentPublicForum,
    /// Handled privately by the security process.
    SecurityHandledPrivately,
    /// A local draft that is never delivered until the user acts.
    LocalDraftNoDelivery,
}

impl CommitmentClass {
    /// Stable token recorded on the sheet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialSupportedCommitment => "official_supported_commitment",
            Self::BestEffortCommunity => "best_effort_community",
            Self::NoCommitmentPublicForum => "no_commitment_public_forum",
            Self::SecurityHandledPrivately => "security_handled_privately",
            Self::LocalDraftNoDelivery => "local_draft_no_delivery",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OfficialSupportedCommitment => "Official supported commitment",
            Self::BestEffortCommunity => "Best-effort community",
            Self::NoCommitmentPublicForum => "No commitment (public forum)",
            Self::SecurityHandledPrivately => "Security handled privately",
            Self::LocalDraftNoDelivery => "Local draft, no delivery",
        }
    }

    /// Whether this commitment class is consistent with the trust class.
    pub fn allowed_for_trust(self, trust: DestinationTrustClass) -> bool {
        use DestinationTrustClass as T;
        match trust {
            T::OfficialAuthenticated => matches!(self, Self::OfficialSupportedCommitment),
            T::OfficialPublic => {
                matches!(
                    self,
                    Self::NoCommitmentPublicForum | Self::BestEffortCommunity
                )
            }
            T::Community => {
                matches!(
                    self,
                    Self::BestEffortCommunity | Self::NoCommitmentPublicForum
                )
            }
            T::PrivateSecurity => matches!(self, Self::SecurityHandledPrivately),
            T::LocalOnly => matches!(self, Self::LocalDraftNoDelivery),
        }
    }

    /// True only for a class that represents a guaranteed product commitment.
    pub const fn is_guaranteed(self) -> bool {
        matches!(self, Self::OfficialSupportedCommitment)
    }
}

/// The commitment-honesty block stated on every sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentHonesty {
    /// Whether sharing through this route is a guaranteed product commitment.
    pub guaranteed_product_commitment: bool,
    /// The closed commitment class for this route.
    pub commitment_class: CommitmentClass,
    /// A bounded reviewable sentence stating the commitment honestly.
    pub honesty_note: String,
}

/// The exact anchor / object identity a handoff is about, so the report names a
/// precise object rather than a fuzzy description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectAnchor {
    /// Opaque ref of the originating anchor (surface, position, selection).
    pub anchor_ref: String,
    /// Opaque ref of the object the report is about.
    pub object_ref: String,
    /// Reviewer-facing anchor label.
    pub anchor_label: String,
}

/// Issue-template / structured-intake support, preserved where Aureline can hand
/// off richer context than a free-form message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueTemplateSupport {
    /// Opaque ref of the template the lane fills.
    pub template_ref: String,
    /// Reviewer-facing template label.
    pub template_label: String,
    /// Export class the template's body is redacted for.
    pub export_class: BuildContextExportClass,
    /// Whether the template carries structured fields beyond free text.
    pub carries_structured_fields: bool,
    /// A bounded reviewable sentence describing what the template carries.
    pub template_summary: String,
}

/// The local-safe fallback every sheet carries: a destination that never leaves
/// the product, so a blocked, offline, or unsupported-profile route degrades to
/// a labeled local path instead of dead-ending.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafeFallback {
    /// Opaque ref of the local fallback action.
    pub fallback_ref: String,
    /// Trust class of the fallback — always [`DestinationTrustClass::LocalOnly`].
    pub trust_class: DestinationTrustClass,
    /// Visibility boundary — always [`VisibilityBoundaryClass::LocalNeverLeaves`].
    pub visibility_boundary: VisibilityBoundaryClass,
    /// Auth expectation — always [`AuthExpectationClass::LocalNoNetwork`].
    pub auth_expectation: AuthExpectationClass,
    /// Data-exit boundary — always
    /// [`DataExitBoundary::NoPayloadLeavesProduct`].
    pub data_exit_boundary: DataExitBoundary,
    /// A bounded reviewable sentence describing the local fallback.
    pub fallback_summary: String,
}

/// One community-handoff target review sheet rendered before a report leaves the
/// product for one outbound route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityHandoffTargetSheet {
    /// Schema version for this sheet shape.
    pub community_handoff_target_sheet_schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable target id; prefixed `community_handoff_target:`.
    pub target_id: String,
    /// The outbound route this sheet describes.
    pub route_class: CommunityHandoffRouteClass,
    /// The destination trust class.
    pub trust_class: DestinationTrustClass,
    /// The visibility boundary that applies once the report leaves.
    pub visibility_boundary: VisibilityBoundaryClass,
    /// The authentication the user should expect.
    pub auth_expectation: AuthExpectationClass,
    /// The data-exit boundary the payload obeys.
    pub data_exit_boundary: DataExitBoundary,
    /// A bounded reviewable sentence naming who receives the data and the
    /// boundary that applies.
    pub data_exit_note: String,
    /// Opaque ref of the destination identity.
    pub destination_identity_ref: String,
    /// Reviewer-facing destination label.
    pub destination_label: String,
    /// Reviewer-facing label naming who will receive the data.
    pub recipient_label: String,
    /// The commitment-honesty block.
    pub commitment_honesty: CommitmentHonesty,
    /// The exact object anchor, when the lane can hand off richer context.
    pub object_anchor: Option<ObjectAnchor>,
    /// Issue-template / structured-intake support, when available.
    pub issue_template: Option<IssueTemplateSupport>,
    /// Versioned, redaction-safe build-context export blocks.
    #[serde(default)]
    pub build_context_exports: Vec<BuildContextExport>,
    /// Opaque refs of at least one safe fallback route.
    pub safe_fallback_refs: Vec<String>,
    /// The mandatory local-safe fallback.
    pub local_safe_fallback: LocalSafeFallback,
    /// Whether the route must be reviewed before it opens (true for
    /// world-readable routes).
    pub requires_prior_review_before_open: bool,
    /// Whether a critical alert may auto-open this route (always false for
    /// world-readable routes).
    pub auto_open_from_critical_alert_allowed: bool,
    /// Whether an unsupported-profile disclosure is required before exposing the
    /// route (true for private/security routes).
    pub unsupported_profile_disclosure_required: bool,
    /// Reviewer-facing headline label.
    pub headline_label: String,
    /// A bounded reviewable sentence summarizing the sheet.
    pub target_summary: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
    /// Optional reviewer note.
    pub notes: Option<String>,
}

impl CommunityHandoffTargetSheet {
    /// Validate the sheet against the community-handoff target contract.
    pub fn validate(&self) -> Result<(), CommunityHandoffTargetError> {
        if self.community_handoff_target_sheet_schema_version
            != COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION
        {
            return Err(CommunityHandoffTargetError::WrongSheetSchemaVersion {
                target_id: self.target_id.clone(),
                actual: self.community_handoff_target_sheet_schema_version,
            });
        }
        if self.record_kind != COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND {
            return Err(CommunityHandoffTargetError::WrongSheetRecordKind {
                target_id: self.target_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.target_id.starts_with("community_handoff_target:") {
            return Err(CommunityHandoffTargetError::MalformedTargetId {
                target_id: self.target_id.clone(),
            });
        }
        if self.contract_doc_ref != M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF {
            return Err(CommunityHandoffTargetError::WrongContractDocRef {
                record_id: self.target_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("target_summary", &self.target_summary),
            ("destination_label", &self.destination_label),
            ("recipient_label", &self.recipient_label),
            ("data_exit_note", &self.data_exit_note),
        ] {
            if non_empty(value).is_none() {
                return Err(CommunityHandoffTargetError::EmptyRequiredField {
                    record_id: self.target_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.destination_identity_ref) {
            return Err(CommunityHandoffTargetError::RawRefLeak {
                record_id: self.target_id.clone(),
                field: "destination_identity_ref",
            });
        }

        // The route may only target a trust class from its allowed set — no
        // accidental coercion into a public target.
        if !self.route_class.allows_trust(self.trust_class) {
            return Err(CommunityHandoffTargetError::RouteTrustMismatch {
                target_id: self.target_id.clone(),
                route: self.route_class,
                trust: self.trust_class,
            });
        }
        // Trust class pins the visibility boundary, auth expectation, and
        // data-exit boundary.
        if !self.visibility_boundary.allowed_for_trust(self.trust_class) {
            return Err(CommunityHandoffTargetError::TrustVisibilityMismatch {
                target_id: self.target_id.clone(),
                trust: self.trust_class,
                visibility: self.visibility_boundary,
            });
        }
        if !self.auth_expectation.allowed_for_trust(self.trust_class) {
            return Err(CommunityHandoffTargetError::TrustAuthMismatch {
                target_id: self.target_id.clone(),
                trust: self.trust_class,
                auth: self.auth_expectation,
            });
        }
        if !trust_allows_data_exit(self.trust_class, self.data_exit_boundary) {
            return Err(CommunityHandoffTargetError::TrustDataExitMismatch {
                target_id: self.target_id.clone(),
                trust: self.trust_class,
                data_exit: self.data_exit_boundary,
            });
        }

        // Commitment honesty: a guaranteed product commitment is only honest for
        // an official authenticated support route, and the commitment class must
        // be consistent with the trust class.
        if non_empty(&self.commitment_honesty.honesty_note).is_none() {
            return Err(CommunityHandoffTargetError::EmptyRequiredField {
                record_id: self.target_id.clone(),
                field: "commitment_honesty.honesty_note",
            });
        }
        if !self
            .commitment_honesty
            .commitment_class
            .allowed_for_trust(self.trust_class)
        {
            return Err(CommunityHandoffTargetError::CommitmentTrustMismatch {
                target_id: self.target_id.clone(),
                trust: self.trust_class,
                commitment: self.commitment_honesty.commitment_class,
            });
        }
        if self.commitment_honesty.guaranteed_product_commitment
            != self.commitment_honesty.commitment_class.is_guaranteed()
        {
            return Err(
                CommunityHandoffTargetError::CommitmentMasqueradesAsGuarantee {
                    target_id: self.target_id.clone(),
                },
            );
        }

        // Object anchor and issue-template refs stay opaque when present.
        if let Some(anchor) = &self.object_anchor {
            if !ref_is_opaque(&anchor.anchor_ref) || !ref_is_opaque(&anchor.object_ref) {
                return Err(CommunityHandoffTargetError::RawRefLeak {
                    record_id: self.target_id.clone(),
                    field: "object_anchor",
                });
            }
            if non_empty(&anchor.anchor_label).is_none() {
                return Err(CommunityHandoffTargetError::EmptyRequiredField {
                    record_id: self.target_id.clone(),
                    field: "object_anchor.anchor_label",
                });
            }
        }
        if let Some(template) = &self.issue_template {
            if !ref_is_opaque(&template.template_ref) {
                return Err(CommunityHandoffTargetError::RawRefLeak {
                    record_id: self.target_id.clone(),
                    field: "issue_template.template_ref",
                });
            }
            if non_empty(&template.template_label).is_none()
                || non_empty(&template.template_summary).is_none()
            {
                return Err(CommunityHandoffTargetError::EmptyRequiredField {
                    record_id: self.target_id.clone(),
                    field: "issue_template",
                });
            }
        }

        // Every lane attaches a versioned, redaction-safe build-context export.
        if self.build_context_exports.is_empty() {
            return Err(CommunityHandoffTargetError::MissingBuildContextExport {
                target_id: self.target_id.clone(),
            });
        }
        for export in &self.build_context_exports {
            if export.export_block_schema_version < 1 {
                return Err(
                    CommunityHandoffTargetError::BuildContextExportSchemaVersionInvalid {
                        target_id: self.target_id.clone(),
                        actual: export.export_block_schema_version,
                    },
                );
            }
            if !export.raw_screenshots_excluded || !export.raw_secrets_excluded {
                return Err(
                    CommunityHandoffTargetError::BuildContextExportNotRedactionSafe {
                        target_id: self.target_id.clone(),
                    },
                );
            }
            if non_empty(&export.export_block_ref).is_none()
                || non_empty(&export.export_summary).is_none()
            {
                return Err(CommunityHandoffTargetError::BuildContextExportFieldEmpty {
                    target_id: self.target_id.clone(),
                });
            }
        }

        // Every target offers at least one safe fallback so a blocked route
        // degrades to a labeled path instead of dead-ending.
        if self.safe_fallback_refs.is_empty() {
            return Err(CommunityHandoffTargetError::MissingSafeFallback {
                target_id: self.target_id.clone(),
            });
        }
        for fallback in &self.safe_fallback_refs {
            if !ref_is_opaque(fallback) {
                return Err(CommunityHandoffTargetError::RawRefLeak {
                    record_id: self.target_id.clone(),
                    field: "safe_fallback_refs",
                });
            }
        }

        // The local-safe fallback never leaves the product.
        self.local_safe_fallback.validate(&self.target_id)?;

        // Guardrail: a world-readable route must require prior review and may
        // never auto-open from a critical alert.
        if self.trust_class.is_world_readable() {
            if !self.requires_prior_review_before_open {
                return Err(CommunityHandoffTargetError::WorldReadableSkipsReview {
                    target_id: self.target_id.clone(),
                });
            }
            if self.auto_open_from_critical_alert_allowed {
                return Err(CommunityHandoffTargetError::WorldReadableAutoOpens {
                    target_id: self.target_id.clone(),
                });
            }
        }

        // Out-of-scope guardrail: a private/security route is exposed only with
        // an explicit unsupported-profile disclosure (and always with a
        // local-safe fallback, enforced above).
        if matches!(self.trust_class, DestinationTrustClass::PrivateSecurity)
            && !self.unsupported_profile_disclosure_required
        {
            return Err(CommunityHandoffTargetError::PrivateRouteMissingDisclosure {
                target_id: self.target_id.clone(),
            });
        }

        Ok(())
    }

    /// Render a deterministic plaintext block for support exports and
    /// reviewer-facing previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {} — route={} trust={} ({})\n",
            self.target_id,
            self.headline_label,
            self.route_class.as_str(),
            self.trust_class.as_str(),
            self.trust_class.label(),
        ));
        out.push_str(&format!(
            "    recipient: {} | visibility={} auth={} data_exit={}\n",
            self.recipient_label,
            self.visibility_boundary.as_str(),
            self.auth_expectation.as_str(),
            self.data_exit_boundary.as_str(),
        ));
        out.push_str(&format!("    data-exit note: {}\n", self.data_exit_note));
        out.push_str(&format!(
            "    commitment: {} (guaranteed={})\n",
            self.commitment_honesty.commitment_class.as_str(),
            self.commitment_honesty.guaranteed_product_commitment,
        ));
        if let Some(anchor) = &self.object_anchor {
            out.push_str(&format!(
                "    anchor: {} (object={})\n",
                anchor.anchor_ref, anchor.object_ref
            ));
        }
        if let Some(template) = &self.issue_template {
            out.push_str(&format!(
                "    issue template: {} ({})\n",
                template.template_ref,
                template.export_class.as_str()
            ));
        }
        for fallback in &self.safe_fallback_refs {
            out.push_str(&format!("    safe fallback: {fallback}\n"));
        }
        out.push_str(&format!(
            "    local-safe fallback: {} ({})\n",
            self.local_safe_fallback.fallback_ref,
            self.local_safe_fallback.trust_class.as_str(),
        ));
        out
    }
}

impl LocalSafeFallback {
    fn validate(&self, target_id: &str) -> Result<(), CommunityHandoffTargetError> {
        if !ref_is_opaque(&self.fallback_ref) {
            return Err(CommunityHandoffTargetError::RawRefLeak {
                record_id: target_id.to_owned(),
                field: "local_safe_fallback.fallback_ref",
            });
        }
        if non_empty(&self.fallback_summary).is_none() {
            return Err(CommunityHandoffTargetError::EmptyRequiredField {
                record_id: target_id.to_owned(),
                field: "local_safe_fallback.fallback_summary",
            });
        }
        if self.trust_class != DestinationTrustClass::LocalOnly
            || self.visibility_boundary != VisibilityBoundaryClass::LocalNeverLeaves
            || self.auth_expectation != AuthExpectationClass::LocalNoNetwork
            || self.data_exit_boundary != DataExitBoundary::NoPayloadLeavesProduct
        {
            return Err(CommunityHandoffTargetError::LocalFallbackNotLocal {
                target_id: target_id.to_owned(),
            });
        }
        Ok(())
    }
}

/// A bundled set of community-handoff target review sheets, one per governed
/// route, checked in as the canonical M5 source for outbound-route truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommunityHandoffTargetSheetSet {
    /// Schema version for the sheet-set shape.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable id for the sheet set.
    pub sheet_set_id: String,
    /// Reviewer-facing label for the sheet set.
    pub sheet_set_label: String,
    /// One sheet per governed route.
    pub sheets: Vec<CommunityHandoffTargetSheet>,
    /// Source contracts this set binds to by id.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token covering the export boundary.
    pub redaction_class_token: String,
    /// Opaque mint timestamp ref.
    pub minted_at: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
}

impl M5CommunityHandoffTargetSheetSet {
    /// Validate the sheet set: every sheet validates, every route and trust
    /// class is represented, official and community routes stay distinguishable,
    /// no two sheets share a target id, and the source contracts are present.
    pub fn validate(&self) -> Result<(), CommunityHandoffTargetError> {
        if self.schema_version != M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_SCHEMA_VERSION {
            return Err(CommunityHandoffTargetError::WrongSetSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_RECORD_KIND {
            return Err(CommunityHandoffTargetError::WrongSetRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if non_empty(&self.sheet_set_id).is_none()
            || non_empty(&self.sheet_set_label).is_none()
            || non_empty(&self.redaction_class_token).is_none()
            || non_empty(&self.minted_at).is_none()
        {
            return Err(CommunityHandoffTargetError::SetIdentityIncomplete);
        }
        if self.contract_doc_ref != M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF {
            return Err(CommunityHandoffTargetError::WrongContractDocRef {
                record_id: self.sheet_set_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for sheet in &self.sheets {
            sheet.validate()?;
            if !seen.insert(sheet.target_id.as_str()) {
                return Err(CommunityHandoffTargetError::DuplicateTargetId {
                    target_id: sheet.target_id.clone(),
                });
            }
        }

        // Every governed route is named exactly once.
        for route in CommunityHandoffRouteClass::ALL {
            if !self.sheets.iter().any(|s| s.route_class == route) {
                return Err(CommunityHandoffTargetError::RouteMissing { route });
            }
        }

        // Every trust class is carried by some sheet, either as a primary
        // destination or as the local-safe fallback.
        for trust in DestinationTrustClass::ALL {
            let primary = self.sheets.iter().any(|s| s.trust_class == trust);
            let fallback = self
                .sheets
                .iter()
                .any(|s| s.local_safe_fallback.trust_class == trust);
            if !primary && !fallback {
                return Err(CommunityHandoffTargetError::TrustClassMissing { trust });
            }
        }

        // Official and community routes stay distinguishable: at least one of
        // each must be present.
        if !self.sheets.iter().any(|s| s.trust_class.is_official()) {
            return Err(CommunityHandoffTargetError::OfficialRouteMissing);
        }
        if !self.sheets.iter().any(|s| s.trust_class.is_community()) {
            return Err(CommunityHandoffTargetError::CommunityRouteMissing);
        }

        // Source contracts bound by id.
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            M5_COMMUNITY_HANDOFF_TARGET_SCHEMA_REF,
            M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF,
            M5_COMMUNITY_HANDOFF_PACKET_CONTRACT_REF,
            M5_COMMUNITY_HANDOFF_TARGET_REVIEW_BASE_REF,
            M5_COMMUNITY_HANDOFF_PUBLIC_MATRIX_REF,
        ] {
            if !refs.contains(required) {
                return Err(CommunityHandoffTargetError::MissingSourceContracts);
            }
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("community-handoff sheet set serializes"),
        ) {
            return Err(CommunityHandoffTargetError::RawMaterialInExport);
        }

        Ok(())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("community-handoff sheet set serializes")
    }

    /// Deterministic, machine-readable CSV: one row per governed route, naming
    /// its trust class, visibility boundary, auth expectation, data-exit
    /// boundary, commitment class, and whether it is a guaranteed commitment.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "route,trust_class,visibility_boundary,auth_expectation,data_exit_boundary,commitment_class,guaranteed_commitment,requires_prior_review\n",
        );
        for sheet in &self.sheets {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                sheet.route_class.as_str(),
                sheet.trust_class.as_str(),
                sheet.visibility_boundary.as_str(),
                sheet.auth_expectation.as_str(),
                sheet.data_exit_boundary.as_str(),
                sheet.commitment_honesty.commitment_class.as_str(),
                sheet.commitment_honesty.guaranteed_product_commitment,
                sheet.requires_prior_review_before_open,
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 community-handoff target review\n\n");
        out.push_str(&format!("Sheet set: `{}`\n\n", self.sheet_set_id));
        out.push_str(
            "| Route | Trust class | Visibility | Auth | Data exit | Commitment | Guaranteed? |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for sheet in &self.sheets {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | `{}` | {} | {} |\n",
                sheet.route_class.as_str(),
                sheet.trust_class.label(),
                sheet.visibility_boundary.label(),
                sheet.auth_expectation.label(),
                sheet.data_exit_boundary.as_str(),
                sheet.commitment_honesty.commitment_class.label(),
                sheet.commitment_honesty.guaranteed_product_commitment,
            ));
        }
        out.push('\n');
        out.push_str("Every route carries a local-safe fallback that never leaves the product, ");
        out.push_str("and world-readable routes require prior review and never auto-open from a critical alert.\n");
        out
    }
}

/// Whether a trust class permits the given data-exit boundary.
fn trust_allows_data_exit(trust: DestinationTrustClass, data_exit: DataExitBoundary) -> bool {
    use DataExitBoundary as D;
    use DestinationTrustClass as T;
    match trust {
        T::OfficialPublic => matches!(
            data_exit,
            D::NoPayloadLeavesProduct
                | D::MetadataSafeObjectRefs
                | D::ProposalRefsOnly
                | D::ExternalPublicBrowse
        ),
        T::Community => matches!(
            data_exit,
            D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs | D::ProposalRefsOnly
        ),
        T::OfficialAuthenticated => matches!(
            data_exit,
            D::RedactedSupportPacket | D::MetadataSafeObjectRefs | D::NoPayloadLeavesProduct
        ),
        T::PrivateSecurity => matches!(data_exit, D::SecurityPayloadsOnly),
        T::LocalOnly => matches!(data_exit, D::NoPayloadLeavesProduct),
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
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Closed validation-error vocabulary for the community-handoff target contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunityHandoffTargetError {
    WrongSheetSchemaVersion {
        target_id: String,
        actual: u32,
    },
    WrongSheetRecordKind {
        target_id: String,
        actual: String,
    },
    MalformedTargetId {
        target_id: String,
    },
    RouteTrustMismatch {
        target_id: String,
        route: CommunityHandoffRouteClass,
        trust: DestinationTrustClass,
    },
    TrustVisibilityMismatch {
        target_id: String,
        trust: DestinationTrustClass,
        visibility: VisibilityBoundaryClass,
    },
    TrustAuthMismatch {
        target_id: String,
        trust: DestinationTrustClass,
        auth: AuthExpectationClass,
    },
    TrustDataExitMismatch {
        target_id: String,
        trust: DestinationTrustClass,
        data_exit: DataExitBoundary,
    },
    CommitmentTrustMismatch {
        target_id: String,
        trust: DestinationTrustClass,
        commitment: CommitmentClass,
    },
    CommitmentMasqueradesAsGuarantee {
        target_id: String,
    },
    MissingBuildContextExport {
        target_id: String,
    },
    BuildContextExportSchemaVersionInvalid {
        target_id: String,
        actual: u32,
    },
    BuildContextExportNotRedactionSafe {
        target_id: String,
    },
    BuildContextExportFieldEmpty {
        target_id: String,
    },
    MissingSafeFallback {
        target_id: String,
    },
    LocalFallbackNotLocal {
        target_id: String,
    },
    WorldReadableSkipsReview {
        target_id: String,
    },
    WorldReadableAutoOpens {
        target_id: String,
    },
    PrivateRouteMissingDisclosure {
        target_id: String,
    },
    WrongSetSchemaVersion {
        actual: u32,
    },
    WrongSetRecordKind {
        actual: String,
    },
    SetIdentityIncomplete,
    DuplicateTargetId {
        target_id: String,
    },
    RouteMissing {
        route: CommunityHandoffRouteClass,
    },
    TrustClassMissing {
        trust: DestinationTrustClass,
    },
    OfficialRouteMissing,
    CommunityRouteMissing,
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

impl fmt::Display for CommunityHandoffTargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSheetSchemaVersion { target_id, actual } => write!(
                f,
                "target {target_id} has unsupported community_handoff_target_sheet_schema_version {actual}"
            ),
            Self::WrongSheetRecordKind { target_id, actual } => {
                write!(f, "target {target_id} has unsupported record kind {actual}")
            }
            Self::MalformedTargetId { target_id } => write!(
                f,
                "target id {target_id} must start with community_handoff_target:"
            ),
            Self::RouteTrustMismatch {
                target_id,
                route,
                trust,
            } => write!(
                f,
                "target {target_id} route {} cannot target trust class {}",
                route.as_str(),
                trust.as_str()
            ),
            Self::TrustVisibilityMismatch {
                target_id,
                trust,
                visibility,
            } => write!(
                f,
                "target {target_id} trust class {} cannot use visibility boundary {}",
                trust.as_str(),
                visibility.as_str()
            ),
            Self::TrustAuthMismatch {
                target_id,
                trust,
                auth,
            } => write!(
                f,
                "target {target_id} trust class {} cannot use auth expectation {}",
                trust.as_str(),
                auth.as_str()
            ),
            Self::TrustDataExitMismatch {
                target_id,
                trust,
                data_exit,
            } => write!(
                f,
                "target {target_id} trust class {} cannot use data exit {}",
                trust.as_str(),
                data_exit.as_str()
            ),
            Self::CommitmentTrustMismatch {
                target_id,
                trust,
                commitment,
            } => write!(
                f,
                "target {target_id} commitment class {} is not consistent with trust class {}",
                commitment.as_str(),
                trust.as_str()
            ),
            Self::CommitmentMasqueradesAsGuarantee { target_id } => write!(
                f,
                "target {target_id} guaranteed_product_commitment disagrees with its commitment class"
            ),
            Self::MissingBuildContextExport { target_id } => write!(
                f,
                "target {target_id} must attach a build-context export block"
            ),
            Self::BuildContextExportSchemaVersionInvalid { target_id, actual } => write!(
                f,
                "target {target_id} has invalid build-context export schema version {actual}"
            ),
            Self::BuildContextExportNotRedactionSafe { target_id } => write!(
                f,
                "target {target_id} build-context export is not redaction safe"
            ),
            Self::BuildContextExportFieldEmpty { target_id } => {
                write!(f, "target {target_id} has an empty build-context export field")
            }
            Self::MissingSafeFallback { target_id } => write!(
                f,
                "target {target_id} must offer at least one safe fallback route"
            ),
            Self::LocalFallbackNotLocal { target_id } => write!(
                f,
                "target {target_id} local-safe fallback must stay local and never leave the product"
            ),
            Self::WorldReadableSkipsReview { target_id } => write!(
                f,
                "target {target_id} is world-readable and must require prior review before opening"
            ),
            Self::WorldReadableAutoOpens { target_id } => write!(
                f,
                "target {target_id} is world-readable and must not auto-open from a critical alert"
            ),
            Self::PrivateRouteMissingDisclosure { target_id } => write!(
                f,
                "target {target_id} is a private/security route and must require an unsupported-profile disclosure"
            ),
            Self::WrongSetSchemaVersion { actual } => write!(
                f,
                "sheet set has unsupported schema_version {actual}"
            ),
            Self::WrongSetRecordKind { actual } => {
                write!(f, "sheet set has unsupported record kind {actual}")
            }
            Self::SetIdentityIncomplete => write!(f, "sheet set is missing required identity fields"),
            Self::DuplicateTargetId { target_id } => {
                write!(f, "sheet set has duplicate target id {target_id}")
            }
            Self::RouteMissing { route } => {
                write!(f, "sheet set is missing route {}", route.as_str())
            }
            Self::TrustClassMissing { trust } => {
                write!(f, "sheet set never carries trust class {}", trust.as_str())
            }
            Self::OfficialRouteMissing => {
                write!(f, "sheet set must carry at least one official route")
            }
            Self::CommunityRouteMissing => {
                write!(f, "sheet set must carry at least one community route")
            }
            Self::MissingSourceContracts => {
                write!(f, "sheet set is missing a required source contract ref")
            }
            Self::RawMaterialInExport => {
                write!(f, "sheet set export carries forbidden raw material")
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

impl Error for CommunityHandoffTargetError {}

/// Reads and validates the checked-in stable community-handoff target sheet set.
pub fn current_stable_m5_community_handoff_target_set(
) -> Result<M5CommunityHandoffTargetSheetSet, Box<dyn Error>> {
    let set: M5CommunityHandoffTargetSheetSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-community-handoff-proof/target_set.json"
    )))?;
    set.validate()?;
    Ok(set)
}
