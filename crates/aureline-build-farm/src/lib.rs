//! Metadata-only build-farm trust-domain and provenance-chain model.
//!
//! The crate models the release pipeline boundary between build agents,
//! publisher keys, release registries, and mirror origins. It deliberately
//! stores refs, states, and validation results only; it never signs bytes,
//! verifies signatures, or handles private key material.

use serde::{Deserialize, Serialize};

/// Schema version for build-farm provenance records.
pub const BUILD_FARM_PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one provenance-chain transition.
pub const PROVENANCE_CHAIN_RECORD_KIND: &str = "build_farm_provenance_chain_record";

/// Stable record-kind tag for a signature projection.
pub const SIGNATURE_PROJECTION_RECORD_KIND: &str = "build_farm_signature_projection";

/// Trust domain that owns one stage of a release provenance chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustDomain {
    /// Build or package worker that materializes artifacts from governed inputs.
    BuildAgent,
    /// Isolated publisher-key boundary that asserts digest sets without exposing raw keys.
    PublisherKey,
    /// Release registry or channel origin that publishes signed artifact metadata.
    ReleaseRegistry,
    /// Mirror or offline origin that preserves upstream signatures and emits receipts.
    MirrorOrigin,
}

impl TrustDomain {
    /// Returns the stable snake-case token used in serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildAgent => "build_agent",
            Self::PublisherKey => "publisher_key",
            Self::ReleaseRegistry => "release_registry",
            Self::MirrorOrigin => "mirror_origin",
        }
    }

    /// Returns true when this domain may hand evidence to `next`.
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::BuildAgent, Self::PublisherKey)
                | (Self::PublisherKey, Self::ReleaseRegistry)
                | (Self::ReleaseRegistry, Self::MirrorOrigin)
        )
    }
}

/// One metadata-only transition in a release provenance chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceChainRecord {
    /// Record schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable record id within the projection.
    pub record_id: String,
    /// Zero-based sequence number inside the chain.
    pub sequence: u32,
    /// Domain that emitted or handed off this evidence.
    pub source_domain: TrustDomain,
    /// Domain that consumes this evidence next.
    pub destination_domain: TrustDomain,
    /// Exact-build identity set covered by this transition.
    pub exact_build_identity_ref: String,
    /// Artifact, bundle, registry row, or mirror row this transition covers.
    pub subject_ref: String,
    /// Digest-set ref this transition binds.
    pub digest_set_ref: String,
    /// Evidence row, packet, or manifest ref backing the transition.
    pub evidence_ref: String,
    /// Transition class such as `request_signature_by_digest`.
    pub transition_class: String,
    /// Whether raw signing material is excluded from the record.
    pub raw_signing_material_excluded: bool,
}

impl ProvenanceChainRecord {
    /// Builds a metadata-only provenance-chain transition.
    pub fn new(
        record_id: impl Into<String>,
        sequence: u32,
        source_domain: TrustDomain,
        destination_domain: TrustDomain,
        exact_build_identity_ref: impl Into<String>,
        subject_ref: impl Into<String>,
        digest_set_ref: impl Into<String>,
        evidence_ref: impl Into<String>,
        transition_class: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: BUILD_FARM_PROVENANCE_SCHEMA_VERSION,
            record_kind: PROVENANCE_CHAIN_RECORD_KIND.to_owned(),
            record_id: record_id.into(),
            sequence,
            source_domain,
            destination_domain,
            exact_build_identity_ref: exact_build_identity_ref.into(),
            subject_ref: subject_ref.into(),
            digest_set_ref: digest_set_ref.into(),
            evidence_ref: evidence_ref.into(),
            transition_class: transition_class.into(),
            raw_signing_material_excluded: true,
        }
    }

    /// Validates this transition against the closed trust-domain schema.
    pub fn validate(&self) -> Vec<ProvenanceChainViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUILD_FARM_PROVENANCE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "provenance_chain.schema_version",
                &self.record_id,
                "provenance-chain record schema_version must be 1",
            );
        }
        if self.record_kind != PROVENANCE_CHAIN_RECORD_KIND {
            push_violation(
                &mut violations,
                "provenance_chain.record_kind",
                &self.record_id,
                "provenance-chain record kind is not supported",
            );
        }

        for (field, value) in [
            ("record_id", self.record_id.as_str()),
            (
                "exact_build_identity_ref",
                self.exact_build_identity_ref.as_str(),
            ),
            ("subject_ref", self.subject_ref.as_str()),
            ("digest_set_ref", self.digest_set_ref.as_str()),
            ("evidence_ref", self.evidence_ref.as_str()),
            ("transition_class", self.transition_class.as_str()),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    &mut violations,
                    &format!("provenance_chain.{field}"),
                    &self.record_id,
                    "provenance-chain fields must be non-empty",
                );
            }
        }

        if !self
            .source_domain
            .can_transition_to(self.destination_domain)
        {
            push_violation(
                &mut violations,
                "provenance_chain.domain_transition",
                &self.record_id,
                "provenance-chain domain transition is not allowed",
            );
        }
        if !self.raw_signing_material_excluded {
            push_violation(
                &mut violations,
                "provenance_chain.raw_signing_material_excluded",
                &self.record_id,
                "provenance-chain records must exclude raw signing material",
            );
        }

        violations
    }

    /// Returns true when this transition has no validation violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Metadata-only support/export projection of signing and provenance state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureProjection {
    /// Projection schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable projection id.
    pub projection_id: String,
    /// Exact-build identity set covered by the projection.
    pub exact_build_identity_ref: String,
    /// Artifact bundle ref whose digest set is being projected.
    pub artifact_bundle_ref: String,
    /// Digest-set ref covered by the projection.
    pub digest_set_ref: String,
    /// Signature state copied from the release graph.
    pub signature_state: String,
    /// Attestation state copied from the release graph.
    pub attestation_state: String,
    /// Ordered domain-transition records for build, signing, registry, and mirror custody.
    pub provenance_chain: Vec<ProvenanceChainRecord>,
    /// Whether raw signing material is excluded from the projection.
    pub raw_signing_material_excluded: bool,
}

impl SignatureProjection {
    /// Builds a metadata-only signature projection.
    pub fn new(
        projection_id: impl Into<String>,
        exact_build_identity_ref: impl Into<String>,
        artifact_bundle_ref: impl Into<String>,
        digest_set_ref: impl Into<String>,
        signature_state: impl Into<String>,
        attestation_state: impl Into<String>,
        provenance_chain: Vec<ProvenanceChainRecord>,
    ) -> Self {
        Self {
            schema_version: BUILD_FARM_PROVENANCE_SCHEMA_VERSION,
            record_kind: SIGNATURE_PROJECTION_RECORD_KIND.to_owned(),
            projection_id: projection_id.into(),
            exact_build_identity_ref: exact_build_identity_ref.into(),
            artifact_bundle_ref: artifact_bundle_ref.into(),
            digest_set_ref: digest_set_ref.into(),
            signature_state: signature_state.into(),
            attestation_state: attestation_state.into(),
            provenance_chain,
            raw_signing_material_excluded: true,
        }
    }

    /// Validates the projection and every transition it carries.
    pub fn validate(&self) -> Vec<ProvenanceChainViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUILD_FARM_PROVENANCE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "signature_projection.schema_version",
                &self.projection_id,
                "signature projection schema_version must be 1",
            );
        }
        if self.record_kind != SIGNATURE_PROJECTION_RECORD_KIND {
            push_violation(
                &mut violations,
                "signature_projection.record_kind",
                &self.projection_id,
                "signature projection record kind is not supported",
            );
        }

        for (field, value) in [
            ("projection_id", self.projection_id.as_str()),
            (
                "exact_build_identity_ref",
                self.exact_build_identity_ref.as_str(),
            ),
            ("artifact_bundle_ref", self.artifact_bundle_ref.as_str()),
            ("digest_set_ref", self.digest_set_ref.as_str()),
            ("signature_state", self.signature_state.as_str()),
            ("attestation_state", self.attestation_state.as_str()),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    &mut violations,
                    &format!("signature_projection.{field}"),
                    &self.projection_id,
                    "signature projection fields must be non-empty",
                );
            }
        }

        if self.provenance_chain.is_empty() {
            push_violation(
                &mut violations,
                "signature_projection.provenance_chain",
                &self.projection_id,
                "signature projection must carry at least one provenance-chain record",
            );
        }
        if !self.raw_signing_material_excluded {
            push_violation(
                &mut violations,
                "signature_projection.raw_signing_material_excluded",
                &self.projection_id,
                "signature projection must exclude raw signing material",
            );
        }

        for (index, record) in self.provenance_chain.iter().enumerate() {
            violations.extend(record.validate());
            if record.sequence as usize != index {
                push_violation(
                    &mut violations,
                    "signature_projection.sequence",
                    &record.record_id,
                    "provenance-chain sequence must be contiguous and zero-based",
                );
            }
            if record.exact_build_identity_ref != self.exact_build_identity_ref {
                push_violation(
                    &mut violations,
                    "signature_projection.exact_build_identity_ref",
                    &record.record_id,
                    "provenance-chain exact-build ref must match the signature projection",
                );
            }
            if record.digest_set_ref != self.digest_set_ref {
                push_violation(
                    &mut violations,
                    "signature_projection.digest_set_ref",
                    &record.record_id,
                    "provenance-chain digest-set ref must match the signature projection",
                );
            }
            if index > 0 {
                let previous = &self.provenance_chain[index - 1];
                if previous.destination_domain != record.source_domain {
                    push_violation(
                        &mut violations,
                        "signature_projection.chain_continuity",
                        &record.record_id,
                        "provenance-chain records must join adjacent trust domains",
                    );
                }
            }
        }

        violations
    }

    /// Returns true when the carried chain validates without violations.
    pub fn carries_valid_chain(&self) -> bool {
        self.validate().is_empty()
    }
}

/// One validation violation emitted by the build-farm provenance model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceChainViolation {
    /// Stable check id for the violated rule.
    pub check_id: String,
    /// Ref or record location associated with the violation.
    pub reference: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<ProvenanceChainViolation>,
    check_id: &str,
    reference: &str,
    message: &str,
) {
    violations.push(ProvenanceChainViolation {
        check_id: check_id.to_owned(),
        reference: reference.to_owned(),
        message: message.to_owned(),
    });
}
