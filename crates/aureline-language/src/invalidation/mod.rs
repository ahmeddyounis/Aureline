//! Incremental parse invalidation for editor-to-language updates.
//!
//! This module owns the alpha contract for applying editor edits to an
//! existing Tree-sitter tree, reparsing with that edited tree, and publishing
//! export-safe invalidation plus symbol-snapshot records for downstream
//! consumers.

use std::fmt;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use tree_sitter::{InputEdit, Point};

use crate::{
    BufferRef, CacheStatusClass, DerivedCueClass, ExportPolicy, IncrementalBudget,
    ParseCacheContext, ParseOutput, ParseRequest, ParseRequestClass, ParseSessionRecord,
    SourceRange, SymbolSnapshotExportRequest, SymbolSnapshotExporter, SymbolSnapshotRecord,
    TreeSitterParserSupervisor,
};

/// Integer schema version for [`IncrementalParseInvalidationRecord`] payloads.
pub type ParseInvalidationSchemaVersion = u32;

/// Workload class for an editor edit applied to parser state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditWorkloadClass {
    /// Single-line or very small typing-path edit.
    OrdinaryTyping,
    /// Larger paste, delete, format, or generated replacement region.
    LargeChangedRegion,
}

impl EditWorkloadClass {
    /// Returns the stable schema token for this workload class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryTyping => "ordinary_typing",
            Self::LargeChangedRegion => "large_changed_region",
        }
    }
}

/// Parse strategy selected after an edit invalidates syntax state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationDecisionClass {
    /// Previous Tree-sitter tree was edited and passed back to the parser.
    ReusePreviousTree,
    /// Parser fell back to a full parse because no previous tree was usable.
    ReparseFromScratch,
    /// Parser could not produce structure for the edited buffer.
    NoStructureAvailable,
}

impl InvalidationDecisionClass {
    /// Returns the stable schema token for this invalidation decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReusePreviousTree => "reuse_previous_tree",
            Self::ReparseFromScratch => "reparse_from_scratch",
            Self::NoStructureAvailable => "no_structure_available",
        }
    }
}

/// Error returned when an editor edit cannot be mapped onto the current buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationError {
    /// The edit byte range is outside the current buffer.
    EditRangeOutOfBounds {
        /// Edit start byte.
        start_byte: usize,
        /// Edit old-end byte.
        old_end_byte: usize,
        /// Current buffer byte length.
        buffer_len: usize,
    },
    /// The edit byte range does not align to UTF-8 boundaries.
    EditRangeNotUtf8Boundary {
        /// Edit start byte.
        start_byte: usize,
        /// Edit old-end byte.
        old_end_byte: usize,
    },
}

impl fmt::Display for InvalidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EditRangeOutOfBounds {
                start_byte,
                old_end_byte,
                buffer_len,
            } => write!(
                f,
                "edit range {start_byte}..{old_end_byte} is outside buffer length {buffer_len}"
            ),
            Self::EditRangeNotUtf8Boundary {
                start_byte,
                old_end_byte,
            } => write!(
                f,
                "edit range {start_byte}..{old_end_byte} does not align to UTF-8 boundaries"
            ),
        }
    }
}

impl std::error::Error for InvalidationError {}

/// Editor text edit admitted by the incremental parser lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEdit {
    /// Stable edit id for support joins.
    pub edit_id: String,
    /// Start byte in the current buffer before this edit is applied.
    pub start_byte: usize,
    /// Old end byte in the current buffer before this edit is applied.
    pub old_end_byte: usize,
    /// Replacement text inserted at the edit range.
    pub replacement_text: String,
}

impl TextEdit {
    /// Builds a replacement edit over a byte range.
    pub fn replace(
        edit_id: impl Into<String>,
        start_byte: usize,
        old_end_byte: usize,
        replacement_text: impl Into<String>,
    ) -> Self {
        Self {
            edit_id: edit_id.into(),
            start_byte,
            old_end_byte,
            replacement_text: replacement_text.into(),
        }
    }

    /// Returns the byte length removed by the edit.
    pub fn replaced_byte_len(&self) -> usize {
        self.old_end_byte.saturating_sub(self.start_byte)
    }

    /// Returns the byte length inserted by the edit.
    pub fn inserted_byte_len(&self) -> usize {
        self.replacement_text.len()
    }
}

/// Export-safe record for one edit mapped to Tree-sitter input coordinates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditOperationRecord {
    /// Stable edit id supplied by the caller.
    pub edit_id: String,
    /// Range removed from the previous buffer.
    pub old_range: SourceRange,
    /// Range occupied by replacement text in the new buffer.
    pub new_range: SourceRange,
    /// Workload class used for parser budget and benchmark grouping.
    pub workload_class: EditWorkloadClass,
    /// Number of bytes removed by the edit.
    pub replaced_byte_len: usize,
    /// Number of bytes inserted by the edit.
    pub inserted_byte_len: usize,
    /// Whether the edit spans more than one source line.
    pub spans_multiple_lines: bool,
    /// Reviewer-facing edit summary.
    pub summary: String,
}

/// Benchmark sample captured around one incremental parse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationBenchmarkSample {
    /// Workload class represented by this sample.
    pub workload_class: EditWorkloadClass,
    /// Elapsed parse time in microseconds for this sample.
    pub measured_parse_elapsed_micros: u128,
    /// Number of changed syntax ranges reported by Tree-sitter.
    pub changed_range_count: usize,
    /// Number of bytes covered by changed syntax ranges.
    pub changed_byte_count: usize,
    /// Number of edited bytes that triggered the parse.
    pub edited_byte_count: usize,
    /// Current buffer byte length after the edit.
    pub total_buffer_bytes: usize,
    /// Whether the previous tree was reused.
    pub reused_previous_tree: bool,
    /// Reviewer-facing benchmark summary.
    pub summary: String,
}

/// Exportable invalidation record for one incremental parse update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalParseInvalidationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this record.
    pub parse_invalidation_schema_version: ParseInvalidationSchemaVersion,
    /// Stable invalidation id.
    pub invalidation_id: String,
    /// Document reference whose parser state changed.
    pub document_ref: String,
    /// Previous buffer identity.
    pub previous_buffer_ref: BufferRef,
    /// Current buffer identity.
    pub current_buffer_ref: BufferRef,
    /// Previous syntax tree id, or `syntax-tree:none`.
    pub previous_syntax_tree_id: String,
    /// Current syntax tree id, or `syntax-tree:none`.
    pub current_syntax_tree_id: String,
    /// Edit operations applied before parsing.
    pub edit_operations: Vec<EditOperationRecord>,
    /// Changed syntax ranges reported by Tree-sitter.
    pub changed_ranges: Vec<SourceRange>,
    /// Number of bytes covered by changed syntax ranges.
    pub changed_byte_count: usize,
    /// Parser invalidation decision.
    pub decision_class: InvalidationDecisionClass,
    /// Cache status attached to the resulting parse session.
    pub cache_status_class: CacheStatusClass,
    /// Cue classes invalidated by this edit.
    pub invalidated_cue_classes: Vec<DerivedCueClass>,
    /// Benchmark sample for this incremental parse.
    pub benchmark_sample: InvalidationBenchmarkSample,
    /// Export policy inherited by this record.
    pub export_policy: ExportPolicy,
    /// Timestamp when the record was captured.
    pub captured_at: String,
    /// Export-safe reviewer summary.
    pub export_safe_summary: String,
}

impl IncrementalParseInvalidationRecord {
    /// Stable record-kind tag carried in serialized invalidation records.
    pub const RECORD_KIND: &'static str = "incremental_parse_invalidation_record";

    /// Integer schema version for invalidation records.
    pub const SCHEMA_VERSION: ParseInvalidationSchemaVersion = 1;
}

/// Result of applying one or more editor edits to incremental parser state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalParseUpdate {
    /// Invalidation record emitted for the edit.
    pub invalidation_record: IncrementalParseInvalidationRecord,
    /// Parse-session record emitted after the edit.
    pub parse_session: ParseSessionRecord,
    /// Root node kind reported by the current parse, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_kind: Option<String>,
    /// Symbol snapshot exported from the edited parse.
    pub symbol_snapshot: SymbolSnapshotRecord,
}

/// Stateful incremental parse buffer for one editor document.
pub struct IncrementalParseBuffer {
    supervisor: TreeSitterParserSupervisor,
    source_text: String,
    latest_output: ParseOutput,
}

impl IncrementalParseBuffer {
    /// Opens a buffer by running an initial parse through the supplied supervisor.
    pub fn open(
        supervisor: TreeSitterParserSupervisor,
        request: ParseRequest,
        source_text: impl Into<String>,
    ) -> Self {
        let source_text = source_text.into();
        let latest_output = supervisor.parse_text(request, &source_text);
        Self {
            supervisor,
            source_text,
            latest_output,
        }
    }

    /// Opens a buffer using the curated launch-language grammar registry.
    pub fn open_with_default_registry(
        request: ParseRequest,
        source_text: impl Into<String>,
    ) -> Self {
        Self::open(
            TreeSitterParserSupervisor::with_default_registry(),
            request,
            source_text,
        )
    }

    /// Returns the current source text held by the parser buffer.
    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    /// Returns the latest parse-session record.
    pub fn latest_parse_session(&self) -> &ParseSessionRecord {
        &self.latest_output.record
    }

    /// Exports a symbol snapshot from the current parse state.
    pub fn export_symbol_snapshot(
        &self,
        request: SymbolSnapshotExportRequest,
    ) -> SymbolSnapshotRecord {
        SymbolSnapshotExporter::export(&self.latest_output, &self.source_text, request)
    }

    /// Applies one editor edit and returns invalidation plus symbol export records.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidationError`] when the edit range is outside the current
    /// buffer or not aligned to UTF-8 boundaries.
    pub fn apply_edit(
        &mut self,
        edit: TextEdit,
        parse_session_id: impl Into<String>,
        workspace_relative_path: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> Result<IncrementalParseUpdate, InvalidationError> {
        self.apply_edits(
            vec![edit],
            parse_session_id,
            workspace_relative_path,
            captured_at,
        )
    }

    /// Applies editor edits and returns invalidation plus symbol export records.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidationError`] when any edit range is outside the current
    /// buffer or not aligned to UTF-8 boundaries.
    pub fn apply_edits(
        &mut self,
        edits: Vec<TextEdit>,
        parse_session_id: impl Into<String>,
        workspace_relative_path: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> Result<IncrementalParseUpdate, InvalidationError> {
        let parse_session_id = parse_session_id.into();
        let workspace_relative_path = workspace_relative_path.into();
        let captured_at = captured_at.into();
        let previous_record = self.latest_output.record.clone();
        let previous_tree = self.latest_output.tree().cloned();
        let previous_syntax_tree_id = syntax_tree_id_from_record(&previous_record);
        let mut edited_tree = previous_tree.clone();
        let mut next_source = self.source_text.clone();
        let mut edit_operations = Vec::with_capacity(edits.len());

        for edit in edits {
            let (input_edit, operation) = apply_edit_to_source(&mut next_source, edit)?;
            if let Some(tree) = edited_tree.as_mut() {
                tree.edit(&input_edit);
            }
            edit_operations.push(operation);
        }

        let mut request =
            next_parse_request(&previous_record, parse_session_id, captured_at.clone());
        let cache_context = edited_tree.as_ref().map(|_| {
            ParseCacheContext::invalidated_by_edit(
                &request.buffer_ref,
                previous_syntax_tree_id.clone(),
            )
        });
        let parse_started = Instant::now();
        let new_output = if let (Some(edited_tree), Some(cache_context)) =
            (edited_tree.as_ref(), cache_context)
        {
            match self
                .supervisor
                .start_parser(request.runtime_session_id.clone(), &request.language_id)
            {
                Ok(mut handle) => handle.parse_text_with_cache_context(
                    request.clone(),
                    &next_source,
                    Some(edited_tree),
                    cache_context,
                ),
                Err(_) => self.supervisor.parse_text(request.clone(), &next_source),
            }
        } else {
            request.incremental_budget =
                IncrementalBudget::foreground_visible_file("cancel:parse:foreground");
            self.supervisor.parse_text(request.clone(), &next_source)
        };
        let measured_parse_elapsed_micros = parse_started.elapsed().as_micros();

        let changed_ranges = match (edited_tree.as_ref(), new_output.tree()) {
            (Some(edited_tree), Some(new_tree)) => edited_tree
                .changed_ranges(new_tree)
                .map(SourceRange::from)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        };
        let changed_byte_count = changed_ranges
            .iter()
            .map(|range| range.end_byte.saturating_sub(range.start_byte))
            .sum::<usize>();
        let changed_range_count = changed_ranges.len();
        let edited_byte_count = edit_operations
            .iter()
            .map(|operation| operation.replaced_byte_len.max(operation.inserted_byte_len))
            .sum::<usize>();
        let workload_class = aggregate_workload_class(&edit_operations);
        let decision_class = if new_output.tree().is_none() {
            InvalidationDecisionClass::NoStructureAvailable
        } else if previous_tree.is_some() {
            InvalidationDecisionClass::ReusePreviousTree
        } else {
            InvalidationDecisionClass::ReparseFromScratch
        };
        let current_syntax_tree_id = syntax_tree_id_from_record(&new_output.record);
        let benchmark_sample = InvalidationBenchmarkSample {
            workload_class,
            measured_parse_elapsed_micros,
            changed_range_count,
            changed_byte_count,
            edited_byte_count,
            total_buffer_bytes: next_source.len(),
            reused_previous_tree: previous_tree.is_some(),
            summary: format!(
                "{} parse reused_previous_tree={} changed_ranges={}.",
                workload_class.as_str(),
                previous_tree.is_some(),
                changed_range_count
            ),
        };
        let invalidation_record = IncrementalParseInvalidationRecord {
            record_kind: IncrementalParseInvalidationRecord::RECORD_KIND.into(),
            parse_invalidation_schema_version: IncrementalParseInvalidationRecord::SCHEMA_VERSION,
            invalidation_id: format!(
                "parse-invalidation:{}:{}",
                previous_record.buffer_ref.buffer_id, request.buffer_ref.buffer_version
            ),
            document_ref: previous_record.document_ref.clone(),
            previous_buffer_ref: previous_record.buffer_ref,
            current_buffer_ref: request.buffer_ref.clone(),
            previous_syntax_tree_id,
            current_syntax_tree_id,
            edit_operations,
            changed_ranges,
            changed_byte_count,
            decision_class,
            cache_status_class: new_output.record.cache_record.cache_status_class,
            invalidated_cue_classes: new_output.record.requested_derived_cue_classes.clone(),
            benchmark_sample,
            export_policy: new_output.record.export_policy.clone(),
            captured_at: captured_at.clone(),
            export_safe_summary: format!(
                "Incremental parse update used {} and produced {} changed syntax ranges.",
                decision_class.as_str(),
                changed_range_count
            ),
        };
        let symbol_snapshot = SymbolSnapshotExporter::export(
            &new_output,
            &next_source,
            SymbolSnapshotExportRequest::local_file(
                format!(
                    "symbol-snapshot:{}:{}",
                    sanitize_id(&workspace_relative_path),
                    request.buffer_ref.buffer_version
                ),
                workspace_relative_path,
                captured_at,
            ),
        );
        let parse_session = new_output.record.clone();
        let root_kind = new_output.root_kind.clone();

        self.source_text = next_source;
        self.latest_output = new_output;

        Ok(IncrementalParseUpdate {
            invalidation_record,
            parse_session,
            root_kind,
            symbol_snapshot,
        })
    }
}

fn apply_edit_to_source(
    source_text: &mut String,
    edit: TextEdit,
) -> Result<(InputEdit, EditOperationRecord), InvalidationError> {
    validate_edit_range(source_text, &edit)?;
    let start_position = point_for_byte(source_text, edit.start_byte);
    let old_end_position = point_for_byte(source_text, edit.old_end_byte);
    let new_end_byte = edit.start_byte + edit.replacement_text.len();
    let new_end_position = point_after_text(start_position, &edit.replacement_text);
    let old_range = SourceRange {
        start_byte: edit.start_byte,
        end_byte: edit.old_end_byte,
        start_point: start_position.into(),
        end_point: old_end_position.into(),
    };
    let new_range = SourceRange {
        start_byte: edit.start_byte,
        end_byte: new_end_byte,
        start_point: start_position.into(),
        end_point: new_end_position.into(),
    };
    let workload_class = classify_workload(&edit, old_range, new_range);
    let spans_multiple_lines = old_range.start_point.row != old_range.end_point.row
        || new_range.start_point.row != new_range.end_point.row;
    let operation = EditOperationRecord {
        edit_id: edit.edit_id.clone(),
        old_range,
        new_range,
        workload_class,
        replaced_byte_len: edit.replaced_byte_len(),
        inserted_byte_len: edit.inserted_byte_len(),
        spans_multiple_lines,
        summary: format!(
            "{} edit replaced {} bytes with {} bytes.",
            workload_class.as_str(),
            edit.replaced_byte_len(),
            edit.inserted_byte_len()
        ),
    };
    let input_edit = InputEdit {
        start_byte: edit.start_byte,
        old_end_byte: edit.old_end_byte,
        new_end_byte,
        start_position,
        old_end_position,
        new_end_position,
    };

    source_text.replace_range(edit.start_byte..edit.old_end_byte, &edit.replacement_text);
    Ok((input_edit, operation))
}

fn validate_edit_range(source_text: &str, edit: &TextEdit) -> Result<(), InvalidationError> {
    if edit.start_byte > edit.old_end_byte || edit.old_end_byte > source_text.len() {
        return Err(InvalidationError::EditRangeOutOfBounds {
            start_byte: edit.start_byte,
            old_end_byte: edit.old_end_byte,
            buffer_len: source_text.len(),
        });
    }
    if !source_text.is_char_boundary(edit.start_byte)
        || !source_text.is_char_boundary(edit.old_end_byte)
    {
        return Err(InvalidationError::EditRangeNotUtf8Boundary {
            start_byte: edit.start_byte,
            old_end_byte: edit.old_end_byte,
        });
    }
    Ok(())
}

fn point_for_byte(source_text: &str, byte: usize) -> Point {
    let byte = byte.min(source_text.len());
    let mut row = 0;
    let mut line_start = 0;

    for (index, ch) in source_text.char_indices() {
        if index >= byte {
            break;
        }
        if ch == '\n' {
            row += 1;
            line_start = index + ch.len_utf8();
        }
    }

    Point {
        row,
        column: byte.saturating_sub(line_start),
    }
}

fn point_after_text(start: Point, text: &str) -> Point {
    let mut row = start.row;
    let mut column = start.column;

    for ch in text.chars() {
        if ch == '\n' {
            row += 1;
            column = 0;
        } else {
            column += ch.len_utf8();
        }
    }

    Point { row, column }
}

fn classify_workload(
    edit: &TextEdit,
    old_range: SourceRange,
    new_range: SourceRange,
) -> EditWorkloadClass {
    let touched_bytes = edit.replaced_byte_len().max(edit.inserted_byte_len());
    let touched_lines = old_range
        .end_point
        .row
        .saturating_sub(old_range.start_point.row)
        .max(
            new_range
                .end_point
                .row
                .saturating_sub(new_range.start_point.row),
        );

    if touched_bytes <= 128 && touched_lines <= 1 {
        EditWorkloadClass::OrdinaryTyping
    } else {
        EditWorkloadClass::LargeChangedRegion
    }
}

fn aggregate_workload_class(operations: &[EditOperationRecord]) -> EditWorkloadClass {
    if operations
        .iter()
        .any(|operation| operation.workload_class == EditWorkloadClass::LargeChangedRegion)
    {
        EditWorkloadClass::LargeChangedRegion
    } else {
        EditWorkloadClass::OrdinaryTyping
    }
}

fn next_parse_request(
    previous_record: &ParseSessionRecord,
    parse_session_id: String,
    captured_at: String,
) -> ParseRequest {
    let buffer_ref = BufferRef {
        buffer_id: previous_record.buffer_ref.buffer_id.clone(),
        buffer_version: previous_record.buffer_ref.buffer_version + 1,
        buffer_content_hash_ref: format!(
            "hash:buffer:{}:v{}",
            previous_record.buffer_ref.buffer_id,
            previous_record.buffer_ref.buffer_version + 1
        ),
        decoded_text_hash_ref: format!(
            "hash:decoded:{}:v{}",
            previous_record.buffer_ref.buffer_id,
            previous_record.buffer_ref.buffer_version + 1
        ),
        encoding_state_ref: previous_record.buffer_ref.encoding_state_ref.clone(),
        decode_recovery_state: previous_record.buffer_ref.decode_recovery_state.clone(),
    };

    ParseRequest {
        runtime_session_id: format!("parser-runtime:{parse_session_id}"),
        parse_session_id,
        document_ref: previous_record.document_ref.clone(),
        buffer_ref,
        language_id: previous_record.grammar_resolution.language_id.clone(),
        scope_ref: previous_record.grammar_resolution.scope_ref.clone(),
        coordinate_profile_ref: previous_record.coordinate_profile_ref.clone(),
        parse_request_class: ParseRequestClass::VisibleEditIncremental,
        requested_derived_cue_classes: previous_record.requested_derived_cue_classes.clone(),
        incremental_budget: IncrementalBudget::visible_edit("cancel:parse:visible-edit"),
        captured_at,
    }
}

fn syntax_tree_id_from_record(record: &ParseSessionRecord) -> String {
    record
        .syntax_tree_identity
        .as_ref()
        .map(|identity| identity.syntax_tree_id.clone())
        .unwrap_or_else(|| "syntax-tree:none".into())
}

fn sanitize_id(value: &str) -> String {
    value
        .trim()
        .trim_start_matches("language:")
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_owned()
}
