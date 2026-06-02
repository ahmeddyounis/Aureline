//! Framework support strips, exact-vs-heuristic row certainty, convention
//! diagnostics, and review-first generator / codemod previews.
//!
//! This crate owns the cross-surface truth records every framework-aware
//! Aureline surface (route explorer, component / service tree, convention-
//! diagnostics lane, generator-preview review, notebook framework-context
//! inspector, AI-assist context lane, and support export) must read before
//! a framework claim becomes actionable. The records compose with — and do
//! NOT duplicate — the framework_certainty_row and source_sync_chip
//! contracts frozen at
//! [`/docs/framework/framework_certainty_and_source_sync_contract.md`](../../../docs/framework/framework_certainty_and_source_sync_contract.md).
//!
//! Two record models live here:
//!
//! - [`status_and_certainty::FrameworkSupportStrip`] — the header strip
//!   that names the detected framework / version, support class (core
//!   native / framework pack / bridge / heuristic / unsupported), pack or
//!   bridge source, pack / runtime health, freshness, local-or-remote
//!   scope, version-compatibility posture, and the closed list of
//!   fallback-open actions. Mirrors
//!   [`/schemas/framework/framework_support_strip.schema.json`](../../../schemas/framework/framework_support_strip.schema.json).
//! - [`status_and_certainty::FrameworkObjectCertainty`] — the row-level
//!   exact-vs-heuristic record carried by route / component / service /
//!   entity rows, convention-diagnostic rows, and generator / codemod /
//!   scaffold previews. Mirrors
//!   [`/schemas/framework/framework_object_certainty.schema.json`](../../../schemas/framework/framework_object_certainty.schema.json).
//!
//! No raw URLs, raw absolute paths, raw hostnames, raw IPs, raw secret
//! bytes, raw generated bodies, or raw file bodies ever cross either
//! record. Only opaque ids, closed-vocabulary tokens, and export-safe
//! labels do. Both records expose a `validate()` method that surfaces the
//! truth-rule findings the schemas freeze so consumers can refuse to
//! render rows that quietly broaden authority (e.g. a route row that
//! claims `exact_pack_backed` without a pack source, a heuristic strip
//! that offers a `request_pack_update` action, a generator preview that
//! tries to delete a user-owned file without confirmation, or a
//! convention diagnostic that wires `open_generator_preview` without
//! pairing a generator-preview ref).

#![doc(html_root_url = "https://docs.rs/aureline-framework/0.0.0")]

pub mod status_and_certainty;

pub use status_and_certainty::{
    AuthoredOriginClass, CertaintyLabelClass, CompatibilityBlock, ConventionCertaintyClass,
    ConventionDiagnosticBlock, ConventionDiagnosticClass, ConventionFixActionClass,
    DependencyImpactClass, EvidenceAnchor, EvidenceAnchorKindClass, FileEffectClass,
    FileOwnershipClass, Finding, FrameworkFamilyClass, FrameworkIdentityBlock,
    FrameworkObjectCertainty, FrameworkObjectKind, FrameworkObjectRowBlock, FrameworkObjectRowKind,
    FrameworkSupportActionClass, FrameworkSupportStrip, FreshnessClass, GeneratorFileEffectRow,
    GeneratorKindClass, GeneratorPreviewBlock, HealthBlock, HealthClass, LocalityClass,
    PackOrBridgeSourceBlock, PackSourceClass, RollbackClass, RowActionClass, ScopeBlock,
    SupportClass, SurfaceClass, VersionCompatibilityClass, FRAMEWORK_FRESHNESS_AUTHORITATIVE_LIVE,
    FRAMEWORK_FRESHNESS_DEGRADED_CACHED, FRAMEWORK_FRESHNESS_STALE, FRAMEWORK_FRESHNESS_UNVERIFIED,
    FRAMEWORK_FRESHNESS_WARM_CACHED, FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND,
    FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION, FRAMEWORK_SUPPORT_STRIP_RECORD_KIND,
    FRAMEWORK_SUPPORT_STRIP_SCHEMA_VERSION,
};
