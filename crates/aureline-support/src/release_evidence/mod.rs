//! Alpha release-evidence graph support projection.
//!
//! This module consumes the checked-in artifact graph at
//! `/artifacts/release/alpha_artifact_graph.yaml` and projects the fields
//! support/export and release-center reconstruction need before any live
//! publication backend exists. The projection is metadata-only: it preserves
//! exact-build refs, artifact-node refs, digest material refs, rollout target
//! classes, auth-source classes, and rollback targets without reading raw
//! package bytes, support attachments, credentials, or private logs.

use std::collections::{BTreeMap, BTreeSet};

pub use aureline_build_farm::{ProvenanceChainRecord, SignatureProjection, TrustDomain};
use serde::{Deserialize, Serialize};

use crate::fitness::{current_fitness_packet_alpha, FitnessPacketProjection};

/// Stable record-kind tag for the alpha artifact graph.
pub const ALPHA_ARTIFACT_GRAPH_RECORD_KIND: &str = "alpha_artifact_graph";

/// Stable record-kind tag for collected alpha release evidence packets.
pub const ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND: &str = "alpha_release_evidence_packet";

/// Current schema version for the checked-in artifact graph.
pub const ALPHA_ARTIFACT_GRAPH_SCHEMA_VERSION: u32 = 1;

/// Current schema version for the support/export evidence projection.
pub const ALPHA_RELEASE_EVIDENCE_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked-in alpha artifact graph.
pub const CURRENT_ALPHA_ARTIFACT_GRAPH_PATH: &str = "artifacts/release/alpha_artifact_graph.yaml";

const CURRENT_ALPHA_ARTIFACT_GRAPH_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/alpha_artifact_graph.yaml"
));

/// Loads the checked-in alpha artifact graph.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in artifact does not match
/// [`AlphaArtifactGraph`].
pub fn current_alpha_artifact_graph() -> Result<AlphaArtifactGraph, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_ARTIFACT_GRAPH_YAML)
}

/// Canonical alpha graph joining release artifacts, evidence, and descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaArtifactGraph {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable graph id used by evidence packets.
    pub graph_id: String,
    /// UTC timestamp for the seed graph revision.
    pub as_of: String,
    /// Owning release/support reviewer.
    pub owner_dri: String,
    /// Graph state such as `seeded_pending_publication_bytes`.
    pub status: String,
    /// Exact-build identity shared by the candidate and artifact nodes.
    pub exact_build_identity_ref: String,
    /// Human-readable candidate version carried into release-center rows.
    pub candidate_version: String,
    /// Release channel class such as `preview`.
    pub channel_class: String,
    /// Source contracts consumed by this graph.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Build identity metadata for the candidate.
    pub build_identity: BuildIdentityDescriptor,
    /// Build roots that materialize or rehearse the graph.
    #[serde(default)]
    pub build_roots: Vec<BuildRootDescriptor>,
    /// Provenance inputs required by release evidence.
    #[serde(default)]
    pub provenance_inputs: Vec<ProvenanceInputDescriptor>,
    /// Artifact-family nodes covered by the graph.
    pub artifact_families: ArtifactFamilies,
    /// Release-center object descriptors seeded by the graph.
    pub release_center_objects: ReleaseCenterObjects,
    /// Evidence collection contract for headless reconstruction.
    pub evidence_collection: EvidenceCollectionDescriptor,
    /// Acceptance proof metadata for validators and support/export consumers.
    pub acceptance: GraphAcceptance,
}

impl AlphaArtifactGraph {
    /// Returns every artifact node in the graph.
    pub fn artifact_nodes(&self) -> Vec<&AlphaArtifactNode> {
        self.artifact_families.all_nodes()
    }

    /// Validates the graph's release-evidence and release-center invariants.
    pub fn validate(&self) -> Vec<AlphaReleaseEvidenceViolation> {
        let mut violations = Vec::new();

        if self.schema_version != ALPHA_ARTIFACT_GRAPH_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "graph.schema_version",
                &self.graph_id,
                "alpha artifact graph schema_version must be 1",
            );
        }
        if self.record_kind != ALPHA_ARTIFACT_GRAPH_RECORD_KIND {
            push_violation(
                &mut violations,
                "graph.record_kind",
                &self.graph_id,
                "alpha artifact graph record_kind is not supported",
            );
        }
        if !self
            .exact_build_identity_ref
            .starts_with("build-id:aureline:")
        {
            push_violation(
                &mut violations,
                "graph.exact_build_identity_ref",
                &self.graph_id,
                "exact_build_identity_ref must be an Aureline build-id ref",
            );
        }
        if self.build_identity.primary_exact_build_identity_ref != self.exact_build_identity_ref {
            push_violation(
                &mut violations,
                "build_identity.primary_exact_build_identity_ref",
                &self.build_identity.primary_exact_build_identity_ref,
                "build identity primary ref must match the graph exact-build ref",
            );
        }

        for (key, value) in &self.source_contract_refs {
            if key.trim().is_empty() || value.trim().is_empty() {
                push_violation(
                    &mut violations,
                    "source_contract_refs.empty",
                    &self.graph_id,
                    "source contract refs must have non-empty keys and values",
                );
            }
        }

        self.artifact_families.validate_presence(&mut violations);

        let mut node_ids = BTreeSet::new();
        for node in self.artifact_nodes() {
            node.validate(&self.exact_build_identity_ref, &mut violations);
            if !node_ids.insert(node.node_id.as_str()) {
                push_violation(
                    &mut violations,
                    "artifact_node.duplicate_node_id",
                    &node.node_id,
                    "artifact node ids must be unique",
                );
            }
        }

        self.release_center_objects
            .validate(&node_ids, &mut violations);
        self.evidence_collection.validate(&mut violations);

        violations
    }

    /// Projects the graph into a metadata-only release evidence packet.
    pub fn release_evidence_packet(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AlphaReleaseEvidencePacket {
        let candidate = self
            .release_center_objects
            .release_candidate_descriptors
            .first();
        let target = candidate
            .and_then(|candidate| candidate.publish_target_refs.first())
            .and_then(|target_ref| {
                self.release_center_objects
                    .publish_target_descriptors
                    .iter()
                    .find(|target| target.publish_target_id == *target_ref)
            })
            .or_else(|| {
                self.release_center_objects
                    .publish_target_descriptors
                    .first()
            });
        let bundle = candidate
            .and_then(|candidate| candidate.artifact_bundle_refs.first())
            .and_then(|bundle_ref| {
                self.release_center_objects
                    .artifact_bundle_descriptors
                    .iter()
                    .find(|bundle| bundle.bundle_id == *bundle_ref)
            })
            .or_else(|| {
                self.release_center_objects
                    .artifact_bundle_descriptors
                    .first()
            });

        let node_lookup = self
            .artifact_nodes()
            .into_iter()
            .map(|node| (node.node_id.as_str(), node))
            .collect::<BTreeMap<_, _>>();

        let digest_set = bundle
            .map(|bundle| {
                bundle
                    .digest_set
                    .iter()
                    .map(|entry| AlphaDigestSetEntry {
                        digest_id: entry.digest_id.clone(),
                        artifact_node_ref: entry.artifact_node_ref.clone(),
                        family_class: node_lookup
                            .get(entry.artifact_node_ref.as_str())
                            .map(|node| node.family_class.clone())
                            .unwrap_or_default(),
                        digest_material_ref: entry.digest_material_ref.clone(),
                        algorithm: entry.algorithm.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut trust_domain_refs = BTreeSet::new();
        trust_domain_refs.extend(
            self.build_roots
                .iter()
                .map(|root| root.trust_domain.clone()),
        );
        trust_domain_refs.extend(
            self.provenance_inputs
                .iter()
                .map(|input| input.trust_domain.clone()),
        );
        trust_domain_refs.extend(
            self.artifact_nodes()
                .into_iter()
                .map(|node| node.trust_domain.clone()),
        );

        let packet_id = packet_id.into();
        let generated_at = generated_at.into();
        let signature_projection = signature_projection_for(
            &packet_id,
            &self.exact_build_identity_ref,
            bundle,
            &self.source_contract_refs,
        );

        AlphaReleaseEvidencePacket {
            schema_version: ALPHA_RELEASE_EVIDENCE_PACKET_SCHEMA_VERSION,
            record_kind: ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND.to_owned(),
            packet_id,
            generated_at,
            graph_ref: CURRENT_ALPHA_ARTIFACT_GRAPH_PATH.to_owned(),
            graph_id: self.graph_id.clone(),
            exact_build_identity_ref: self.exact_build_identity_ref.clone(),
            candidate_id: candidate
                .map(|candidate| candidate.candidate_id.clone())
                .unwrap_or_default(),
            candidate_version: candidate
                .map(|candidate| candidate.candidate_version.clone())
                .unwrap_or_default(),
            target_class: target
                .map(|target| target.target_class.clone())
                .unwrap_or_default(),
            publish_target_id: target
                .map(|target| target.publish_target_id.clone())
                .unwrap_or_default(),
            rollout_ring: target
                .map(|target| target.rollout_ring.clone())
                .unwrap_or_default(),
            auth_source_class: target
                .map(|target| target.auth_source_class.clone())
                .unwrap_or_default(),
            rollback_target_ref: target
                .map(|target| target.rollback_target_ref.clone())
                .or_else(|| candidate.map(|candidate| candidate.rollback_target_ref.clone()))
                .unwrap_or_default(),
            artifact_bundle_ref: bundle
                .map(|bundle| bundle.bundle_id.clone())
                .unwrap_or_default(),
            digest_set,
            artifact_node_refs: bundle
                .map(|bundle| bundle.artifact_node_refs.clone())
                .unwrap_or_default(),
            evidence_refs: candidate
                .map(|candidate| candidate.evidence_refs.clone())
                .unwrap_or_default(),
            protected_fitness_packet: current_fitness_packet_alpha()
                .ok()
                .map(|packet| packet.release_evidence_projection()),
            signature_projection,
            trust_domain_refs: trust_domain_refs.into_iter().collect(),
            raw_private_material_excluded: true,
        }
    }
}

fn signature_projection_for(
    packet_id: &str,
    exact_build_identity_ref: &str,
    bundle: Option<&ArtifactBundleDescriptor>,
    source_contract_refs: &BTreeMap<String, String>,
) -> SignatureProjection {
    let artifact_bundle_ref = bundle
        .map(|bundle| bundle.bundle_id.clone())
        .unwrap_or_else(|| "artifact_bundle:unknown".to_owned());
    let digest_set_ref = bundle
        .map(|bundle| bundle.digest_set_ref.clone())
        .unwrap_or_else(|| "digest_set:unknown".to_owned());
    let signature_state = bundle
        .map(|bundle| bundle.signature_state.clone())
        .unwrap_or_else(|| "signature_state_unknown".to_owned());
    let attestation_state = bundle
        .map(|bundle| bundle.attestation_state.clone())
        .unwrap_or_else(|| "attestation_state_unknown".to_owned());

    let build_agent_evidence_ref = source_contract_refs
        .get("clean_room_rebuild_alpha_ref")
        .or_else(|| source_contract_refs.get("collector_ref"))
        .cloned()
        .unwrap_or_else(|| CURRENT_ALPHA_ARTIFACT_GRAPH_PATH.to_owned());
    let publisher_key_evidence_ref = source_contract_refs
        .get("alpha_notice_sbom_provenance_dry_run_ref")
        .or_else(|| source_contract_refs.get("trust_domain_baseline_ref"))
        .cloned()
        .unwrap_or_else(|| CURRENT_ALPHA_ARTIFACT_GRAPH_PATH.to_owned());
    let mirror_evidence_ref = source_contract_refs
        .get("mirror_offline_publication_dry_run_ref")
        .or_else(|| source_contract_refs.get("alpha_publication_manifest_ref"))
        .cloned()
        .unwrap_or_else(|| CURRENT_ALPHA_ARTIFACT_GRAPH_PATH.to_owned());

    SignatureProjection::new(
        format!("signature_projection:{packet_id}"),
        exact_build_identity_ref.to_owned(),
        artifact_bundle_ref.clone(),
        digest_set_ref.clone(),
        signature_state,
        attestation_state,
        vec![
            ProvenanceChainRecord::new(
                format!("provenance_chain:{packet_id}:build_agent_to_publisher_key"),
                0,
                TrustDomain::BuildAgent,
                TrustDomain::PublisherKey,
                exact_build_identity_ref.to_owned(),
                artifact_bundle_ref.clone(),
                digest_set_ref.clone(),
                build_agent_evidence_ref,
                "request_signature_by_digest",
            ),
            ProvenanceChainRecord::new(
                format!("provenance_chain:{packet_id}:publisher_key_to_release_registry"),
                1,
                TrustDomain::PublisherKey,
                TrustDomain::ReleaseRegistry,
                exact_build_identity_ref.to_owned(),
                artifact_bundle_ref.clone(),
                digest_set_ref.clone(),
                publisher_key_evidence_ref,
                "project_signature_to_registry",
            ),
            ProvenanceChainRecord::new(
                format!("provenance_chain:{packet_id}:release_registry_to_mirror_origin"),
                2,
                TrustDomain::ReleaseRegistry,
                TrustDomain::MirrorOrigin,
                exact_build_identity_ref.to_owned(),
                artifact_bundle_ref,
                digest_set_ref,
                mirror_evidence_ref,
                "preserve_origin_signature_for_mirror",
            ),
        ],
    )
}

/// Build identity metadata for a candidate graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIdentityDescriptor {
    /// Primary exact-build identity ref used by the graph.
    pub primary_exact_build_identity_ref: String,
    /// Path to the checked-in source for the primary identity.
    pub build_identity_source_ref: String,
    /// Current local build identity source used as a comparison input.
    pub current_local_build_identity_ref: String,
    /// Candidate version label.
    pub version: String,
    /// Release channel class.
    pub channel_class: String,
    /// Target triple for the candidate.
    pub target_triple: String,
    /// Build profile for the candidate.
    pub profile: String,
    /// Source revision ref for the candidate.
    pub source_revision_ref: String,
    /// Build root refs that materialize the candidate.
    #[serde(default)]
    pub build_root_refs: Vec<String>,
}

/// One build root or clean-room materialization root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRootDescriptor {
    /// Stable build-root id.
    pub build_root_id: String,
    /// Trust domain used by the root.
    pub trust_domain: String,
    /// Component id such as `ci_runner`.
    pub component_id: String,
    /// Source ref for root evidence.
    pub source_ref: String,
    /// Materialization state for the root.
    pub materialization_state: String,
    /// Cache write posture for the root.
    pub cache_write_posture: String,
    /// Redaction-safe notes for reviewers.
    pub notes: String,
}

/// One provenance input consumed by release evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceInputDescriptor {
    /// Stable provenance input id.
    pub provenance_input_id: String,
    /// Trust domain that owns the input.
    pub trust_domain: String,
    /// Source ref for the input.
    pub source_ref: String,
    /// Evidence class carried by the input.
    pub evidence_class: String,
    /// Whether raw private material is excluded from the input.
    pub raw_private_material_excluded: bool,
}

/// Artifact-family nodes grouped by release graph role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFamilies {
    /// Runnable binary and command-line seed nodes.
    #[serde(default)]
    pub binaries: Vec<AlphaArtifactNode>,
    /// Symbol, source-map, and crash-symbol seed nodes.
    #[serde(default)]
    pub symbols: Vec<AlphaArtifactNode>,
    /// Docs/help pack nodes.
    #[serde(default)]
    pub docs_help_packs: Vec<AlphaArtifactNode>,
    /// Schema and contract export nodes.
    #[serde(default)]
    pub schemas: Vec<AlphaArtifactNode>,
    /// Support/export projection nodes.
    #[serde(default)]
    pub support_exports: Vec<AlphaArtifactNode>,
    /// Release evidence packet nodes.
    #[serde(default)]
    pub release_evidence: Vec<AlphaArtifactNode>,
    /// Update metadata nodes.
    #[serde(default)]
    pub update_metadata: Vec<AlphaArtifactNode>,
    /// Supply-chain and provenance nodes.
    #[serde(default)]
    pub supply_chain: Vec<AlphaArtifactNode>,
}

impl ArtifactFamilies {
    fn all_nodes(&self) -> Vec<&AlphaArtifactNode> {
        let mut nodes = Vec::new();
        nodes.extend(self.binaries.iter());
        nodes.extend(self.symbols.iter());
        nodes.extend(self.docs_help_packs.iter());
        nodes.extend(self.schemas.iter());
        nodes.extend(self.support_exports.iter());
        nodes.extend(self.release_evidence.iter());
        nodes.extend(self.update_metadata.iter());
        nodes.extend(self.supply_chain.iter());
        nodes
    }

    fn validate_presence(&self, violations: &mut Vec<AlphaReleaseEvidenceViolation>) {
        for (field, len) in [
            ("artifact_families.binaries", self.binaries.len()),
            ("artifact_families.symbols", self.symbols.len()),
            (
                "artifact_families.docs_help_packs",
                self.docs_help_packs.len(),
            ),
            ("artifact_families.schemas", self.schemas.len()),
            (
                "artifact_families.support_exports",
                self.support_exports.len(),
            ),
            (
                "artifact_families.release_evidence",
                self.release_evidence.len(),
            ),
        ] {
            if len == 0 {
                push_violation(
                    violations,
                    field,
                    field,
                    "required alpha artifact family must have at least one node",
                );
            }
        }
    }
}

/// One artifact node in the alpha graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaArtifactNode {
    /// Stable artifact node id.
    pub node_id: String,
    /// Release artifact family class.
    pub family_class: String,
    /// Reviewer-facing display name.
    pub display_name: String,
    /// Exact-build identity ref that binds the node.
    pub exact_build_identity_ref: String,
    /// Source ref used by support and evidence review.
    pub source_ref: String,
    /// Source ref whose bytes are hashed by the collector.
    pub digest_source_ref: String,
    /// Trust domain that produced or owns the node.
    pub trust_domain: String,
    /// Material state such as checked-in packet or pending package bytes.
    pub material_state: String,
    /// Whether this node is required for the candidate bundle.
    pub required_for_candidate: bool,
    /// Support/export or release-center ref associated with the node.
    pub support_ref: String,
}

impl AlphaArtifactNode {
    fn validate(
        &self,
        graph_exact_build_identity_ref: &str,
        violations: &mut Vec<AlphaReleaseEvidenceViolation>,
    ) {
        for (field, value) in [
            ("node_id", &self.node_id),
            ("family_class", &self.family_class),
            ("source_ref", &self.source_ref),
            ("digest_source_ref", &self.digest_source_ref),
            ("trust_domain", &self.trust_domain),
            ("material_state", &self.material_state),
            ("support_ref", &self.support_ref),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("artifact_node.{field}"),
                    &self.node_id,
                    "artifact node fields must be non-empty",
                );
            }
        }
        if self.exact_build_identity_ref != graph_exact_build_identity_ref {
            push_violation(
                violations,
                "artifact_node.exact_build_identity_ref",
                &self.node_id,
                "artifact node exact-build ref must match the graph exact-build ref",
            );
        }
    }
}

/// Release-center descriptors carried by the graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterObjects {
    /// Candidate descriptors shared by release center and headless flows.
    #[serde(default)]
    pub release_candidate_descriptors: Vec<ReleaseCandidateDescriptor>,
    /// Publish target descriptors shared by release center and headless flows.
    #[serde(default)]
    pub publish_target_descriptors: Vec<PublishTargetDescriptor>,
    /// Promotion timeline descriptors.
    #[serde(default)]
    pub promotion_timeline_descriptors: Vec<PromotionTimelineDescriptor>,
    /// Artifact bundle descriptors.
    #[serde(default)]
    pub artifact_bundle_descriptors: Vec<ArtifactBundleDescriptor>,
}

impl ReleaseCenterObjects {
    fn validate(
        &self,
        node_ids: &BTreeSet<&str>,
        violations: &mut Vec<AlphaReleaseEvidenceViolation>,
    ) {
        if self.release_candidate_descriptors.is_empty() {
            push_violation(
                violations,
                "release_candidate_descriptors.empty",
                "release_center_objects",
                "at least one release candidate descriptor is required",
            );
        }
        if self.publish_target_descriptors.is_empty() {
            push_violation(
                violations,
                "publish_target_descriptors.empty",
                "release_center_objects",
                "at least one publish target descriptor is required",
            );
        }
        if self.promotion_timeline_descriptors.is_empty() {
            push_violation(
                violations,
                "promotion_timeline_descriptors.empty",
                "release_center_objects",
                "at least one promotion timeline descriptor is required",
            );
        }
        if self.artifact_bundle_descriptors.is_empty() {
            push_violation(
                violations,
                "artifact_bundle_descriptors.empty",
                "release_center_objects",
                "at least one artifact bundle descriptor is required",
            );
        }

        let target_ids = self
            .publish_target_descriptors
            .iter()
            .map(|target| target.publish_target_id.as_str())
            .collect::<BTreeSet<_>>();
        let bundle_ids = self
            .artifact_bundle_descriptors
            .iter()
            .map(|bundle| bundle.bundle_id.as_str())
            .collect::<BTreeSet<_>>();

        for candidate in &self.release_candidate_descriptors {
            candidate.validate(&target_ids, &bundle_ids, violations);
        }
        for target in &self.publish_target_descriptors {
            target.validate(violations);
        }
        for timeline in &self.promotion_timeline_descriptors {
            timeline.validate(violations);
        }
        for bundle in &self.artifact_bundle_descriptors {
            bundle.validate(node_ids, violations);
        }
    }
}

/// Release candidate descriptor shared by UI and headless publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCandidateDescriptor {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Candidate version label.
    pub candidate_version: String,
    /// Channel family for the candidate.
    pub channel_family: String,
    /// Current release stage class.
    pub current_stage_class: String,
    /// Exact-build identity ref for the candidate.
    pub exact_build_identity_ref: String,
    /// Artifact bundle refs scoped to the candidate.
    #[serde(default)]
    pub artifact_bundle_refs: Vec<String>,
    /// Publish target refs scoped to the candidate.
    #[serde(default)]
    pub publish_target_refs: Vec<String>,
    /// Evidence refs required by the candidate.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Known issue refs associated with the candidate.
    #[serde(default)]
    pub known_issue_refs: Vec<String>,
    /// Rollback target ref for the candidate.
    pub rollback_target_ref: String,
    /// Blocking conditions that prevent wider publication.
    #[serde(default)]
    pub blockers: Vec<String>,
    /// Support packet refs associated with the candidate.
    #[serde(default)]
    pub support_packet_refs: Vec<String>,
}

impl ReleaseCandidateDescriptor {
    fn validate(
        &self,
        target_ids: &BTreeSet<&str>,
        bundle_ids: &BTreeSet<&str>,
        violations: &mut Vec<AlphaReleaseEvidenceViolation>,
    ) {
        for (field, value) in [
            ("candidate_id", &self.candidate_id),
            ("candidate_version", &self.candidate_version),
            ("channel_family", &self.channel_family),
            ("current_stage_class", &self.current_stage_class),
            ("exact_build_identity_ref", &self.exact_build_identity_ref),
            ("rollback_target_ref", &self.rollback_target_ref),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("release_candidate.{field}"),
                    &self.candidate_id,
                    "release candidate fields must be non-empty",
                );
            }
        }
        for target_ref in &self.publish_target_refs {
            if !target_ids.contains(target_ref.as_str()) {
                push_violation(
                    violations,
                    "release_candidate.publish_target_refs",
                    target_ref,
                    "release candidate references an unknown publish target",
                );
            }
        }
        for bundle_ref in &self.artifact_bundle_refs {
            if !bundle_ids.contains(bundle_ref.as_str()) {
                push_violation(
                    violations,
                    "release_candidate.artifact_bundle_refs",
                    bundle_ref,
                    "release candidate references an unknown artifact bundle",
                );
            }
        }
    }
}

/// Publish target descriptor shared by release-center and headless flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishTargetDescriptor {
    /// Stable publish target id.
    pub publish_target_id: String,
    /// Publish target class.
    pub target_class: String,
    /// Destination class for publication.
    pub destination_class: String,
    /// Visibility class after publication.
    pub visibility_class: String,
    /// Mutability class after publication.
    pub mutability_class: String,
    /// Support class associated with the target.
    pub support_class: String,
    /// Auth-source class disclosed before mutation.
    pub auth_source_class: String,
    /// Actor class expected to perform the action.
    pub actor_class: String,
    /// Rollout ring for the target.
    pub rollout_ring: String,
    /// Dry-run availability class.
    pub dry_run_availability_class: String,
    /// Evidence freshness class.
    pub evidence_freshness_class: String,
    /// Rollback path class.
    pub rollback_path_class: String,
    /// Rollback target ref for the target.
    pub rollback_target_ref: String,
    /// Exact-build identity refs in target scope.
    #[serde(default)]
    pub exact_build_identity_refs: Vec<String>,
    /// Cross-surface parity refs.
    #[serde(default)]
    pub surface_parity_refs: Vec<String>,
    /// Redaction-safe notes for reviewers.
    pub notes: String,
}

impl PublishTargetDescriptor {
    fn validate(&self, violations: &mut Vec<AlphaReleaseEvidenceViolation>) {
        for (field, value) in [
            ("publish_target_id", &self.publish_target_id),
            ("target_class", &self.target_class),
            ("destination_class", &self.destination_class),
            ("visibility_class", &self.visibility_class),
            ("auth_source_class", &self.auth_source_class),
            ("rollout_ring", &self.rollout_ring),
            ("rollback_target_ref", &self.rollback_target_ref),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("publish_target.{field}"),
                    &self.publish_target_id,
                    "publish target fields must be non-empty",
                );
            }
        }
    }
}

/// Promotion timeline descriptor for release-center reconstruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionTimelineDescriptor {
    /// Stable timeline entry id.
    pub timeline_entry_id: String,
    /// Candidate ref moved by the timeline entry.
    pub candidate_ref: String,
    /// Source stage class.
    pub source_stage_class: String,
    /// Destination stage class.
    pub destination_stage_class: String,
    /// Current stage class.
    pub current_stage_class: String,
    /// Timeline event class.
    pub timeline_event_class: String,
    /// Semantic change class.
    pub semantic_change_class: String,
    /// Rollout ring for the transition.
    pub rollout_ring: String,
    /// Auth-source class for the transition.
    pub auth_source_class: String,
    /// Digest set ref bound to the transition.
    pub digest_set_ref: String,
    /// Evidence refs for the transition.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Rollback target ref for the transition.
    pub rollback_target_ref: String,
    /// Reversible window for the transition.
    pub reversible_window: String,
    /// Break-glass state class for the transition.
    pub break_glass_state_class: String,
}

impl PromotionTimelineDescriptor {
    fn validate(&self, violations: &mut Vec<AlphaReleaseEvidenceViolation>) {
        for (field, value) in [
            ("timeline_entry_id", &self.timeline_entry_id),
            ("candidate_ref", &self.candidate_ref),
            ("source_stage_class", &self.source_stage_class),
            ("destination_stage_class", &self.destination_stage_class),
            ("rollout_ring", &self.rollout_ring),
            ("auth_source_class", &self.auth_source_class),
            ("digest_set_ref", &self.digest_set_ref),
            ("rollback_target_ref", &self.rollback_target_ref),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("promotion_timeline.{field}"),
                    &self.timeline_entry_id,
                    "promotion timeline fields must be non-empty",
                );
            }
        }
    }
}

/// Artifact bundle descriptor for coordinated release-family movement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactBundleDescriptor {
    /// Stable artifact bundle id.
    pub bundle_id: String,
    /// Bundle class.
    pub bundle_class: String,
    /// Exact-build identity ref for the bundle.
    pub exact_build_identity_ref: String,
    /// Digest set ref for the bundle.
    pub digest_set_ref: String,
    /// Signature state for the bundle.
    pub signature_state: String,
    /// Attestation state for the bundle.
    pub attestation_state: String,
    /// Export actions supported by the bundle seed.
    #[serde(default)]
    pub export_actions: Vec<String>,
    /// Artifact node refs included in the bundle.
    #[serde(default)]
    pub artifact_node_refs: Vec<String>,
    /// Digest material entries included in the bundle.
    #[serde(default)]
    pub digest_set: Vec<BundleDigestEntry>,
}

impl ArtifactBundleDescriptor {
    fn validate(
        &self,
        node_ids: &BTreeSet<&str>,
        violations: &mut Vec<AlphaReleaseEvidenceViolation>,
    ) {
        for (field, value) in [
            ("bundle_id", &self.bundle_id),
            ("bundle_class", &self.bundle_class),
            ("exact_build_identity_ref", &self.exact_build_identity_ref),
            ("digest_set_ref", &self.digest_set_ref),
            ("signature_state", &self.signature_state),
            ("attestation_state", &self.attestation_state),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("artifact_bundle.{field}"),
                    &self.bundle_id,
                    "artifact bundle fields must be non-empty",
                );
            }
        }
        if self.artifact_node_refs.is_empty() {
            push_violation(
                violations,
                "artifact_bundle.artifact_node_refs",
                &self.bundle_id,
                "artifact bundle must include artifact node refs",
            );
        }
        if self.digest_set.is_empty() {
            push_violation(
                violations,
                "artifact_bundle.digest_set",
                &self.bundle_id,
                "artifact bundle must include digest set entries",
            );
        }
        for node_ref in &self.artifact_node_refs {
            if !node_ids.contains(node_ref.as_str()) {
                push_violation(
                    violations,
                    "artifact_bundle.artifact_node_refs",
                    node_ref,
                    "artifact bundle references an unknown artifact node",
                );
            }
        }
        for entry in &self.digest_set {
            entry.validate(node_ids, violations);
        }
    }
}

/// Digest material entry for an artifact bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDigestEntry {
    /// Stable digest entry id.
    pub digest_id: String,
    /// Artifact node ref whose material is hashed.
    pub artifact_node_ref: String,
    /// Digest algorithm such as `sha256`.
    pub algorithm: String,
    /// Source ref whose bytes are hashed.
    pub digest_material_ref: String,
}

impl BundleDigestEntry {
    fn validate(
        &self,
        node_ids: &BTreeSet<&str>,
        violations: &mut Vec<AlphaReleaseEvidenceViolation>,
    ) {
        for (field, value) in [
            ("digest_id", &self.digest_id),
            ("artifact_node_ref", &self.artifact_node_ref),
            ("algorithm", &self.algorithm),
            ("digest_material_ref", &self.digest_material_ref),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("artifact_bundle.digest_set.{field}"),
                    &self.digest_id,
                    "digest set fields must be non-empty",
                );
            }
        }
        if !node_ids.contains(self.artifact_node_ref.as_str()) {
            push_violation(
                violations,
                "artifact_bundle.digest_set.artifact_node_ref",
                &self.artifact_node_ref,
                "digest set entry references an unknown artifact node",
            );
        }
    }
}

/// Evidence collection contract for headless packet generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCollectionDescriptor {
    /// Collector script ref.
    pub collector_ref: String,
    /// Default graph ref consumed by the collector.
    pub default_graph_ref: String,
    /// Output record kind emitted by the collector.
    pub output_record_kind: String,
    /// Default output ref emitted by the collector.
    pub default_output_ref: String,
    /// Fields that the collected packet must reconstruct.
    #[serde(default)]
    pub required_reconstruction_fields: Vec<String>,
    /// Graph paths used to populate packet fields.
    pub generated_packet_refs: BTreeMap<String, String>,
}

impl EvidenceCollectionDescriptor {
    fn validate(&self, violations: &mut Vec<AlphaReleaseEvidenceViolation>) {
        if self.output_record_kind != ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND {
            push_violation(
                violations,
                "evidence_collection.output_record_kind",
                &self.output_record_kind,
                "collector output record kind must be alpha_release_evidence_packet",
            );
        }
        let required = [
            "candidate_version",
            "target_class",
            "digest_set",
            "rollout_ring",
            "auth_source_class",
            "rollback_target_ref",
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        let present = self
            .required_reconstruction_fields
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for field in required.difference(&present) {
            push_violation(
                violations,
                "evidence_collection.required_reconstruction_fields",
                field,
                "release evidence collector is missing a required reconstruction field",
            );
        }
    }
}

/// Acceptance metadata for graph validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphAcceptance {
    /// Commands that validate the graph and consumer projection.
    #[serde(default)]
    pub validation_commands: Vec<String>,
    /// Protected fixture refs consumed by this graph.
    #[serde(default)]
    pub protected_fixture_refs: Vec<String>,
    /// Accepted state names proven by this graph.
    #[serde(default)]
    pub accepted_states: Vec<String>,
}

/// Metadata-only release evidence projection over the graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaReleaseEvidencePacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Graph ref consumed by the packet.
    pub graph_ref: String,
    /// Graph id consumed by the packet.
    pub graph_id: String,
    /// Exact-build identity ref reconstructed from the graph.
    pub exact_build_identity_ref: String,
    /// Release candidate id reconstructed from the graph.
    pub candidate_id: String,
    /// Candidate version reconstructed from the graph.
    pub candidate_version: String,
    /// Publish target class reconstructed from the graph.
    pub target_class: String,
    /// Publish target id reconstructed from the graph.
    pub publish_target_id: String,
    /// Rollout ring reconstructed from the graph.
    pub rollout_ring: String,
    /// Auth-source class reconstructed from the graph.
    pub auth_source_class: String,
    /// Rollback target ref reconstructed from the graph.
    pub rollback_target_ref: String,
    /// Artifact bundle ref reconstructed from the graph.
    pub artifact_bundle_ref: String,
    /// Digest set entries reconstructed from bundle material refs.
    pub digest_set: Vec<AlphaDigestSetEntry>,
    /// Artifact node refs included in the bundle.
    pub artifact_node_refs: Vec<String>,
    /// Evidence refs linked to the candidate.
    pub evidence_refs: Vec<String>,
    /// Typed protected fitness packet projection linked to release evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_fitness_packet: Option<FitnessPacketProjection>,
    /// Metadata-only signature and trust-domain chain projection.
    pub signature_projection: SignatureProjection,
    /// Trust domains visible in the packet.
    pub trust_domain_refs: Vec<String>,
    /// Whether raw private material is excluded from the projection.
    pub raw_private_material_excluded: bool,
}

impl AlphaReleaseEvidencePacket {
    /// Returns true when the packet carries all required reconstruction fields.
    pub fn reconstructs_required_release_fields(&self) -> bool {
        !self.candidate_version.trim().is_empty()
            && !self.target_class.trim().is_empty()
            && !self.digest_set.is_empty()
            && !self.rollout_ring.trim().is_empty()
            && !self.auth_source_class.trim().is_empty()
            && !self.rollback_target_ref.trim().is_empty()
    }

    /// Returns true when the packet carries a valid build-farm signature chain.
    pub fn carries_valid_signature_projection(&self) -> bool {
        self.signature_projection.carries_valid_chain()
    }
}

/// One digest set row reconstructed from the artifact bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaDigestSetEntry {
    /// Stable digest entry id.
    pub digest_id: String,
    /// Artifact node ref the digest belongs to.
    pub artifact_node_ref: String,
    /// Artifact family class for the node.
    pub family_class: String,
    /// Source ref whose bytes are hashed by the collector.
    pub digest_material_ref: String,
    /// Digest algorithm such as `sha256`.
    pub algorithm: String,
}

/// One validation violation emitted by the graph consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaReleaseEvidenceViolation {
    /// Stable check id for the violated rule.
    pub check_id: String,
    /// Ref or graph location associated with the violation.
    pub reference: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<AlphaReleaseEvidenceViolation>,
    check_id: &str,
    reference: &str,
    message: &str,
) {
    violations.push(AlphaReleaseEvidenceViolation {
        check_id: check_id.to_owned(),
        reference: reference.to_owned(),
        message: message.to_owned(),
    });
}
