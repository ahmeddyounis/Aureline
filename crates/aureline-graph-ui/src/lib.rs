//! Graph understanding projections for topology maps, impact explorers, and cited explainers.
//!
//! This crate is the data-surface layer between the shared semantic graph and
//! product UI surfaces. It does not perform canvas layout. Instead it builds
//! stable, exportable records that keep graph node and edge identity, scope
//! vocabulary, evidence citations, and list/table parity intact for renderers,
//! support packets, and headless consumers.

pub mod codebase_explainer;
pub mod impact_explorer;
pub mod topology;

pub use codebase_explainer::{
    CitationRef, CitationValidationError, CodebaseExplainerSurface, ExplainerClaim,
    ExplainerExportPacket, ExplainerTextSource,
};
pub use impact_explorer::{
    BatchActionClass, ImpactConfidenceFamily, ImpactExplorerRow, ImpactExplorerSurface,
    ImpactReasonClass, LoadedScopeNote,
};
pub use topology::{
    GraphDisclosureState, GraphFreshnessProvenanceStrip, RelationLegendClass, RelationLegendEntry,
    ScopeVocabularyClass, SelectionState, SurfaceAction, SurfaceParityError, TopologyEdgeRow,
    TopologyNodeRow, TopologySurface,
};
