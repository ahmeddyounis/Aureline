//! Exact-build identity capture for the support-bundle manifest.
//!
//! The support bundle is the field-side join point that lets a reviewer
//! match a crash, symbol pack, docs pack, advisory, or release-center
//! entry to the exact build the user was running. The seed never
//! re-derives that identity — it quotes
//! [`aureline_build_info::build_identity`] and
//! [`aureline_build_info::release_channel_class`] so the manifest's build
//! truth tracks the running binary verbatim.

use aureline_build_info::{build_identity, exact_build_identity_ref, release_channel_class};

use super::manifest::BuildIdentity;
use super::vocabulary::ReleaseChannelClass;

/// Snapshot of the running build's identity, ready to be embedded in a
/// support-bundle manifest. Keeping the capture as a small value type lets
/// tests construct deterministic fixtures without touching the build-info
/// build script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactBuildCapture {
    /// Stable build id used as both `build_id` and `producer_build_id` in
    /// the manifest. The two fields are split in the schema so a future
    /// reproducible-rebuild lane can record an alternative producer; the
    /// seed quotes the same identity for both.
    pub build_id: String,
    /// Workspace product version (e.g. `0.0.0`).
    pub product_version: String,
    /// Closed release-channel class derived from the build-info channel
    /// token. Unknown tokens settle on
    /// [`ReleaseChannelClass::DevLocal`] so the manifest never silently
    /// claims a stable install.
    pub release_channel_class: ReleaseChannelClass,
    /// Exact-build refs the bundle would use to join crashes, symbols,
    /// docs packs, and release-center entries. Always non-empty.
    pub exact_build_refs: Vec<String>,
    /// Tree-state bit lifted from the build-info record. The seed uses
    /// this so reviewers can tell whether the manifest came from a
    /// dirty dev tree or a clean release build.
    pub dirty: bool,
}

impl ExactBuildCapture {
    /// Capture the running build's identity from
    /// [`aureline_build_info::build_identity`].
    pub fn from_running_build() -> Self {
        let record = build_identity();
        Self {
            build_id: exact_build_identity_ref(),
            product_version: record.workspace_version,
            release_channel_class: ReleaseChannelClass::from_build_token(release_channel_class()),
            exact_build_refs: vec![exact_build_identity_ref()],
            dirty: record.dirty,
        }
    }

    /// Construct a deterministic capture for tests and reviewer fixtures.
    /// Real shells should use [`ExactBuildCapture::from_running_build`].
    pub fn for_fixture(
        build_id: impl Into<String>,
        product_version: impl Into<String>,
        release_channel_class: ReleaseChannelClass,
    ) -> Self {
        let build_id = build_id.into();
        Self {
            exact_build_refs: vec![build_id.clone()],
            build_id,
            product_version: product_version.into(),
            release_channel_class,
            dirty: false,
        }
    }

    /// Attach an additional exact-build ref (for example, a debug-symbols
    /// variant of the same release). Duplicates are dropped so the
    /// manifest's `exact_build_refs` array stays unique.
    pub fn with_extra_exact_build_ref(mut self, exact_build_ref: impl Into<String>) -> Self {
        let candidate = exact_build_ref.into();
        if !self.exact_build_refs.contains(&candidate) {
            self.exact_build_refs.push(candidate);
        }
        self
    }

    /// Project the capture onto the manifest's [`BuildIdentity`] record.
    pub fn to_build_identity(&self) -> BuildIdentity {
        BuildIdentity {
            build_id: self.build_id.clone(),
            producer_build_id: self.build_id.clone(),
            product_version: self.product_version.clone(),
            release_channel_class: self.release_channel_class,
            exact_build_refs: self.exact_build_refs.clone(),
        }
    }
}
