//! Release-center / provenance skeleton with support/export linkage.
//!
//! This is the M1 seed for the canonical "what shipped, where did it come
//! from, and how does support pull the receipts" surface. Today it is a
//! protected-row entry point a reviewer can open from a dogfood build to
//! confirm that the running build's exact-build identity, channel, and
//! provenance row scaffold are linked to the live support-bundle preview
//! instead of forking into a different vocabulary per surface.
//!
//! ## Why one truth model, not several
//!
//! The release center, the public provenance line, the support-bundle
//! manifest, the About card, and (later) the update/rollback tools all
//! need to answer the same question when a reviewer asks "what build is
//! this and where did it come from?". Forking a private layout per surface
//! lets one entry drift its vocabulary while another lags — for example,
//! the release center quoting one exact-build identity while the support
//! bundle quotes a stale label. This module mints one
//! [`ReleaseCenterSurface`] record that joins the canonical
//! [`aureline_build_info::BuildIdentityRecord`], the release-channel-class
//! token, and the live [`crate::support_seed::SupportSeedSurface`] preview
//! whose manifest already carries the seed's exact-build refs verbatim.
//!
//! ## What the seed surface carries
//!
//! - **Build identity** — every value comes from the build-info record
//!   minted at compile time; the surface never re-derives versions.
//! - **Channel** — quoted verbatim from the running build's
//!   [`aureline_build_info::release_channel_class`] token plus the mirror
//!   it lands on in the support manifest's
//!   [`aureline_support::bundle::ReleaseChannelClass`] vocabulary.
//! - **Release-candidate row** — one inspectable row that joins the
//!   exact-build identity, channel, and provenance link state. The seed
//!   only mints the running-build row; the staged-candidate / promoted /
//!   revoked / rolled-back rows are reserved.
//! - **Provenance row scaffold** — typed seed-placeholder rows for
//!   signature, attestation, checksum, SBOM, and revocation state. Mirrors
//!   the Help/About provenance scaffold so the same vocabulary lights both
//!   surfaces.
//! - **Support/export linkage** — the running release-candidate row quotes
//!   the live support-seed surface's manifest exact-build refs. When the
//!   support manifest carries no exact-build refs (failure drill), the
//!   linkage row's state flips to `missing_chain` so the chrome's banner
//!   cannot fabricate "linked" while the chain is broken.
//! - **Closed action set** — `open_local_support_preview`,
//!   `copy_provenance_line_for_support`, and `view_exact_build_identity`
//!   are live; publish / promote / rollback / revoke / yank rows are
//!   reserved with stable tokens so the chrome cannot silently activate
//!   them.
//!
//! ## Failure-drill posture
//!
//! Breaking the provenance linkage is the named failure drill. When the
//! support-seed surface's manifest carries no exact-build refs, or the
//! running build's exact-build identity is empty, the surface's linkage
//! row reports `missing_chain`, the seed lights `honesty_marker_present`,
//! and the support-export action stays live so a reviewer can still copy
//! the surface for a support packet.

use serde::{Deserialize, Serialize};

use aureline_build_info::BuildIdentityRecord;
use aureline_support::bundle::ReleaseChannelClass;

use crate::help_about::{InstallModeClass, ProvenanceRowClass, ProvenanceRowState, TreeStateClass};
use crate::support_seed::SupportSeedSurface;

/// Stable record-kind tag carried in serialized release-center payloads.
pub const RELEASE_CENTER_SURFACE_RECORD_KIND: &str = "release_center_surface_record";

/// Schema version for the [`ReleaseCenterSurface`] payload shape.
pub const RELEASE_CENTER_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every release-center surface so the
/// lane's depth is not overstated.
pub const RELEASE_CENTER_SEED_SCOPE_NOTICE: &str =
    "Release center seed: live rows quote the running build's exact-build identity, the resolved \
     release-channel class, and the linked support-bundle preview. Provenance row state, \
     publish / promote / rollback / revoke / yank actions, and full release-candidate browsing \
     are reserved for a later milestone.";

/// Stable command id for opening the linked local support-bundle preview.
pub const COMMAND_ID_OPEN_LOCAL_SUPPORT_PREVIEW: &str =
    "cmd:release_center.open_local_support_preview";

/// Stable command id for copying the provenance line for support hand-off.
pub const COMMAND_ID_COPY_PROVENANCE_LINE_FOR_SUPPORT: &str =
    "cmd:release_center.copy_provenance_line_for_support";

/// Stable command id for opening the exact-build identity inspector.
pub const COMMAND_ID_VIEW_EXACT_BUILD_IDENTITY: &str =
    "cmd:release_center.view_exact_build_identity";

/// Stable section ids the seed surface renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseCenterSectionId {
    BuildIdentity,
    ReleaseCandidate,
    ProvenanceScaffold,
    SupportLinkage,
    Actions,
}

impl ReleaseCenterSectionId {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::ReleaseCandidate => "release_candidate",
            Self::ProvenanceScaffold => "provenance_scaffold",
            Self::SupportLinkage => "support_linkage",
            Self::Actions => "actions",
        }
    }

    /// Human-readable section heading.
    pub const fn heading(self) -> &'static str {
        match self {
            Self::BuildIdentity => "Build identity",
            Self::ReleaseCandidate => "Release candidate",
            Self::ProvenanceScaffold => "Provenance",
            Self::SupportLinkage => "Support linkage",
            Self::Actions => "Actions",
        }
    }
}

/// Stable role for one release-center row. The seed mints only the
/// running-build row; staged-candidate / promoted / revoked / rolled-back
/// rows are reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseCandidateRoleClass {
    /// The build the user is currently running. Live in the seed.
    RunningBuild,
    /// A staged release candidate awaiting promotion. Reserved.
    StagedCandidate,
    /// A promoted release candidate. Reserved.
    PromotedRelease,
    /// A revoked artifact. Reserved.
    RevokedArtifact,
    /// A rollback candidate. Reserved.
    RollbackCandidate,
}

impl ReleaseCandidateRoleClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunningBuild => "running_build",
            Self::StagedCandidate => "staged_candidate",
            Self::PromotedRelease => "promoted_release",
            Self::RevokedArtifact => "revoked_artifact",
            Self::RollbackCandidate => "rollback_candidate",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RunningBuild => "Running build",
            Self::StagedCandidate => "Staged candidate",
            Self::PromotedRelease => "Promoted release",
            Self::RevokedArtifact => "Revoked artifact",
            Self::RollbackCandidate => "Rollback candidate",
        }
    }
}

/// Linkage state between the release-center row and a downstream
/// support-export / provenance surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceLinkState {
    /// The linked surface carries the same exact-build identity. Healthy.
    Linked,
    /// The seed has not wired this lane yet; the row is labeled honestly so
    /// the chrome cannot claim a link this seed does not own.
    SeedPlaceholderAwaitingWiring,
    /// The downstream surface exists but does not carry a matching
    /// exact-build identity. Failure-drill state.
    MissingChain,
    /// The downstream surface was not wired at all (e.g. no support-seed
    /// surface was provided). The row stays honest about the missing input.
    NotWired,
}

impl ProvenanceLinkState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::SeedPlaceholderAwaitingWiring => "seed_placeholder_awaiting_wiring",
            Self::MissingChain => "missing_chain",
            Self::NotWired => "not_wired",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Linked => "Linked",
            Self::SeedPlaceholderAwaitingWiring => "Seed placeholder (wiring pending)",
            Self::MissingChain => "Missing chain",
            Self::NotWired => "Not wired",
        }
    }

    /// True when the link state should light the global honesty marker.
    pub const fn is_honest_warning(self) -> bool {
        matches!(self, Self::MissingChain | Self::NotWired)
    }
}

/// Frozen seed-action vocabulary for the release-center surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseCenterActionClass {
    /// Open the linked local support-bundle preview without any upload.
    /// Live in the seed.
    OpenLocalSupportPreview,
    /// Copy the deterministic provenance line to the clipboard for a
    /// support hand-off. Live in the seed.
    CopyProvenanceLineForSupport,
    /// Open the exact-build identity inspector. Live in the seed.
    ViewExactBuildIdentity,
    /// Publish a staged candidate to a publish target. Reserved.
    PublishStagedCandidate,
    /// Promote a release candidate. Reserved.
    PromoteReleaseCandidate,
    /// Roll a release back to a prior exact-build identity. Reserved.
    RollbackToPriorBuild,
    /// Revoke a published artifact. Reserved.
    RevokePublishedArtifact,
    /// Yank a published artifact from a registry. Reserved.
    YankPublishedArtifact,
}

impl ReleaseCenterActionClass {
    /// Stable string token recorded on the action row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocalSupportPreview => "open_local_support_preview",
            Self::CopyProvenanceLineForSupport => "copy_provenance_line_for_support",
            Self::ViewExactBuildIdentity => "view_exact_build_identity",
            Self::PublishStagedCandidate => "publish_staged_candidate",
            Self::PromoteReleaseCandidate => "promote_release_candidate",
            Self::RollbackToPriorBuild => "rollback_to_prior_build",
            Self::RevokePublishedArtifact => "revoke_published_artifact",
            Self::YankPublishedArtifact => "yank_published_artifact",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenLocalSupportPreview => "Open local support preview",
            Self::CopyProvenanceLineForSupport => "Copy provenance line for support",
            Self::ViewExactBuildIdentity => "View exact-build identity",
            Self::PublishStagedCandidate => "Publish staged candidate",
            Self::PromoteReleaseCandidate => "Promote release candidate",
            Self::RollbackToPriorBuild => "Roll back to prior build",
            Self::RevokePublishedArtifact => "Revoke published artifact",
            Self::YankPublishedArtifact => "Yank published artifact",
        }
    }

    /// True when the action is wired to a live command in this seed.
    pub const fn is_live(self) -> bool {
        matches!(
            self,
            Self::OpenLocalSupportPreview
                | Self::CopyProvenanceLineForSupport
                | Self::ViewExactBuildIdentity
        )
    }

    /// Stable command id when the action is live; `None` for reserved rows
    /// so the chrome cannot silently route them.
    pub const fn command_id(self) -> Option<&'static str> {
        match self {
            Self::OpenLocalSupportPreview => Some(COMMAND_ID_OPEN_LOCAL_SUPPORT_PREVIEW),
            Self::CopyProvenanceLineForSupport => Some(COMMAND_ID_COPY_PROVENANCE_LINE_FOR_SUPPORT),
            Self::ViewExactBuildIdentity => Some(COMMAND_ID_VIEW_EXACT_BUILD_IDENTITY),
            Self::PublishStagedCandidate
            | Self::PromoteReleaseCandidate
            | Self::RollbackToPriorBuild
            | Self::RevokePublishedArtifact
            | Self::YankPublishedArtifact => None,
        }
    }
}

/// Availability class rendered on every action row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseCenterActionAvailability {
    /// Live within the seed.
    Live,
    /// Reserved for a later milestone; the row stays visible but disabled.
    ReservedForLaterMilestone,
    /// Live target action is held back because the linkage row is missing.
    BlockedByMissingLinkage,
}

impl ReleaseCenterActionAvailability {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::ReservedForLaterMilestone => "reserved_for_later_milestone",
            Self::BlockedByMissingLinkage => "blocked_by_missing_linkage",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::ReservedForLaterMilestone => "Reserved for a later milestone",
            Self::BlockedByMissingLinkage => "Blocked: missing linkage",
        }
    }
}

/// Origin posture quoted on the public provenance line. Mirrors the
/// vocabulary frozen in
/// `docs/release/release_center_provenance_linkage.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginPostureClass {
    /// Built locally from source (dev tree).
    LocalDevBuild,
    /// Official channel install.
    OfficialChannelInstall,
    /// Mirrored from an authorized mirror (subset of official).
    MirroredOfficial,
    /// Side-loaded artifact whose provenance is not verified by this seed.
    SideLoadedUnverified,
    /// Origin posture cannot be derived (e.g. unknown channel token).
    UnknownOriginPosture,
}

impl OriginPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDevBuild => "local_dev_build",
            Self::OfficialChannelInstall => "official_channel_install",
            Self::MirroredOfficial => "mirrored_official",
            Self::SideLoadedUnverified => "side_loaded_unverified",
            Self::UnknownOriginPosture => "unknown_origin_posture",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDevBuild => "Local — built from source",
            Self::OfficialChannelInstall => "Official",
            Self::MirroredOfficial => "Mirrored — official",
            Self::SideLoadedUnverified => "Side-loaded — unverified",
            Self::UnknownOriginPosture => "Unknown origin",
        }
    }

    /// Derive the seed's origin posture from the install-mode class. The
    /// seed never makes a "verified" claim — that's reserved for a later
    /// milestone — so an official-channel install is always labeled
    /// `OfficialChannelInstall` without claiming signature trust.
    pub const fn from_install_mode(install_mode: InstallModeClass) -> Self {
        match install_mode {
            InstallModeClass::DevLocalBuiltFromSource => Self::LocalDevBuild,
            InstallModeClass::NightlyLocalInstall
            | InstallModeClass::PreviewLocalInstall
            | InstallModeClass::BetaLocalInstall
            | InstallModeClass::StableLocalInstall
            | InstallModeClass::LtsLocalInstall
            | InstallModeClass::HotfixLocalInstall => Self::OfficialChannelInstall,
            InstallModeClass::UnknownInstallMode => Self::UnknownOriginPosture,
        }
    }
}

/// Inputs the surface needs to project one record. Every field comes from
/// an upstream truth source the release-center lane reuses; the projection
/// never invents build, channel, or support truth of its own.
#[derive(Debug, Clone)]
pub struct ReleaseCenterInputs<'a> {
    /// Build-identity record minted at compile time by the build-info
    /// crate.
    pub build_identity: &'a BuildIdentityRecord,
    /// Stable release-channel-class token (e.g. `dev_local`, `nightly`,
    /// `stable`). Comes from
    /// [`aureline_build_info::release_channel_class`].
    pub release_channel_class_token: &'a str,
    /// Exact-build identity ref minted by
    /// [`aureline_build_info::exact_build_identity_ref`]. Held as an input
    /// so tests can inject a deterministic value.
    pub exact_build_identity_ref: &'a str,
    /// Live support-seed surface. The seed reads its manifest's
    /// exact-build refs to confirm the support/export linkage. `None`
    /// signals that no support-seed surface was wired at all; the linkage
    /// row degrades to a typed `not_wired` state and the seed lights its
    /// global honesty marker.
    pub support_seed: Option<&'a SupportSeedSurface>,
}

/// Build-identity section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterBuildIdentitySection {
    pub product_name_class: String,
    pub workspace_version: String,
    pub release_channel_class_token: String,
    pub release_channel_class: ReleaseChannelClass,
    pub install_mode_class: InstallModeClass,
    pub install_mode_token: String,
    pub install_mode_label: String,
    pub exact_build_identity_ref: String,
    pub commit: String,
    pub commit_short: String,
    pub tree_state_class: TreeStateClass,
    pub tree_state_class_token: String,
    pub tree_state_label: String,
    pub host_triple: String,
    pub target_triple: String,
    pub profile: String,
    pub origin_posture_class: OriginPostureClass,
    pub origin_posture_token: String,
    pub origin_posture_label: String,
    /// True when the running build's exact-build identity ref is empty or
    /// the channel token did not resolve. Lights the global honesty
    /// marker.
    pub honesty_marker_present: bool,
}

/// One release-candidate row on the surface. The seed only mints the
/// running-build row; staged / promoted / revoked / rolled-back rows are
/// reserved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCandidateRow {
    pub role_class: ReleaseCandidateRoleClass,
    pub role_class_token: String,
    pub role_label: String,
    pub release_channel_class: ReleaseChannelClass,
    pub release_channel_class_token: String,
    pub exact_build_identity_ref: String,
    pub product_version: String,
    pub provenance_line: String,
    pub support_link_state: ProvenanceLinkState,
    pub support_link_state_token: String,
    pub support_link_state_label: String,
    /// Stable refs to the linked support-bundle preview's exact-build
    /// identity set. Empty when no link is wired or the chain is missing.
    pub linked_support_exact_build_refs: Vec<String>,
    /// True when the row degraded to a `missing_chain` or `not_wired`
    /// state.
    pub honesty_marker_present: bool,
}

/// One provenance-scaffold row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterProvenanceRow {
    pub row_class: ProvenanceRowClass,
    pub row_class_token: String,
    pub label: String,
    pub state: ProvenanceRowState,
    pub state_token: String,
    pub state_label: String,
}

/// Provenance-scaffold section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterProvenanceSection {
    pub rows: Vec<ReleaseCenterProvenanceRow>,
    pub honesty_marker_present: bool,
}

/// Support/export linkage section. Quotes the live support-seed manifest's
/// exact-build refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterSupportLinkageSection {
    pub link_state: ProvenanceLinkState,
    pub link_state_token: String,
    pub link_state_label: String,
    /// Stable command id for the live "open local support preview" action
    /// when the link is wired. `None` when reserved or blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_preview_command_id: Option<String>,
    pub support_seed_record_kind_present: bool,
    pub support_manifest_exact_build_refs: Vec<String>,
    pub support_manifest_has_prohibited_row: bool,
    pub honesty_marker_present: bool,
}

/// One action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterAction {
    pub action_class: ReleaseCenterActionClass,
    pub action_class_token: String,
    pub label: String,
    pub availability: ReleaseCenterActionAvailability,
    pub availability_token: String,
    pub availability_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved_reason: Option<String>,
}

impl ReleaseCenterAction {
    fn build(
        action_class: ReleaseCenterActionClass,
        availability: ReleaseCenterActionAvailability,
    ) -> Self {
        let command_id = if matches!(availability, ReleaseCenterActionAvailability::Live) {
            action_class.command_id().map(|id| id.to_owned())
        } else {
            None
        };
        let reserved_reason = match availability {
            ReleaseCenterActionAvailability::ReservedForLaterMilestone => Some(
                "Action is reserved for a later milestone; the seed never silently activates it."
                    .to_owned(),
            ),
            ReleaseCenterActionAvailability::BlockedByMissingLinkage => Some(
                "Linked support preview is missing or has no exact-build refs; reopening the \
                 preview is held back until the chain is restored."
                    .to_owned(),
            ),
            ReleaseCenterActionAvailability::Live => None,
        };
        Self {
            action_class,
            action_class_token: action_class.as_str().to_owned(),
            label: action_class.label().to_owned(),
            availability,
            availability_token: availability.as_str().to_owned(),
            availability_label: availability.label().to_owned(),
            command_id,
            reserved_reason,
        }
    }
}

/// Release-center / provenance seed surface record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterSurface {
    pub record_kind: String,
    pub schema_version: u32,
    pub seed_scope_notice: String,
    pub heading: String,
    pub build_identity: ReleaseCenterBuildIdentitySection,
    pub running_release_candidate: ReleaseCandidateRow,
    pub provenance_scaffold: ReleaseCenterProvenanceSection,
    pub support_linkage: ReleaseCenterSupportLinkageSection,
    pub actions: Vec<ReleaseCenterAction>,
    pub honesty_marker_present: bool,
}

impl ReleaseCenterSurface {
    /// Project a release-center surface from the named upstream inputs.
    pub fn project(inputs: ReleaseCenterInputs<'_>) -> Self {
        let ReleaseCenterInputs {
            build_identity,
            release_channel_class_token,
            exact_build_identity_ref,
            support_seed,
        } = inputs;

        let install_mode = InstallModeClass::from_channel_token(release_channel_class_token);
        let release_channel_class =
            ReleaseChannelClass::from_build_token(release_channel_class_token);
        let origin_posture = OriginPostureClass::from_install_mode(install_mode);
        let tree_state = if build_identity.dirty {
            TreeStateClass::DirtyLocal
        } else {
            TreeStateClass::CleanCheckout
        };

        let build_identity_section = ReleaseCenterBuildIdentitySection {
            product_name_class: "aureline".to_owned(),
            workspace_version: build_identity.workspace_version.clone(),
            release_channel_class_token: release_channel_class_token.to_owned(),
            release_channel_class,
            install_mode_class: install_mode,
            install_mode_token: install_mode.as_str().to_owned(),
            install_mode_label: install_mode.label().to_owned(),
            exact_build_identity_ref: exact_build_identity_ref.to_owned(),
            commit: build_identity.commit.clone(),
            commit_short: build_identity.commit_short.clone(),
            tree_state_class: tree_state,
            tree_state_class_token: tree_state.as_str().to_owned(),
            tree_state_label: tree_state.label().to_owned(),
            host_triple: build_identity.host_triple.clone(),
            target_triple: build_identity.target_triple.clone(),
            profile: build_identity.profile.clone(),
            origin_posture_class: origin_posture,
            origin_posture_token: origin_posture.as_str().to_owned(),
            origin_posture_label: origin_posture.label().to_owned(),
            honesty_marker_present: exact_build_identity_ref.is_empty()
                || matches!(install_mode, InstallModeClass::UnknownInstallMode),
        };

        let support_linkage_section =
            project_support_linkage(exact_build_identity_ref, support_seed);

        let provenance_line = compose_provenance_line(
            &build_identity_section.workspace_version,
            release_channel_class_token,
            origin_posture,
            exact_build_identity_ref,
        );

        let running_release_candidate = ReleaseCandidateRow {
            role_class: ReleaseCandidateRoleClass::RunningBuild,
            role_class_token: ReleaseCandidateRoleClass::RunningBuild.as_str().to_owned(),
            role_label: ReleaseCandidateRoleClass::RunningBuild.label().to_owned(),
            release_channel_class,
            release_channel_class_token: release_channel_class_token.to_owned(),
            exact_build_identity_ref: exact_build_identity_ref.to_owned(),
            product_version: build_identity.workspace_version.clone(),
            provenance_line,
            support_link_state: support_linkage_section.link_state,
            support_link_state_token: support_linkage_section.link_state_token.clone(),
            support_link_state_label: support_linkage_section.link_state_label.clone(),
            linked_support_exact_build_refs: support_linkage_section
                .support_manifest_exact_build_refs
                .clone(),
            honesty_marker_present: support_linkage_section.honesty_marker_present
                || build_identity_section.honesty_marker_present,
        };

        let provenance_scaffold = project_provenance_scaffold();

        let actions = build_actions(support_linkage_section.link_state);

        let honesty_marker_present = build_identity_section.honesty_marker_present
            || running_release_candidate.honesty_marker_present
            || support_linkage_section.honesty_marker_present
            || provenance_scaffold.honesty_marker_present;

        Self {
            record_kind: RELEASE_CENTER_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RELEASE_CENTER_SURFACE_SCHEMA_VERSION,
            seed_scope_notice: RELEASE_CENTER_SEED_SCOPE_NOTICE.to_owned(),
            heading: "Release center — running build and support linkage".to_owned(),
            build_identity: build_identity_section,
            running_release_candidate,
            provenance_scaffold,
            support_linkage: support_linkage_section,
            actions,
            honesty_marker_present,
        }
    }

    /// Find the first action with the given class.
    pub fn find_action(
        &self,
        action_class: ReleaseCenterActionClass,
    ) -> Option<&ReleaseCenterAction> {
        self.actions.iter().find(|a| a.action_class == action_class)
    }

    /// True when the running release candidate carries a non-empty
    /// exact-build identity ref. The protected-walk acceptance condition.
    pub fn has_running_build_exact_build_identity(&self) -> bool {
        !self
            .running_release_candidate
            .exact_build_identity_ref
            .is_empty()
    }

    /// True when the support linkage row is in a healthy `linked` state.
    pub fn support_linkage_is_linked(&self) -> bool {
        matches!(self.support_linkage.link_state, ProvenanceLinkState::Linked)
    }

    /// Render a deterministic plaintext block for the copy-context action
    /// and support exports. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Release center surface\n");
        out.push_str(&format!("Heading: {}\n", self.heading));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        out.push('\n');

        out.push_str(&format!(
            "[{}]\n  Product: {}\n  Version: {}\n  Channel: {}\n  Install mode: {} ({})\n  Origin: {} ({})\n  Exact build: {}\n  Commit: {} (full: {})\n  Tree state: {} ({})\n  Host: {}\n  Target: {}\n  Profile: {}\n\n",
            ReleaseCenterSectionId::BuildIdentity.heading(),
            self.build_identity.product_name_class,
            self.build_identity.workspace_version,
            self.build_identity.release_channel_class_token,
            self.build_identity.install_mode_label,
            self.build_identity.install_mode_token,
            self.build_identity.origin_posture_label,
            self.build_identity.origin_posture_token,
            self.build_identity.exact_build_identity_ref,
            self.build_identity.commit_short,
            self.build_identity.commit,
            self.build_identity.tree_state_label,
            self.build_identity.tree_state_class_token,
            self.build_identity.host_triple,
            self.build_identity.target_triple,
            self.build_identity.profile,
        ));

        out.push_str(&format!(
            "[{}]\n  Role: {} ({})\n  Channel: {}\n  Exact build: {}\n  Provenance: {}\n  Support link: {} ({})\n",
            ReleaseCenterSectionId::ReleaseCandidate.heading(),
            self.running_release_candidate.role_label,
            self.running_release_candidate.role_class_token,
            self.running_release_candidate.release_channel_class_token,
            self.running_release_candidate.exact_build_identity_ref,
            self.running_release_candidate.provenance_line,
            self.running_release_candidate.support_link_state_label,
            self.running_release_candidate.support_link_state_token,
        ));
        if !self
            .running_release_candidate
            .linked_support_exact_build_refs
            .is_empty()
        {
            out.push_str("  Linked support refs:\n");
            for refed in &self
                .running_release_candidate
                .linked_support_exact_build_refs
            {
                out.push_str(&format!("    - {refed}\n"));
            }
        }
        out.push('\n');

        out.push_str(&format!(
            "[{}]\n",
            ReleaseCenterSectionId::ProvenanceScaffold.heading(),
        ));
        for row in &self.provenance_scaffold.rows {
            out.push_str(&format!(
                "  - {}: {} [{}]\n",
                row.row_class_token, row.label, row.state_token,
            ));
        }
        out.push('\n');

        out.push_str(&format!(
            "[{}]\n  State: {} ({})\n  Support seed wired: {}\n  Manifest has prohibited row: {}\n",
            ReleaseCenterSectionId::SupportLinkage.heading(),
            self.support_linkage.link_state_label,
            self.support_linkage.link_state_token,
            self.support_linkage.support_seed_record_kind_present,
            self.support_linkage.support_manifest_has_prohibited_row,
        ));
        if !self
            .support_linkage
            .support_manifest_exact_build_refs
            .is_empty()
        {
            out.push_str("  Manifest exact-build refs:\n");
            for refed in &self.support_linkage.support_manifest_exact_build_refs {
                out.push_str(&format!("    - {refed}\n"));
            }
        }
        out.push('\n');

        out.push_str(&format!(
            "[{}]\n",
            ReleaseCenterSectionId::Actions.heading()
        ));
        for action in &self.actions {
            out.push_str(&format!(
                "  - {}: {} [{}]\n",
                action.action_class_token, action.label, action.availability_token,
            ));
        }
        out.push('\n');
        out.push_str(&format!("Notice: {}\n", self.seed_scope_notice));
        out
    }
}

fn project_support_linkage(
    exact_build_identity_ref: &str,
    support_seed: Option<&SupportSeedSurface>,
) -> ReleaseCenterSupportLinkageSection {
    match support_seed {
        Some(seed) => {
            let manifest_refs = seed.manifest().build_identity.exact_build_refs.clone();
            let has_match = !exact_build_identity_ref.is_empty()
                && manifest_refs.iter().any(|r| r == exact_build_identity_ref);
            let link_state = if has_match {
                ProvenanceLinkState::Linked
            } else {
                ProvenanceLinkState::MissingChain
            };
            let open_preview_command_id = match link_state {
                ProvenanceLinkState::Linked => {
                    Some(COMMAND_ID_OPEN_LOCAL_SUPPORT_PREVIEW.to_owned())
                }
                _ => None,
            };
            ReleaseCenterSupportLinkageSection {
                link_state,
                link_state_token: link_state.as_str().to_owned(),
                link_state_label: link_state.label().to_owned(),
                open_preview_command_id,
                support_seed_record_kind_present: true,
                support_manifest_exact_build_refs: manifest_refs,
                support_manifest_has_prohibited_row: seed.has_prohibited_row(),
                honesty_marker_present: link_state.is_honest_warning(),
            }
        }
        None => {
            let link_state = ProvenanceLinkState::NotWired;
            ReleaseCenterSupportLinkageSection {
                link_state,
                link_state_token: link_state.as_str().to_owned(),
                link_state_label: link_state.label().to_owned(),
                open_preview_command_id: None,
                support_seed_record_kind_present: false,
                support_manifest_exact_build_refs: Vec::new(),
                support_manifest_has_prohibited_row: false,
                honesty_marker_present: link_state.is_honest_warning(),
            }
        }
    }
}

fn project_provenance_scaffold() -> ReleaseCenterProvenanceSection {
    let row_classes = [
        ProvenanceRowClass::SignatureState,
        ProvenanceRowClass::AttestationState,
        ProvenanceRowClass::ChecksumState,
        ProvenanceRowClass::SbomState,
        ProvenanceRowClass::AdvisoryOpenState,
    ];
    let rows = row_classes
        .into_iter()
        .map(|class| ReleaseCenterProvenanceRow {
            row_class: class,
            row_class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            state: ProvenanceRowState::SeedPlaceholderAwaitingWiring,
            state_token: ProvenanceRowState::SeedPlaceholderAwaitingWiring
                .as_str()
                .to_owned(),
            state_label: ProvenanceRowState::SeedPlaceholderAwaitingWiring
                .label()
                .to_owned(),
        })
        .collect();
    ReleaseCenterProvenanceSection {
        rows,
        // Seed placeholders are in-spec for the M1 lane; they do not light
        // the global honesty marker on their own.
        honesty_marker_present: false,
    }
}

fn build_actions(link_state: ProvenanceLinkState) -> Vec<ReleaseCenterAction> {
    let action_classes = [
        ReleaseCenterActionClass::OpenLocalSupportPreview,
        ReleaseCenterActionClass::CopyProvenanceLineForSupport,
        ReleaseCenterActionClass::ViewExactBuildIdentity,
        ReleaseCenterActionClass::PublishStagedCandidate,
        ReleaseCenterActionClass::PromoteReleaseCandidate,
        ReleaseCenterActionClass::RollbackToPriorBuild,
        ReleaseCenterActionClass::RevokePublishedArtifact,
        ReleaseCenterActionClass::YankPublishedArtifact,
    ];
    action_classes
        .into_iter()
        .map(|class| {
            let availability = derive_availability(class, link_state);
            ReleaseCenterAction::build(class, availability)
        })
        .collect()
}

fn derive_availability(
    class: ReleaseCenterActionClass,
    link_state: ProvenanceLinkState,
) -> ReleaseCenterActionAvailability {
    if !class.is_live() {
        return ReleaseCenterActionAvailability::ReservedForLaterMilestone;
    }
    // The "open local support preview" action depends on a healthy support
    // linkage. Copying the provenance line and viewing the exact-build
    // identity stay live so a reviewer can still hand the running build's
    // identity to support during a missing-chain failure drill.
    if matches!(class, ReleaseCenterActionClass::OpenLocalSupportPreview) {
        return match link_state {
            ProvenanceLinkState::Linked => ReleaseCenterActionAvailability::Live,
            _ => ReleaseCenterActionAvailability::BlockedByMissingLinkage,
        };
    }
    ReleaseCenterActionAvailability::Live
}

fn compose_provenance_line(
    workspace_version: &str,
    release_channel_class_token: &str,
    origin_posture: OriginPostureClass,
    exact_build_identity_ref: &str,
) -> String {
    let exact_build = if exact_build_identity_ref.is_empty() {
        "unknown_exact_build_identity".to_owned()
    } else {
        exact_build_identity_ref.to_owned()
    };
    format!(
        "Aureline {version} ({channel}) — {origin} — {exact_build}",
        version = workspace_version,
        channel = release_channel_class_token,
        origin = origin_posture.label(),
        exact_build = exact_build,
    )
}

#[cfg(test)]
mod tests;
