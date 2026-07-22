//! Docs-pack loading for citation-aware docs/help content.
//!
//! This module turns checked-in YAML manifests or Markdown files with YAML
//! front matter into [`DocsNodeIdentity`] records. It intentionally consumes
//! the citation vocabulary from [`crate::citations`] instead of defining
//! parallel source, version, freshness, locality, or anchor tokens.

use std::error::Error;
use std::fmt;
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::Path;

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

const MAX_DOCS_PACK_BYTES: u64 = 8 * 1024 * 1024;
const MAX_DOCS_PACK_NODES: usize = 4_096;
const MAX_DOCS_PACK_ANCHORS_PER_NODE: usize = 512;
const MAX_DOCS_PACK_IDENTITY_BYTES: usize = 4 * 1024;
const MAX_DOCS_PACK_LABEL_BYTES: usize = 16 * 1024;
const MAX_DOCS_PACK_NODE_BODY_BYTES: usize = 4 * 1024 * 1024;

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
        let format = match path.extension().and_then(|extension| extension.to_str()) {
            Some("yaml" | "yml") => DocsPackFileFormat::Yaml,
            Some("md" | "markdown") => DocsPackFileFormat::Markdown,
            _ => return Err(DocsPackLoadError::UnsupportedExtension),
        };
        let raw = read_bounded_docs_pack(path)?;
        match format {
            DocsPackFileFormat::Yaml => Self::from_yaml_str(&raw),
            DocsPackFileFormat::Markdown => Self::from_markdown_str(&raw),
        }
    }

    /// Loads a docs pack from a YAML manifest string.
    ///
    /// # Errors
    ///
    /// Returns [`DocsPackLoadError`] when parsing or semantic validation fails.
    pub fn from_yaml_str(raw: &str) -> Result<Self, DocsPackLoadError> {
        validate_raw_size(raw)?;
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
        validate_raw_size(raw)?;
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
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// The path extension is not a supported docs-pack format.
    UnsupportedExtension,
    /// The selected object was not a stable regular file.
    UnsafeFileType,
    /// The selected file changed identity or metadata while it was read.
    FileChangedDuringRead,
    /// Docs-pack input was not valid UTF-8.
    InvalidUtf8,
    /// A bounded docs-pack resource exceeded its declared limit.
    ResourceLimitExceeded {
        /// Stable field or resource class that exceeded the limit.
        resource: &'static str,
        /// Maximum admitted size or count.
        limit: usize,
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
            Self::Io { .. } => write!(formatter, "failed to read docs pack"),
            Self::UnsupportedExtension => write!(formatter, "unsupported docs pack extension"),
            Self::UnsafeFileType => {
                write!(formatter, "docs pack is not a stable regular file")
            }
            Self::FileChangedDuringRead => {
                write!(formatter, "docs pack changed while it was being read")
            }
            Self::InvalidUtf8 => write!(formatter, "docs pack is not valid UTF-8"),
            Self::ResourceLimitExceeded { resource, limit } => write!(
                formatter,
                "docs pack {resource} exceeds the configured limit of {limit}"
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
            Self::InvalidDocsNode { violations, .. } => write!(
                formatter,
                "docs node failed citation validation with {} violation(s)",
                violations.len()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocsPackFileFormat {
    Yaml,
    Markdown,
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

fn validate_raw_size(raw: &str) -> Result<(), DocsPackLoadError> {
    if raw.len() as u64 > MAX_DOCS_PACK_BYTES {
        return Err(DocsPackLoadError::ResourceLimitExceeded {
            resource: "input bytes",
            limit: MAX_DOCS_PACK_BYTES as usize,
        });
    }
    Ok(())
}

fn read_bounded_docs_pack(path: &Path) -> Result<String, DocsPackLoadError> {
    let before = fs::symlink_metadata(path).map_err(|source| DocsPackLoadError::Io { source })?;
    if before.file_type().is_symlink() || !before.is_file() {
        return Err(DocsPackLoadError::UnsafeFileType);
    }
    if before.len() > MAX_DOCS_PACK_BYTES {
        return Err(DocsPackLoadError::ResourceLimitExceeded {
            resource: "input bytes",
            limit: MAX_DOCS_PACK_BYTES as usize,
        });
    }

    let mut file = File::open(path).map_err(|source| DocsPackLoadError::Io { source })?;
    let opened = file
        .metadata()
        .map_err(|source| DocsPackLoadError::Io { source })?;
    if !stable_file_metadata(&before, &opened) {
        return Err(DocsPackLoadError::FileChangedDuringRead);
    }

    let mut bytes = Vec::with_capacity(opened.len() as usize);
    (&mut file)
        .take(MAX_DOCS_PACK_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|source| DocsPackLoadError::Io { source })?;
    if bytes.len() as u64 > MAX_DOCS_PACK_BYTES {
        return Err(DocsPackLoadError::ResourceLimitExceeded {
            resource: "input bytes",
            limit: MAX_DOCS_PACK_BYTES as usize,
        });
    }

    let descriptor_after = file
        .metadata()
        .map_err(|source| DocsPackLoadError::Io { source })?;
    let path_after =
        fs::symlink_metadata(path).map_err(|source| DocsPackLoadError::Io { source })?;
    if !stable_file_metadata(&opened, &descriptor_after)
        || !stable_file_metadata(&descriptor_after, &path_after)
    {
        return Err(DocsPackLoadError::FileChangedDuringRead);
    }

    String::from_utf8(bytes).map_err(|_| DocsPackLoadError::InvalidUtf8)
}

#[cfg(unix)]
fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev() && left.ino() == right.ino()
}

#[cfg(not(unix))]
fn same_file_identity(_left: &Metadata, _right: &Metadata) -> bool {
    true
}

fn stable_file_metadata(left: &Metadata, right: &Metadata) -> bool {
    left.is_file()
        && right.is_file()
        && !left.file_type().is_symlink()
        && !right.file_type().is_symlink()
        && left.len() == right.len()
        && matches!(
            (left.modified(), right.modified()),
            (Ok(left_modified), Ok(right_modified)) if left_modified == right_modified
        )
        && same_file_identity(left, right)
}

fn parse_manifest(raw: &str) -> Result<DocsPackManifest, DocsPackLoadError> {
    serde_yaml::from_str(raw).map_err(|error| DocsPackLoadError::ParseYaml {
        message: error.location().map_or_else(
            || "invalid YAML structure".to_string(),
            |location| {
                format!(
                    "invalid structure at line {}, column {}",
                    location.line(),
                    location.column()
                )
            },
        ),
    })
}

fn build_pack(
    mut manifest: DocsPackManifest,
    markdown_body: Option<String>,
) -> Result<DocsPack, DocsPackLoadError> {
    validate_manifest_limits(&manifest, markdown_body.as_deref())?;
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

fn validate_manifest_limits(
    manifest: &DocsPackManifest,
    markdown_body: Option<&str>,
) -> Result<(), DocsPackLoadError> {
    validate_optional_text(
        manifest.pack_id.as_deref(),
        "pack_id bytes",
        MAX_DOCS_PACK_IDENTITY_BYTES,
    )?;
    validate_optional_text(
        manifest.pack_revision_ref.as_deref(),
        "pack_revision_ref bytes",
        MAX_DOCS_PACK_IDENTITY_BYTES,
    )?;
    validate_optional_text(
        manifest.pack_label.as_deref(),
        "pack_label bytes",
        MAX_DOCS_PACK_LABEL_BYTES,
    )?;
    for (value, resource) in [
        (manifest.source_locale.as_deref(), "source_locale bytes"),
        (
            manifest.requested_locale.as_deref(),
            "requested_locale bytes",
        ),
        (
            manifest.effective_locale.as_deref(),
            "effective_locale bytes",
        ),
    ] {
        validate_optional_text(value, resource, MAX_DOCS_PACK_IDENTITY_BYTES)?;
    }

    if let Some(source_truth) = manifest.source_truth.as_ref() {
        for (value, resource, limit) in [
            (
                Some(source_truth.version_or_revision_ref.as_str()),
                "source_truth.version_or_revision_ref bytes",
                MAX_DOCS_PACK_IDENTITY_BYTES,
            ),
            (
                Some(source_truth.running_build_identity_ref.as_str()),
                "source_truth.running_build_identity_ref bytes",
                MAX_DOCS_PACK_IDENTITY_BYTES,
            ),
            (
                source_truth.source_build_at.as_deref(),
                "source_truth.source_build_at bytes",
                MAX_DOCS_PACK_IDENTITY_BYTES,
            ),
            (
                source_truth.snapshot_age_label.as_deref(),
                "source_truth.snapshot_age_label bytes",
                MAX_DOCS_PACK_LABEL_BYTES,
            ),
            (
                source_truth.help_status_badge_ref.as_deref(),
                "source_truth.help_status_badge_ref bytes",
                MAX_DOCS_PACK_IDENTITY_BYTES,
            ),
            (
                source_truth.browser_handoff_packet_ref.as_deref(),
                "source_truth.browser_handoff_packet_ref bytes",
                MAX_DOCS_PACK_IDENTITY_BYTES,
            ),
            (
                source_truth.source_language_fallback_ref.as_deref(),
                "source_truth.source_language_fallback_ref bytes",
                MAX_DOCS_PACK_IDENTITY_BYTES,
            ),
            (
                source_truth.hidden_or_omitted_note.as_deref(),
                "source_truth.hidden_or_omitted_note bytes",
                MAX_DOCS_PACK_LABEL_BYTES,
            ),
        ] {
            validate_optional_text(value, resource, limit)?;
        }
    }

    let node_count = manifest
        .nodes
        .len()
        .saturating_add(usize::from(manifest.node.is_some()));
    if node_count > MAX_DOCS_PACK_NODES {
        return Err(DocsPackLoadError::ResourceLimitExceeded {
            resource: "node count",
            limit: MAX_DOCS_PACK_NODES,
        });
    }
    for node in manifest.nodes.iter().chain(manifest.node.iter()) {
        validate_node_limits(node)?;
    }
    if manifest
        .node
        .as_ref()
        .is_some_and(|node| node.body_markdown.as_deref().map_or(true, str::is_empty))
    {
        validate_optional_text(
            markdown_body,
            "Markdown body bytes",
            MAX_DOCS_PACK_NODE_BODY_BYTES,
        )?;
    }
    Ok(())
}

fn validate_node_limits(node: &DocsPackNodeManifest) -> Result<(), DocsPackLoadError> {
    for (value, resource) in [
        (node.docs_node_id.as_deref(), "nodes[].docs_node_id bytes"),
        (
            node.version_or_revision_ref.as_deref(),
            "nodes[].version_or_revision_ref bytes",
        ),
        (node.source_locale.as_deref(), "nodes[].source_locale bytes"),
        (
            node.requested_locale.as_deref(),
            "nodes[].requested_locale bytes",
        ),
        (
            node.effective_locale.as_deref(),
            "nodes[].effective_locale bytes",
        ),
        (
            node.source_language_fallback_ref.as_deref(),
            "nodes[].source_language_fallback_ref bytes",
        ),
        (
            node.exact_reopen_ref.as_deref(),
            "nodes[].exact_reopen_ref bytes",
        ),
        (node.source_ref.as_deref(), "nodes[].source_ref bytes"),
    ] {
        validate_optional_text(value, resource, MAX_DOCS_PACK_IDENTITY_BYTES)?;
    }
    for (value, resource) in [
        (
            node.hidden_or_omitted_note.as_deref(),
            "nodes[].hidden_or_omitted_note bytes",
        ),
        (node.title.as_deref(), "nodes[].title bytes"),
        (node.summary.as_deref(), "nodes[].summary bytes"),
    ] {
        validate_optional_text(value, resource, MAX_DOCS_PACK_LABEL_BYTES)?;
    }
    validate_optional_text(
        node.body_markdown.as_deref(),
        "nodes[].body_markdown bytes",
        MAX_DOCS_PACK_NODE_BODY_BYTES,
    )?;
    if node.citation_anchor_refs.len() > MAX_DOCS_PACK_ANCHORS_PER_NODE {
        return Err(DocsPackLoadError::ResourceLimitExceeded {
            resource: "nodes[].citation_anchor_refs count",
            limit: MAX_DOCS_PACK_ANCHORS_PER_NODE,
        });
    }
    for anchor in &node.citation_anchor_refs {
        validate_optional_text(
            Some(anchor),
            "nodes[].citation_anchor_refs[] bytes",
            MAX_DOCS_PACK_IDENTITY_BYTES,
        )?;
    }
    Ok(())
}

fn validate_optional_text(
    value: Option<&str>,
    resource: &'static str,
    limit: usize,
) -> Result<(), DocsPackLoadError> {
    if value.is_some_and(|value| value.len() > limit) {
        return Err(DocsPackLoadError::ResourceLimitExceeded { resource, limit });
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn temp_path(extension: &str) -> PathBuf {
        let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "aureline-docs-pack-loader-{}-{sequence}.{extension}",
            std::process::id()
        ))
    }

    #[test]
    fn unsupported_extensions_are_rejected_before_file_access() {
        let path = temp_path("private-unsupported");

        let error = DocsPack::load_path(&path).expect_err("extension must be rejected");

        assert!(matches!(error, DocsPackLoadError::UnsupportedExtension));
    }

    #[test]
    fn file_errors_do_not_disclose_the_selected_path() {
        let path = temp_path("yaml");
        let path_text = path.to_string_lossy().into_owned();

        let error = DocsPack::load_path(&path).expect_err("missing file must fail");

        assert!(matches!(error, DocsPackLoadError::Io { .. }));
        assert!(!error.to_string().contains(&path_text));
    }

    #[test]
    fn oversized_files_are_rejected_before_allocation() {
        let path = temp_path("yaml");
        let file = File::create(&path).expect("create sparse test file");
        file.set_len(MAX_DOCS_PACK_BYTES + 1)
            .expect("extend sparse test file");

        let error = DocsPack::load_path(&path).expect_err("oversized pack must fail");
        fs::remove_file(&path).expect("remove sparse test file");

        assert!(matches!(
            error,
            DocsPackLoadError::ResourceLimitExceeded {
                resource: "input bytes",
                ..
            }
        ));
    }

    #[test]
    fn parser_errors_do_not_echo_untrusted_scalar_values() {
        let private_value = "private-customer-token-must-not-escape";
        let raw = format!("schema_version: 1\nsource_truth:\n  source_class: {private_value}\n");

        let error = DocsPack::from_yaml_str(&raw).expect_err("invalid enum must fail");

        assert!(matches!(error, DocsPackLoadError::ParseYaml { .. }));
        assert!(!error.to_string().contains(private_value));
    }

    #[test]
    fn in_memory_inputs_obey_the_same_byte_limit() {
        let raw = "x".repeat(MAX_DOCS_PACK_BYTES as usize + 1);

        let error = DocsPack::from_yaml_str(&raw).expect_err("oversized input must fail");

        assert!(matches!(
            error,
            DocsPackLoadError::ResourceLimitExceeded {
                resource: "input bytes",
                ..
            }
        ));
    }

    #[cfg(unix)]
    #[test]
    fn docs_pack_files_must_not_be_symbolic_links() {
        use std::os::unix::fs::symlink;

        let target = temp_path("target");
        let link = temp_path("yaml");
        fs::write(&target, b"schema_version: 1\n").expect("write symlink target");
        symlink(&target, &link).expect("create symlink");

        let error = DocsPack::load_path(&link).expect_err("symlink must fail closed");
        fs::remove_file(&link).expect("remove symlink");
        fs::remove_file(&target).expect("remove target");

        assert!(matches!(error, DocsPackLoadError::UnsafeFileType));
    }
}
