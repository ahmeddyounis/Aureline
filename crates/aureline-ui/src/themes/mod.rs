//! Theme packs and appearance-session record vocabulary.
//!
//! Theme packs define the semantic token values for each first-party theme
//! class (dark, light, and high-contrast variants). Appearance-session records
//! persist the active theme selection so shell surfaces can switch live and
//! rehydrate the same appearance on restart.

mod packs;
mod session;

pub use packs::{load_first_party_theme_pack, ThemePack, ThemePackError};
pub use session::{
    AccentSourceClass, AccessibilityPostureClass, AppearanceSessionRecord, AppearanceSessionRecordKind,
    AppearanceAxis, ContrastMode, DensityClass, FollowSystemPosture, LiveAxisRow,
    LiveFollowSystemPolicyRecord, LiveFollowSystemPolicyRecordKind, LiveUpdateClass,
    OsSignalClass, PolicyContext, PolicyLockReasonClass, PreviewState, RedactionClass,
    ReducedMotionSource, SurfaceScopeClass, TextScale, TextScaleSource, TrustState,
};
