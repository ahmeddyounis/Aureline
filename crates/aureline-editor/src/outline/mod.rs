//! Tree-sitter-backed structural snapshots for editor navigation surfaces.
//!
//! The analyzer in this module is the first editor consumer of the shared
//! language runtime. It turns parse-session records into syntax-highlight
//! spans, fold ranges, and file-local outline nodes while preserving provider
//! labels, freshness, degraded states, and export-safe summaries.

use std::collections::BTreeSet;

use aureline_language::{
    DerivedCueClass, DerivedCuePostureClass, FailureReasonClass, ParseFreshnessClass,
    ParseQualityClass, ParseRequest, ParseSessionRecord, TreeSitterParserSupervisor,
};
use serde::{Deserialize, Serialize};
use tree_sitter::Node;
use unicode_segmentation::UnicodeSegmentation;

use crate::highlight::{
    EditorTextRange, SyntaxHighlightKind, SyntaxHighlightSourceClass, SyntaxHighlightSpan,
};
use crate::viewport::TextPoint;

/// Integer schema version for [`EditorStructuralSnapshot`] payloads.
pub type StructuralSnapshotSchemaVersion = u32;

/// Provider class used by a structural editor record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuralProviderClass {
    /// Structure was produced from the current Tree-sitter syntax tree.
    TreeSitter,
    /// Only an explicitly labeled plain-text fallback is available.
    FallbackPlainText,
    /// No structural provider is available for the requested cue.
    Unavailable,
}

impl StructuralProviderClass {
    /// Returns the stable schema token for this provider class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TreeSitter => "tree_sitter",
            Self::FallbackPlainText => "fallback_plain_text",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Availability state for one editor structural feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuralFeatureState {
    /// Feature is exact for the current parse tree.
    AvailableExact,
    /// Feature is available but narrowed by parser errors or partial data.
    AvailablePartial,
    /// Feature is served by an explicit limited fallback.
    FallbackLimited,
    /// Feature is unavailable and should render a visible degraded state.
    Unavailable,
}

impl StructuralFeatureState {
    /// Returns the stable schema token for this feature state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableExact => "available_exact",
            Self::AvailablePartial => "available_partial",
            Self::FallbackLimited => "fallback_limited",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Default fold visibility vocabulary shared with editor chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FoldVisibilityState {
    /// Fold range is available and currently expanded.
    Expanded,
    /// Fold range is available and currently folded.
    Folded,
    /// Fold is summarized without rendering its full body.
    SummaryOnly,
    /// Fold controls are disabled by large-file mode.
    DisabledLargeFile,
    /// Fold controls are disabled by low-resource mode.
    DisabledLowResource,
    /// Fold controls are disabled by user or workspace setting.
    DisabledBySetting,
}

impl FoldVisibilityState {
    /// Returns the stable schema token for this fold visibility state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Expanded => "expanded",
            Self::Folded => "folded",
            Self::SummaryOnly => "summary_only",
            Self::DisabledLargeFile => "disabled_large_file",
            Self::DisabledLowResource => "disabled_low_resource",
            Self::DisabledBySetting => "disabled_by_setting",
        }
    }
}

/// File-local outline node kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutlineNodeKind {
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

impl OutlineNodeKind {
    /// Returns the stable schema token for this outline node kind.
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

/// One foldable range derived from the syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldRange {
    /// Stable fold-range id for this buffer snapshot.
    pub fold_id: String,
    /// Text range covered by the fold.
    pub range: EditorTextRange,
    /// Human-readable fold summary.
    pub label: String,
    /// Tree-sitter node kind that produced the fold.
    pub node_kind: String,
    /// Provider source that produced the fold.
    pub provider_class: StructuralProviderClass,
    /// Current fold visibility state.
    pub visibility_state: FoldVisibilityState,
    /// Number of physical lines hidden when this range is folded.
    pub hidden_line_count: usize,
    /// Whether hidden diagnostics, conflicts, or trust warnings are known inside.
    pub contains_hidden_alerts: bool,
    /// Command id for keyboard-accessible fold toggling.
    pub keyboard_toggle_command: String,
    /// Short accessible description for fold chrome and screen readers.
    pub accessibility_label: String,
}

/// One file-local outline node derived from the syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlineNode {
    /// Stable outline node id for this buffer snapshot.
    pub node_id: String,
    /// Parent outline node id when this node is nested.
    pub parent_node_id: Option<String>,
    /// Zero-based nesting depth in the outline.
    pub depth: usize,
    /// Human-readable node label.
    pub label: String,
    /// Language-neutral role for this node.
    pub kind: OutlineNodeKind,
    /// Full structural range for this node.
    pub range: EditorTextRange,
    /// Preferred selection range, usually the declaration name.
    pub selection_range: EditorTextRange,
    /// Provider source that produced this node.
    pub provider_class: StructuralProviderClass,
    /// Freshness of the syntax tree used for this node.
    pub freshness_class: ParseFreshnessClass,
    /// Short accessible description for outline rows and quick navigation.
    pub accessibility_label: String,
}

/// Shared visible state for syntax highlighting, folds, and outline data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralSurfaceState {
    /// Syntax-highlight availability state.
    pub highlighting: StructuralFeatureState,
    /// Fold availability state.
    pub folds: StructuralFeatureState,
    /// Outline availability state.
    pub outline: StructuralFeatureState,
    /// Provider class for the best current structural data.
    pub provider_class: StructuralProviderClass,
    /// Quality of the parse session backing this snapshot.
    pub parse_quality_class: ParseQualityClass,
    /// Freshness of the parse session backing this snapshot.
    pub parse_freshness_class: ParseFreshnessClass,
    /// Failure or degradation reasons that must be surfaced.
    pub degraded_reason_classes: Vec<FailureReasonClass>,
    /// Visible fallback label when a feature is narrowed or unavailable.
    pub fallback_label: Option<String>,
    /// Whether raw source text is excluded from exported state.
    pub raw_source_excluded: bool,
    /// Short accessible summary of the structural state.
    pub accessibility_summary: String,
}

/// Complete editor structural projection for one parsed buffer snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorStructuralSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this snapshot record.
    pub structural_snapshot_schema_version: StructuralSnapshotSchemaVersion,
    /// Document reference parsed by the snapshot.
    pub document_ref: String,
    /// Language id resolved for the snapshot.
    pub language_id: String,
    /// Parse session that produced this snapshot.
    pub parse_session_id: String,
    /// Boundary parse-session record consumed from the language runtime.
    pub parse_session: ParseSessionRecord,
    /// Visible structural feature states.
    pub state: StructuralSurfaceState,
    /// Syntax-highlight spans for the current buffer.
    pub highlights: Vec<SyntaxHighlightSpan>,
    /// Fold ranges for the current buffer.
    pub folds: Vec<FoldRange>,
    /// File-local outline rows for the current buffer.
    pub outline: Vec<OutlineNode>,
    /// Export-safe reviewer summary.
    pub export_safe_summary: String,
}

impl EditorStructuralSnapshot {
    /// Stable record-kind tag carried in serialized structural snapshots.
    pub const RECORD_KIND: &'static str = "editor_structural_snapshot";

    /// Integer schema version for editor structural snapshots.
    pub const SCHEMA_VERSION: StructuralSnapshotSchemaVersion = 1;

    /// Returns true when at least one structural feature is degraded.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.state.highlighting != StructuralFeatureState::AvailableExact
            || self.state.folds != StructuralFeatureState::AvailableExact
            || self.state.outline != StructuralFeatureState::AvailableExact
            || !self.state.degraded_reason_classes.is_empty()
    }
}

/// Builds editor syntax-highlight, fold, and outline records from Tree-sitter.
#[derive(Debug, Clone)]
pub struct StructuralEditorAnalyzer {
    supervisor: TreeSitterParserSupervisor,
}

impl StructuralEditorAnalyzer {
    /// Builds an analyzer over the curated launch-language registry.
    pub fn with_default_registry() -> Self {
        Self {
            supervisor: TreeSitterParserSupervisor::with_default_registry(),
        }
    }

    /// Builds an analyzer from a caller-provided parser supervisor.
    pub fn new(supervisor: TreeSitterParserSupervisor) -> Self {
        Self { supervisor }
    }

    /// Parses source text and returns editor structural records.
    pub fn analyze_text(
        &self,
        mut request: ParseRequest,
        source_text: &str,
    ) -> EditorStructuralSnapshot {
        ensure_structural_cues(&mut request);

        let output = self.supervisor.parse_text(request, source_text);
        let line_index = LineIndex::new(source_text);
        let parse_record = &output.record;

        let highlighting_state = feature_state(parse_record, DerivedCueClass::SyntaxHighlighting);
        let folds_state = feature_state(parse_record, DerivedCueClass::Folds);
        let outline_state = feature_state(parse_record, DerivedCueClass::Outline);

        let (mut highlights, mut folds, mut outline) = match output.tree() {
            Some(tree) => {
                let root = tree.root_node();
                let highlights = if highlighting_state != StructuralFeatureState::Unavailable {
                    collect_highlights(root, &line_index)
                } else {
                    Vec::new()
                };
                let folds = if folds_state == StructuralFeatureState::AvailableExact {
                    collect_folds(root, source_text, &line_index)
                } else {
                    Vec::new()
                };
                let outline = if outline_state == StructuralFeatureState::AvailableExact {
                    collect_outline(root, source_text, &line_index)
                } else {
                    Vec::new()
                };
                (highlights, folds, outline)
            }
            None => (Vec::new(), Vec::new(), Vec::new()),
        };

        if highlights.is_empty() && highlighting_state == StructuralFeatureState::FallbackLimited {
            highlights.push(fallback_plain_text_span(source_text, &line_index));
        }

        highlights.sort_by_key(|span| {
            (
                span.range.start_byte,
                span.range.end_byte,
                span.kind.as_str(),
            )
        });
        folds.sort_by_key(|fold| (fold.range.start_byte, fold.range.end_byte));
        outline.sort_by_key(|node| (node.range.start_byte, node.range.end_byte));

        let state = StructuralSurfaceState {
            highlighting: highlighting_state,
            folds: folds_state,
            outline: outline_state,
            provider_class: provider_class(parse_record),
            parse_quality_class: parse_record.parse_state.parse_quality_class,
            parse_freshness_class: parse_record.parse_state.parse_freshness_class,
            degraded_reason_classes: degraded_reasons(parse_record),
            fallback_label: fallback_label(
                parse_record,
                highlighting_state,
                folds_state,
                outline_state,
            ),
            raw_source_excluded: parse_record.export_policy.raw_source_excluded,
            accessibility_summary: accessibility_summary(
                highlighting_state,
                folds_state,
                outline_state,
                parse_record,
            ),
        };

        EditorStructuralSnapshot {
            record_kind: EditorStructuralSnapshot::RECORD_KIND.into(),
            structural_snapshot_schema_version: EditorStructuralSnapshot::SCHEMA_VERSION,
            document_ref: parse_record.document_ref.clone(),
            language_id: parse_record.grammar_resolution.language_id.clone(),
            parse_session_id: parse_record.parse_session_id.clone(),
            parse_session: output.record,
            export_safe_summary: format!(
                "Editor structural snapshot has {} highlight spans, {} folds, and {} outline nodes.",
                highlights.len(),
                folds.len(),
                outline.len()
            ),
            state,
            highlights,
            folds,
            outline,
        }
    }
}

impl Default for StructuralEditorAnalyzer {
    fn default() -> Self {
        Self::with_default_registry()
    }
}

fn ensure_structural_cues(request: &mut ParseRequest) {
    for cue in [
        DerivedCueClass::SyntaxHighlighting,
        DerivedCueClass::Folds,
        DerivedCueClass::Outline,
        DerivedCueClass::Breadcrumbs,
        DerivedCueClass::LocalSymbols,
        DerivedCueClass::SupportExport,
    ] {
        if !request.requested_derived_cue_classes.contains(&cue) {
            request.requested_derived_cue_classes.push(cue);
        }
    }
}

fn feature_state(record: &ParseSessionRecord, cue: DerivedCueClass) -> StructuralFeatureState {
    let posture = record
        .derived_cues
        .iter()
        .find(|entry| entry.derived_cue_class == cue)
        .map(|entry| entry.derived_cue_posture_class);

    match posture {
        Some(DerivedCuePostureClass::AvailableExact) => StructuralFeatureState::AvailableExact,
        Some(DerivedCuePostureClass::AvailablePartial | DerivedCuePostureClass::CachedOnly) => {
            StructuralFeatureState::AvailablePartial
        }
        Some(DerivedCuePostureClass::FallbackHeuristic) => StructuralFeatureState::FallbackLimited,
        Some(
            DerivedCuePostureClass::SuppressedDueToDegradation | DerivedCuePostureClass::Blocked,
        )
        | None => StructuralFeatureState::Unavailable,
    }
}

fn provider_class(record: &ParseSessionRecord) -> StructuralProviderClass {
    if record.syntax_tree_identity.is_some() {
        StructuralProviderClass::TreeSitter
    } else if feature_state(record, DerivedCueClass::SyntaxHighlighting)
        == StructuralFeatureState::FallbackLimited
    {
        StructuralProviderClass::FallbackPlainText
    } else {
        StructuralProviderClass::Unavailable
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

fn fallback_label(
    record: &ParseSessionRecord,
    highlighting: StructuralFeatureState,
    folds: StructuralFeatureState,
    outline: StructuralFeatureState,
) -> Option<String> {
    if highlighting == StructuralFeatureState::AvailableExact
        && folds == StructuralFeatureState::AvailableExact
        && outline == StructuralFeatureState::AvailableExact
        && !record.requires_degraded_disclosure()
    {
        None
    } else {
        Some(record.parse_state.summary.clone())
    }
}

fn accessibility_summary(
    highlighting: StructuralFeatureState,
    folds: StructuralFeatureState,
    outline: StructuralFeatureState,
    record: &ParseSessionRecord,
) -> String {
    format!(
        "Syntax highlighting is {}, folds are {}, and outline is {} for {}.",
        highlighting.as_str(),
        folds.as_str(),
        outline.as_str(),
        record.grammar_resolution.language_id
    )
}

fn collect_highlights(root: Node<'_>, line_index: &LineIndex<'_>) -> Vec<SyntaxHighlightSpan> {
    let mut spans = Vec::new();
    collect_highlights_from_node(root, line_index, &mut spans);
    spans
}

fn collect_highlights_from_node(
    node: Node<'_>,
    line_index: &LineIndex<'_>,
    spans: &mut Vec<SyntaxHighlightSpan>,
) {
    if let Some(kind) = highlight_kind_for_node(node) {
        if node.end_byte() > node.start_byte() {
            spans.push(SyntaxHighlightSpan {
                range: line_index.range_for_node(node),
                kind,
                source_class: SyntaxHighlightSourceClass::TreeSitter,
                node_kind: node.kind().into(),
                accessibility_label: format!(
                    "{} token from Tree-sitter node `{}`",
                    kind.as_str(),
                    node.kind()
                ),
            });
        }

        if matches!(
            kind,
            SyntaxHighlightKind::String
                | SyntaxHighlightKind::Number
                | SyntaxHighlightKind::Comment
                | SyntaxHighlightKind::Constant
        ) {
            return;
        }
    }

    for index in 0..node.child_count() {
        if let Some(child) = node.child(index) {
            collect_highlights_from_node(child, line_index, spans);
        }
    }
}

fn highlight_kind_for_node(node: Node<'_>) -> Option<SyntaxHighlightKind> {
    if node.is_error() || node.is_missing() {
        return Some(SyntaxHighlightKind::Error);
    }

    let kind = node.kind();
    match kind {
        "comment" => Some(SyntaxHighlightKind::Comment),
        "string"
        | "string_fragment"
        | "template_string"
        | "raw_string"
        | "interpreted_string_literal"
        | "double_quote_scalar"
        | "single_quote_scalar" => Some(SyntaxHighlightKind::String),
        "number" | "integer" | "float" => Some(SyntaxHighlightKind::Number),
        "true" | "false" | "null" | "undefined" | "True" | "False" | "None" => {
            Some(SyntaxHighlightKind::Constant)
        }
        "tag_name" => Some(SyntaxHighlightKind::Tag),
        "attribute_name" => Some(SyntaxHighlightKind::Attribute),
        "property_identifier" => Some(property_identifier_kind(node)),
        "type_identifier" | "predefined_type" => Some(SyntaxHighlightKind::Type),
        "identifier" | "shorthand_property_identifier" => Some(identifier_kind(node)),
        _ if is_keyword_kind(kind) => Some(SyntaxHighlightKind::Keyword),
        _ if is_operator_kind(kind) => Some(SyntaxHighlightKind::Operator),
        _ if is_punctuation_kind(kind) => Some(SyntaxHighlightKind::Punctuation),
        _ => None,
    }
}

fn property_identifier_kind(node: Node<'_>) -> SyntaxHighlightKind {
    match node.parent().map(|parent| parent.kind()) {
        Some("method_definition") => SyntaxHighlightKind::Method,
        Some("call_expression") => SyntaxHighlightKind::Function,
        _ => SyntaxHighlightKind::Property,
    }
}

fn identifier_kind(node: Node<'_>) -> SyntaxHighlightKind {
    let Some(parent) = node.parent() else {
        return SyntaxHighlightKind::Variable;
    };

    match parent.kind() {
        "class_declaration" | "class_definition" | "interface_declaration" => {
            SyntaxHighlightKind::Type
        }
        "function_declaration" => SyntaxHighlightKind::Function,
        "function_definition" if is_method_like_function(parent) => SyntaxHighlightKind::Method,
        "function_definition" => SyntaxHighlightKind::Function,
        "method_definition" => SyntaxHighlightKind::Method,
        "call_expression" if is_field_child(parent, node, "function") => {
            SyntaxHighlightKind::Function
        }
        "member_expression" if is_field_child(parent, node, "property") => {
            SyntaxHighlightKind::Property
        }
        _ => SyntaxHighlightKind::Variable,
    }
}

fn is_keyword_kind(kind: &str) -> bool {
    matches!(
        kind,
        "as" | "async"
            | "await"
            | "break"
            | "case"
            | "catch"
            | "class"
            | "const"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "export"
            | "extends"
            | "finally"
            | "for"
            | "from"
            | "function"
            | "if"
            | "import"
            | "in"
            | "interface"
            | "let"
            | "pass"
            | "return"
            | "try"
            | "var"
            | "while"
            | "with"
            | "yield"
    )
}

fn is_operator_kind(kind: &str) -> bool {
    matches!(
        kind,
        "=" | "=>"
            | "+"
            | "-"
            | "*"
            | "/"
            | "%"
            | "=="
            | "==="
            | "!="
            | "!=="
            | "<"
            | ">"
            | "<="
            | ">="
            | "&&"
            | "||"
            | "!"
            | "?"
    )
}

fn is_punctuation_kind(kind: &str) -> bool {
    matches!(
        kind,
        "{" | "}" | "[" | "]" | "(" | ")" | "." | "," | ":" | ";" | "<" | ">"
    )
}

fn fallback_plain_text_span(source_text: &str, line_index: &LineIndex<'_>) -> SyntaxHighlightSpan {
    SyntaxHighlightSpan {
        range: line_index.range_for_bytes(0, source_text.len()),
        kind: SyntaxHighlightKind::PlainText,
        source_class: SyntaxHighlightSourceClass::FallbackPlainText,
        node_kind: "plain_text_fallback".into(),
        accessibility_label: "Plain-text fallback token; syntax grammar unavailable.".into(),
    }
}

fn collect_folds(root: Node<'_>, source_text: &str, line_index: &LineIndex<'_>) -> Vec<FoldRange> {
    let mut folds = Vec::new();
    let mut seen_ranges = BTreeSet::new();
    collect_folds_from_node(root, source_text, line_index, &mut folds, &mut seen_ranges);
    folds
}

fn collect_folds_from_node(
    node: Node<'_>,
    source_text: &str,
    line_index: &LineIndex<'_>,
    folds: &mut Vec<FoldRange>,
    seen_ranges: &mut BTreeSet<(usize, usize)>,
) {
    if is_foldable_node(node) && seen_ranges.insert((node.start_byte(), node.end_byte())) {
        let range = line_index.range_for_node(node);
        let label = fold_label_for_node(node, source_text);
        let hidden_line_count = range.end.line.saturating_sub(range.start.line);
        folds.push(FoldRange {
            fold_id: format!(
                "fold:{}:{}:{}",
                range.start_byte,
                range.end_byte,
                sanitize_id(&label)
            ),
            range,
            label: label.clone(),
            node_kind: node.kind().into(),
            provider_class: StructuralProviderClass::TreeSitter,
            visibility_state: FoldVisibilityState::Expanded,
            hidden_line_count,
            contains_hidden_alerts: false,
            keyboard_toggle_command: "editor.fold.toggle".into(),
            accessibility_label: format!("{label}, {hidden_line_count} hidden lines when folded"),
        });
    }

    for index in 0..node.child_count() {
        if let Some(child) = node.child(index) {
            collect_folds_from_node(child, source_text, line_index, folds, seen_ranges);
        }
    }
}

fn is_foldable_node(node: Node<'_>) -> bool {
    let start_line = node.start_position().row;
    let end_line = node.end_position().row;
    if end_line <= start_line || !node.is_named() {
        return false;
    }

    matches!(
        node.kind(),
        "class_declaration"
            | "class_definition"
            | "function_declaration"
            | "function_definition"
            | "method_definition"
            | "if_statement"
            | "for_statement"
            | "while_statement"
            | "try_statement"
            | "catch_clause"
            | "statement_block"
            | "block"
            | "class_body"
            | "object"
            | "array"
            | "dictionary"
            | "list"
            | "tuple"
            | "element"
            | "script_element"
            | "style_element"
            | "rule_set"
            | "block_mapping"
            | "block_sequence"
    )
}

fn fold_label_for_node(node: Node<'_>, source_text: &str) -> String {
    if matches!(node.kind(), "statement_block" | "block" | "class_body") {
        if let Some(parent) = node.parent() {
            if let Some((_, label, _)) = outline_candidate(parent, source_text) {
                return label;
            }
        }
    }

    outline_candidate(node, source_text)
        .map(|(_, label, _)| label)
        .or_else(|| {
            node.parent()
                .and_then(|parent| outline_candidate(parent, source_text))
                .map(|(_, label, _)| label)
        })
        .unwrap_or_else(|| node.kind().replace('_', " "))
}

fn collect_outline(
    root: Node<'_>,
    source_text: &str,
    line_index: &LineIndex<'_>,
) -> Vec<OutlineNode> {
    let mut nodes = Vec::new();
    collect_outline_from_node(root, source_text, line_index, None, 0, &mut nodes);
    nodes
}

fn collect_outline_from_node(
    node: Node<'_>,
    source_text: &str,
    line_index: &LineIndex<'_>,
    parent_node_id: Option<String>,
    depth: usize,
    nodes: &mut Vec<OutlineNode>,
) {
    let mut next_parent = parent_node_id.clone();
    let mut next_depth = depth;

    if let Some((kind, label, selection_node)) = outline_candidate(node, source_text) {
        let range = line_index.range_for_node(node);
        let selection_range = line_index.range_for_node(selection_node);
        let node_id = format!(
            "outline:{}:{}:{}",
            range.start_byte,
            range.end_byte,
            sanitize_id(&label)
        );
        nodes.push(OutlineNode {
            node_id: node_id.clone(),
            parent_node_id,
            depth,
            label: label.clone(),
            kind,
            range,
            selection_range,
            provider_class: StructuralProviderClass::TreeSitter,
            freshness_class: ParseFreshnessClass::CurrentBufferVersion,
            accessibility_label: format!(
                "{} {} at line {}",
                kind.as_str(),
                label,
                selection_node.start_position().row + 1
            ),
        });
        next_parent = Some(node_id);
        next_depth = depth + 1;
    }

    for index in 0..node.child_count() {
        if let Some(child) = node.child(index) {
            collect_outline_from_node(
                child,
                source_text,
                line_index,
                next_parent.clone(),
                next_depth,
                nodes,
            );
        }
    }
}

fn outline_candidate<'tree>(
    node: Node<'tree>,
    source_text: &str,
) -> Option<(OutlineNodeKind, String, Node<'tree>)> {
    match node.kind() {
        "class_declaration" | "class_definition" | "interface_declaration" => {
            named_field(node, "name", source_text)
                .map(|(label, name)| (OutlineNodeKind::Class, label, name))
        }
        "function_declaration" => named_field(node, "name", source_text)
            .map(|(label, name)| (OutlineNodeKind::Function, label, name)),
        "function_definition" if is_method_like_function(node) => {
            named_field(node, "name", source_text)
                .map(|(label, name)| (OutlineNodeKind::Method, label, name))
        }
        "function_definition" => named_field(node, "name", source_text)
            .map(|(label, name)| (OutlineNodeKind::Function, label, name)),
        "method_definition" => named_field(node, "name", source_text)
            .map(|(label, name)| (OutlineNodeKind::Method, label, name)),
        "variable_declarator" if has_function_initializer(node) => {
            named_field(node, "name", source_text)
                .map(|(label, name)| (OutlineNodeKind::Function, label, name))
        }
        "import_statement" | "import_from_statement" => Some((
            OutlineNodeKind::Import,
            first_line_label(node, source_text),
            node,
        )),
        "pair" | "block_mapping_pair" => named_field(node, "key", source_text)
            .map(|(label, key)| (OutlineNodeKind::Property, label, key)),
        "element" => find_descendant_kind(node, "tag_name").map(|tag| {
            (
                OutlineNodeKind::Element,
                normalize_label(text_for_node(tag, source_text)),
                tag,
            )
        }),
        "rule_set" => Some((
            OutlineNodeKind::Selector,
            first_line_label(node, source_text),
            node,
        )),
        "atx_heading" | "setext_heading" => Some((
            OutlineNodeKind::Section,
            heading_label(node, source_text),
            node,
        )),
        _ => None,
    }
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

fn named_field<'tree>(
    node: Node<'tree>,
    field_name: &str,
    source_text: &str,
) -> Option<(String, Node<'tree>)> {
    let child = node.child_by_field_name(field_name)?;
    Some((normalize_label(text_for_node(child, source_text)), child))
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

fn text_for_node(node: Node<'_>, source_text: &str) -> String {
    node.utf8_text(source_text.as_bytes())
        .unwrap_or("")
        .to_owned()
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

fn is_field_child(parent: Node<'_>, node: Node<'_>, field_name: &str) -> bool {
    parent
        .child_by_field_name(field_name)
        .is_some_and(|candidate| same_node(candidate, node))
}

fn same_node(left: Node<'_>, right: Node<'_>) -> bool {
    left.kind() == right.kind()
        && left.start_byte() == right.start_byte()
        && left.end_byte() == right.end_byte()
}

fn sanitize_id(value: &str) -> String {
    value
        .trim()
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

struct SourceLine<'a> {
    start_byte: usize,
    text: &'a str,
}

struct LineIndex<'a> {
    source_text: &'a str,
    lines: Vec<SourceLine<'a>>,
}

impl<'a> LineIndex<'a> {
    fn new(source_text: &'a str) -> Self {
        let mut lines = Vec::new();
        let mut line_start = 0;

        for (byte_index, ch) in source_text.char_indices() {
            if ch == '\n' {
                lines.push(SourceLine {
                    start_byte: line_start,
                    text: &source_text[line_start..byte_index],
                });
                line_start = byte_index + ch.len_utf8();
            }
        }

        lines.push(SourceLine {
            start_byte: line_start,
            text: &source_text[line_start..],
        });

        Self { source_text, lines }
    }

    fn range_for_node(&self, node: Node<'_>) -> EditorTextRange {
        self.range_for_bytes(node.start_byte(), node.end_byte())
    }

    fn range_for_bytes(&self, start_byte: usize, end_byte: usize) -> EditorTextRange {
        EditorTextRange {
            start: self.point_for_byte(start_byte),
            end: self.point_for_byte(end_byte),
            start_byte,
            end_byte,
        }
    }

    fn point_for_byte(&self, byte: usize) -> TextPoint {
        let byte = byte.min(self.source_text.len());
        let line_index = self
            .lines
            .partition_point(|line| line.start_byte <= byte)
            .saturating_sub(1);
        let line = &self.lines[line_index];
        let mut relative = byte.saturating_sub(line.start_byte).min(line.text.len());
        while !line.text.is_char_boundary(relative) {
            relative = relative.saturating_sub(1);
        }

        TextPoint {
            line: line_index,
            grapheme: line.text[..relative].graphemes(true).count(),
        }
    }
}
