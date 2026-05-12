//! Degraded-state vocabulary shared across core shell surfaces.
//!
//! The token set is intentionally small and stable so shell surfaces, docs, and
//! exported evidence can reuse the same terminology without drift.

use serde::{Deserialize, Serialize};

/// Degraded-state tokens projected into chrome, placeholder cards, and exported evidence.
///
/// The serialized form matches the UX schemas that carry degraded-state tokens
/// (for example `schemas/ux/title_context_bar_state.schema.json` and
/// `schemas/ux/lifecycle_status_card.schema.json`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DegradedStateToken {
    /// Background preparation is in progress.
    Warming,
    /// Served from prior known-good state.
    Cached,
    /// Only some scope is represented.
    Partial,
    /// Known to be out of date.
    Stale,
    /// Required network/service target unavailable.
    Offline,
    /// Denied by org or local trust policy.
    PolicyBlocked,
    /// Supported with narrower guarantees.
    Limited,
    /// Intentionally outside the current supported promise.
    Unsupported,
    /// Available as an early Labs capability with an unstable contract.
    Labs,
    /// Present but not yet stable or certified.
    Experimental,
    /// Compatibility evidence is stale and needs to be refreshed.
    RetestPending,
}

impl DegradedStateToken {
    /// Returns the stable token used in exported evidence.
    pub const fn token(self) -> &'static str {
        match self {
            Self::Warming => "Warming",
            Self::Cached => "Cached",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Offline => "Offline",
            Self::PolicyBlocked => "PolicyBlocked",
            Self::Limited => "Limited",
            Self::Unsupported => "Unsupported",
            Self::Labs => "Labs",
            Self::Experimental => "Experimental",
            Self::RetestPending => "RetestPending",
        }
    }

    /// Returns the human-readable label shown in shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Warming => "Warming",
            Self::Cached => "Cached",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Offline => "Offline",
            Self::PolicyBlocked => "Policy blocked",
            Self::Limited => "Limited",
            Self::Unsupported => "Unsupported",
            Self::Labs => "Labs",
            Self::Experimental => "Experimental",
            Self::RetestPending => "Retest pending",
        }
    }

    /// Returns a short default description suitable for placeholder-card body copy.
    pub const fn default_description(self) -> &'static str {
        match self {
            Self::Warming => "Background preparation is in progress.",
            Self::Cached => "Showing cached data from the last known-good snapshot.",
            Self::Partial => "Only part of the requested scope is currently available.",
            Self::Stale => "The visible data is known to be out of date.",
            Self::Offline => "A required target is unavailable; local-only work may continue.",
            Self::PolicyBlocked => {
                "Policy or trust settings block this capability in the current context."
            }
            Self::Limited => "The surface remains usable with reduced capability.",
            Self::Unsupported => {
                "This surface is not supported in the current environment or build."
            }
            Self::Labs => "This capability is available through Labs with a narrower contract.",
            Self::Experimental => "This capability is available but not yet stable or certified.",
            Self::RetestPending => {
                "Compatibility evidence is pending a refresh before it can be treated as current."
            }
        }
    }
}
