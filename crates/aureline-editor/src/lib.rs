//! Editor viewport, composition, and paint primitives.
//!
//! This crate owns the canonical editor viewport model: scroll offsets, caret
//! and selection state, line-layout caching, and the software compositor used
//! by the current desktop shell raster path. Higher layers (shell zones,
//! command surfaces, and future multi-window wiring) should treat the types in
//! this crate as the single source of truth for editor viewport paint and
//! invalidation semantics.

#![doc(html_root_url = "https://docs.rs/aureline-editor/0.0.0")]

pub mod clipboard;
pub mod find_replace;
pub mod highlight;
pub mod large_file;
pub mod outline;
pub mod paint;
pub mod selection;
pub mod text_nav;
pub mod undo;
pub mod viewport;

pub use find_replace::{FindOptions, FindReplaceError, FindReplaceMode, FindReplaceState};
pub use highlight::{
    EditorTextRange, HighlightOverlaySet, HighlightSpan, SyntaxHighlightKind,
    SyntaxHighlightSourceClass, SyntaxHighlightSpan,
};
pub use large_file::{
    open_document, BomKind, ClassificationDecision, ClassificationPolicy, DocumentOpenDisposition,
    DocumentOpenError, DocumentOpenOutcome, FileMode, LargeFileDocument, LargeFileModeNotice,
    LargeFileOverrideInfo, LargeFileTrigger, LargeFileViewer, LargeFileViewerConfig,
    LargeFileViewerError, NormalDocument, ReaderMetrics,
};
pub use outline::{
    EditorStructuralSnapshot, FoldRange, FoldVisibilityState, OutlineNode, OutlineNodeKind,
    StructuralEditorAnalyzer, StructuralFeatureState, StructuralProviderClass,
    StructuralSnapshotSchemaVersion, StructuralSurfaceState,
};
pub use paint::{EditorTextRuntime, ViewportCompositor, ViewportPaintStyle};
pub use selection::{CaretSelection, SelectionState, TextEditOutcome, TextEditScope};
pub use undo::UndoGroupSummary;
pub use viewport::{
    CaretMove, EditorAction, EditorViewport, EditorViewportSnapshot, ImeComposition,
    SecondarySelectionSnapshot, SelectionDelta, TextPoint, ViewportDamage,
};
