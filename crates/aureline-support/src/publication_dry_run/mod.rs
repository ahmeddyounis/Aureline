//! Alpha publication dry-run support projection.
//!
//! This module consumes the checked-in publication manifest at
//! `/artifacts/release/alpha_publication_manifest.yaml` and exposes a
//! metadata-only support/export projection. It keeps clean-room rebuild,
//! mirror-only, deny-all, offline verification, notice, SBOM, provenance,
//! blocker, and live-truth degradation state inspectable without publishing
//! channels or reading raw package bytes.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the alpha publication dry-run manifest.
pub const ALPHA_PUBLICATION_MANIFEST_RECORD_KIND: &str = "alpha_publication_manifest";

/// Current schema version for the alpha publication dry-run manifest.
pub const ALPHA_PUBLICATION_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked-in alpha publication manifest.
pub const CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH: &str =
    "artifacts/release/alpha_publication_manifest.yaml";

const CURRENT_ALPHA_PUBLICATION_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/alpha_publication_manifest.yaml"
));

const REQUIRED_FAMILIES: &[&str] = &[
    "binaries",
    "docs_help_packs",
    "policy_bundles",
    "symbols",
    "support_schemas",
    "notices",
    "sbom_provenance",
];

const REQUIRED_POSTURES: &[&str] = &["mirror_only", "deny_all", "offline_verification"];

const REQUIRED_DEGRADATION_TRUTH: &[&str] = &["live_service_health", "advisory", "revocation"];

/// Loads the checked-in alpha publication manifest.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in artifact does not match
/// [`AlphaPublicationManifest`].
pub fn current_alpha_publication_manifest() -> Result<AlphaPublicationManifest, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_PUBLICATION_MANIFEST_YAML)
}

/// Canonical dry-run manifest joining publication artifact families and proof postures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPublicationManifest {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest id used by support and release evidence.
    pub manifest_id: String,
    /// Manifest revision for additive dry-run changes.
    pub manifest_revision: u32,
    /// UTC timestamp for the manifest revision.
    pub as_of: String,
    /// Owning release/support reviewer.
    pub owner_dri: String,
    /// Manifest state such as `dry_run_blocked_before_broader_publication`.
    pub status: String,
    /// Exact-build identity shared with the alpha artifact graph.
    pub exact_build_identity_ref: String,
    /// Release candidate ref scoped by the dry run.
    pub release_candidate_ref: String,
    /// Artifact bundle ref scoped by the dry run.
    pub artifact_bundle_ref: String,
    /// Whether this manifest allows broader publication.
    pub broader_publication_allowed: bool,
    /// Source contracts consumed by this manifest.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Freshness limits used by mirror/offline review.
    pub freshness_limits: BTreeMap<String, String>,
    /// Required artifact family keys that must have rows and receipts.
    #[serde(default)]
    pub required_artifact_family_keys: Vec<String>,
    /// Artifact family rows covered by the dry run.
    #[serde(default)]
    pub artifact_family_rows: Vec<ArtifactFamilyPublicationRow>,
    /// Publication postures exercised by the dry run.
    #[serde(default)]
    pub publication_postures: Vec<PublicationPosture>,
    /// Verification receipts produced by the dry run.
    #[serde(default)]
    pub verification_receipts: Vec<VerificationReceipt>,
    /// Rules explaining where live truth degrades under mirror/offline review.
    #[serde(default)]
    pub live_truth_degradation_rules: Vec<LiveTruthDegradationRule>,
    /// Publication blockers and review-required gaps.
    #[serde(default)]
    pub blockers: Vec<PublicationBlocker>,
    /// Acceptance metadata for validators.
    pub acceptance: PublicationAcceptance,
}

impl AlphaPublicationManifest {
    /// Validates the manifest's dry-run coverage and honesty invariants.
    pub fn validate(&self) -> Vec<PublicationDryRunViolation> {
        let mut violations = Vec::new();

        if self.schema_version != ALPHA_PUBLICATION_MANIFEST_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "manifest.schema_version",
                &self.manifest_id,
                "alpha publication manifest schema_version must be 1",
            );
        }
        if self.record_kind != ALPHA_PUBLICATION_MANIFEST_RECORD_KIND {
            push_violation(
                &mut violations,
                "manifest.record_kind",
                &self.manifest_id,
                "alpha publication manifest record_kind is not supported",
            );
        }
        if self.broader_publication_allowed {
            push_violation(
                &mut violations,
                "manifest.broader_publication_allowed",
                &self.manifest_id,
                "dry-run manifest must not allow broader publication",
            );
        }
        if !self
            .exact_build_identity_ref
            .starts_with("build-id:aureline:")
        {
            push_violation(
                &mut violations,
                "manifest.exact_build_identity_ref",
                &self.manifest_id,
                "exact_build_identity_ref must be an Aureline build-id ref",
            );
        }

        let required_families = REQUIRED_FAMILIES.iter().copied().collect::<BTreeSet<_>>();
        let declared_families = self
            .required_artifact_family_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for family in required_families.difference(&declared_families) {
            push_violation(
                &mut violations,
                "required_artifact_family_keys.missing",
                family,
                "required artifact family key is missing",
            );
        }

        let mut family_rows = BTreeMap::new();
        for row in &self.artifact_family_rows {
            row.validate(&mut violations);
            if family_rows.insert(row.family_key.as_str(), row).is_some() {
                push_violation(
                    &mut violations,
                    "artifact_family_rows.duplicate",
                    &row.family_key,
                    "artifact family rows must be unique",
                );
            }
        }
        for family in REQUIRED_FAMILIES {
            if !family_rows.contains_key(family) {
                push_violation(
                    &mut violations,
                    "artifact_family_rows.missing",
                    family,
                    "required artifact family row is missing",
                );
            }
        }

        let mut posture_by_class = BTreeMap::new();
        for posture in &self.publication_postures {
            posture.validate(&mut violations);
            posture_by_class.insert(posture.posture_class.as_str(), posture);
        }
        for posture in REQUIRED_POSTURES {
            if !posture_by_class.contains_key(posture) {
                push_violation(
                    &mut violations,
                    "publication_postures.missing",
                    posture,
                    "required publication posture is missing",
                );
            }
        }

        let posture_ids = self
            .publication_postures
            .iter()
            .map(|posture| posture.posture_id.as_str())
            .collect::<BTreeSet<_>>();
        let mut receipt_coverage = BTreeSet::new();
        for receipt in &self.verification_receipts {
            receipt.validate(&posture_ids, &mut violations);
            let posture_class = receipt
                .posture_ref
                .strip_prefix("publication_posture:")
                .unwrap_or(receipt.posture_ref.as_str());
            for family in &receipt.artifact_family_keys {
                receipt_coverage.insert((posture_class.to_owned(), family.as_str()));
            }
        }
        for posture in REQUIRED_POSTURES {
            for family in REQUIRED_FAMILIES {
                if !receipt_coverage.contains(&(posture.to_string(), *family)) {
                    push_violation(
                        &mut violations,
                        "verification_receipts.coverage",
                        &format!("{posture}:{family}"),
                        "required posture and artifact family receipt is missing",
                    );
                }
            }
        }

        let degradation_truth = self
            .live_truth_degradation_rules
            .iter()
            .map(|rule| rule.truth_class.as_str())
            .collect::<BTreeSet<_>>();
        for truth in REQUIRED_DEGRADATION_TRUTH {
            if !degradation_truth.contains(truth) {
                push_violation(
                    &mut violations,
                    "live_truth_degradation_rules.missing",
                    truth,
                    "required live-truth degradation rule is missing",
                );
            }
        }
        for rule in &self.live_truth_degradation_rules {
            rule.validate(&self.freshness_limits, &mut violations);
        }

        if !self
            .blockers
            .iter()
            .any(|blocker| blocker.blocks_broader_publication)
        {
            push_violation(
                &mut violations,
                "blockers.blocks_broader_publication",
                &self.manifest_id,
                "at least one blocker must block broader publication",
            );
        }
        let blocker_ids = self
            .blockers
            .iter()
            .map(|blocker| blocker.blocker_id.as_str())
            .collect::<BTreeSet<_>>();
        for blocker in &self.blockers {
            blocker.validate(&mut violations);
        }
        for row in &self.artifact_family_rows {
            for blocker_ref in &row.blocker_refs {
                if !blocker_ids.contains(blocker_ref.as_str()) {
                    push_violation(
                        &mut violations,
                        "artifact_family_rows.blocker_refs",
                        blocker_ref,
                        "artifact family row references an unknown blocker",
                    );
                }
            }
        }

        violations
    }

    /// Returns true when every required artifact family stays verifiable without vendor reachability.
    pub fn all_required_families_are_vendor_unreachable_verifiable(&self) -> bool {
        let verifiable = self
            .artifact_family_rows
            .iter()
            .filter(|row| row.verifiable_without_vendor_reachability)
            .map(|row| row.family_key.as_str())
            .collect::<BTreeSet<_>>();
        REQUIRED_FAMILIES
            .iter()
            .all(|family| verifiable.contains(family))
    }

    /// Returns true when mirror-only, deny-all, and offline verification postures are present.
    pub fn exercises_required_postures(&self) -> bool {
        let postures = self
            .publication_postures
            .iter()
            .map(|posture| posture.posture_class.as_str())
            .collect::<BTreeSet<_>>();
        REQUIRED_POSTURES
            .iter()
            .all(|posture| postures.contains(posture))
    }

    /// Builds a support/export summary without package bytes or private material.
    pub fn support_projection(&self) -> PublicationDryRunSupportProjection {
        PublicationDryRunSupportProjection {
            manifest_id: self.manifest_id.clone(),
            exact_build_identity_ref: self.exact_build_identity_ref.clone(),
            required_family_keys: self.required_artifact_family_keys.clone(),
            posture_classes: self
                .publication_postures
                .iter()
                .map(|posture| posture.posture_class.clone())
                .collect(),
            receipt_count: self.verification_receipts.len(),
            blocking_blocker_count: self
                .blockers
                .iter()
                .filter(|blocker| blocker.blocks_broader_publication)
                .count(),
            vendor_unreachable_family_keys: self
                .artifact_family_rows
                .iter()
                .filter(|row| row.verifiable_without_vendor_reachability)
                .map(|row| row.family_key.clone())
                .collect(),
            live_truth_degradation_classes: self
                .live_truth_degradation_rules
                .iter()
                .map(|rule| rule.truth_class.clone())
                .collect(),
            raw_private_material_excluded: true,
        }
    }
}

/// One artifact-family row covered by the publication dry run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFamilyPublicationRow {
    /// Required family key such as `binaries` or `notices`.
    pub family_key: String,
    /// Reviewer-facing family name.
    pub display_name: String,
    /// Artifact subject classes aligned with mirror/offline verification.
    #[serde(default)]
    pub subject_classes: Vec<String>,
    /// Release artifact family classes aligned with publish-target vocabulary.
    #[serde(default)]
    pub artifact_family_classes: Vec<String>,
    /// Alpha artifact graph node refs covered by the row.
    #[serde(default)]
    pub graph_node_refs: Vec<String>,
    /// Source refs that make the row reviewable.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// Source refs whose bytes or contents are comparable in the dry run.
    #[serde(default)]
    pub digest_material_refs: Vec<String>,
    /// Verification receipts associated with the row.
    #[serde(default)]
    pub verification_receipt_refs: Vec<String>,
    /// Mirror-integrity packet refs associated with the row.
    #[serde(default)]
    pub mirror_integrity_packet_refs: Vec<String>,
    /// Offline-verification packet refs associated with the row.
    #[serde(default)]
    pub offline_verification_packet_refs: Vec<String>,
    /// Import instruction refs associated with the row.
    #[serde(default)]
    pub import_instruction_refs: Vec<String>,
    /// Whether this family remains verifiable without vendor reachability.
    pub verifiable_without_vendor_reachability: bool,
    /// Human-readable degradation statement for mirror/offline review.
    pub live_truth_degradation: String,
    /// Blockers or review-required gaps attached to the row.
    #[serde(default)]
    pub blocker_refs: Vec<String>,
}

impl ArtifactFamilyPublicationRow {
    fn validate(&self, violations: &mut Vec<PublicationDryRunViolation>) {
        for (field, value) in [
            ("family_key", &self.family_key),
            ("display_name", &self.display_name),
            ("live_truth_degradation", &self.live_truth_degradation),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("artifact_family_rows.{field}"),
                    &self.family_key,
                    "artifact family row field must be non-empty",
                );
            }
        }
        for (field, len) in [
            ("subject_classes", self.subject_classes.len()),
            (
                "artifact_family_classes",
                self.artifact_family_classes.len(),
            ),
            ("source_refs", self.source_refs.len()),
            ("digest_material_refs", self.digest_material_refs.len()),
            (
                "verification_receipt_refs",
                self.verification_receipt_refs.len(),
            ),
            (
                "import_instruction_refs",
                self.import_instruction_refs.len(),
            ),
            ("blocker_refs", self.blocker_refs.len()),
        ] {
            if len == 0 {
                push_violation(
                    violations,
                    &format!("artifact_family_rows.{field}"),
                    &self.family_key,
                    "artifact family row collection must be non-empty",
                );
            }
        }
        if !self.verifiable_without_vendor_reachability {
            push_violation(
                violations,
                "artifact_family_rows.verifiable_without_vendor_reachability",
                &self.family_key,
                "artifact family must declare vendor-unreachable verification",
            );
        }
        if self.family_key == "policy_bundles" && !self.graph_node_refs.is_empty() {
            push_violation(
                violations,
                "artifact_family_rows.policy_graph_gap",
                &self.family_key,
                "policy bundle graph linkage gap must remain explicit in this dry run",
            );
        }
    }
}

/// One publication posture exercised by the dry run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationPosture {
    /// Stable posture id.
    pub posture_id: String,
    /// Posture class such as `mirror_only`.
    pub posture_class: String,
    /// Connectivity class required by the posture.
    pub connectivity_class: String,
    /// Whether the posture may mutate publication state.
    pub publication_mutations_allowed: bool,
    /// Whether vendor reachability is required to complete validation.
    pub vendor_reachability_required: bool,
    /// Import instructions for this posture.
    #[serde(default)]
    pub import_instructions: Vec<ImportInstruction>,
    /// Receipt refs that prove this posture.
    #[serde(default)]
    pub verification_receipt_refs: Vec<String>,
    /// Freshness limit for posture-level metadata.
    pub freshness_limit: String,
    /// Statements naming live-truth degradation under this posture.
    #[serde(default)]
    pub degraded_truth: Vec<String>,
}

impl PublicationPosture {
    fn validate(&self, violations: &mut Vec<PublicationDryRunViolation>) {
        for (field, value) in [
            ("posture_id", &self.posture_id),
            ("posture_class", &self.posture_class),
            ("connectivity_class", &self.connectivity_class),
            ("freshness_limit", &self.freshness_limit),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("publication_postures.{field}"),
                    &self.posture_id,
                    "publication posture field must be non-empty",
                );
            }
        }
        if self.publication_mutations_allowed {
            push_violation(
                violations,
                "publication_postures.publication_mutations_allowed",
                &self.posture_id,
                "dry-run posture must not allow publication mutations",
            );
        }
        if self.vendor_reachability_required {
            push_violation(
                violations,
                "publication_postures.vendor_reachability_required",
                &self.posture_id,
                "dry-run posture must not require vendor reachability",
            );
        }
        if self.import_instructions.is_empty() {
            push_violation(
                violations,
                "publication_postures.import_instructions",
                &self.posture_id,
                "publication posture must include import instructions",
            );
        }
        if self.verification_receipt_refs.is_empty() {
            push_violation(
                violations,
                "publication_postures.verification_receipt_refs",
                &self.posture_id,
                "publication posture must include receipt refs",
            );
        }
        if self.degraded_truth.is_empty() {
            push_violation(
                violations,
                "publication_postures.degraded_truth",
                &self.posture_id,
                "publication posture must name degraded truth",
            );
        }
        for instruction in &self.import_instructions {
            instruction.validate(&self.posture_id, violations);
        }
    }
}

/// One import instruction attached to a publication posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportInstruction {
    /// Stable import instruction id.
    pub instruction_id: String,
    /// Action class such as `import_offline_bundle`.
    pub action_class: String,
    /// Artifact family keys covered by the instruction.
    #[serde(default)]
    pub artifact_family_keys: Vec<String>,
    /// Command ref that validates the instruction.
    pub command_ref: String,
    /// Expected receipt refs produced by the instruction.
    #[serde(default)]
    pub expected_receipt_refs: Vec<String>,
}

impl ImportInstruction {
    fn validate(&self, posture_id: &str, violations: &mut Vec<PublicationDryRunViolation>) {
        for (field, value) in [
            ("instruction_id", &self.instruction_id),
            ("action_class", &self.action_class),
            ("command_ref", &self.command_ref),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("publication_postures.import_instructions.{field}"),
                    posture_id,
                    "import instruction field must be non-empty",
                );
            }
        }
        if self.artifact_family_keys.is_empty() {
            push_violation(
                violations,
                "publication_postures.import_instructions.artifact_family_keys",
                &self.instruction_id,
                "import instruction must name artifact family keys",
            );
        }
        if self.expected_receipt_refs.is_empty() {
            push_violation(
                violations,
                "publication_postures.import_instructions.expected_receipt_refs",
                &self.instruction_id,
                "import instruction must name expected receipts",
            );
        }
    }
}

/// One verification receipt produced by a posture for one or more families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationReceipt {
    /// Stable receipt id.
    pub receipt_id: String,
    /// Publication posture ref that produced the receipt.
    pub posture_ref: String,
    /// Receipt class such as `mirror_receipt`.
    pub receipt_class: String,
    /// Result class such as `dry_run_verified_with_blockers`.
    pub result_class: String,
    /// Artifact families covered by the receipt.
    #[serde(default)]
    pub artifact_family_keys: Vec<String>,
    /// Evidence refs cited by the receipt.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Freshness limit for the receipt.
    pub freshness_limit: String,
    /// Whether vendor reachability is required for the receipt.
    pub vendor_reachability_required: bool,
    /// Whether the receipt permits publication mutations.
    pub publication_mutations_allowed: bool,
    /// Redaction-safe reviewer notes.
    pub notes: String,
}

impl VerificationReceipt {
    fn validate(
        &self,
        posture_ids: &BTreeSet<&str>,
        violations: &mut Vec<PublicationDryRunViolation>,
    ) {
        for (field, value) in [
            ("receipt_id", &self.receipt_id),
            ("posture_ref", &self.posture_ref),
            ("receipt_class", &self.receipt_class),
            ("result_class", &self.result_class),
            ("freshness_limit", &self.freshness_limit),
            ("notes", &self.notes),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("verification_receipts.{field}"),
                    &self.receipt_id,
                    "verification receipt field must be non-empty",
                );
            }
        }
        if !posture_ids.contains(self.posture_ref.as_str()) {
            push_violation(
                violations,
                "verification_receipts.posture_ref",
                &self.receipt_id,
                "verification receipt references an unknown posture",
            );
        }
        if self.artifact_family_keys.is_empty() {
            push_violation(
                violations,
                "verification_receipts.artifact_family_keys",
                &self.receipt_id,
                "verification receipt must name artifact family keys",
            );
        }
        if self.evidence_refs.is_empty() {
            push_violation(
                violations,
                "verification_receipts.evidence_refs",
                &self.receipt_id,
                "verification receipt must cite evidence refs",
            );
        }
        if self.vendor_reachability_required {
            push_violation(
                violations,
                "verification_receipts.vendor_reachability_required",
                &self.receipt_id,
                "verification receipt must not require vendor reachability",
            );
        }
        if self.publication_mutations_allowed {
            push_violation(
                violations,
                "verification_receipts.publication_mutations_allowed",
                &self.receipt_id,
                "verification receipt must not allow publication mutations",
            );
        }
    }
}

/// One rule explaining mirror/offline live-truth degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTruthDegradationRule {
    /// Stable degradation rule id.
    pub rule_id: String,
    /// Truth class such as `advisory` or `revocation`.
    pub truth_class: String,
    /// State that surfaces must render under mirror/offline review.
    pub offline_or_mirror_state: String,
    /// Freshness limit key referenced by this degradation rule.
    pub freshness_limit_ref: String,
    /// Required surface copy or summary.
    pub required_surface_copy: String,
}

impl LiveTruthDegradationRule {
    fn validate(
        &self,
        freshness_limits: &BTreeMap<String, String>,
        violations: &mut Vec<PublicationDryRunViolation>,
    ) {
        for (field, value) in [
            ("rule_id", &self.rule_id),
            ("truth_class", &self.truth_class),
            ("offline_or_mirror_state", &self.offline_or_mirror_state),
            ("freshness_limit_ref", &self.freshness_limit_ref),
            ("required_surface_copy", &self.required_surface_copy),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("live_truth_degradation_rules.{field}"),
                    &self.rule_id,
                    "live-truth degradation rule field must be non-empty",
                );
            }
        }
        if !freshness_limits.contains_key(&self.freshness_limit_ref) {
            push_violation(
                violations,
                "live_truth_degradation_rules.freshness_limit_ref",
                &self.rule_id,
                "live-truth degradation rule references an unknown freshness limit",
            );
        }
    }
}

/// One blocker or review-required gap in the publication dry run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationBlocker {
    /// Stable blocker id.
    pub blocker_id: String,
    /// Severity class for review.
    pub severity: String,
    /// Whether the blocker prevents broader publication.
    pub blocks_broader_publication: bool,
    /// Short blocker summary.
    pub summary: String,
    /// Condition that closes the blocker.
    pub closure_condition: String,
    /// Evidence refs associated with the blocker.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl PublicationBlocker {
    fn validate(&self, violations: &mut Vec<PublicationDryRunViolation>) {
        for (field, value) in [
            ("blocker_id", &self.blocker_id),
            ("severity", &self.severity),
            ("summary", &self.summary),
            ("closure_condition", &self.closure_condition),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    violations,
                    &format!("blockers.{field}"),
                    &self.blocker_id,
                    "blocker field must be non-empty",
                );
            }
        }
        if self.evidence_refs.is_empty() {
            push_violation(
                violations,
                "blockers.evidence_refs",
                &self.blocker_id,
                "blocker must cite evidence refs",
            );
        }
    }
}

/// Acceptance metadata for publication dry-run validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationAcceptance {
    /// Commands that validate the publication dry run.
    #[serde(default)]
    pub validation_commands: Vec<String>,
    /// Protected proof refs consumed by this lane.
    #[serde(default)]
    pub protected_proof_refs: Vec<String>,
    /// Accepted state names proven by the dry run.
    #[serde(default)]
    pub accepted_states: Vec<String>,
}

/// Support/export projection over the publication dry-run manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationDryRunSupportProjection {
    /// Manifest id projected into support/export review.
    pub manifest_id: String,
    /// Exact-build identity ref projected into support/export review.
    pub exact_build_identity_ref: String,
    /// Required artifact family keys covered by the dry run.
    pub required_family_keys: Vec<String>,
    /// Publication posture classes exercised by the dry run.
    pub posture_classes: Vec<String>,
    /// Number of verification receipts in the manifest.
    pub receipt_count: usize,
    /// Number of blockers that prevent broader publication.
    pub blocking_blocker_count: usize,
    /// Families that remain verifiable without vendor reachability.
    pub vendor_unreachable_family_keys: Vec<String>,
    /// Live-truth classes that intentionally degrade offline.
    pub live_truth_degradation_classes: Vec<String>,
    /// Whether raw private material is excluded from the projection.
    pub raw_private_material_excluded: bool,
}

/// One validation violation emitted by the publication dry-run consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationDryRunViolation {
    /// Stable check id for the violated rule.
    pub check_id: String,
    /// Ref or manifest location associated with the violation.
    pub reference: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<PublicationDryRunViolation>,
    check_id: &str,
    reference: &str,
    message: &str,
) {
    violations.push(PublicationDryRunViolation {
        check_id: check_id.to_owned(),
        reference: reference.to_owned(),
        message: message.to_owned(),
    });
}
