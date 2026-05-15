//! Docs/help browser skeleton with source / version / freshness rows.
//!
//! The docs/help browser is the in-product surface for opening project,
//! mirrored, generated, or vendor docs without pretending an embedded web
//! tab is timeless local truth. It does NOT redefine the embedded-surface
//! owner/origin chrome substrate: it consumes the shared
//! [`EmbeddedBoundaryCardRecord`] contract and projects a focused row card
//! whose vocabulary names the docs-truth axes the user must see — source
//! class, version match, freshness, client-scope (data-boundary + identity
//! mode + trust state), and the host-owned browser handoff.
//!
//! Failure-drill rule: when the upstream record carries `unknown_target_build`
//! or `unverified` truth (or has no `source_truth` block at all), the row
//! card MUST keep the rows explicit instead of blanking them out. Embedded
//! docs panes never get to silently drop their provenance just because the
//! data is degraded.
//!
//! [`EmbeddedBoundaryCardRecord`]: crate::embedded::boundary_card::EmbeddedBoundaryCardRecord

pub mod content;
pub mod state;

pub use content::{docs_browser_row_cards_from_pack, DocsBrowserContentContext};
pub use state::{
    DocsBrowserBrowserHandoffRow, DocsBrowserClientScopeRow, DocsBrowserFreshnessRow,
    DocsBrowserRowCard, DocsBrowserSourceRow, DocsBrowserSurfaceState, DocsBrowserVersionRow,
};
