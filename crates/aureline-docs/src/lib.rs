//! Docs-node identity and citation evidence primitives.
//!
//! This crate owns the bounded alpha records that let docs/help rows,
//! graph explainers, onboarding packs, support exports, and AI evidence
//! packets preserve one citation vocabulary. The records carry stable ids,
//! pack revisions, locale/fallback state, freshness, locality, exact-anchor
//! availability, and inference/confidence labels. They intentionally do not
//! carry raw document bodies, raw source files, raw URLs, or prompt text.

#![doc(html_root_url = "https://docs.rs/aureline-docs/0.0.0")]

pub mod citations;
pub mod index;
pub mod pack;

pub use citations::{
    CitationAnchorAlpha, CitationAnchorAlphaInput, CitationAnchorAvailability,
    CitationConfidenceClass, CitationDrawerEvidenceView, CitationDrawerEvidenceViewInput,
    CitationDrawerRow, CitationEvidenceExport, CitationEvidenceExportInput,
    CitationInferenceMarker, CitationLocalityClass, CitationSourceClass, CitationTruthViolation,
    DocsFreshnessClass, DocsNodeIdentity, DocsNodeIdentityInput, DocsNodeKind, DocsScopeClass,
    HelpPackItemEvidence, LocaleOverlayState, SourcePrecedenceClass, VersionMatchState,
    CITATION_ANCHOR_ALPHA_RECORD_KIND, CITATION_DRAWER_ALPHA_RECORD_KIND,
    CITATION_EVIDENCE_EXPORT_ALPHA_RECORD_KIND, DOCS_CITATION_ALPHA_SCHEMA_VERSION,
    DOCS_NODE_ALPHA_RECORD_KIND,
};
pub use index::{
    DocsSearchIndex, DocsSearchIndexEntry, DocsSearchQueryResult,
    DOCS_SEARCH_INDEX_ENTRY_RECORD_KIND, DOCS_SEARCH_INDEX_RECORD_KIND,
    DOCS_SEARCH_INDEX_SCHEMA_VERSION, DOCS_SEARCH_QUERY_RESULT_RECORD_KIND,
    DOCS_SEARCH_RESULT_KIND_TOKEN,
};
pub use pack::{
    DocsPack, DocsPackLoadError, DocsPackNode, DocsPackSourceTruth, DOCS_PACK_ALPHA_RECORD_KIND,
    DOCS_PACK_ALPHA_SCHEMA_VERSION,
};
