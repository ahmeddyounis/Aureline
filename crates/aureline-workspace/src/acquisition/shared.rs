//! Shared vocabulary for the repository-acquisition beta lane.
//!
//! The three boundary records in [`super::descriptors`] (source locator,
//! checkout plan, bootstrap queue item) and the cross-surface
//! [`super::beta`] projection all share the optional `__fixture__`
//! prelude and the closed surface set that names which client consumes
//! the projection.

use serde::{Deserialize, Serialize};

/// Optional worked-example prelude carried by every acquisition fixture
/// under the `__fixture__` key. Surface code never reads this block; the
/// integration suite does. Mirrors the `fixture_metadata` `$defs` block on
/// the three boundary schemas (`additionalProperties: true`).
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

/// Client surface that consumes a [`super::beta::RepositoryAcquisitionBetaProjection`].
///
/// Start Center, the command palette, deep-link resolvers, and the
/// CLI/headless entry points all read the same projection so no surface
/// mints a private acquisition vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionSurface {
    /// Start Center primary / secondary entry surface and its trust-review sheet.
    StartCenter,
    /// Command palette open / clone / import / resume rows.
    CommandPalette,
    /// Product-owned deep-link intent review.
    DeepLink,
    /// CLI or headless acquisition path.
    CliHeadless,
    /// First-run policy-guided deployment lane.
    PolicyGuidedDeployment,
    /// Support-bundle / claim-manifest export surface.
    Support,
}

impl AcquisitionSurface {
    /// Stable snake_case token used by fixtures and audit packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::DeepLink => "deep_link",
            Self::CliHeadless => "cli_headless",
            Self::PolicyGuidedDeployment => "policy_guided_deployment",
            Self::Support => "support",
        }
    }
}
