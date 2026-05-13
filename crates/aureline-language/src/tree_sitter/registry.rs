use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::records::{
    BudgetPolicyClass, DerivedCueClass, ExportPolicyClass, GrammarResolution,
    GrammarResolutionStateClass, GrammarSourceClass, ParserSubstrateClass,
};

/// Integer schema version for grammar registry records.
pub const TREE_SITTER_GRAMMAR_REGISTRY_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum GrammarKind {
    TypeScript,
    Tsx,
    JavaScript,
    Jsx,
    Html,
    Css,
    Json,
    Yaml,
    Markdown,
    Python,
}

impl GrammarKind {
    fn language(self) -> ::tree_sitter::Language {
        match self {
            Self::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Self::JavaScript | Self::Jsx => tree_sitter_javascript::LANGUAGE.into(),
            Self::Html => tree_sitter_html::LANGUAGE.into(),
            Self::Css => tree_sitter_css::LANGUAGE.into(),
            Self::Json => tree_sitter_json::LANGUAGE.into(),
            Self::Yaml => tree_sitter_yaml::language(),
            Self::Markdown => tree_sitter_md::LANGUAGE.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
        }
    }
}

/// Error returned when the grammar registry cannot resolve or load a grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarRegistryError {
    /// Two entries claimed the same language alias.
    DuplicateLanguageAlias(String),
    /// Two entries claimed the same file extension.
    DuplicateFileExtension(String),
    /// No descriptor exists for the requested language id.
    UnknownLanguageId(String),
    /// No descriptor exists for the requested file extension.
    UnknownFileExtension(String),
}

impl fmt::Display for GrammarRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLanguageAlias(alias) => {
                write!(f, "duplicate tree-sitter language alias: {alias}")
            }
            Self::DuplicateFileExtension(extension) => {
                write!(f, "duplicate tree-sitter file extension: {extension}")
            }
            Self::UnknownLanguageId(language_id) => {
                write!(f, "no tree-sitter grammar registered for {language_id}")
            }
            Self::UnknownFileExtension(extension) => {
                write!(
                    f,
                    "no tree-sitter grammar registered for extension {extension}"
                )
            }
        }
    }
}

impl std::error::Error for GrammarRegistryError {}

/// Static metadata for one bundled Tree-sitter grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarDescriptor {
    /// Stable grammar family id.
    pub grammar_id: &'static str,
    /// Canonical language id served by the grammar.
    pub language_id: &'static str,
    /// Plain-language name for support and debug displays.
    pub display_name: &'static str,
    /// Crate name that supplies the grammar.
    pub crate_name: &'static str,
    /// Crate version pinned by this registry.
    pub crate_version: &'static str,
    /// Source class for the grammar package.
    pub grammar_source_class: GrammarSourceClass,
    /// Query-pack identity bundled with the grammar entry.
    pub query_pack_ref: &'static str,
    /// Export-safe artifact hash or package identity reference.
    pub artifact_hash_ref: &'static str,
    /// Export-safe signature, checksum, or provenance reference.
    pub signature_ref: &'static str,
    /// Upstream grammar project reference.
    pub upstream_ref: &'static str,
    /// Local patch reference, or `not_applicable`.
    pub local_patch_ref: &'static str,
    /// File extensions resolved to this grammar.
    pub file_extensions: &'static [&'static str],
    /// Language aliases resolved to this grammar.
    pub language_aliases: &'static [&'static str],
    /// Parse-derived cues this alpha registry admits for the grammar.
    pub supported_cues: &'static [DerivedCueClass],
    /// Default budget policy for foreground parses.
    pub default_budget_policy: BudgetPolicyClass,
    kind: GrammarKind,
}

impl GrammarDescriptor {
    /// Returns the exact grammar version reference for parse-session records.
    pub fn grammar_version_ref(&self) -> String {
        format!("grammar-version:{}:{}", self.crate_name, self.crate_version)
    }

    /// Returns the Tree-sitter ABI reference reported by the loaded grammar.
    pub fn grammar_abi_ref(&self) -> String {
        format!("tree-sitter-abi:{}", self.load_language().version())
    }

    /// Loads the Tree-sitter language object for this descriptor.
    pub fn load_language(&self) -> ::tree_sitter::Language {
        self.kind.language()
    }

    /// Projects this descriptor into a parse-session grammar-resolution record.
    pub fn grammar_resolution(&self, scope_ref: impl Into<String>) -> GrammarResolution {
        GrammarResolution {
            grammar_id: self.grammar_id.into(),
            language_id: self.language_id.into(),
            grammar_source_class: self.grammar_source_class,
            grammar_resolution_state_class: GrammarResolutionStateClass::ResolvedCurrent,
            grammar_version_ref: self.grammar_version_ref(),
            grammar_abi_ref: self.grammar_abi_ref(),
            query_pack_ref: self.query_pack_ref.into(),
            artifact_hash_ref: self.artifact_hash_ref.into(),
            signature_ref: self.signature_ref.into(),
            upstream_ref: self.upstream_ref.into(),
            local_patch_ref: self.local_patch_ref.into(),
            scope_ref: scope_ref.into(),
            summary: format!(
                "Bundled {} grammar resolved through the shared Tree-sitter registry.",
                self.display_name
            ),
        }
    }

    /// Projects this descriptor into a serializable grammar registry entry.
    pub fn registry_entry(&self) -> GrammarRegistryEntry {
        let language = self.load_language();
        GrammarRegistryEntry {
            grammar_id: self.grammar_id.into(),
            language_id: self.language_id.into(),
            display_name: self.display_name.into(),
            parser_substrate_class: ParserSubstrateClass::TreeSitter,
            grammar_source_class: self.grammar_source_class,
            grammar_resolution_state_class: GrammarResolutionStateClass::ResolvedCurrent,
            tree_sitter_crate: self.crate_name.into(),
            crate_version: self.crate_version.into(),
            grammar_version_ref: self.grammar_version_ref(),
            grammar_abi_ref: format!("tree-sitter-abi:{}", language.version()),
            query_pack_ref: self.query_pack_ref.into(),
            artifact_hash_ref: self.artifact_hash_ref.into(),
            signature_ref: self.signature_ref.into(),
            upstream_ref: self.upstream_ref.into(),
            local_patch_ref: self.local_patch_ref.into(),
            file_extensions: self
                .file_extensions
                .iter()
                .map(|extension| (*extension).to_owned())
                .collect(),
            language_aliases: self
                .language_aliases
                .iter()
                .map(|alias| (*alias).to_owned())
                .collect(),
            supported_cue_classes: self.supported_cues.to_vec(),
            default_budget_policy_class: self.default_budget_policy,
            runtime_loader: "rust_tree_sitter_static_language_fn".into(),
            node_kind_count: language.node_kind_count(),
            export_policy_class: ExportPolicyClass::MetadataSafeDefault,
            fallback_label: "Use plain-text editing with syntax-derived cues disabled if this grammar cannot load.".into(),
        }
    }
}

/// Serializable registry row for one launch-language grammar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrammarRegistryEntry {
    /// Stable grammar family id.
    pub grammar_id: String,
    /// Canonical language id served by the grammar.
    pub language_id: String,
    /// Plain-language grammar name for support displays.
    pub display_name: String,
    /// Parser substrate for this entry.
    pub parser_substrate_class: ParserSubstrateClass,
    /// Source class for the grammar package.
    pub grammar_source_class: GrammarSourceClass,
    /// Resolution state represented by this registry row.
    pub grammar_resolution_state_class: GrammarResolutionStateClass,
    /// Rust crate that provides the grammar.
    pub tree_sitter_crate: String,
    /// Crate version pinned by this registry.
    pub crate_version: String,
    /// Exact grammar version reference.
    pub grammar_version_ref: String,
    /// Tree-sitter ABI reference reported by the grammar.
    pub grammar_abi_ref: String,
    /// Query-pack identity bundled with this grammar.
    pub query_pack_ref: String,
    /// Export-safe artifact hash or package identity reference.
    pub artifact_hash_ref: String,
    /// Export-safe signature, checksum, or provenance reference.
    pub signature_ref: String,
    /// Upstream grammar project reference.
    pub upstream_ref: String,
    /// Local patch reference, or `not_applicable`.
    pub local_patch_ref: String,
    /// File extensions resolved to this grammar.
    pub file_extensions: Vec<String>,
    /// Language aliases resolved to this grammar.
    pub language_aliases: Vec<String>,
    /// Parse-derived cue classes admitted for this grammar.
    pub supported_cue_classes: Vec<DerivedCueClass>,
    /// Default parser budget policy.
    pub default_budget_policy_class: BudgetPolicyClass,
    /// Loader strategy used by this runtime.
    pub runtime_loader: String,
    /// Number of node kinds reported by the grammar.
    pub node_kind_count: usize,
    /// Export policy for registry metadata.
    pub export_policy_class: ExportPolicyClass,
    /// Explicit fallback label for degraded loading.
    pub fallback_label: String,
}

/// Serializable grammar registry artifact consumed by runtime and support paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrammarRegistryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for registry records.
    pub grammar_registry_schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Registry status.
    pub status: String,
    /// Export-safe creation or capture timestamp.
    pub captured_at: String,
    /// Grammar rows registered by this artifact.
    pub entries: Vec<GrammarRegistryEntry>,
    /// Reviewer-facing registry summary.
    pub export_safe_summary: String,
}

impl GrammarRegistryRecord {
    /// Stable record-kind tag carried in serialized registry records.
    pub const RECORD_KIND: &'static str = "tree_sitter_grammar_registry_record";
}

/// Shared launch-language Tree-sitter grammar registry.
#[derive(Debug, Clone)]
pub struct TreeSitterGrammarRegistry {
    descriptors: Vec<GrammarDescriptor>,
    by_language_alias: BTreeMap<String, usize>,
    by_file_extension: BTreeMap<String, usize>,
}

impl TreeSitterGrammarRegistry {
    /// Builds a registry from descriptors and rejects ambiguous aliases.
    ///
    /// # Errors
    ///
    /// Returns an error when two descriptors claim the same language alias or
    /// file extension.
    pub fn new(descriptors: Vec<GrammarDescriptor>) -> Result<Self, GrammarRegistryError> {
        let mut by_language_alias = BTreeMap::new();
        let mut by_file_extension = BTreeMap::new();

        for (index, descriptor) in descriptors.iter().enumerate() {
            let mut aliases = Vec::with_capacity(descriptor.language_aliases.len() + 2);
            aliases.push(descriptor.language_id);
            aliases.push(descriptor.language_id.trim_start_matches("language:"));
            aliases.extend(descriptor.language_aliases.iter().copied());

            for alias in aliases {
                let normalized = normalize_lookup_key(alias);
                if by_language_alias
                    .insert(normalized.clone(), index)
                    .is_some_and(|previous| previous != index)
                {
                    return Err(GrammarRegistryError::DuplicateLanguageAlias(normalized));
                }
            }

            for extension in descriptor.file_extensions {
                let normalized = normalize_extension(extension);
                if by_file_extension
                    .insert(normalized.clone(), index)
                    .is_some()
                {
                    return Err(GrammarRegistryError::DuplicateFileExtension(normalized));
                }
            }
        }

        Ok(Self {
            descriptors,
            by_language_alias,
            by_file_extension,
        })
    }

    /// Returns all grammar descriptors in stable registry order.
    pub fn descriptors(&self) -> &[GrammarDescriptor] {
        &self.descriptors
    }

    /// Resolves a grammar by language id or alias.
    pub fn resolve_language_id(&self, language_id: &str) -> Option<&GrammarDescriptor> {
        self.by_language_alias
            .get(&normalize_lookup_key(language_id))
            .and_then(|index| self.descriptors.get(*index))
    }

    /// Resolves a grammar by file extension.
    pub fn resolve_file_extension(&self, extension: &str) -> Option<&GrammarDescriptor> {
        self.by_file_extension
            .get(&normalize_extension(extension))
            .and_then(|index| self.descriptors.get(*index))
    }

    /// Resolves a grammar by language id and returns a typed error on miss.
    ///
    /// # Errors
    ///
    /// Returns [`GrammarRegistryError::UnknownLanguageId`] when the registry
    /// has no descriptor for the requested id.
    pub fn require_language_id(
        &self,
        language_id: &str,
    ) -> Result<&GrammarDescriptor, GrammarRegistryError> {
        self.resolve_language_id(language_id)
            .ok_or_else(|| GrammarRegistryError::UnknownLanguageId(language_id.to_owned()))
    }

    /// Resolves a grammar by extension and returns a typed error on miss.
    ///
    /// # Errors
    ///
    /// Returns [`GrammarRegistryError::UnknownFileExtension`] when the registry
    /// has no descriptor for the extension.
    pub fn require_file_extension(
        &self,
        extension: &str,
    ) -> Result<&GrammarDescriptor, GrammarRegistryError> {
        self.resolve_file_extension(extension)
            .ok_or_else(|| GrammarRegistryError::UnknownFileExtension(extension.to_owned()))
    }

    /// Builds a serializable registry record with live ABI and node counts.
    pub fn registry_record(&self, captured_at: impl Into<String>) -> GrammarRegistryRecord {
        GrammarRegistryRecord {
            record_kind: GrammarRegistryRecord::RECORD_KIND.into(),
            grammar_registry_schema_version: TREE_SITTER_GRAMMAR_REGISTRY_SCHEMA_VERSION,
            registry_id: "tree-sitter:launch-language-registry".into(),
            status: "seeded_launch_alpha".into(),
            captured_at: captured_at.into(),
            entries: self
                .descriptors
                .iter()
                .map(GrammarDescriptor::registry_entry)
                .collect(),
            export_safe_summary:
                "Launch-language Tree-sitter grammars resolve through one runtime registry.".into(),
        }
    }
}

/// Returns the curated launch-language grammar registry.
pub fn default_launch_grammar_registry() -> TreeSitterGrammarRegistry {
    TreeSitterGrammarRegistry::new(launch_descriptors())
        .expect("launch grammar registry descriptors must be unambiguous")
}

fn normalize_lookup_key(value: &str) -> String {
    value
        .trim()
        .trim_start_matches("language:")
        .to_ascii_lowercase()
        .replace('_', "-")
}

fn normalize_extension(extension: &str) -> String {
    extension
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase()
}

const CORE_CUES: &[DerivedCueClass] = &[
    DerivedCueClass::SyntaxHighlighting,
    DerivedCueClass::Folds,
    DerivedCueClass::IndentGuides,
    DerivedCueClass::StructuralSelection,
    DerivedCueClass::Breadcrumbs,
    DerivedCueClass::LocalSymbols,
    DerivedCueClass::BracketMatching,
    DerivedCueClass::MinimapMarkers,
    DerivedCueClass::SupportExport,
];

fn launch_descriptors() -> Vec<GrammarDescriptor> {
    vec![
        GrammarDescriptor {
            grammar_id: "grammar:typescript",
            language_id: "language:typescript",
            display_name: "TypeScript",
            crate_name: "tree-sitter-typescript",
            crate_version: "0.23.2",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:typescript:tree-sitter-typescript:0.23.2",
            artifact_hash_ref: "artifact:crate:tree-sitter-typescript:0.23.2",
            signature_ref: "provenance:crates-io:tree-sitter-typescript:0.23.2",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-typescript:0.23.2",
            local_patch_ref: "not_applicable",
            file_extensions: &["ts", "mts", "cts"],
            language_aliases: &["typescript", "ts"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::TypeScript,
        },
        GrammarDescriptor {
            grammar_id: "grammar:tsx",
            language_id: "language:tsx",
            display_name: "TSX",
            crate_name: "tree-sitter-typescript",
            crate_version: "0.23.2",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:tsx:tree-sitter-typescript:0.23.2",
            artifact_hash_ref: "artifact:crate:tree-sitter-typescript:0.23.2",
            signature_ref: "provenance:crates-io:tree-sitter-typescript:0.23.2",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-typescript:0.23.2",
            local_patch_ref: "not_applicable",
            file_extensions: &["tsx"],
            language_aliases: &["tsx"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Tsx,
        },
        GrammarDescriptor {
            grammar_id: "grammar:javascript",
            language_id: "language:javascript",
            display_name: "JavaScript",
            crate_name: "tree-sitter-javascript",
            crate_version: "0.23.1",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:javascript:tree-sitter-javascript:0.23.1",
            artifact_hash_ref: "artifact:crate:tree-sitter-javascript:0.23.1",
            signature_ref: "provenance:crates-io:tree-sitter-javascript:0.23.1",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-javascript:0.23.1",
            local_patch_ref: "not_applicable",
            file_extensions: &["js", "mjs", "cjs"],
            language_aliases: &["javascript", "js"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::JavaScript,
        },
        GrammarDescriptor {
            grammar_id: "grammar:jsx",
            language_id: "language:jsx",
            display_name: "JSX",
            crate_name: "tree-sitter-javascript",
            crate_version: "0.23.1",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:jsx:tree-sitter-javascript:0.23.1",
            artifact_hash_ref: "artifact:crate:tree-sitter-javascript:0.23.1",
            signature_ref: "provenance:crates-io:tree-sitter-javascript:0.23.1",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-javascript:0.23.1",
            local_patch_ref: "not_applicable",
            file_extensions: &["jsx"],
            language_aliases: &["jsx"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Jsx,
        },
        GrammarDescriptor {
            grammar_id: "grammar:html",
            language_id: "language:html",
            display_name: "HTML",
            crate_name: "tree-sitter-html",
            crate_version: "0.23.2",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:html:tree-sitter-html:0.23.2",
            artifact_hash_ref: "artifact:crate:tree-sitter-html:0.23.2",
            signature_ref: "provenance:crates-io:tree-sitter-html:0.23.2",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-html:0.23.2",
            local_patch_ref: "not_applicable",
            file_extensions: &["html", "htm"],
            language_aliases: &["html"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Html,
        },
        GrammarDescriptor {
            grammar_id: "grammar:css",
            language_id: "language:css",
            display_name: "CSS",
            crate_name: "tree-sitter-css",
            crate_version: "0.23.2",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:css:tree-sitter-css:0.23.2",
            artifact_hash_ref: "artifact:crate:tree-sitter-css:0.23.2",
            signature_ref: "provenance:crates-io:tree-sitter-css:0.23.2",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-css:0.23.2",
            local_patch_ref: "not_applicable",
            file_extensions: &["css"],
            language_aliases: &["css"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Css,
        },
        GrammarDescriptor {
            grammar_id: "grammar:json",
            language_id: "language:json",
            display_name: "JSON",
            crate_name: "tree-sitter-json",
            crate_version: "0.24.8",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:json:tree-sitter-json:0.24.8",
            artifact_hash_ref: "artifact:crate:tree-sitter-json:0.24.8",
            signature_ref: "provenance:crates-io:tree-sitter-json:0.24.8",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-json:0.24.8",
            local_patch_ref: "not_applicable",
            file_extensions: &["json", "jsonc"],
            language_aliases: &["json"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Json,
        },
        GrammarDescriptor {
            grammar_id: "grammar:yaml",
            language_id: "language:yaml",
            display_name: "YAML",
            crate_name: "tree-sitter-yaml",
            crate_version: "0.6.1",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:yaml:tree-sitter-yaml:0.6.1",
            artifact_hash_ref: "artifact:crate:tree-sitter-yaml:0.6.1",
            signature_ref: "provenance:crates-io:tree-sitter-yaml:0.6.1",
            upstream_ref: "upstream:github:tree-sitter-grammars/tree-sitter-yaml:0.6.1",
            local_patch_ref: "not_applicable",
            file_extensions: &["yaml", "yml"],
            language_aliases: &["yaml", "yml"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Yaml,
        },
        GrammarDescriptor {
            grammar_id: "grammar:markdown",
            language_id: "language:markdown",
            display_name: "Markdown",
            crate_name: "tree-sitter-md",
            crate_version: "0.3.2",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:markdown:tree-sitter-md:0.3.2",
            artifact_hash_ref: "artifact:crate:tree-sitter-md:0.3.2",
            signature_ref: "provenance:crates-io:tree-sitter-md:0.3.2",
            upstream_ref: "upstream:github:tree-sitter-grammars/tree-sitter-markdown:0.3.2",
            local_patch_ref: "not_applicable",
            file_extensions: &["md", "markdown"],
            language_aliases: &["markdown", "md"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Markdown,
        },
        GrammarDescriptor {
            grammar_id: "grammar:python",
            language_id: "language:python",
            display_name: "Python",
            crate_name: "tree-sitter-python",
            crate_version: "0.23.6",
            grammar_source_class: GrammarSourceClass::BundledCuratedUpstream,
            query_pack_ref: "query-pack:python:tree-sitter-python:0.23.6",
            artifact_hash_ref: "artifact:crate:tree-sitter-python:0.23.6",
            signature_ref: "provenance:crates-io:tree-sitter-python:0.23.6",
            upstream_ref: "upstream:github:tree-sitter/tree-sitter-python:0.23.6",
            local_patch_ref: "not_applicable",
            file_extensions: &["py", "pyw"],
            language_aliases: &["python", "py"],
            supported_cues: CORE_CUES,
            default_budget_policy: BudgetPolicyClass::ForegroundVisibleFile,
            kind: GrammarKind::Python,
        },
    ]
}
