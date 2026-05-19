//! Public about/source/community-handoff/open-vs-managed boundary and
//! upgrade-honesty surfaces with account-optional local-path parity.
//!
//! This module is the shell-side projection that About, Help, marketplace,
//! issue/reporting, governance, contributing, community-discussion, release-
//! notes, upgrade-or-hosted, sponsorship, troubleshooting, and source-
//! repository surfaces read so they answer "where does official source,
//! issue, discussion, governance, funding/upgrade, and local-only fallback
//! live?" with the same destination vocabulary and the same open-vs-managed
//! story.
//!
//! Two record families are minted here:
//!
//! - [`AboutDestinationRecord`] — one labeled destination row carried by
//!   About, Help, source-repository, issue-reporting, governance, and
//!   community-handoff surfaces. Each row pins a typed
//!   [`DestinationClass`] (official_public / official_private / community /
//!   third_party_vendor), a typed [`DestinationRole`], a typed
//!   [`RouteState`], a typed [`AccountRequirement`], a typed
//!   [`DataExitBoundary`], a typed [`SupportProminence`], and a typed
//!   [`LocalOnlyParity`]. Redirected, archived, replaced, or decommissioned
//!   destinations MUST cite a replacement or local-only fallback so dead
//!   links degrade to a labeled successor instead of failing silently.
//!
//! - [`CapabilityBoundaryCardRecord`] — one boundary card per consuming
//!   surface stating posture (local_open, local_open_account_optional,
//!   managed_first_party, self_hosted_customer_operated, mirrored_offline,
//!   premium_hosted, third_party_vendor, community_operated), identity and
//!   network requirement, data boundary, continue-local-only or rollback
//!   path, and the upgrade-honesty rule. premium_hosted and
//!   managed_first_party surfaces MUST NOT carry
//!   [`UpgradeHonestyRule::LocalPathHiddenViolation`] — the validator
//!   denies the row so an upgrade prompt cannot push a valid local/open
//!   path off the surface.
//!
//! A page record [`AboutAndBoundaryTruthPage`] bundles the destinations and
//! boundary cards rendered on the same view, plus a cross-validator that
//! enforces the contract joins: every linked destination ref on a boundary
//! card resolves to a destination on the same page, support-oriented
//! surfaces keep support routes ahead of upgrade CTAs, account-required
//! write/subscribe rows keep an account-optional local fallback, and
//! premium/hosted surfaces never hide a valid local/open path.
//!
//! Raw URLs, raw email addresses, raw chat-room URLs, raw on-call rotation
//! entries, and raw secret material MUST NOT appear; the records carry
//! opaque refs and bounded reviewable summaries only.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AboutDestinationRecord`].
pub const ABOUT_DESTINATION_RECORD_KIND: &str = "about_destination_record";

/// Stable record-kind tag carried by [`CapabilityBoundaryCardRecord`].
pub const CAPABILITY_BOUNDARY_CARD_RECORD_KIND: &str = "capability_boundary_card_record";

/// Stable record-kind tag carried by [`AboutAndBoundaryTruthPage`].
pub const ABOUT_AND_BOUNDARY_TRUTH_PAGE_RECORD_KIND: &str = "about_and_boundary_truth_page_record";

/// Schema version for the [`AboutDestinationRecord`] payload shape.
pub const ABOUT_DESTINATION_SCHEMA_VERSION: u32 = 1;

/// Schema version for the [`CapabilityBoundaryCardRecord`] payload shape.
pub const CAPABILITY_BOUNDARY_CARD_SCHEMA_VERSION: u32 = 1;

/// Schema version for the [`AboutAndBoundaryTruthPage`] payload shape.
pub const ABOUT_AND_BOUNDARY_TRUTH_PAGE_SCHEMA_VERSION: u32 = 1;

/// Frozen reference to the beta contract doc both schemas point at.
pub const ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF: &str =
    "docs/public/m3/about_source_and_boundary_beta.md";

/// Closed four-class destination-trust vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationClass {
    OfficialPublic,
    OfficialPrivate,
    Community,
    ThirdPartyVendor,
}

impl DestinationClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialPrivate => "official_private",
            Self::Community => "community",
            Self::ThirdPartyVendor => "third_party_vendor",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::OfficialPublic => "Official public",
            Self::OfficialPrivate => "Official private",
            Self::Community => "Community",
            Self::ThirdPartyVendor => "Third-party / vendor",
        }
    }
}

/// Closed destination-role vocabulary shared across About, Help,
/// marketplace, issue/reporting, governance, contributing, release-notes,
/// upgrade-or-hosted, and community-handoff surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationRole {
    SourceRepository,
    IssueTracker,
    DiscussionForum,
    RfcForum,
    GovernanceCharter,
    ContributingGuide,
    SecurityIntake,
    SupportIntake,
    StatusPage,
    DocsOrHelp,
    ReleaseNotes,
    ReleasePacket,
    MarketplaceIndex,
    UpgradeOrHosted,
    SponsorshipOrFunding,
    CommunityHandoffRouter,
    LocalOnlyFallback,
    MirrorOrArchive,
}

impl DestinationRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceRepository => "source_repository",
            Self::IssueTracker => "issue_tracker",
            Self::DiscussionForum => "discussion_forum",
            Self::RfcForum => "rfc_forum",
            Self::GovernanceCharter => "governance_charter",
            Self::ContributingGuide => "contributing_guide",
            Self::SecurityIntake => "security_intake",
            Self::SupportIntake => "support_intake",
            Self::StatusPage => "status_page",
            Self::DocsOrHelp => "docs_or_help",
            Self::ReleaseNotes => "release_notes",
            Self::ReleasePacket => "release_packet",
            Self::MarketplaceIndex => "marketplace_index",
            Self::UpgradeOrHosted => "upgrade_or_hosted",
            Self::SponsorshipOrFunding => "sponsorship_or_funding",
            Self::CommunityHandoffRouter => "community_handoff_router",
            Self::LocalOnlyFallback => "local_only_fallback",
            Self::MirrorOrArchive => "mirror_or_archive",
        }
    }

    /// True when the role is a support-oriented lane that MUST NOT be
    /// deprioritized below an upgrade or sponsorship CTA.
    pub const fn is_support_oriented(self) -> bool {
        matches!(
            self,
            Self::IssueTracker
                | Self::SupportIntake
                | Self::SecurityIntake
                | Self::SourceRepository
                | Self::ContributingGuide
                | Self::StatusPage
        )
    }

    /// True when the role requires a build-context export block so the
    /// lane can attach a versioned export instead of a screenshot.
    pub const fn requires_build_context_export(self) -> bool {
        matches!(
            self,
            Self::IssueTracker
                | Self::SupportIntake
                | Self::SecurityIntake
                | Self::CommunityHandoffRouter
                | Self::DiscussionForum
        )
    }
}

/// Closed route-state vocabulary disclosed on every destination row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteState {
    Current,
    Redirected,
    Archived,
    Replaced,
    Decommissioned,
    UnreachableProbed,
}

impl RouteState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Redirected => "redirected",
            Self::Archived => "archived",
            Self::Replaced => "replaced",
            Self::Decommissioned => "decommissioned",
            Self::UnreachableProbed => "unreachable_probed",
        }
    }

    pub const fn requires_replacement(self) -> bool {
        matches!(self, Self::Redirected | Self::Archived | Self::Replaced)
    }

    pub const fn requires_local_fallback(self) -> bool {
        matches!(self, Self::Decommissioned)
    }
}

/// Closed account-requirement vocabulary disclosed before navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountRequirement {
    None,
    OptionalForAccountFeatures,
    RequiredForView,
    RequiredForWrite,
    RequiredForSubscribe,
    RequiredForPremiumHosted,
}

impl AccountRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OptionalForAccountFeatures => "optional_for_account_features",
            Self::RequiredForView => "required_for_view",
            Self::RequiredForWrite => "required_for_write",
            Self::RequiredForSubscribe => "required_for_subscribe",
            Self::RequiredForPremiumHosted => "required_for_premium_hosted",
        }
    }

    pub const fn coerces_account(self) -> bool {
        matches!(
            self,
            Self::RequiredForWrite | Self::RequiredForSubscribe | Self::RequiredForPremiumHosted
        )
    }
}

/// Closed data-exit-boundary vocabulary disclosed before navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataExitBoundary {
    NoPayloadLeavesProduct,
    MetadataSafeObjectRefs,
    ProposalRefsOnly,
    RedactedSupportPacket,
    SecurityPayloadsOnly,
    ExternalPublicBrowse,
    VendorOrThirdPartyOutbound,
}

impl DataExitBoundary {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPayloadLeavesProduct => "no_payload_leaves_product",
            Self::MetadataSafeObjectRefs => "metadata_safe_object_refs",
            Self::ProposalRefsOnly => "proposal_refs_only",
            Self::RedactedSupportPacket => "redacted_support_packet",
            Self::SecurityPayloadsOnly => "security_payloads_only",
            Self::ExternalPublicBrowse => "external_public_browse",
            Self::VendorOrThirdPartyOutbound => "vendor_or_third_party_outbound",
        }
    }
}

/// Closed support-prominence vocabulary shared by destinations and cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportProminence {
    TroubleshootingFirst,
    SupportFirst,
    SourceFirst,
    ParityWithUpgrade,
    BelowUpgrade,
}

impl SupportProminence {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TroubleshootingFirst => "troubleshooting_first",
            Self::SupportFirst => "support_first",
            Self::SourceFirst => "source_first",
            Self::ParityWithUpgrade => "parity_with_upgrade",
            Self::BelowUpgrade => "below_upgrade",
        }
    }

    /// True when the prominence ranks support or source ahead of upgrade.
    pub const fn ranks_support_above_upgrade(self) -> bool {
        matches!(
            self,
            Self::TroubleshootingFirst | Self::SupportFirst | Self::SourceFirst
        )
    }
}

/// Closed local-only parity vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalOnlyParity {
    AccountOptionalLocalParity,
    LocalOnlyOnly,
    HostedOnlyNoLocalFallback,
    MixedLocalOptionalAccount,
}

impl LocalOnlyParity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountOptionalLocalParity => "account_optional_local_parity",
            Self::LocalOnlyOnly => "local_only_only",
            Self::HostedOnlyNoLocalFallback => "hosted_only_no_local_fallback",
            Self::MixedLocalOptionalAccount => "mixed_local_optional_account",
        }
    }

    pub const fn requires_local_fallback_when_account_coerces(self) -> bool {
        matches!(
            self,
            Self::AccountOptionalLocalParity | Self::MixedLocalOptionalAccount
        )
    }
}

/// Closed build-context export-block vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildContextExportClass {
    PublicIssueTemplateBlock,
    PrivateSupportIntakeBlock,
    PrivateSecurityIntakeBlock,
    CommunityDiscussionBlock,
}

impl BuildContextExportClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicIssueTemplateBlock => "public_issue_template_block",
            Self::PrivateSupportIntakeBlock => "private_support_intake_block",
            Self::PrivateSecurityIntakeBlock => "private_security_intake_block",
            Self::CommunityDiscussionBlock => "community_discussion_block",
        }
    }
}

/// One build-context export block attached to a destination row so the
/// lane can carry a versioned export instead of a screenshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildContextExport {
    pub export_class: BuildContextExportClass,
    pub export_block_ref: String,
    pub export_block_schema_version: u32,
    pub redacted_for_audience: BuildContextExportClass,
    #[serde(default = "always_true")]
    pub raw_screenshots_excluded: bool,
    #[serde(default = "always_true")]
    pub raw_secrets_excluded: bool,
    pub export_summary: String,
}

fn always_true() -> bool {
    true
}

/// One destination row carried by About, Help, marketplace, issue/
/// reporting, governance, contributing, community-handoff, release-notes,
/// upgrade-or-hosted, sponsorship, troubleshooting, and source-repository
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AboutDestinationRecord {
    pub about_destination_schema_version: u32,
    pub record_kind: String,
    pub destination_id: String,
    pub destination_class: DestinationClass,
    pub destination_role_class: DestinationRole,
    pub route_state_class: RouteState,
    pub account_requirement_class: AccountRequirement,
    pub data_exit_boundary_class: DataExitBoundary,
    pub support_prominence_class: SupportProminence,
    pub local_only_parity_class: LocalOnlyParity,
    pub headline_label: String,
    pub destination_summary: String,
    pub replacement_destination_ref: Option<String>,
    pub local_only_fallback_ref: Option<String>,
    pub source_surface_refs: Vec<String>,
    #[serde(default)]
    pub build_context_exports: Vec<BuildContextExport>,
    #[serde(default)]
    pub issue_template_refs: Vec<String>,
    pub contract_doc_ref: String,
    pub notes: Option<String>,
}

impl AboutDestinationRecord {
    /// Stable token for fixture and review output.
    pub fn destination_id_token(&self) -> &str {
        &self.destination_id
    }

    /// Validate the row against the boundary contract.
    pub fn validate(&self) -> Result<(), AboutAndBoundaryValidationError> {
        if self.about_destination_schema_version != ABOUT_DESTINATION_SCHEMA_VERSION {
            return Err(
                AboutAndBoundaryValidationError::WrongAboutDestinationSchemaVersion {
                    destination_id: self.destination_id.clone(),
                    actual: self.about_destination_schema_version,
                },
            );
        }
        if self.record_kind != ABOUT_DESTINATION_RECORD_KIND {
            return Err(
                AboutAndBoundaryValidationError::WrongAboutDestinationRecordKind {
                    destination_id: self.destination_id.clone(),
                    actual: self.record_kind.clone(),
                },
            );
        }
        if !self.destination_id.starts_with("about_destination:") {
            return Err(AboutAndBoundaryValidationError::MalformedDestinationId {
                destination_id: self.destination_id.clone(),
            });
        }
        if self.contract_doc_ref != ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AboutAndBoundaryValidationError::WrongContractDocRef {
                destination_id: self.destination_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        if non_empty(&self.headline_label).is_none() {
            return Err(AboutAndBoundaryValidationError::EmptyRequiredField {
                destination_id: self.destination_id.clone(),
                field: "headline_label",
            });
        }
        if non_empty(&self.destination_summary).is_none() {
            return Err(AboutAndBoundaryValidationError::EmptyRequiredField {
                destination_id: self.destination_id.clone(),
                field: "destination_summary",
            });
        }
        if self.source_surface_refs.is_empty() {
            return Err(AboutAndBoundaryValidationError::MissingSourceSurfaceRefs {
                destination_id: self.destination_id.clone(),
            });
        }

        if self.route_state_class.requires_replacement()
            && self.replacement_destination_ref.is_none()
        {
            return Err(
                AboutAndBoundaryValidationError::DeadDestinationMissingReplacement {
                    destination_id: self.destination_id.clone(),
                    route_state: self.route_state_class,
                },
            );
        }
        if self.route_state_class.requires_local_fallback()
            && self.local_only_fallback_ref.is_none()
        {
            return Err(
                AboutAndBoundaryValidationError::DecommissionedRouteMissingLocalFallback {
                    destination_id: self.destination_id.clone(),
                },
            );
        }

        if matches!(self.destination_class, DestinationClass::ThirdPartyVendor)
            && !matches!(
                self.data_exit_boundary_class,
                DataExitBoundary::ExternalPublicBrowse
                    | DataExitBoundary::VendorOrThirdPartyOutbound
            )
        {
            return Err(
                AboutAndBoundaryValidationError::ThirdPartyVendorWithUnsupportedDataExit {
                    destination_id: self.destination_id.clone(),
                    data_exit: self.data_exit_boundary_class,
                },
            );
        }

        if matches!(self.destination_role_class, DestinationRole::SecurityIntake)
            && self.data_exit_boundary_class != DataExitBoundary::SecurityPayloadsOnly
        {
            return Err(
                AboutAndBoundaryValidationError::SecurityIntakeWithWrongDataExit {
                    destination_id: self.destination_id.clone(),
                    data_exit: self.data_exit_boundary_class,
                },
            );
        }
        if matches!(self.destination_role_class, DestinationRole::SupportIntake)
            && (self.data_exit_boundary_class != DataExitBoundary::RedactedSupportPacket
                || self.destination_class != DestinationClass::OfficialPrivate)
        {
            return Err(AboutAndBoundaryValidationError::SupportIntakeMislabeled {
                destination_id: self.destination_id.clone(),
            });
        }

        if self.destination_role_class.is_support_oriented()
            && !self.support_prominence_class.ranks_support_above_upgrade()
        {
            return Err(
                AboutAndBoundaryValidationError::SupportRouteDeprioritizedBelowUpgrade {
                    destination_id: self.destination_id.clone(),
                    role: self.destination_role_class,
                    prominence: self.support_prominence_class,
                },
            );
        }

        if matches!(
            self.destination_role_class,
            DestinationRole::UpgradeOrHosted | DestinationRole::SponsorshipOrFunding
        ) && self.support_prominence_class.ranks_support_above_upgrade()
        {
            return Err(
                AboutAndBoundaryValidationError::UpgradeRouteOutranksSupport {
                    destination_id: self.destination_id.clone(),
                    role: self.destination_role_class,
                    prominence: self.support_prominence_class,
                },
            );
        }

        if self.account_requirement_class.coerces_account()
            && self
                .local_only_parity_class
                .requires_local_fallback_when_account_coerces()
            && self.local_only_fallback_ref.is_none()
        {
            return Err(
                AboutAndBoundaryValidationError::AccountCoercingRouteMissingLocalFallback {
                    destination_id: self.destination_id.clone(),
                    account_requirement: self.account_requirement_class,
                    parity: self.local_only_parity_class,
                },
            );
        }

        if matches!(
            self.destination_role_class,
            DestinationRole::LocalOnlyFallback
        ) {
            if self.account_requirement_class != AccountRequirement::None {
                return Err(
                    AboutAndBoundaryValidationError::LocalOnlyFallbackCoercesAccount {
                        destination_id: self.destination_id.clone(),
                        account_requirement: self.account_requirement_class,
                    },
                );
            }
            if !self.support_prominence_class.ranks_support_above_upgrade() {
                return Err(
                    AboutAndBoundaryValidationError::LocalOnlyFallbackDeprioritized {
                        destination_id: self.destination_id.clone(),
                        prominence: self.support_prominence_class,
                    },
                );
            }
        }

        if self.destination_role_class.requires_build_context_export()
            && self.build_context_exports.is_empty()
        {
            return Err(
                AboutAndBoundaryValidationError::HandoffRouteMissingBuildContextExport {
                    destination_id: self.destination_id.clone(),
                    role: self.destination_role_class,
                },
            );
        }

        for export in &self.build_context_exports {
            if export.export_block_schema_version < 1 {
                return Err(
                    AboutAndBoundaryValidationError::BuildContextExportSchemaVersionInvalid {
                        destination_id: self.destination_id.clone(),
                        actual: export.export_block_schema_version,
                    },
                );
            }
            if !export.raw_screenshots_excluded || !export.raw_secrets_excluded {
                return Err(
                    AboutAndBoundaryValidationError::BuildContextExportNotRedactionSafe {
                        destination_id: self.destination_id.clone(),
                    },
                );
            }
            if non_empty(&export.export_block_ref).is_none()
                || non_empty(&export.export_summary).is_none()
            {
                return Err(
                    AboutAndBoundaryValidationError::BuildContextExportFieldEmpty {
                        destination_id: self.destination_id.clone(),
                    },
                );
            }
        }

        Ok(())
    }
}

/// Closed surface vocabulary mirrored from the boundary-card schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryCardSurface {
    AboutPane,
    HelpPane,
    SourceRepositoryPanel,
    IssueReportingPanel,
    DiscussionForumPanel,
    GovernancePanel,
    ContributingGuidePanel,
    MarketplacePanel,
    ReleaseNotesPanel,
    UpgradeOrHostedCta,
    SponsorshipCta,
    TroubleshootingPanel,
}

impl BoundaryCardSurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutPane => "about_pane",
            Self::HelpPane => "help_pane",
            Self::SourceRepositoryPanel => "source_repository_panel",
            Self::IssueReportingPanel => "issue_reporting_panel",
            Self::DiscussionForumPanel => "discussion_forum_panel",
            Self::GovernancePanel => "governance_panel",
            Self::ContributingGuidePanel => "contributing_guide_panel",
            Self::MarketplacePanel => "marketplace_panel",
            Self::ReleaseNotesPanel => "release_notes_panel",
            Self::UpgradeOrHostedCta => "upgrade_or_hosted_cta",
            Self::SponsorshipCta => "sponsorship_cta",
            Self::TroubleshootingPanel => "troubleshooting_panel",
        }
    }

    pub const fn is_support_oriented(self) -> bool {
        matches!(
            self,
            Self::IssueReportingPanel
                | Self::TroubleshootingPanel
                | Self::SourceRepositoryPanel
                | Self::ContributingGuidePanel
        )
    }

    pub const fn is_upgrade_cta(self) -> bool {
        matches!(self, Self::UpgradeOrHostedCta | Self::SponsorshipCta)
    }
}

/// Closed posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfacePosture {
    LocalOpen,
    LocalOpenAccountOptional,
    ManagedFirstParty,
    SelfHostedCustomerOperated,
    MirroredOffline,
    PremiumHosted,
    ThirdPartyVendor,
    CommunityOperated,
}

impl SurfacePosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOpen => "local_open",
            Self::LocalOpenAccountOptional => "local_open_account_optional",
            Self::ManagedFirstParty => "managed_first_party",
            Self::SelfHostedCustomerOperated => "self_hosted_customer_operated",
            Self::MirroredOffline => "mirrored_offline",
            Self::PremiumHosted => "premium_hosted",
            Self::ThirdPartyVendor => "third_party_vendor",
            Self::CommunityOperated => "community_operated",
        }
    }

    pub const fn is_local_open(self) -> bool {
        matches!(self, Self::LocalOpen | Self::LocalOpenAccountOptional)
    }

    pub const fn is_premium_or_managed(self) -> bool {
        matches!(self, Self::PremiumHosted | Self::ManagedFirstParty)
    }
}

/// Closed identity-requirement vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityRequirement {
    None,
    OptionalLocalAccount,
    RequiredAccountForWrite,
    RequiredAccountForSubscribe,
    RequiredSecurityIdentity,
    RequiredSupportIdentity,
    RequiredVendorIdentity,
}

impl IdentityRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OptionalLocalAccount => "optional_local_account",
            Self::RequiredAccountForWrite => "required_account_for_write",
            Self::RequiredAccountForSubscribe => "required_account_for_subscribe",
            Self::RequiredSecurityIdentity => "required_security_identity",
            Self::RequiredSupportIdentity => "required_support_identity",
            Self::RequiredVendorIdentity => "required_vendor_identity",
        }
    }

    pub const fn is_local_open_compatible(self) -> bool {
        matches!(self, Self::None | Self::OptionalLocalAccount)
    }
}

/// Closed network-requirement vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkRequirement {
    OfflineLocalOnly,
    AccountFreeMetadataOnly,
    AccountFreeBrowse,
    AuthenticatedManagedPlane,
    AuthenticatedPremiumPlane,
    VendorOrThirdPartyCall,
    CommunityPublicCall,
}

impl NetworkRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineLocalOnly => "offline_local_only",
            Self::AccountFreeMetadataOnly => "account_free_metadata_only",
            Self::AccountFreeBrowse => "account_free_browse",
            Self::AuthenticatedManagedPlane => "authenticated_managed_plane",
            Self::AuthenticatedPremiumPlane => "authenticated_premium_plane",
            Self::VendorOrThirdPartyCall => "vendor_or_third_party_call",
            Self::CommunityPublicCall => "community_public_call",
        }
    }

    pub const fn is_local_open_compatible(self) -> bool {
        matches!(
            self,
            Self::OfflineLocalOnly
                | Self::AccountFreeMetadataOnly
                | Self::AccountFreeBrowse
                | Self::CommunityPublicCall
        )
    }
}

/// Closed data-boundary vocabulary for boundary cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceDataBoundary {
    StaysOnDevice,
    MetadataOnlyOutbound,
    RedactedOutbound,
    AuthenticatedManagedOutbound,
    AuthenticatedPremiumOutbound,
    VendorOrThirdPartyOutbound,
    CommunityPublic,
}

impl SurfaceDataBoundary {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaysOnDevice => "stays_on_device",
            Self::MetadataOnlyOutbound => "metadata_only_outbound",
            Self::RedactedOutbound => "redacted_outbound",
            Self::AuthenticatedManagedOutbound => "authenticated_managed_outbound",
            Self::AuthenticatedPremiumOutbound => "authenticated_premium_outbound",
            Self::VendorOrThirdPartyOutbound => "vendor_or_third_party_outbound",
            Self::CommunityPublic => "community_public",
        }
    }

    pub const fn is_local_open_compatible(self) -> bool {
        matches!(
            self,
            Self::StaysOnDevice | Self::MetadataOnlyOutbound | Self::CommunityPublic
        )
    }
}

/// Closed rollback / downgrade path vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPath {
    ContinueLocalOnly,
    DowngradeToLocalOpen,
    SwitchToMirroredOffline,
    SwitchToSelfHostedCustomerOperated,
    NoLocalAlternativeDisclosed,
    NotApplicable,
}

impl RollbackPath {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueLocalOnly => "continue_local_only",
            Self::DowngradeToLocalOpen => "downgrade_to_local_open",
            Self::SwitchToMirroredOffline => "switch_to_mirrored_offline",
            Self::SwitchToSelfHostedCustomerOperated => "switch_to_self_hosted_customer_operated",
            Self::NoLocalAlternativeDisclosed => "no_local_alternative_disclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    pub const fn is_local_path(self) -> bool {
        matches!(
            self,
            Self::ContinueLocalOnly
                | Self::DowngradeToLocalOpen
                | Self::SwitchToMirroredOffline
                | Self::SwitchToSelfHostedCustomerOperated
        )
    }
}

/// Closed upgrade-honesty rule vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeHonestyRule {
    LocalPathVisible,
    LocalPathHiddenViolation,
    NoLocalPathApplicable,
}

impl UpgradeHonestyRule {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPathVisible => "local_path_visible",
            Self::LocalPathHiddenViolation => "local_path_hidden_violation",
            Self::NoLocalPathApplicable => "no_local_path_applicable",
        }
    }

    pub const fn keeps_local_path_visible(self) -> bool {
        matches!(self, Self::LocalPathVisible)
    }
}

/// One boundary card rendered on About, Help, marketplace, issue/reporting,
/// governance, contributing, discussion, release-notes, upgrade-or-hosted,
/// sponsorship, troubleshooting, or source-repository surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityBoundaryCardRecord {
    pub capability_boundary_card_schema_version: u32,
    pub record_kind: String,
    pub card_id: String,
    pub surface_class: BoundaryCardSurface,
    pub surface_ref: String,
    pub posture_class: SurfacePosture,
    pub identity_requirement_class: IdentityRequirement,
    pub network_requirement_class: NetworkRequirement,
    pub data_boundary_class: SurfaceDataBoundary,
    pub rollback_path_class: RollbackPath,
    pub continue_local_only_path_ref: Option<String>,
    pub rollback_or_downgrade_path_ref: Option<String>,
    pub upgrade_honesty_rule_class: UpgradeHonestyRule,
    pub support_prominence_class: SupportProminence,
    pub local_only_parity_class: LocalOnlyParity,
    pub linked_destination_refs: Vec<String>,
    #[serde(default)]
    pub linked_about_destination_refs: Vec<String>,
    pub headline_label: String,
    pub card_summary: String,
    pub contract_doc_ref: String,
    pub notes: Option<String>,
}

impl CapabilityBoundaryCardRecord {
    pub fn validate(&self) -> Result<(), AboutAndBoundaryValidationError> {
        if self.capability_boundary_card_schema_version != CAPABILITY_BOUNDARY_CARD_SCHEMA_VERSION {
            return Err(
                AboutAndBoundaryValidationError::WrongCapabilityBoundaryCardSchemaVersion {
                    card_id: self.card_id.clone(),
                    actual: self.capability_boundary_card_schema_version,
                },
            );
        }
        if self.record_kind != CAPABILITY_BOUNDARY_CARD_RECORD_KIND {
            return Err(
                AboutAndBoundaryValidationError::WrongCapabilityBoundaryCardRecordKind {
                    card_id: self.card_id.clone(),
                    actual: self.record_kind.clone(),
                },
            );
        }
        if !self.card_id.starts_with("capability_boundary_card:") {
            return Err(AboutAndBoundaryValidationError::MalformedCardId {
                card_id: self.card_id.clone(),
            });
        }
        if self.contract_doc_ref != ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AboutAndBoundaryValidationError::WrongContractDocRef {
                destination_id: self.card_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        if non_empty(&self.headline_label).is_none()
            || non_empty(&self.card_summary).is_none()
            || non_empty(&self.surface_ref).is_none()
        {
            return Err(AboutAndBoundaryValidationError::EmptyRequiredField {
                destination_id: self.card_id.clone(),
                field: "headline_label/card_summary/surface_ref",
            });
        }
        if self.linked_destination_refs.is_empty() {
            return Err(
                AboutAndBoundaryValidationError::CardMissingLinkedDestinations {
                    card_id: self.card_id.clone(),
                },
            );
        }

        if self.posture_class.is_premium_or_managed()
            && matches!(
                self.upgrade_honesty_rule_class,
                UpgradeHonestyRule::LocalPathHiddenViolation
            )
        {
            return Err(
                AboutAndBoundaryValidationError::PremiumOrManagedHidesLocalPath {
                    card_id: self.card_id.clone(),
                    posture: self.posture_class,
                },
            );
        }

        if matches!(
            self.upgrade_honesty_rule_class,
            UpgradeHonestyRule::LocalPathVisible
        ) {
            if self.continue_local_only_path_ref.is_none() {
                return Err(
                    AboutAndBoundaryValidationError::LocalPathVisibleMissingPathRef {
                        card_id: self.card_id.clone(),
                    },
                );
            }
            if !self.rollback_path_class.is_local_path() {
                return Err(
                    AboutAndBoundaryValidationError::LocalPathVisibleWithoutLocalRollback {
                        card_id: self.card_id.clone(),
                        rollback: self.rollback_path_class,
                    },
                );
            }
        }

        if self.posture_class.is_local_open() {
            if !self.identity_requirement_class.is_local_open_compatible()
                || !self.network_requirement_class.is_local_open_compatible()
                || !self.data_boundary_class.is_local_open_compatible()
            {
                return Err(
                    AboutAndBoundaryValidationError::LocalOpenSurfaceWithIncompatibleAxes {
                        card_id: self.card_id.clone(),
                    },
                );
            }
            if matches!(
                self.local_only_parity_class,
                LocalOnlyParity::HostedOnlyNoLocalFallback
            ) {
                return Err(
                    AboutAndBoundaryValidationError::LocalOpenSurfaceClaimsHostedOnly {
                        card_id: self.card_id.clone(),
                    },
                );
            }
        }

        if self.surface_class.is_support_oriented()
            && !self.support_prominence_class.ranks_support_above_upgrade()
        {
            return Err(
                AboutAndBoundaryValidationError::SupportSurfaceDeprioritized {
                    card_id: self.card_id.clone(),
                    surface: self.surface_class,
                    prominence: self.support_prominence_class,
                },
            );
        }

        if self.surface_class.is_upgrade_cta()
            && self.support_prominence_class.ranks_support_above_upgrade()
        {
            return Err(AboutAndBoundaryValidationError::UpgradeCtaOutranksSupport {
                card_id: self.card_id.clone(),
                surface: self.surface_class,
                prominence: self.support_prominence_class,
            });
        }

        if matches!(self.posture_class, SurfacePosture::ThirdPartyVendor)
            && !matches!(
                self.network_requirement_class,
                NetworkRequirement::VendorOrThirdPartyCall
                    | NetworkRequirement::CommunityPublicCall
            )
        {
            return Err(
                AboutAndBoundaryValidationError::ThirdPartyVendorWithoutVendorNetwork {
                    card_id: self.card_id.clone(),
                    network: self.network_requirement_class,
                },
            );
        }

        if matches!(
            self.upgrade_honesty_rule_class,
            UpgradeHonestyRule::NoLocalPathApplicable
        ) {
            if !matches!(
                self.local_only_parity_class,
                LocalOnlyParity::HostedOnlyNoLocalFallback | LocalOnlyParity::LocalOnlyOnly
            ) {
                return Err(
                    AboutAndBoundaryValidationError::NoLocalPathRuleWithLocalParityMismatch {
                        card_id: self.card_id.clone(),
                        parity: self.local_only_parity_class,
                    },
                );
            }
        }

        Ok(())
    }
}

/// One page bundling the destinations and boundary cards rendered together
/// on About, Help, or related surfaces. Used as the cross-validator
/// container so the page can prove that every linked destination ref
/// resolves and that the support-first / upgrade-honest invariants are
/// honored across the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AboutAndBoundaryTruthPage {
    pub about_and_boundary_truth_page_schema_version: u32,
    pub record_kind: String,
    pub page_id: String,
    pub page_summary: String,
    pub destinations: Vec<AboutDestinationRecord>,
    pub capability_boundary_cards: Vec<CapabilityBoundaryCardRecord>,
    pub contract_doc_ref: String,
    #[serde(default)]
    pub notes: Option<String>,
}

impl AboutAndBoundaryTruthPage {
    /// Cross-validate every destination and card on the page, then enforce
    /// page-level invariants (referential closure, account-optional parity,
    /// upgrade honesty, and destination uniqueness).
    pub fn validate(&self) -> Result<(), AboutAndBoundaryValidationError> {
        if self.about_and_boundary_truth_page_schema_version
            != ABOUT_AND_BOUNDARY_TRUTH_PAGE_SCHEMA_VERSION
        {
            return Err(AboutAndBoundaryValidationError::WrongPageSchemaVersion {
                page_id: self.page_id.clone(),
                actual: self.about_and_boundary_truth_page_schema_version,
            });
        }
        if self.record_kind != ABOUT_AND_BOUNDARY_TRUTH_PAGE_RECORD_KIND {
            return Err(AboutAndBoundaryValidationError::WrongPageRecordKind {
                page_id: self.page_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if self.contract_doc_ref != ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AboutAndBoundaryValidationError::WrongContractDocRef {
                destination_id: self.page_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        if non_empty(&self.page_summary).is_none() {
            return Err(AboutAndBoundaryValidationError::EmptyRequiredField {
                destination_id: self.page_id.clone(),
                field: "page_summary",
            });
        }
        if self.destinations.is_empty() {
            return Err(AboutAndBoundaryValidationError::EmptyPage {
                page_id: self.page_id.clone(),
            });
        }

        let mut destination_ids: BTreeSet<&str> = BTreeSet::new();
        for destination in &self.destinations {
            destination.validate()?;
            if !destination_ids.insert(destination.destination_id.as_str()) {
                return Err(AboutAndBoundaryValidationError::DuplicateDestinationId {
                    destination_id: destination.destination_id.clone(),
                });
            }
        }

        for destination in &self.destinations {
            if let Some(replacement) = destination.replacement_destination_ref.as_deref() {
                if !destination_ids.contains(replacement) {
                    return Err(
                        AboutAndBoundaryValidationError::ReplacementRefMissingFromPage {
                            destination_id: destination.destination_id.clone(),
                            replacement_ref: replacement.to_owned(),
                        },
                    );
                }
            }
            if let Some(fallback) = destination.local_only_fallback_ref.as_deref() {
                if !destination_ids.contains(fallback) {
                    return Err(
                        AboutAndBoundaryValidationError::LocalFallbackRefMissingFromPage {
                            destination_id: destination.destination_id.clone(),
                            fallback_ref: fallback.to_owned(),
                        },
                    );
                }
            }
        }

        let mut card_ids: BTreeSet<&str> = BTreeSet::new();
        for card in &self.capability_boundary_cards {
            card.validate()?;
            if !card_ids.insert(card.card_id.as_str()) {
                return Err(AboutAndBoundaryValidationError::DuplicateCardId {
                    card_id: card.card_id.clone(),
                });
            }
            for linked in &card.linked_destination_refs {
                if !destination_ids.contains(linked.as_str()) {
                    return Err(
                        AboutAndBoundaryValidationError::CardLinkedDestinationMissingFromPage {
                            card_id: card.card_id.clone(),
                            destination_ref: linked.clone(),
                        },
                    );
                }
            }
            for linked in &card.linked_about_destination_refs {
                if !destination_ids.contains(linked.as_str()) {
                    return Err(
                        AboutAndBoundaryValidationError::CardLinkedDestinationMissingFromPage {
                            card_id: card.card_id.clone(),
                            destination_ref: linked.clone(),
                        },
                    );
                }
            }
            if let Some(local_path) = card.continue_local_only_path_ref.as_deref() {
                if !destination_ids.contains(local_path) {
                    return Err(
                        AboutAndBoundaryValidationError::CardLocalPathRefMissingFromPage {
                            card_id: card.card_id.clone(),
                            destination_ref: local_path.to_owned(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Render a deterministic plaintext block for support exports and
    /// reviewer-facing previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("About / source / community-handoff and capability-boundary truth\n");
        out.push_str(&format!("Page: {}\n", self.page_id));
        out.push_str(&format!("Summary: {}\n\n", self.page_summary));

        out.push_str("Destinations:\n");
        for destination in &self.destinations {
            out.push_str(&format!(
                "- [{}] {} — class={} role={} route={} account={} data_exit={} prominence={} parity={}\n",
                destination.destination_id,
                destination.headline_label,
                destination.destination_class.as_str(),
                destination.destination_role_class.as_str(),
                destination.route_state_class.as_str(),
                destination.account_requirement_class.as_str(),
                destination.data_exit_boundary_class.as_str(),
                destination.support_prominence_class.as_str(),
                destination.local_only_parity_class.as_str(),
            ));
            if let Some(replacement) = &destination.replacement_destination_ref {
                out.push_str(&format!("    replacement: {replacement}\n"));
            }
            if let Some(fallback) = &destination.local_only_fallback_ref {
                out.push_str(&format!("    local-only fallback: {fallback}\n"));
            }
            for export in &destination.build_context_exports {
                out.push_str(&format!(
                    "    build-context export: {} (v{}, redacted_for={}, ref={})\n",
                    export.export_class.as_str(),
                    export.export_block_schema_version,
                    export.redacted_for_audience.as_str(),
                    export.export_block_ref,
                ));
            }
        }
        out.push('\n');

        out.push_str("Capability boundary cards:\n");
        for card in &self.capability_boundary_cards {
            out.push_str(&format!(
                "- [{}] {} — surface={} posture={} identity={} network={} data={} rollback={} honesty={}\n",
                card.card_id,
                card.headline_label,
                card.surface_class.as_str(),
                card.posture_class.as_str(),
                card.identity_requirement_class.as_str(),
                card.network_requirement_class.as_str(),
                card.data_boundary_class.as_str(),
                card.rollback_path_class.as_str(),
                card.upgrade_honesty_rule_class.as_str(),
            ));
            if let Some(local_path) = &card.continue_local_only_path_ref {
                out.push_str(&format!("    continue local-only: {local_path}\n"));
            }
            if let Some(rollback) = &card.rollback_or_downgrade_path_ref {
                out.push_str(&format!("    rollback path: {rollback}\n"));
            }
        }
        out
    }
}

/// Validation error vocabulary for the about-destination and capability-
/// boundary-card contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AboutAndBoundaryValidationError {
    WrongAboutDestinationSchemaVersion {
        destination_id: String,
        actual: u32,
    },
    WrongAboutDestinationRecordKind {
        destination_id: String,
        actual: String,
    },
    MalformedDestinationId {
        destination_id: String,
    },
    WrongContractDocRef {
        destination_id: String,
        actual: String,
    },
    EmptyRequiredField {
        destination_id: String,
        field: &'static str,
    },
    MissingSourceSurfaceRefs {
        destination_id: String,
    },
    DeadDestinationMissingReplacement {
        destination_id: String,
        route_state: RouteState,
    },
    DecommissionedRouteMissingLocalFallback {
        destination_id: String,
    },
    ThirdPartyVendorWithUnsupportedDataExit {
        destination_id: String,
        data_exit: DataExitBoundary,
    },
    SecurityIntakeWithWrongDataExit {
        destination_id: String,
        data_exit: DataExitBoundary,
    },
    SupportIntakeMislabeled {
        destination_id: String,
    },
    SupportRouteDeprioritizedBelowUpgrade {
        destination_id: String,
        role: DestinationRole,
        prominence: SupportProminence,
    },
    UpgradeRouteOutranksSupport {
        destination_id: String,
        role: DestinationRole,
        prominence: SupportProminence,
    },
    AccountCoercingRouteMissingLocalFallback {
        destination_id: String,
        account_requirement: AccountRequirement,
        parity: LocalOnlyParity,
    },
    LocalOnlyFallbackCoercesAccount {
        destination_id: String,
        account_requirement: AccountRequirement,
    },
    LocalOnlyFallbackDeprioritized {
        destination_id: String,
        prominence: SupportProminence,
    },
    HandoffRouteMissingBuildContextExport {
        destination_id: String,
        role: DestinationRole,
    },
    BuildContextExportSchemaVersionInvalid {
        destination_id: String,
        actual: u32,
    },
    BuildContextExportNotRedactionSafe {
        destination_id: String,
    },
    BuildContextExportFieldEmpty {
        destination_id: String,
    },

    WrongCapabilityBoundaryCardSchemaVersion {
        card_id: String,
        actual: u32,
    },
    WrongCapabilityBoundaryCardRecordKind {
        card_id: String,
        actual: String,
    },
    MalformedCardId {
        card_id: String,
    },
    CardMissingLinkedDestinations {
        card_id: String,
    },
    PremiumOrManagedHidesLocalPath {
        card_id: String,
        posture: SurfacePosture,
    },
    LocalPathVisibleMissingPathRef {
        card_id: String,
    },
    LocalPathVisibleWithoutLocalRollback {
        card_id: String,
        rollback: RollbackPath,
    },
    LocalOpenSurfaceWithIncompatibleAxes {
        card_id: String,
    },
    LocalOpenSurfaceClaimsHostedOnly {
        card_id: String,
    },
    SupportSurfaceDeprioritized {
        card_id: String,
        surface: BoundaryCardSurface,
        prominence: SupportProminence,
    },
    UpgradeCtaOutranksSupport {
        card_id: String,
        surface: BoundaryCardSurface,
        prominence: SupportProminence,
    },
    ThirdPartyVendorWithoutVendorNetwork {
        card_id: String,
        network: NetworkRequirement,
    },
    NoLocalPathRuleWithLocalParityMismatch {
        card_id: String,
        parity: LocalOnlyParity,
    },

    WrongPageSchemaVersion {
        page_id: String,
        actual: u32,
    },
    WrongPageRecordKind {
        page_id: String,
        actual: String,
    },
    EmptyPage {
        page_id: String,
    },
    DuplicateDestinationId {
        destination_id: String,
    },
    DuplicateCardId {
        card_id: String,
    },
    ReplacementRefMissingFromPage {
        destination_id: String,
        replacement_ref: String,
    },
    LocalFallbackRefMissingFromPage {
        destination_id: String,
        fallback_ref: String,
    },
    CardLinkedDestinationMissingFromPage {
        card_id: String,
        destination_ref: String,
    },
    CardLocalPathRefMissingFromPage {
        card_id: String,
        destination_ref: String,
    },
}

impl fmt::Display for AboutAndBoundaryValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongAboutDestinationSchemaVersion {
                destination_id,
                actual,
            } => write!(
                f,
                "destination {destination_id} has unsupported about_destination_schema_version {actual}"
            ),
            Self::WrongAboutDestinationRecordKind {
                destination_id,
                actual,
            } => write!(
                f,
                "destination {destination_id} has unsupported record kind {actual}"
            ),
            Self::MalformedDestinationId { destination_id } => {
                write!(f, "destination id {destination_id} must start with about_destination:")
            }
            Self::WrongContractDocRef {
                destination_id,
                actual,
            } => write!(
                f,
                "record {destination_id} cites wrong contract doc {actual}"
            ),
            Self::EmptyRequiredField {
                destination_id,
                field,
            } => write!(f, "record {destination_id} is missing required field {field}"),
            Self::MissingSourceSurfaceRefs { destination_id } => {
                write!(f, "destination {destination_id} is missing source_surface_refs")
            }
            Self::DeadDestinationMissingReplacement {
                destination_id,
                route_state,
            } => write!(
                f,
                "destination {destination_id} has route state {} but no replacement_destination_ref",
                route_state.as_str()
            ),
            Self::DecommissionedRouteMissingLocalFallback { destination_id } => write!(
                f,
                "decommissioned destination {destination_id} is missing local_only_fallback_ref"
            ),
            Self::ThirdPartyVendorWithUnsupportedDataExit {
                destination_id,
                data_exit,
            } => write!(
                f,
                "third-party vendor destination {destination_id} has unsupported data_exit_boundary {}",
                data_exit.as_str()
            ),
            Self::SecurityIntakeWithWrongDataExit {
                destination_id,
                data_exit,
            } => write!(
                f,
                "security intake destination {destination_id} must carry security_payloads_only data exit, not {}",
                data_exit.as_str()
            ),
            Self::SupportIntakeMislabeled { destination_id } => write!(
                f,
                "support intake destination {destination_id} must be official_private with redacted_support_packet data exit"
            ),
            Self::SupportRouteDeprioritizedBelowUpgrade {
                destination_id,
                role,
                prominence,
            } => write!(
                f,
                "support route {destination_id} ({}) cannot use prominence {}",
                role.as_str(),
                prominence.as_str()
            ),
            Self::UpgradeRouteOutranksSupport {
                destination_id,
                role,
                prominence,
            } => write!(
                f,
                "upgrade/sponsorship route {destination_id} ({}) cannot use prominence {}",
                role.as_str(),
                prominence.as_str()
            ),
            Self::AccountCoercingRouteMissingLocalFallback {
                destination_id,
                account_requirement,
                parity,
            } => write!(
                f,
                "destination {destination_id} coerces account ({}) under parity {} but is missing local_only_fallback_ref",
                account_requirement.as_str(),
                parity.as_str()
            ),
            Self::LocalOnlyFallbackCoercesAccount {
                destination_id,
                account_requirement,
            } => write!(
                f,
                "local-only fallback {destination_id} must not coerce account (got {})",
                account_requirement.as_str()
            ),
            Self::LocalOnlyFallbackDeprioritized {
                destination_id,
                prominence,
            } => write!(
                f,
                "local-only fallback {destination_id} must rank support above upgrade (got {})",
                prominence.as_str()
            ),
            Self::HandoffRouteMissingBuildContextExport {
                destination_id,
                role,
            } => write!(
                f,
                "handoff route {destination_id} ({}) must attach a build-context export block",
                role.as_str()
            ),
            Self::BuildContextExportSchemaVersionInvalid {
                destination_id,
                actual,
            } => write!(
                f,
                "destination {destination_id} has invalid build-context export schema version {actual}"
            ),
            Self::BuildContextExportNotRedactionSafe { destination_id } => write!(
                f,
                "destination {destination_id} build-context export is not redaction safe"
            ),
            Self::BuildContextExportFieldEmpty { destination_id } => write!(
                f,
                "destination {destination_id} has an empty build-context export field"
            ),

            Self::WrongCapabilityBoundaryCardSchemaVersion { card_id, actual } => write!(
                f,
                "boundary card {card_id} has unsupported capability_boundary_card_schema_version {actual}"
            ),
            Self::WrongCapabilityBoundaryCardRecordKind { card_id, actual } => {
                write!(f, "boundary card {card_id} has unsupported record kind {actual}")
            }
            Self::MalformedCardId { card_id } => {
                write!(f, "boundary card id {card_id} must start with capability_boundary_card:")
            }
            Self::CardMissingLinkedDestinations { card_id } => {
                write!(f, "boundary card {card_id} is missing linked_destination_refs")
            }
            Self::PremiumOrManagedHidesLocalPath { card_id, posture } => write!(
                f,
                "boundary card {card_id} ({}) cannot violate upgrade honesty by hiding the local path",
                posture.as_str()
            ),
            Self::LocalPathVisibleMissingPathRef { card_id } => write!(
                f,
                "boundary card {card_id} declares local_path_visible but is missing continue_local_only_path_ref"
            ),
            Self::LocalPathVisibleWithoutLocalRollback { card_id, rollback } => write!(
                f,
                "boundary card {card_id} declares local_path_visible but rollback_path_class is {} (must resolve to a local path)",
                rollback.as_str()
            ),
            Self::LocalOpenSurfaceWithIncompatibleAxes { card_id } => write!(
                f,
                "boundary card {card_id} declares local_open but identity/network/data axes are incompatible with account-optional local"
            ),
            Self::LocalOpenSurfaceClaimsHostedOnly { card_id } => write!(
                f,
                "boundary card {card_id} declares local_open posture but local_only_parity is hosted_only_no_local_fallback"
            ),
            Self::SupportSurfaceDeprioritized {
                card_id,
                surface,
                prominence,
            } => write!(
                f,
                "support surface {card_id} ({}) cannot use prominence {}",
                surface.as_str(),
                prominence.as_str()
            ),
            Self::UpgradeCtaOutranksSupport {
                card_id,
                surface,
                prominence,
            } => write!(
                f,
                "upgrade CTA {card_id} ({}) cannot use prominence {}",
                surface.as_str(),
                prominence.as_str()
            ),
            Self::ThirdPartyVendorWithoutVendorNetwork { card_id, network } => write!(
                f,
                "third-party vendor surface {card_id} must disclose vendor/third-party network call, not {}",
                network.as_str()
            ),
            Self::NoLocalPathRuleWithLocalParityMismatch { card_id, parity } => write!(
                f,
                "boundary card {card_id} declares no_local_path_applicable but parity is {}",
                parity.as_str()
            ),

            Self::WrongPageSchemaVersion { page_id, actual } => write!(
                f,
                "page {page_id} has unsupported page schema version {actual}"
            ),
            Self::WrongPageRecordKind { page_id, actual } => {
                write!(f, "page {page_id} has unsupported record kind {actual}")
            }
            Self::EmptyPage { page_id } => write!(f, "page {page_id} is empty"),
            Self::DuplicateDestinationId { destination_id } => {
                write!(f, "duplicate destination id {destination_id}")
            }
            Self::DuplicateCardId { card_id } => write!(f, "duplicate card id {card_id}"),
            Self::ReplacementRefMissingFromPage {
                destination_id,
                replacement_ref,
            } => write!(
                f,
                "destination {destination_id} cites replacement {replacement_ref} which is not present on the page"
            ),
            Self::LocalFallbackRefMissingFromPage {
                destination_id,
                fallback_ref,
            } => write!(
                f,
                "destination {destination_id} cites local fallback {fallback_ref} which is not present on the page"
            ),
            Self::CardLinkedDestinationMissingFromPage {
                card_id,
                destination_ref,
            } => write!(
                f,
                "boundary card {card_id} links destination {destination_ref} which is not present on the page"
            ),
            Self::CardLocalPathRefMissingFromPage {
                card_id,
                destination_ref,
            } => write!(
                f,
                "boundary card {card_id} cites local path {destination_ref} which is not present on the page"
            ),
        }
    }
}

impl std::error::Error for AboutAndBoundaryValidationError {}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests;
