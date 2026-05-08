//! In-memory synthetic filesystem model.
//!
//! The prototype deliberately does NOT call the real filesystem.
//! Instead it models one workspace root with a reviewable map of
//! canonical objects, their alias presentations, generation
//! tokens, and permission snapshots. This keeps the emitted
//! save-plan records byte-stable across hosts: the same scenario
//! table produces the same JSON on every machine because no
//! machine-specific inode / mtime / FS casing leaks into the
//! record.
//!
//! A [`SyntheticRoot`] owns:
//!
//! - A [`crate::capabilities::RootCapabilityEnvelope`].
//! - A map from canonical URI to [`CanonicalObjectState`]
//!   (strongest + fallback tokens, permission snapshot, alias
//!   list, content bytes, mutable generation counter).
//! - A map from presentation URI to canonical URI, carrying the
//!   [`crate::identity::AliasKind`] disclosed at open.
//!
//! The save pipeline in [`crate::save`] drives these maps: open
//! reads the current strongest token onto the save-target token,
//! [`SyntheticRoot::apply_external_change`] bumps the generation
//! to simulate a sibling writer, and [`SyntheticRoot::apply_commit`]
//! records the new generation after a successful commit.

use std::collections::BTreeMap;

use crate::capabilities::{
    CapabilityFlags, FallbackIdentityTokenKind, NormalizationForm, RootCapabilityEnvelope,
    RootClass, StrongestIdentityTokenKind,
};
use crate::identity::{
    Alias, AliasKind, AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityToken,
    LogicalWorkspaceIdentity, PresentationPath, TrustState,
};
use crate::save::PermissionSnapshot;

/// Mutable state for one canonical filesystem object inside a
/// synthetic root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalObjectState {
    pub canonical_uri: String,
    /// Workspace-relative logical URI that identifies this object
    /// across aliases and across case-only / normalization
    /// renames.
    pub logical_uri: String,
    pub normalization_form: NormalizationForm,
    pub strongest_token_kind: StrongestIdentityTokenKind,
    /// Base part of the strongest token (e.g. `"dev:1/ino:100"`
    /// for a POSIX root). The generation counter is appended on
    /// every read.
    pub strongest_token_base: String,
    pub generation: u64,
    pub fallback_tokens: Vec<FallbackIdentityToken>,
    pub permission_snapshot: PermissionSnapshot,
    pub aliases: Vec<Alias>,
    pub content: Vec<u8>,
}

/// One entry in the presentation map: how a presentation URI
/// resolves to a canonical object and which alias kind the VFS
/// records on the identity record at open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationBinding {
    pub canonical_uri: String,
    pub display_label: String,
    pub alias_kind_at_open: Option<AliasKind>,
    /// Resolution chain rendered on the opened alias entry.
    pub resolution_chain: Vec<String>,
}

/// A synthetic workspace root. Owns the capability envelope,
/// canonical object map, and presentation map.
#[derive(Debug, Clone)]
pub struct SyntheticRoot {
    envelope: RootCapabilityEnvelope,
    logical_workspace_identity_template: LogicalWorkspaceIdentity,
    root_badge: String,
    canonical_objects: BTreeMap<String, CanonicalObjectState>,
    presentations: BTreeMap<String, PresentationBinding>,
}

impl SyntheticRoot {
    pub fn envelope(&self) -> &RootCapabilityEnvelope {
        &self.envelope
    }

    pub fn root_badge(&self) -> &str {
        &self.root_badge
    }

    pub fn canonical_objects(&self) -> impl Iterator<Item = (&String, &CanonicalObjectState)> {
        self.canonical_objects.iter()
    }

    /// Resolve a presentation URI. Returns an error string when
    /// the URI is unknown or the target canonical object is
    /// missing; both are prototype / fixture bugs rather than
    /// real-world conditions.
    pub fn resolve(&self, presentation_uri: &str) -> Result<ResolvedPresentation<'_>, String> {
        let binding = self
            .presentations
            .get(presentation_uri)
            .ok_or_else(|| format!("unknown presentation uri: {presentation_uri}"))?;
        let object = self
            .canonical_objects
            .get(&binding.canonical_uri)
            .ok_or_else(|| {
                format!(
                    "presentation {presentation_uri} points at missing canonical {}",
                    binding.canonical_uri
                )
            })?;
        Ok(ResolvedPresentation { binding, object })
    }

    /// Read the current strongest identity token (generation
    /// counter observed now). Every read is cheap; the harness
    /// calls this at open time and at compare-before-write time.
    pub fn read_strongest_token(&self, canonical_uri: &str) -> Option<IdentityToken> {
        let obj = self.canonical_objects.get(canonical_uri)?;
        Some(render_strongest_token(obj))
    }

    /// Read the full fallback identity token list.
    pub fn fallback_tokens(&self, canonical_uri: &str) -> Vec<FallbackIdentityToken> {
        self.canonical_objects
            .get(canonical_uri)
            .map(|obj| obj.fallback_tokens.clone())
            .unwrap_or_default()
    }

    /// Return the identity-record layers (1-4) for the opened
    /// presentation URI. Layer 5 is the save-target token issued
    /// by [`crate::save`].
    pub fn identity_record(
        &self,
        presentation_uri: &str,
    ) -> Result<crate::identity::IdentityRecord, String> {
        let resolved = self.resolve(presentation_uri)?;
        let presentation_path = PresentationPath {
            uri: presentation_uri.to_owned(),
            display_label: resolved.binding.display_label.clone(),
            root_badge: self.root_badge.clone(),
        };
        let logical = LogicalWorkspaceIdentity {
            workspace_id: self
                .logical_workspace_identity_template
                .workspace_id
                .clone(),
            root_id: self.envelope.root_id.clone(),
            logical_uri: resolved.object.logical_uri.clone(),
            trust_state: self.logical_workspace_identity_template.trust_state,
            policy_scope: self
                .logical_workspace_identity_template
                .policy_scope
                .clone(),
        };
        let canonical = CanonicalFilesystemObject {
            canonical_uri: resolved.object.canonical_uri.clone(),
            normalization_form: resolved.object.normalization_form,
            strongest_identity_token: render_strongest_token(resolved.object),
            fallback_identity_tokens: resolved.object.fallback_tokens.clone(),
        };
        Ok(crate::identity::IdentityRecord {
            presentation_path,
            logical_workspace_identity: logical,
            canonical_filesystem_object: canonical,
            alias_set: AliasSet {
                aliases: resolved.object.aliases.clone(),
            },
        })
    }

    /// Simulate a sibling writer (or any external change) by
    /// bumping the generation counter on a canonical object. Used
    /// by the external-change-detected scenario.
    pub fn apply_external_change(&mut self, canonical_uri: &str) -> Option<u64> {
        let obj = self.canonical_objects.get_mut(canonical_uri)?;
        obj.generation += 1;
        Some(obj.generation)
    }

    /// Record a successful commit by bumping the generation and
    /// storing the new content. The harness calls this when the
    /// save pipeline succeeds so a follow-up open sees the new
    /// generation token.
    pub fn apply_commit(&mut self, canonical_uri: &str, new_content: Vec<u8>) -> Option<u64> {
        let obj = self.canonical_objects.get_mut(canonical_uri)?;
        obj.generation += 1;
        obj.content = new_content;
        Some(obj.generation)
    }

    pub fn permission_snapshot(&self, canonical_uri: &str) -> Option<PermissionSnapshot> {
        self.canonical_objects
            .get(canonical_uri)
            .map(|o| o.permission_snapshot.clone())
    }
}

/// The resolved presentation → canonical mapping plus a pointer
/// to the canonical object state. Returned by [`SyntheticRoot::resolve`].
#[derive(Debug, Clone, Copy)]
pub struct ResolvedPresentation<'a> {
    pub binding: &'a PresentationBinding,
    pub object: &'a CanonicalObjectState,
}

/// One synthetic workspace: one root + one logical workspace
/// identity template. The prototype models one root per workspace
/// so multi-root behaviour is out of scope.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: SyntheticRoot,
}

impl Workspace {
    pub fn new(root: SyntheticRoot) -> Self {
        Self { root }
    }
}

fn render_strongest_token(obj: &CanonicalObjectState) -> IdentityToken {
    IdentityToken {
        kind: obj.strongest_token_kind,
        value: format!("{}/gen:{}", obj.strongest_token_base, obj.generation),
    }
}

// ---------------------------------------------------------------------------
// Builder — keeps scenario construction readable.
// ---------------------------------------------------------------------------

/// Builder for a [`SyntheticRoot`].
#[derive(Debug, Clone)]
pub struct SyntheticRootBuilder {
    root_class: RootClass,
    root_id: String,
    root_badge: String,
    workspace_id: String,
    trust_state: TrustState,
    capability_flags: CapabilityFlags,
    strongest_token_kind: StrongestIdentityTokenKind,
    fallback_token_kinds: Vec<FallbackIdentityTokenKind>,
    preferred_save_mode: crate::capabilities::AtomicWriteMode,
    permitted_save_modes: Vec<crate::capabilities::AtomicWriteMode>,
    watcher_source: crate::watcher::WatcherSource,
    mount_graph_hash: Option<String>,
    canonical_objects: BTreeMap<String, CanonicalObjectState>,
    presentations: BTreeMap<String, PresentationBinding>,
}

impl SyntheticRootBuilder {
    pub fn new(
        root_id: impl Into<String>,
        root_class: RootClass,
        capability_flags: CapabilityFlags,
    ) -> Self {
        let root_badge = match root_class {
            RootClass::LocalPosixLike | RootClass::LocalWindowsLike => "local",
            RootClass::RemoteAgentMount => "remote",
            RootClass::ContainerMount => "container",
            RootClass::VirtualGeneratedDocument => "virtual",
            RootClass::ArchiveLikeView => "archive",
        }
        .to_owned();
        let strongest_token_kind = match root_class {
            RootClass::LocalPosixLike => StrongestIdentityTokenKind::DeviceInodeGeneration,
            RootClass::LocalWindowsLike => StrongestIdentityTokenKind::WindowsObjectId,
            RootClass::RemoteAgentMount => StrongestIdentityTokenKind::ProviderObjectIdRevision,
            RootClass::ContainerMount => StrongestIdentityTokenKind::DeviceInodeGeneration,
            RootClass::VirtualGeneratedDocument => {
                StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs
            }
            RootClass::ArchiveLikeView => StrongestIdentityTokenKind::ContentHashOnly,
        };
        let watcher_source = match root_class {
            RootClass::LocalPosixLike | RootClass::LocalWindowsLike => {
                crate::watcher::WatcherSource::OsNativeWatcher
            }
            RootClass::RemoteAgentMount | RootClass::ContainerMount => {
                crate::watcher::WatcherSource::RemoteAgentWatcherStream
            }
            RootClass::VirtualGeneratedDocument | RootClass::ArchiveLikeView => {
                crate::watcher::WatcherSource::PollingFallback
            }
        };
        let preferred_save_mode =
            if capability_flags.read_only || capability_flags.policy_constrained {
                crate::capabilities::AtomicWriteMode::Blocked
            } else if capability_flags.supports_atomic_replace {
                crate::capabilities::AtomicWriteMode::AtomicReplace
            } else if capability_flags.supports_conditional_remote_write {
                crate::capabilities::AtomicWriteMode::ConditionalRemoteWrite
            } else if capability_flags.supports_in_place_write {
                crate::capabilities::AtomicWriteMode::InPlaceWrite
            } else {
                crate::capabilities::AtomicWriteMode::Blocked
            };
        let mut permitted = Vec::new();
        if capability_flags.supports_atomic_replace {
            permitted.push(crate::capabilities::AtomicWriteMode::AtomicReplace);
        }
        if capability_flags.supports_in_place_write {
            permitted.push(crate::capabilities::AtomicWriteMode::InPlaceWrite);
        }
        if capability_flags.supports_conditional_remote_write {
            permitted.push(crate::capabilities::AtomicWriteMode::ConditionalRemoteWrite);
        }
        if capability_flags.read_only || capability_flags.policy_constrained {
            permitted.push(crate::capabilities::AtomicWriteMode::Blocked);
        }
        Self {
            root_class,
            root_id: root_id.into(),
            root_badge,
            workspace_id: "ws-aureline-primary".to_owned(),
            trust_state: TrustState::Trusted,
            capability_flags,
            strongest_token_kind,
            fallback_token_kinds: vec![FallbackIdentityTokenKind::InodeMtimeSize],
            preferred_save_mode,
            permitted_save_modes: permitted,
            watcher_source,
            mount_graph_hash: None,
            canonical_objects: BTreeMap::new(),
            presentations: BTreeMap::new(),
        }
    }

    pub fn with_workspace_id(mut self, id: impl Into<String>) -> Self {
        self.workspace_id = id.into();
        self
    }

    pub fn with_trust_state(mut self, trust: TrustState) -> Self {
        self.trust_state = trust;
        self
    }

    pub fn with_fallback_token_kinds(mut self, kinds: Vec<FallbackIdentityTokenKind>) -> Self {
        self.fallback_token_kinds = kinds;
        self
    }

    pub fn with_mount_graph_hash(mut self, hash: impl Into<String>) -> Self {
        self.mount_graph_hash = Some(hash.into());
        self
    }

    /// Override the preferred save mode. Used by scenarios that
    /// model roots which prefer conditional remote writes over
    /// atomic replace even when both are technically supported.
    pub fn with_preferred_save_mode(mut self, mode: crate::capabilities::AtomicWriteMode) -> Self {
        self.preferred_save_mode = mode;
        self
    }

    /// Register a canonical filesystem object with the root.
    ///
    /// `generation` is the initial generation counter; every read
    /// observes `strongest_token_base` + `/gen:<current>` so the
    /// sniff token is cheap to compute.
    #[allow(clippy::too_many_arguments)]
    pub fn add_canonical_object(
        mut self,
        canonical_uri: impl Into<String>,
        logical_uri: impl Into<String>,
        normalization_form: NormalizationForm,
        strongest_token_base: impl Into<String>,
        generation: u64,
        fallback_tokens: Vec<FallbackIdentityToken>,
        permission_snapshot: PermissionSnapshot,
        aliases: Vec<Alias>,
        content: Vec<u8>,
    ) -> Self {
        let canonical_uri: String = canonical_uri.into();
        self.canonical_objects.insert(
            canonical_uri.clone(),
            CanonicalObjectState {
                canonical_uri,
                logical_uri: logical_uri.into(),
                normalization_form,
                strongest_token_kind: self.strongest_token_kind,
                strongest_token_base: strongest_token_base.into(),
                generation,
                fallback_tokens,
                permission_snapshot,
                aliases,
                content,
            },
        );
        self
    }

    /// Register a presentation URI that resolves to `canonical_uri`.
    pub fn add_presentation(
        mut self,
        presentation_uri: impl Into<String>,
        display_label: impl Into<String>,
        canonical_uri: impl Into<String>,
        alias_kind_at_open: Option<AliasKind>,
        resolution_chain: Vec<String>,
    ) -> Self {
        let presentation_uri = presentation_uri.into();
        let binding = PresentationBinding {
            canonical_uri: canonical_uri.into(),
            display_label: display_label.into(),
            alias_kind_at_open,
            resolution_chain,
        };
        self.presentations.insert(presentation_uri, binding);
        self
    }

    pub fn build(self) -> SyntheticRoot {
        let envelope = RootCapabilityEnvelope {
            root_id: self.root_id.clone(),
            root_class: self.root_class,
            capability_flags: self.capability_flags,
            strongest_identity_token_kind: self.strongest_token_kind,
            fallback_identity_token_kinds: self.fallback_token_kinds,
            preferred_save_mode: self.preferred_save_mode,
            permitted_save_modes: self.permitted_save_modes,
            watcher_source: self.watcher_source,
            mount_graph_hash: self.mount_graph_hash,
        };
        let logical_template = LogicalWorkspaceIdentity {
            workspace_id: self.workspace_id,
            root_id: self.root_id.clone(),
            logical_uri: String::new(),
            trust_state: self.trust_state,
            policy_scope: None,
        };
        SyntheticRoot {
            envelope,
            logical_workspace_identity_template: logical_template,
            root_badge: self.root_badge,
            canonical_objects: self.canonical_objects,
            presentations: self.presentations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::CaseSensitivity;
    use crate::capabilities::SymlinkEscapePolicy;

    fn posix_flags() -> CapabilityFlags {
        CapabilityFlags {
            supports_atomic_replace: true,
            supports_in_place_write: true,
            supports_conditional_remote_write: false,
            case_sensitivity: CaseSensitivity::InsensitivePreserving,
            unicode_normalization: NormalizationForm::MixedObserved,
            supports_case_only_rename: true,
            supports_unicode_normalization_rename: true,
            symlink_escape_policy: SymlinkEscapePolicy::Warn,
            read_only: false,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        }
    }

    #[test]
    fn resolve_returns_binding_and_object() {
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
            .add_canonical_object(
                "file:///ws/README.md",
                "aureline-ws://ws-aureline-primary/root-1/README.md",
                NormalizationForm::MixedObserved,
                "dev:1/ino:1",
                1,
                vec![],
                PermissionSnapshot::writable_default(),
                vec![],
                b"hello".to_vec(),
            )
            .add_presentation(
                "file:///ws/README.md",
                "README.md",
                "file:///ws/README.md",
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();
        let resolved = root.resolve("file:///ws/README.md").unwrap();
        assert_eq!(resolved.object.canonical_uri, "file:///ws/README.md");
    }

    #[test]
    fn apply_external_change_bumps_generation() {
        let mut root =
            SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, posix_flags())
                .add_canonical_object(
                    "file:///ws/lib.rs",
                    "aureline-ws://ws-aureline-primary/root-1/lib.rs",
                    NormalizationForm::MixedObserved,
                    "dev:1/ino:2",
                    3,
                    vec![],
                    PermissionSnapshot::writable_default(),
                    vec![],
                    b"fn main() {}".to_vec(),
                )
                .add_presentation(
                    "file:///ws/lib.rs",
                    "lib.rs",
                    "file:///ws/lib.rs",
                    None,
                    vec!["-> canonical".to_owned()],
                )
                .build();
        let before = root.read_strongest_token("file:///ws/lib.rs").unwrap();
        let after_gen = root.apply_external_change("file:///ws/lib.rs").unwrap();
        let after = root.read_strongest_token("file:///ws/lib.rs").unwrap();
        assert_eq!(after_gen, 4);
        assert_ne!(before.value, after.value);
    }
}
