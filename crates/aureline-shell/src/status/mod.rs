//! Shell status projections for shared runtime truth surfaces.
//!
//! Status modules consume canonical product-state records from owner crates
//! and materialize chrome-facing rows without inventing new vocabulary.

pub mod index_state;

pub use index_state::{
    IndexStateResultPaneRecord, IndexStateStatusRecord, IndexStateSurfaceBundle,
    INDEX_STATE_RESULT_PANE_RECORD_KIND, INDEX_STATE_STATUS_RECORD_KIND,
    INDEX_STATE_SURFACE_BUNDLE_RECORD_KIND,
};
