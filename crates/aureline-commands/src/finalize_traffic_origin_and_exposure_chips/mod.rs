//! Traffic-origin and exposure chips, tunnel, port, and publish-target
//! explainability packet.
//!
//! This module finalizes the traffic-origin and endpoint-exposure lane into
//! one export-safe packet that binds — for a claimed stable command or
//! network-action family — the five explainability facts a route-explicit and
//! exposure-explicit lane must surface without a debug or admin toggle:
//!
//! - the **traffic-origin chip set** — for each declared traffic-origin class
//!   (`local_process`, `loopback_client`, `tunnel_ingress`,
//!   `port_forward_ingress`, `publish_target_relay`, `provider_callback`,
//!   `browser_companion_relay`) a chip is visible in preview, approval,
//!   active-session status, and support export, bound to the named origin and
//!   never inferred from ambient context;
//! - the **exposure chip set** — for each exposed endpoint the exposure class
//!   (`unexposed`, `localhost_only`, `port_forwarded`, `tunnel_exposed`,
//!   `publicly_published`, `provider_managed`, `enterprise_gateway`) is
//!   surfaced in the chip object, never implicit; any elevation from a narrower
//!   class to a wider one forces visible reapproval rather than a silent state
//!   change;
//! - the **tunnel explainability record** — for each active or pending tunnel a
//!   single structured record names the tunnel kind, the target-port ref
//!   (opaque), the traffic-origin class, the exposure class, the approval scope
//!   ref, and the drift-forces-reapproval invariant; the record is reachable
//!   from preview, approval, active-session UI, CLI, AI-tool run, and support
//!   export;
//! - the **port explainability record** — for each forwarded or exposed port a
//!   single structured record names the port ref (opaque), the protocol class,
//!   the exposure class, the traffic-origin class, and the chip-and-export
//!   disclosure state; and
//! - the **publish-target explainability record** — for each publish target a
//!   single structured record names the target class, the exposure class, the
//!   traffic-origin class, the approval scope ref, the spend-posture token, and
//!   the chip-and-export disclosure state.
//!
//! It does not re-derive the descriptor, registry, invocation, authority,
//! high-risk hardening, command-parity, or client-origin/route-class models.
//! The [`crate::stabilize_client_origin_route_class::ClientOriginRouteClassPacket`]
//! owns the capability-route inspector, approval scope, and cross-surface
//! inspector parity. This packet references it by stable schema ref and adds the
//! traffic-origin chip, exposure chip, tunnel, port, and publish-target
//! explainability invariants so every stable surface reads the same exposure
//! truth without inferring it from ambient context.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw port numbers, raw endpoint hostnames, raw
//! tunnel URLs, raw credentials, exact cost figures, and billing-account ids
//! stay outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_command_contract::{
    CommandContractEvidenceExport, CommandSurfaceClass, StableContractRefs,
    SurfaceQualificationClass,
};

/// Stable record-kind tag carried by [`FinalizeTrafficOriginExposureChipsPacket`].
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_RECORD_KIND: &str =
    "traffic_origin_exposure_chips_finalization_packet";

/// Schema version for traffic-origin / exposure-chips finalization records.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the traffic-origin / exposure-chips boundary schema.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_REF: &str =
    "schemas/commands/finalize_traffic_origin_and_exposure_chips.schema.json";

/// Repo-relative path of the traffic-origin / exposure-chips doc.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DOC_REF: &str =
    "docs/commands/m4/finalize_traffic_origin_and_exposure_chips.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_FIXTURE_DIR: &str =
    "fixtures/commands/m4/finalize_traffic_origin_and_exposure_chips";

/// Repo-relative path of the checked-in support export artifact.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_ARTIFACT_REF: &str =
    "artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips/support_export.json";

/// Repo-relative path of the checked-in Markdown summary.
pub const FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SUMMARY_REF: &str =
    "artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips/summary.md";

/// Origin class naming where a piece of traffic initiates.
///
/// These classes drive chip selection and disclosure depth. A command family
/// declares the subset of origin classes that can reach it; the packet does
/// not require every class on one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrafficOriginClass {
    /// Traffic originates within the local IDE process.
    LocalProcess,
    /// Traffic from a loopback client on the same device.
    LoopbackClient,
    /// Traffic arriving via an active tunnel (SSH, dev tunnel, reverse tunnel).
    TunnelIngress,
    /// Traffic arriving via a forwarded port.
    PortForwardIngress,
    /// Traffic relayed from a publish target (CDN, static host, managed service).
    PublishTargetRelay,
    /// Traffic initiated by a provider action callback.
    ProviderCallback,
    /// Traffic relayed via the browser companion extension.
    BrowserCompanionRelay,
}

impl TrafficOriginClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProcess => "local_process",
            Self::LoopbackClient => "loopback_client",
            Self::TunnelIngress => "tunnel_ingress",
            Self::PortForwardIngress => "port_forward_ingress",
            Self::PublishTargetRelay => "publish_target_relay",
            Self::ProviderCallback => "provider_callback",
            Self::BrowserCompanionRelay => "browser_companion_relay",
        }
    }
}

/// Exposure class naming how broadly an endpoint is accessible.
///
/// Elevation from a narrower class to a wider one must force visible
/// reapproval; silent class elevation is a policy violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExposureChipClass {
    /// Not listening; no external access possible.
    Unexposed,
    /// Bound to loopback only; accessible only on the same device.
    LocalhostOnly,
    /// Forwarded to a port reachable from the local network or a tunnel.
    PortForwarded,
    /// Exposed via a tunnel (SSH, dev tunnel, reverse tunnel).
    TunnelExposed,
    /// Published to a publicly accessible URL or endpoint.
    PubliclyPublished,
    /// Managed by a provider service and accessible on its terms.
    ProviderManaged,
    /// Exposed through an enterprise gateway.
    EnterpriseGateway,
}

impl ExposureChipClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unexposed => "unexposed",
            Self::LocalhostOnly => "localhost_only",
            Self::PortForwarded => "port_forwarded",
            Self::TunnelExposed => "tunnel_exposed",
            Self::PubliclyPublished => "publicly_published",
            Self::ProviderManaged => "provider_managed",
            Self::EnterpriseGateway => "enterprise_gateway",
        }
    }

    /// Returns `true` when the class implies external network reachability.
    pub const fn is_externally_reachable(self) -> bool {
        matches!(
            self,
            Self::TunnelExposed
                | Self::PubliclyPublished
                | Self::ProviderManaged
                | Self::EnterpriseGateway
        )
    }
}

/// Kind of tunnel surfaced in a [`TunnelExplainabilityRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TunnelKindClass {
    /// SSH local or remote port forward.
    SshForward,
    /// Built-in dev-tunnel service.
    DevTunnel,
    /// Reverse tunnel (e.g. an ngrok-style proxy).
    ReverseTunnel,
    /// Tunnel managed and provisioned by a provider.
    ProviderTunnel,
}

impl TunnelKindClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SshForward => "ssh_forward",
            Self::DevTunnel => "dev_tunnel",
            Self::ReverseTunnel => "reverse_tunnel",
            Self::ProviderTunnel => "provider_tunnel",
        }
    }
}

/// Network protocol of a port surfaced in a [`PortExplainabilityRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortProtocolClass {
    /// Plain TCP.
    Tcp,
    /// HTTP.
    Http,
    /// HTTPS (TLS-wrapped HTTP).
    Https,
    /// WebSocket.
    Ws,
    /// WebSocket Secure (TLS-wrapped WebSocket).
    Wss,
    /// UDP.
    Udp,
}

impl PortProtocolClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Http => "http",
            Self::Https => "https",
            Self::Ws => "ws",
            Self::Wss => "wss",
            Self::Udp => "udp",
        }
    }
}

/// Publish-target category surfaced in a [`PublishTargetExplainabilityRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishTargetClass {
    /// Static file host (pages service, CDN-backed static site).
    StaticHost,
    /// Container image registry.
    ContainerRegistry,
    /// Managed cloud service (serverless, PaaS, etc.).
    ManagedService,
    /// User-provided external host.
    ExternalHost,
    /// Push to a provider-defined destination.
    ProviderPush,
}

impl PublishTargetClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticHost => "static_host",
            Self::ContainerRegistry => "container_registry",
            Self::ManagedService => "managed_service",
            Self::ExternalHost => "external_host",
            Self::ProviderPush => "provider_push",
        }
    }
}

/// Structured explainability record for a single active or pending tunnel.
///
/// All fields carry opaque refs, state tokens, and coarse classes only. Raw
/// tunnel URLs, raw endpoint hostnames, raw port numbers, and credentials stay
/// outside the export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TunnelExplainabilityRecord {
    /// Stable opaque tunnel ref (never a raw URL or hostname).
    pub tunnel_ref: String,
    /// Kind of tunnel.
    pub tunnel_kind: TunnelKindClass,
    /// Opaque ref identifying the target port (never the raw port number).
    pub target_port_ref: String,
    /// Traffic-origin class for ingress on this tunnel.
    pub traffic_origin: TrafficOriginClass,
    /// Exposure class for this tunnel.
    pub exposure: ExposureChipClass,
    /// Approval scope ref governing this tunnel's activation.
    pub approval_scope_ref: String,
    /// Display-safe owner label for the approval scope (not PII).
    pub approval_owner_label: String,
    /// True when exposure-class elevation forces visible reapproval.
    pub drift_forces_reapproval: bool,
    /// True when this tunnel is surfaced in active-session and preview chips.
    pub disclosed_in_chip: bool,
    /// True when this tunnel is disclosed in the approval UI.
    pub disclosed_in_preview: bool,
    /// True when this tunnel appears in the support export.
    pub disclosed_in_support_export: bool,
}

impl TunnelExplainabilityRecord {
    fn guards_hold(&self) -> bool {
        !self.tunnel_ref.trim().is_empty()
            && !self.target_port_ref.trim().is_empty()
            && !self.approval_scope_ref.trim().is_empty()
            && !self.approval_owner_label.trim().is_empty()
            && self.drift_forces_reapproval
            && self.disclosed_in_chip
            && self.disclosed_in_preview
            && self.disclosed_in_support_export
    }
}

/// Structured explainability record for a single forwarded or exposed port.
///
/// All fields carry opaque refs and coarse classes only. The raw port number
/// stays outside the export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortExplainabilityRecord {
    /// Stable opaque port ref (never the raw port number).
    pub port_ref: String,
    /// Network protocol of this port.
    pub protocol: PortProtocolClass,
    /// Exposure class for this port.
    pub exposure: ExposureChipClass,
    /// Traffic-origin class for ingress on this port.
    pub traffic_origin: TrafficOriginClass,
    /// True when this port is surfaced in active-session and status chips.
    pub disclosed_in_chip: bool,
    /// True when this port appears in the support export.
    pub disclosed_in_support_export: bool,
}

impl PortExplainabilityRecord {
    fn guards_hold(&self) -> bool {
        !self.port_ref.trim().is_empty()
            && self.disclosed_in_chip
            && self.disclosed_in_support_export
    }
}

/// Structured explainability record for a single publish target.
///
/// All fields carry opaque refs and coarse classes only. Raw destination URLs,
/// credentials, and exact cost figures stay outside the export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishTargetExplainabilityRecord {
    /// Stable opaque publish-target ref (never a raw URL or hostname).
    pub publish_target_ref: String,
    /// Category of the publish target.
    pub target_class: PublishTargetClass,
    /// Exposure class for this publish target.
    pub exposure: ExposureChipClass,
    /// Traffic-origin class for traffic relayed via this publish target.
    pub traffic_origin: TrafficOriginClass,
    /// Approval scope ref governing publication to this target.
    pub approval_scope_ref: String,
    /// Spend/egress posture token for this publish target.
    pub spend_posture_token: String,
    /// True when this publish target is surfaced in preview and status chips.
    pub disclosed_in_chip: bool,
    /// True when this publish target appears in the support export.
    pub disclosed_in_support_export: bool,
}

impl PublishTargetExplainabilityRecord {
    fn guards_hold(&self) -> bool {
        !self.publish_target_ref.trim().is_empty()
            && !self.approval_scope_ref.trim().is_empty()
            && !self.spend_posture_token.trim().is_empty()
            && self.disclosed_in_chip
            && self.disclosed_in_support_export
    }
}

/// Cross-surface parity row for traffic-origin and exposure chips.
///
/// One row per [`CommandSurfaceClass`]; a stable surface must surface every
/// chip and must not widen authority beyond the descriptor claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrafficOriginChipRow {
    /// Command invocation surface this row covers.
    pub surface_class: CommandSurfaceClass,
    /// True when traffic-origin and exposure chips are visible on this surface.
    pub chip_visible: bool,
    /// True when the traffic-origin class is disclosed on this surface.
    pub discloses_origin_class: bool,
    /// True when the exposure class is disclosed on this surface.
    pub discloses_exposure_class: bool,
    /// True when tunnel, port, and publish-target explainability is surfaced.
    pub discloses_network_explainability: bool,
    /// True when policy checks are enforced on this surface.
    pub policy_checked: bool,
    /// True when this surface never widens traffic authority beyond the descriptor.
    pub no_authority_widening: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl TrafficOriginChipRow {
    fn preserves_full_disclosure(&self) -> bool {
        self.chip_visible
            && self.discloses_origin_class
            && self.discloses_exposure_class
            && self.discloses_network_explainability
            && self.policy_checked
            && self.no_authority_widening
    }
}

/// Constructor input for [`FinalizeTrafficOriginExposureChipsPacket::new`].
#[derive(Debug, Clone, PartialEq)]
pub struct FinalizeTrafficOriginExposureChipsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Command or action family id this packet covers.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// Traffic-origin classes declared by this command family.
    pub traffic_origin_classes: Vec<TrafficOriginClass>,
    /// Exposure classes this command family can reach.
    pub exposure_classes: Vec<ExposureChipClass>,
    /// Tunnel explainability records for this family.
    pub tunnel_records: Vec<TunnelExplainabilityRecord>,
    /// Port explainability records for this family.
    pub port_records: Vec<PortExplainabilityRecord>,
    /// Publish-target explainability records for this family.
    pub publish_target_records: Vec<PublishTargetExplainabilityRecord>,
    /// Cross-surface chip parity rows.
    pub chip_surface_rows: Vec<TrafficOriginChipRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe traffic-origin and exposure chips finalization record.
///
/// Binds the traffic-origin chip set, exposure chip set, tunnel/port/publish-
/// target explainability records, and cross-surface chip parity for one
/// command or network-action family on a claimed stable row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeTrafficOriginExposureChipsPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Command or action family id this packet covers.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// Traffic-origin classes declared by this command family.
    pub traffic_origin_classes: Vec<TrafficOriginClass>,
    /// Exposure classes this command family can reach.
    pub exposure_classes: Vec<ExposureChipClass>,
    /// Tunnel explainability records for this family.
    pub tunnel_records: Vec<TunnelExplainabilityRecord>,
    /// Port explainability records for this family.
    pub port_records: Vec<PortExplainabilityRecord>,
    /// Publish-target explainability records for this family.
    pub publish_target_records: Vec<PublishTargetExplainabilityRecord>,
    /// Cross-surface chip parity rows.
    pub chip_surface_rows: Vec<TrafficOriginChipRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl FinalizeTrafficOriginExposureChipsPacket {
    /// Builds a traffic-origin / exposure-chips finalization packet from
    /// canonical rows.
    pub fn new(input: FinalizeTrafficOriginExposureChipsPacketInput) -> Self {
        Self {
            record_kind: FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            command_family_id: input.command_family_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            traffic_origin_classes: input.traffic_origin_classes,
            exposure_classes: input.exposure_classes,
            tunnel_records: input.tunnel_records,
            port_records: input.port_records,
            publish_target_records: input.publish_target_records,
            chip_surface_rows: input.chip_surface_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's stable-line invariants.
    ///
    /// Returns every [`FinalizeTrafficOriginExposureChipsViolation`] found; an
    /// empty vec means the packet is conformant.
    pub fn validate(&self) -> Vec<FinalizeTrafficOriginExposureChipsViolation> {
        let mut violations = Vec::new();
        if self.record_kind != FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_RECORD_KIND {
            violations.push(FinalizeTrafficOriginExposureChipsViolation::WrongRecordKind);
        }
        if self.schema_version != FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_VERSION {
            violations.push(FinalizeTrafficOriginExposureChipsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.command_family_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(FinalizeTrafficOriginExposureChipsViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_traffic_origin_chips(self, &mut violations);
        validate_exposure_chips(self, &mut violations);
        validate_tunnel_records(self, &mut violations);
        validate_port_records(self, &mut violations);
        validate_publish_target_records(self, &mut violations);
        validate_chip_surface_rows(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("traffic-origin exposure-chips packet serializes"),
        ) {
            violations.push(FinalizeTrafficOriginExposureChipsViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("traffic-origin exposure-chips packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .chip_surface_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# Traffic-Origin and Exposure Chips, Tunnel, Port, and Publish-Target Explainability\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Command family: `{}`\n",
            self.command_family_id
        ));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Traffic-origin classes: {}\n",
            self.traffic_origin_classes
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Exposure classes: {}\n",
            self.exposure_classes
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Tunnel records: {}\n",
            self.tunnel_records.len()
        ));
        out.push_str(&format!("- Port records: {}\n", self.port_records.len()));
        out.push_str(&format!(
            "- Publish-target records: {}\n",
            self.publish_target_records.len()
        ));
        out.push_str(&format!(
            "- Chip surfaces: {} ({} narrowed below Stable)\n",
            self.chip_surface_rows.len(),
            narrowed_surfaces
        ));
        out
    }
}

/// Errors emitted when reading the checked-in support export.
#[derive(Debug)]
pub enum FinalizeTrafficOriginExposureChipsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FinalizeTrafficOriginExposureChipsViolation>),
}

impl fmt::Display for FinalizeTrafficOriginExposureChipsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "traffic-origin exposure-chips export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "traffic-origin exposure-chips export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FinalizeTrafficOriginExposureChipsArtifactError {}

/// Validation failures emitted by [`FinalizeTrafficOriginExposureChipsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FinalizeTrafficOriginExposureChipsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The bound canonical contract refs drifted from the single registry/schema.
    ContractRefsNotCanonical,
    /// No traffic-origin classes declared.
    TrafficOriginChipsMissing,
    /// No exposure classes declared.
    ExposureChipsMissing,
    /// A tunnel record is missing required disclosure guards.
    TunnelRecordGuardsBroken,
    /// A port record is missing required disclosure guards.
    PortRecordGuardsBroken,
    /// A publish-target record is missing required disclosure guards.
    PublishTargetRecordGuardsBroken,
    /// Cross-surface coverage is incomplete (missing a required surface class).
    ChipSurfaceCoverageMissing,
    /// A stable, reachable surface dropped required chip disclosure or policy checks.
    ChipSurfaceParityBroken,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl FinalizeTrafficOriginExposureChipsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::TrafficOriginChipsMissing => "traffic_origin_chips_missing",
            Self::ExposureChipsMissing => "exposure_chips_missing",
            Self::TunnelRecordGuardsBroken => "tunnel_record_guards_broken",
            Self::PortRecordGuardsBroken => "port_record_guards_broken",
            Self::PublishTargetRecordGuardsBroken => "publish_target_record_guards_broken",
            Self::ChipSurfaceCoverageMissing => "chip_surface_coverage_missing",
            Self::ChipSurfaceParityBroken => "chip_surface_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in traffic-origin / exposure-chips support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_traffic_origin_exposure_chips_export(
) -> Result<FinalizeTrafficOriginExposureChipsPacket, FinalizeTrafficOriginExposureChipsArtifactError>
{
    let packet: FinalizeTrafficOriginExposureChipsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips/support_export.json"
        )))
        .map_err(FinalizeTrafficOriginExposureChipsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FinalizeTrafficOriginExposureChipsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    for required in [
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DOC_REF,
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_REF,
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DESCRIPTOR_CONTRACT_REF,
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_PARITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations
                .push(FinalizeTrafficOriginExposureChipsViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    if packet.contract_refs != StableContractRefs::canonical() {
        violations
            .push(FinalizeTrafficOriginExposureChipsViolation::ContractRefsNotCanonical);
    }
}

fn validate_traffic_origin_chips(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    if packet.traffic_origin_classes.is_empty() {
        violations.push(FinalizeTrafficOriginExposureChipsViolation::TrafficOriginChipsMissing);
    }
}

fn validate_exposure_chips(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    if packet.exposure_classes.is_empty() {
        violations.push(FinalizeTrafficOriginExposureChipsViolation::ExposureChipsMissing);
    }
}

fn validate_tunnel_records(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    for record in &packet.tunnel_records {
        if !record.guards_hold() {
            violations
                .push(FinalizeTrafficOriginExposureChipsViolation::TunnelRecordGuardsBroken);
            break;
        }
    }
}

fn validate_port_records(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    for record in &packet.port_records {
        if !record.guards_hold() {
            violations
                .push(FinalizeTrafficOriginExposureChipsViolation::PortRecordGuardsBroken);
            break;
        }
    }
}

fn validate_publish_target_records(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    for record in &packet.publish_target_records {
        if !record.guards_hold() {
            violations.push(
                FinalizeTrafficOriginExposureChipsViolation::PublishTargetRecordGuardsBroken,
            );
            break;
        }
    }
}

fn validate_chip_surface_rows(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .chip_surface_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations
                .push(FinalizeTrafficOriginExposureChipsViolation::ChipSurfaceCoverageMissing);
            break;
        }
    }
    for row in &packet.chip_surface_rows {
        if row.claimed_stable && !row.qualification.is_stable() {
            violations
                .push(FinalizeTrafficOriginExposureChipsViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
        if row.qualification.is_stable() && row.chip_visible && !row.preserves_full_disclosure() {
            violations
                .push(FinalizeTrafficOriginExposureChipsViolation::ChipSurfaceParityBroken);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &FinalizeTrafficOriginExposureChipsPacket,
    violations: &mut Vec<FinalizeTrafficOriginExposureChipsViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(FinalizeTrafficOriginExposureChipsViolation::EvidenceExportRefsMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_diff")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
