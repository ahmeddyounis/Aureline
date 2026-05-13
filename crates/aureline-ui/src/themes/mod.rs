//! Theme packs and appearance-session record vocabulary.
//!
//! Theme packs define the semantic token values for each first-party theme
//! class (dark, light, and high-contrast variants). Appearance-session records
//! persist the active theme selection so shell surfaces can switch live and
//! rehydrate the same appearance on restart.

mod audit;
mod import_review;
mod package;
mod packs;
mod session;

pub use crate::density::DensityClass;
pub use audit::{
    alpha_appearance_audit_manifest, AppearanceAuditError, AppearanceVisualDiffAuditManifest,
};
pub use import_review::{
    imported_theme_mapping_report_with_warnings, ImportedThemeParityReadiness,
    ThemeImportMappingError, ThemeImportMappingReport, ThemeImportMappingSummary,
};
pub use package::{
    first_party_theme_package_manifest, ThemePackageAppearanceManifest, ThemePackageManifestError,
};
pub use packs::{load_first_party_theme_pack, ThemePack, ThemePackError};
pub use session::{
    AccentSourceClass, AccessibilityPostureClass, AppearanceAtomicApplyError, AppearanceAxis,
    AppearanceChangeSet, AppearanceSessionCheckpoint, AppearanceSessionRecord,
    AppearanceSessionRecordKind, AppearanceSessionRevisionEvent,
    AppearanceSessionRevisionEventRecordKind, CauseClass, ContrastMode, FollowSystemPosture,
    LiveAxisRow, LiveFollowSystemPolicyRecord, LiveFollowSystemPolicyRecordKind, LiveUpdateClass,
    OsSignalClass, PolicyContext, PolicyLockReasonClass, PreviewState, RedactionClass,
    ReducedMotionSource, SurfaceScopeClass, TextScale, TextScaleSource, TrustState,
};
