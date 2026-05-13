//! Exportable file-local symbol snapshots derived from parser state.
//!
//! The snapshot types in this module keep Tree-sitter structural symbols in
//! the language crate so search, diagnostics, graph, and support consumers can
//! reuse one source-labeled contract instead of scraping editor outline state.

use serde::{Deserialize, Serialize};
use tree_sitter::Node;

use crate::{
    BufferRef, EpochBinding, ExportPolicy, ExportPolicyClass, FailureReasonClass,
    ParseFreshnessClass, ParseOutput, ParseQualityClass, ParseSessionRecord,
};

/// Integer schema version for [`SymbolSnapshotRecord`] payloads.
pub type SymbolSnapshotSchemaVersion = u32;

/// Zero-based byte-column point inside a decoded UTF-8 document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourcePoint {
    /// Zero-based line number.
    pub row: usize,
    /// Zero-based byte column within the line.
    pub column: usize,
}

impl From<tree_sitter::Point> for SourcePoint {
    fn from(point: tree_sitter::Point) -> Self {
        Self {
            row: point.row,
            column: point.column,
        }
    }
}

/// Byte and point range for an exported symbol or changed syntax span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceRange {
    /// Start byte in the decoded UTF-8 buffer.
    pub start_byte: usize,
    /// End byte in the decoded UTF-8 buffer.
    pub end_byte: usize,
    /// Start point in zero-based byte-column coordinates.
    pub start_point: SourcePoint,
    /// End point in zero-based byte-column coordinates.
    pub end_point: SourcePoint,
}

impl SourceRange {
    /// Builds a range from a Tree-sitter node.
    pub fn for_node(node: Node<'_>) -> Self {
        Self {
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_point: node.start_position().into(),
            end_point: node.end_position().into(),
        }
    }
}

impl From<tree_sitter::Range> for SourceRange {
    fn from(range: tree_sitter::Range) -> Self {
        Self {
            start_byte: range.start_byte,
            end_byte: range.end_byte,
            start_point: range.start_point.into(),
            end_point: range.end_point.into(),
        }
    }
}

/// File-local symbol role exported by the structural symbol snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKindClass {
    /// Module or file-level container.
    Module,
    /// Class, interface, struct, or equivalent type container.
    Class,
    /// Function declaration or function-valued binding.
    Function,
    /// Method declaration.
    Method,
    /// Import or export row.
    Import,
    /// Variable or constant binding.
    Variable,
    /// Object, JSON, YAML, or CSS property row.
    Property,
    /// Markup element row.
    Element,
    /// CSS selector row.
    Selector,
    /// Markdown or document section row.
    Section,
    /// Structural row whose language-specific role is unknown.
    Unknown,
}

impl SymbolKindClass {
    /// Returns the stable schema token for this symbol kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Class => "class",
            Self::Function => "function",
            Self::Method => "method",
            Self::Import => "import",
            Self::Variable => "variable",
            Self::Property => "property",
            Self::Element => "element",
            Self::Selector => "selector",
            Self::Section => "section",
            Self::Unknown => "unknown",
        }
    }
}

/// Producer class for exported symbol rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolProviderClass {
    /// Symbols were derived from the current Tree-sitter syntax tree.
    TreeSitter,
    /// Symbols are unavailable because the parser did not produce a tree.
    Unavailable,
}

impl SymbolProviderClass {
    /// Returns the stable schema token for this provider class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TreeSitter => "tree_sitter",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Completeness state attached to one symbol snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolSnapshotCompletenessClass {
    /// Snapshot covers the current file-local syntax tree.
    CompleteCurrentFile,
    /// Snapshot is available but parser errors can hide or distort rows.
    PartialParserErrors,
    /// Snapshot has no structural provider for this buffer.
    UnavailableNoStructure,
}

impl SymbolSnapshotCompletenessClass {
    /// Returns the stable schema token for this completeness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteCurrentFile => "complete_current_file",
            Self::PartialParserErrors => "partial_parser_errors",
            Self::UnavailableNoStructure => "unavailable_no_structure",
        }
    }
}

/// Visible state shared by symbol-search, diagnostics, graph, and support consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolSnapshotState {
    /// Provider that produced this snapshot.
    pub provider_class: SymbolProviderClass,
    /// Completeness of exported file-local symbols.
    pub completeness_class: SymbolSnapshotCompletenessClass,
    /// Freshness of the parse tree used for symbols.
    pub parse_freshness_class: ParseFreshnessClass,
    /// Quality of the parse tree used for symbols.
    pub parse_quality_class: ParseQualityClass,
    /// Symbol epoch advanced by this snapshot.
    pub symbol_epoch_ref: String,
    /// Syntax tree used by the snapshot, or `syntax-tree:none`.
    pub syntax_tree_id: String,
    /// Failure or degradation reasons inherited from parsing.
    pub degraded_reason_classes: Vec<FailureReasonClass>,
    /// Export-safe partial-truth cause tokens for downstream planners.
    pub partial_truth_causes: Vec<String>,
    /// Whether the snapshot can seed structural symbol search.
    pub search_consumable: bool,
    /// Whether the snapshot can seed graph or diagnostics ingest.
    pub graph_or_diagnostics_consumable: bool,
    /// Reviewer-facing state summary.
    pub summary: String,
}

/// One file-local symbol exported from a syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolRecord {
    /// Stable symbol row id within this snapshot.
    pub symbol_id: String,
    /// Stable symbol reference for search, diagnostics, and graph seed rows.
    pub stable_symbol_ref: String,
    /// Parent symbol id when this symbol is nested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_symbol_id: Option<String>,
    /// Qualified display path inside the file.
    pub qualified_name: String,
    /// Human-readable symbol name.
    pub name: String,
    /// File-local symbol kind.
    pub kind_class: SymbolKindClass,
    /// Workspace-relative path for navigation and search rows.
    pub workspace_relative_path: String,
    /// Full structural range for this symbol.
    pub range: SourceRange,
    /// Preferred selection or definition-name range.
    pub selection_range: SourceRange,
    /// Tree-sitter node kind that produced this symbol.
    pub node_kind: String,
    /// Parse session that produced this row.
    pub producer_parse_session_id: String,
    /// Freshness inherited from the parser.
    pub parse_freshness_class: ParseFreshnessClass,
    /// Export-safe evidence references for this symbol row.
    pub evidence_refs: Vec<String>,
    /// Candidate-specific partial-truth causes.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
}

/// Inputs that identify a symbol snapshot export without storing raw source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolSnapshotExportRequest {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace-relative path for search and navigation consumers.
    pub workspace_relative_path: String,
    /// Producer identity for the snapshot.
    pub producer_ref: String,
    /// Capture timestamp used by deterministic fixtures.
    pub captured_at: String,
}

impl SymbolSnapshotExportRequest {
    /// Builds a local-file snapshot export request.
    pub fn local_file(
        snapshot_id: impl Into<String>,
        workspace_relative_path: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> Self {
        Self {
            snapshot_id: snapshot_id.into(),
            workspace_relative_path: workspace_relative_path.into(),
            producer_ref: "language:symbol-snapshot:tree-sitter".into(),
            captured_at: captured_at.into(),
        }
    }
}

/// Exportable file-local symbol snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolSnapshotRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this snapshot.
    pub symbol_snapshot_schema_version: SymbolSnapshotSchemaVersion,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Document reference parsed by the source parse session.
    pub document_ref: String,
    /// Workspace-relative path for navigation and search consumers.
    pub workspace_relative_path: String,
    /// Buffer identity parsed by the source parse session.
    pub buffer_ref: BufferRef,
    /// Language id resolved for the parse session.
    pub language_id: String,
    /// Parse session that produced the snapshot.
    pub parse_session_id: String,
    /// Producer identity for this snapshot.
    pub producer_ref: String,
    /// Grammar version used by the source parse.
    pub grammar_version_ref: String,
    /// Query-pack identity used by the source parse.
    pub query_pack_ref: String,
    /// Visible snapshot state.
    pub state: SymbolSnapshotState,
    /// File-local symbol rows.
    pub symbols: Vec<SymbolRecord>,
    /// Epoch bindings current when symbols were exported.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Export policy for this record.
    pub export_policy: ExportPolicy,
    /// Timestamp when the record was captured.
    pub captured_at: String,
    /// Export-safe reviewer summary.
    pub export_safe_summary: String,
}

impl SymbolSnapshotRecord {
    /// Stable record-kind tag carried in serialized symbol snapshots.
    pub const RECORD_KIND: &'static str = "symbol_snapshot_record";

    /// Integer schema version for symbol snapshot records.
    pub const SCHEMA_VERSION: SymbolSnapshotSchemaVersion = 1;

    /// Returns true when consumers must show partial or degraded state.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.state.completeness_class != SymbolSnapshotCompletenessClass::CompleteCurrentFile
            || !self.state.degraded_reason_classes.is_empty()
    }

    /// Returns the first exported symbol with the requested name.
    pub fn symbol_named(&self, name: &str) -> Option<&SymbolRecord> {
        self.symbols.iter().find(|symbol| symbol.name == name)
    }
}

/// Builds symbol snapshots from parse outputs.
#[derive(Debug, Default, Clone, Copy)]
pub struct SymbolSnapshotExporter;

impl SymbolSnapshotExporter {
    /// Exports file-local symbols from a parse output without storing raw source.
    pub fn export(
        output: &ParseOutput,
        source_text: &str,
        request: SymbolSnapshotExportRequest,
    ) -> SymbolSnapshotRecord {
        let parse_record = &output.record;
        let syntax_tree_id = parse_record
            .syntax_tree_identity
            .as_ref()
            .map(|identity| identity.syntax_tree_id.clone())
            .unwrap_or_else(|| "syntax-tree:none".into());
        let mut symbols = output
            .tree()
            .map(|tree| {
                collect_symbols(
                    tree.root_node(),
                    source_text,
                    parse_record,
                    &request.workspace_relative_path,
                )
            })
            .unwrap_or_default();
        symbols.sort_by_key(|symbol| {
            (
                symbol.range.start_byte,
                symbol.range.end_byte,
                symbol.stable_symbol_ref.clone(),
            )
        });

        let degraded_reason_classes = degraded_reasons(parse_record);
        let completeness_class = if output.tree().is_none() {
            SymbolSnapshotCompletenessClass::UnavailableNoStructure
        } else if parse_record.parse_state.parse_quality_class == ParseQualityClass::FullTree {
            SymbolSnapshotCompletenessClass::CompleteCurrentFile
        } else {
            SymbolSnapshotCompletenessClass::PartialParserErrors
        };
        let partial_truth_causes = partial_truth_causes(parse_record, completeness_class);
        let symbol_epoch_ref = format!(
            "epoch:symbol-snapshot:{}:{}",
            sanitize_id(&request.workspace_relative_path),
            parse_record.buffer_ref.buffer_version
        );
        let state = SymbolSnapshotState {
            provider_class: if output.tree().is_some() {
                SymbolProviderClass::TreeSitter
            } else {
                SymbolProviderClass::Unavailable
            },
            completeness_class,
            parse_freshness_class: parse_record.parse_state.parse_freshness_class,
            parse_quality_class: parse_record.parse_state.parse_quality_class,
            symbol_epoch_ref: symbol_epoch_ref.clone(),
            syntax_tree_id: syntax_tree_id.clone(),
            degraded_reason_classes,
            partial_truth_causes,
            search_consumable: output.tree().is_some(),
            graph_or_diagnostics_consumable: output.tree().is_some(),
            summary: snapshot_state_summary(completeness_class, symbols.len()),
        };
        let symbol_count = symbols.len();
        let provider_label = if output.tree().is_some() {
            "Tree-sitter"
        } else {
            "no structural provider"
        };

        SymbolSnapshotRecord {
            record_kind: SymbolSnapshotRecord::RECORD_KIND.into(),
            symbol_snapshot_schema_version: SymbolSnapshotRecord::SCHEMA_VERSION,
            snapshot_id: request.snapshot_id,
            document_ref: parse_record.document_ref.clone(),
            workspace_relative_path: request.workspace_relative_path,
            buffer_ref: parse_record.buffer_ref.clone(),
            language_id: parse_record.grammar_resolution.language_id.clone(),
            parse_session_id: parse_record.parse_session_id.clone(),
            producer_ref: request.producer_ref,
            grammar_version_ref: parse_record.grammar_resolution.grammar_version_ref.clone(),
            query_pack_ref: parse_record.grammar_resolution.query_pack_ref.clone(),
            state,
            symbols,
            current_epoch_bindings: parse_record.current_epoch_bindings.clone(),
            export_policy: symbol_export_policy(),
            captured_at: request.captured_at,
            export_safe_summary: format!(
                "Symbol snapshot exported {symbol_count} file-local symbols from {provider_label} using {syntax_tree_id}."
            ),
        }
    }
}

fn collect_symbols(
    root: Node<'_>,
    source_text: &str,
    parse_record: &ParseSessionRecord,
    workspace_relative_path: &str,
) -> Vec<SymbolRecord> {
    let mut symbols = Vec::new();
    collect_symbols_from_node(
        root,
        source_text,
        parse_record,
        workspace_relative_path,
        None,
        None,
        &mut symbols,
    );
    symbols
}

fn collect_symbols_from_node(
    node: Node<'_>,
    source_text: &str,
    parse_record: &ParseSessionRecord,
    workspace_relative_path: &str,
    parent_symbol_id: Option<String>,
    parent_qualified_name: Option<String>,
    symbols: &mut Vec<SymbolRecord>,
) {
    let mut next_parent_id = parent_symbol_id.clone();
    let mut next_parent_name = parent_qualified_name.clone();

    if let Some((kind_class, name, selection_node)) = symbol_candidate(node, source_text) {
        let range = SourceRange::for_node(node);
        let selection_range = SourceRange::for_node(selection_node);
        let qualified_name = parent_qualified_name
            .as_ref()
            .map(|parent| format!("{parent}::{name}"))
            .unwrap_or_else(|| name.clone());
        let symbol_id = format!(
            "symbol:{}:{}:{}:{}",
            sanitize_id(workspace_relative_path),
            range.start_byte,
            range.end_byte,
            sanitize_id(&qualified_name)
        );
        let stable_symbol_ref = format!(
            "symbol-ref:{}:{}:{}:{}",
            sanitize_id(&parse_record.grammar_resolution.language_id),
            sanitize_id(workspace_relative_path),
            kind_class.as_str(),
            sanitize_id(&qualified_name)
        );

        symbols.push(SymbolRecord {
            symbol_id: symbol_id.clone(),
            stable_symbol_ref,
            parent_symbol_id,
            qualified_name: qualified_name.clone(),
            name,
            kind_class,
            workspace_relative_path: workspace_relative_path.into(),
            range,
            selection_range,
            node_kind: node.kind().into(),
            producer_parse_session_id: parse_record.parse_session_id.clone(),
            parse_freshness_class: parse_record.parse_state.parse_freshness_class,
            evidence_refs: vec![
                parse_record.parse_session_id.clone(),
                parse_record
                    .syntax_tree_identity
                    .as_ref()
                    .map(|identity| identity.syntax_tree_id.clone())
                    .unwrap_or_else(|| "syntax-tree:none".into()),
            ],
            partial_truth_causes: symbol_partial_truth_causes(parse_record),
        });
        next_parent_id = Some(symbol_id);
        next_parent_name = Some(qualified_name);
    }

    for index in 0..node.child_count() {
        if let Some(child) = node.child(index) {
            collect_symbols_from_node(
                child,
                source_text,
                parse_record,
                workspace_relative_path,
                next_parent_id.clone(),
                next_parent_name.clone(),
                symbols,
            );
        }
    }
}

fn symbol_candidate<'tree>(
    node: Node<'tree>,
    source_text: &str,
) -> Option<(SymbolKindClass, String, Node<'tree>)> {
    match node.kind() {
        "class_declaration" | "class_definition" | "interface_declaration" => {
            named_field(node, "name", source_text)
                .map(|(name, name_node)| (SymbolKindClass::Class, name, name_node))
        }
        "function_declaration" => named_field(node, "name", source_text)
            .map(|(name, name_node)| (SymbolKindClass::Function, name, name_node)),
        "function_definition" if is_method_like_function(node) => {
            named_field(node, "name", source_text)
                .map(|(name, name_node)| (SymbolKindClass::Method, name, name_node))
        }
        "function_definition" => named_field(node, "name", source_text)
            .map(|(name, name_node)| (SymbolKindClass::Function, name, name_node)),
        "method_definition" => named_field(node, "name", source_text)
            .map(|(name, name_node)| (SymbolKindClass::Method, name, name_node)),
        "variable_declarator" if has_function_initializer(node) => {
            named_field(node, "name", source_text)
                .map(|(name, name_node)| (SymbolKindClass::Function, name, name_node))
        }
        "lexical_declaration" | "variable_declaration" => {
            first_binding_identifier(node).map(|name_node| {
                (
                    SymbolKindClass::Variable,
                    normalize_label(text_for_node(name_node, source_text)),
                    name_node,
                )
            })
        }
        "import_statement" | "import_from_statement" => Some((
            SymbolKindClass::Import,
            first_line_label(node, source_text),
            node,
        )),
        "pair" | "block_mapping_pair" => named_field(node, "key", source_text)
            .map(|(name, key_node)| (SymbolKindClass::Property, name, key_node)),
        "element" => find_descendant_kind(node, "tag_name").map(|tag| {
            (
                SymbolKindClass::Element,
                normalize_label(text_for_node(tag, source_text)),
                tag,
            )
        }),
        "rule_set" => Some((
            SymbolKindClass::Selector,
            first_line_label(node, source_text),
            node,
        )),
        "atx_heading" | "setext_heading" => Some((
            SymbolKindClass::Section,
            heading_label(node, source_text),
            node,
        )),
        _ => None,
    }
}

fn named_field<'tree>(
    node: Node<'tree>,
    field_name: &str,
    source_text: &str,
) -> Option<(String, Node<'tree>)> {
    let child = node.child_by_field_name(field_name)?;
    Some((normalize_label(text_for_node(child, source_text)), child))
}

fn first_binding_identifier(node: Node<'_>) -> Option<Node<'_>> {
    find_descendant_kind(node, "identifier")
        .or_else(|| find_descendant_kind(node, "shorthand_property_identifier"))
}

fn has_function_initializer(node: Node<'_>) -> bool {
    node.child_by_field_name("value")
        .is_some_and(|value| matches!(value.kind(), "arrow_function" | "function"))
}

fn is_method_like_function(node: Node<'_>) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            "class_definition" => return true,
            "function_definition" | "function_declaration" => return false,
            _ => current = parent.parent(),
        }
    }

    false
}

fn find_descendant_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }

    for index in 0..node.child_count() {
        if let Some(child) = node.child(index) {
            if let Some(found) = find_descendant_kind(child, kind) {
                return Some(found);
            }
        }
    }

    None
}

fn text_for_node(node: Node<'_>, source_text: &str) -> String {
    node.utf8_text(source_text.as_bytes())
        .unwrap_or("")
        .to_owned()
}

fn first_line_label(node: Node<'_>, source_text: &str) -> String {
    let raw = text_for_node(node, source_text);
    let first_line = raw.lines().next().unwrap_or(raw.as_str());
    let before_block = first_line.split('{').next().unwrap_or(first_line);
    normalize_label(before_block)
}

fn heading_label(node: Node<'_>, source_text: &str) -> String {
    normalize_label(
        text_for_node(node, source_text)
            .trim_start_matches('#')
            .trim(),
    )
}

fn normalize_label(value: impl AsRef<str>) -> String {
    let joined = value
        .as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let trimmed = joined
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim_matches('`')
        .trim()
        .to_owned();

    if trimmed.chars().count() > 80 {
        format!("{}...", trimmed.chars().take(77).collect::<String>())
    } else if trimmed.is_empty() {
        "unnamed".into()
    } else {
        trimmed
    }
}

fn degraded_reasons(record: &ParseSessionRecord) -> Vec<FailureReasonClass> {
    record
        .parse_state
        .failure_reason_classes
        .iter()
        .copied()
        .filter(|reason| *reason != FailureReasonClass::None)
        .collect()
}

fn partial_truth_causes(
    record: &ParseSessionRecord,
    completeness_class: SymbolSnapshotCompletenessClass,
) -> Vec<String> {
    let mut causes = symbol_partial_truth_causes(record);
    if completeness_class == SymbolSnapshotCompletenessClass::UnavailableNoStructure {
        causes.push("syntax_tree_unavailable".into());
    }
    causes
}

fn symbol_partial_truth_causes(record: &ParseSessionRecord) -> Vec<String> {
    record
        .parse_state
        .failure_reason_classes
        .iter()
        .copied()
        .filter(|reason| *reason != FailureReasonClass::None)
        .map(|reason| reason.as_str().to_string())
        .collect()
}

fn snapshot_state_summary(
    completeness_class: SymbolSnapshotCompletenessClass,
    symbol_count: usize,
) -> String {
    format!(
        "Symbol snapshot is {} with {} file-local symbols.",
        completeness_class.as_str(),
        symbol_count
    )
}

fn symbol_export_policy() -> ExportPolicy {
    ExportPolicy {
        export_policy_class: ExportPolicyClass::MetadataSafeDefault,
        redaction_class: "metadata_safe_default".into(),
        raw_source_excluded: true,
        raw_parser_logs_excluded: true,
        range_export_requires_coordinate_mapping: true,
        summary: "Symbol snapshot export includes metadata and range refs, never raw source text."
            .into(),
    }
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
