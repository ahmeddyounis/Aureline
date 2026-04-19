//! Method-manifest types.
//!
//! The manifest is the vocabulary used by the capability-negotiation
//! handshake. The Rust registry is the source of record; the JSON
//! Schema at `schemas/rpc/method_manifest.schema.json` pins the shape
//! that external tooling validates against.

use std::collections::BTreeMap;

use crate::envelope::{ActorClass, ContractVersion, DeliveryMode, MethodName};
use crate::errors::ErrorClass;

/// Required scope for a method or event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Workspace,
    Global,
    Either,
}

impl ScopeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Global => "global",
            Self::Either => "either",
        }
    }
}

/// Method shape: unary, server-streaming, or long-lived subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MethodKind {
    Unary,
    ServerStream,
    Subscription,
}

/// Content digest of the canonicalized manifest bytes. The handshake
/// exchanges and pins this value so peers can detect drift without
/// re-exchanging the full manifest.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestDigest(pub String);

#[derive(Debug, Clone)]
pub struct MethodEntry {
    pub name: MethodName,
    pub kind: MethodKind,
    pub scope: ScopeKind,
    pub actor_classes: Vec<ActorClass>,
    pub contract_versions: Vec<ContractVersion>,
    /// Default deadline in nanoseconds. Zero means "no default".
    pub default_deadline_ns: u64,
    /// Maximum deadline in nanoseconds. Zero means "no upper bound",
    /// legal only for subscriptions.
    pub max_deadline_ns: u64,
    pub deadline_required: bool,
    pub idempotency: Idempotency,
    pub error_classes: Vec<ErrorClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Idempotency {
    Required,
    Recommended,
    NotApplicable,
}

#[derive(Debug, Clone)]
pub struct EventEntry {
    pub kind: String,
    pub scope: ScopeKind,
    pub delivery_mode: DeliveryMode,
    pub schema_versions: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct MethodManifest {
    pub service: String,
    pub service_version: ContractVersion,
    pub digest: ManifestDigest,
    pub methods: BTreeMap<String, MethodEntry>,
    pub events: BTreeMap<String, EventEntry>,
}

impl MethodManifest {
    pub fn new(
        service: impl Into<String>,
        service_version: ContractVersion,
        digest: ManifestDigest,
    ) -> Self {
        Self {
            service: service.into(),
            service_version,
            digest,
            methods: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn with_method(mut self, entry: MethodEntry) -> Self {
        self.methods.insert(entry.name.as_str().to_string(), entry);
        self
    }

    pub fn with_event(mut self, entry: EventEntry) -> Self {
        self.events.insert(entry.kind.clone(), entry);
        self
    }

    pub fn method(&self, name: &str) -> Option<&MethodEntry> {
        self.methods.get(name)
    }

    pub fn supports_method(&self, name: &str) -> bool {
        self.methods.contains_key(name)
    }

    /// Return the capability intersection of two manifests: the set of
    /// method names both sides serve. The handshake chooses the
    /// intersection; a missing method on either side is
    /// `unavailable`, not silently downgraded.
    pub fn intersect_methods<'a>(&'a self, peer: &'a Self) -> Vec<&'a str> {
        self.methods
            .keys()
            .filter(|k| peer.methods.contains_key(k.as_str()))
            .map(|k| k.as_str())
            .collect()
    }
}
