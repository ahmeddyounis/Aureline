//! Scope-truth chip projections for open and search foundations.
//!
//! This module is the protected-row consumer for scope-truth chips on the
//! live shell's open and search surfaces (explorer, quick open, search
//! shell, docs/help browser, generated-artifact cues). It owns no scope,
//! readiness, or workset vocabulary of its own; it projects:
//!
//! - the canonical [`aureline_workspace::ScopeClass`] +
//!   [`aureline_workspace::WorksetArtifactRecord::project_chip`] truth into
//!   a serializable shell-facing [`ScopeTruthChipCard`], and
//! - shell-side visible / loaded / all-matching counts into a typed
//!   [`ScopeCountsRecord`] so a viewport row count never reads as
//!   workspace-wide truth when the active scope is narrowed.
//!
//! Surfaces consume the record kinds defined here verbatim. They MUST NOT
//! invent a parallel surface-only chip label, presentation state, or count
//! vocabulary — that would re-fork scope truth across surfaces.
//!
//! ## Honesty contract
//!
//! 1. Every chip card carries the [`ScopeTruthSurfaceClass`] that emitted
//!    it. Surfaces never relabel a chip from another lane (e.g. a quick
//!    open chip never appears with a `search_shell` token).
//! 2. The chip card's `partial_scope` flag is true whenever the active
//!    scope is narrower than the workspace OR the active scope is partial
//!    (warming, cached, manifest_known, hidden_by_policy). The chrome
//!    SHOULD NOT collapse this flag into a generic "loading" badge.
//! 3. Counts disclose visible/loaded/all-matching truth separately. A
//!    surface that only knows `visible_in_view` MUST leave the other two
//!    counts as `None` rather than copying `visible_in_view` into them.
//! 4. The `outside_current_scope_marker_visible` flag is reserved for
//!    cross-repo result rows whose owning root is not in the active
//!    workset's `root_refs`; it is never set on the active scope chip.

pub mod card;
pub mod counts;

pub use card::{
    project_outside_scope_truth_chip_card, project_scope_truth_chip_card,
    project_scope_truth_chip_card_for_artifact, render_scope_truth_chip_lines,
    ScopeTruthChipCard, ScopeTruthSurfaceClass, SCOPE_TRUTH_CHIP_RECORD_KIND,
    SCOPE_TRUTH_CHIP_SCHEMA_VERSION,
};
pub use counts::{ScopeCountsClass, ScopeCountsInputs, ScopeCountsRecord};
