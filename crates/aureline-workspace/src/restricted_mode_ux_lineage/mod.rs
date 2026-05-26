//! Restricted-mode UX lineage: the governed, export-safe projection
//! that finalizes the user-facing restricted-mode experience and the
//! explainability surfaces that ride on top of workspace-trust gating.
//!
//! Where the trust-gating lineage proves which privileged surfaces are
//! gated and why, this projection proves what the user sees when they
//! land in a restricted (or pending) workspace: which UX surfaces show
//! the restriction, which named explanation each one carries, which
//! escape paths are offered without silent commitments, which read-only
//! affordances stay reachable, which accessibility postures are honored,
//! and which stability tier each surface is allowed to claim on the
//! release branch.
//!
//! The projection ingests a live [`RestrictedModeUxInputs`] envelope
//! verbatim (one [`RestrictedModeSurfaceObservation`] per restricted-mode
//! UX surface plus the controlled inspection-hook table) and produces a
//! lineage record that proves the contract claims the stable line is
//! anchored on:
//!
//! - **Surface coverage truth.** Every restricted-mode UX surface that
//!   ships a restriction message is bound to one closed
//!   [`RestrictedModeSurfaceKind`] (status bar, editor chrome, command
//!   palette, action menu, Help/About, support export), and the corpus
//!   seeds one row per kind so the user never lands on a restricted
//!   workspace surface that hides the restriction entirely.
//! - **Explainability truth.** Every surface declares a named
//!   [`RestrictionReasonClass`] and references a stable `explanation_id`
//!   so the user can pivot to the restriction explanation from any
//!   surface without re-routing through the trust grant flow first.
//! - **Escape-path honesty.** Every surface declares an
//!   [`EscapePathClass`] drawn from a closed vocabulary
//!   (`grant_trust`, `repair_workspace`, `leave_workspace`,
//!   `stay_read_only`, `contact_support`). A `grant_trust` escape path
//!   must reference an explicit action id and a disclosure id so it
//!   cannot silently commit to widening trust.
//! - **Read-only affordance truth.** Surfaces that claim
//!   [`ClaimedStableTier::StableReadOnly`] declare only inspect-class
//!   affordances and explicitly exclude mutation, execution, and
//!   exfiltration affordances. The projection re-derives the affordance
//!   posture so a read-only claim that exposes mutation narrows.
//! - **Claimed-tier truth.** Each surface declares one
//!   [`ClaimedStableTier`]; the projection re-derives the worst-case
//!   tier from the captured restriction posture so a `stable_full`
//!   claim cannot ride out of a restricted workspace.
//! - **Accessibility truth.** Every surface declares whether it ships
//!   the five required accessibility postures (keyboard, screen reader,
//!   IME / grapheme / bidi, zoom / high contrast, reduced motion). A
//!   surface missing any required posture narrows below Stable.
//! - **Support-export honesty.** Each surface's support-export
//!   projection preserves the surface kind, restriction reason,
//!   explanation id, escape path, claimed tier, and accessibility
//!   posture while excluding raw secrets, approval tickets, delegated
//!   credentials, and live authority handles.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks
//!   (`inspect_restriction`, `review_escape_path`,
//!   `compare_unrestricted`, `rollback_grant`, `export`, `repair`) is
//!   reachable so destructive grants and tier-widening commits stay
//!   reviewable.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to the
//!   source corpus, workspace, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`RestrictedModeUxLineageRecord`].
pub const RESTRICTED_MODE_UX_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the restricted-mode UX lineage record.
pub const RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/restricted_mode_ux_lineage.schema.json";

/// Stable record-kind tag for the restricted-mode UX lineage record.
pub const RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND: &str = "restricted_mode_ux_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the restricted-mode UX surfaces that ship a
/// user-facing restriction message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModeSurfaceKind {
    /// The workspace trust status indicator in the shell chrome.
    StatusBar,
    /// The editor chrome banner shown above the active document.
    EditorChrome,
    /// The command palette restricted-action filter row.
    CommandPalette,
    /// The action menu on a privileged surface (tasks / terminal /
    /// debug / AI apply / privileged extensions).
    ActionMenu,
    /// The Help / About panel explanation entry for restricted mode.
    HelpAbout,
    /// The support-export projection of the restricted-mode UX state.
    SupportExport,
}

impl RestrictedModeSurfaceKind {
    /// Returns the stable snake_case token for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusBar => "status_bar",
            Self::EditorChrome => "editor_chrome",
            Self::CommandPalette => "command_palette",
            Self::ActionMenu => "action_menu",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
        }
    }
}

/// Closed list of restricted-mode UX surfaces every restricted-mode
/// projection must cover.
pub const REQUIRED_RESTRICTED_MODE_SURFACES: [RestrictedModeSurfaceKind; 6] = [
    RestrictedModeSurfaceKind::StatusBar,
    RestrictedModeSurfaceKind::EditorChrome,
    RestrictedModeSurfaceKind::CommandPalette,
    RestrictedModeSurfaceKind::ActionMenu,
    RestrictedModeSurfaceKind::HelpAbout,
    RestrictedModeSurfaceKind::SupportExport,
];

/// Closed workspace restriction-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModePosture {
    /// Workspace is fully trusted; restricted-mode UX surfaces are
    /// dormant.
    Trusted,
    /// Workspace is restricted; restricted-mode UX surfaces are active
    /// and must explain the restriction.
    Restricted,
    /// Trust decision is pending; restricted-mode UX surfaces are
    /// active and must explain the pending state.
    PendingEvaluation,
}

impl RestrictedModePosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
        }
    }

    /// True when the workspace is in a restricted-mode UX posture.
    pub const fn is_restricted_mode(self) -> bool {
        matches!(self, Self::Restricted | Self::PendingEvaluation)
    }
}

/// Closed restriction-reason vocabulary explaining why a surface is
/// restricted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictionReasonClass {
    /// The workspace itself is restricted (no trust grant in effect).
    WorkspaceRestricted,
    /// The workspace trust decision is pending.
    WorkspacePendingEvaluation,
    /// The surface is held in inspect-only / read-only mode.
    SurfaceReadOnly,
    /// The surface touches a credential store gated by policy.
    CredentialStoreRestricted,
    /// A policy explicitly blocks the action on this surface.
    PolicyBlock,
    /// The user landed in a non-durable entry flow (e.g. inspect-only
    /// staging) and the surface stays restricted until durable
    /// destination review.
    PostEntryStagingRestricted,
}

impl RestrictionReasonClass {
    /// Returns the stable snake_case token for this restriction reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRestricted => "workspace_restricted",
            Self::WorkspacePendingEvaluation => "workspace_pending_evaluation",
            Self::SurfaceReadOnly => "surface_read_only",
            Self::CredentialStoreRestricted => "credential_store_restricted",
            Self::PolicyBlock => "policy_block",
            Self::PostEntryStagingRestricted => "post_entry_staging_restricted",
        }
    }
}

/// Closed escape-path vocabulary: the named, disclosed paths a
/// restricted-mode UX surface offers the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscapePathClass {
    /// Open the workspace trust grant flow.
    GrantTrust,
    /// Open the repair sheet for a restricted workspace.
    RepairWorkspace,
    /// Leave the workspace without committing.
    LeaveWorkspace,
    /// Stay in restricted mode with read-only affordances.
    StayReadOnly,
    /// Contact support / export the restricted-mode lineage record.
    ContactSupport,
}

impl EscapePathClass {
    /// Returns the stable snake_case token for this escape path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantTrust => "grant_trust",
            Self::RepairWorkspace => "repair_workspace",
            Self::LeaveWorkspace => "leave_workspace",
            Self::StayReadOnly => "stay_read_only",
            Self::ContactSupport => "contact_support",
        }
    }

    /// True when the escape path commits to widening workspace trust.
    pub const fn widens_trust(self) -> bool {
        matches!(self, Self::GrantTrust)
    }
}

/// Closed restricted-mode affordance vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedAffordanceClass {
    /// Inspect-only: read content; no mutation, no execution.
    InspectOnly,
    /// Copy content to clipboard (read-only egress allowed).
    CopyToClipboard,
    /// Navigate-only: open document / pane references; no mutation.
    NavigateOnly,
    /// View diff: render a diff view; no apply.
    ViewDiffOnly,
    /// The action is blocked but the surface shows an explanation.
    BlockedWithExplanation,
    /// Read-only allow: the surface may execute in an inspect-only
    /// posture but cannot mutate workspace state.
    AllowReadOnlyNoMutation,
}

impl RestrictedAffordanceClass {
    /// Returns the stable snake_case token for this affordance class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::CopyToClipboard => "copy_to_clipboard",
            Self::NavigateOnly => "navigate_only",
            Self::ViewDiffOnly => "view_diff_only",
            Self::BlockedWithExplanation => "blocked_with_explanation",
            Self::AllowReadOnlyNoMutation => "allow_read_only_no_mutation",
        }
    }

    /// True when the affordance is safe to expose on a read-only-tier
    /// surface (no mutation, no execution, no exfiltration beyond
    /// disclosed clipboard egress).
    pub const fn is_read_only_safe(self) -> bool {
        matches!(
            self,
            Self::InspectOnly
                | Self::CopyToClipboard
                | Self::NavigateOnly
                | Self::ViewDiffOnly
                | Self::BlockedWithExplanation
                | Self::AllowReadOnlyNoMutation
        )
    }
}

/// Closed claimed-stable-tier vocabulary for a restricted-mode UX
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedStableTier {
    /// The surface claims stable behavior across trusted, restricted,
    /// and pending postures.
    StableFull,
    /// The surface claims stable behavior in a read-only-only posture.
    StableReadOnly,
    /// The surface explicitly narrows itself below Stable.
    NarrowedBelowStable,
}

impl ClaimedStableTier {
    /// Returns the stable snake_case token for this tier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableFull => "stable_full",
            Self::StableReadOnly => "stable_read_only",
            Self::NarrowedBelowStable => "narrowed_below_stable",
        }
    }
}

/// Closed accessibility-posture-class vocabulary for a restricted-mode
/// UX surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityPostureClass {
    /// Keyboard-only navigation is reachable.
    KeyboardOnly,
    /// Screen-reader labels and ARIA roles are wired.
    ScreenReader,
    /// IME, grapheme cluster, and bidi text behave correctly.
    ImeGraphemeBidi,
    /// Zoom and high-contrast themes preserve legibility.
    ZoomHighContrast,
    /// Reduced-motion preferences are honored.
    ReducedMotion,
}

impl AccessibilityPostureClass {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardOnly => "keyboard_only",
            Self::ScreenReader => "screen_reader",
            Self::ImeGraphemeBidi => "ime_grapheme_bidi",
            Self::ZoomHighContrast => "zoom_high_contrast",
            Self::ReducedMotion => "reduced_motion",
        }
    }
}

/// Closed list of accessibility postures every restricted-mode UX
/// surface must support before it can claim Stable.
pub const REQUIRED_ACCESSIBILITY_POSTURES: [AccessibilityPostureClass; 5] = [
    AccessibilityPostureClass::KeyboardOnly,
    AccessibilityPostureClass::ScreenReader,
    AccessibilityPostureClass::ImeGraphemeBidi,
    AccessibilityPostureClass::ZoomHighContrast,
    AccessibilityPostureClass::ReducedMotion,
];

/// Closed support-export-posture vocabulary (mirrors the workspace
/// state-package vocabulary so support bundles can share posture
/// classifications across lineages).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModeSupportExportPosture {
    /// Surface stays local-only; the support packet redacts the
    /// surface's restricted-mode state entirely.
    LocalOnly,
    /// Surface ships a metadata-safe projection of its restricted-mode
    /// state in the support packet.
    MetadataSafeExport,
    /// Surface withholds the restricted-mode state from the support
    /// packet until a manual export reviews it.
    HeldRecord,
}

impl RestrictedModeSupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Class of pre-action inspection / repair hook offered before any
/// restricted-mode UX commits an escape path or a tier-widening grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModeInspectionHookClass {
    /// Open the restriction inspector for the workspace.
    InspectRestriction,
    /// Open the escape-path review sheet before any escape commits.
    ReviewEscapePath,
    /// Compare the restricted-mode posture against the unrestricted
    /// baseline so the user can see what changes if they widen trust.
    CompareUnrestricted,
    /// Capture a one-step rollback before committing a trust grant.
    RollbackGrant,
    /// Export the restricted-mode lineage record (support-safe).
    Export,
    /// Open the repair sheet for a restricted workspace.
    Repair,
}

impl RestrictedModeInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectRestriction => "inspect_restriction",
            Self::ReviewEscapePath => "review_escape_path",
            Self::CompareUnrestricted => "compare_unrestricted",
            Self::RollbackGrant => "rollback_grant",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-action inspection / repair hook offered before a
/// restricted-mode UX surface commits an escape path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeInspectionHook {
    /// Hook class.
    pub hook_class: RestrictedModeInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-action inspection / repair hook table.
pub fn default_restricted_mode_ux_inspection_hooks() -> Vec<RestrictedModeInspectionHook> {
    vec![
        RestrictedModeInspectionHook {
            hook_class: RestrictedModeInspectionHookClass::InspectRestriction,
            action_id: "restricted_mode.inspect_restriction".to_owned(),
            label: "Inspect restriction".to_owned(),
            available: true,
            disclosure:
                "Opens the restriction inspector with the captured restriction reason, the surface affected, and the current escape-path offer."
                    .to_owned(),
        },
        RestrictedModeInspectionHook {
            hook_class: RestrictedModeInspectionHookClass::ReviewEscapePath,
            action_id: "restricted_mode.review_escape_path".to_owned(),
            label: "Review escape path".to_owned(),
            available: true,
            disclosure:
                "Opens the escape-path review sheet so any tier-widening or trust-grant commit can be reviewed before it fires."
                    .to_owned(),
        },
        RestrictedModeInspectionHook {
            hook_class: RestrictedModeInspectionHookClass::CompareUnrestricted,
            action_id: "restricted_mode.compare_unrestricted".to_owned(),
            label: "Compare with unrestricted baseline".to_owned(),
            available: true,
            disclosure:
                "Renders the surface affordance diff between the restricted-mode posture and the unrestricted baseline before the user widens trust."
                    .to_owned(),
        },
        RestrictedModeInspectionHook {
            hook_class: RestrictedModeInspectionHookClass::RollbackGrant,
            action_id: "restricted_mode.rollback_grant".to_owned(),
            label: "Rollback trust grant".to_owned(),
            available: true,
            disclosure:
                "Captures a one-step rollback so the user can revert a trust grant if a restricted-mode surface widens unexpectedly."
                    .to_owned(),
        },
        RestrictedModeInspectionHook {
            hook_class: RestrictedModeInspectionHookClass::Export,
            action_id: "restricted_mode.export".to_owned(),
            label: "Export restricted-mode lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this restricted-mode UX lineage record for support without raw secrets, approval tickets, or delegated credentials."
                    .to_owned(),
        },
        RestrictedModeInspectionHook {
            hook_class: RestrictedModeInspectionHookClass::Repair,
            action_id: "restricted_mode.repair".to_owned(),
            label: "Open repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the repair sheet for a restricted workspace and surfaces the manual remediation steps that lift the restriction."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a restricted-mode
/// UX surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeSupportExportInputs {
    pub posture: RestrictedModeSupportExportPosture,
    pub includes_surface_kind: bool,
    pub includes_restriction_reason: bool,
    pub includes_explanation_id: bool,
    pub includes_escape_path: bool,
    pub includes_claimed_tier: bool,
    pub includes_accessibility_postures: bool,
    pub raw_secrets_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl RestrictedModeSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: RestrictedModeSupportExportPosture) -> Self {
        Self {
            posture,
            includes_surface_kind: true,
            includes_restriction_reason: true,
            includes_explanation_id: true,
            includes_escape_path: true,
            includes_claimed_tier: true,
            includes_accessibility_postures: true,
            raw_secrets_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// One observation of a restricted-mode UX surface at a captured
/// moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeSurfaceObservation {
    /// Stable surface id (route-style, e.g. `status_bar.workspace_trust`).
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed surface kind.
    pub surface_kind: RestrictedModeSurfaceKind,
    /// Named restriction reason.
    pub restriction_reason: RestrictionReasonClass,
    /// Stable explanation id (non-empty when the surface ships a
    /// restriction message).
    pub explanation_id: String,
    /// Named escape path.
    pub escape_path: EscapePathClass,
    /// Stable escape-action id (required when the escape path is not
    /// `stay_read_only`).
    pub escape_action_id: String,
    /// Stable escape-disclosure id (required for any tier-widening
    /// escape path).
    pub escape_disclosure_id: String,
    /// Declared affordance classes available on this surface.
    pub affordances: Vec<RestrictedAffordanceClass>,
    /// Claimed stability tier.
    pub claimed_tier: ClaimedStableTier,
    /// Whether the surface touches a credential store (and therefore
    /// must ship a non-local-only support-export posture).
    pub touches_credential_store: bool,
    /// Accessibility postures supported by the surface.
    pub accessibility_postures: Vec<AccessibilityPostureClass>,
    /// Support-export projection.
    pub support_export: RestrictedModeSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeUxInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured workspace restricted-mode posture.
    pub posture: RestrictedModePosture,
    /// Captured surface observations.
    pub surfaces: Vec<RestrictedModeSurfaceObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a restricted-mode UX lineage record narrows below
/// Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedModeUxLineageNarrowReason {
    /// The captured input had no surface observations.
    CorpusEmpty,
    /// A required restricted-mode UX surface kind is missing from the
    /// corpus.
    RequiredRestrictedSurfaceMissing,
    /// A surface in a restricted workspace did not ship an explanation
    /// id.
    ExplanationMissing,
    /// A surface declared a `grant_trust` escape path but omitted the
    /// escape action id or disclosure id.
    GrantTrustEscapeUndisclosed,
    /// A surface declared a non-`stay_read_only` escape path but
    /// omitted its escape action id.
    EscapePathActionMissing,
    /// A surface claimed `stable_read_only` but exposed a mutation
    /// affordance.
    ReadOnlyClaimExposesMutation,
    /// A surface claimed `stable_full` in a restricted or pending
    /// workspace (the projection must downgrade to read-only or
    /// narrowed).
    ClaimedFullInRestrictedPosture,
    /// A surface declared no affordances at all.
    AffordancesEmpty,
    /// A surface is missing at least one required accessibility
    /// posture.
    AccessibilityPostureMissing,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, approval tickets, delegated credentials, or live
    /// authority handles slipped into a support-export projection.
    SupportExportRedactionUnsafe,
    /// A credential-touching surface declared `local_only` support
    /// export.
    SupportExportPostureUnsafe,
    /// Producer attribution is incomplete (producer ref / captured-at).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl RestrictedModeUxLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredRestrictedSurfaceMissing => "required_restricted_surface_missing",
            Self::ExplanationMissing => "explanation_missing",
            Self::GrantTrustEscapeUndisclosed => "grant_trust_escape_undisclosed",
            Self::EscapePathActionMissing => "escape_path_action_missing",
            Self::ReadOnlyClaimExposesMutation => "read_only_claim_exposes_mutation",
            Self::ClaimedFullInRestrictedPosture => "claimed_full_in_restricted_posture",
            Self::AffordancesEmpty => "affordances_empty",
            Self::AccessibilityPostureMissing => "accessibility_posture_missing",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::SupportExportPostureUnsafe => "support_export_posture_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a restricted-mode UX lineage
/// record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeUxLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<RestrictedModeUxLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One restricted-mode UX surface row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeSurfaceRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Restricted-mode UX surface kind.
    pub surface_kind: RestrictedModeSurfaceKind,
    /// Named restriction reason.
    pub restriction_reason: RestrictionReasonClass,
    /// True when an explanation id is provided.
    pub has_explanation: bool,
    /// Named escape path.
    pub escape_path: EscapePathClass,
    /// True when an escape-action id was provided.
    pub has_escape_action: bool,
    /// True when an escape-disclosure id was provided.
    pub has_escape_disclosure: bool,
    /// Affordances declared on this surface.
    pub affordances: Vec<RestrictedAffordanceClass>,
    /// True when every declared affordance is read-only-safe.
    pub affordances_read_only_safe: bool,
    /// Declared claimed tier.
    pub declared_tier: ClaimedStableTier,
    /// Re-derived worst-case tier given the workspace posture and the
    /// declared affordance set.
    pub derived_tier: ClaimedStableTier,
    /// True when the declared tier is consistent with the derived
    /// tier.
    pub tier_matches: bool,
    /// True when the surface touches a credential store.
    pub touches_credential_store: bool,
    /// Accessibility postures supported by the surface.
    pub accessibility_postures: Vec<AccessibilityPostureClass>,
    /// True when every required accessibility posture is present.
    pub accessibility_complete: bool,
    /// Support-export posture for this surface.
    pub support_export_posture: RestrictedModeSupportExportPosture,
}

/// Surface coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeSurfaceCoverageSummary {
    /// All surface rows carried by the corpus.
    pub surface_rows: Vec<RestrictedModeSurfaceRow>,
    /// True when every required surface kind is present.
    pub all_required_surfaces_present: bool,
}

/// Explainability posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainabilityTruthSummary {
    /// True when every restricted-mode surface ships an explanation
    /// id.
    pub all_surfaces_have_explanation: bool,
    /// Count of unique restriction reasons observed.
    pub distinct_restriction_reasons: usize,
}

/// Escape-path honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscapePathHonestySummary {
    /// True when every non-`stay_read_only` escape path references an
    /// action id.
    pub all_escape_paths_have_action: bool,
    /// True when every `grant_trust` escape path references a
    /// disclosure id.
    pub all_grant_trust_escapes_disclosed: bool,
    /// Number of surfaces offering a `grant_trust` escape path.
    pub grant_trust_escape_count: usize,
}

/// Read-only affordance posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadOnlyAffordanceTruthSummary {
    /// True when every surface claiming `stable_read_only` exposes
    /// only read-only-safe affordances.
    pub all_read_only_surfaces_safe: bool,
    /// Number of read-only-tier surfaces in the corpus.
    pub read_only_surface_count: usize,
    /// True when every surface declares at least one affordance.
    pub all_surfaces_have_affordances: bool,
}

/// Claimed-tier posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedTierTruthSummary {
    /// True when no surface claims `stable_full` while the workspace
    /// posture is restricted or pending.
    pub no_full_claim_in_restricted_posture: bool,
    /// True when every declared tier matches the re-derived worst-case
    /// tier.
    pub all_tiers_match_derived: bool,
}

/// Accessibility posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityTruthSummary {
    /// True when every surface declares the full required set of
    /// accessibility postures.
    pub all_surfaces_accessibility_complete: bool,
    /// Count of surfaces missing at least one required posture.
    pub surfaces_missing_postures: usize,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeSupportExportHonestySummary {
    /// True when every surface's support-export projection preserves
    /// the required restricted-mode fields.
    pub all_surfaces_preserve_fields: bool,
    /// True when every surface declares
    /// `raw_secrets_excluded = true`.
    pub all_surfaces_redact_raw_secrets: bool,
    /// True when every surface declares
    /// `approval_tickets_excluded = true`.
    pub all_surfaces_exclude_approval_tickets: bool,
    /// True when every surface declares
    /// `delegated_credentials_excluded = true`.
    pub all_surfaces_exclude_delegated_credentials: bool,
    /// True when every surface declares
    /// `live_authority_handles_excluded = true`.
    pub all_surfaces_exclude_live_authority_handles: bool,
    /// True when every credential-touching surface declares a
    /// non-`local_only` support-export posture.
    pub all_credential_surfaces_have_safe_posture: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Schema version pinned by the input.
    pub schema_version: u32,
    /// Opaque integrity hash derived from the input surface identities.
    pub integrity_hash: String,
    /// Input capture timestamp.
    pub captured_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe restricted-mode UX lineage record per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestrictedModeUxLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub restricted_mode_ux_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque ref to the corpus the projection ingested.
    pub corpus_ref: String,
    /// Captured workspace restricted-mode posture.
    pub posture: RestrictedModePosture,
    /// Producer attribution pillar.
    pub producer_attribution: RestrictedModeProducerAttributionSummary,
    /// Surface coverage pillar.
    pub surface_coverage: RestrictedModeSurfaceCoverageSummary,
    /// Explainability pillar.
    pub explainability_truth: ExplainabilityTruthSummary,
    /// Escape-path honesty pillar.
    pub escape_path_honesty: EscapePathHonestySummary,
    /// Read-only affordance truth pillar.
    pub read_only_affordance_truth: ReadOnlyAffordanceTruthSummary,
    /// Claimed-tier truth pillar.
    pub claimed_tier_truth: ClaimedTierTruthSummary,
    /// Accessibility truth pillar.
    pub accessibility_truth: AccessibilityTruthSummary,
    /// Support-export honesty pillar.
    pub support_export_honesty: RestrictedModeSupportExportHonestySummary,
    /// Pre-action inspection / repair hooks.
    pub inspection_hooks: Vec<RestrictedModeInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: RestrictedModeUxLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl RestrictedModeUxLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF
            && self.record_kind == RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: RestrictedModeInspectionHookClass,
    ) -> Option<&RestrictedModeInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed restricted-mode UX lineage record from a live
/// [`RestrictedModeUxInputs`] envelope using the default inspection-hook
/// set.
pub fn project_restricted_mode_ux_lineage(
    posture_id: impl Into<String>,
    inputs: &RestrictedModeUxInputs,
) -> RestrictedModeUxLineageRecord {
    project_restricted_mode_ux_lineage_with_hooks(
        posture_id,
        inputs,
        default_restricted_mode_ux_inspection_hooks(),
    )
}

/// Like [`project_restricted_mode_ux_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_restricted_mode_ux_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &RestrictedModeUxInputs,
    inspection_hooks: Vec<RestrictedModeInspectionHook>,
) -> RestrictedModeUxLineageRecord {
    let posture_id: String = posture_id.into();

    let surface_coverage = project_surface_coverage(inputs);
    let explainability_truth = project_explainability_truth(inputs);
    let escape_path_honesty = project_escape_path_honesty(inputs);
    let read_only_affordance_truth = project_read_only_affordance_truth(&surface_coverage);
    let claimed_tier_truth = project_claimed_tier_truth(&surface_coverage, inputs.posture);
    let accessibility_truth = project_accessibility_truth(&surface_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.surfaces.is_empty() {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::CorpusEmpty);
    }
    if !surface_coverage.all_required_surfaces_present {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::RequiredRestrictedSurfaceMissing);
    }

    collect_explainability_narrows(&explainability_truth, inputs.posture, &mut narrow_reasons);
    collect_escape_path_narrows(&escape_path_honesty, &mut narrow_reasons);
    collect_read_only_affordance_narrows(&read_only_affordance_truth, &mut narrow_reasons);
    collect_claimed_tier_narrows(&claimed_tier_truth, &mut narrow_reasons);
    collect_accessibility_narrows(&accessibility_truth, &mut narrow_reasons);
    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    let required_hooks = [
        RestrictedModeInspectionHookClass::InspectRestriction,
        RestrictedModeInspectionHookClass::ReviewEscapePath,
        RestrictedModeInspectionHookClass::CompareUnrestricted,
        RestrictedModeInspectionHookClass::RollbackGrant,
        RestrictedModeInspectionHookClass::Export,
        RestrictedModeInspectionHookClass::Repair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::InspectionHookUnavailable);
    }

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = RestrictedModeUxLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        inputs.posture,
        &surface_coverage,
        &claimed_tier_truth,
        &stable_qualification,
    );

    RestrictedModeUxLineageRecord {
        record_kind: RESTRICTED_MODE_UX_LINEAGE_RECORD_KIND.to_owned(),
        restricted_mode_ux_lineage_schema_version: RESTRICTED_MODE_UX_LINEAGE_SCHEMA_VERSION,
        schema_ref: RESTRICTED_MODE_UX_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        posture: inputs.posture,
        producer_attribution,
        surface_coverage,
        explainability_truth,
        escape_path_honesty,
        read_only_affordance_truth,
        claimed_tier_truth,
        accessibility_truth,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_surface_coverage(
    inputs: &RestrictedModeUxInputs,
) -> RestrictedModeSurfaceCoverageSummary {
    let surface_rows: Vec<RestrictedModeSurfaceRow> = inputs
        .surfaces
        .iter()
        .map(|surface| project_surface_row(surface, inputs.posture))
        .collect();
    let observed: BTreeSet<_> = surface_rows.iter().map(|row| row.surface_kind).collect();
    let all_required_surfaces_present = REQUIRED_RESTRICTED_MODE_SURFACES
        .iter()
        .all(|required| observed.contains(required));
    RestrictedModeSurfaceCoverageSummary {
        surface_rows,
        all_required_surfaces_present,
    }
}

fn project_surface_row(
    surface: &RestrictedModeSurfaceObservation,
    posture: RestrictedModePosture,
) -> RestrictedModeSurfaceRow {
    let affordances_read_only_safe = surface
        .affordances
        .iter()
        .all(|affordance| affordance.is_read_only_safe());
    let derived_tier = derive_tier(posture, surface, affordances_read_only_safe);
    let tier_matches = tier_consistent_with_derived(surface.claimed_tier, derived_tier);
    let accessibility_set: BTreeSet<_> = surface.accessibility_postures.iter().copied().collect();
    let accessibility_complete = REQUIRED_ACCESSIBILITY_POSTURES
        .iter()
        .all(|required| accessibility_set.contains(required));
    RestrictedModeSurfaceRow {
        surface_id: surface.surface_id.clone(),
        title: surface.title.clone(),
        surface_kind: surface.surface_kind,
        restriction_reason: surface.restriction_reason,
        has_explanation: !surface.explanation_id.trim().is_empty(),
        escape_path: surface.escape_path,
        has_escape_action: !surface.escape_action_id.trim().is_empty(),
        has_escape_disclosure: !surface.escape_disclosure_id.trim().is_empty(),
        affordances: surface.affordances.clone(),
        affordances_read_only_safe,
        declared_tier: surface.claimed_tier,
        derived_tier,
        tier_matches,
        touches_credential_store: surface.touches_credential_store,
        accessibility_postures: surface.accessibility_postures.clone(),
        accessibility_complete,
        support_export_posture: surface.support_export.posture,
    }
}

fn derive_tier(
    posture: RestrictedModePosture,
    surface: &RestrictedModeSurfaceObservation,
    affordances_read_only_safe: bool,
) -> ClaimedStableTier {
    if surface.affordances.is_empty() {
        return ClaimedStableTier::NarrowedBelowStable;
    }
    match posture {
        RestrictedModePosture::Trusted => match surface.claimed_tier {
            ClaimedStableTier::StableFull => ClaimedStableTier::StableFull,
            ClaimedStableTier::StableReadOnly => {
                if affordances_read_only_safe {
                    ClaimedStableTier::StableReadOnly
                } else {
                    ClaimedStableTier::NarrowedBelowStable
                }
            }
            ClaimedStableTier::NarrowedBelowStable => ClaimedStableTier::NarrowedBelowStable,
        },
        RestrictedModePosture::Restricted | RestrictedModePosture::PendingEvaluation => {
            if affordances_read_only_safe {
                match surface.claimed_tier {
                    ClaimedStableTier::StableReadOnly => ClaimedStableTier::StableReadOnly,
                    _ => ClaimedStableTier::NarrowedBelowStable,
                }
            } else {
                ClaimedStableTier::NarrowedBelowStable
            }
        }
    }
}

fn tier_consistent_with_derived(declared: ClaimedStableTier, derived: ClaimedStableTier) -> bool {
    declared == derived
}

fn project_explainability_truth(inputs: &RestrictedModeUxInputs) -> ExplainabilityTruthSummary {
    let mut all_surfaces_have_explanation = true;
    let mut reasons: BTreeSet<_> = BTreeSet::new();
    for surface in &inputs.surfaces {
        reasons.insert(surface.restriction_reason);
        if inputs.posture.is_restricted_mode() && surface.explanation_id.trim().is_empty() {
            all_surfaces_have_explanation = false;
        }
    }
    ExplainabilityTruthSummary {
        all_surfaces_have_explanation,
        distinct_restriction_reasons: reasons.len(),
    }
}

fn project_escape_path_honesty(inputs: &RestrictedModeUxInputs) -> EscapePathHonestySummary {
    let mut all_escape_paths_have_action = true;
    let mut all_grant_trust_escapes_disclosed = true;
    let mut grant_trust_escape_count = 0usize;

    for surface in &inputs.surfaces {
        if surface.escape_path != EscapePathClass::StayReadOnly
            && surface.escape_action_id.trim().is_empty()
        {
            all_escape_paths_have_action = false;
        }
        if surface.escape_path.widens_trust() {
            grant_trust_escape_count += 1;
            if surface.escape_action_id.trim().is_empty()
                || surface.escape_disclosure_id.trim().is_empty()
            {
                all_grant_trust_escapes_disclosed = false;
            }
        }
    }

    EscapePathHonestySummary {
        all_escape_paths_have_action,
        all_grant_trust_escapes_disclosed,
        grant_trust_escape_count,
    }
}

fn project_read_only_affordance_truth(
    coverage: &RestrictedModeSurfaceCoverageSummary,
) -> ReadOnlyAffordanceTruthSummary {
    let mut all_read_only_surfaces_safe = true;
    let mut read_only_surface_count = 0usize;
    let mut all_surfaces_have_affordances = true;
    for row in &coverage.surface_rows {
        if row.affordances.is_empty() {
            all_surfaces_have_affordances = false;
        }
        if row.declared_tier == ClaimedStableTier::StableReadOnly {
            read_only_surface_count += 1;
            if !row.affordances_read_only_safe {
                all_read_only_surfaces_safe = false;
            }
        }
    }
    ReadOnlyAffordanceTruthSummary {
        all_read_only_surfaces_safe,
        read_only_surface_count,
        all_surfaces_have_affordances,
    }
}

fn project_claimed_tier_truth(
    coverage: &RestrictedModeSurfaceCoverageSummary,
    posture: RestrictedModePosture,
) -> ClaimedTierTruthSummary {
    let mut no_full_claim_in_restricted_posture = true;
    let mut all_tiers_match_derived = true;
    for row in &coverage.surface_rows {
        if posture.is_restricted_mode() && row.declared_tier == ClaimedStableTier::StableFull {
            no_full_claim_in_restricted_posture = false;
        }
        if !row.tier_matches {
            all_tiers_match_derived = false;
        }
    }
    ClaimedTierTruthSummary {
        no_full_claim_in_restricted_posture,
        all_tiers_match_derived,
    }
}

fn project_accessibility_truth(
    coverage: &RestrictedModeSurfaceCoverageSummary,
) -> AccessibilityTruthSummary {
    let mut all_surfaces_accessibility_complete = true;
    let mut surfaces_missing_postures = 0usize;
    for row in &coverage.surface_rows {
        if !row.accessibility_complete {
            all_surfaces_accessibility_complete = false;
            surfaces_missing_postures += 1;
        }
    }
    AccessibilityTruthSummary {
        all_surfaces_accessibility_complete,
        surfaces_missing_postures,
    }
}

fn project_support_export_honesty(
    inputs: &RestrictedModeUxInputs,
) -> RestrictedModeSupportExportHonestySummary {
    let mut all_surfaces_preserve_fields = true;
    let mut all_surfaces_redact_raw_secrets = true;
    let mut all_surfaces_exclude_approval_tickets = true;
    let mut all_surfaces_exclude_delegated_credentials = true;
    let mut all_surfaces_exclude_live_authority_handles = true;
    let mut all_credential_surfaces_have_safe_posture = true;

    for surface in &inputs.surfaces {
        let support = surface.support_export;
        if !(support.includes_surface_kind
            && support.includes_restriction_reason
            && support.includes_explanation_id
            && support.includes_escape_path
            && support.includes_claimed_tier
            && support.includes_accessibility_postures)
        {
            all_surfaces_preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            all_surfaces_redact_raw_secrets = false;
        }
        if !support.approval_tickets_excluded {
            all_surfaces_exclude_approval_tickets = false;
        }
        if !support.delegated_credentials_excluded {
            all_surfaces_exclude_delegated_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            all_surfaces_exclude_live_authority_handles = false;
        }
        if surface.touches_credential_store
            && support.posture == RestrictedModeSupportExportPosture::LocalOnly
        {
            all_credential_surfaces_have_safe_posture = false;
        }
    }

    RestrictedModeSupportExportHonestySummary {
        all_surfaces_preserve_fields,
        all_surfaces_redact_raw_secrets,
        all_surfaces_exclude_approval_tickets,
        all_surfaces_exclude_delegated_credentials,
        all_surfaces_exclude_live_authority_handles,
        all_credential_surfaces_have_safe_posture,
    }
}

fn project_producer_attribution(
    inputs: &RestrictedModeUxInputs,
) -> RestrictedModeProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    RestrictedModeProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: RESTRICTED_MODE_UX_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_explainability_narrows(
    summary: &ExplainabilityTruthSummary,
    posture: RestrictedModePosture,
    narrow_reasons: &mut Vec<RestrictedModeUxLineageNarrowReason>,
) {
    if posture.is_restricted_mode() && !summary.all_surfaces_have_explanation {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::ExplanationMissing);
    }
}

fn collect_escape_path_narrows(
    summary: &EscapePathHonestySummary,
    narrow_reasons: &mut Vec<RestrictedModeUxLineageNarrowReason>,
) {
    if !summary.all_escape_paths_have_action {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::EscapePathActionMissing);
    }
    if !summary.all_grant_trust_escapes_disclosed {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::GrantTrustEscapeUndisclosed);
    }
}

fn collect_read_only_affordance_narrows(
    summary: &ReadOnlyAffordanceTruthSummary,
    narrow_reasons: &mut Vec<RestrictedModeUxLineageNarrowReason>,
) {
    if !summary.all_read_only_surfaces_safe {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::ReadOnlyClaimExposesMutation);
    }
    if !summary.all_surfaces_have_affordances {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::AffordancesEmpty);
    }
}

fn collect_claimed_tier_narrows(
    summary: &ClaimedTierTruthSummary,
    narrow_reasons: &mut Vec<RestrictedModeUxLineageNarrowReason>,
) {
    if !summary.no_full_claim_in_restricted_posture {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::ClaimedFullInRestrictedPosture);
    }
    if !summary.all_tiers_match_derived
        && !narrow_reasons
            .contains(&RestrictedModeUxLineageNarrowReason::ClaimedFullInRestrictedPosture)
    {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::ClaimedFullInRestrictedPosture);
    }
}

fn collect_accessibility_narrows(
    summary: &AccessibilityTruthSummary,
    narrow_reasons: &mut Vec<RestrictedModeUxLineageNarrowReason>,
) {
    if !summary.all_surfaces_accessibility_complete {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::AccessibilityPostureMissing);
    }
}

fn collect_support_export_narrows(
    summary: &RestrictedModeSupportExportHonestySummary,
    narrow_reasons: &mut Vec<RestrictedModeUxLineageNarrowReason>,
) {
    if !summary.all_surfaces_preserve_fields {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !summary.all_credential_surfaces_have_safe_posture {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::SupportExportPostureUnsafe);
    }
    if !(summary.all_surfaces_redact_raw_secrets
        && summary.all_surfaces_exclude_approval_tickets
        && summary.all_surfaces_exclude_delegated_credentials
        && summary.all_surfaces_exclude_live_authority_handles)
    {
        narrow_reasons.push(RestrictedModeUxLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &RestrictedModeUxInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
        inputs.posture.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for surface in &inputs.surfaces {
        for byte in surface.surface_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(surface.surface_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(surface.restriction_reason.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(surface.escape_path.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(surface.claimed_tier.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("rmu:{hash:016x}")
}

fn hook_available(
    hooks: &[RestrictedModeInspectionHook],
    class: RestrictedModeInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    posture: RestrictedModePosture,
    coverage: &RestrictedModeSurfaceCoverageSummary,
    tier: &ClaimedTierTruthSummary,
    qualification: &RestrictedModeUxLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Restricted-mode UX lineage proven Stable: posture={posture} surfaces={total} no_full_claim_in_restricted={safe} all_tiers_match_derived={tiers}.",
            posture = posture.as_str(),
            total = coverage.surface_rows.len(),
            safe = tier.no_full_claim_in_restricted_posture,
            tiers = tier.all_tiers_match_derived,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Restricted-mode UX lineage narrowed below Stable (posture={posture}, surfaces={total}): {reasons}.",
            posture = posture.as_str(),
            total = coverage.surface_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a restricted-mode UX
/// lineage record. The same projection is consumed by the workspace
/// restricted-mode status surface, the headless CLI emitter,
/// Help/About, and support export.
pub fn restricted_mode_ux_lineage_lines(record: &RestrictedModeUxLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Restricted-mode UX lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={} posture={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
        record.posture.as_str(),
    ));
    lines.push(format!(
        "surface_coverage: surfaces={} required_surfaces_present={}",
        record.surface_coverage.surface_rows.len(),
        record.surface_coverage.all_required_surfaces_present,
    ));
    lines.push("Surface rows:".to_owned());
    for row in &record.surface_coverage.surface_rows {
        let affordance_list: Vec<&str> = row
            .affordances
            .iter()
            .map(|affordance| affordance.as_str())
            .collect();
        let accessibility_list: Vec<&str> = row
            .accessibility_postures
            .iter()
            .map(|posture| posture.as_str())
            .collect();
        lines.push(format!(
            "  - {kind} {id} reason={reason} explanation={explanation} escape={escape} action={action} disclosure={disclosure} tier_declared={declared} tier_derived={derived} tier_matches={matches} read_only_safe={ro_safe} accessibility=[{accessibility}] accessibility_complete={accessibility_complete} affordances=[{affordances}] credential_store={credential} support_export={posture}",
            kind = row.surface_kind.as_str(),
            id = row.surface_id,
            reason = row.restriction_reason.as_str(),
            explanation = row.has_explanation,
            escape = row.escape_path.as_str(),
            action = row.has_escape_action,
            disclosure = row.has_escape_disclosure,
            declared = row.declared_tier.as_str(),
            derived = row.derived_tier.as_str(),
            matches = row.tier_matches,
            ro_safe = row.affordances_read_only_safe,
            accessibility = accessibility_list.join(","),
            accessibility_complete = row.accessibility_complete,
            affordances = affordance_list.join(","),
            credential = row.touches_credential_store,
            posture = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Explainability truth: all_surfaces_have_explanation={explanation} distinct_reasons={reasons}",
        explanation = record.explainability_truth.all_surfaces_have_explanation,
        reasons = record.explainability_truth.distinct_restriction_reasons,
    ));
    lines.push(format!(
        "Escape path honesty: all_have_action={action} grant_trust_count={count} grant_trust_disclosed={disclosed}",
        action = record.escape_path_honesty.all_escape_paths_have_action,
        count = record.escape_path_honesty.grant_trust_escape_count,
        disclosed = record.escape_path_honesty.all_grant_trust_escapes_disclosed,
    ));
    lines.push(format!(
        "Read-only affordance truth: read_only_count={count} read_only_safe={safe} affordances_non_empty={non_empty}",
        count = record.read_only_affordance_truth.read_only_surface_count,
        safe = record.read_only_affordance_truth.all_read_only_surfaces_safe,
        non_empty = record.read_only_affordance_truth.all_surfaces_have_affordances,
    ));
    lines.push(format!(
        "Claimed tier truth: no_full_in_restricted={safe} all_tiers_match_derived={tiers}",
        safe = record
            .claimed_tier_truth
            .no_full_claim_in_restricted_posture,
        tiers = record.claimed_tier_truth.all_tiers_match_derived,
    ));
    lines.push(format!(
        "Accessibility truth: all_complete={complete} missing_count={missing}",
        complete = record
            .accessibility_truth
            .all_surfaces_accessibility_complete,
        missing = record.accessibility_truth.surfaces_missing_postures,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} redact_secrets={secrets} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority} credential_surfaces_safe={safe}",
        fields = record.support_export_honesty.all_surfaces_preserve_fields,
        secrets = record.support_export_honesty.all_surfaces_redact_raw_secrets,
        approvals = record.support_export_honesty.all_surfaces_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_surfaces_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_surfaces_exclude_live_authority_handles,
        safe = record
            .support_export_honesty
            .all_credential_surfaces_have_safe_posture,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
