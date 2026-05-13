//! Syntax-token highlight records for Tree-sitter-backed editor views.
//!
//! The records in this module are view-independent: they carry editor
//! coordinates, byte offsets for support joins, and explicit provider labels
//! without requiring the paint layer to understand parser internals.

use serde::{Deserialize, Serialize};

use crate::viewport::TextPoint;

/// A source range expressed in editor coordinates and UTF-8 byte offsets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorTextRange {
    /// Inclusive start point for editor navigation and painting.
    pub start: TextPoint,
    /// Exclusive end point for editor navigation and painting.
    pub end: TextPoint,
    /// Inclusive UTF-8 byte offset in the decoded source snapshot.
    pub start_byte: usize,
    /// Exclusive UTF-8 byte offset in the decoded source snapshot.
    pub end_byte: usize,
}

/// Semantic class applied to a syntax-highlight token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntaxHighlightKind {
    /// Language keyword such as `function`, `class`, or `return`.
    Keyword,
    /// Named type, class, interface, or markup tag.
    Type,
    /// Function declaration or callable reference.
    Function,
    /// Method declaration or method-like member.
    Method,
    /// Object, record, selector, or attribute property.
    Property,
    /// Local variable, parameter, or binding.
    Variable,
    /// String literal or quoted scalar.
    String,
    /// Numeric literal.
    Number,
    /// Boolean or null-like constant.
    Constant,
    /// Source comment.
    Comment,
    /// Operator token.
    Operator,
    /// Punctuation or delimiter token.
    Punctuation,
    /// Markup tag token.
    Tag,
    /// Markup or configuration attribute token.
    Attribute,
    /// Parser error or missing-node token.
    Error,
    /// Plain-text fallback token when no grammar is available.
    PlainText,
}

impl SyntaxHighlightKind {
    /// Returns the stable schema token for this highlight kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyword => "keyword",
            Self::Type => "type",
            Self::Function => "function",
            Self::Method => "method",
            Self::Property => "property",
            Self::Variable => "variable",
            Self::String => "string",
            Self::Number => "number",
            Self::Constant => "constant",
            Self::Comment => "comment",
            Self::Operator => "operator",
            Self::Punctuation => "punctuation",
            Self::Tag => "tag",
            Self::Attribute => "attribute",
            Self::Error => "error",
            Self::PlainText => "plain_text",
        }
    }
}

/// Provider source for a syntax-highlight token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntaxHighlightSourceClass {
    /// Token was derived from the current Tree-sitter syntax tree.
    TreeSitter,
    /// Token was emitted by an explicitly labeled plain-text fallback.
    FallbackPlainText,
}

impl SyntaxHighlightSourceClass {
    /// Returns the stable schema token for this highlight source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TreeSitter => "tree_sitter",
            Self::FallbackPlainText => "fallback_plain_text",
        }
    }
}

/// One syntax-highlight token span for the editor paint layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxHighlightSpan {
    /// Text range covered by this token.
    pub range: EditorTextRange,
    /// Semantic highlight class selected for this token.
    pub kind: SyntaxHighlightKind,
    /// Provider source that produced this token.
    pub source_class: SyntaxHighlightSourceClass,
    /// Tree-sitter node kind or fallback token kind.
    pub node_kind: String,
    /// Short accessible description for non-color presentation.
    pub accessibility_label: String,
}
