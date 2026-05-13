//! Launch language-pack records and enablement projections.
//!
//! This module binds checked-in pack artifacts to the runtime contracts already
//! owned by the language crate: Tree-sitter grammar resolution, LSP routing,
//! diagnostic defaults, quality-tool hooks, icon metadata, docs refs, and
//! content-integrity posture.

pub mod python_service;
pub mod tsjs_web;

pub use python_service::{
    PythonServiceClaimDepthClass, PythonServiceDiagnosticsDefault, PythonServiceDocsPackRef,
    PythonServiceEnablementFlow, PythonServiceGitSurfaceClass, PythonServiceGitSurfaceRow,
    PythonServiceIconRow, PythonServiceKnownGapRow, PythonServiceLanguagePack,
    PythonServiceLanguagePackEnablementRequest, PythonServiceLanguagePackEnablementSnapshot,
    PythonServiceLanguagePackEnablementStateClass, PythonServiceLanguagePackManifest,
    PythonServiceLanguagePackSchemaVersion, PythonServiceLanguageRow,
    PythonServiceLanguageSupportClass, PythonServiceLaunchBundleReportRef,
    PythonServiceProviderRoute, PythonServiceToolHook, PythonServiceTrustAndIntegrityPolicy,
    PYTHON_SERVICE_LANGUAGE_PACK_SCHEMA_VERSION,
};
pub use tsjs_web::{
    TsJsWebClaimDepthClass, TsJsWebDiagnosticsDefault, TsJsWebDocsPackRef, TsJsWebEnablementFlow,
    TsJsWebIconRow, TsJsWebKnownGapRow, TsJsWebLanguagePack, TsJsWebLanguagePackEnablementRequest,
    TsJsWebLanguagePackEnablementSnapshot, TsJsWebLanguagePackEnablementStateClass,
    TsJsWebLanguagePackManifest, TsJsWebLanguagePackSchemaVersion, TsJsWebLanguageRow,
    TsJsWebLanguageSupportClass, TsJsWebProviderRoute, TsJsWebToolHook,
    TsJsWebTrustAndIntegrityPolicy, TSJS_WEB_LANGUAGE_PACK_SCHEMA_VERSION,
};
