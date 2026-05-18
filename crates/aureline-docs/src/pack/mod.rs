//! Docs-pack loading for citation-aware docs/help content.
//!
//! This module turns checked-in YAML manifests or Markdown files with YAML
//! front matter into [`DocsNodeIdentity`] records. It intentionally consumes
//! the citation vocabulary from [`crate::citations`] instead of defining
//! parallel source, version, freshness, locality, or anchor tokens.

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::citations::{
    CitationAnchorAvailability, CitationLocalityClass, CitationSourceClass, CitationTruthViolation,
    DocsFreshnessClass, DocsNodeIdentity, DocsNodeIdentityInput, DocsNodeKind, DocsScopeClass,
    LocaleOverlayState, VersionMatchState,
};

/// Schema version used by docs-pack alpha manifests.
pub const DOCS_PACK_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DocsPack`] payloads.
pub const DOCS_PACK_ALPHA_RECORD_KIND: &str = "docs_pack_alpha_record";

/// Loaded docs pack with body content and resolved docs-node identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPack {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable pack id.
    pub pack_id: String,
    /// Pack revision, source snapshot, or compatibility revision.
    pub pack_revision_ref: String,
    /// Human-readable pack label.
    pub pack_label: String,
    /// Canonical source locale for pack content.
    pub source_locale: String,
    /// Requested locale for this loaded projection.
    pub requested_locale: String,
    /// Effective rendered locale after fallback.
    pub effective_locale: String,
    /// Source, version, freshness, locality, and handoff truth for the pack.
    pub source_truth: DocsPackSourceTruth,
    /// Docs nodes resolved from the pack.
    pub nodes: Vec<DocsPackNode>,
}

impl DocsPack {
    /// Loads a docs pack from a `.yaml`, `.yml`, or `.md` path.
    ///
    /// # Errors
    ///
    /// Returns [`DocsPackLoadError`] when the file cannot be read, the format
    /// is unsupported, YAML cannot be parsed, required fields are missing, or
    /// a resolved [`DocsNodeIdentity`] violates citation truth rules.
    pub fn load_path(path: impl AsRef<Path>) -> Result<Self, DocsPackLoadError> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path).map_err(|source| DocsPackLoadError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("yaml" | "yml") => Self::from_yaml_str(&raw),
            Some("md" | "markdown") => Self::from_markdown_str(&raw),
            Some(extension) => Err(DocsPackLoadError::UnsupportedExtension {
                path: path.to_path_buf(),
                extension: extension.to_owned(),
            }),
            None => Err(DocsPackLoadError::UnsupportedExtension {
                path: path.to_path_buf(),
                extension: String::new(),
            }),
        }
    }

    /// Loads a docs pack from a YAML manifest string.
    ///
    /// # Errors
    ///
    /// Returns [`DocsPackLoadError`] when parsing or semantic validation fails.
    pub fn from_yaml_str(raw: &str) -> Result<Self, DocsPackLoadError> {
        let manifest = parse_manifest(raw)?;
        build_pack(manifest, None)
    }

    /// Loads a docs pack from a Markdown document with YAML front matter.
    ///
    /// The front matter may carry either `nodes` or a single `node`; when a
    /// single `node` omits `body_markdown`, the Markdown body becomes that
    /// node's body.
    ///
    /// # Errors
    ///
    /// Returns [`DocsPackLoadError`] when front matter is missing, parsing
    /// fails, or semantic validation fails.
    pub fn from_markdown_str(raw: &str) -> Result<Self, DocsPackLoadError> {
        let (front_matter, body) = split_markdown_front_matter(raw)?;
        let manifest = parse_manifest(front_matter)?;
        build_pack(manifest, Some(body.to_owned()))
    }

    /// Returns the resolved docs-node identities for this pack.
    pub fn docs_node_identities(&self) -> impl Iterator<Item = &DocsNodeIdentity> {
        self.nodes.iter().map(|node| &node.docs_node)
    }
}

/// Pack-level source, version, freshness, locality, and browser-handoff truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackSourceTruth {
    /// Source class for nodes in this pack unless overridden by a node.
    pub source_class: CitationSourceClass,
    /// Scope class for nodes in this pack unless overridden by a node.
    pub scope_class: DocsScopeClass,
    /// Version or revision represented by this pack.
    pub version_or_revision_ref: String,
    /// Version-match state against the active target.
    pub version_match_state: VersionMatchState,
    /// Freshness state at pack mint time.
    pub freshness_class: DocsFreshnessClass,
    /// Locality posture for this pack.
    pub locality_class: CitationLocalityClass,
    /// Default citation-anchor availability for pack nodes.
    pub citation_availability: CitationAnchorAvailability,
    /// Running build identity used by shell-side docs/browser rows.
    pub running_build_identity_ref: String,
    /// Source build date or deterministic build stamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_build_at: Option<String>,
    /// Optional source snapshot age label for UI rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_age_label: Option<String>,
    /// Optional Help/About status badge ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_status_badge_ref: Option<String>,
    /// Optional system-browser handoff packet ref for the pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Default source-language fallback ref when locale fallback applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language_fallback_ref: Option<String>,
    /// Default disclosure note for hidden, omitted, or missing anchors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_note: Option<String>,
}

/// One content item resolved from a docs pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackNode {
    /// Resolved docs-node identity for citation-aware consumers.
    pub docs_node: DocsNodeIdentity,
    /// User-visible title.
    pub title: String,
    /// Export-safe summary for result rows and support packets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Stable source ref used to reconstruct the source material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Markdown body for the node.
    pub body_markdown: String,
}

/// Error returned when a docs pack cannot be loaded or validated.
#[derive(Debug)]
pub enum DocsPackLoadError {
    /// Filesystem read failed.
    Io {
        /// Path that failed to read.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// The path extension is not a supported docs-pack format.
    UnsupportedExtension {
        /// Path that was rejected.
        path: PathBuf,
        /// Extension that was rejected.
        extension: String,
    },
    /// Markdown input did not start with a YAML front matter block.
    MissingMarkdownFrontMatter,
    /// YAML parsing failed.
    ParseYaml {
        /// Parser error detail.
        message: String,
    },
    /// The manifest declared a schema version this loader does not support.
    UnsupportedSchemaVersion {
        /// Schema version declared by the manifest.
        schema_version: u32,
    },
    /// The required `source_truth` block was absent.
    MissingSourceTruth,
    /// A required string field was absent or blank.
    MissingField {
        /// Field path relative to the pack manifest.
        field: &'static str,
    },
    /// The pack contained no docs nodes.
    EmptyNodes,
    /// A resolved docs-node identity failed citation truth validation.
    InvalidDocsNode {
        /// Docs node id being validated.
        docs_node_id: String,
        /// Violations reported by [`DocsNodeIdentity::validate`].
        violations: Vec<CitationTruthViolation>,
    },
}

impl fmt::Display for DocsPackLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(
                    formatter,
                    "failed to read docs pack {}: {source}",
                    path.display()
                )
            }
            Self::UnsupportedExtension { path, extension } => write!(
                formatter,
                "unsupported docs pack extension {extension:?} for {}",
                path.display()
            ),
            Self::MissingMarkdownFrontMatter => {
                write!(formatter, "markdown docs pack is missing YAML front matter")
            }
            Self::ParseYaml { message } => {
                write!(formatter, "failed to parse docs pack YAML: {message}")
            }
            Self::UnsupportedSchemaVersion { schema_version } => write!(
                formatter,
                "unsupported docs pack schema version {schema_version}"
            ),
            Self::MissingSourceTruth => write!(formatter, "docs pack is missing source_truth"),
            Self::MissingField { field } => {
                write!(formatter, "docs pack is missing required field {field}")
            }
            Self::EmptyNodes => write!(formatter, "docs pack must contain at least one node"),
            Self::InvalidDocsNode {
                docs_node_id,
                violations,
            } => write!(
                formatter,
                "docs node {docs_node_id} failed citation validation: {violations:?}"
            ),
        }
    }
}

impl Error for DocsPackLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DocsPackManifest {
    schema_version: Option<u32>,
    pack_id: Option<String>,
    pack_revision_ref: Option<String>,
    pack_label: Option<String>,
    source_locale: Option<String>,
    requested_locale: Option<String>,
    effective_locale: Option<String>,
    source_truth: Option<DocsPackSourceTruth>,
    #[serde(default)]
    nodes: Vec<DocsPackNodeManifest>,
    #[serde(default)]
    node: Option<DocsPackNodeManifest>,
}

#[derive(Debug, Clone, Deserialize)]
struct DocsPackNodeManifest {
    docs_node_id: Option<String>,
    doc_kind: Option<DocsNodeKind>,
    source_class: Option<CitationSourceClass>,
    scope_class: Option<DocsScopeClass>,
    version_or_revision_ref: Option<String>,
    version_match_state: Option<VersionMatchState>,
    freshness_class: Option<DocsFreshnessClass>,
    locality_class: Option<CitationLocalityClass>,
    source_locale: Option<String>,
    requested_locale: Option<String>,
    effective_locale: Option<String>,
    locale_overlay_state: Option<LocaleOverlayState>,
    source_language_fallback_ref: Option<String>,
    citation_availability: Option<CitationAnchorAvailability>,
    #[serde(default)]
    citation_anchor_refs: Vec<String>,
    exact_reopen_ref: Option<String>,
    hidden_or_omitted_note: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    source_ref: Option<String>,
    body_markdown: Option<String>,
}

fn parse_manifest(raw: &str) -> Result<DocsPackManifest, DocsPackLoadError> {
    serde_yaml::from_str(raw).map_err(|error| DocsPackLoadError::ParseYaml {
        message: error.to_string(),
    })
}

fn build_pack(
    mut manifest: DocsPackManifest,
    markdown_body: Option<String>,
) -> Result<DocsPack, DocsPackLoadError> {
    let schema_version = manifest
        .schema_version
        .unwrap_or(DOCS_PACK_ALPHA_SCHEMA_VERSION);
    if schema_version != DOCS_PACK_ALPHA_SCHEMA_VERSION {
        return Err(DocsPackLoadError::UnsupportedSchemaVersion { schema_version });
    }

    let source_truth = manifest
        .source_truth
        .take()
        .ok_or(DocsPackLoadError::MissingSourceTruth)?;
    validate_source_truth(&source_truth)?;

    let pack_id = required(manifest.pack_id, "pack_id")?;
    let pack_revision_ref = required(manifest.pack_revision_ref, "pack_revision_ref")?;
    let pack_label = required(manifest.pack_label, "pack_label")?;
    let source_locale = required(manifest.source_locale, "source_locale")?;
    let requested_locale = manifest
        .requested_locale
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| source_locale.clone());
    let effective_locale = manifest
        .effective_locale
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| requested_locale.clone());

    let mut raw_nodes = manifest.nodes;
    if let Some(mut node) = manifest.node {
        if node.body_markdown.as_deref().map_or(true, str::is_empty) {
            node.body_markdown = markdown_body;
        }
        raw_nodes.push(node);
    }
    if raw_nodes.is_empty() {
        return Err(DocsPackLoadError::EmptyNodes);
    }

    let nodes = raw_nodes
        .into_iter()
        .map(|node| {
            build_node(
                node,
                &pack_id,
                &pack_revision_ref,
                &source_locale,
                &requested_locale,
                &effective_locale,
                &source_truth,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DocsPack {
        record_kind: DOCS_PACK_ALPHA_RECORD_KIND.to_owned(),
        schema_version,
        pack_id,
        pack_revision_ref,
        pack_label,
        source_locale,
        requested_locale,
        effective_locale,
        source_truth,
        nodes,
    })
}

fn validate_source_truth(source_truth: &DocsPackSourceTruth) -> Result<(), DocsPackLoadError> {
    if source_truth.version_or_revision_ref.trim().is_empty() {
        return Err(DocsPackLoadError::MissingField {
            field: "source_truth.version_or_revision_ref",
        });
    }
    if source_truth.running_build_identity_ref.trim().is_empty() {
        return Err(DocsPackLoadError::MissingField {
            field: "source_truth.running_build_identity_ref",
        });
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_node(
    node: DocsPackNodeManifest,
    pack_id: &str,
    pack_revision_ref: &str,
    source_locale: &str,
    requested_locale: &str,
    effective_locale: &str,
    source_truth: &DocsPackSourceTruth,
) -> Result<DocsPackNode, DocsPackLoadError> {
    let docs_node_id = required(node.docs_node_id, "nodes[].docs_node_id")?;
    let title = required(node.title, "nodes[].title")?;
    let exact_reopen_ref = node
        .exact_reopen_ref
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| default_reopen_ref(pack_id, &docs_node_id));
    let locale_overlay_state = node
        .locale_overlay_state
        .unwrap_or(LocaleOverlayState::SourceLanguageOriginal);
    let source_language_fallback_ref = node
        .source_language_fallback_ref
        .or_else(|| source_truth.source_language_fallback_ref.clone());
    let hidden_or_omitted_note = node
        .hidden_or_omitted_note
        .or_else(|| source_truth.hidden_or_omitted_note.clone());

    let docs_node = DocsNodeIdentity::new(DocsNodeIdentityInput {
        docs_node_id: docs_node_id.clone(),
        doc_kind: node.doc_kind.unwrap_or(DocsNodeKind::ProductHelp),
        source_class: node.source_class.unwrap_or(source_truth.source_class),
        scope_class: node.scope_class.unwrap_or(source_truth.scope_class),
        source_pack_ref: pack_id.to_owned(),
        source_pack_revision_ref: pack_revision_ref.to_owned(),
        version_or_revision_ref: node
            .version_or_revision_ref
            .unwrap_or_else(|| source_truth.version_or_revision_ref.clone()),
        version_match_state: node
            .version_match_state
            .unwrap_or(source_truth.version_match_state),
        freshness_class: node.freshness_class.unwrap_or(source_truth.freshness_class),
        locality_class: node.locality_class.unwrap_or(source_truth.locality_class),
        source_locale: node
            .source_locale
            .unwrap_or_else(|| source_locale.to_owned()),
        requested_locale: node
            .requested_locale
            .unwrap_or_else(|| requested_locale.to_owned()),
        effective_locale: node
            .effective_locale
            .unwrap_or_else(|| effective_locale.to_owned()),
        locale_overlay_state,
        source_language_fallback_ref,
        citation_availability: node
            .citation_availability
            .unwrap_or(source_truth.citation_availability),
        citation_anchor_refs: node.citation_anchor_refs,
        exact_reopen_ref,
        hidden_or_omitted_note,
    });
    let violations = docs_node.validate();
    if !violations.is_empty() {
        return Err(DocsPackLoadError::InvalidDocsNode {
            docs_node_id,
            violations,
        });
    }

    Ok(DocsPackNode {
        docs_node,
        title,
        summary: node.summary,
        source_ref: node.source_ref,
        body_markdown: node.body_markdown.unwrap_or_default(),
    })
}

fn required(value: Option<String>, field: &'static str) -> Result<String, DocsPackLoadError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or(DocsPackLoadError::MissingField { field })
}

fn split_markdown_front_matter(raw: &str) -> Result<(&str, &str), DocsPackLoadError> {
    let raw = raw
        .strip_prefix("---\n")
        .ok_or(DocsPackLoadError::MissingMarkdownFrontMatter)?;
    let Some((front_matter, body)) = raw.split_once("\n---\n") else {
        return Err(DocsPackLoadError::MissingMarkdownFrontMatter);
    };
    Ok((front_matter, body))
}

fn default_reopen_ref(pack_id: &str, docs_node_id: &str) -> String {
    format!(
        "id:docs-reopen:{}:{}",
        sanitize_id(pack_id),
        sanitize_id(docs_node_id)
    )
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}
