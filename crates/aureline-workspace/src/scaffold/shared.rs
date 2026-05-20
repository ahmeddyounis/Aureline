//! Shared vocabulary for the scaffold-safety beta lane.
//!
//! The three boundary records in [`super::descriptors`] (template /
//! generator descriptor, scaffold plan, scaffold run) and the
//! cross-surface [`super::beta`] projection all share the optional
//! `__fixture__` prelude and the closed surface set that names which
//! client consumes the projection.

use serde::{Deserialize, Serialize};

/// Optional worked-example prelude carried by every scaffold fixture
/// under the `__fixture__` key. Surface code never reads this block; the
/// integration suite does. Mirrors the `fixture_metadata` `$defs` block on
/// the boundary schemas (`additionalProperties: true`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureMetadata {
    /// Stable fixture name.
    pub name: String,
    /// Short scenario description.
    pub scenario: String,
    /// Doc sections the fixture motivates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub doc_sections: Vec<String>,
    /// Any additional keys the fixture carries.
    #[serde(flatten, default)]
    pub extras: serde_json::Map<String, serde_json::Value>,
}

/// Client surface that consumes a [`super::beta::ScaffoldSafetyBetaProjection`].
///
/// Start Center starter rows, the command-palette generator rows, the
/// generator-preview sheet, AI-assisted generation, extension-provided
/// generation, the CLI / headless creation path, and support exporters all
/// read the same projection so no surface mints a private scaffold-safety
/// vocabulary or grants itself IDE-only generation authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldSurface {
    /// Start Center starter / scaffold rows and the create-project flow.
    StartCenter,
    /// Command-palette "new from template" / "run generator" rows.
    CommandPalette,
    /// The generator-preview / scaffold-plan review sheet.
    GeneratorPreview,
    /// AI-assisted generation that proposes a scaffold plan.
    AiAssist,
    /// Extension-provided template / generator.
    Extension,
    /// CLI or headless project-creation path.
    CliHeadless,
    /// Support-bundle / claim-manifest export surface.
    Support,
}

impl ScaffoldSurface {
    /// Stable snake_case token used by fixtures and audit packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::GeneratorPreview => "generator_preview",
            Self::AiAssist => "ai_assist",
            Self::Extension => "extension",
            Self::CliHeadless => "cli_headless",
            Self::Support => "support",
        }
    }

    /// True when the surface is an AI-assisted or extension-provided
    /// generation path. Those paths reuse the same governed scaffold-plan
    /// and diff-review surface and never gain IDE-only authority.
    pub const fn is_ai_or_extension(self) -> bool {
        matches!(self, Self::AiAssist | Self::Extension)
    }
}
