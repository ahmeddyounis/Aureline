//! Metadata-only helper and remote-agent capability negotiation.
//!
//! This module sits above the RPC method manifest and below shell drift
//! projection. It never starts a helper process or opens a transport; it
//! compares two advertised manifests, applies the supplied compatibility
//! window, and returns an inspectable response that downstream surfaces can
//! turn into drift truth or support exports.

use std::collections::BTreeSet;
use std::fmt;

use aureline_rpc::MethodManifest;
use serde::{Deserialize, Serialize};

/// Schema version for helper capability negotiation response records.
pub const HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION: u32 = 1;

/// Requiredness of a capability requested from a helper or remote agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRequirementClass {
    /// The requested surface cannot claim support without this capability.
    Required,
    /// The surface may proceed in a narrower posture when this capability is absent.
    Optional,
    /// The capability improves the experience but is not part of the support contract.
    NiceToHave,
}

impl CapabilityRequirementClass {
    /// Stable token used by manifests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::NiceToHave => "nice_to_have",
        }
    }

    /// Parses a manifest token into a requirement class.
    pub fn from_token(token: &str) -> Result<Self, CapabilityNegotiationParseError> {
        match token {
            "required" => Ok(Self::Required),
            "optional" => Ok(Self::Optional),
            "nice_to_have" => Ok(Self::NiceToHave),
            actual => Err(CapabilityNegotiationParseError::UnknownRequirement {
                actual: actual.to_owned(),
            }),
        }
    }
}

/// Runtime effect guarded by a helper capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityEffectClass {
    /// Read-only helper access, such as file inspection or search.
    ReadOnly,
    /// Local write behavior that does not mutate the remote target.
    LocalWrite,
    /// Remote file or workspace mutation.
    RemoteWrite,
    /// Network access mediated by the helper.
    Network,
    /// Process execution, including task and test commands.
    Process,
    /// Interactive terminal capability.
    Terminal,
    /// Debug adapter or debugger-control capability.
    Debug,
    /// Helper-backed AI runtime capability.
    AiRuntime,
    /// Managed control-plane mutation capability.
    ManagedControlPlaneWrite,
    /// Review-only inspection or preview capability.
    ReviewOnly,
}

impl CapabilityEffectClass {
    /// Stable token used by manifests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::LocalWrite => "local_write",
            Self::RemoteWrite => "remote_write",
            Self::Network => "network",
            Self::Process => "process",
            Self::Terminal => "terminal",
            Self::Debug => "debug",
            Self::AiRuntime => "ai_runtime",
            Self::ManagedControlPlaneWrite => "managed_control_plane_write",
            Self::ReviewOnly => "review_only",
        }
    }

    /// True when admitting this capability would allow helper-side mutation or execution.
    pub const fn is_mutating(self) -> bool {
        matches!(
            self,
            Self::RemoteWrite
                | Self::Network
                | Self::Process
                | Self::Terminal
                | Self::Debug
                | Self::AiRuntime
                | Self::ManagedControlPlaneWrite
        )
    }

    /// Parses a manifest token into an effect class.
    pub fn from_token(token: &str) -> Result<Self, CapabilityNegotiationParseError> {
        match token {
            "read_only" => Ok(Self::ReadOnly),
            "local_write" => Ok(Self::LocalWrite),
            "remote_write" => Ok(Self::RemoteWrite),
            "network" => Ok(Self::Network),
            "process" => Ok(Self::Process),
            "terminal" => Ok(Self::Terminal),
            "debug" => Ok(Self::Debug),
            "ai_runtime" => Ok(Self::AiRuntime),
            "managed_control_plane_write" => Ok(Self::ManagedControlPlaneWrite),
            "review_only" => Ok(Self::ReviewOnly),
            actual => Err(CapabilityNegotiationParseError::UnknownEffect {
                actual: actual.to_owned(),
            }),
        }
    }
}

/// Compatibility status supplied by the machine-readable skew window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityWindowStatus {
    /// The helper pairing is inside the supported window.
    Supported,
    /// The helper pairing is allowed only with a narrowed capability posture.
    BestEffort,
    /// The helper pairing needs a probe or reattach before support can be claimed.
    Untested,
    /// The helper pairing is outside the supported window and must fail closed.
    Unsupported,
}

impl CompatibilityWindowStatus {
    /// Stable token used by manifests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::BestEffort => "best_effort",
            Self::Untested => "untested",
            Self::Unsupported => "unsupported",
        }
    }

    /// Parses a manifest token into a compatibility status.
    pub fn from_token(token: &str) -> Result<Self, CapabilityNegotiationParseError> {
        match token {
            "supported" => Ok(Self::Supported),
            "best_effort" => Ok(Self::BestEffort),
            "untested" => Ok(Self::Untested),
            "unsupported" => Ok(Self::Unsupported),
            actual => Err(CapabilityNegotiationParseError::UnknownWindowStatus {
                actual: actual.to_owned(),
            }),
        }
    }
}

/// Effective helper posture after negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveCapabilityPosture {
    /// Full remote capability is admitted.
    FullRemote,
    /// Review-only remote inspection is admitted.
    ReviewOnly,
    /// File-only remote inspection is admitted.
    FileOnly,
    /// Inspect-only metadata and read paths are admitted.
    InspectOnly,
    /// Local-only continuation is admitted.
    LocalOnly,
    /// No helper-backed capability is admitted.
    Blocked,
}

impl EffectiveCapabilityPosture {
    /// Stable token used by manifests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRemote => "full_remote",
            Self::ReviewOnly => "review_only",
            Self::FileOnly => "file_only",
            Self::InspectOnly => "inspect_only",
            Self::LocalOnly => "local_only",
            Self::Blocked => "blocked",
        }
    }

    /// True when this posture can carry remote mutation authority.
    pub const fn allows_remote_mutation(self) -> bool {
        matches!(self, Self::FullRemote)
    }

    /// Parses a manifest token into an effective posture.
    pub fn from_token(token: &str) -> Result<Self, CapabilityNegotiationParseError> {
        match token {
            "full_remote" => Ok(Self::FullRemote),
            "review_only" => Ok(Self::ReviewOnly),
            "file_only" => Ok(Self::FileOnly),
            "inspect_only" => Ok(Self::InspectOnly),
            "local_only" => Ok(Self::LocalOnly),
            "blocked" => Ok(Self::Blocked),
            actual => Err(CapabilityNegotiationParseError::UnknownPosture {
                actual: actual.to_owned(),
            }),
        }
    }
}

/// Typed reason a requested capability was not admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingCapabilityReasonClass {
    /// The helper manifest does not advertise the capability.
    HelperDoesNotOffer,
    /// The client requested a capability outside its own manifest vocabulary.
    ClientRequiresUnknownFeature,
    /// The compatibility window blocks the capability.
    OutsideSkewWindow,
    /// The selected protocol does not meet the capability floor.
    ProtocolFloorMismatch,
    /// Policy narrowed the effective capability set.
    PolicyNarrowed,
    /// Trust checks did not verify the capability boundary.
    TrustNotVerified,
    /// A probe must run before the capability can be admitted.
    ProbeRequired,
}

impl MissingCapabilityReasonClass {
    /// Stable token used by manifests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelperDoesNotOffer => "helper_does_not_offer",
            Self::ClientRequiresUnknownFeature => "client_requires_unknown_feature",
            Self::OutsideSkewWindow => "outside_skew_window",
            Self::ProtocolFloorMismatch => "protocol_floor_mismatch",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::TrustNotVerified => "trust_not_verified",
            Self::ProbeRequired => "probe_required",
        }
    }
}

/// Result of matching a helper request against a helper manifest and compatibility window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegotiationOutcome {
    /// The required capability set is admitted without narrowing.
    Match,
    /// A metadata-safe narrowed posture is available.
    Downgrade,
    /// Helper-backed capability is refused until repair, probe, or reattach.
    Refuse,
}

impl NegotiationOutcome {
    /// Stable token used by manifests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Downgrade => "downgrade",
            Self::Refuse => "refuse",
        }
    }
}

/// One capability requested by a shell or helper-backed runtime surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelperCapabilityRequirement {
    /// Capability or method name from the RPC manifest vocabulary.
    pub capability: String,
    /// Whether the requested surface requires this capability.
    pub requirement: CapabilityRequirementClass,
    /// Runtime effect guarded by this capability.
    pub effect_class: CapabilityEffectClass,
    /// Redaction-safe reason shown when the capability is missing.
    pub visible_reason: String,
}

impl HelperCapabilityRequirement {
    /// True when the capability would admit helper-side mutation or execution.
    pub const fn is_mutating(&self) -> bool {
        self.effect_class.is_mutating()
    }
}

/// Capability that was requested but not admitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DroppedHelperCapability {
    /// Capability or method name that was dropped.
    pub capability: String,
    /// Typed reason for dropping the capability.
    pub reason_class: MissingCapabilityReasonClass,
    /// Redaction-safe visible reason.
    pub visible_reason: String,
    /// True when a probe, refresh, upgrade, or reattach may admit the capability later.
    pub retryable: bool,
}

/// Compatibility refs imported from the mixed-version and skew-window manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityWindow {
    /// Boundary family from the mixed-version compatibility matrix.
    pub boundary_family: String,
    /// Compatibility row that owns the boundary.
    pub compatibility_row_ref: String,
    /// Version-skew register bound to the boundary.
    pub version_skew_register_ref: String,
    /// Concrete skew case selected for this negotiation.
    pub skew_case_ref: String,
    /// Skew-window declaration selected for the boundary.
    pub skew_window_declaration_ref: String,
    /// Status supplied by the skew-window manifest.
    pub status: CompatibilityWindowStatus,
    /// Protocol or contract ref selected by the negotiation source manifest.
    pub selected_protocol_ref: String,
    /// Schema, fixture, doc, or artifact refs that support this window.
    #[serde(default)]
    pub source_refs: Vec<String>,
}

impl CompatibilityWindow {
    /// True when all refs required by shell drift truth are present.
    pub fn has_required_refs(&self) -> bool {
        !self.boundary_family.trim().is_empty()
            && !self.compatibility_row_ref.trim().is_empty()
            && !self.version_skew_register_ref.trim().is_empty()
            && !self.skew_case_ref.trim().is_empty()
            && !self.skew_window_declaration_ref.trim().is_empty()
            && !self.selected_protocol_ref.trim().is_empty()
    }
}

/// Metadata-only request for helper or remote-agent capability negotiation.
#[derive(Debug, Clone)]
pub struct HelperCapabilityRequest {
    /// Stable request or envelope id.
    pub request_id: String,
    /// Stable drift row id for downstream surfaces.
    pub row_id: String,
    /// Opaque source surface ref.
    pub surface_ref: String,
    /// Short title for review and support surfaces.
    pub title: String,
    /// Client-side RPC method manifest.
    pub client_manifest: MethodManifest,
    /// Helper or remote-agent RPC method manifest.
    pub helper_manifest: MethodManifest,
    /// Capabilities needed by the surface.
    pub requested_capabilities: Vec<HelperCapabilityRequirement>,
    /// Compatibility window selected from machine-readable manifests.
    pub compatibility_window: CompatibilityWindow,
    /// Posture to use when the negotiation narrows but does not fully refuse.
    pub downgrade_posture: EffectiveCapabilityPosture,
    /// Posture to use when helper capability is refused.
    pub refusal_posture: EffectiveCapabilityPosture,
    /// Redaction-safe summary supplied by the source manifest.
    pub visible_summary: String,
    /// Redaction-safe safe-continuation summary supplied by the source manifest.
    pub safe_continuation: String,
    /// Recovery, repair, probe, or continuation refs for refused and downgraded outcomes.
    pub recovery_refs: Vec<String>,
    /// Mutations or actions blocked by a downgraded or refused outcome.
    pub blocked_action_refs: Vec<String>,
    /// Read-only refs preserved while the helper boundary is narrowed.
    pub preserved_read_only_refs: Vec<String>,
    /// Optional retry or probe ref used by retry-required cases.
    pub retry_ref: Option<String>,
    /// Redaction-safe support packet refs.
    pub support_packet_refs: Vec<String>,
    /// Redaction-safe review packet refs.
    pub review_packet_refs: Vec<String>,
    /// Additional refs local to this negotiation.
    pub source_refs: Vec<String>,
}

impl HelperCapabilityRequest {
    /// Negotiates the request against the helper manifest and compatibility window.
    pub fn negotiate(&self) -> HelperCapabilityResponse {
        let negotiated_capabilities = self.negotiated_capabilities();
        let dropped_capabilities = self.dropped_capabilities(&negotiated_capabilities);
        let missing_required = self.missing_required_capabilities(&negotiated_capabilities);

        let outcome = match self.compatibility_window.status {
            CompatibilityWindowStatus::Unsupported | CompatibilityWindowStatus::Untested => {
                NegotiationOutcome::Refuse
            }
            CompatibilityWindowStatus::BestEffort => NegotiationOutcome::Downgrade,
            CompatibilityWindowStatus::Supported => {
                if missing_required.is_empty() {
                    NegotiationOutcome::Match
                } else if negotiated_capabilities.is_empty() {
                    NegotiationOutcome::Refuse
                } else {
                    NegotiationOutcome::Downgrade
                }
            }
        };

        let effective_posture = match outcome {
            NegotiationOutcome::Match => EffectiveCapabilityPosture::FullRemote,
            NegotiationOutcome::Downgrade => self.downgrade_posture,
            NegotiationOutcome::Refuse => self.refusal_posture,
        };

        let mutation_allowed = outcome == NegotiationOutcome::Match
            && effective_posture.allows_remote_mutation()
            && self
                .requested_capabilities
                .iter()
                .filter(|request| request.is_mutating())
                .all(|request| negotiated_capabilities.contains(&request.capability));

        HelperCapabilityResponse {
            schema_version: HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
            request_id: self.request_id.clone(),
            row_id: self.row_id.clone(),
            surface_ref: self.surface_ref.clone(),
            title: self.title.clone(),
            outcome,
            selected_protocol_ref: self.compatibility_window.selected_protocol_ref.clone(),
            negotiated_capabilities,
            dropped_capabilities,
            mutation_allowed,
            effective_posture,
            visible_summary: self.visible_summary.clone(),
            safe_continuation: self.safe_continuation.clone(),
            primary_recovery_ref: self.recovery_refs.first().cloned(),
            recovery_refs: self.recovery_refs.clone(),
            blocked_action_refs: self.blocked_action_refs.clone(),
            preserved_read_only_refs: self.preserved_read_only_refs.clone(),
            retry_ref: self.retry_ref.clone(),
            support_packet_refs: self.support_packet_refs.clone(),
            review_packet_refs: self.review_packet_refs.clone(),
            source_refs: self.source_refs.clone(),
            client_manifest_digest: self.client_manifest.digest.0.clone(),
            helper_manifest_digest: self.helper_manifest.digest.0.clone(),
            compatibility_window: self.compatibility_window.clone(),
        }
    }

    fn negotiated_capabilities(&self) -> Vec<String> {
        if self.compatibility_window.status == CompatibilityWindowStatus::Unsupported {
            return Vec::new();
        }

        self.requested_capabilities
            .iter()
            .filter(|request| {
                self.client_manifest.supports_method(&request.capability)
                    && self.helper_manifest.supports_method(&request.capability)
            })
            .filter(|request| {
                self.compatibility_window.status == CompatibilityWindowStatus::Supported
                    || !request.is_mutating()
            })
            .map(|request| request.capability.clone())
            .collect()
    }

    fn dropped_capabilities(
        &self,
        negotiated_capabilities: &[String],
    ) -> Vec<DroppedHelperCapability> {
        let negotiated = negotiated_capabilities
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();

        self.requested_capabilities
            .iter()
            .filter(|request| !negotiated.contains(request.capability.as_str()))
            .map(|request| DroppedHelperCapability {
                capability: request.capability.clone(),
                reason_class: self.drop_reason_for(request),
                visible_reason: request.visible_reason.clone(),
                retryable: self.compatibility_window.status
                    != CompatibilityWindowStatus::Unsupported,
            })
            .collect()
    }

    fn missing_required_capabilities(&self, negotiated_capabilities: &[String]) -> Vec<String> {
        let negotiated = negotiated_capabilities
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        self.requested_capabilities
            .iter()
            .filter(|request| request.requirement == CapabilityRequirementClass::Required)
            .filter(|request| !negotiated.contains(request.capability.as_str()))
            .map(|request| request.capability.clone())
            .collect()
    }

    fn drop_reason_for(
        &self,
        request: &HelperCapabilityRequirement,
    ) -> MissingCapabilityReasonClass {
        match self.compatibility_window.status {
            CompatibilityWindowStatus::Unsupported => {
                MissingCapabilityReasonClass::OutsideSkewWindow
            }
            CompatibilityWindowStatus::Untested => MissingCapabilityReasonClass::ProbeRequired,
            CompatibilityWindowStatus::BestEffort if request.is_mutating() => {
                MissingCapabilityReasonClass::OutsideSkewWindow
            }
            _ if !self.client_manifest.supports_method(&request.capability) => {
                MissingCapabilityReasonClass::ClientRequiresUnknownFeature
            }
            _ if !self.helper_manifest.supports_method(&request.capability) => {
                MissingCapabilityReasonClass::HelperDoesNotOffer
            }
            _ => MissingCapabilityReasonClass::ProtocolFloorMismatch,
        }
    }
}

/// Metadata-only response produced by helper capability negotiation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelperCapabilityResponse {
    /// Response schema version.
    pub schema_version: u32,
    /// Stable request or envelope id.
    pub request_id: String,
    /// Stable drift row id for downstream surfaces.
    pub row_id: String,
    /// Opaque source surface ref.
    pub surface_ref: String,
    /// Short title for review and support surfaces.
    pub title: String,
    /// Negotiation result.
    pub outcome: NegotiationOutcome,
    /// Protocol or contract ref selected by the negotiation source manifest.
    pub selected_protocol_ref: String,
    /// Capability intersection admitted for this response.
    pub negotiated_capabilities: Vec<String>,
    /// Requested capabilities that were not admitted.
    pub dropped_capabilities: Vec<DroppedHelperCapability>,
    /// True when remote mutation authority is admitted.
    pub mutation_allowed: bool,
    /// Effective helper posture after negotiation.
    pub effective_posture: EffectiveCapabilityPosture,
    /// Redaction-safe visible summary.
    pub visible_summary: String,
    /// Redaction-safe continuation summary.
    pub safe_continuation: String,
    /// Primary recovery ref for refused or downgraded outcomes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_recovery_ref: Option<String>,
    /// Recovery, repair, probe, or continuation refs.
    #[serde(default)]
    pub recovery_refs: Vec<String>,
    /// Mutations or actions blocked by this response.
    #[serde(default)]
    pub blocked_action_refs: Vec<String>,
    /// Read-only refs preserved while the helper boundary is narrowed.
    #[serde(default)]
    pub preserved_read_only_refs: Vec<String>,
    /// Retry or probe ref for retry-required cases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_ref: Option<String>,
    /// Redaction-safe support packet refs.
    #[serde(default)]
    pub support_packet_refs: Vec<String>,
    /// Redaction-safe review packet refs.
    #[serde(default)]
    pub review_packet_refs: Vec<String>,
    /// Additional refs local to this negotiation.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// Digest of the client RPC manifest used by the negotiation.
    pub client_manifest_digest: String,
    /// Digest of the helper RPC manifest used by the negotiation.
    pub helper_manifest_digest: String,
    /// Compatibility refs imported from skew-window manifests.
    pub compatibility_window: CompatibilityWindow,
}

impl HelperCapabilityResponse {
    /// True when helper-backed capability was refused.
    pub const fn is_refused(&self) -> bool {
        matches!(self.outcome, NegotiationOutcome::Refuse)
    }

    /// True when a response carries only metadata and refs.
    pub fn is_metadata_only(&self) -> bool {
        self.compatibility_window.has_required_refs()
            && !self.client_manifest_digest.trim().is_empty()
            && !self.helper_manifest_digest.trim().is_empty()
    }
}

/// Parse error for manifest-backed capability negotiation tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityNegotiationParseError {
    /// Requirement token is not part of the closed vocabulary.
    UnknownRequirement { actual: String },
    /// Effect token is not part of the closed vocabulary.
    UnknownEffect { actual: String },
    /// Compatibility-window status token is not part of the closed vocabulary.
    UnknownWindowStatus { actual: String },
    /// Effective-posture token is not part of the closed vocabulary.
    UnknownPosture { actual: String },
}

impl fmt::Display for CapabilityNegotiationParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRequirement { actual } => {
                write!(f, "unknown capability requirement {actual}")
            }
            Self::UnknownEffect { actual } => write!(f, "unknown capability effect {actual}"),
            Self::UnknownWindowStatus { actual } => {
                write!(f, "unknown compatibility-window status {actual}")
            }
            Self::UnknownPosture { actual } => {
                write!(f, "unknown effective capability posture {actual}")
            }
        }
    }
}

impl std::error::Error for CapabilityNegotiationParseError {}
