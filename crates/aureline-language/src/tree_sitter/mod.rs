//! Tree-sitter grammar registry and parser lifecycle runtime.
//!
//! The registry is the single launch-language table for bundled Tree-sitter
//! grammars. The parser supervisor consumes that table, publishes lifecycle
//! snapshots, and emits parse-session records compatible with the existing
//! language boundary contract.

mod records;
mod registry;
mod runtime;

pub use records::{
    BudgetPolicyClass, BufferRef, CacheRecord, CacheStatusClass, DerivedCueClass,
    DerivedCuePostureClass, DerivedCueRecord, EpochBinding, EpochRoleClass, ExportPolicy,
    ExportPolicyClass, FailureReasonClass, GrammarResolution, GrammarResolutionStateClass,
    GrammarSourceClass, IncrementalBudget, ParseFreshnessClass, ParseLifecycleStateClass,
    ParseQualityClass, ParseRequestClass, ParseSessionRecord, ParseSessionSchemaVersion,
    ParseState, ParserHost, ParserHostClass, ParserSubstrateClass, SyntaxTreeIdentity, TrustState,
};
pub use registry::{
    default_launch_grammar_registry, GrammarDescriptor, GrammarRegistryEntry, GrammarRegistryError,
    GrammarRegistryRecord, TreeSitterGrammarRegistry, TREE_SITTER_GRAMMAR_REGISTRY_SCHEMA_VERSION,
};
pub use runtime::{
    ParseOutput, ParseRequest, ParserLifecycleSnapshot, ParserRuntimeHandle,
    ParserRuntimeStateClass, ParserStartupError, TreeSitterParserSupervisor,
};
