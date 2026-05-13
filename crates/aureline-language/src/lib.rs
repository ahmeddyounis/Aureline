//! Language-platform runtime foundations.
//!
//! This crate owns the first launch-language syntax substrate: a curated
//! Tree-sitter grammar registry plus a parser lifecycle that exposes startup,
//! parse, degraded, failure, and shutdown states as reusable records. Editor,
//! search, support, and future router surfaces should consume these records
//! rather than embedding grammar metadata or parser fallback rules privately.

#![doc(html_root_url = "https://docs.rs/aureline-language/0.0.0")]

pub mod tree_sitter;

pub use tree_sitter::{
    default_launch_grammar_registry, BudgetPolicyClass, BufferRef, CacheRecord, CacheStatusClass,
    DerivedCueClass, DerivedCuePostureClass, DerivedCueRecord, EpochBinding, EpochRoleClass,
    ExportPolicy, ExportPolicyClass, FailureReasonClass, GrammarDescriptor, GrammarRegistryEntry,
    GrammarRegistryError, GrammarRegistryRecord, GrammarResolution, GrammarResolutionStateClass,
    GrammarSourceClass, IncrementalBudget, ParseFreshnessClass, ParseLifecycleStateClass,
    ParseOutput, ParseQualityClass, ParseRequest, ParseRequestClass, ParseSessionRecord,
    ParseSessionSchemaVersion, ParseState, ParserHost, ParserHostClass, ParserLifecycleSnapshot,
    ParserRuntimeHandle, ParserRuntimeStateClass, ParserStartupError, ParserSubstrateClass,
    SyntaxTreeIdentity, TreeSitterGrammarRegistry, TreeSitterParserSupervisor, TrustState,
    TREE_SITTER_GRAMMAR_REGISTRY_SCHEMA_VERSION,
};
