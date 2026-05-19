//! Browser-runtime session-origin descriptor.
//!
//! Mirrors the boundary schema at
//! `/schemas/browser_runtime/session_origin.schema.json`. The descriptor is
//! the cross-surface answer to "which browser session is producing this
//! view, and what cross-origin / protocol limits apply when we inspect it."

use serde::{Deserialize, Serialize};

use super::{PreviewOriginFinding, SourceMappingDescriptor, SourceMappingQualityClass};

/// Stable record-kind tag.
pub const BROWSER_SESSION_ORIGIN_RECORD_KIND: &str = "browser_runtime_session_origin_record";

/// Schema version mirrored by
/// `/schemas/browser_runtime/session_origin.schema.json#browser_runtime_session_origin_record`.
pub const BROWSER_SESSION_ORIGIN_SCHEMA_VERSION: u32 = 1;

/// Closed browser-session-origin vocabulary. Names which kind of session
/// produced the inspectable target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserSessionOriginClass {
    /// Attached local browser — Aureline drives a local Chromium / Firefox
    /// / Safari instance over the workspace-bound transport.
    AttachedLocalBrowser,
    /// Embedded webview — a webview hosted inside Aureline's shell or an
    /// extension-host.
    EmbeddedWebview,
    /// Remote devtools bridge — Aureline talks to a remote browser
    /// runtime (containerised dev server, remote workspace, managed
    /// preview service) through a bridge.
    RemoteDevtoolsBridge,
    /// External handoff browser — the user opened the preview in their
    /// system browser; Aureline only retains the handoff record.
    ExternalHandoffBrowser,
    /// No session attached — the descriptor is published for
    /// completeness (e.g. a static evidence projection) but no live
    /// browser session is reachable.
    NoSessionAttached,
}

impl BrowserSessionOriginClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttachedLocalBrowser => "attached_local_browser",
            Self::EmbeddedWebview => "embedded_webview",
            Self::RemoteDevtoolsBridge => "remote_devtools_bridge",
            Self::ExternalHandoffBrowser => "external_handoff_browser",
            Self::NoSessionAttached => "no_session_attached",
        }
    }

    /// True when this session origin admits live runtime inspection (a
    /// real bridge / process is reachable).
    pub const fn admits_inspection(self) -> bool {
        matches!(
            self,
            Self::AttachedLocalBrowser | Self::EmbeddedWebview | Self::RemoteDevtoolsBridge
        )
    }

    /// True when this session origin admits mutation-capable actions.
    pub const fn admits_mutation(self) -> bool {
        matches!(
            self,
            Self::AttachedLocalBrowser | Self::EmbeddedWebview | Self::RemoteDevtoolsBridge
        )
    }
}

/// Closed session-scope vocabulary. Names the boundary that the session
/// is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserSessionScopeClass {
    PerTab,
    PerWindow,
    PerProfile,
    PerWebview,
    PerExtensionHost,
    HandoffOnly,
    NotApplicable,
}

impl BrowserSessionScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerTab => "per_tab",
            Self::PerWindow => "per_window",
            Self::PerProfile => "per_profile",
            Self::PerWebview => "per_webview",
            Self::PerExtensionHost => "per_extension_host",
            Self::HandoffOnly => "handoff_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed cross-origin posture vocabulary. Browser-runtime inspectors and
/// source jumps must keep cross-origin limits explicit; the descriptor
/// quotes the posture verbatim instead of silently downgrading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossOriginPostureClass {
    SameOrigin,
    CrossOriginAllowed,
    CrossOriginBlocked,
    CrossOriginPartiallyBlocked,
    MixedContentBlocked,
    NotApplicable,
}

impl CrossOriginPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameOrigin => "same_origin",
            Self::CrossOriginAllowed => "cross_origin_allowed",
            Self::CrossOriginBlocked => "cross_origin_blocked",
            Self::CrossOriginPartiallyBlocked => "cross_origin_partially_blocked",
            Self::MixedContentBlocked => "mixed_content_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when at least one inspection target is blocked by the
    /// cross-origin posture. Inspectors MUST disclose this.
    pub const fn implies_blocked_targets(self) -> bool {
        matches!(
            self,
            Self::CrossOriginBlocked
                | Self::CrossOriginPartiallyBlocked
                | Self::MixedContentBlocked
        )
    }
}

/// Closed protocol-posture vocabulary. Mirrors the cross-origin posture
/// for protocol-level limits (HTTPS vs HTTP, devtools-only protocols,
/// etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolPostureClass {
    SecureContext,
    InsecureContextLocal,
    InsecureContextRemoteDowngraded,
    DevtoolsProtocolOnly,
    BridgeOnlyNoProtocol,
    NotApplicable,
}

impl ProtocolPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecureContext => "secure_context",
            Self::InsecureContextLocal => "insecure_context_local",
            Self::InsecureContextRemoteDowngraded => "insecure_context_remote_downgraded",
            Self::DevtoolsProtocolOnly => "devtools_protocol_only",
            Self::BridgeOnlyNoProtocol => "bridge_only_no_protocol",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Canonical browser-runtime session-origin record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeSessionOrigin {
    pub record_kind: String,
    pub browser_runtime_session_origin_schema_version: u32,
    pub browser_runtime_session_origin_id: String,

    /// Opaque ref to the owning preview-origin descriptor.
    pub preview_origin_descriptor_ref: String,
    /// Opaque ref to the owning preview-target descriptor.
    pub preview_target_descriptor_ref: String,

    pub session_origin_class: BrowserSessionOriginClass,
    pub session_scope_class: BrowserSessionScopeClass,
    pub cross_origin_posture: CrossOriginPostureClass,
    pub protocol_posture: ProtocolPostureClass,

    /// Source-mapping quality the inspector should advertise on jumps
    /// from this session. Browser-runtime inspectors MUST quote this on
    /// the source-jump button row.
    pub source_mapping: SourceMappingDescriptor,

    /// Opaque handle for the underlying session (tab id family, webview
    /// id family, bridge id family). Raw URLs / cookies / origin
    /// hostnames never appear here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_handle_ref: Option<String>,
    /// Opaque ref to the handoff record when the session was handed off
    /// to the system browser (the user's external browser took over).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff_record_ref: Option<String>,

    /// Reviewer-facing redacted label.
    pub redacted_session_label: String,
    pub summary: String,
}

impl BrowserRuntimeSessionOrigin {
    pub fn validate(&self) -> Vec<PreviewOriginFinding> {
        let mut findings = Vec::new();
        let subject = self.browser_runtime_session_origin_id.as_str();

        if self.record_kind != BROWSER_SESSION_ORIGIN_RECORD_KIND {
            findings.push(PreviewOriginFinding::new(
                "browser_runtime_session_origin.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    BROWSER_SESSION_ORIGIN_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.browser_runtime_session_origin_schema_version
            != BROWSER_SESSION_ORIGIN_SCHEMA_VERSION
        {
            findings.push(PreviewOriginFinding::new(
                "browser_runtime_session_origin.schema_version",
                subject,
                format!(
                    "schema_version must be {}, found {}",
                    BROWSER_SESSION_ORIGIN_SCHEMA_VERSION,
                    self.browser_runtime_session_origin_schema_version
                ),
            ));
        }

        // External handoff sessions cannot also publish an active session
        // handle — they have only a handoff record.
        if matches!(
            self.session_origin_class,
            BrowserSessionOriginClass::ExternalHandoffBrowser
        ) {
            if self.session_handle_ref.is_some() {
                findings.push(PreviewOriginFinding::new(
                    "browser_runtime_session_origin.handoff_forbids_session_handle",
                    subject,
                    "external_handoff_browser must not publish a live session_handle_ref",
                ));
            }
            if self.handoff_record_ref.is_none() {
                findings.push(PreviewOriginFinding::new(
                    "browser_runtime_session_origin.handoff_requires_handoff_record",
                    subject,
                    "external_handoff_browser requires a non-null handoff_record_ref",
                ));
            }
            if self.session_scope_class != BrowserSessionScopeClass::HandoffOnly {
                findings.push(PreviewOriginFinding::new(
                    "browser_runtime_session_origin.handoff_scope",
                    subject,
                    "external_handoff_browser requires session_scope_class = handoff_only",
                ));
            }
        }

        // NoSessionAttached requires not_applicable scope and no live
        // handles.
        if matches!(
            self.session_origin_class,
            BrowserSessionOriginClass::NoSessionAttached
        ) {
            if self.session_handle_ref.is_some() {
                findings.push(PreviewOriginFinding::new(
                    "browser_runtime_session_origin.no_session_forbids_handle",
                    subject,
                    "no_session_attached cannot publish a session_handle_ref",
                ));
            }
            if self.cross_origin_posture != CrossOriginPostureClass::NotApplicable {
                findings.push(PreviewOriginFinding::new(
                    "browser_runtime_session_origin.no_session_cross_origin",
                    subject,
                    "no_session_attached requires cross_origin_posture = not_applicable",
                ));
            }
            if self.protocol_posture != ProtocolPostureClass::NotApplicable {
                findings.push(PreviewOriginFinding::new(
                    "browser_runtime_session_origin.no_session_protocol",
                    subject,
                    "no_session_attached requires protocol_posture = not_applicable",
                ));
            }
        }

        // Inspection-capable sessions need a real session handle.
        if self.session_origin_class.admits_inspection() && self.session_handle_ref.is_none() {
            findings.push(PreviewOriginFinding::new(
                "browser_runtime_session_origin.inspection_requires_session_handle",
                subject,
                "inspectable sessions must publish a session_handle_ref",
            ));
        }

        // Cross-origin posture honesty: if the posture implies blocked
        // targets, the source mapping cannot claim exact / heuristic on a
        // jump that has no admissible target — surface MUST set mapping
        // to partial / unavailable. (This is the explicit "cross-origin
        // limits preserved" invariant.)
        if self.cross_origin_posture.implies_blocked_targets()
            && matches!(
                self.source_mapping.source_mapping_quality_class,
                SourceMappingQualityClass::Exact
            )
        {
            findings.push(PreviewOriginFinding::new(
                "browser_runtime_session_origin.cross_origin_blocked_exact_jump",
                subject,
                "blocked / partially-blocked cross-origin posture cannot claim exact source mapping for jumps",
            ));
        }

        // Mapping descriptor invariants.
        findings.extend(self.source_mapping.validate(subject));

        findings
    }

    /// True when an inspection-mutation action against this session is
    /// admissible at all.
    pub fn admits_mutation(&self) -> bool {
        self.session_origin_class.admits_mutation()
    }

    /// Render a deterministic plaintext summary safe to embed in support
    /// exports.
    pub fn render_plaintext(&self) -> String {
        format!(
            "browser_session {id} origin={origin} scope={scope} cross_origin={co} protocol={proto} mapping={map}: {summary}",
            id = self.browser_runtime_session_origin_id,
            origin = self.session_origin_class.as_str(),
            scope = self.session_scope_class.as_str(),
            co = self.cross_origin_posture.as_str(),
            proto = self.protocol_posture.as_str(),
            map = self.source_mapping.source_mapping_quality_class.as_str(),
            summary = self.summary,
        )
    }
}
