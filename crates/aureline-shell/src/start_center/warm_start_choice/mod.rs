//! Beta warm-start choice projection for Start Center and workspace switching.
//!
//! ## Why one warm-start truth, not one per surface
//!
//! When a user enters Aureline from the Start Center, the workspace switcher, a
//! scripted/headless entry review, or while reading docs/help, they all need the
//! same honest answer to one question: *what will actually happen if I pick this
//! card?* Will Aureline resume a live workspace, reuse a stored snapshot, clone
//! something fresh over the network, or simply open local files? Forking a
//! private layout per surface lets one surface render a managed/remote entry as
//! a plain local open while another discloses the network and trust cost — the
//! exact start-of-work confusion this lane exists to remove.
//!
//! This module mints a single [`WarmStartChoiceCard`] object model. Each card
//! carries the template/source identity, the available entry **lanes**
//! ([`WarmStartPathClass`]), per-lane side-effect and trust truth, snapshot
//! freshness/fingerprint/invalidation facts, and an environment-starter summary
//! describing where setup runs and how to bypass or defer it. The Start Center,
//! workspace switcher, CLI/headless entry review, docs/help, and the support
//! export are all meant to read this record verbatim instead of hand-editing
//! per-surface copy.
//!
//! ## Invariants the projection holds
//!
//! - **Path clarity before commitment.** Every card exposes its lanes explicitly
//!   so the user can distinguish `resume_live_workspace`, `start_from_snapshot`,
//!   `clone_fresh`, `open_minimal`, `set_up_later`, and `use_template` before any
//!   networked or trust-widening side effect occurs.
//! - **The default is always local-safe.** A card's `safest_next_action` must
//!   resolve to a lane with no network egress, no setup tasks, and no trust
//!   grant. Hitting the default never widens trust or materializes hidden
//!   networked work.
//! - **Open-minimal and set-up-later stay same-weight on local-first rows.** A
//!   card that claims a local-first runtime MUST keep both escape hatches at the
//!   same weight as the convenience lanes.
//! - **A stale snapshot is never a live resume.** When a snapshot is stale or
//!   invalidated the card surfaces the invalidation reason and disables (does not
//!   merely warn on) the `resume_live_workspace` lane.
//! - **Remote/managed lanes cannot masquerade as a local open.** Any lane that
//!   requires the network, attaches managed/remote runtime, or widens trust must
//!   not advertise a local-safe side-effect class and must not be one of the
//!   local escape-hatch lanes.
//!
//! ## Out of scope
//!
//! The projection is read-only. It does not create cloud control-plane
//! workspaces, productize collaboration/session-join, run setup tasks, mint
//! credentials, or perform any clone/resume itself. It only describes the
//! choices so the user can decide before anything happens.

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for serialized warm-start choice pages.
pub const WARM_START_CHOICE_PAGE_RECORD_KIND: &str = "warm_start_choice_page_record";

/// Stable record-kind tag for serialized warm-start choice cards.
pub const WARM_START_CHOICE_CARD_RECORD_KIND: &str = "warm_start_choice_card_record";

/// Stable record-kind tag for serialized warm-start choice support exports.
pub const WARM_START_CHOICE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "warm_start_choice_support_export_record";

/// Schema version exported with every warm-start choice payload.
pub const WARM_START_CHOICE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that reads this model.
pub const WARM_START_CHOICE_SHARED_CONTRACT_REF: &str =
    "shell:start_center_warm_start_choice_beta:v1";

/// Normative contract document this projection reads.
pub const WARM_START_CHOICE_CONTRACT_REF: &str = "docs/workspace/m3/start_center_warm_start_beta.md";

/// Reviewer-facing notice rendered on every warm-start choice surface so the
/// lane's scope is never overstated.
pub const WARM_START_CHOICE_NOTICE: &str =
    "Warm-start choice beta surface: cards describe what each entry path will do before any \
     networked or trust-widening side effect occurs. The shell never resumes, clones, runs setup \
     tasks, widens trust, or materializes remote work from this surface; it only lets the user \
     choose. Open-minimal and set-up-later stay same-weight on local-first rows.";

/// The five surfaces this beta truth lane MUST stay consistent across.
pub const WARM_START_CHOICE_CONSUMING_SURFACES: &[&str] = &[
    "start_center",
    "workspace_switcher",
    "cli_headless_entry_review",
    "docs_help",
    "support_export",
];

/// Surface a card is rendered on first. The same object model is shared, so a
/// card minted for the switcher is byte-identical to one minted for the Start
/// Center other than this origin tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartSurfaceClass {
    StartCenter,
    WorkspaceSwitcher,
}

impl WarmStartSurfaceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::WorkspaceSwitcher => "workspace_switcher",
        }
    }
}

/// Closed source-class vocabulary describing what backs a warm-start card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartSourceClass {
    /// A reviewable workspace template/scaffold.
    WorkspaceTemplate,
    /// A stored prebuild/warm-start snapshot.
    PrebuildSnapshot,
    /// A live workspace that can be resumed.
    LiveWorkspace,
    /// A remote repository that would be cloned.
    RemoteRepository,
    /// A plain local folder.
    LocalFolder,
    /// Token did not match the closed vocabulary; held honest.
    UnknownSource,
}

impl WarmStartSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTemplate => "workspace_template",
            Self::PrebuildSnapshot => "prebuild_snapshot",
            Self::LiveWorkspace => "live_workspace",
            Self::RemoteRepository => "remote_repository",
            Self::LocalFolder => "local_folder",
            Self::UnknownSource => "unknown_source",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceTemplate => "Workspace template",
            Self::PrebuildSnapshot => "Prebuild snapshot",
            Self::LiveWorkspace => "Live workspace",
            Self::RemoteRepository => "Remote repository",
            Self::LocalFolder => "Local folder",
            Self::UnknownSource => "Unknown source",
        }
    }

    /// Parse from a source-class token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "workspace_template" => Self::WorkspaceTemplate,
            "prebuild_snapshot" => Self::PrebuildSnapshot,
            "live_workspace" => Self::LiveWorkspace,
            "remote_repository" => Self::RemoteRepository,
            "local_folder" => Self::LocalFolder,
            _ => Self::UnknownSource,
        }
    }
}

/// Closed support-class vocabulary mirrored from the claim/compatibility lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartSupportClass {
    Certified,
    Supported,
    Limited,
    Experimental,
    Community,
    Unsupported,
    UnknownSupport,
}

impl WarmStartSupportClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Experimental => "experimental",
            Self::Community => "community",
            Self::Unsupported => "unsupported",
            Self::UnknownSupport => "unknown_support",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Certified => "Certified",
            Self::Supported => "Supported",
            Self::Limited => "Limited",
            Self::Experimental => "Experimental",
            Self::Community => "Community",
            Self::Unsupported => "Unsupported",
            Self::UnknownSupport => "Unknown support",
        }
    }

    /// Parse from a support-class token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "certified" => Self::Certified,
            "supported" => Self::Supported,
            "limited" => Self::Limited,
            "experimental" => Self::Experimental,
            "community" => Self::Community,
            "unsupported" => Self::Unsupported,
            _ => Self::UnknownSupport,
        }
    }
}

/// Where the resulting workspace runs / what host model it uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeHostModelClass {
    LocalHost,
    Devcontainer,
    ManagedCloudWorkspace,
    SshWorkspace,
    UnknownRuntimeHostModel,
}

impl RuntimeHostModelClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::Devcontainer => "devcontainer",
            Self::ManagedCloudWorkspace => "managed_cloud_workspace",
            Self::SshWorkspace => "ssh_workspace",
            Self::UnknownRuntimeHostModel => "unknown_runtime_host_model",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalHost => "Local host",
            Self::Devcontainer => "Dev container",
            Self::ManagedCloudWorkspace => "Managed cloud workspace",
            Self::SshWorkspace => "SSH workspace",
            Self::UnknownRuntimeHostModel => "Unknown runtime/host model",
        }
    }

    /// Parse from a runtime/host token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "local_host" => Self::LocalHost,
            "devcontainer" => Self::Devcontainer,
            "managed_cloud_workspace" => Self::ManagedCloudWorkspace,
            "ssh_workspace" => Self::SshWorkspace,
            _ => Self::UnknownRuntimeHostModel,
        }
    }
}

/// The closed warm-start entry-path vocabulary. These are the picker lanes the
/// user chooses between before anything happens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartPathClass {
    /// Re-attach to a live workspace/session.
    ResumeLiveWorkspace,
    /// Reuse a stored snapshot/prebuild.
    StartFromSnapshot,
    /// Clone the source fresh.
    CloneFresh,
    /// Open the local files only, with no starter and no setup.
    OpenMinimal,
    /// Record the choice and defer setup to later.
    SetUpLater,
    /// Generate a new workspace from a template/scaffold.
    UseTemplate,
    /// Token did not match the closed vocabulary; held honest.
    UnknownPath,
}

impl WarmStartPathClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumeLiveWorkspace => "resume_live_workspace",
            Self::StartFromSnapshot => "start_from_snapshot",
            Self::CloneFresh => "clone_fresh",
            Self::OpenMinimal => "open_minimal",
            Self::SetUpLater => "set_up_later",
            Self::UseTemplate => "use_template",
            Self::UnknownPath => "unknown_path",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ResumeLiveWorkspace => "Resume live workspace",
            Self::StartFromSnapshot => "Start from snapshot",
            Self::CloneFresh => "Clone fresh",
            Self::OpenMinimal => "Open minimal",
            Self::SetUpLater => "Set up later",
            Self::UseTemplate => "Use template",
            Self::UnknownPath => "Unknown path",
        }
    }

    /// Parse from a path token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "resume_live_workspace" => Self::ResumeLiveWorkspace,
            "start_from_snapshot" => Self::StartFromSnapshot,
            "clone_fresh" => Self::CloneFresh,
            "open_minimal" => Self::OpenMinimal,
            "set_up_later" => Self::SetUpLater,
            "use_template" => Self::UseTemplate,
            _ => Self::UnknownPath,
        }
    }

    /// True for the two local escape-hatch lanes that must remain same-weight on
    /// local-first cards and never carry a side effect.
    pub const fn is_local_escape_hatch(self) -> bool {
        matches!(self, Self::OpenMinimal | Self::SetUpLater)
    }
}

/// Whether a lane can be taken now, and if not, why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartLaneAvailability {
    /// Takeable immediately.
    Available,
    /// Takeable after the user confirms the disclosed side effects.
    AvailableAfterReview,
    /// Requires re-authentication/re-authorization before it can run.
    RequiresReauth,
    /// Disabled because the backing snapshot is stale/invalidated.
    UnavailableStaleSnapshot,
    /// Disabled by policy in this context.
    BlockedByPolicy,
    /// Token did not match the closed vocabulary; held honest.
    UnknownAvailability,
}

impl WarmStartLaneAvailability {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::AvailableAfterReview => "available_after_review",
            Self::RequiresReauth => "requires_reauth",
            Self::UnavailableStaleSnapshot => "unavailable_stale_snapshot",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::UnknownAvailability => "unknown_availability",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::AvailableAfterReview => "Available after review",
            Self::RequiresReauth => "Requires re-authorization",
            Self::UnavailableStaleSnapshot => "Unavailable — snapshot stale",
            Self::BlockedByPolicy => "Blocked by policy",
            Self::UnknownAvailability => "Unknown availability",
        }
    }

    /// Parse from an availability token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "available" => Self::Available,
            "available_after_review" => Self::AvailableAfterReview,
            "requires_reauth" => Self::RequiresReauth,
            "unavailable_stale_snapshot" => Self::UnavailableStaleSnapshot,
            "blocked_by_policy" => Self::BlockedByPolicy,
            _ => Self::UnknownAvailability,
        }
    }

    /// True when this availability is itself a reason to light the honesty
    /// marker (the user would otherwise be surprised by the gate).
    pub const fn is_honest_warning(self) -> bool {
        matches!(
            self,
            Self::RequiresReauth | Self::UnavailableStaleSnapshot | Self::BlockedByPolicy
        )
    }

    /// True when the lane can actually be taken (immediately or after review).
    pub const fn is_takeable(self) -> bool {
        matches!(self, Self::Available | Self::AvailableAfterReview)
    }
}

/// The dominant side-effect class a lane carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartSideEffectClass {
    /// No side effect at all.
    NoSideEffect,
    /// Reads local/cached state only; no writes, network, or trust change.
    LocalReadOnly,
    /// Runs local setup tasks (no network, no trust widening).
    LocalSetup,
    /// Performs network egress (downloads, fetch, clone).
    NetworkEgress,
    /// Attaches a managed/remote runtime.
    ManagedAttach,
    /// Widens workspace trust.
    TrustWidening,
    /// Token did not match the closed vocabulary; held honest.
    UnknownSideEffect,
}

impl WarmStartSideEffectClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffect => "no_side_effect",
            Self::LocalReadOnly => "local_read_only",
            Self::LocalSetup => "local_setup",
            Self::NetworkEgress => "network_egress",
            Self::ManagedAttach => "managed_attach",
            Self::TrustWidening => "trust_widening",
            Self::UnknownSideEffect => "unknown_side_effect",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoSideEffect => "No side effect",
            Self::LocalReadOnly => "Local, read-only",
            Self::LocalSetup => "Local setup",
            Self::NetworkEgress => "Network egress",
            Self::ManagedAttach => "Managed/remote attach",
            Self::TrustWidening => "Trust widening",
            Self::UnknownSideEffect => "Unknown side effect",
        }
    }

    /// Parse from a side-effect token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "no_side_effect" => Self::NoSideEffect,
            "local_read_only" => Self::LocalReadOnly,
            "local_setup" => Self::LocalSetup,
            "network_egress" => Self::NetworkEgress,
            "managed_attach" => Self::ManagedAttach,
            "trust_widening" => Self::TrustWidening,
            _ => Self::UnknownSideEffect,
        }
    }

    /// True for the side-effect classes a local-safe default may carry.
    pub const fn is_local_safe(self) -> bool {
        matches!(self, Self::NoSideEffect | Self::LocalReadOnly)
    }
}

/// Snapshot freshness classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartFreshnessClass {
    Fresh,
    Cached,
    Stale,
    Invalidated,
    Unverified,
    UnknownFreshness,
}

impl WarmStartFreshnessClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Invalidated => "invalidated",
            Self::Unverified => "unverified",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Fresh",
            Self::Cached => "Cached",
            Self::Stale => "Stale",
            Self::Invalidated => "Invalidated",
            Self::Unverified => "Unverified",
            Self::UnknownFreshness => "Unknown freshness",
        }
    }

    /// Parse from a freshness token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "fresh" => Self::Fresh,
            "cached" => Self::Cached,
            "stale" => Self::Stale,
            "invalidated" => Self::Invalidated,
            "unverified" => Self::Unverified,
            _ => Self::UnknownFreshness,
        }
    }

    /// True when the snapshot cannot back a live resume.
    pub const fn is_stale_or_invalidated(self) -> bool {
        matches!(self, Self::Stale | Self::Invalidated)
    }
}

/// Snapshot age bucket, kept coarse so it stays support-safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartAgeClass {
    WithinHours,
    WithinDays,
    WithinWeeks,
    BeyondReviewWindow,
    UnknownAge,
}

impl WarmStartAgeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinHours => "within_hours",
            Self::WithinDays => "within_days",
            Self::WithinWeeks => "within_weeks",
            Self::BeyondReviewWindow => "beyond_review_window",
            Self::UnknownAge => "unknown_age",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WithinHours => "Within hours",
            Self::WithinDays => "Within days",
            Self::WithinWeeks => "Within weeks",
            Self::BeyondReviewWindow => "Beyond review window",
            Self::UnknownAge => "Unknown age",
        }
    }

    /// Parse from an age token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "within_hours" => Self::WithinHours,
            "within_days" => Self::WithinDays,
            "within_weeks" => Self::WithinWeeks,
            "beyond_review_window" => Self::BeyondReviewWindow,
            _ => Self::UnknownAge,
        }
    }
}

/// Where an environment starter would run setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartSetupLocationClass {
    NoSetup,
    LocalHost,
    Devcontainer,
    ManagedCloud,
    SshHost,
    UnknownSetupLocation,
}

impl WarmStartSetupLocationClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSetup => "no_setup",
            Self::LocalHost => "local_host",
            Self::Devcontainer => "devcontainer",
            Self::ManagedCloud => "managed_cloud",
            Self::SshHost => "ssh_host",
            Self::UnknownSetupLocation => "unknown_setup_location",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoSetup => "No setup",
            Self::LocalHost => "Local host",
            Self::Devcontainer => "Dev container",
            Self::ManagedCloud => "Managed cloud",
            Self::SshHost => "SSH host",
            Self::UnknownSetupLocation => "Unknown setup location",
        }
    }

    /// Parse from a setup-location token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "no_setup" => Self::NoSetup,
            "local_host" => Self::LocalHost,
            "devcontainer" => Self::Devcontainer,
            "managed_cloud" => Self::ManagedCloud,
            "ssh_host" => Self::SshHost,
            _ => Self::UnknownSetupLocation,
        }
    }
}

/// One entry lane offered by a warm-start card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartChoiceLane {
    pub path_class: WarmStartPathClass,
    pub path_token: String,
    pub label: String,
    pub summary: String,
    pub availability: WarmStartLaneAvailability,
    pub availability_token: String,
    pub side_effect_class: WarmStartSideEffectClass,
    pub side_effect_token: String,
    /// Whether taking the lane widens workspace trust.
    pub requires_trust_grant: bool,
    /// Whether taking the lane requires network egress.
    pub requires_network: bool,
    /// Whether taking the lane runs setup tasks.
    pub runs_setup_tasks: bool,
    /// Whether taking the lane attaches managed/remote runtime work.
    pub materializes_remote_work: bool,
    /// Whether this lane is one of the same-weight local escape hatches.
    pub same_weight_local_path: bool,
    /// Invalidation reason quoted verbatim when a snapshot lane is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalidation_reason: Option<String>,
}

impl WarmStartChoiceLane {
    /// True when the lane reaches outside local state — network, managed/remote
    /// attach, or trust widening. Such a lane must never look like a plain
    /// local open.
    pub fn is_remote_or_widening(&self) -> bool {
        self.requires_network
            || self.materializes_remote_work
            || self.requires_trust_grant
            || matches!(
                self.side_effect_class,
                WarmStartSideEffectClass::NetworkEgress
                    | WarmStartSideEffectClass::ManagedAttach
                    | WarmStartSideEffectClass::TrustWidening
            )
    }

    /// True when the lane is local-safe enough to be a card's default.
    pub fn is_local_safe(&self) -> bool {
        !self.requires_network
            && !self.requires_trust_grant
            && !self.runs_setup_tasks
            && !self.materializes_remote_work
            && self.side_effect_class.is_local_safe()
    }
}

/// Snapshot facts surfaced directly on the entry card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartSnapshotFacts {
    pub source_class: WarmStartSourceClass,
    pub source_class_token: String,
    /// Opaque, support-safe fingerprint reference (never raw content).
    pub fingerprint_ref: String,
    pub freshness: WarmStartFreshnessClass,
    pub freshness_token: String,
    pub age_class: WarmStartAgeClass,
    pub age_token: String,
    /// Reviewable capture timestamp.
    pub captured_at: String,
    /// Invalidation reason; required when freshness is stale/invalidated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalidation_reason: Option<String>,
    /// Always true: a stale snapshot must never render as a live resume.
    pub stale_must_not_render_as_live_resume: bool,
}

/// Side-effect summary surfaced on the card before any lane is taken.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartSideEffectSummary {
    pub network_egress: bool,
    pub extension_installs: bool,
    pub setup_tasks: bool,
    pub trust_prompt: bool,
    pub managed_or_remote_attach: bool,
    pub notes: Vec<String>,
}

/// Environment-starter summary: where setup runs and how to bypass/defer it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartEnvironmentStarter {
    pub setup_location_class: WarmStartSetupLocationClass,
    pub setup_location_token: String,
    pub downloads_involved: bool,
    pub extensions_involved: bool,
    pub tasks_involved: bool,
    pub trust_prompt_involved: bool,
    /// Lane/route ids that bypass the starter (open without setup).
    pub bypass_route_ids: Vec<String>,
    /// Lane/route ids that defer the starter to later.
    pub defer_route_ids: Vec<String>,
    pub summary: String,
}

/// One warm-start choice card shared across every entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartChoiceCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub card_id: String,
    pub surface_origin: WarmStartSurfaceClass,
    pub surface_origin_token: String,
    pub display_label: String,
    pub summary: String,
    pub source_class: WarmStartSourceClass,
    pub source_class_token: String,
    pub support_class: WarmStartSupportClass,
    pub support_class_token: String,
    pub runtime_or_host_model: RuntimeHostModelClass,
    pub runtime_or_host_model_token: String,
    /// Whether the resulting workspace claims a local-first runtime.
    pub local_first: bool,
    /// Plain-language setup actions a starter would perform.
    pub expected_setup_actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<WarmStartSnapshotFacts>,
    pub side_effects: WarmStartSideEffectSummary,
    pub environment_starter: WarmStartEnvironmentStarter,
    pub choice_lanes: Vec<WarmStartChoiceLane>,
    /// The lane the user gets by default; always resolves to a local-safe lane.
    pub safest_next_action: WarmStartPathClass,
    pub safest_next_action_token: String,
    /// Whether the default action widens trust. Always false.
    pub default_widens_trust: bool,
    /// Whether the default action runs networked work. Always false.
    pub default_runs_networked_work: bool,
    /// Whether this card should light the honesty marker.
    pub honesty_marker_present: bool,
}

impl WarmStartChoiceCard {
    /// Returns the lane matching the given path class, if any.
    pub fn lane(&self, path: WarmStartPathClass) -> Option<&WarmStartChoiceLane> {
        self.choice_lanes
            .iter()
            .find(|lane| lane.path_class == path)
    }
}

/// Aggregated counters across all cards on the page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WarmStartChoiceSummary {
    pub total_card_count: u32,
    pub template_card_count: u32,
    pub live_resume_card_count: u32,
    pub snapshot_card_count: u32,
    pub clone_fresh_card_count: u32,
    pub local_first_card_count: u32,
    pub stale_snapshot_card_count: u32,
    pub revalidation_required_card_count: u32,
    pub honesty_marker_card_count: u32,
}

/// Page-level warm-start choice projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartChoicePage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_label: String,
    pub contract_ref: String,
    pub notice: String,
    pub consuming_surfaces: Vec<String>,
    pub cards: Vec<WarmStartChoiceCard>,
    pub summary: WarmStartChoiceSummary,
    /// Whether open-minimal stays same-weight on every local-first card.
    pub open_minimal_same_weight_on_local_first: bool,
    /// Whether set-up-later stays same-weight on every local-first card.
    pub set_up_later_same_weight_on_local_first: bool,
    pub honesty_marker_present: bool,
}

/// Support-export wrapper for the warm-start choice page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartChoiceSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub exported_at: String,
    pub page: WarmStartChoicePage,
    pub card_ids: Vec<String>,
    /// Raw secret / credential material is excluded by construction.
    pub raw_secret_material_excluded: bool,
}

impl WarmStartChoiceSupportExport {
    /// Builds a support export from a page projection.
    pub fn from_page(export_id: &str, exported_at: &str, page: WarmStartChoicePage) -> Self {
        let card_ids = page.cards.iter().map(|card| card.card_id.clone()).collect();
        Self {
            record_kind: WARM_START_CHOICE_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: WARM_START_CHOICE_SCHEMA_VERSION,
            shared_contract_ref: WARM_START_CHOICE_SHARED_CONTRACT_REF.to_string(),
            export_id: export_id.to_string(),
            exported_at: exported_at.to_string(),
            page,
            card_ids,
            raw_secret_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Seed
// ---------------------------------------------------------------------------

/// Builds the deterministic seeded warm-start choice page. Five cards cover the
/// full path taxonomy: a local-first template, a managed live-resume that needs
/// re-authorization, a valid local snapshot, a stale snapshot that cannot
/// masquerade as a live resume, and a clone-fresh remote repository.
pub fn seeded_warm_start_choice_page() -> WarmStartChoicePage {
    let cards = vec![
        seed_template_card(),
        seed_managed_live_resume_card(),
        seed_valid_snapshot_card(),
        seed_stale_snapshot_card(),
        seed_clone_fresh_card(),
    ];
    finalize_page(cards)
}

/// Builds the seeded support export for the warm-start choice page.
pub fn seeded_warm_start_choice_support_export() -> WarmStartChoiceSupportExport {
    WarmStartChoiceSupportExport::from_page(
        "support-export:warm-start-choice-beta:001",
        "2026-05-20T00:00:00Z",
        seeded_warm_start_choice_page(),
    )
}

fn finalize_page(cards: Vec<WarmStartChoiceCard>) -> WarmStartChoicePage {
    let summary = compute_summary(&cards);
    let honesty_marker_present = cards.iter().any(|card| card.honesty_marker_present);
    let open_minimal_same_weight_on_local_first = cards
        .iter()
        .filter(|card| card.local_first)
        .all(|card| lane_same_weight(card, WarmStartPathClass::OpenMinimal));
    let set_up_later_same_weight_on_local_first = cards
        .iter()
        .filter(|card| card.local_first)
        .all(|card| lane_same_weight(card, WarmStartPathClass::SetUpLater));

    WarmStartChoicePage {
        record_kind: WARM_START_CHOICE_PAGE_RECORD_KIND.to_string(),
        schema_version: WARM_START_CHOICE_SCHEMA_VERSION,
        shared_contract_ref: WARM_START_CHOICE_SHARED_CONTRACT_REF.to_string(),
        page_label: "Start Center warm-start choices beta".to_string(),
        contract_ref: WARM_START_CHOICE_CONTRACT_REF.to_string(),
        notice: WARM_START_CHOICE_NOTICE.to_string(),
        consuming_surfaces: WARM_START_CHOICE_CONSUMING_SURFACES
            .iter()
            .map(|surface| (*surface).to_string())
            .collect(),
        cards,
        summary,
        open_minimal_same_weight_on_local_first,
        set_up_later_same_weight_on_local_first,
        honesty_marker_present,
    }
}

fn lane_same_weight(card: &WarmStartChoiceCard, path: WarmStartPathClass) -> bool {
    card.lane(path)
        .map(|lane| lane.same_weight_local_path && lane.is_local_safe())
        .unwrap_or(false)
}

fn compute_summary(cards: &[WarmStartChoiceCard]) -> WarmStartChoiceSummary {
    let mut summary = WarmStartChoiceSummary {
        total_card_count: cards.len() as u32,
        ..WarmStartChoiceSummary::default()
    };
    for card in cards {
        if card.source_class == WarmStartSourceClass::WorkspaceTemplate {
            summary.template_card_count += 1;
        }
        if card
            .lane(WarmStartPathClass::ResumeLiveWorkspace)
            .is_some()
        {
            summary.live_resume_card_count += 1;
        }
        if card.snapshot.is_some() {
            summary.snapshot_card_count += 1;
        }
        if card.lane(WarmStartPathClass::CloneFresh).is_some() {
            summary.clone_fresh_card_count += 1;
        }
        if card.local_first {
            summary.local_first_card_count += 1;
        }
        if card
            .snapshot
            .as_ref()
            .map(|snapshot| snapshot.freshness.is_stale_or_invalidated())
            .unwrap_or(false)
        {
            summary.stale_snapshot_card_count += 1;
        }
        if card
            .choice_lanes
            .iter()
            .any(|lane| lane.availability == WarmStartLaneAvailability::RequiresReauth)
        {
            summary.revalidation_required_card_count += 1;
        }
        if card.honesty_marker_present {
            summary.honesty_marker_card_count += 1;
        }
    }
    summary
}

fn lane(
    path: WarmStartPathClass,
    label: &str,
    summary: &str,
    availability: WarmStartLaneAvailability,
    side_effect_class: WarmStartSideEffectClass,
) -> WarmStartChoiceLane {
    WarmStartChoiceLane {
        path_class: path,
        path_token: path.as_str().to_string(),
        label: label.to_string(),
        summary: summary.to_string(),
        availability,
        availability_token: availability.as_str().to_string(),
        side_effect_class,
        side_effect_token: side_effect_class.as_str().to_string(),
        requires_trust_grant: false,
        requires_network: false,
        runs_setup_tasks: false,
        materializes_remote_work: false,
        same_weight_local_path: path.is_local_escape_hatch(),
        invalidation_reason: None,
    }
}

fn open_minimal_lane() -> WarmStartChoiceLane {
    lane(
        WarmStartPathClass::OpenMinimal,
        WarmStartPathClass::OpenMinimal.label(),
        "Open the local files only. No starter, no setup, no network.",
        WarmStartLaneAvailability::Available,
        WarmStartSideEffectClass::NoSideEffect,
    )
}

fn set_up_later_lane() -> WarmStartChoiceLane {
    lane(
        WarmStartPathClass::SetUpLater,
        WarmStartPathClass::SetUpLater.label(),
        "Open now and keep the starter available to run later. Nothing runs yet.",
        WarmStartLaneAvailability::Available,
        WarmStartSideEffectClass::NoSideEffect,
    )
}

fn seed_template_card() -> WarmStartChoiceCard {
    let mut use_template = lane(
        WarmStartPathClass::UseTemplate,
        WarmStartPathClass::UseTemplate.label(),
        "Scaffold a new TypeScript web workspace, then install dependencies on this machine.",
        WarmStartLaneAvailability::AvailableAfterReview,
        WarmStartSideEffectClass::NetworkEgress,
    );
    use_template.requires_network = true;
    use_template.runs_setup_tasks = true;

    let choice_lanes = vec![use_template, open_minimal_lane(), set_up_later_lane()];
    let side_effects = WarmStartSideEffectSummary {
        network_egress: true,
        extension_installs: true,
        setup_tasks: true,
        trust_prompt: false,
        managed_or_remote_attach: false,
        notes: vec![
            "Dependency install fetches packages from the configured registry.".to_string(),
            "Recommended extensions install into this workspace only.".to_string(),
        ],
    };
    let environment_starter = WarmStartEnvironmentStarter {
        setup_location_class: WarmStartSetupLocationClass::LocalHost,
        setup_location_token: WarmStartSetupLocationClass::LocalHost.as_str().to_string(),
        downloads_involved: true,
        extensions_involved: true,
        tasks_involved: true,
        trust_prompt_involved: false,
        bypass_route_ids: vec!["open_minimal".to_string()],
        defer_route_ids: vec!["set_up_later".to_string()],
        summary: "Setup runs on your local host. You can open minimal or set up later instead."
            .to_string(),
    };
    finalize_card(WarmStartChoiceCardSeed {
        card_id: "warm_start_card:template.ts_web.local",
        surface_origin: WarmStartSurfaceClass::StartCenter,
        display_label: "TypeScript web app template",
        summary: "Start a new local-first TypeScript web workspace from a supported template.",
        source_class: WarmStartSourceClass::WorkspaceTemplate,
        support_class: WarmStartSupportClass::Supported,
        runtime_or_host_model: RuntimeHostModelClass::LocalHost,
        local_first: true,
        expected_setup_actions: vec![
            "Write template files into the chosen folder".to_string(),
            "Install dependencies with the project package manager".to_string(),
            "Install recommended workspace extensions".to_string(),
        ],
        snapshot: None,
        side_effects,
        environment_starter,
        choice_lanes,
        safest_next_action: WarmStartPathClass::OpenMinimal,
    })
}

fn seed_managed_live_resume_card() -> WarmStartChoiceCard {
    let mut resume = lane(
        WarmStartPathClass::ResumeLiveWorkspace,
        WarmStartPathClass::ResumeLiveWorkspace.label(),
        "Re-attach to the running managed cloud workspace after re-authorizing.",
        WarmStartLaneAvailability::RequiresReauth,
        WarmStartSideEffectClass::ManagedAttach,
    );
    resume.requires_trust_grant = true;
    resume.requires_network = true;
    resume.materializes_remote_work = true;

    let snapshot_lane = lane(
        WarmStartPathClass::StartFromSnapshot,
        WarmStartPathClass::StartFromSnapshot.label(),
        "Open the cached read-only view of the last synced state. No managed attach.",
        WarmStartLaneAvailability::AvailableAfterReview,
        WarmStartSideEffectClass::LocalReadOnly,
    );

    let choice_lanes = vec![
        resume,
        snapshot_lane,
        open_minimal_lane(),
        set_up_later_lane(),
    ];
    let side_effects = WarmStartSideEffectSummary {
        network_egress: true,
        extension_installs: false,
        setup_tasks: false,
        trust_prompt: true,
        managed_or_remote_attach: true,
        notes: vec![
            "Resuming attaches a managed cloud runtime and requires re-authorization.".to_string(),
            "The cached snapshot view stays local and read-only.".to_string(),
        ],
    };
    let environment_starter = WarmStartEnvironmentStarter {
        setup_location_class: WarmStartSetupLocationClass::ManagedCloud,
        setup_location_token: WarmStartSetupLocationClass::ManagedCloud.as_str().to_string(),
        downloads_involved: true,
        extensions_involved: false,
        tasks_involved: true,
        trust_prompt_involved: true,
        bypass_route_ids: vec!["open_minimal".to_string(), "start_from_snapshot".to_string()],
        defer_route_ids: vec!["set_up_later".to_string()],
        summary: "Setup runs in the managed cloud and prompts for trust. Open the cached snapshot \
                  or local files to stay off the network."
            .to_string(),
    };
    let snapshot = Some(WarmStartSnapshotFacts {
        source_class: WarmStartSourceClass::PrebuildSnapshot,
        source_class_token: WarmStartSourceClass::PrebuildSnapshot.as_str().to_string(),
        fingerprint_ref: "sha256:managed-data-workspace-cache-0c7a".to_string(),
        freshness: WarmStartFreshnessClass::Cached,
        freshness_token: WarmStartFreshnessClass::Cached.as_str().to_string(),
        age_class: WarmStartAgeClass::WithinDays,
        age_token: WarmStartAgeClass::WithinDays.as_str().to_string(),
        captured_at: "2026-05-18T09:10:00Z".to_string(),
        invalidation_reason: None,
        stale_must_not_render_as_live_resume: true,
    });
    finalize_card(WarmStartChoiceCardSeed {
        card_id: "warm_start_card:live_resume.managed_data_workspace",
        surface_origin: WarmStartSurfaceClass::WorkspaceSwitcher,
        display_label: "Managed data workspace",
        summary: "Resume a managed cloud workspace, or stay local with the cached snapshot.",
        source_class: WarmStartSourceClass::LiveWorkspace,
        support_class: WarmStartSupportClass::Limited,
        runtime_or_host_model: RuntimeHostModelClass::ManagedCloudWorkspace,
        local_first: false,
        expected_setup_actions: vec![
            "Re-authorize the managed workspace".to_string(),
            "Re-attach the managed cloud runtime".to_string(),
        ],
        snapshot,
        side_effects,
        environment_starter,
        choice_lanes,
        safest_next_action: WarmStartPathClass::OpenMinimal,
    })
}

fn seed_valid_snapshot_card() -> WarmStartChoiceCard {
    let resume = lane(
        WarmStartPathClass::ResumeLiveWorkspace,
        WarmStartPathClass::ResumeLiveWorkspace.label(),
        "Reopen the local workspace with its restored layout. Stays on this machine.",
        WarmStartLaneAvailability::Available,
        WarmStartSideEffectClass::LocalReadOnly,
    );
    let snapshot_lane = lane(
        WarmStartPathClass::StartFromSnapshot,
        WarmStartPathClass::StartFromSnapshot.label(),
        "Reuse the fresh local prebuild snapshot. No download, no setup.",
        WarmStartLaneAvailability::Available,
        WarmStartSideEffectClass::LocalReadOnly,
    );
    let mut clone_fresh = lane(
        WarmStartPathClass::CloneFresh,
        WarmStartPathClass::CloneFresh.label(),
        "Discard the snapshot and rebuild from the source. Fetches dependencies.",
        WarmStartLaneAvailability::AvailableAfterReview,
        WarmStartSideEffectClass::NetworkEgress,
    );
    clone_fresh.requires_network = true;
    clone_fresh.runs_setup_tasks = true;

    let choice_lanes = vec![
        resume,
        snapshot_lane,
        clone_fresh,
        open_minimal_lane(),
        set_up_later_lane(),
    ];
    let side_effects = WarmStartSideEffectSummary {
        network_egress: false,
        extension_installs: false,
        setup_tasks: false,
        trust_prompt: false,
        managed_or_remote_attach: false,
        notes: vec![
            "Snapshot reuse and resume stay local and read-only.".to_string(),
            "Only clone-fresh reaches the network.".to_string(),
        ],
    };
    let environment_starter = WarmStartEnvironmentStarter {
        setup_location_class: WarmStartSetupLocationClass::LocalHost,
        setup_location_token: WarmStartSetupLocationClass::LocalHost.as_str().to_string(),
        downloads_involved: false,
        extensions_involved: false,
        tasks_involved: false,
        trust_prompt_involved: false,
        bypass_route_ids: vec!["open_minimal".to_string()],
        defer_route_ids: vec!["set_up_later".to_string()],
        summary: "No setup is required to reuse the snapshot. Clone-fresh is the only networked \
                  path."
            .to_string(),
    };
    let snapshot = Some(WarmStartSnapshotFacts {
        source_class: WarmStartSourceClass::PrebuildSnapshot,
        source_class_token: WarmStartSourceClass::PrebuildSnapshot.as_str().to_string(),
        fingerprint_ref: "sha256:ts-web-local-dependency-cache-9f31".to_string(),
        freshness: WarmStartFreshnessClass::Fresh,
        freshness_token: WarmStartFreshnessClass::Fresh.as_str().to_string(),
        age_class: WarmStartAgeClass::WithinHours,
        age_token: WarmStartAgeClass::WithinHours.as_str().to_string(),
        captured_at: "2026-05-20T06:30:00Z".to_string(),
        invalidation_reason: None,
        stale_must_not_render_as_live_resume: true,
    });
    finalize_card(WarmStartChoiceCardSeed {
        card_id: "warm_start_card:snapshot.ts_web.local_fresh",
        surface_origin: WarmStartSurfaceClass::StartCenter,
        display_label: "Web client (recent snapshot)",
        summary: "Resume the local web client, reuse the fresh snapshot, or rebuild from source.",
        source_class: WarmStartSourceClass::PrebuildSnapshot,
        support_class: WarmStartSupportClass::Supported,
        runtime_or_host_model: RuntimeHostModelClass::LocalHost,
        local_first: true,
        expected_setup_actions: vec![
            "Reuse the cached dependency snapshot".to_string(),
            "Restore the saved editor layout".to_string(),
        ],
        snapshot,
        side_effects,
        environment_starter,
        choice_lanes,
        safest_next_action: WarmStartPathClass::OpenMinimal,
    })
}

fn seed_stale_snapshot_card() -> WarmStartChoiceCard {
    let mut resume = lane(
        WarmStartPathClass::ResumeLiveWorkspace,
        WarmStartPathClass::ResumeLiveWorkspace.label(),
        "Unavailable: the snapshot drifted from the current environment capsule.",
        WarmStartLaneAvailability::UnavailableStaleSnapshot,
        WarmStartSideEffectClass::LocalReadOnly,
    );
    resume.invalidation_reason = Some("capsule_drift".to_string());

    let mut snapshot_lane = lane(
        WarmStartPathClass::StartFromSnapshot,
        WarmStartPathClass::StartFromSnapshot.label(),
        "Open the stale snapshot read-only for inspection. It will not be treated as live.",
        WarmStartLaneAvailability::AvailableAfterReview,
        WarmStartSideEffectClass::LocalReadOnly,
    );
    snapshot_lane.invalidation_reason = Some("capsule_drift".to_string());

    let mut clone_fresh = lane(
        WarmStartPathClass::CloneFresh,
        WarmStartPathClass::CloneFresh.label(),
        "Rebuild the dev container from source to clear the drift. Fetches dependencies.",
        WarmStartLaneAvailability::AvailableAfterReview,
        WarmStartSideEffectClass::NetworkEgress,
    );
    clone_fresh.requires_network = true;
    clone_fresh.runs_setup_tasks = true;

    let choice_lanes = vec![
        resume,
        snapshot_lane,
        clone_fresh,
        open_minimal_lane(),
        set_up_later_lane(),
    ];
    let side_effects = WarmStartSideEffectSummary {
        network_egress: false,
        extension_installs: false,
        setup_tasks: false,
        trust_prompt: false,
        managed_or_remote_attach: false,
        notes: vec![
            "The snapshot is stale; live resume is disabled.".to_string(),
            "Inspect read-only or rebuild from source.".to_string(),
        ],
    };
    let environment_starter = WarmStartEnvironmentStarter {
        setup_location_class: WarmStartSetupLocationClass::Devcontainer,
        setup_location_token: WarmStartSetupLocationClass::Devcontainer.as_str().to_string(),
        downloads_involved: true,
        extensions_involved: false,
        tasks_involved: true,
        trust_prompt_involved: false,
        bypass_route_ids: vec!["open_minimal".to_string(), "start_from_snapshot".to_string()],
        defer_route_ids: vec!["set_up_later".to_string()],
        summary: "Rebuild runs in the dev container. Inspect the snapshot or open local files \
                  without rebuilding."
            .to_string(),
    };
    let snapshot = Some(WarmStartSnapshotFacts {
        source_class: WarmStartSourceClass::PrebuildSnapshot,
        source_class_token: WarmStartSourceClass::PrebuildSnapshot.as_str().to_string(),
        fingerprint_ref: "sha256:python-devcontainer-snapshot-4b88".to_string(),
        freshness: WarmStartFreshnessClass::Stale,
        freshness_token: WarmStartFreshnessClass::Stale.as_str().to_string(),
        age_class: WarmStartAgeClass::BeyondReviewWindow,
        age_token: WarmStartAgeClass::BeyondReviewWindow.as_str().to_string(),
        captured_at: "2026-04-02T11:45:00Z".to_string(),
        invalidation_reason: Some("capsule_drift".to_string()),
        stale_must_not_render_as_live_resume: true,
    });
    finalize_card(WarmStartChoiceCardSeed {
        card_id: "warm_start_card:snapshot.python_devcontainer.stale",
        surface_origin: WarmStartSurfaceClass::StartCenter,
        display_label: "Python service (stale snapshot)",
        summary: "The snapshot is stale. Inspect it read-only or rebuild the dev container.",
        source_class: WarmStartSourceClass::PrebuildSnapshot,
        support_class: WarmStartSupportClass::Community,
        runtime_or_host_model: RuntimeHostModelClass::Devcontainer,
        local_first: true,
        expected_setup_actions: vec![
            "Rebuild the dev container image".to_string(),
            "Reinstall dependencies inside the container".to_string(),
        ],
        snapshot,
        side_effects,
        environment_starter,
        choice_lanes,
        safest_next_action: WarmStartPathClass::OpenMinimal,
    })
}

fn seed_clone_fresh_card() -> WarmStartChoiceCard {
    let mut clone_fresh = lane(
        WarmStartPathClass::CloneFresh,
        WarmStartPathClass::CloneFresh.label(),
        "Clone the remote repository into a new local workspace, then review before setup.",
        WarmStartLaneAvailability::AvailableAfterReview,
        WarmStartSideEffectClass::NetworkEgress,
    );
    clone_fresh.requires_network = true;

    let choice_lanes = vec![clone_fresh, open_minimal_lane(), set_up_later_lane()];
    let side_effects = WarmStartSideEffectSummary {
        network_egress: true,
        extension_installs: false,
        setup_tasks: false,
        trust_prompt: false,
        managed_or_remote_attach: false,
        notes: vec![
            "Clone fetches the repository over the network; setup is reviewed afterward."
                .to_string(),
        ],
    };
    let environment_starter = WarmStartEnvironmentStarter {
        setup_location_class: WarmStartSetupLocationClass::LocalHost,
        setup_location_token: WarmStartSetupLocationClass::LocalHost.as_str().to_string(),
        downloads_involved: true,
        extensions_involved: false,
        tasks_involved: false,
        trust_prompt_involved: false,
        bypass_route_ids: vec!["open_minimal".to_string()],
        defer_route_ids: vec!["set_up_later".to_string()],
        summary: "Clone downloads to your local host. Open an existing local folder instead to \
                  stay offline."
            .to_string(),
    };
    finalize_card(WarmStartChoiceCardSeed {
        card_id: "warm_start_card:clone_fresh.platform_repository",
        surface_origin: WarmStartSurfaceClass::StartCenter,
        display_label: "Platform repository",
        summary: "Clone a remote repository fresh into a local workspace, or open local files.",
        source_class: WarmStartSourceClass::RemoteRepository,
        support_class: WarmStartSupportClass::Supported,
        runtime_or_host_model: RuntimeHostModelClass::LocalHost,
        local_first: true,
        expected_setup_actions: vec![
            "Clone the repository to the chosen destination".to_string(),
            "Review the entry before any setup runs".to_string(),
        ],
        snapshot: None,
        side_effects,
        environment_starter,
        choice_lanes,
        safest_next_action: WarmStartPathClass::OpenMinimal,
    })
}

struct WarmStartChoiceCardSeed {
    card_id: &'static str,
    surface_origin: WarmStartSurfaceClass,
    display_label: &'static str,
    summary: &'static str,
    source_class: WarmStartSourceClass,
    support_class: WarmStartSupportClass,
    runtime_or_host_model: RuntimeHostModelClass,
    local_first: bool,
    expected_setup_actions: Vec<String>,
    snapshot: Option<WarmStartSnapshotFacts>,
    side_effects: WarmStartSideEffectSummary,
    environment_starter: WarmStartEnvironmentStarter,
    choice_lanes: Vec<WarmStartChoiceLane>,
    safest_next_action: WarmStartPathClass,
}

fn finalize_card(seed: WarmStartChoiceCardSeed) -> WarmStartChoiceCard {
    let honesty_marker_present = card_honesty_marker(&seed.snapshot, &seed.choice_lanes);
    WarmStartChoiceCard {
        record_kind: WARM_START_CHOICE_CARD_RECORD_KIND.to_string(),
        schema_version: WARM_START_CHOICE_SCHEMA_VERSION,
        shared_contract_ref: WARM_START_CHOICE_SHARED_CONTRACT_REF.to_string(),
        card_id: seed.card_id.to_string(),
        surface_origin: seed.surface_origin,
        surface_origin_token: seed.surface_origin.as_str().to_string(),
        display_label: seed.display_label.to_string(),
        summary: seed.summary.to_string(),
        source_class: seed.source_class,
        source_class_token: seed.source_class.as_str().to_string(),
        support_class: seed.support_class,
        support_class_token: seed.support_class.as_str().to_string(),
        runtime_or_host_model: seed.runtime_or_host_model,
        runtime_or_host_model_token: seed.runtime_or_host_model.as_str().to_string(),
        local_first: seed.local_first,
        expected_setup_actions: seed.expected_setup_actions,
        snapshot: seed.snapshot,
        side_effects: seed.side_effects,
        environment_starter: seed.environment_starter,
        choice_lanes: seed.choice_lanes,
        safest_next_action: seed.safest_next_action,
        safest_next_action_token: seed.safest_next_action.as_str().to_string(),
        default_widens_trust: false,
        default_runs_networked_work: false,
        honesty_marker_present,
    }
}

fn card_honesty_marker(
    snapshot: &Option<WarmStartSnapshotFacts>,
    lanes: &[WarmStartChoiceLane],
) -> bool {
    let stale_snapshot = snapshot
        .as_ref()
        .map(|snapshot| snapshot.freshness.is_stale_or_invalidated())
        .unwrap_or(false);
    let gated_lane = lanes
        .iter()
        .any(|lane| lane.availability.is_honest_warning());
    stale_snapshot || gated_lane
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validates the warm-start choice page invariants. Returns the full list of
/// findings so a reviewer or headless inspector can see every problem at once.
pub fn validate_warm_start_choice_page(page: &WarmStartChoicePage) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if page.record_kind != WARM_START_CHOICE_PAGE_RECORD_KIND {
        errors.push("page.record_kind.invalid".to_string());
    }
    if page.schema_version != WARM_START_CHOICE_SCHEMA_VERSION {
        errors.push("page.schema_version.invalid".to_string());
    }
    if page.shared_contract_ref != WARM_START_CHOICE_SHARED_CONTRACT_REF {
        errors.push("page.shared_contract_ref.invalid".to_string());
    }
    if page.notice != WARM_START_CHOICE_NOTICE {
        errors.push("page.notice.invalid".to_string());
    }
    for required in WARM_START_CHOICE_CONSUMING_SURFACES {
        if !page
            .consuming_surfaces
            .iter()
            .any(|surface| surface == required)
        {
            errors.push(format!("page.consuming_surfaces.{required}.missing"));
        }
    }
    if page.cards.is_empty() {
        errors.push("page.cards.empty".to_string());
    }

    for card in &page.cards {
        validate_card(card, &mut errors);
    }

    validate_summary(page, &mut errors);

    let any_card_marked = page.cards.iter().any(|card| card.honesty_marker_present);
    if page.honesty_marker_present != any_card_marked {
        errors.push("page.honesty_marker.inconsistent".to_string());
    }

    let open_minimal_ok = page
        .cards
        .iter()
        .filter(|card| card.local_first)
        .all(|card| lane_same_weight(card, WarmStartPathClass::OpenMinimal));
    if page.open_minimal_same_weight_on_local_first != open_minimal_ok {
        errors.push("page.open_minimal_same_weight.inconsistent".to_string());
    }
    if !page.open_minimal_same_weight_on_local_first {
        errors.push("page.open_minimal_same_weight.not_held".to_string());
    }
    let set_up_later_ok = page
        .cards
        .iter()
        .filter(|card| card.local_first)
        .all(|card| lane_same_weight(card, WarmStartPathClass::SetUpLater));
    if page.set_up_later_same_weight_on_local_first != set_up_later_ok {
        errors.push("page.set_up_later_same_weight.inconsistent".to_string());
    }
    if !page.set_up_later_same_weight_on_local_first {
        errors.push("page.set_up_later_same_weight.not_held".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_card(card: &WarmStartChoiceCard, errors: &mut Vec<String>) {
    let id = &card.card_id;

    if card.record_kind != WARM_START_CHOICE_CARD_RECORD_KIND {
        errors.push(format!("card.{id}.record_kind.invalid"));
    }
    if card.schema_version != WARM_START_CHOICE_SCHEMA_VERSION {
        errors.push(format!("card.{id}.schema_version.invalid"));
    }
    if card.shared_contract_ref != WARM_START_CHOICE_SHARED_CONTRACT_REF {
        errors.push(format!("card.{id}.shared_contract_ref.invalid"));
    }
    if card.display_label.trim().is_empty() {
        errors.push(format!("card.{id}.display_label.empty"));
    }
    if card.surface_origin_token != card.surface_origin.as_str() {
        errors.push(format!("card.{id}.surface_origin_token.drift"));
    }
    if card.source_class_token != card.source_class.as_str() {
        errors.push(format!("card.{id}.source_class_token.drift"));
    }
    if card.support_class_token != card.support_class.as_str() {
        errors.push(format!("card.{id}.support_class_token.drift"));
    }
    if card.runtime_or_host_model_token != card.runtime_or_host_model.as_str() {
        errors.push(format!("card.{id}.runtime_or_host_model_token.drift"));
    }
    if card.safest_next_action_token != card.safest_next_action.as_str() {
        errors.push(format!("card.{id}.safest_next_action_token.drift"));
    }
    if card.source_class == WarmStartSourceClass::UnknownSource {
        errors.push(format!("card.{id}.source_class.unknown"));
    }
    if card.support_class == WarmStartSupportClass::UnknownSupport {
        errors.push(format!("card.{id}.support_class.unknown"));
    }
    if card.runtime_or_host_model == RuntimeHostModelClass::UnknownRuntimeHostModel {
        errors.push(format!("card.{id}.runtime_or_host_model.unknown"));
    }

    if card.choice_lanes.is_empty() {
        errors.push(format!("card.{id}.choice_lanes.empty"));
    }

    // The default must never widen trust or run networked work.
    if card.default_widens_trust {
        errors.push(format!("card.{id}.default_widens_trust"));
    }
    if card.default_runs_networked_work {
        errors.push(format!("card.{id}.default_runs_networked_work"));
    }

    // The safest_next_action must resolve to a local-safe lane.
    match card.lane(card.safest_next_action) {
        None => errors.push(format!("card.{id}.safest_next_action.lane_missing")),
        Some(lane) => {
            if !lane.is_local_safe() {
                errors.push(format!("card.{id}.safest_next_action.not_local_safe"));
            }
        }
    }

    // At least one local-safe lane must exist.
    if !card.choice_lanes.iter().any(WarmStartChoiceLane::is_local_safe) {
        errors.push(format!("card.{id}.no_local_safe_lane"));
    }

    for lane in &card.choice_lanes {
        validate_lane(card, lane, errors);
    }

    validate_snapshot_consistency(card, errors);
    validate_environment_starter(card, errors);

    // Local-first cards must keep both escape hatches at same weight.
    if card.local_first {
        for path in [WarmStartPathClass::OpenMinimal, WarmStartPathClass::SetUpLater] {
            match card.lane(path) {
                None => errors.push(format!("card.{id}.local_first.{}.missing", path.as_str())),
                Some(lane) => {
                    if !lane.same_weight_local_path {
                        errors.push(format!(
                            "card.{id}.local_first.{}.not_same_weight",
                            path.as_str()
                        ));
                    }
                    if !lane.is_local_safe() {
                        errors.push(format!(
                            "card.{id}.local_first.{}.not_local_safe",
                            path.as_str()
                        ));
                    }
                }
            }
        }
    }

    // Honesty marker must match the derived value.
    let expected_marker = card_honesty_marker(&card.snapshot, &card.choice_lanes);
    if card.honesty_marker_present != expected_marker {
        errors.push(format!("card.{id}.honesty_marker.inconsistent"));
    }
}

fn validate_lane(card: &WarmStartChoiceCard, lane: &WarmStartChoiceLane, errors: &mut Vec<String>) {
    let id = &card.card_id;
    let path = lane.path_token.as_str();

    if lane.path_token != lane.path_class.as_str() {
        errors.push(format!("card.{id}.lane.{path}.path_token.drift"));
    }
    if lane.availability_token != lane.availability.as_str() {
        errors.push(format!("card.{id}.lane.{path}.availability_token.drift"));
    }
    if lane.side_effect_token != lane.side_effect_class.as_str() {
        errors.push(format!("card.{id}.lane.{path}.side_effect_token.drift"));
    }
    if lane.path_class == WarmStartPathClass::UnknownPath {
        errors.push(format!("card.{id}.lane.{path}.path.unknown"));
    }

    // A remote/widening lane cannot look like a plain local open.
    if lane.is_remote_or_widening() {
        if lane.side_effect_class.is_local_safe() {
            errors.push(format!("card.{id}.lane.{path}.remote_masquerades_as_local"));
        }
        if lane.path_class.is_local_escape_hatch() {
            errors.push(format!("card.{id}.lane.{path}.escape_hatch_has_side_effect"));
        }
        if lane.same_weight_local_path {
            errors.push(format!("card.{id}.lane.{path}.remote_marked_same_weight"));
        }
    }

    // Escape-hatch lanes must stay local-safe and same-weight.
    if lane.path_class.is_local_escape_hatch() {
        if !lane.is_local_safe() {
            errors.push(format!("card.{id}.lane.{path}.escape_hatch_not_local_safe"));
        }
        if !lane.same_weight_local_path {
            errors.push(format!("card.{id}.lane.{path}.escape_hatch_not_same_weight"));
        }
    }
}

fn validate_snapshot_consistency(card: &WarmStartChoiceCard, errors: &mut Vec<String>) {
    let id = &card.card_id;
    let resume_lane = card.lane(WarmStartPathClass::ResumeLiveWorkspace);
    let snapshot_lane = card.lane(WarmStartPathClass::StartFromSnapshot);

    match &card.snapshot {
        Some(snapshot) => {
            // Every card with a snapshot must offer the snapshot lane.
            if snapshot_lane.is_none() {
                errors.push(format!("card.{id}.snapshot.start_from_snapshot_missing"));
            }
            if snapshot.source_class_token != snapshot.source_class.as_str() {
                errors.push(format!("card.{id}.snapshot.source_class_token.drift"));
            }
            if snapshot.freshness_token != snapshot.freshness.as_str() {
                errors.push(format!("card.{id}.snapshot.freshness_token.drift"));
            }
            if snapshot.age_token != snapshot.age_class.as_str() {
                errors.push(format!("card.{id}.snapshot.age_token.drift"));
            }
            if snapshot.fingerprint_ref.trim().is_empty() {
                errors.push(format!("card.{id}.snapshot.fingerprint_ref.empty"));
            }
            if !snapshot.stale_must_not_render_as_live_resume {
                errors.push(format!("card.{id}.snapshot.live_resume_guard_off"));
            }
            // A stale/invalidated snapshot must name a reason and must not back
            // an available live resume.
            if snapshot.freshness.is_stale_or_invalidated() {
                if snapshot.invalidation_reason.is_none() {
                    errors.push(format!("card.{id}.snapshot.invalidation_reason.missing"));
                }
                if let Some(resume) = resume_lane {
                    if resume.availability.is_takeable() {
                        errors.push(format!("card.{id}.snapshot.stale_resume_still_takeable"));
                    }
                }
            }
        }
        None => {
            if snapshot_lane.is_some() {
                errors.push(format!("card.{id}.start_from_snapshot_without_snapshot"));
            }
        }
    }
}

fn validate_environment_starter(card: &WarmStartChoiceCard, errors: &mut Vec<String>) {
    let id = &card.card_id;
    let starter = &card.environment_starter;

    if starter.setup_location_token != starter.setup_location_class.as_str() {
        errors.push(format!("card.{id}.environment_starter.location_token.drift"));
    }

    // A disclosed trust prompt on the card must be reflected by the starter.
    if card.side_effects.trust_prompt && !starter.trust_prompt_involved {
        errors.push(format!("card.{id}.environment_starter.trust_prompt_undisclosed"));
    }
    // A managed/remote attach implies the side-effect summary names it.
    let attaches = card
        .choice_lanes
        .iter()
        .any(|lane| lane.materializes_remote_work);
    if attaches && !card.side_effects.managed_or_remote_attach {
        errors.push(format!("card.{id}.side_effects.attach_undisclosed"));
    }
    // Whenever a starter runs setup somewhere, it must offer bypass and defer
    // routes so the user can decline.
    if starter.setup_location_class != WarmStartSetupLocationClass::NoSetup {
        if starter.bypass_route_ids.is_empty() {
            errors.push(format!("card.{id}.environment_starter.bypass_missing"));
        }
        if starter.defer_route_ids.is_empty() {
            errors.push(format!("card.{id}.environment_starter.defer_missing"));
        }
    }
}

fn validate_summary(page: &WarmStartChoicePage, errors: &mut Vec<String>) {
    let expected = compute_summary(&page.cards);
    if page.summary != expected {
        errors.push("page.summary.drift".to_string());
    }
}

/// Validates the support-export wrapper against its embedded page.
pub fn validate_warm_start_choice_support_export(
    export: &WarmStartChoiceSupportExport,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if export.record_kind != WARM_START_CHOICE_SUPPORT_EXPORT_RECORD_KIND {
        errors.push("support_export.record_kind.invalid".to_string());
    }
    if export.schema_version != WARM_START_CHOICE_SCHEMA_VERSION {
        errors.push("support_export.schema_version.invalid".to_string());
    }
    if export.shared_contract_ref != WARM_START_CHOICE_SHARED_CONTRACT_REF {
        errors.push("support_export.shared_contract_ref.invalid".to_string());
    }
    if !export.raw_secret_material_excluded {
        errors.push("support_export.raw_secret_material_present".to_string());
    }
    let expected_ids: Vec<String> = export
        .page
        .cards
        .iter()
        .map(|card| card.card_id.clone())
        .collect();
    if export.card_ids != expected_ids {
        errors.push("support_export.card_ids.drift_from_page".to_string());
    }
    if let Err(page_errors) = validate_warm_start_choice_page(&export.page) {
        errors.extend(page_errors.into_iter().map(|error| format!("page.{error}")));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// ---------------------------------------------------------------------------
// Plaintext rendering
// ---------------------------------------------------------------------------

/// Renders a deterministic plaintext block for CLI/headless/docs/support
/// consumers. Stable for the same input page.
pub fn render_warm_start_choice_plaintext(page: &WarmStartChoicePage) -> String {
    let mut out = String::new();
    out.push_str("Warm-start choices beta\n");
    out.push_str(&format!("Page: {}\n", page.page_label));
    out.push_str(&format!("Contract: {}\n", page.contract_ref));
    out.push_str(&format!(
        "Surfaces: {}\n",
        page.consuming_surfaces.join(", ")
    ));
    out.push_str(&format!(
        "Honesty marker: {}\n",
        if page.honesty_marker_present {
            "present"
        } else {
            "none"
        }
    ));
    out.push_str(&format!(
        "Summary: cards={} templates={} live_resume={} snapshots={} clone_fresh={} local_first={} stale_snapshots={} revalidation_required={}\n\n",
        page.summary.total_card_count,
        page.summary.template_card_count,
        page.summary.live_resume_card_count,
        page.summary.snapshot_card_count,
        page.summary.clone_fresh_card_count,
        page.summary.local_first_card_count,
        page.summary.stale_snapshot_card_count,
        page.summary.revalidation_required_card_count,
    ));

    for card in &page.cards {
        out.push_str(&format!("- {} [{}]\n", card.card_id, card.surface_origin_token));
        out.push_str(&format!("    label: {}\n", card.display_label));
        out.push_str(&format!(
            "    source: {} | support: {} | runtime: {} | local_first: {}\n",
            card.source_class_token,
            card.support_class_token,
            card.runtime_or_host_model_token,
            card.local_first,
        ));
        out.push_str(&format!(
            "    safest next action: {}\n",
            card.safest_next_action_token,
        ));
        if let Some(snapshot) = &card.snapshot {
            out.push_str(&format!(
                "    snapshot: {} ({}) age={} fingerprint={} captured={}\n",
                snapshot.freshness_token,
                snapshot.source_class_token,
                snapshot.age_token,
                snapshot.fingerprint_ref,
                snapshot.captured_at,
            ));
            if let Some(reason) = &snapshot.invalidation_reason {
                out.push_str(&format!("    snapshot invalidation: {reason}\n"));
            }
        }
        out.push_str(&format!(
            "    side effects: network={} extensions={} setup_tasks={} trust_prompt={} managed_attach={}\n",
            card.side_effects.network_egress,
            card.side_effects.extension_installs,
            card.side_effects.setup_tasks,
            card.side_effects.trust_prompt,
            card.side_effects.managed_or_remote_attach,
        ));
        out.push_str(&format!(
            "    environment starter: location={} downloads={} extensions={} tasks={} trust_prompt={} bypass=[{}] defer=[{}]\n",
            card.environment_starter.setup_location_token,
            card.environment_starter.downloads_involved,
            card.environment_starter.extensions_involved,
            card.environment_starter.tasks_involved,
            card.environment_starter.trust_prompt_involved,
            card.environment_starter.bypass_route_ids.join(","),
            card.environment_starter.defer_route_ids.join(","),
        ));
        for lane in &card.choice_lanes {
            out.push_str(&format!(
                "      lane {} -> {} [{}] network={} trust={} setup_tasks={} remote={}{}\n",
                lane.path_token,
                lane.availability_token,
                lane.side_effect_token,
                lane.requires_network,
                lane.requires_trust_grant,
                lane.runs_setup_tasks,
                lane.materializes_remote_work,
                lane.invalidation_reason
                    .as_ref()
                    .map(|reason| format!(" invalidation={reason}"))
                    .unwrap_or_default(),
            ));
        }
        if card.honesty_marker_present {
            out.push_str("    honesty marker: present\n");
        }
    }

    out
}

/// Returns the full closed vocabulary the surface uses, grouped by dimension.
/// Headless consumers print this to confirm the lane taxonomy without reading
/// source.
pub fn warm_start_choice_vocabulary() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (
            "path_class",
            vec![
                WarmStartPathClass::ResumeLiveWorkspace.as_str(),
                WarmStartPathClass::StartFromSnapshot.as_str(),
                WarmStartPathClass::CloneFresh.as_str(),
                WarmStartPathClass::OpenMinimal.as_str(),
                WarmStartPathClass::SetUpLater.as_str(),
                WarmStartPathClass::UseTemplate.as_str(),
            ],
        ),
        (
            "source_class",
            vec![
                WarmStartSourceClass::WorkspaceTemplate.as_str(),
                WarmStartSourceClass::PrebuildSnapshot.as_str(),
                WarmStartSourceClass::LiveWorkspace.as_str(),
                WarmStartSourceClass::RemoteRepository.as_str(),
                WarmStartSourceClass::LocalFolder.as_str(),
            ],
        ),
        (
            "availability",
            vec![
                WarmStartLaneAvailability::Available.as_str(),
                WarmStartLaneAvailability::AvailableAfterReview.as_str(),
                WarmStartLaneAvailability::RequiresReauth.as_str(),
                WarmStartLaneAvailability::UnavailableStaleSnapshot.as_str(),
                WarmStartLaneAvailability::BlockedByPolicy.as_str(),
            ],
        ),
        (
            "side_effect_class",
            vec![
                WarmStartSideEffectClass::NoSideEffect.as_str(),
                WarmStartSideEffectClass::LocalReadOnly.as_str(),
                WarmStartSideEffectClass::LocalSetup.as_str(),
                WarmStartSideEffectClass::NetworkEgress.as_str(),
                WarmStartSideEffectClass::ManagedAttach.as_str(),
                WarmStartSideEffectClass::TrustWidening.as_str(),
            ],
        ),
        (
            "freshness",
            vec![
                WarmStartFreshnessClass::Fresh.as_str(),
                WarmStartFreshnessClass::Cached.as_str(),
                WarmStartFreshnessClass::Stale.as_str(),
                WarmStartFreshnessClass::Invalidated.as_str(),
                WarmStartFreshnessClass::Unverified.as_str(),
            ],
        ),
        (
            "setup_location_class",
            vec![
                WarmStartSetupLocationClass::NoSetup.as_str(),
                WarmStartSetupLocationClass::LocalHost.as_str(),
                WarmStartSetupLocationClass::Devcontainer.as_str(),
                WarmStartSetupLocationClass::ManagedCloud.as_str(),
                WarmStartSetupLocationClass::SshHost.as_str(),
            ],
        ),
    ]
}

#[cfg(test)]
mod tests;
