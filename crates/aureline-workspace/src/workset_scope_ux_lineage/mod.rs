//! Workset, sparse-scope, and policy-limited-view UX lineage: the
//! governed, export-safe projection that proves how named worksets,
//! sparse slices, and policy-limited views surface across workspace
//! UX so the user can always distinguish `outside current slice`,
//! `omitted path`, and `policy hidden` content from `no result`, and
//! so that widening from a sparse or partial view to a fuller
//! checkout/workset preserves root identity, query/session continuity,
//! and restore provenance instead of creating a second ambiguous
//! workspace truth.
//!
//! The projection ingests a live [`WorksetScopeUxInputs`] envelope
//! verbatim (one [`ScopeObservation`] per governed scope, one
//! [`SurfaceObservation`] per UX surface that labels a scope, one
//! [`WidenPreviewObservation`] per widen-scope preview path, plus the
//! controlled inspection-hook table) and produces a lineage record
//! that proves the contract claims the stable line is anchored on:
//!
//! - **Scope-class coverage truth.** Every governed scope class
//!   (`selected_workset`, `sparse_slice`, `policy_limited_view`,
//!   `full_workspace`) ships a row bound to one closed
//!   [`ScopeKind`]; `current_repo` rides optionally.
//! - **Surface coverage truth.** Every required UX surface
//!   (`workset_switcher`, `scope_chip`, `search`, `tree`, `graph`,
//!   `review`, `support_export`) ships a row bound to one closed
//!   [`SurfaceKind`]; the optional refactor / AI-context / export /
//!   deep-link surfaces ride on top without changing the required set.
//! - **Outside-vs-omitted distinction.** Every result-bearing surface
//!   (`search`, `tree`, `graph`, `review`) distinguishes
//!   `outside_current_slice`, `omitted_path`, and `policy_hidden`
//!   states from `no_result`, so a hidden member is never mislabeled
//!   as missing content.
//! - **Hidden-result disclosure.** Result-bearing and scope-labeling
//!   surfaces disclose hidden-result counts; export-propagation
//!   surfaces propagate the count into the export envelope.
//! - **Slice-ref propagation truth.** Every surface carries the slice
//!   / profile ref into deep-link flows; export-propagation surfaces
//!   additionally carry it into export envelopes so reopened or
//!   exported views land on the exact same scope identity.
//! - **Widen-preview truth.** Every widen-scope preview previews
//!   hidden-result counts, omitted-root classes, fetch / deepen
//!   implications, and the consequences for blame, history, and
//!   search completeness before any apply commits.
//! - **Widen-preservation truth.** Every widen preview preserves
//!   root identity, query / session continuity, and restore
//!   provenance — never silently mints a new workspace identity.
//! - **Policy-limited disclosure truth.** Every `policy_limited_view`
//!   declares one closed [`NarrowingCause`] and refuses to expose the
//!   hidden member list when the cause is admin-policy or
//!   license / export control.
//! - **Readiness truth.** Every scope row declares one closed
//!   [`ReadinessState`]; a `ready` scope must disclose whether its
//!   hidden-result count is known so consumers cannot conflate
//!   "fully indexed" with "no hidden members."
//! - **Support-export honesty.** Each row's support-export projection
//!   preserves the scope class, included roots, hidden-result count,
//!   narrowing cause, and readiness state while excluding raw
//!   secrets, approval tickets, delegated credentials, live authority
//!   handles, and the hidden member list when an admin policy or
//!   license control forbids it.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks
//!   (`inspect_scope`, `compare_before_widen`, `preview_widen`,
//!   `export_scope`, `rollback_widen`, `repair_scope`) is reachable
//!   before any destructive widen / narrow commits.
//! - **Producer attribution.** Each record carries the producer ref,
//!   the schema version, the capture timestamp, and an integrity
//!   hash derived from the input identities so replay and support
//!   pipelines can pin the source before applying.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to
//!   the source workspace, corpus, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`WorksetScopeUxLineageRecord`].
pub const WORKSET_SCOPE_UX_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the workset / scope UX lineage record.
pub const WORKSET_SCOPE_UX_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/workset_scope_ux_lineage.schema.json";

/// Stable record-kind tag for the workset / scope UX lineage record.
pub const WORKSET_SCOPE_UX_LINEAGE_RECORD_KIND: &str = "workset_scope_ux_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the scope classes governed by this lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// A named, durable workset.
    SelectedWorkset,
    /// A sparse slice (sub-tree, glob, or sparse profile).
    SparseSlice,
    /// A policy-limited view overlaid on a base workset/slice.
    PolicyLimitedView,
    /// The full workspace (no narrowing).
    FullWorkspace,
    /// The single active repo root.
    CurrentRepo,
}

impl ScopeKind {
    /// Returns the stable snake_case token for this scope class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedWorkset => "selected_workset",
            Self::SparseSlice => "sparse_slice",
            Self::PolicyLimitedView => "policy_limited_view",
            Self::FullWorkspace => "full_workspace",
            Self::CurrentRepo => "current_repo",
        }
    }

    /// True when this scope class is part of the required set every
    /// Stable corpus must seed.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::SelectedWorkset
                | Self::SparseSlice
                | Self::PolicyLimitedView
                | Self::FullWorkspace
        )
    }

    /// True when this scope class narrows the workspace.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::SelectedWorkset
                | Self::SparseSlice
                | Self::PolicyLimitedView
                | Self::CurrentRepo
        )
    }
}

/// Closed list of scope classes every lineage record must seed.
pub const REQUIRED_SCOPE_CLASSES: [ScopeKind; 4] = [
    ScopeKind::SelectedWorkset,
    ScopeKind::SparseSlice,
    ScopeKind::PolicyLimitedView,
    ScopeKind::FullWorkspace,
];

/// Closed vocabulary for the UX surface kinds that label scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// Named-workset switcher / picker.
    WorksetSwitcher,
    /// Active-scope chip in the status bar / scope banner.
    ScopeChip,
    /// Search results surface (cross-repo or in-repo).
    Search,
    /// Explorer / file tree surface.
    Tree,
    /// Dependency / call / reference graph surface.
    Graph,
    /// Review / diff / refactor preview surface.
    Review,
    /// Support / diagnostic export header.
    SupportExport,
    /// Refactor scope footer (optional).
    RefactorScopeFooter,
    /// AI-context inspector (optional).
    AiContextInspector,
    /// Export scope footer (optional).
    ExportScopeFooter,
    /// Deep-link dispatcher (optional).
    DeepLinkDispatcher,
}

impl SurfaceKind {
    /// Returns the stable snake_case token for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorksetSwitcher => "workset_switcher",
            Self::ScopeChip => "scope_chip",
            Self::Search => "search",
            Self::Tree => "tree",
            Self::Graph => "graph",
            Self::Review => "review",
            Self::SupportExport => "support_export",
            Self::RefactorScopeFooter => "refactor_scope_footer",
            Self::AiContextInspector => "ai_context_inspector",
            Self::ExportScopeFooter => "export_scope_footer",
            Self::DeepLinkDispatcher => "deep_link_dispatcher",
        }
    }

    /// True when the surface is part of the required set every
    /// Stable corpus must seed.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::WorksetSwitcher
                | Self::ScopeChip
                | Self::Search
                | Self::Tree
                | Self::Graph
                | Self::Review
                | Self::SupportExport
        )
    }

    /// True when the surface yields result rows and therefore must
    /// distinguish `outside_current_slice` / `omitted_path` /
    /// `policy_hidden` from `no_result`.
    pub const fn is_result_bearing(self) -> bool {
        matches!(self, Self::Search | Self::Tree | Self::Graph | Self::Review)
    }

    /// True when the surface propagates the slice / profile ref into
    /// export envelopes.
    pub const fn is_export_propagating(self) -> bool {
        matches!(
            self,
            Self::SupportExport | Self::ExportScopeFooter | Self::DeepLinkDispatcher
        )
    }

    /// True when the surface must disclose hidden-result counts.
    pub const fn discloses_hidden_count(self) -> bool {
        // Every surface except the bare deep-link dispatcher discloses
        // a hidden-result count; deep links carry the count via the
        // slice ref so they do not duplicate the surface chrome.
        !matches!(self, Self::DeepLinkDispatcher)
    }
}

/// Closed list of UX surfaces every lineage record must seed.
pub const REQUIRED_SURFACE_KINDS: [SurfaceKind; 7] = [
    SurfaceKind::WorksetSwitcher,
    SurfaceKind::ScopeChip,
    SurfaceKind::Search,
    SurfaceKind::Tree,
    SurfaceKind::Graph,
    SurfaceKind::Review,
    SurfaceKind::SupportExport,
];

/// Closed vocabulary for the cause of a policy-limited narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingCause {
    /// Admin policy narrows the view.
    AdminPolicy,
    /// Trust policy narrows the view.
    TrustPolicy,
    /// License or export-control narrows the view.
    LicenseOrExportControl,
    /// A remote source is currently unavailable.
    RemoteUnavailable,
    /// The local index has not been built yet.
    IndexNotBuilt,
    /// The user muted certain members.
    UserMuted,
}

impl NarrowingCause {
    /// Returns the stable snake_case token for this narrowing cause.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminPolicy => "admin_policy",
            Self::TrustPolicy => "trust_policy",
            Self::LicenseOrExportControl => "license_or_export_control",
            Self::RemoteUnavailable => "remote_unavailable",
            Self::IndexNotBuilt => "index_not_built",
            Self::UserMuted => "user_muted",
        }
    }

    /// True when this narrowing cause forbids exposing the hidden
    /// member list anywhere outside a policy-admin surface.
    pub const fn forbids_hidden_member_list(self) -> bool {
        matches!(self, Self::AdminPolicy | Self::LicenseOrExportControl)
    }
}

/// Closed readiness state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    Cold,
    Warming,
    Warm,
    Partial,
    Ready,
}

impl ReadinessState {
    /// Returns the stable snake_case token for this readiness state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cold => "cold",
            Self::Warming => "warming",
            Self::Warm => "warm",
            Self::Partial => "partial",
            Self::Ready => "ready",
        }
    }

    /// True when the scope must disclose whether the hidden-result
    /// count is known.
    pub const fn requires_known_hidden_count(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Closed vocabulary for widen actions a scope row may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidenActionClass {
    WidenToFullWorkspace,
    WidenWithReview,
    NarrowToCurrentRepo,
    KeepCurrentScope,
    RevealHiddenResultsAdminOnly,
}

impl WidenActionClass {
    /// Returns the stable snake_case token for this widen action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenToFullWorkspace => "widen_to_full_workspace",
            Self::WidenWithReview => "widen_with_review",
            Self::NarrowToCurrentRepo => "narrow_to_current_repo",
            Self::KeepCurrentScope => "keep_current_scope",
            Self::RevealHiddenResultsAdminOnly => "reveal_hidden_results_admin_only",
        }
    }
}

/// Closed widen-preservation posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidenPreservationPosture {
    /// The widen action preserves root identity, query / session
    /// continuity, and restore provenance.
    PreservesIdentityAndContinuity,
    /// The widen action preserves identity only after an explicit
    /// review and ships a disclosure carrying the preserved
    /// breadcrumbs.
    PreservedAfterReviewWithDisclosure,
    /// The widen action creates a new workspace truth (forbidden on
    /// Stable rows).
    CreatesNewWorkspaceTruth,
}

impl WidenPreservationPosture {
    /// Returns the stable snake_case token for this preservation
    /// posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservesIdentityAndContinuity => "preserves_identity_and_continuity",
            Self::PreservedAfterReviewWithDisclosure => "preserved_after_review_with_disclosure",
            Self::CreatesNewWorkspaceTruth => "creates_new_workspace_truth",
        }
    }

    /// True when this preservation posture is safe to ship on a
    /// Stable row.
    pub const fn safe_for_stable(self) -> bool {
        matches!(
            self,
            Self::PreservesIdentityAndContinuity | Self::PreservedAfterReviewWithDisclosure
        )
    }
}

/// Closed support-export posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportPosture {
    /// Row ships a metadata-safe projection in the support packet.
    MetadataSafeExport,
    /// Row withholds its state until manual review.
    HeldRecord,
}

impl SupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Closed vocabulary for pre-action inspection / repair hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetScopeUxInspectionHookClass {
    /// Open the scope inspector with the current scope, included
    /// roots, narrowing cause (if any), and readiness state.
    InspectScope,
    /// Compare the base and candidate scope before any widen / narrow
    /// commits.
    CompareBeforeWiden,
    /// Preview hidden-result counts, omitted-root classes, fetch /
    /// deepen implications, and blame / history / search completeness
    /// consequences before any apply.
    PreviewWiden,
    /// Export the scope artifact (support-safe) so the exact slice can
    /// be replayed elsewhere.
    ExportScope,
    /// Roll a widen action back to the previous scope identity.
    RollbackWiden,
    /// Open the typed repair sheet (e.g. rebuild missing index).
    RepairScope,
}

impl WorksetScopeUxInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectScope => "inspect_scope",
            Self::CompareBeforeWiden => "compare_before_widen",
            Self::PreviewWiden => "preview_widen",
            Self::ExportScope => "export_scope",
            Self::RollbackWiden => "rollback_widen",
            Self::RepairScope => "repair_scope",
        }
    }
}

/// One pre-action inspection / repair hook offered before a
/// destructive widen / narrow commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxInspectionHook {
    /// Hook class.
    pub hook_class: WorksetScopeUxInspectionHookClass,
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
pub fn default_workset_scope_ux_inspection_hooks() -> Vec<WorksetScopeUxInspectionHook> {
    vec![
        WorksetScopeUxInspectionHook {
            hook_class: WorksetScopeUxInspectionHookClass::InspectScope,
            action_id: "workset_scope_ux.inspect_scope".to_owned(),
            label: "Inspect scope".to_owned(),
            available: true,
            disclosure:
                "Opens the scope inspector with the current scope class, included roots, policy narrowing cause (if any), and readiness state before any widen / narrow commits."
                    .to_owned(),
        },
        WorksetScopeUxInspectionHook {
            hook_class: WorksetScopeUxInspectionHookClass::CompareBeforeWiden,
            action_id: "workset_scope_ux.compare_before_widen".to_owned(),
            label: "Compare scope before widen".to_owned(),
            available: true,
            disclosure:
                "Renders the typed widen diff between the active scope and the candidate scope so the user can review the change before any apply."
                    .to_owned(),
        },
        WorksetScopeUxInspectionHook {
            hook_class: WorksetScopeUxInspectionHookClass::PreviewWiden,
            action_id: "workset_scope_ux.preview_widen".to_owned(),
            label: "Preview widen implications".to_owned(),
            available: true,
            disclosure:
                "Previews hidden-result counts, omitted-root classes, fetch / deepen implications, and the consequences for blame / history / search completeness before any apply commits."
                    .to_owned(),
        },
        WorksetScopeUxInspectionHook {
            hook_class: WorksetScopeUxInspectionHookClass::ExportScope,
            action_id: "workset_scope_ux.export_scope".to_owned(),
            label: "Export scope artifact".to_owned(),
            available: true,
            disclosure:
                "Exports the current scope artifact (support-safe) so the exact slice identity, included roots, narrowing cause, and readiness state can be replayed elsewhere."
                    .to_owned(),
        },
        WorksetScopeUxInspectionHook {
            hook_class: WorksetScopeUxInspectionHookClass::RollbackWiden,
            action_id: "workset_scope_ux.rollback_widen".to_owned(),
            label: "Roll back widen".to_owned(),
            available: true,
            disclosure:
                "Reverts the most recent widen / narrow back to the previous scope identity, preserving root identity, query / session continuity, and restore provenance."
                    .to_owned(),
        },
        WorksetScopeUxInspectionHook {
            hook_class: WorksetScopeUxInspectionHookClass::RepairScope,
            action_id: "workset_scope_ux.repair_scope".to_owned(),
            label: "Open typed repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the typed repair sheet for the current scope (e.g. rebuild missing index, redeem remote, request policy override) rather than firing a repair as a shortcut."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a workset /
/// scope UX row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxSupportExportInputs {
    pub posture: SupportExportPosture,
    pub includes_scope_class: bool,
    pub includes_included_roots: bool,
    pub includes_hidden_result_count: bool,
    pub includes_narrowing_cause: bool,
    pub includes_readiness_state: bool,
    pub includes_slice_ref: bool,
    pub raw_secrets_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
    /// Whether the row excludes the hidden member list when an admin
    /// policy or license / export control forbids exposing it.
    pub admin_hidden_list_excluded: bool,
}

impl WorksetScopeUxSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: SupportExportPosture) -> Self {
        Self {
            posture,
            includes_scope_class: true,
            includes_included_roots: true,
            includes_hidden_result_count: true,
            includes_narrowing_cause: true,
            includes_readiness_state: true,
            includes_slice_ref: true,
            raw_secrets_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
            admin_hidden_list_excluded: true,
        }
    }
}

/// One observation of a governed scope at a captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeObservation {
    /// Stable scope identity every consumer preserves.
    pub scope_id: String,
    /// Stable workset id (may equal scope_id for an anonymous slice).
    pub workset_ref: String,
    /// Human-readable workset / slice name.
    pub workset_name: String,
    /// Closed scope class.
    pub scope_class: ScopeKind,
    /// Stable root refs included in this scope.
    pub root_refs: Vec<String>,
    /// Excluded root classes disclosed to the user (e.g.
    /// `vendor_directories`, `policy_redacted`).
    pub excluded_root_classes: Vec<String>,
    /// Optional ref to the policy-limitation artifact.
    pub policy_limitation_ref: Option<String>,
    /// Optional narrowing cause (only on `policy_limited_view`).
    pub narrowing_cause: Option<NarrowingCause>,
    /// Whether the hidden member list is visible on this scope.
    pub hidden_member_list_visible: bool,
    /// Declared readiness state.
    pub readiness_state: ReadinessState,
    /// Whether the hidden-result count is known.
    pub hidden_result_count_known: bool,
    /// The hidden-result count itself, when known.
    pub hidden_result_count: Option<u64>,
    /// Widen / narrow actions offered on this scope.
    pub widen_actions_offered: Vec<WidenActionClass>,
    /// Support-export projection for the scope row.
    pub support_export: WorksetScopeUxSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One observation of a UX surface labeling a scope at a captured
/// moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceObservation {
    /// Stable surface id (e.g. `workspace.scope_chip`).
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed surface kind.
    pub surface_kind: SurfaceKind,
    /// The stable scope id the surface labels.
    pub scope_id: String,
    /// True when the surface distinguishes `outside_current_slice`
    /// from `no_result`.
    pub shows_outside_current_slice: bool,
    /// True when the surface distinguishes `omitted_path` from
    /// `no_result`.
    pub shows_omitted_path: bool,
    /// True when the surface distinguishes `policy_hidden` from
    /// `no_result`.
    pub shows_policy_hidden: bool,
    /// True when the surface discloses the hidden-result count.
    pub discloses_hidden_result_count: bool,
    /// True when the surface carries the slice / profile ref into
    /// deep-link flows.
    pub carries_slice_ref_into_deep_links: bool,
    /// True when the surface carries the slice / profile ref into
    /// export envelopes (required for export-propagating surfaces).
    pub carries_slice_ref_into_export: bool,
    /// Support-export projection for the surface row.
    pub support_export: WorksetScopeUxSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One observation of a widen-scope preview path at a captured
/// moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidenPreviewObservation {
    /// Stable preview id.
    pub preview_id: String,
    /// Stable scope id of the active scope before widening.
    pub base_scope_id: String,
    /// Stable scope id of the candidate scope.
    pub candidate_scope_id: String,
    /// True when the preview discloses hidden-result counts.
    pub previews_hidden_result_count: bool,
    /// True when the preview discloses omitted-root classes.
    pub previews_omitted_root_classes: bool,
    /// True when the preview discloses fetch / deepen implications.
    pub previews_fetch_deepen_implications: bool,
    /// True when the preview discloses the consequences for blame /
    /// history / search completeness.
    pub previews_blame_history_search_consequences: bool,
    /// True when the widen preserves root identity across the
    /// transition.
    pub preserves_root_identity: bool,
    /// True when the widen preserves query / session continuity.
    pub preserves_query_session_continuity: bool,
    /// True when the widen preserves restore provenance.
    pub preserves_restore_provenance: bool,
    /// Declared preservation posture.
    pub preservation_posture: WidenPreservationPosture,
    /// Stable id of the apply action that commits the widen.
    pub apply_action_id: String,
    /// Stable id of the disclosure paired with the apply action.
    pub apply_disclosure_id: String,
    /// Support-export projection for the preview row.
    pub support_export: WorksetScopeUxSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured scope observations.
    pub scopes: Vec<ScopeObservation>,
    /// Captured surface observations.
    pub surfaces: Vec<SurfaceObservation>,
    /// Captured widen-preview observations.
    pub widen_previews: Vec<WidenPreviewObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a workset / scope UX lineage record narrows below
/// Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetScopeUxLineageNarrowReason {
    /// The captured input had no scopes, no surfaces, or no widen
    /// previews.
    CorpusEmpty,
    /// A required scope class is missing from the corpus.
    RequiredScopeClassMissing,
    /// A required UX surface kind is missing from the corpus.
    RequiredSurfaceKindMissing,
    /// A surface labels a scope id that is not present in the corpus.
    SurfaceReferencesUnknownScope,
    /// A widen preview references a base or candidate scope that is
    /// not present in the corpus.
    WidenPreviewReferencesUnknownScope,
    /// A result-bearing surface does not distinguish
    /// `outside_current_slice` from `no_result`.
    OutsideMarkerMissing,
    /// A result-bearing surface does not distinguish `omitted_path`
    /// from `no_result`.
    OmittedMarkerMissing,
    /// A result-bearing surface does not distinguish `policy_hidden`
    /// from `no_result`.
    PolicyHiddenMarkerMissing,
    /// A surface that must disclose hidden-result counts does not.
    HiddenResultCountNotDisclosed,
    /// A surface does not carry the slice / profile ref into
    /// deep-link flows.
    SliceRefNotPropagatedIntoDeepLinks,
    /// An export-propagating surface does not carry the slice ref
    /// into export envelopes.
    SliceRefNotPropagatedIntoExport,
    /// A widen preview is missing one of the required preview
    /// disclosures (hidden count, omitted classes, fetch / deepen,
    /// blame / history / search consequences).
    WidenPreviewFieldMissing,
    /// A widen preview loses root identity.
    WidenLosesRootIdentity,
    /// A widen preview loses query / session continuity.
    WidenLosesQuerySessionContinuity,
    /// A widen preview loses restore provenance.
    WidenLosesRestoreProvenance,
    /// A `policy_limited_view` scope is missing its narrowing cause.
    PolicyLimitedNarrowingCauseMissing,
    /// A `policy_limited_view` scope with an admin / license cause
    /// exposes the hidden member list.
    PolicyAdminHiddenListExposed,
    /// A `ready` scope does not disclose whether the hidden-result
    /// count is known.
    ReadinessHiddenCountUnknown,
    /// A widen preview is missing its apply action id or disclosure
    /// id.
    ApplyActionMetadataMissing,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, approval tickets, delegated credentials, live
    /// authority handles, or an admin-policy hidden member list
    /// slipped into a support-export projection.
    SupportExportRedactionUnsafe,
    /// Producer attribution is incomplete (producer ref or
    /// captured-at empty).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl WorksetScopeUxLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredScopeClassMissing => "required_scope_class_missing",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::SurfaceReferencesUnknownScope => "surface_references_unknown_scope",
            Self::WidenPreviewReferencesUnknownScope => "widen_preview_references_unknown_scope",
            Self::OutsideMarkerMissing => "outside_marker_missing",
            Self::OmittedMarkerMissing => "omitted_marker_missing",
            Self::PolicyHiddenMarkerMissing => "policy_hidden_marker_missing",
            Self::HiddenResultCountNotDisclosed => "hidden_result_count_not_disclosed",
            Self::SliceRefNotPropagatedIntoDeepLinks => "slice_ref_not_propagated_into_deep_links",
            Self::SliceRefNotPropagatedIntoExport => "slice_ref_not_propagated_into_export",
            Self::WidenPreviewFieldMissing => "widen_preview_field_missing",
            Self::WidenLosesRootIdentity => "widen_loses_root_identity",
            Self::WidenLosesQuerySessionContinuity => "widen_loses_query_session_continuity",
            Self::WidenLosesRestoreProvenance => "widen_loses_restore_provenance",
            Self::PolicyLimitedNarrowingCauseMissing => "policy_limited_narrowing_cause_missing",
            Self::PolicyAdminHiddenListExposed => "policy_admin_hidden_list_exposed",
            Self::ReadinessHiddenCountUnknown => "readiness_hidden_count_unknown",
            Self::ApplyActionMetadataMissing => "apply_action_metadata_missing",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a workset / scope UX lineage
/// record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<WorksetScopeUxLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One scope row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRow {
    pub scope_id: String,
    pub workset_ref: String,
    pub workset_name: String,
    pub scope_class: ScopeKind,
    pub root_refs: Vec<String>,
    pub excluded_root_classes: Vec<String>,
    pub policy_limitation_ref: Option<String>,
    pub narrowing_cause: Option<NarrowingCause>,
    pub hidden_member_list_visible: bool,
    pub readiness_state: ReadinessState,
    pub hidden_result_count_known: bool,
    pub hidden_result_count: Option<u64>,
    pub widen_actions_offered: Vec<WidenActionClass>,
    pub support_export_posture: SupportExportPosture,
    pub is_required: bool,
}

/// One surface row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRow {
    pub surface_id: String,
    pub title: String,
    pub surface_kind: SurfaceKind,
    pub scope_id: String,
    pub shows_outside_current_slice: bool,
    pub shows_omitted_path: bool,
    pub shows_policy_hidden: bool,
    pub discloses_hidden_result_count: bool,
    pub carries_slice_ref_into_deep_links: bool,
    pub carries_slice_ref_into_export: bool,
    pub is_result_bearing: bool,
    pub is_export_propagating: bool,
    pub support_export_posture: SupportExportPosture,
    pub is_required: bool,
}

/// One widen-preview row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidenPreviewRow {
    pub preview_id: String,
    pub base_scope_id: String,
    pub candidate_scope_id: String,
    pub previews_hidden_result_count: bool,
    pub previews_omitted_root_classes: bool,
    pub previews_fetch_deepen_implications: bool,
    pub previews_blame_history_search_consequences: bool,
    pub preserves_root_identity: bool,
    pub preserves_query_session_continuity: bool,
    pub preserves_restore_provenance: bool,
    pub preservation_posture: WidenPreservationPosture,
    pub apply_action_id: String,
    pub apply_disclosure_id: String,
    pub support_export_posture: SupportExportPosture,
}

/// Scope coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCoverageSummary {
    pub scope_rows: Vec<ScopeRow>,
    pub all_required_scope_classes_present: bool,
}

/// Surface coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoverageSummary {
    pub surface_rows: Vec<SurfaceRow>,
    pub all_required_surface_kinds_present: bool,
}

/// Outside-vs-omitted distinction posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutsideMarkerHonestySummary {
    pub all_result_bearing_surfaces_show_outside_current_slice: bool,
    pub all_result_bearing_surfaces_show_omitted_path: bool,
    pub all_result_bearing_surfaces_show_policy_hidden: bool,
}

/// Hidden-result disclosure posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenResultDisclosureSummary {
    pub all_surfaces_disclose_hidden_count: bool,
}

/// Slice-ref propagation posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceRefPropagationSummary {
    pub all_surfaces_carry_slice_ref_into_deep_links: bool,
    pub all_export_propagating_surfaces_carry_slice_ref_into_export: bool,
}

/// Widen-preview truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidenPreviewTruthSummary {
    pub widen_preview_rows: Vec<WidenPreviewRow>,
    pub all_previews_have_required_fields: bool,
    pub all_previews_have_apply_action_metadata: bool,
}

/// Widen-preservation truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidenPreservationTruthSummary {
    pub all_widens_preserve_root_identity: bool,
    pub all_widens_preserve_query_session_continuity: bool,
    pub all_widens_preserve_restore_provenance: bool,
    pub all_preservation_postures_safe: bool,
}

/// Policy-limited disclosure posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLimitedDisclosureSummary {
    pub policy_limited_scope_count: usize,
    pub all_policy_limited_have_narrowing_cause: bool,
    pub no_admin_or_license_policy_exposes_hidden_list: bool,
}

/// Readiness truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessTruthSummary {
    pub all_ready_scopes_disclose_hidden_count_known: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxSupportExportHonestySummary {
    pub all_rows_preserve_fields: bool,
    pub all_rows_redact_raw_secrets: bool,
    pub all_rows_exclude_approval_tickets: bool,
    pub all_rows_exclude_delegated_credentials: bool,
    pub all_rows_exclude_live_authority_handles: bool,
    pub all_rows_exclude_admin_hidden_list: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxProducerAttributionSummary {
    pub producer_ref: String,
    pub schema_version: u32,
    pub integrity_hash: String,
    pub captured_at: String,
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe workset / scope UX lineage record per
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeUxLineageRecord {
    pub record_kind: String,
    pub workset_scope_ux_lineage_schema_version: u32,
    pub schema_ref: String,
    pub posture_id: String,
    pub workspace_ref: String,
    pub corpus_ref: String,
    pub producer_attribution: WorksetScopeUxProducerAttributionSummary,
    pub scope_coverage: ScopeCoverageSummary,
    pub surface_coverage: SurfaceCoverageSummary,
    pub outside_marker_honesty: OutsideMarkerHonestySummary,
    pub hidden_result_disclosure: HiddenResultDisclosureSummary,
    pub slice_ref_propagation: SliceRefPropagationSummary,
    pub widen_preview_truth: WidenPreviewTruthSummary,
    pub widen_preservation_truth: WidenPreservationTruthSummary,
    pub policy_limited_disclosure: PolicyLimitedDisclosureSummary,
    pub readiness_truth: ReadinessTruthSummary,
    pub support_export_honesty: WorksetScopeUxSupportExportHonestySummary,
    pub inspection_hooks: Vec<WorksetScopeUxInspectionHook>,
    pub stable_qualification: WorksetScopeUxLineageQualification,
    pub raw_payload_excluded: bool,
    pub summary: String,
}

impl WorksetScopeUxLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == WORKSET_SCOPE_UX_LINEAGE_SCHEMA_REF
            && self.record_kind == WORKSET_SCOPE_UX_LINEAGE_RECORD_KIND
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
        class: WorksetScopeUxInspectionHookClass,
    ) -> Option<&WorksetScopeUxInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed workset / scope UX lineage record from a live
/// [`WorksetScopeUxInputs`] envelope using the default
/// inspection-hook set.
pub fn project_workset_scope_ux_lineage(
    posture_id: impl Into<String>,
    inputs: &WorksetScopeUxInputs,
) -> WorksetScopeUxLineageRecord {
    project_workset_scope_ux_lineage_with_hooks(
        posture_id,
        inputs,
        default_workset_scope_ux_inspection_hooks(),
    )
}

/// Like [`project_workset_scope_ux_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_workset_scope_ux_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &WorksetScopeUxInputs,
    inspection_hooks: Vec<WorksetScopeUxInspectionHook>,
) -> WorksetScopeUxLineageRecord {
    let posture_id: String = posture_id.into();

    let scope_coverage = project_scope_coverage(inputs);
    let surface_coverage = project_surface_coverage(inputs);
    let outside_marker_honesty = project_outside_marker_honesty(&surface_coverage);
    let hidden_result_disclosure = project_hidden_result_disclosure(&surface_coverage);
    let slice_ref_propagation = project_slice_ref_propagation(&surface_coverage);
    let widen_preview_truth = project_widen_preview_truth(inputs);
    let widen_preservation_truth = project_widen_preservation_truth(&widen_preview_truth);
    let policy_limited_disclosure = project_policy_limited_disclosure(&scope_coverage);
    let readiness_truth = project_readiness_truth(&scope_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let known_scope_ids: BTreeSet<&str> = scope_coverage
        .scope_rows
        .iter()
        .map(|row| row.scope_id.as_str())
        .collect();

    let mut narrow_reasons = Vec::new();

    if inputs.scopes.is_empty() || inputs.surfaces.is_empty() || inputs.widen_previews.is_empty() {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::CorpusEmpty);
    }
    if !scope_coverage.all_required_scope_classes_present {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::RequiredScopeClassMissing);
    }
    if !surface_coverage.all_required_surface_kinds_present {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::RequiredSurfaceKindMissing);
    }
    if surface_coverage
        .surface_rows
        .iter()
        .any(|row| !known_scope_ids.contains(row.scope_id.as_str()))
    {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::SurfaceReferencesUnknownScope);
    }
    if widen_preview_truth.widen_preview_rows.iter().any(|row| {
        !known_scope_ids.contains(row.base_scope_id.as_str())
            || !known_scope_ids.contains(row.candidate_scope_id.as_str())
    }) {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::WidenPreviewReferencesUnknownScope);
    }
    if !outside_marker_honesty.all_result_bearing_surfaces_show_outside_current_slice {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::OutsideMarkerMissing);
    }
    if !outside_marker_honesty.all_result_bearing_surfaces_show_omitted_path {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::OmittedMarkerMissing);
    }
    if !outside_marker_honesty.all_result_bearing_surfaces_show_policy_hidden {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::PolicyHiddenMarkerMissing);
    }
    if !hidden_result_disclosure.all_surfaces_disclose_hidden_count {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::HiddenResultCountNotDisclosed);
    }
    if !slice_ref_propagation.all_surfaces_carry_slice_ref_into_deep_links {
        narrow_reasons
            .push(WorksetScopeUxLineageNarrowReason::SliceRefNotPropagatedIntoDeepLinks);
    }
    if !slice_ref_propagation.all_export_propagating_surfaces_carry_slice_ref_into_export {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::SliceRefNotPropagatedIntoExport);
    }
    if !widen_preview_truth.all_previews_have_required_fields {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::WidenPreviewFieldMissing);
    }
    if !widen_preview_truth.all_previews_have_apply_action_metadata {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::ApplyActionMetadataMissing);
    }
    if !widen_preservation_truth.all_widens_preserve_root_identity {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::WidenLosesRootIdentity);
    }
    if !widen_preservation_truth.all_widens_preserve_query_session_continuity {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::WidenLosesQuerySessionContinuity);
    }
    if !widen_preservation_truth.all_widens_preserve_restore_provenance {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::WidenLosesRestoreProvenance);
    }
    if !widen_preservation_truth.all_preservation_postures_safe {
        // `creates_new_workspace_truth` shows up via either the
        // preservation flags or the posture; we already enumerate the
        // specific lost-identity reason above. We add a posture-level
        // reason only if no other widen-preservation reason was
        // recorded (so the narrow list does not duplicate the
        // diagnosis).
        if !narrow_reasons
            .iter()
            .any(|reason| {
                matches!(
                    reason,
                    WorksetScopeUxLineageNarrowReason::WidenLosesRootIdentity
                        | WorksetScopeUxLineageNarrowReason::WidenLosesQuerySessionContinuity
                        | WorksetScopeUxLineageNarrowReason::WidenLosesRestoreProvenance
                )
            })
        {
            narrow_reasons.push(WorksetScopeUxLineageNarrowReason::WidenLosesRootIdentity);
        }
    }
    if !policy_limited_disclosure.all_policy_limited_have_narrowing_cause {
        narrow_reasons
            .push(WorksetScopeUxLineageNarrowReason::PolicyLimitedNarrowingCauseMissing);
    }
    if !policy_limited_disclosure.no_admin_or_license_policy_exposes_hidden_list {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::PolicyAdminHiddenListExposed);
    }
    if !readiness_truth.all_ready_scopes_disclose_hidden_count_known {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::ReadinessHiddenCountUnknown);
    }

    let required_hooks = [
        WorksetScopeUxInspectionHookClass::InspectScope,
        WorksetScopeUxInspectionHookClass::CompareBeforeWiden,
        WorksetScopeUxInspectionHookClass::PreviewWiden,
        WorksetScopeUxInspectionHookClass::ExportScope,
        WorksetScopeUxInspectionHookClass::RollbackWiden,
        WorksetScopeUxInspectionHookClass::RepairScope,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::InspectionHookUnavailable);
    }

    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = WorksetScopeUxLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &scope_coverage,
        &surface_coverage,
        &widen_preview_truth,
        &policy_limited_disclosure,
        &stable_qualification,
    );

    WorksetScopeUxLineageRecord {
        record_kind: WORKSET_SCOPE_UX_LINEAGE_RECORD_KIND.to_owned(),
        workset_scope_ux_lineage_schema_version: WORKSET_SCOPE_UX_LINEAGE_SCHEMA_VERSION,
        schema_ref: WORKSET_SCOPE_UX_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        scope_coverage,
        surface_coverage,
        outside_marker_honesty,
        hidden_result_disclosure,
        slice_ref_propagation,
        widen_preview_truth,
        widen_preservation_truth,
        policy_limited_disclosure,
        readiness_truth,
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

fn project_scope_coverage(inputs: &WorksetScopeUxInputs) -> ScopeCoverageSummary {
    let scope_rows: Vec<ScopeRow> = inputs.scopes.iter().map(project_scope_row).collect();
    let observed: BTreeSet<_> = scope_rows.iter().map(|row| row.scope_class).collect();
    let all_required_scope_classes_present = REQUIRED_SCOPE_CLASSES
        .iter()
        .all(|required| observed.contains(required));
    ScopeCoverageSummary {
        scope_rows,
        all_required_scope_classes_present,
    }
}

fn project_scope_row(observation: &ScopeObservation) -> ScopeRow {
    ScopeRow {
        scope_id: observation.scope_id.clone(),
        workset_ref: observation.workset_ref.clone(),
        workset_name: observation.workset_name.clone(),
        scope_class: observation.scope_class,
        root_refs: observation.root_refs.clone(),
        excluded_root_classes: observation.excluded_root_classes.clone(),
        policy_limitation_ref: observation.policy_limitation_ref.clone(),
        narrowing_cause: observation.narrowing_cause,
        hidden_member_list_visible: observation.hidden_member_list_visible,
        readiness_state: observation.readiness_state,
        hidden_result_count_known: observation.hidden_result_count_known,
        hidden_result_count: observation.hidden_result_count,
        widen_actions_offered: observation.widen_actions_offered.clone(),
        support_export_posture: observation.support_export.posture,
        is_required: observation.scope_class.is_required(),
    }
}

fn project_surface_coverage(inputs: &WorksetScopeUxInputs) -> SurfaceCoverageSummary {
    let surface_rows: Vec<SurfaceRow> = inputs.surfaces.iter().map(project_surface_row).collect();
    let observed: BTreeSet<_> = surface_rows.iter().map(|row| row.surface_kind).collect();
    let all_required_surface_kinds_present = REQUIRED_SURFACE_KINDS
        .iter()
        .all(|required| observed.contains(required));
    SurfaceCoverageSummary {
        surface_rows,
        all_required_surface_kinds_present,
    }
}

fn project_surface_row(observation: &SurfaceObservation) -> SurfaceRow {
    SurfaceRow {
        surface_id: observation.surface_id.clone(),
        title: observation.title.clone(),
        surface_kind: observation.surface_kind,
        scope_id: observation.scope_id.clone(),
        shows_outside_current_slice: observation.shows_outside_current_slice,
        shows_omitted_path: observation.shows_omitted_path,
        shows_policy_hidden: observation.shows_policy_hidden,
        discloses_hidden_result_count: observation.discloses_hidden_result_count,
        carries_slice_ref_into_deep_links: observation.carries_slice_ref_into_deep_links,
        carries_slice_ref_into_export: observation.carries_slice_ref_into_export,
        is_result_bearing: observation.surface_kind.is_result_bearing(),
        is_export_propagating: observation.surface_kind.is_export_propagating(),
        support_export_posture: observation.support_export.posture,
        is_required: observation.surface_kind.is_required(),
    }
}

fn project_outside_marker_honesty(
    coverage: &SurfaceCoverageSummary,
) -> OutsideMarkerHonestySummary {
    let mut outside = true;
    let mut omitted = true;
    let mut policy_hidden = true;
    for row in &coverage.surface_rows {
        if !row.is_result_bearing {
            continue;
        }
        if !row.shows_outside_current_slice {
            outside = false;
        }
        if !row.shows_omitted_path {
            omitted = false;
        }
        if !row.shows_policy_hidden {
            policy_hidden = false;
        }
    }
    OutsideMarkerHonestySummary {
        all_result_bearing_surfaces_show_outside_current_slice: outside,
        all_result_bearing_surfaces_show_omitted_path: omitted,
        all_result_bearing_surfaces_show_policy_hidden: policy_hidden,
    }
}

fn project_hidden_result_disclosure(
    coverage: &SurfaceCoverageSummary,
) -> HiddenResultDisclosureSummary {
    let all_surfaces_disclose_hidden_count = coverage.surface_rows.iter().all(|row| {
        if row.surface_kind.discloses_hidden_count() {
            row.discloses_hidden_result_count
        } else {
            true
        }
    });
    HiddenResultDisclosureSummary {
        all_surfaces_disclose_hidden_count,
    }
}

fn project_slice_ref_propagation(coverage: &SurfaceCoverageSummary) -> SliceRefPropagationSummary {
    let mut deep_links_ok = true;
    let mut export_ok = true;
    for row in &coverage.surface_rows {
        if !row.carries_slice_ref_into_deep_links {
            deep_links_ok = false;
        }
        if row.is_export_propagating && !row.carries_slice_ref_into_export {
            export_ok = false;
        }
    }
    SliceRefPropagationSummary {
        all_surfaces_carry_slice_ref_into_deep_links: deep_links_ok,
        all_export_propagating_surfaces_carry_slice_ref_into_export: export_ok,
    }
}

fn project_widen_preview_truth(inputs: &WorksetScopeUxInputs) -> WidenPreviewTruthSummary {
    let widen_preview_rows: Vec<WidenPreviewRow> = inputs
        .widen_previews
        .iter()
        .map(project_widen_preview_row)
        .collect();
    let all_previews_have_required_fields = widen_preview_rows.iter().all(|row| {
        row.previews_hidden_result_count
            && row.previews_omitted_root_classes
            && row.previews_fetch_deepen_implications
            && row.previews_blame_history_search_consequences
    });
    let all_previews_have_apply_action_metadata = widen_preview_rows
        .iter()
        .all(|row| !row.apply_action_id.trim().is_empty() && !row.apply_disclosure_id.trim().is_empty());
    WidenPreviewTruthSummary {
        widen_preview_rows,
        all_previews_have_required_fields,
        all_previews_have_apply_action_metadata,
    }
}

fn project_widen_preview_row(observation: &WidenPreviewObservation) -> WidenPreviewRow {
    WidenPreviewRow {
        preview_id: observation.preview_id.clone(),
        base_scope_id: observation.base_scope_id.clone(),
        candidate_scope_id: observation.candidate_scope_id.clone(),
        previews_hidden_result_count: observation.previews_hidden_result_count,
        previews_omitted_root_classes: observation.previews_omitted_root_classes,
        previews_fetch_deepen_implications: observation.previews_fetch_deepen_implications,
        previews_blame_history_search_consequences: observation
            .previews_blame_history_search_consequences,
        preserves_root_identity: observation.preserves_root_identity,
        preserves_query_session_continuity: observation.preserves_query_session_continuity,
        preserves_restore_provenance: observation.preserves_restore_provenance,
        preservation_posture: observation.preservation_posture,
        apply_action_id: observation.apply_action_id.clone(),
        apply_disclosure_id: observation.apply_disclosure_id.clone(),
        support_export_posture: observation.support_export.posture,
    }
}

fn project_widen_preservation_truth(
    summary: &WidenPreviewTruthSummary,
) -> WidenPreservationTruthSummary {
    let mut root_ok = true;
    let mut query_ok = true;
    let mut restore_ok = true;
    let mut posture_ok = true;
    for row in &summary.widen_preview_rows {
        if !row.preserves_root_identity {
            root_ok = false;
        }
        if !row.preserves_query_session_continuity {
            query_ok = false;
        }
        if !row.preserves_restore_provenance {
            restore_ok = false;
        }
        if !row.preservation_posture.safe_for_stable() {
            posture_ok = false;
        }
    }
    WidenPreservationTruthSummary {
        all_widens_preserve_root_identity: root_ok,
        all_widens_preserve_query_session_continuity: query_ok,
        all_widens_preserve_restore_provenance: restore_ok,
        all_preservation_postures_safe: posture_ok,
    }
}

fn project_policy_limited_disclosure(
    coverage: &ScopeCoverageSummary,
) -> PolicyLimitedDisclosureSummary {
    let mut policy_limited_scope_count = 0usize;
    let mut have_cause = true;
    let mut admin_safe = true;
    for row in &coverage.scope_rows {
        if row.scope_class != ScopeKind::PolicyLimitedView {
            continue;
        }
        policy_limited_scope_count += 1;
        let cause = match row.narrowing_cause {
            Some(cause) => cause,
            None => {
                have_cause = false;
                continue;
            }
        };
        if cause.forbids_hidden_member_list() && row.hidden_member_list_visible {
            admin_safe = false;
        }
    }
    PolicyLimitedDisclosureSummary {
        policy_limited_scope_count,
        all_policy_limited_have_narrowing_cause: have_cause,
        no_admin_or_license_policy_exposes_hidden_list: admin_safe,
    }
}

fn project_readiness_truth(coverage: &ScopeCoverageSummary) -> ReadinessTruthSummary {
    let all_ready_scopes_disclose_hidden_count_known = coverage.scope_rows.iter().all(|row| {
        if row.readiness_state.requires_known_hidden_count() {
            row.hidden_result_count_known
        } else {
            true
        }
    });
    ReadinessTruthSummary {
        all_ready_scopes_disclose_hidden_count_known,
    }
}

fn project_support_export_honesty(
    inputs: &WorksetScopeUxInputs,
) -> WorksetScopeUxSupportExportHonestySummary {
    let mut preserve_fields = true;
    let mut redact_secrets = true;
    let mut exclude_approvals = true;
    let mut exclude_credentials = true;
    let mut exclude_authority = true;
    let mut exclude_admin_hidden = true;

    let supports = inputs
        .scopes
        .iter()
        .map(|s| s.support_export)
        .chain(inputs.surfaces.iter().map(|s| s.support_export))
        .chain(inputs.widen_previews.iter().map(|w| w.support_export));

    for support in supports {
        if !(support.includes_scope_class
            && support.includes_included_roots
            && support.includes_hidden_result_count
            && support.includes_narrowing_cause
            && support.includes_readiness_state
            && support.includes_slice_ref)
        {
            preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            redact_secrets = false;
        }
        if !support.approval_tickets_excluded {
            exclude_approvals = false;
        }
        if !support.delegated_credentials_excluded {
            exclude_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            exclude_authority = false;
        }
        if !support.admin_hidden_list_excluded {
            exclude_admin_hidden = false;
        }
    }

    WorksetScopeUxSupportExportHonestySummary {
        all_rows_preserve_fields: preserve_fields,
        all_rows_redact_raw_secrets: redact_secrets,
        all_rows_exclude_approval_tickets: exclude_approvals,
        all_rows_exclude_delegated_credentials: exclude_credentials,
        all_rows_exclude_live_authority_handles: exclude_authority,
        all_rows_exclude_admin_hidden_list: exclude_admin_hidden,
    }
}

fn project_producer_attribution(
    inputs: &WorksetScopeUxInputs,
) -> WorksetScopeUxProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    WorksetScopeUxProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: WORKSET_SCOPE_UX_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_support_export_narrows(
    summary: &WorksetScopeUxSupportExportHonestySummary,
    narrow_reasons: &mut Vec<WorksetScopeUxLineageNarrowReason>,
) {
    if !summary.all_rows_preserve_fields {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !(summary.all_rows_redact_raw_secrets
        && summary.all_rows_exclude_approval_tickets
        && summary.all_rows_exclude_delegated_credentials
        && summary.all_rows_exclude_live_authority_handles
        && summary.all_rows_exclude_admin_hidden_list)
    {
        narrow_reasons.push(WorksetScopeUxLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &WorksetScopeUxInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for scope in &inputs.scopes {
        for byte in scope.scope_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(scope.scope_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(scope.readiness_state.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for surface in &inputs.surfaces {
        for byte in surface.surface_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(surface.surface_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for widen in &inputs.widen_previews {
        for byte in widen.preview_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(widen.preservation_posture.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("wsx:{hash:016x}")
}

fn hook_available(
    hooks: &[WorksetScopeUxInspectionHook],
    class: WorksetScopeUxInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    scope_coverage: &ScopeCoverageSummary,
    surface_coverage: &SurfaceCoverageSummary,
    widen_preview_truth: &WidenPreviewTruthSummary,
    policy_limited_disclosure: &PolicyLimitedDisclosureSummary,
    qualification: &WorksetScopeUxLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Workset/scope UX lineage proven Stable: scopes={scopes} surfaces={surfaces} widens={widens} policy_limited={policy_limited}.",
            scopes = scope_coverage.scope_rows.len(),
            surfaces = surface_coverage.surface_rows.len(),
            widens = widen_preview_truth.widen_preview_rows.len(),
            policy_limited = policy_limited_disclosure.policy_limited_scope_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Workset/scope UX lineage narrowed below Stable (scopes={scopes} surfaces={surfaces} widens={widens}): {reasons}.",
            scopes = scope_coverage.scope_rows.len(),
            surfaces = surface_coverage.surface_rows.len(),
            widens = widen_preview_truth.widen_preview_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a workset / scope UX
/// lineage record. The same projection is consumed by the workspace
/// scope status surface, the headless CLI emitter, Help/About, and
/// support export.
pub fn workset_scope_ux_lineage_lines(record: &WorksetScopeUxLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Workset/scope UX lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
    ));
    lines.push(format!(
        "scope_coverage: scopes={} required_present={}",
        record.scope_coverage.scope_rows.len(),
        record.scope_coverage.all_required_scope_classes_present,
    ));
    lines.push("Scopes:".to_owned());
    for row in &record.scope_coverage.scope_rows {
        let widen: Vec<&str> = row
            .widen_actions_offered
            .iter()
            .map(|a| a.as_str())
            .collect();
        let cause = row
            .narrowing_cause
            .map(|c| c.as_str())
            .unwrap_or("none");
        let hidden_count = row
            .hidden_result_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "unknown".to_owned());
        lines.push(format!(
            "  - {kind} {id} workset={workset} required={required} readiness={readiness} hidden_known={hidden_known} hidden_count={hidden_count} narrowing_cause={cause} hidden_list_visible={hidden_list_visible} widen_actions=[{widen}] support_export={support}",
            kind = row.scope_class.as_str(),
            id = row.scope_id,
            workset = row.workset_ref,
            required = row.is_required,
            readiness = row.readiness_state.as_str(),
            hidden_known = row.hidden_result_count_known,
            hidden_count = hidden_count,
            cause = cause,
            hidden_list_visible = row.hidden_member_list_visible,
            widen = widen.join(","),
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "surface_coverage: surfaces={} required_present={}",
        record.surface_coverage.surface_rows.len(),
        record.surface_coverage.all_required_surface_kinds_present,
    ));
    lines.push("Surfaces:".to_owned());
    for row in &record.surface_coverage.surface_rows {
        lines.push(format!(
            "  - {kind} {id} scope={scope} result_bearing={result_bearing} export_propagating={export_propagating} outside={outside} omitted={omitted} policy_hidden={policy_hidden} hidden_count={hidden_count} deep_link_slice={deep_link_slice} export_slice={export_slice} required={required} support_export={support}",
            kind = row.surface_kind.as_str(),
            id = row.surface_id,
            scope = row.scope_id,
            result_bearing = row.is_result_bearing,
            export_propagating = row.is_export_propagating,
            outside = row.shows_outside_current_slice,
            omitted = row.shows_omitted_path,
            policy_hidden = row.shows_policy_hidden,
            hidden_count = row.discloses_hidden_result_count,
            deep_link_slice = row.carries_slice_ref_into_deep_links,
            export_slice = row.carries_slice_ref_into_export,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Outside-vs-omitted: outside={o} omitted={m} policy_hidden={p}",
        o = record
            .outside_marker_honesty
            .all_result_bearing_surfaces_show_outside_current_slice,
        m = record
            .outside_marker_honesty
            .all_result_bearing_surfaces_show_omitted_path,
        p = record
            .outside_marker_honesty
            .all_result_bearing_surfaces_show_policy_hidden,
    ));
    lines.push(format!(
        "Hidden-result disclosure: all_disclose={}",
        record.hidden_result_disclosure.all_surfaces_disclose_hidden_count,
    ));
    lines.push(format!(
        "Slice-ref propagation: deep_links={d} export={e}",
        d = record
            .slice_ref_propagation
            .all_surfaces_carry_slice_ref_into_deep_links,
        e = record
            .slice_ref_propagation
            .all_export_propagating_surfaces_carry_slice_ref_into_export,
    ));
    lines.push(format!(
        "widen_preview_truth: previews={count} all_fields={fields} all_apply_metadata={apply}",
        count = record.widen_preview_truth.widen_preview_rows.len(),
        fields = record.widen_preview_truth.all_previews_have_required_fields,
        apply = record
            .widen_preview_truth
            .all_previews_have_apply_action_metadata,
    ));
    lines.push("Widen previews:".to_owned());
    for row in &record.widen_preview_truth.widen_preview_rows {
        lines.push(format!(
            "  - {id} base={base} candidate={candidate} hidden_count={hidden_count} omitted_classes={omitted} fetch_deepen={fetch_deepen} blame_history_search={blame_hist} preserves_identity={identity} preserves_query={query} preserves_restore={restore} posture={posture}",
            id = row.preview_id,
            base = row.base_scope_id,
            candidate = row.candidate_scope_id,
            hidden_count = row.previews_hidden_result_count,
            omitted = row.previews_omitted_root_classes,
            fetch_deepen = row.previews_fetch_deepen_implications,
            blame_hist = row.previews_blame_history_search_consequences,
            identity = row.preserves_root_identity,
            query = row.preserves_query_session_continuity,
            restore = row.preserves_restore_provenance,
            posture = row.preservation_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Widen-preservation truth: identity={i} query={q} restore={r} posture={p}",
        i = record
            .widen_preservation_truth
            .all_widens_preserve_root_identity,
        q = record
            .widen_preservation_truth
            .all_widens_preserve_query_session_continuity,
        r = record
            .widen_preservation_truth
            .all_widens_preserve_restore_provenance,
        p = record
            .widen_preservation_truth
            .all_preservation_postures_safe,
    ));
    lines.push(format!(
        "Policy-limited disclosure: policy_limited_count={count} all_have_cause={cause} admin_safe={admin_safe}",
        count = record.policy_limited_disclosure.policy_limited_scope_count,
        cause = record
            .policy_limited_disclosure
            .all_policy_limited_have_narrowing_cause,
        admin_safe = record
            .policy_limited_disclosure
            .no_admin_or_license_policy_exposes_hidden_list,
    ));
    lines.push(format!(
        "Readiness truth: ready_scopes_disclose_hidden_known={}",
        record
            .readiness_truth
            .all_ready_scopes_disclose_hidden_count_known,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} redact_secrets={secrets} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority} exclude_admin_hidden={admin_hidden}",
        fields = record.support_export_honesty.all_rows_preserve_fields,
        secrets = record.support_export_honesty.all_rows_redact_raw_secrets,
        approvals = record
            .support_export_honesty
            .all_rows_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles,
        admin_hidden = record
            .support_export_honesty
            .all_rows_exclude_admin_hidden_list,
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
