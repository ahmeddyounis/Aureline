//! Two reusable M5 scaffold / project-entry components — the generated-project diff card and the
//! scaffold handoff banner — so a user can review exactly what a starter wrote and recover from it
//! after generation, not just before: the diff card names its created / modified / renamed /
//! deleted counts (the same vocabulary Aureline uses for AI patches, importers, and refactors), its
//! template or generator source, its config / dependency / task / extension impact, a named
//! checkpoint or rollback / delete-generated path, and a generated-versus-user-owned boundary cue;
//! the handoff banner names its created-workspace identity, its trust state, its health summary,
//! `Run now` / `Run later` / `Review files` / `Open manifest` choices, and a delete-generated or
//! reopen-preflight recovery route, so optional setup stays visibly optional and the safest next
//! step is never assumed for the user.
//!
//! Aureline's frozen scaffold-component matrix
//! ([`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`])
//! names the generated-project diff card and the scaffold handoff banner as two governed component
//! families and freezes their controlled vocabulary — the generated-zone classes (`generated_only`,
//! `user_owned`, `generated_then_edited`, `runtime_only`, `mixed_zone`, `zone_unknown`) and
//! diff-review states (`preview_ready`, `review_required`, `no_changes`, `conflict_detected`,
//! `diff_unavailable`, `blocked`) a diff card binds; the handoff outcome classes (`create_succeeded`,
//! `partial_bootstrap`, `create_failed`, `continued_without_starter`, `created_empty`,
//! `provisioning_pending`) and recovery actions (`open_workspace`, `retry_bootstrap`,
//! `delete_generated`, `continue_without_starter`, `keep_partial_review`, `no_recovery_needed`) a
//! handoff banner binds; the one controlled disposition vocabulary; the surface families; the
//! deployment lines; the consumer surfaces; the accessibility routes; the required labels; and the
//! downgrade triggers. This module *implements* that contract as two co-equal component vectors so a
//! claimed M5 diff-review, workspace-handoff, start-center, or CLI surface can project a diff card
//! and a handoff banner that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_diff_disclosure`] — takes a diff card's frozen generated-zone class and diff-review
//!    state and derives its generated-versus-user-owned **boundary posture** and its **review
//!    disposition** (reviewable preview, review required before write, nothing to review, conflict
//!    blocked, or diff unavailable / blocked), so a conflict or unavailable diff can never read as a
//!    clean applied change and a user-owned zone can never read as free-to-overwrite generated
//!    output.
//! 2. [`resolve_handoff_disclosure`] — takes a handoff banner's frozen outcome class and derives its
//!    **outcome posture** (clean create, partial needing recovery, failed needing recovery,
//!    continued without starter, created empty, or provisioning pending), so a partial or failed
//!    bootstrap can never read as a clean create.
//!
//! A single controls packet — [`GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket`] —
//! binds one vector of diff cards and one vector of handoff banners to the same change-count,
//! impact, boundary, trust, recovery, deep-link, and non-visual accessibility vocabulary, so what a
//! starter wrote and how to recover stay explicit across desktop, headless / export, and support
//! consumers.
//!
//! The generated-zone class ([`M5GeneratedZoneClass`]), diff-review state ([`M5DiffReviewState`]),
//! handoff outcome class ([`M5HandoffOutcomeClass`]), handoff recovery action
//! ([`M5HandoffRecoveryAction`]), disposition ([`M5ScaffoldDisposition`]), surface family
//! ([`M5ScaffoldSurfaceFamily`]), deployment line ([`M5ScaffoldDeploymentLine`]), consumer surface
//! ([`M5ScaffoldConsumerSurface`]), accessibility route ([`M5ScaffoldAccessibilityRoute`]),
//! required label ([`M5ScaffoldRequiredLabel`]), and downgrade trigger
//! ([`M5ScaffoldDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module mints
//! new vocabulary only for what that matrix left implicit about the two components themselves: the
//! acceptance-criteria create / modify / rename / delete change kinds, the diff source kind, the
//! derived review disposition and generated-versus-user-owned boundary posture, the bounded
//! diff-card actions, the handoff trust state, the derived handoff outcome posture, the bounded
//! handoff-banner actions, and the deep-link kinds. No M5 generation surface invents a second
//! diff-card or handoff-banner grammar.
//!
//! Raw file bodies, raw secret values, pasted local paths, repository URLs, credentials, and secrets
//! stay outside the export boundary; every note, deep-link reference, and component identity is
//! carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The generated-zone classes and diff-review states, the handoff outcome classes and recovery
// actions, the disposition vocabulary, and the surface / deployment / consumer / accessibility /
// label / downgrade vocabularies are frozen once, in the scaffold-component matrix. This lane reuses
// them verbatim so it never invents a parallel diff-card or handoff-banner vocabulary.
pub use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5DiffReviewState, M5GeneratedZoneClass, M5HandoffOutcomeClass, M5HandoffRecoveryAction,
    M5ScaffoldAccessibilityRoute, M5ScaffoldComponentFamily, M5ScaffoldConsumerSurface,
    M5ScaffoldDeploymentLine, M5ScaffoldDisposition, M5ScaffoldDowngradeTrigger,
    M5ScaffoldRequiredLabel, M5ScaffoldSurfaceFamily, M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF,
    M5_SCAFFOLD_COMPONENT_DOC_REF, M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket`].
pub const SCAFFOLD_GENERATION_CONTROLS_RECORD_KIND: &str =
    "implement_generated_project_diff_cards_and_scaffold_handoff_banners_with_create_modify_rename_delete_counts_dependency_task_extension_impact_trust_state_and_run_now_later_review_recovery_truth_across_claimed_m5_generation_flows";

/// Schema version for M5 generated-project-diff-card / scaffold-handoff-banner control records.
pub const SCAFFOLD_GENERATION_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const SCAFFOLD_GENERATION_CONTROLS_DOC_REF: &str =
    "docs/templates/m5_generated_project_diff_card_scaffold_handoff_banner_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const SCAFFOLD_GENERATION_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls";

/// Repo-relative path of the checked support-export artifact.
pub const SCAFFOLD_GENERATION_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-generated-project-diff-card-scaffold-handoff-banner-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SCAFFOLD_GENERATION_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-generated-project-diff-card-scaffold-handoff-banner-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const SCAFFOLD_GENERATION_CONTROLS_REPORT_REF: &str =
    "artifacts/design/m5-generated-project-diff-card-scaffold-handoff-banner.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a scaffold-generation component binds its next step against, so a
/// diff card or handoff banner never routes through an ephemeral overlay — every next step is a
/// stable template manifest, starter-registry entry, docs, or policy reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable template-manifest reference.
    TemplateManifest,
    /// A stable starter-registry entry reference.
    StarterRegistryEntry,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable policy reference.
    PolicyReference,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TemplateManifest,
        Self::StarterRegistryEntry,
        Self::DocsAnchor,
        Self::PolicyReference,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateManifest => "template_manifest",
            Self::StarterRegistryEntry => "starter_registry_entry",
            Self::DocsAnchor => "docs_anchor",
            Self::PolicyReference => "policy_reference",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- generated-project-diff-card vocabulary -----------------------------

/// The change kind a generated-project diff card counts. These are the exact acceptance-criteria
/// labels — the same `create` / `modify` / `rename` / `delete` vocabulary Aureline uses for AI
/// patches, importers, and refactors — so generated output is previewed and reviewed with one
/// grammar rather than a parallel scaffold-only one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffChangeKind {
    /// A created file.
    Created,
    /// A modified file.
    Modified,
    /// A renamed file.
    Renamed,
    /// A deleted file.
    Deleted,
}

impl DiffChangeKind {
    /// Every change kind, in declaration order (the exact create / modify / rename / delete
    /// vocabulary).
    pub const ALL: [Self; 4] = [Self::Created, Self::Modified, Self::Renamed, Self::Deleted];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Modified => "modified",
            Self::Renamed => "renamed",
            Self::Deleted => "deleted",
        }
    }
}

/// Where a generated-project diff card's changes came from, so a card never leaves the template or
/// generator source of its writes implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffSourceKind {
    /// Written by a template starter.
    TemplateStarter,
    /// Written by a framework generator.
    FrameworkGenerator,
    /// Written by a codemod.
    Codemod,
    /// Written by importing an existing source.
    ImportedSource,
    /// Authored by the user (already present, not generated).
    UserAuthored,
}

impl DiffSourceKind {
    /// Every diff source kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TemplateStarter,
        Self::FrameworkGenerator,
        Self::Codemod,
        Self::ImportedSource,
        Self::UserAuthored,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateStarter => "template_starter",
            Self::FrameworkGenerator => "framework_generator",
            Self::Codemod => "codemod",
            Self::ImportedSource => "imported_source",
            Self::UserAuthored => "user_authored",
        }
    }

    /// True when the source is machine-generated rather than user-authored.
    pub const fn is_generated(self) -> bool {
        !matches!(self, Self::UserAuthored)
    }
}

/// Derived review disposition a generated-project diff card may present.
///
/// This is the diff honesty axis: the disposition is derived from the frozen diff-review state,
/// never asserted, so a conflict or unavailable diff can never present as a clean applied change and
/// no write is implied before review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffReviewDisposition {
    /// A reviewable diff preview is ready.
    ReviewablePreview,
    /// Review is required before any write.
    ReviewRequiredBeforeWrite,
    /// No changes; nothing to review.
    NoChangesToReview,
    /// A conflict is blocking the apply.
    ConflictBlocked,
    /// The diff is unavailable or blocked.
    DiffUnavailableBlocked,
}

impl DiffReviewDisposition {
    /// Every review disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReviewablePreview,
        Self::ReviewRequiredBeforeWrite,
        Self::NoChangesToReview,
        Self::ConflictBlocked,
        Self::DiffUnavailableBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewablePreview => "reviewable_preview",
            Self::ReviewRequiredBeforeWrite => "review_required_before_write",
            Self::NoChangesToReview => "no_changes_to_review",
            Self::ConflictBlocked => "conflict_blocked",
            Self::DiffUnavailableBlocked => "diff_unavailable_blocked",
        }
    }

    /// True when the diff can still be reviewed.
    pub const fn is_reviewable(self) -> bool {
        matches!(
            self,
            Self::ReviewablePreview | Self::ReviewRequiredBeforeWrite
        )
    }

    /// True when the diff is blocked from applying.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::ConflictBlocked | Self::DiffUnavailableBlocked)
    }
}

/// Derived generated-versus-user-owned boundary posture a diff card may present.
///
/// This is the ownership honesty axis: the posture is derived from the frozen generated-zone class,
/// never asserted, so a user-owned zone can never present as free-to-overwrite generated output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedBoundaryPosture {
    /// Generated and owned by the generator.
    GeneratedOwned,
    /// Owned by the user.
    UserOwned,
    /// Generated and then hand-edited by the user.
    GeneratedThenUserEdited,
    /// Runtime-only output (caches, build artifacts).
    RuntimeOnly,
    /// A mixed generated / user-owned zone.
    MixedOwnership,
    /// Ownership is unknown; review is required.
    OwnershipUnknown,
}

impl GeneratedBoundaryPosture {
    /// Every boundary posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GeneratedOwned,
        Self::UserOwned,
        Self::GeneratedThenUserEdited,
        Self::RuntimeOnly,
        Self::MixedOwnership,
        Self::OwnershipUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedOwned => "generated_owned",
            Self::UserOwned => "user_owned",
            Self::GeneratedThenUserEdited => "generated_then_user_edited",
            Self::RuntimeOnly => "runtime_only",
            Self::MixedOwnership => "mixed_ownership",
            Self::OwnershipUnknown => "ownership_unknown",
        }
    }

    /// True only when the zone is purely user-owned.
    pub const fn is_user_owned(self) -> bool {
        matches!(self, Self::UserOwned)
    }

    /// True when the zone contains user work that a write must not silently overwrite.
    pub const fn contains_user_work(self) -> bool {
        matches!(
            self,
            Self::UserOwned | Self::GeneratedThenUserEdited | Self::MixedOwnership
        )
    }
}

/// One keyboard-complete default action a generated-project diff card offers, so a card never hides
/// its diff, impact, or ownership-boundary affordance behind a pointer-only gesture and always keeps
/// an explicit rollback / delete-generated recovery path. `ReviewGeneratedDiff`, `ReviewChangeImpact`,
/// and `ReviewOwnershipBoundary` are always offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffCardAction {
    /// Review the created / modified / renamed / deleted diff (always available).
    ReviewGeneratedDiff,
    /// Review the config / dependency / task / extension impact (always available).
    ReviewChangeImpact,
    /// Review the generated-versus-user-owned boundary (always available).
    ReviewOwnershipBoundary,
    /// Roll back or delete the generated output (the explicit recovery path).
    RollbackGenerated,
    /// Keep the generated output.
    KeepGenerated,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
}

impl DiffCardAction {
    /// Every diff-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewGeneratedDiff,
        Self::ReviewChangeImpact,
        Self::ReviewOwnershipBoundary,
        Self::RollbackGenerated,
        Self::KeepGenerated,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete diff card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::ReviewGeneratedDiff,
        Self::ReviewChangeImpact,
        Self::ReviewOwnershipBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewGeneratedDiff => "review_generated_diff",
            Self::ReviewChangeImpact => "review_change_impact",
            Self::ReviewOwnershipBoundary => "review_ownership_boundary",
            Self::RollbackGenerated => "rollback_generated",
            Self::KeepGenerated => "keep_generated",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a generated-project diff card must carry, derived from the frozen generated-zone
/// class and diff-review state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiffCardDisclosure {
    /// The derived review disposition this card may present.
    pub review_disposition: DiffReviewDisposition,
    /// The derived generated-versus-user-owned boundary posture this card may present.
    pub boundary_posture: GeneratedBoundaryPosture,
    /// Whether the diff can still be reviewed.
    pub is_reviewable: bool,
    /// Whether the diff is blocked from applying.
    pub is_blocking: bool,
    /// Whether the zone is purely user-owned.
    pub is_user_owned: bool,
    /// Whether the card must carry an explicit review-required note.
    pub needs_review_required_note: bool,
    /// Whether the card must carry an explicit no-changes note.
    pub needs_no_changes_note: bool,
    /// Whether the card must carry an explicit conflict note.
    pub needs_conflict_note: bool,
    /// Whether the card must carry an explicit diff-unavailable note.
    pub needs_unavailable_note: bool,
}

/// Resolves the boundary and review truth a generated-project diff card may present.
///
/// A `preview_ready` diff is a reviewable preview, a `review_required` diff is review-required
/// before write, a `no_changes` diff has nothing to review, a `conflict_detected` diff is conflict
/// blocked, and a `diff_unavailable` or `blocked` diff is unavailable / blocked — so a conflict or
/// unavailable diff can never read as a clean applied change. Independently, the generated-zone class
/// derives the generated-versus-user-owned boundary posture so a user-owned zone can never read as
/// free-to-overwrite generated output.
pub fn resolve_diff_disclosure(
    generated_zone_class: M5GeneratedZoneClass,
    diff_review_state: M5DiffReviewState,
) -> DiffCardDisclosure {
    use DiffReviewDisposition as Review;
    use GeneratedBoundaryPosture as Boundary;
    use M5DiffReviewState as State;
    use M5GeneratedZoneClass as Zone;

    let review_disposition = match diff_review_state {
        State::PreviewReady => Review::ReviewablePreview,
        State::ReviewRequired => Review::ReviewRequiredBeforeWrite,
        State::NoChanges => Review::NoChangesToReview,
        State::ConflictDetected => Review::ConflictBlocked,
        State::DiffUnavailable | State::Blocked => Review::DiffUnavailableBlocked,
    };

    let boundary_posture = match generated_zone_class {
        Zone::GeneratedOnly => Boundary::GeneratedOwned,
        Zone::UserOwned => Boundary::UserOwned,
        Zone::GeneratedThenEdited => Boundary::GeneratedThenUserEdited,
        Zone::RuntimeOnly => Boundary::RuntimeOnly,
        Zone::MixedZone => Boundary::MixedOwnership,
        Zone::ZoneUnknown => Boundary::OwnershipUnknown,
    };

    DiffCardDisclosure {
        review_disposition,
        boundary_posture,
        is_reviewable: review_disposition.is_reviewable(),
        is_blocking: review_disposition.is_blocking(),
        is_user_owned: boundary_posture.is_user_owned(),
        needs_review_required_note: matches!(review_disposition, Review::ReviewRequiredBeforeWrite),
        needs_no_changes_note: matches!(review_disposition, Review::NoChangesToReview),
        needs_conflict_note: matches!(review_disposition, Review::ConflictBlocked),
        needs_unavailable_note: matches!(review_disposition, Review::DiffUnavailableBlocked),
    }
}

/// A generated-project diff card naming its created / modified / renamed / deleted counts, its
/// template or generator source, its config / dependency / task / extension impact, a named
/// checkpoint or rollback / delete-generated path, its generated-versus-user-owned boundary cue, its
/// derived review disposition and boundary posture, bounded review / rollback actions, and a stable
/// manifest / registry / docs / policy deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedProjectDiffCard {
    /// Frozen component this control implements; must be `generated_project_diff_card`.
    pub component: M5ScaffoldComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable card name; required and non-empty.
    pub card_name: String,
    /// Generated-zone class, reused from the frozen matrix.
    pub generated_zone_class: M5GeneratedZoneClass,
    /// Diff-review state, reused from the frozen matrix.
    pub diff_review_state: M5DiffReviewState,
    /// The template or generator source of the changes.
    pub source_kind: DiffSourceKind,
    /// Source label; always required so the template or generator source stays explicit.
    pub source_label: String,
    /// Count of created files.
    pub created_count: u32,
    /// Count of modified files.
    pub modified_count: u32,
    /// Count of renamed files.
    pub renamed_count: u32,
    /// Count of deleted files.
    pub deleted_count: u32,
    /// Change-summary note; always required so the create / modify / rename / delete counts stay
    /// explicit.
    pub change_summary_note: String,
    /// Derived review disposition (must equal the resolved disposition).
    pub derived_review_disposition: DiffReviewDisposition,
    /// Derived generated-versus-user-owned boundary posture (must equal the resolved posture).
    pub derived_boundary_posture: GeneratedBoundaryPosture,
    /// Whether the card claims the diff is reviewable (must equal derived truth).
    pub claims_reviewable: bool,
    /// Whether the card claims the diff is blocked (must equal derived truth).
    pub claims_blocking: bool,
    /// Whether the card claims the zone is user-owned (must equal derived truth).
    pub claims_user_owned_boundary: bool,
    /// Review-required note; required when review is required before write.
    pub review_required_note: String,
    /// No-changes note; required when there is nothing to review.
    pub no_changes_note: String,
    /// Conflict note; required when a conflict is blocking the apply.
    pub conflict_note: String,
    /// Diff-unavailable note; required when the diff is unavailable or blocked.
    pub unavailable_note: String,
    /// Generated-versus-user-owned boundary note; always required so the ownership boundary stays
    /// explicit.
    pub boundary_note: String,
    /// Config impact label; always required.
    pub config_impact_label: String,
    /// Dependency impact label; always required.
    pub dependency_impact_label: String,
    /// Task impact label; always required.
    pub task_impact_label: String,
    /// Extension impact label; always required.
    pub extension_impact_label: String,
    /// Checkpoint label; always required (the named checkpoint the rollback restores to).
    pub checkpoint_label: String,
    /// Rollback / delete-generated note; always required so recovery stays explicit.
    pub rollback_note: String,
    /// Context note; always required so the card names what to review before committing.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include review-diff / review-impact /
    /// review-boundary and a rollback-generated recovery path).
    pub card_actions: Vec<DiffCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides the generated-versus-user-owned boundary. MUST be `false`.
    pub hides_generated_versus_user_owned_boundary: bool,
    /// Hard invariant: never hides a side effect or trust state behind a generic action. MUST be
    /// `false`.
    pub hides_side_effect_or_trust_state: bool,
    /// Hard invariant: never assumes the safest next step for the user without keeping recovery
    /// explicit. MUST be `false`.
    pub assumes_safest_next_step_without_recovery: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl GeneratedProjectDiffCard {
    /// Boundary and review disclosures this card must carry, derived from the frozen fields.
    pub fn diff_disclosure(&self) -> DiffCardDisclosure {
        resolve_diff_disclosure(self.generated_zone_class, self.diff_review_state)
    }

    /// Total counted changes across the create / modify / rename / delete vocabulary.
    pub fn total_changes(&self) -> u32 {
        self.created_count + self.modified_count + self.renamed_count + self.deleted_count
    }

    /// The per-kind count for a change kind.
    fn count_for(&self, kind: DiffChangeKind) -> u32 {
        match kind {
            DiffChangeKind::Created => self.created_count,
            DiffChangeKind::Modified => self.modified_count,
            DiffChangeKind::Renamed => self.renamed_count,
            DiffChangeKind::Deleted => self.deleted_count,
        }
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DiffCardAction> = self.card_actions.iter().copied().collect();
        DiffCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card offers an explicit rollback / delete-generated recovery path.
    fn offers_rollback_generated(&self) -> bool {
        self.card_actions
            .contains(&DiffCardAction::RollbackGenerated)
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions.contains(&DiffCardAction::OpenDeepLink)
    }
}

// ---- scaffold-handoff-banner vocabulary ---------------------------------

/// The trust state a scaffold handoff banner reports for the created workspace, so a banner never
/// leaves whether the new workspace is trusted implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTrustState {
    /// The workspace is trusted.
    Trusted,
    /// A trust prompt is pending before the workspace can run code.
    TrustPromptPending,
    /// The workspace runs with restricted trust.
    RestrictedTrust,
    /// The workspace is untrusted and blocked from running code.
    UntrustedBlocked,
    /// Trust does not apply (an empty workspace with no runnable output).
    TrustNotApplicable,
}

impl HandoffTrustState {
    /// Every trust state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Trusted,
        Self::TrustPromptPending,
        Self::RestrictedTrust,
        Self::UntrustedBlocked,
        Self::TrustNotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::TrustPromptPending => "trust_prompt_pending",
            Self::RestrictedTrust => "restricted_trust",
            Self::UntrustedBlocked => "untrusted_blocked",
            Self::TrustNotApplicable => "trust_not_applicable",
        }
    }

    /// True only when the workspace is fully trusted.
    pub const fn is_trusted(self) -> bool {
        matches!(self, Self::Trusted)
    }

    /// True when the banner must carry an explicit trust note.
    pub const fn needs_trust_note(self) -> bool {
        matches!(
            self,
            Self::TrustPromptPending | Self::RestrictedTrust | Self::UntrustedBlocked
        )
    }
}

/// Derived outcome posture a scaffold handoff banner may present.
///
/// This is the handoff honesty axis: the posture is derived from the frozen outcome class, never
/// asserted, so a partial or failed bootstrap can never present as a clean create.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOutcomePosture {
    /// A clean create.
    CleanCreate,
    /// A partial bootstrap that needs recovery.
    PartialNeedsRecovery,
    /// A failed create that needs recovery.
    FailedNeedsRecovery,
    /// Continued without a starter.
    ContinuedWithoutStarter,
    /// Created empty.
    CreatedEmpty,
    /// Remote provisioning is pending.
    ProvisioningPending,
}

impl HandoffOutcomePosture {
    /// Every outcome posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CleanCreate,
        Self::PartialNeedsRecovery,
        Self::FailedNeedsRecovery,
        Self::ContinuedWithoutStarter,
        Self::CreatedEmpty,
        Self::ProvisioningPending,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanCreate => "clean_create",
            Self::PartialNeedsRecovery => "partial_needs_recovery",
            Self::FailedNeedsRecovery => "failed_needs_recovery",
            Self::ContinuedWithoutStarter => "continued_without_starter",
            Self::CreatedEmpty => "created_empty",
            Self::ProvisioningPending => "provisioning_pending",
        }
    }

    /// True only when the create is clean.
    pub const fn is_clean_create(self) -> bool {
        matches!(self, Self::CleanCreate)
    }

    /// True when the outcome needs a recovery path.
    pub const fn needs_recovery(self) -> bool {
        matches!(self, Self::PartialNeedsRecovery | Self::FailedNeedsRecovery)
    }
}

/// One keyboard-complete default action a scaffold handoff banner offers, so a banner never hides
/// its run, review, or manifest affordance behind a pointer-only gesture and keeps optional setup
/// visibly optional. `RunNow`, `RunLater`, `ReviewFiles`, and `OpenManifest` are always offered so
/// the safest next step is never assumed for the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBannerAction {
    /// Run the optional setup now (never the assumed default).
    RunNow,
    /// Run the optional setup later (keeps optional setup optional).
    RunLater,
    /// Review the generated files.
    ReviewFiles,
    /// Open the created workspace's manifest.
    OpenManifest,
    /// Reopen the preflight to reconsider the create.
    ReopenPreflight,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
}

impl HandoffBannerAction {
    /// Every handoff-banner action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunNow,
        Self::RunLater,
        Self::ReviewFiles,
        Self::OpenManifest,
        Self::ReopenPreflight,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete handoff banner must offer.
    pub const MANDATORY: [Self; 4] = [
        Self::RunNow,
        Self::RunLater,
        Self::ReviewFiles,
        Self::OpenManifest,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunNow => "run_now",
            Self::RunLater => "run_later",
            Self::ReviewFiles => "review_files",
            Self::OpenManifest => "open_manifest",
            Self::ReopenPreflight => "reopen_preflight",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// True when a recovery-action set offers a real recovery route (delete-generated,
/// continue-without-starter, retry, or keep-for-review) rather than only open-workspace or
/// no-recovery-needed.
fn offers_real_recovery(actions: &[M5HandoffRecoveryAction]) -> bool {
    actions.iter().any(|action| {
        matches!(
            action,
            M5HandoffRecoveryAction::DeleteGenerated
                | M5HandoffRecoveryAction::ContinueWithoutStarter
                | M5HandoffRecoveryAction::RetryBootstrap
                | M5HandoffRecoveryAction::KeepPartialReview
        )
    })
}

/// Disclosures a scaffold handoff banner must carry, derived from the frozen outcome class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffBannerDisclosure {
    /// The derived outcome posture this banner may present.
    pub outcome_posture: HandoffOutcomePosture,
    /// Whether the create is clean.
    pub is_clean_create: bool,
    /// Whether the outcome needs a recovery path.
    pub needs_recovery: bool,
    /// Whether the banner must carry an explicit partial-bootstrap note.
    pub needs_partial_note: bool,
    /// Whether the banner must carry an explicit failed-create note.
    pub needs_failed_note: bool,
    /// Whether the banner must carry an explicit provisioning-pending note.
    pub needs_pending_note: bool,
}

/// Resolves the outcome truth a scaffold handoff banner may present.
///
/// A `create_succeeded` outcome is a clean create, a `partial_bootstrap` outcome needs recovery, a
/// `create_failed` outcome needs recovery, a `continued_without_starter` outcome continued without a
/// starter, a `created_empty` outcome created empty, and a `provisioning_pending` outcome is pending
/// — so a partial or failed bootstrap can never read as a clean create.
pub fn resolve_handoff_disclosure(outcome_class: M5HandoffOutcomeClass) -> HandoffBannerDisclosure {
    use HandoffOutcomePosture as Posture;
    use M5HandoffOutcomeClass as Outcome;

    let outcome_posture = match outcome_class {
        Outcome::CreateSucceeded => Posture::CleanCreate,
        Outcome::PartialBootstrap => Posture::PartialNeedsRecovery,
        Outcome::CreateFailed => Posture::FailedNeedsRecovery,
        Outcome::ContinuedWithoutStarter => Posture::ContinuedWithoutStarter,
        Outcome::CreatedEmpty => Posture::CreatedEmpty,
        Outcome::ProvisioningPending => Posture::ProvisioningPending,
    };

    HandoffBannerDisclosure {
        outcome_posture,
        is_clean_create: outcome_posture.is_clean_create(),
        needs_recovery: outcome_posture.needs_recovery(),
        needs_partial_note: matches!(outcome_posture, Posture::PartialNeedsRecovery),
        needs_failed_note: matches!(outcome_posture, Posture::FailedNeedsRecovery),
        needs_pending_note: matches!(outcome_posture, Posture::ProvisioningPending),
    }
}

/// A scaffold handoff banner naming its created-workspace identity, its trust state, its health
/// summary, `Run now` / `Run later` / `Review files` / `Open manifest` choices, a delete-generated
/// or reopen-preflight recovery route, its derived outcome posture, bounded actions, and a stable
/// deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldHandoffBanner {
    /// Frozen component this control implements; must be `scaffold_handoff_banner`.
    pub component: M5ScaffoldComponentFamily,
    /// Stable banner id.
    pub banner_id: String,
    /// Human-readable banner name; required and non-empty.
    pub banner_name: String,
    /// Handoff outcome class, reused from the frozen matrix.
    pub outcome_class: M5HandoffOutcomeClass,
    /// Trust state of the created workspace.
    pub trust_state: HandoffTrustState,
    /// Created-workspace id label; always required so the workspace identity stays explicit.
    pub workspace_id_label: String,
    /// Created-workspace name label; always required.
    pub workspace_name_label: String,
    /// Health-summary label; always required so the post-create health stays explicit.
    pub health_summary_label: String,
    /// Derived outcome posture (must equal the resolved posture).
    pub derived_outcome_posture: HandoffOutcomePosture,
    /// Whether the banner claims a clean create (must equal derived truth).
    pub claims_clean_create: bool,
    /// Whether the banner claims the outcome needs recovery (must equal derived truth).
    pub claims_needs_recovery: bool,
    /// Whether the banner claims the workspace is trusted (must equal the trust-state truth).
    pub claims_trusted: bool,
    /// Partial-bootstrap note; required when the outcome is a partial bootstrap.
    pub partial_note: String,
    /// Failed-create note; required when the outcome is a failed create.
    pub failed_note: String,
    /// Provisioning-pending note; required when remote provisioning is pending.
    pub pending_note: String,
    /// Trust note; required when the trust state is not fully trusted.
    pub trust_note: String,
    /// Optional-setup note; always required so optional setup stays visibly optional.
    pub optional_setup_note: String,
    /// Recovery note; always required (a delete-generated or reopen-preflight route).
    pub recovery_note: String,
    /// Context note; always required so the banner names what to check before committing.
    pub context_note: String,
    /// Kind of stable deep link this banner binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include run-now / run-later / review-files /
    /// open-manifest).
    pub banner_actions: Vec<HandoffBannerAction>,
    /// Recovery actions this banner keeps explicit (must offer a real recovery route).
    pub recovery_actions: Vec<M5HandoffRecoveryAction>,
    /// Dispositions this banner binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this banner can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this banner can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this banner.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides the generated-versus-user-owned boundary. MUST be `false`.
    pub hides_generated_versus_user_owned_boundary: bool,
    /// Hard invariant: never hides a side effect or trust state behind a generic action. MUST be
    /// `false`.
    pub hides_side_effect_or_trust_state: bool,
    /// Hard invariant: never assumes the safest next step for the user without keeping recovery
    /// explicit. MUST be `false`.
    pub assumes_safest_next_step_without_recovery: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ScaffoldHandoffBanner {
    /// Outcome disclosures this banner must carry, derived from the frozen outcome class.
    pub fn outcome_disclosure(&self) -> HandoffBannerDisclosure {
        resolve_handoff_disclosure(self.outcome_class)
    }

    /// Whether the banner offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<HandoffBannerAction> = self.banner_actions.iter().copied().collect();
        HandoffBannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the banner offers a real recovery route.
    fn offers_real_recovery(&self) -> bool {
        offers_real_recovery(&self.recovery_actions)
    }

    /// Whether the banner declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the banner offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.banner_actions
            .contains(&HandoffBannerAction::OpenDeepLink)
    }
}

/// Whether a required-label list declares all three mandatory labels.
fn declares_mandatory_labels(labels: &[M5ScaffoldRequiredLabel]) -> bool {
    let present: BTreeSet<M5ScaffoldRequiredLabel> = labels.iter().copied().collect();
    M5ScaffoldRequiredLabel::MANDATORY
        .iter()
        .all(|label| present.contains(label))
}

// ---- review blocks ------------------------------------------------------

/// First-glance scaffold-generation review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldGenerationReview {
    /// The diff card names its created / modified / renamed / deleted counts.
    pub diff_card_shows_change_counts: bool,
    /// The diff card uses the same create / modify / rename / delete vocabulary as AI patches,
    /// importers, and refactors.
    pub diff_card_uses_create_modify_rename_delete_vocabulary: bool,
    /// The diff card names its generated-versus-user-owned boundary.
    pub diff_card_shows_generated_versus_user_owned_boundary: bool,
    /// The diff card names a rollback or delete-generated recovery path.
    pub diff_card_names_rollback_or_delete_generated_path: bool,
    /// The diff card names its config / dependency / task / extension impact.
    pub diff_card_shows_config_dependency_task_extension_impact: bool,
    /// The handoff banner names its created-workspace identity and trust state.
    pub handoff_banner_shows_workspace_identity_and_trust: bool,
    /// The handoff banner names its health summary.
    pub handoff_banner_shows_health_summary: bool,
    /// The handoff banner keeps optional setup visibly optional.
    pub handoff_banner_keeps_optional_setup_optional: bool,
    /// The handoff banner offers run-now / run-later / review-files / open-manifest.
    pub handoff_banner_offers_run_now_run_later_review_open_manifest: bool,
    /// The handoff banner preserves a delete-generated or reopen-preflight recovery route.
    pub handoff_banner_preserves_recovery_and_delete_generated: bool,
    /// Review disposition and outcome posture are derived from state, never asserted.
    pub review_and_outcome_derived_never_asserted: bool,
    /// A conflict or failed bootstrap is never shown as a clean create.
    pub conflict_or_failure_never_shown_as_clean: bool,
    /// A user-owned zone is never silently overwritten.
    pub user_owned_boundary_never_overwritten_silently: bool,
    /// The safest next step is never assumed for the user.
    pub safest_next_step_never_assumed_for_user: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ScaffoldGenerationReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diff_card_shows_change_counts
            && self.diff_card_uses_create_modify_rename_delete_vocabulary
            && self.diff_card_shows_generated_versus_user_owned_boundary
            && self.diff_card_names_rollback_or_delete_generated_path
            && self.diff_card_shows_config_dependency_task_extension_impact
            && self.handoff_banner_shows_workspace_identity_and_trust
            && self.handoff_banner_shows_health_summary
            && self.handoff_banner_keeps_optional_setup_optional
            && self.handoff_banner_offers_run_now_run_later_review_open_manifest
            && self.handoff_banner_preserves_recovery_and_delete_generated
            && self.review_and_outcome_derived_never_asserted
            && self.conflict_or_failure_never_shown_as_clean
            && self.user_owned_boundary_never_overwritten_silently
            && self.safest_next_step_never_assumed_for_user
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldGenerationConsumerProjection {
    /// The diff-review surface reads a single canonical source.
    pub diff_review_surface_reads_single_source: bool,
    /// The workspace-handoff surface reads a single canonical source.
    pub workspace_handoff_reads_single_source: bool,
    /// The start-center reads a single canonical source.
    pub start_center_reads_single_source: bool,
    /// Change counts are visible before commit.
    pub change_counts_visible_before_commit: bool,
    /// The recovery path is visible after create.
    pub recovery_path_visible_after_create: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl ScaffoldGenerationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diff_review_surface_reads_single_source
            && self.workspace_handoff_reads_single_source
            && self.start_center_reads_single_source
            && self.change_counts_visible_before_commit
            && self.recovery_path_visible_after_create
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldGenerationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Generated-project diff cards.
    pub diff_cards: Vec<GeneratedProjectDiffCard>,
    /// Scaffold handoff banners.
    pub handoff_banners: Vec<ScaffoldHandoffBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Scaffold-generation review block.
    pub generation_review: ScaffoldGenerationReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldGenerationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldGenerationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe generated-project-diff-card / scaffold-handoff-banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket {
    /// Record kind; must equal [`SCAFFOLD_GENERATION_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCAFFOLD_GENERATION_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Generated-project diff cards.
    pub diff_cards: Vec<GeneratedProjectDiffCard>,
    /// Scaffold handoff banners.
    pub handoff_banners: Vec<ScaffoldHandoffBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Scaffold-generation review block.
    pub generation_review: ScaffoldGenerationReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldGenerationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldGenerationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket {
    /// Builds a generated-project-diff-card / scaffold-handoff-banner controls packet from
    /// stable-lane input.
    pub fn new(input: GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacketInput) -> Self {
        Self {
            record_kind: SCAFFOLD_GENERATION_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_GENERATION_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            diff_cards: input.diff_cards,
            handoff_banners: input.handoff_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            generation_review: input.generation_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the generated-project-diff-card / scaffold-handoff-banner control invariants.
    pub fn validate(&self) -> Vec<ScaffoldGenerationControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SCAFFOLD_GENERATION_CONTROLS_RECORD_KIND {
            violations.push(ScaffoldGenerationControlsViolation::WrongRecordKind);
        }
        if self.schema_version != SCAFFOLD_GENERATION_CONTROLS_SCHEMA_VERSION {
            violations.push(ScaffoldGenerationControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ScaffoldGenerationControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_diff_cards(self, &mut violations);
        validate_handoff_banners(self, &mut violations);

        if !self.generation_review.all_hold() {
            violations.push(ScaffoldGenerationControlsViolation::GenerationReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ScaffoldGenerationControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ScaffoldGenerationControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("scaffold generation controls packet serializes"),
        ) {
            violations.push(ScaffoldGenerationControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scaffold generation controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,frozen_state,secondary_state,derived,blocking_or_recovery,deep_link_kind\n",
        );
        for card in &self.diff_cards {
            let disclosure = card.diff_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "generated_project_diff_card",
                csv_field(&card.card_id),
                card.generated_zone_class.as_str(),
                card.diff_review_state.as_str(),
                disclosure.review_disposition.as_str(),
                disclosure.is_blocking,
                card.deep_link_kind.as_str(),
            ));
        }
        for banner in &self.handoff_banners {
            let disclosure = banner.outcome_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "scaffold_handoff_banner",
                csv_field(&banner.banner_id),
                banner.outcome_class.as_str(),
                banner.trust_state.as_str(),
                disclosure.outcome_posture.as_str(),
                disclosure.needs_recovery,
                banner.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked_cards = self
            .diff_cards
            .iter()
            .filter(|card| card.diff_disclosure().is_blocking)
            .count();
        let recovery_banners = self
            .handoff_banners
            .iter()
            .filter(|banner| banner.outcome_disclosure().needs_recovery)
            .count();

        let mut out = String::new();
        out.push_str("# Generated-project diff cards and scaffold handoff banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Generated-project diff cards: {} ({} blocked)\n",
            self.diff_cards.len(),
            blocked_cards
        ));
        out.push_str(&format!(
            "- Scaffold handoff banners: {} ({} needing recovery)\n",
            self.handoff_banners.len(),
            recovery_banners
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Generated-project diff cards\n\n");
        for card in &self.diff_cards {
            out.push_str(&format!(
                "- **{}** — zone `{}` → `{}`, review `{}` → `{}`, source `{}`, +{}/~{}/»{}/-{} (created/modified/renamed/deleted), deep link `{}`\n",
                card.card_name,
                card.generated_zone_class.as_str(),
                card.diff_disclosure().boundary_posture.as_str(),
                card.diff_review_state.as_str(),
                card.diff_disclosure().review_disposition.as_str(),
                card.source_kind.as_str(),
                card.created_count,
                card.modified_count,
                card.renamed_count,
                card.deleted_count,
                card.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Scaffold handoff banners\n\n");
        for banner in &self.handoff_banners {
            out.push_str(&format!(
                "- **{}** — outcome `{}` → `{}`, trust `{}`, health `{}`, deep link `{}`\n",
                banner.banner_name,
                banner.outcome_class.as_str(),
                banner.outcome_disclosure().outcome_posture.as_str(),
                banner.trust_state.as_str(),
                banner.health_summary_label,
                banner.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in scaffold-generation controls export.
#[derive(Debug)]
pub enum ScaffoldGenerationControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScaffoldGenerationControlsViolation>),
}

impl fmt::Display for ScaffoldGenerationControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "scaffold generation controls export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "scaffold generation controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScaffoldGenerationControlsArtifactError {}

/// Validation failures emitted by
/// [`GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaffoldGenerationControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No generated-project diff cards are present.
    DiffCardsMissing,
    /// A generated-project diff card is incomplete.
    DiffCardIncomplete,
    /// A generated-project diff card carries the wrong frozen component class.
    DiffCardWrongComponentClass,
    /// A diff card misrepresents its derived review disposition, boundary posture, or claims.
    DiffDispositionMisrepresented,
    /// A diff card does not name its template or generator source.
    DiffSourceLabelMissing,
    /// A diff card does not name its change summary.
    DiffChangeSummaryMissing,
    /// A review-required diff card does not name its review-required state.
    DiffReviewRequiredNoteMissing,
    /// A no-changes diff card does not name its no-changes state.
    DiffNoChangesNoteMissing,
    /// A conflict diff card does not name its conflict.
    DiffConflictNoteMissing,
    /// A diff-unavailable / blocked diff card does not name its unavailable state.
    DiffUnavailableNoteMissing,
    /// A diff card does not name its generated-versus-user-owned boundary.
    DiffBoundaryNoteMissing,
    /// A diff card does not name its config impact.
    DiffConfigImpactMissing,
    /// A diff card does not name its dependency impact.
    DiffDependencyImpactMissing,
    /// A diff card does not name its task impact.
    DiffTaskImpactMissing,
    /// A diff card does not name its extension impact.
    DiffExtensionImpactMissing,
    /// A diff card does not name its checkpoint.
    DiffCheckpointMissing,
    /// A diff card does not name a rollback / delete-generated path.
    DiffRollbackNoteMissing,
    /// A diff card omits a mandatory review action.
    DiffCardActionsIncomplete,
    /// A diff card does not offer a rollback / delete-generated recovery action.
    DiffRollbackRecoveryMissing,
    /// The diff cards do not cover every generated-zone class.
    DiffZoneClassCoverageMissing,
    /// The diff cards do not cover every diff-review state.
    DiffReviewStateCoverageMissing,
    /// The diff cards do not cover every review disposition.
    DiffDispositionCoverageMissing,
    /// The diff cards do not cover every boundary posture.
    DiffBoundaryPostureCoverageMissing,
    /// The diff cards do not cover every create / modify / rename / delete change kind.
    DiffChangeKindCoverageMissing,
    /// The diff cards do not cover every diff source kind.
    DiffSourceKindCoverageMissing,
    /// No scaffold handoff banners are present.
    HandoffBannersMissing,
    /// A scaffold handoff banner is incomplete.
    HandoffBannerIncomplete,
    /// A scaffold handoff banner carries the wrong frozen component class.
    HandoffBannerWrongComponentClass,
    /// A handoff banner misrepresents its derived outcome posture or claims.
    HandoffOutcomeMisrepresented,
    /// A handoff banner does not name its created-workspace id.
    HandoffWorkspaceIdMissing,
    /// A handoff banner does not name its created-workspace name.
    HandoffWorkspaceNameMissing,
    /// A handoff banner does not name its health summary.
    HandoffHealthSummaryMissing,
    /// A partial-bootstrap banner does not name its partial state.
    HandoffPartialNoteMissing,
    /// A failed-create banner does not name its failed state.
    HandoffFailedNoteMissing,
    /// A provisioning-pending banner does not name its pending state.
    HandoffPendingNoteMissing,
    /// A not-fully-trusted banner does not name its trust state.
    HandoffTrustNoteMissing,
    /// A handoff banner does not name its optional-setup note.
    HandoffOptionalSetupNoteMissing,
    /// A handoff banner does not name its recovery route.
    HandoffRecoveryNoteMissing,
    /// A handoff banner omits a mandatory run / review / manifest action.
    HandoffBannerActionsIncomplete,
    /// A handoff banner does not offer a real recovery route.
    HandoffRecoveryPathMissing,
    /// The handoff banners do not cover every handoff outcome class.
    HandoffOutcomeClassCoverageMissing,
    /// The handoff banners do not cover every trust state.
    HandoffTrustStateCoverageMissing,
    /// The handoff banners do not cover every outcome posture.
    HandoffOutcomePostureCoverageMissing,
    /// The handoff banners do not cover every recovery action.
    HandoffRecoveryActionCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component hides the generated-versus-user-owned boundary.
    GeneratedBoundaryHidden,
    /// A component hides a side effect or trust state behind a generic action.
    SideEffectOrTrustStateHidden,
    /// A component assumes the safest next step without keeping recovery explicit.
    SafestNextStepAssumedWithoutRecovery,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Generation review does not satisfy required invariants.
    GenerationReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ScaffoldGenerationControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DiffCardsMissing => "diff_cards_missing",
            Self::DiffCardIncomplete => "diff_card_incomplete",
            Self::DiffCardWrongComponentClass => "diff_card_wrong_component_class",
            Self::DiffDispositionMisrepresented => "diff_disposition_misrepresented",
            Self::DiffSourceLabelMissing => "diff_source_label_missing",
            Self::DiffChangeSummaryMissing => "diff_change_summary_missing",
            Self::DiffReviewRequiredNoteMissing => "diff_review_required_note_missing",
            Self::DiffNoChangesNoteMissing => "diff_no_changes_note_missing",
            Self::DiffConflictNoteMissing => "diff_conflict_note_missing",
            Self::DiffUnavailableNoteMissing => "diff_unavailable_note_missing",
            Self::DiffBoundaryNoteMissing => "diff_boundary_note_missing",
            Self::DiffConfigImpactMissing => "diff_config_impact_missing",
            Self::DiffDependencyImpactMissing => "diff_dependency_impact_missing",
            Self::DiffTaskImpactMissing => "diff_task_impact_missing",
            Self::DiffExtensionImpactMissing => "diff_extension_impact_missing",
            Self::DiffCheckpointMissing => "diff_checkpoint_missing",
            Self::DiffRollbackNoteMissing => "diff_rollback_note_missing",
            Self::DiffCardActionsIncomplete => "diff_card_actions_incomplete",
            Self::DiffRollbackRecoveryMissing => "diff_rollback_recovery_missing",
            Self::DiffZoneClassCoverageMissing => "diff_zone_class_coverage_missing",
            Self::DiffReviewStateCoverageMissing => "diff_review_state_coverage_missing",
            Self::DiffDispositionCoverageMissing => "diff_disposition_coverage_missing",
            Self::DiffBoundaryPostureCoverageMissing => "diff_boundary_posture_coverage_missing",
            Self::DiffChangeKindCoverageMissing => "diff_change_kind_coverage_missing",
            Self::DiffSourceKindCoverageMissing => "diff_source_kind_coverage_missing",
            Self::HandoffBannersMissing => "handoff_banners_missing",
            Self::HandoffBannerIncomplete => "handoff_banner_incomplete",
            Self::HandoffBannerWrongComponentClass => "handoff_banner_wrong_component_class",
            Self::HandoffOutcomeMisrepresented => "handoff_outcome_misrepresented",
            Self::HandoffWorkspaceIdMissing => "handoff_workspace_id_missing",
            Self::HandoffWorkspaceNameMissing => "handoff_workspace_name_missing",
            Self::HandoffHealthSummaryMissing => "handoff_health_summary_missing",
            Self::HandoffPartialNoteMissing => "handoff_partial_note_missing",
            Self::HandoffFailedNoteMissing => "handoff_failed_note_missing",
            Self::HandoffPendingNoteMissing => "handoff_pending_note_missing",
            Self::HandoffTrustNoteMissing => "handoff_trust_note_missing",
            Self::HandoffOptionalSetupNoteMissing => "handoff_optional_setup_note_missing",
            Self::HandoffRecoveryNoteMissing => "handoff_recovery_note_missing",
            Self::HandoffBannerActionsIncomplete => "handoff_banner_actions_incomplete",
            Self::HandoffRecoveryPathMissing => "handoff_recovery_path_missing",
            Self::HandoffOutcomeClassCoverageMissing => "handoff_outcome_class_coverage_missing",
            Self::HandoffTrustStateCoverageMissing => "handoff_trust_state_coverage_missing",
            Self::HandoffOutcomePostureCoverageMissing => {
                "handoff_outcome_posture_coverage_missing"
            }
            Self::HandoffRecoveryActionCoverageMissing => {
                "handoff_recovery_action_coverage_missing"
            }
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::GeneratedBoundaryHidden => "generated_boundary_hidden",
            Self::SideEffectOrTrustStateHidden => "side_effect_or_trust_state_hidden",
            Self::SafestNextStepAssumedWithoutRecovery => {
                "safest_next_step_assumed_without_recovery"
            }
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GenerationReviewIncomplete => "generation_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable scaffold-generation controls export.
///
/// This is the first real consumer of the scaffold-generation component lane: a diff-review,
/// workspace-handoff, start-center, or support-export surface calls it to ingest the canonical
/// components rather than cloning status text.
///
/// # Errors
///
/// Returns [`ScaffoldGenerationControlsArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_scaffold_generation_controls_export() -> Result<
    GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket,
    ScaffoldGenerationControlsArtifactError,
> {
    let packet: GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-generated-project-diff-card-scaffold-handoff-banner-proof/support_export.json"
        )))
        .map_err(ScaffoldGenerationControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScaffoldGenerationControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket,
    violations: &mut Vec<ScaffoldGenerationControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF,
        SCAFFOLD_GENERATION_CONTROLS_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF,
        M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ScaffoldGenerationControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_diff_cards(
    packet: &GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket,
    violations: &mut Vec<ScaffoldGenerationControlsViolation>,
) {
    if packet.diff_cards.is_empty() {
        violations.push(ScaffoldGenerationControlsViolation::DiffCardsMissing);
        return;
    }

    let mut zones: BTreeSet<M5GeneratedZoneClass> = BTreeSet::new();
    let mut reviews: BTreeSet<M5DiffReviewState> = BTreeSet::new();
    let mut dispositions: BTreeSet<DiffReviewDisposition> = BTreeSet::new();
    let mut postures: BTreeSet<GeneratedBoundaryPosture> = BTreeSet::new();
    let mut sources: BTreeSet<DiffSourceKind> = BTreeSet::new();
    let mut change_kinds: BTreeSet<DiffChangeKind> = BTreeSet::new();

    for card in &packet.diff_cards {
        let disclosure = card.diff_disclosure();
        zones.insert(card.generated_zone_class);
        reviews.insert(card.diff_review_state);
        dispositions.insert(disclosure.review_disposition);
        postures.insert(disclosure.boundary_posture);
        sources.insert(card.source_kind);
        for kind in DiffChangeKind::ALL {
            if card.count_for(kind) > 0 {
                change_kinds.insert(kind);
            }
        }

        if card.card_id.trim().is_empty()
            || card.card_name.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(ScaffoldGenerationControlsViolation::DiffCardIncomplete);
        }
        if card.component != M5ScaffoldComponentFamily::GeneratedProjectDiffCard {
            violations.push(ScaffoldGenerationControlsViolation::DiffCardWrongComponentClass);
        }
        if card.derived_review_disposition != disclosure.review_disposition
            || card.derived_boundary_posture != disclosure.boundary_posture
            || card.claims_reviewable != disclosure.is_reviewable
            || card.claims_blocking != disclosure.is_blocking
            || card.claims_user_owned_boundary != disclosure.is_user_owned
        {
            violations.push(ScaffoldGenerationControlsViolation::DiffDispositionMisrepresented);
        }
        if card.source_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffSourceLabelMissing);
        }
        if card.change_summary_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffChangeSummaryMissing);
        }
        if disclosure.needs_review_required_note && card.review_required_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffReviewRequiredNoteMissing);
        }
        if disclosure.needs_no_changes_note && card.no_changes_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffNoChangesNoteMissing);
        }
        if disclosure.needs_conflict_note && card.conflict_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffConflictNoteMissing);
        }
        if disclosure.needs_unavailable_note && card.unavailable_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffUnavailableNoteMissing);
        }
        if card.boundary_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffBoundaryNoteMissing);
        }
        if card.config_impact_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffConfigImpactMissing);
        }
        if card.dependency_impact_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffDependencyImpactMissing);
        }
        if card.task_impact_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffTaskImpactMissing);
        }
        if card.extension_impact_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffExtensionImpactMissing);
        }
        if card.checkpoint_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffCheckpointMissing);
        }
        if card.rollback_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::DiffRollbackNoteMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(ScaffoldGenerationControlsViolation::DiffCardActionsIncomplete);
        }
        if !card.offers_rollback_generated() {
            violations.push(ScaffoldGenerationControlsViolation::DiffRollbackRecoveryMissing);
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                hides_generated_versus_user_owned_boundary: card
                    .hides_generated_versus_user_owned_boundary,
                hides_side_effect_or_trust_state: card.hides_side_effect_or_trust_state,
                assumes_safest_next_step_without_recovery: card
                    .assumes_safest_next_step_without_recovery,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in M5GeneratedZoneClass::ALL {
        if !zones.contains(&required) {
            violations.push(ScaffoldGenerationControlsViolation::DiffZoneClassCoverageMissing);
            break;
        }
    }
    for required in M5DiffReviewState::ALL {
        if !reviews.contains(&required) {
            violations.push(ScaffoldGenerationControlsViolation::DiffReviewStateCoverageMissing);
            break;
        }
    }
    for required in DiffReviewDisposition::ALL {
        if !dispositions.contains(&required) {
            violations.push(ScaffoldGenerationControlsViolation::DiffDispositionCoverageMissing);
            break;
        }
    }
    for required in GeneratedBoundaryPosture::ALL {
        if !postures.contains(&required) {
            violations
                .push(ScaffoldGenerationControlsViolation::DiffBoundaryPostureCoverageMissing);
            break;
        }
    }
    for required in DiffChangeKind::ALL {
        if !change_kinds.contains(&required) {
            violations.push(ScaffoldGenerationControlsViolation::DiffChangeKindCoverageMissing);
            break;
        }
    }
    for required in DiffSourceKind::ALL {
        if !sources.contains(&required) {
            violations.push(ScaffoldGenerationControlsViolation::DiffSourceKindCoverageMissing);
            break;
        }
    }
}

fn validate_handoff_banners(
    packet: &GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket,
    violations: &mut Vec<ScaffoldGenerationControlsViolation>,
) {
    if packet.handoff_banners.is_empty() {
        violations.push(ScaffoldGenerationControlsViolation::HandoffBannersMissing);
        return;
    }

    let mut outcomes: BTreeSet<M5HandoffOutcomeClass> = BTreeSet::new();
    let mut trusts: BTreeSet<HandoffTrustState> = BTreeSet::new();
    let mut postures: BTreeSet<HandoffOutcomePosture> = BTreeSet::new();
    let mut recoveries: BTreeSet<M5HandoffRecoveryAction> = BTreeSet::new();

    for banner in &packet.handoff_banners {
        let disclosure = banner.outcome_disclosure();
        outcomes.insert(banner.outcome_class);
        trusts.insert(banner.trust_state);
        postures.insert(disclosure.outcome_posture);
        for action in &banner.recovery_actions {
            recoveries.insert(*action);
        }

        if banner.banner_id.trim().is_empty()
            || banner.banner_name.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
            || banner.recovery_actions.is_empty()
        {
            violations.push(ScaffoldGenerationControlsViolation::HandoffBannerIncomplete);
        }
        if banner.component != M5ScaffoldComponentFamily::ScaffoldHandoffBanner {
            violations.push(ScaffoldGenerationControlsViolation::HandoffBannerWrongComponentClass);
        }
        if banner.derived_outcome_posture != disclosure.outcome_posture
            || banner.claims_clean_create != disclosure.is_clean_create
            || banner.claims_needs_recovery != disclosure.needs_recovery
            || banner.claims_trusted != banner.trust_state.is_trusted()
        {
            violations.push(ScaffoldGenerationControlsViolation::HandoffOutcomeMisrepresented);
        }
        if banner.workspace_id_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffWorkspaceIdMissing);
        }
        if banner.workspace_name_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffWorkspaceNameMissing);
        }
        if banner.health_summary_label.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffHealthSummaryMissing);
        }
        if disclosure.needs_partial_note && banner.partial_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffPartialNoteMissing);
        }
        if disclosure.needs_failed_note && banner.failed_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffFailedNoteMissing);
        }
        if disclosure.needs_pending_note && banner.pending_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffPendingNoteMissing);
        }
        if banner.trust_state.needs_trust_note() && banner.trust_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffTrustNoteMissing);
        }
        if banner.optional_setup_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffOptionalSetupNoteMissing);
        }
        if banner.recovery_note.trim().is_empty() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffRecoveryNoteMissing);
        }
        if !banner.declares_mandatory_actions() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffBannerActionsIncomplete);
        }
        if !banner.offers_real_recovery() {
            violations.push(ScaffoldGenerationControlsViolation::HandoffRecoveryPathMissing);
        }
        validate_deep_link(
            banner.offers_deep_link_action(),
            banner.deep_link_kind,
            &banner.deep_link_ref,
            &banner.context_note,
            violations,
        );
        validate_common_control(
            &banner.dispositions,
            &banner.downgrade_triggers,
            banner.declares_mandatory_labels(),
            &banner.accessibility_routes,
            ControlInvariants {
                hides_generated_versus_user_owned_boundary: banner
                    .hides_generated_versus_user_owned_boundary,
                hides_side_effect_or_trust_state: banner.hides_side_effect_or_trust_state,
                assumes_safest_next_step_without_recovery: banner
                    .assumes_safest_next_step_without_recovery,
                invents_alternate_state_label: banner.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in M5HandoffOutcomeClass::ALL {
        if !outcomes.contains(&required) {
            violations
                .push(ScaffoldGenerationControlsViolation::HandoffOutcomeClassCoverageMissing);
            break;
        }
    }
    for required in HandoffTrustState::ALL {
        if !trusts.contains(&required) {
            violations.push(ScaffoldGenerationControlsViolation::HandoffTrustStateCoverageMissing);
            break;
        }
    }
    for required in HandoffOutcomePosture::ALL {
        if !postures.contains(&required) {
            violations
                .push(ScaffoldGenerationControlsViolation::HandoffOutcomePostureCoverageMissing);
            break;
        }
    }
    for required in M5HandoffRecoveryAction::ALL {
        if !recoveries.contains(&required) {
            violations
                .push(ScaffoldGenerationControlsViolation::HandoffRecoveryActionCoverageMissing);
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<ScaffoldGenerationControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ScaffoldGenerationControlsViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(ScaffoldGenerationControlsViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(ScaffoldGenerationControlsViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    hides_generated_versus_user_owned_boundary: bool,
    hides_side_effect_or_trust_state: bool,
    assumes_safest_next_step_without_recovery: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5ScaffoldDisposition],
    downgrade_triggers: &[M5ScaffoldDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ScaffoldAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<ScaffoldGenerationControlsViolation>,
) {
    if dispositions.is_empty() {
        violations.push(ScaffoldGenerationControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ScaffoldGenerationControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(ScaffoldGenerationControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(ScaffoldGenerationControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.hides_generated_versus_user_owned_boundary {
        violations.push(ScaffoldGenerationControlsViolation::GeneratedBoundaryHidden);
    }
    if invariants.hides_side_effect_or_trust_state {
        violations.push(ScaffoldGenerationControlsViolation::SideEffectOrTrustStateHidden);
    }
    if invariants.assumes_safest_next_step_without_recovery {
        violations.push(ScaffoldGenerationControlsViolation::SafestNextStepAssumedWithoutRecovery);
    }
    if invariants.invents_alternate_state_label {
        violations.push(ScaffoldGenerationControlsViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
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

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical scaffold-generation controls packet.
pub const SCAFFOLD_GENERATION_CONTROLS_PACKET_ID: &str =
    "m5-generated-project-diff-card-scaffold-handoff-banner-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn diff_card_source_refs() -> Vec<String> {
    strings(&[
        M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn handoff_banner_source_refs() -> Vec<String> {
    strings(&[
        M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn diff_card_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::GeneratedBoundaryBlurred,
        M5ScaffoldDowngradeTrigger::ImpactUndisclosed,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn handoff_banner_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

/// The three mandatory labels plus any extra truth labels.
fn label_set(extra: &[M5ScaffoldRequiredLabel]) -> Vec<M5ScaffoldRequiredLabel> {
    let mut labels = M5ScaffoldRequiredLabel::MANDATORY.to_vec();
    labels.extend_from_slice(extra);
    labels
}

/// Input for [`diff_card`], grouped so the seed builder stays under the argument limit and reads as
/// one diff scenario.
struct DiffCardSeed<'a> {
    card_id: &'a str,
    card_name: &'a str,
    generated_zone_class: M5GeneratedZoneClass,
    diff_review_state: M5DiffReviewState,
    source_kind: DiffSourceKind,
    source_label: &'a str,
    created_count: u32,
    modified_count: u32,
    renamed_count: u32,
    deleted_count: u32,
    boundary_note: &'a str,
    config_impact_label: &'a str,
    dependency_impact_label: &'a str,
    task_impact_label: &'a str,
    extension_impact_label: &'a str,
    checkpoint_label: &'a str,
    rollback_note: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    card_actions: Vec<DiffCardAction>,
    dispositions: Vec<M5ScaffoldDisposition>,
}

/// Builds a generated-project diff card, deriving the review disposition, boundary posture, and the
/// required notes from the honest inputs so the seed is always self-consistent with the resolver.
fn diff_card(seed: DiffCardSeed<'_>) -> GeneratedProjectDiffCard {
    let disclosure = resolve_diff_disclosure(seed.generated_zone_class, seed.diff_review_state);
    GeneratedProjectDiffCard {
        component: M5ScaffoldComponentFamily::GeneratedProjectDiffCard,
        card_id: seed.card_id.to_owned(),
        card_name: seed.card_name.to_owned(),
        generated_zone_class: seed.generated_zone_class,
        diff_review_state: seed.diff_review_state,
        source_kind: seed.source_kind,
        source_label: seed.source_label.to_owned(),
        created_count: seed.created_count,
        modified_count: seed.modified_count,
        renamed_count: seed.renamed_count,
        deleted_count: seed.deleted_count,
        change_summary_note: format!(
            "{} created, {} modified, {} renamed, {} deleted",
            seed.created_count, seed.modified_count, seed.renamed_count, seed.deleted_count
        ),
        derived_review_disposition: disclosure.review_disposition,
        derived_boundary_posture: disclosure.boundary_posture,
        claims_reviewable: disclosure.is_reviewable,
        claims_blocking: disclosure.is_blocking,
        claims_user_owned_boundary: disclosure.is_user_owned,
        review_required_note: if disclosure.needs_review_required_note {
            "Review is required before any write is applied".to_owned()
        } else {
            String::new()
        },
        no_changes_note: if disclosure.needs_no_changes_note {
            "No changes were produced; there is nothing to review".to_owned()
        } else {
            String::new()
        },
        conflict_note: if disclosure.needs_conflict_note {
            "A conflict is blocking the apply; resolve it before the diff can be applied".to_owned()
        } else {
            String::new()
        },
        unavailable_note: if disclosure.needs_unavailable_note {
            "The diff is unavailable or blocked; it cannot be applied until it is recomputed"
                .to_owned()
        } else {
            String::new()
        },
        boundary_note: seed.boundary_note.to_owned(),
        config_impact_label: seed.config_impact_label.to_owned(),
        dependency_impact_label: seed.dependency_impact_label.to_owned(),
        task_impact_label: seed.task_impact_label.to_owned(),
        extension_impact_label: seed.extension_impact_label.to_owned(),
        checkpoint_label: seed.checkpoint_label.to_owned(),
        rollback_note: seed.rollback_note.to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        card_actions: seed.card_actions,
        dispositions: seed.dispositions,
        downgrade_triggers: diff_card_downgrade_triggers(),
        required_labels: label_set(&[M5ScaffoldRequiredLabel::RecoveryAndOwnershipBoundary]),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "created_count",
            "modified_count",
            "renamed_count",
            "deleted_count",
            "source_label",
            "generated_zone_class",
            "diff_review_state",
            "rollback_note",
        ]),
        source_contract_refs: diff_card_source_refs(),
        hides_generated_versus_user_owned_boundary: false,
        hides_side_effect_or_trust_state: false,
        assumes_safest_next_step_without_recovery: false,
        invents_alternate_state_label: false,
    }
}

/// Input for [`handoff_banner`], grouped so the seed builder stays under the argument limit and
/// reads as one handoff scenario.
struct HandoffBannerSeed<'a> {
    banner_id: &'a str,
    banner_name: &'a str,
    outcome_class: M5HandoffOutcomeClass,
    trust_state: HandoffTrustState,
    workspace_id_label: &'a str,
    workspace_name_label: &'a str,
    health_summary_label: &'a str,
    trust_note: &'a str,
    optional_setup_note: &'a str,
    recovery_note: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    banner_actions: Vec<HandoffBannerAction>,
    recovery_actions: Vec<M5HandoffRecoveryAction>,
    dispositions: Vec<M5ScaffoldDisposition>,
}

/// Builds a scaffold handoff banner, deriving the outcome posture, clean / recovery / trusted
/// claims, and the required notes from the honest inputs so the seed is always self-consistent with
/// the resolver.
fn handoff_banner(seed: HandoffBannerSeed<'_>) -> ScaffoldHandoffBanner {
    let disclosure = resolve_handoff_disclosure(seed.outcome_class);
    ScaffoldHandoffBanner {
        component: M5ScaffoldComponentFamily::ScaffoldHandoffBanner,
        banner_id: seed.banner_id.to_owned(),
        banner_name: seed.banner_name.to_owned(),
        outcome_class: seed.outcome_class,
        trust_state: seed.trust_state,
        workspace_id_label: seed.workspace_id_label.to_owned(),
        workspace_name_label: seed.workspace_name_label.to_owned(),
        health_summary_label: seed.health_summary_label.to_owned(),
        derived_outcome_posture: disclosure.outcome_posture,
        claims_clean_create: disclosure.is_clean_create,
        claims_needs_recovery: disclosure.needs_recovery,
        claims_trusted: seed.trust_state.is_trusted(),
        partial_note: if disclosure.needs_partial_note {
            "The bootstrap only partially completed; review and recover the remaining steps"
                .to_owned()
        } else {
            String::new()
        },
        failed_note: if disclosure.needs_failed_note {
            "The create failed; recover by deleting the generated output or retrying".to_owned()
        } else {
            String::new()
        },
        pending_note: if disclosure.needs_pending_note {
            "Remote provisioning is still pending; the workspace is not fully ready yet".to_owned()
        } else {
            String::new()
        },
        trust_note: if seed.trust_state.needs_trust_note() {
            seed.trust_note.to_owned()
        } else {
            String::new()
        },
        optional_setup_note: seed.optional_setup_note.to_owned(),
        recovery_note: seed.recovery_note.to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        banner_actions: seed.banner_actions,
        recovery_actions: seed.recovery_actions,
        dispositions: seed.dispositions,
        downgrade_triggers: handoff_banner_downgrade_triggers(),
        required_labels: label_set(&[
            M5ScaffoldRequiredLabel::RecoveryAndOwnershipBoundary,
            M5ScaffoldRequiredLabel::SideEffectDisclosure,
        ]),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "workspace_name_label",
            "trust_state",
            "health_summary_label",
            "outcome_class",
            "optional_setup_note",
            "recovery_note",
        ]),
        source_contract_refs: handoff_banner_source_refs(),
        hides_generated_versus_user_owned_boundary: false,
        hides_side_effect_or_trust_state: false,
        assumes_safest_next_step_without_recovery: false,
        invents_alternate_state_label: false,
    }
}

fn diff_cards() -> Vec<GeneratedProjectDiffCard> {
    use DeepLinkKind as Link;
    use DiffCardAction as Action;
    use DiffSourceKind as Source;
    use M5DiffReviewState as State;
    use M5GeneratedZoneClass as Zone;
    use M5ScaffoldDisposition as Disp;

    vec![
        // 1. Generated only / preview ready -> reviewable preview + generated-owned.
        diff_card(DiffCardSeed {
            card_id: "diff-generated",
            card_name: "Generated project files",
            generated_zone_class: Zone::GeneratedOnly,
            diff_review_state: State::PreviewReady,
            source_kind: Source::TemplateStarter,
            source_label: "Template starter: react-spa",
            created_count: 24,
            modified_count: 0,
            renamed_count: 0,
            deleted_count: 0,
            boundary_note:
                "Every file is generated and generator-owned; nothing user-owned is touched",
            config_impact_label: "1 config file written",
            dependency_impact_label: "18 dependencies added",
            task_impact_label: "1 setup task",
            extension_impact_label: "1 recommended extension",
            checkpoint_label: "Checkpoint before generation",
            rollback_note: "Roll back to the checkpoint or delete the generated files",
            context_note: "The starter wrote 24 files; review the diff before keeping it",
            deep_link_kind: Link::TemplateManifest,
            deep_link_ref: "manifest:starters/react-spa#diff",
            card_actions: vec![
                Action::ReviewGeneratedDiff,
                Action::ReviewChangeImpact,
                Action::ReviewOwnershipBoundary,
                Action::RollbackGenerated,
                Action::KeepGenerated,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::FirstParty],
        }),
        // 2. User owned / review required -> review required before write + user-owned.
        diff_card(DiffCardSeed {
            card_id: "diff-user-owned",
            card_name: "User-owned files touched",
            generated_zone_class: Zone::UserOwned,
            diff_review_state: State::ReviewRequired,
            source_kind: Source::UserAuthored,
            source_label: "User-authored: existing app entry",
            created_count: 0,
            modified_count: 3,
            renamed_count: 0,
            deleted_count: 0,
            boundary_note: "These files are user-owned; review is required before any overwrite",
            config_impact_label: "No config change",
            dependency_impact_label: "No dependency change",
            task_impact_label: "No task change",
            extension_impact_label: "No extension change",
            checkpoint_label: "Checkpoint before edit",
            rollback_note: "Roll back to the checkpoint to restore the user-owned files",
            context_note: "3 user-owned files would be modified; approve each before writing",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/generated-diff",
            card_actions: vec![
                Action::ReviewGeneratedDiff,
                Action::ReviewChangeImpact,
                Action::ReviewOwnershipBoundary,
                Action::RollbackGenerated,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 3. Generated then edited / conflict detected -> conflict blocked + generated-then-edited.
        diff_card(DiffCardSeed {
            card_id: "diff-conflict",
            card_name: "Regeneration conflict",
            generated_zone_class: Zone::GeneratedThenEdited,
            diff_review_state: State::ConflictDetected,
            source_kind: Source::FrameworkGenerator,
            source_label: "Framework generator: nest resource",
            created_count: 0,
            modified_count: 4,
            renamed_count: 2,
            deleted_count: 0,
            boundary_note:
                "These files were generated and then hand-edited; a conflict blocks the apply",
            config_impact_label: "No config change",
            dependency_impact_label: "No dependency change",
            task_impact_label: "No task change",
            extension_impact_label: "No extension change",
            checkpoint_label: "Checkpoint before regenerate",
            rollback_note: "Roll back to the checkpoint or delete the regenerated output",
            context_note: "A conflict between generated and hand-edited files blocks this apply",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/regeneration-conflicts",
            card_actions: vec![
                Action::ReviewGeneratedDiff,
                Action::ReviewChangeImpact,
                Action::ReviewOwnershipBoundary,
                Action::RollbackGenerated,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Blocked],
        }),
        // 4. Runtime only / no changes -> nothing to review + runtime-only.
        diff_card(DiffCardSeed {
            card_id: "diff-runtime",
            card_name: "Runtime-only output",
            generated_zone_class: Zone::RuntimeOnly,
            diff_review_state: State::NoChanges,
            source_kind: Source::Codemod,
            source_label: "Codemod: build cache refresh",
            created_count: 0,
            modified_count: 0,
            renamed_count: 0,
            deleted_count: 0,
            boundary_note:
                "This zone is runtime-only (caches, build output); it is never user-owned",
            config_impact_label: "No config change",
            dependency_impact_label: "No dependency change",
            task_impact_label: "No task change",
            extension_impact_label: "No extension change",
            checkpoint_label: "Checkpoint before refresh",
            rollback_note: "Roll back to the checkpoint or delete the runtime output",
            context_note: "No source changes were produced; only runtime output would refresh",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/runtime-zones",
            card_actions: vec![
                Action::ReviewGeneratedDiff,
                Action::ReviewChangeImpact,
                Action::ReviewOwnershipBoundary,
                Action::RollbackGenerated,
                Action::KeepGenerated,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Optional],
        }),
        // 5. Mixed zone / diff unavailable -> unavailable / blocked + mixed ownership.
        diff_card(DiffCardSeed {
            card_id: "diff-mixed",
            card_name: "Mixed-ownership tree",
            generated_zone_class: Zone::MixedZone,
            diff_review_state: State::DiffUnavailable,
            source_kind: Source::ImportedSource,
            source_label: "Imported source: cloned repository",
            created_count: 6,
            modified_count: 5,
            renamed_count: 0,
            deleted_count: 4,
            boundary_note:
                "This tree mixes generated and user-owned files; review the boundary before writing",
            config_impact_label: "2 config files affected",
            dependency_impact_label: "Dependency impact unknown until the diff is available",
            task_impact_label: "Task impact unknown",
            extension_impact_label: "No extension change",
            checkpoint_label: "Checkpoint before import",
            rollback_note: "Roll back to the checkpoint or delete the imported output",
            context_note:
                "The diff could not be computed for this mixed-ownership tree; recompute it",
            deep_link_kind: Link::StarterRegistryEntry,
            deep_link_ref: "registry:team/import-starter#diff",
            card_actions: vec![
                Action::ReviewGeneratedDiff,
                Action::ReviewChangeImpact,
                Action::ReviewOwnershipBoundary,
                Action::RollbackGenerated,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 6. Zone unknown / blocked -> unavailable / blocked + ownership unknown.
        diff_card(DiffCardSeed {
            card_id: "diff-unknown",
            card_name: "Unknown-ownership zone",
            generated_zone_class: Zone::ZoneUnknown,
            diff_review_state: State::Blocked,
            source_kind: Source::FrameworkGenerator,
            source_label: "Framework generator: unresolved scope",
            created_count: 0,
            modified_count: 2,
            renamed_count: 0,
            deleted_count: 0,
            boundary_note: "Ownership of this zone is unknown; review is required before any write",
            config_impact_label: "Config impact unknown",
            dependency_impact_label: "No dependency change",
            task_impact_label: "No task change",
            extension_impact_label: "No extension change",
            checkpoint_label: "Checkpoint before write",
            rollback_note: "Roll back to the checkpoint or delete any generated output",
            context_note: "This write is blocked until the zone ownership is resolved",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/generated-zones",
            card_actions: vec![
                Action::ReviewGeneratedDiff,
                Action::ReviewChangeImpact,
                Action::ReviewOwnershipBoundary,
                Action::RollbackGenerated,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Blocked],
        }),
    ]
}

fn handoff_banners() -> Vec<ScaffoldHandoffBanner> {
    use DeepLinkKind as Link;
    use HandoffBannerAction as Action;
    use HandoffTrustState as Trust;
    use M5HandoffOutcomeClass as Outcome;
    use M5HandoffRecoveryAction as Recovery;
    use M5ScaffoldDisposition as Disp;

    vec![
        // 1. Create succeeded / trusted -> clean create.
        handoff_banner(HandoffBannerSeed {
            banner_id: "handoff-succeeded",
            banner_name: "Workspace created",
            outcome_class: Outcome::CreateSucceeded,
            trust_state: Trust::Trusted,
            workspace_id_label: "ws-web-0001",
            workspace_name_label: "web",
            health_summary_label: "All preflight checks passing",
            trust_note: "",
            optional_setup_note: "Running setup now is optional; you can run it later or skip it",
            recovery_note: "You can still delete the generated output if you change your mind",
            context_note: "The workspace was created cleanly; optional setup remains your choice",
            deep_link_kind: Link::TemplateManifest,
            deep_link_ref: "manifest:starters/react-spa#handoff",
            banner_actions: vec![
                Action::RunNow,
                Action::RunLater,
                Action::ReviewFiles,
                Action::OpenManifest,
                Action::OpenDeepLink,
            ],
            recovery_actions: vec![
                Recovery::OpenWorkspace,
                Recovery::NoRecoveryNeeded,
                Recovery::DeleteGenerated,
            ],
            dispositions: vec![Disp::FirstParty],
        }),
        // 2. Partial bootstrap / trust prompt pending -> partial needs recovery.
        handoff_banner(HandoffBannerSeed {
            banner_id: "handoff-partial",
            banner_name: "Workspace partially created",
            outcome_class: Outcome::PartialBootstrap,
            trust_state: Trust::TrustPromptPending,
            workspace_id_label: "ws-web-0002",
            workspace_name_label: "web",
            health_summary_label: "Some setup steps did not finish",
            trust_note: "A trust prompt is pending before the workspace can run code",
            optional_setup_note: "Finishing setup now is optional; the remaining steps can wait",
            recovery_note:
                "Keep the partial output for review, retry, or delete the generated output",
            context_note: "The bootstrap only partially completed; recover before relying on it",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/partial-bootstrap",
            banner_actions: vec![
                Action::RunNow,
                Action::RunLater,
                Action::ReviewFiles,
                Action::OpenManifest,
                Action::ReopenPreflight,
                Action::OpenDeepLink,
            ],
            recovery_actions: vec![
                Recovery::KeepPartialReview,
                Recovery::RetryBootstrap,
                Recovery::DeleteGenerated,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 3. Create failed / untrusted blocked -> failed needs recovery.
        handoff_banner(HandoffBannerSeed {
            banner_id: "handoff-failed",
            banner_name: "Workspace create failed",
            outcome_class: Outcome::CreateFailed,
            trust_state: Trust::UntrustedBlocked,
            workspace_id_label: "ws-api-0003",
            workspace_name_label: "api",
            health_summary_label: "Create failed before completion",
            trust_note: "The workspace is untrusted and blocked from running code",
            optional_setup_note: "No setup will run; recovery is the only next step offered",
            recovery_note:
                "Delete the generated output, retry the bootstrap, or continue without a starter",
            context_note: "The create failed; recover instead of assuming a safe next step",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/trust",
            banner_actions: vec![
                Action::RunNow,
                Action::RunLater,
                Action::ReviewFiles,
                Action::OpenManifest,
                Action::ReopenPreflight,
                Action::OpenDeepLink,
            ],
            recovery_actions: vec![
                Recovery::RetryBootstrap,
                Recovery::DeleteGenerated,
                Recovery::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::Blocked],
        }),
        // 4. Continued without starter / restricted trust -> continued without starter.
        handoff_banner(HandoffBannerSeed {
            banner_id: "handoff-continued",
            banner_name: "Continued without a starter",
            outcome_class: Outcome::ContinuedWithoutStarter,
            trust_state: Trust::RestrictedTrust,
            workspace_id_label: "ws-web-0004",
            workspace_name_label: "web",
            health_summary_label: "No starter applied; workspace is empty of generated files",
            trust_note: "The workspace runs with restricted trust",
            optional_setup_note: "You chose to continue without a starter; setup remains optional",
            recovery_note: "You can still open the workspace or apply a starter later",
            context_note: "You continued without a starter; nothing was generated to recover",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/continue-without-starter",
            banner_actions: vec![
                Action::RunNow,
                Action::RunLater,
                Action::ReviewFiles,
                Action::OpenManifest,
                Action::OpenDeepLink,
            ],
            recovery_actions: vec![Recovery::ContinueWithoutStarter, Recovery::OpenWorkspace],
            dispositions: vec![Disp::ContinueWithoutStarter],
        }),
        // 5. Created empty / trust not applicable -> created empty.
        handoff_banner(HandoffBannerSeed {
            banner_id: "handoff-empty",
            banner_name: "Created empty workspace",
            outcome_class: Outcome::CreatedEmpty,
            trust_state: Trust::TrustNotApplicable,
            workspace_id_label: "ws-web-0005",
            workspace_name_label: "web",
            health_summary_label: "Empty workspace; no runnable output yet",
            trust_note: "",
            optional_setup_note: "The workspace is empty; adding a starter later is optional",
            recovery_note: "You can continue without a starter or open the empty workspace",
            context_note: "You created an empty workspace; no generated output exists to recover",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/create-empty",
            banner_actions: vec![
                Action::RunNow,
                Action::RunLater,
                Action::ReviewFiles,
                Action::OpenManifest,
                Action::OpenDeepLink,
            ],
            recovery_actions: vec![Recovery::OpenWorkspace, Recovery::ContinueWithoutStarter],
            dispositions: vec![Disp::CreateEmpty],
        }),
        // 6. Provisioning pending / trust prompt pending -> provisioning pending.
        handoff_banner(HandoffBannerSeed {
            banner_id: "handoff-provisioning",
            banner_name: "Remote provisioning pending",
            outcome_class: Outcome::ProvisioningPending,
            trust_state: Trust::TrustPromptPending,
            workspace_id_label: "ws-api-0006",
            workspace_name_label: "api",
            health_summary_label: "Local files ready; remote provisioning still in progress",
            trust_note: "A trust prompt is pending before the remote workspace can run code",
            optional_setup_note: "Running setup now is optional while provisioning finishes",
            recovery_note: "Retry provisioning or delete the generated output if you cancel",
            context_note: "Remote provisioning is pending; the workspace is not fully ready yet",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/provisioning",
            banner_actions: vec![
                Action::RunNow,
                Action::RunLater,
                Action::ReviewFiles,
                Action::OpenManifest,
                Action::ReopenPreflight,
                Action::OpenDeepLink,
            ],
            recovery_actions: vec![Recovery::RetryBootstrap, Recovery::DeleteGenerated],
            dispositions: vec![Disp::Warning],
        }),
    ]
}

fn downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::GeneratedBoundaryBlurred,
        M5ScaffoldDowngradeTrigger::ImpactUndisclosed,
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn generation_review() -> ScaffoldGenerationReview {
    ScaffoldGenerationReview {
        diff_card_shows_change_counts: true,
        diff_card_uses_create_modify_rename_delete_vocabulary: true,
        diff_card_shows_generated_versus_user_owned_boundary: true,
        diff_card_names_rollback_or_delete_generated_path: true,
        diff_card_shows_config_dependency_task_extension_impact: true,
        handoff_banner_shows_workspace_identity_and_trust: true,
        handoff_banner_shows_health_summary: true,
        handoff_banner_keeps_optional_setup_optional: true,
        handoff_banner_offers_run_now_run_later_review_open_manifest: true,
        handoff_banner_preserves_recovery_and_delete_generated: true,
        review_and_outcome_derived_never_asserted: true,
        conflict_or_failure_never_shown_as_clean: true,
        user_owned_boundary_never_overwritten_silently: true,
        safest_next_step_never_assumed_for_user: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ScaffoldGenerationConsumerProjection {
    ScaffoldGenerationConsumerProjection {
        diff_review_surface_reads_single_source: true,
        workspace_handoff_reads_single_source: true,
        start_center_reads_single_source: true,
        change_counts_visible_before_commit: true,
        recovery_path_visible_after_create: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> ScaffoldGenerationProofFreshness {
    ScaffoldGenerationProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF,
        SCAFFOLD_GENERATION_CONTROLS_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF,
        M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical generated-project-diff-card / scaffold-handoff-banner controls packet.
pub fn seeded_scaffold_generation_controls(
) -> GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket {
    GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket::new(
        GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacketInput {
            packet_id: SCAFFOLD_GENERATION_CONTROLS_PACKET_ID.to_owned(),
            surface_label:
                "M5 generated-project diff cards and scaffold handoff banners: create/modify/rename/delete counts, dependency-task-extension impact, trust state, and run-now/later/review recovery across claimed generation flows"
                    .to_owned(),
            diff_cards: diff_cards(),
            handoff_banners: handoff_banners(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
            generation_review: generation_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a regeneration-conflict diff card that must never present a
/// user-edited zone as a clean applied change. Every generated-zone class, diff-review state,
/// review disposition, boundary posture, change kind, and source kind stays covered so the fixture
/// validates on its own.
pub fn seeded_scaffold_generation_controls_diff_card_conflict(
) -> GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket {
    let mut packet = seeded_scaffold_generation_controls();
    packet.packet_id =
        "m5-generated-project-diff-card-scaffold-handoff-banner-controls:fixture:diff-card-conflict"
            .to_owned();
    packet.surface_label =
        "M5 generated-project diff cards: a regeneration conflict never reads as a clean applied change"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a partial-bootstrap handoff banner that must keep its
/// delete-generated and retry recovery routes explicit rather than assuming a safe next step. Every
/// outcome class, trust state, outcome posture, and recovery action stays covered so the fixture
/// validates on its own.
pub fn seeded_scaffold_generation_controls_handoff_banner_partial(
) -> GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket {
    let mut packet = seeded_scaffold_generation_controls();
    packet.packet_id =
        "m5-generated-project-diff-card-scaffold-handoff-banner-controls:fixture:handoff-banner-partial"
            .to_owned();
    packet.surface_label =
        "M5 scaffold handoff banners: a partial bootstrap keeps delete-generated recovery explicit"
            .to_owned();
    packet
}
