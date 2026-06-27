//! Canonical seed builders for the M5 design-system foundation package.
//!
//! These builders are the single producer of the checked-in package fixture, its next-version
//! sibling, the diff fixture, and the release-packet proof. The headless emitter and the inline
//! tests both call them so the in-code package, the schema fixtures, and the proof never drift.
//! The density, motion, contrast, and component-state families are derived from the same
//! [`aureline_ui`] and [`crate::CanonicalStateClass`] vocabularies the rest of the design system
//! uses, so those rows read from one source rather than feature-local wiring.

use super::*;

/// Stable id of the canonical foundation package.
pub const M5_FOUNDATION_PACKAGE_ID: &str = "design-system:foundation-package:core";

/// Version of the canonical foundation package.
pub const M5_FOUNDATION_PACKAGE_VERSION: &str = "1.0.0";

/// Version of the next foundation package (the diff drill target).
pub const M5_FOUNDATION_PACKAGE_NEXT_VERSION: &str = "1.1.0";

/// Mint timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

const OWNER_ROLE: &str = "Design system owner";

fn reason_id(entry_id: &str) -> String {
    format!("{}{}.downgrade", M5_FOUNDATION_MESSAGE_ID_PREFIX, entry_id)
}

fn downgrade(to: &str, entry_id: &str, since: &str) -> M5EntryDowngrade {
    M5EntryDowngrade {
        downgraded_to: to.to_owned(),
        reason_message_id: reason_id(entry_id),
        since_package_version: since.to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_FOUNDATION_PACKAGE_SCHEMA_REF.to_owned(),
        M5_FOUNDATION_PACKAGE_DOC_REF.to_owned(),
        M5_FOUNDATION_PACKAGE_PROOF_REF.to_owned(),
    ]
}

// --- Token families that carry literal semantic references. ---

fn color_family(version: u32, entries: Vec<M5FoundationEntry>) -> M5FoundationFamily {
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Color,
        family_id: "color".to_owned(),
        display_name: "Semantic color tokens".to_owned(),
        family_version: version,
        entries,
    }
}

fn canonical_color_entries() -> Vec<M5FoundationEntry> {
    vec![
        M5FoundationEntry::supported(
            "color.surface.shell",
            "Shell surface",
            "al.color.surface.shell",
        ),
        M5FoundationEntry::supported(
            "color.surface.raised",
            "Raised surface",
            "al.color.surface.raised",
        ),
        M5FoundationEntry::supported(
            "color.text.primary",
            "Primary text",
            "al.color.text.primary",
        ),
        M5FoundationEntry::supported(
            "color.text.secondary",
            "Secondary text",
            "al.color.text.secondary",
        ),
        M5FoundationEntry::supported(
            "color.state.success",
            "Success hue",
            "al.color.state.success",
        ),
        // A deprecated token stays published and points at its supported replacement.
        M5FoundationEntry::downgraded(
            "color.text.muted",
            "Muted text (deprecated)",
            "al.color.text.muted",
            M5SupportState::Deprecated,
            downgrade("color.text.secondary", "color.text.muted", "1.0.0"),
        ),
    ]
}

fn spacing_family() -> M5FoundationFamily {
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Spacing,
        family_id: "spacing".to_owned(),
        display_name: "Spacing scale tokens".to_owned(),
        family_version: 1,
        entries: vec![
            M5FoundationEntry::supported("space.2", "Space 2 (8px step)", "space.2"),
            M5FoundationEntry::supported("space.4", "Space 4 (16px step)", "space.4"),
            M5FoundationEntry::supported("space.6", "Space 6 (24px step)", "space.6"),
            M5FoundationEntry::downgraded(
                "space.legacy.tight",
                "Legacy tight spacing (deprecated)",
                "space.legacy.tight",
                M5SupportState::Deprecated,
                downgrade("space.2", "space.legacy.tight", "1.0.0"),
            ),
        ],
    }
}

fn typography_family() -> M5FoundationFamily {
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Typography,
        family_id: "typography".to_owned(),
        display_name: "Typography tokens".to_owned(),
        family_version: 1,
        entries: vec![
            M5FoundationEntry::supported("typography.body", "Body text", "typography.body"),
            M5FoundationEntry::supported(
                "typography.heading",
                "Heading text",
                "typography.heading",
            ),
            M5FoundationEntry::supported("typography.code", "Monospace code", "typography.code"),
        ],
    }
}

fn icon_family() -> M5FoundationFamily {
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Icon,
        family_id: "icon".to_owned(),
        display_name: "Icon tokens".to_owned(),
        family_version: 1,
        entries: vec![
            M5FoundationEntry::supported("icon.size.sm", "Small icon", "size.icon.sm"),
            M5FoundationEntry::supported("icon.size.md", "Medium icon", "size.icon.md"),
            M5FoundationEntry::supported(
                "icon.metaphor.lock",
                "Lock metaphor",
                "icon.metaphor.lock",
            ),
            // An unsupported token stays published and points at its fallback.
            M5FoundationEntry::downgraded(
                "icon.legacy.spinner",
                "Legacy spinner (unsupported)",
                "icon.legacy.spinner",
                M5SupportState::Unsupported,
                downgrade("icon.progress.spinner", "icon.legacy.spinner", "1.0.0"),
            ),
        ],
    }
}

// --- Families derived from the canonical aureline_ui / state vocabularies. ---

fn density_family() -> M5FoundationFamily {
    let entries = [
        (DensityClass::Compact, "Compact density"),
        (DensityClass::Standard, "Standard density"),
        (DensityClass::Comfortable, "Comfortable density"),
    ]
    .iter()
    .map(|(class, name)| {
        M5FoundationEntry::supported(&format!("density.{}", class.token()), name, class.token())
    })
    .collect();
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Density,
        family_id: "density".to_owned(),
        display_name: "Density classes".to_owned(),
        family_version: 1,
        entries,
    }
}

fn motion_family() -> M5FoundationFamily {
    let entries = [
        (AccessibilityPostureClass::MotionStandard, "Standard motion"),
        (AccessibilityPostureClass::MotionReduced, "Reduced motion"),
        (AccessibilityPostureClass::MotionLowMotion, "Low motion"),
        (
            AccessibilityPostureClass::MotionPowerSaver,
            "Power-saving motion",
        ),
        (
            AccessibilityPostureClass::MotionCriticalHotPath,
            "Critical hot-path motion",
        ),
    ]
    .iter()
    .map(|(posture, name)| {
        // The entry id drops the shared `motion_` prefix the posture token carries.
        let suffix = posture
            .token()
            .strip_prefix("motion_")
            .unwrap_or(posture.token());
        M5FoundationEntry::supported(&format!("motion.{suffix}"), name, posture.token())
    })
    .collect();
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Motion,
        family_id: "motion".to_owned(),
        display_name: "Motion postures".to_owned(),
        family_version: 1,
        entries,
    }
}

fn contrast_family() -> M5FoundationFamily {
    let entries = [
        (ThemeClass::DarkReference, "Dark reference"),
        (ThemeClass::LightParity, "Light parity"),
        (ThemeClass::HighContrastDark, "High-contrast dark"),
        (ThemeClass::HighContrastLight, "High-contrast light"),
    ]
    .iter()
    .map(|(theme, name)| {
        M5FoundationEntry::supported(&format!("contrast.{}", theme.token()), name, theme.token())
    })
    .collect();
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::Contrast,
        family_id: "contrast".to_owned(),
        display_name: "Contrast / theme classes".to_owned(),
        family_version: 1,
        entries,
    }
}

fn component_state_family() -> M5FoundationFamily {
    let entries = CanonicalStateClass::required()
        .iter()
        .map(|state| {
            let token = state.as_str();
            M5FoundationEntry::supported(
                &format!("state.{token}"),
                &format!("{} state", capitalize(token)),
                token,
            )
        })
        .collect();
    M5FoundationFamily {
        family_kind: M5FoundationFamilyKind::ComponentState,
        family_id: "component_state".to_owned(),
        display_name: "Controlled component states".to_owned(),
        family_version: 1,
        entries,
    }
}

fn capitalize(token: &str) -> String {
    let mut chars = token.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn package(package_version: &str, families: Vec<M5FoundationFamily>) -> M5FoundationPackage {
    M5FoundationPackage {
        record_kind: M5_FOUNDATION_PACKAGE_RECORD_KIND.to_owned(),
        schema_version: M5_FOUNDATION_PACKAGE_SCHEMA_VERSION,
        package_id: M5_FOUNDATION_PACKAGE_ID.to_owned(),
        package_version: package_version.to_owned(),
        owner_role: OWNER_ROLE.to_owned(),
        families,
        proof_lane_ref: M5_FOUNDATION_PACKAGE_PROOF_REF.to_owned(),
        release_packet_ref: M5_FOUNDATION_PACKAGE_RELEASE_PACKET_REF.to_owned(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_FOUNDATION_MESSAGE_ID_PREFIX, M5_FOUNDATION_PACKAGE_ID
        ),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical foundation package (version 1.0.0).
///
/// Publishes one family per [`M5FoundationFamilyKind`]; the density, motion, contrast, and
/// component-state families read from the same canonical vocabulary the rest of the design
/// system uses, and three entries are deliberately downgraded (two deprecated, one unsupported)
/// so the downgrade-preservation path is exercised by the checked-in fixture.
pub fn seeded_m5_foundation_package() -> M5FoundationPackage {
    package(
        M5_FOUNDATION_PACKAGE_VERSION,
        vec![
            color_family(1, canonical_color_entries()),
            spacing_family(),
            typography_family(),
            icon_family(),
            density_family(),
            motion_family(),
            contrast_family(),
            component_state_family(),
        ],
    )
}

/// Builds the next foundation package (version 1.1.0): the diff drill target.
///
/// Only the color family changes, so the diff names every change kind against one family: a new
/// supported entry (`color.text.tertiary`), a value change (`color.surface.raised`), a support
/// downgrade (`color.text.muted` moves deprecated → unsupported), and a removed entry
/// (`color.state.success`). The removed and downgraded entries are retained in the diff, not
/// dropped.
pub fn seeded_m5_foundation_package_next() -> M5FoundationPackage {
    let next_color = vec![
        M5FoundationEntry::supported(
            "color.surface.shell",
            "Shell surface",
            "al.color.surface.shell",
        ),
        // Value change: the raised surface now resolves to the elevated token.
        M5FoundationEntry::supported(
            "color.surface.raised",
            "Raised surface",
            "al.color.surface.elevated",
        ),
        M5FoundationEntry::supported(
            "color.text.primary",
            "Primary text",
            "al.color.text.primary",
        ),
        M5FoundationEntry::supported(
            "color.text.secondary",
            "Secondary text",
            "al.color.text.secondary",
        ),
        // New entry.
        M5FoundationEntry::supported(
            "color.text.tertiary",
            "Tertiary text",
            "al.color.text.tertiary",
        ),
        // Support downgrade: deprecated → unsupported, recorded at the new version.
        M5FoundationEntry::downgraded(
            "color.text.muted",
            "Muted text (unsupported)",
            "al.color.text.muted",
            M5SupportState::Unsupported,
            downgrade("color.text.secondary", "color.text.muted", "1.1.0"),
        ),
        // `color.state.success` removed.
    ];
    package(
        M5_FOUNDATION_PACKAGE_NEXT_VERSION,
        vec![
            color_family(2, next_color),
            spacing_family(),
            typography_family(),
            icon_family(),
            density_family(),
            motion_family(),
            contrast_family(),
            component_state_family(),
        ],
    )
}
