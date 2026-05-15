//! Alpha publication dry-run support projection.
//!
//! This module consumes the checked-in publication manifest at
//! `/artifacts/release/alpha_publication_manifest.yaml` and exposes a
//! metadata-only support/export projection. It keeps clean-room rebuild,
//! mirror-only, deny-all, offline verification, notice, SBOM, provenance,
//! blocker, and live-truth degradation state inspectable without publishing
//! channels or reading raw package bytes.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the alpha publication dry-run manifest.
pub const ALPHA_PUBLICATION_MANIFEST_RECORD_KIND: &str = "alpha_publication_manifest";

/// Current schema version for the alpha publication dry-run manifest.
pub const ALPHA_PUBLICATION_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked-in alpha publication manifest.
pub const CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH: &str =
    "artifacts/release/alpha_publication_manifest.yaml";

/// Stable record-kind tag for the clean-room rebuild projection record.
pub const CLEAN_ROOM_REBUILD_PROJECTION_RECORD_KIND: &str = "clean_room_rebuild_projection";

/// Current schema version for the clean-room rebuild projection record.
pub const CLEAN_ROOM_REBUILD_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the benchmark and publication rehearsal methodology.
pub const CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_PATH: &str =
    "artifacts/benchmarks/m2_publication_rehearsal.md";

/// Repository-relative path to the external alpha known-limits packet.
pub const CURRENT_ALPHA_KNOWN_LIMITS_PATH: &str = "artifacts/milestones/m2/known_limits_alpha.yaml";

/// Known-limit id that keeps publication rehearsal evidence methodology-only.
pub const PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID: &str =
    "known_limit:external_alpha.publication_rehearsal_methodology_only";

const CURRENT_ALPHA_PUBLICATION_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/alpha_publication_manifest.yaml"
));

const CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_MARKDOWN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/benchmarks/m2_publication_rehearsal.md"
));

const CURRENT_ALPHA_KNOWN_LIMITS_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/milestones/m2/known_limits_alpha.yaml"
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

/// Loads the checked-in publication rehearsal methodology.
///
/// # Errors
///
/// Returns a parse error when the checked-in Markdown packet no longer has
/// the expected methodology tables or required fields.
pub fn current_publication_rehearsal_methodology(
) -> Result<PublicationRehearsalMethodology, PublicationRehearsalParseError> {
    PublicationRehearsalMethodology::from_markdown(
        CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_MARKDOWN,
    )
}

/// Loads the checked-in external alpha known-limits packet.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in artifact does not match
/// [`KnownLimitsAlphaPacket`].
pub fn current_alpha_known_limits_packet() -> Result<KnownLimitsAlphaPacket, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_KNOWN_LIMITS_YAML)
}

/// Builds the checked-in clean-room rebuild projection after validating inputs.
///
/// # Errors
///
/// Returns an error when the publication manifest, rehearsal methodology, or
/// known-limits packet cannot be loaded, or when their combined projection
/// would overclaim beyond methodology-only evidence.
pub fn current_clean_room_rebuild_projection(
) -> Result<CleanRoomRebuildProjection, CleanRoomProjectionError> {
    let manifest =
        current_alpha_publication_manifest().map_err(CleanRoomProjectionError::Manifest)?;
    let methodology =
        current_publication_rehearsal_methodology().map_err(CleanRoomProjectionError::Rehearsal)?;
    let known_limits =
        current_alpha_known_limits_packet().map_err(CleanRoomProjectionError::KnownLimits)?;

    let mut violations = manifest.validate();
    violations.extend(methodology.validate(&known_limits));
    let projection = methodology.clean_room_rebuild_projection(&manifest, &known_limits);
    violations.extend(projection.validate());

    if violations.is_empty() {
        Ok(projection)
    } else {
        Err(CleanRoomProjectionError::Validation(violations))
    }
}

/// Error emitted when the publication rehearsal Markdown cannot be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationRehearsalParseError {
    /// Markdown section associated with the parse failure.
    pub section: String,
    /// Redaction-safe parse failure message.
    pub message: String,
}

impl PublicationRehearsalParseError {
    fn new(section: &str, message: &str) -> Self {
        Self {
            section: section.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for PublicationRehearsalParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.section, self.message)
    }
}

impl Error for PublicationRehearsalParseError {}

/// Error emitted when the clean-room rebuild projection cannot be built.
#[derive(Debug)]
pub enum CleanRoomProjectionError {
    /// The alpha publication manifest could not be loaded.
    Manifest(serde_yaml::Error),
    /// The publication rehearsal methodology could not be loaded.
    Rehearsal(PublicationRehearsalParseError),
    /// The external alpha known-limits packet could not be loaded.
    KnownLimits(serde_yaml::Error),
    /// The loaded inputs would produce an invalid or overclaiming projection.
    Validation(Vec<PublicationDryRunViolation>),
}

impl fmt::Display for CleanRoomProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manifest(error) => write!(formatter, "publication manifest: {error}"),
            Self::Rehearsal(error) => write!(formatter, "publication rehearsal: {error}"),
            Self::KnownLimits(error) => write!(formatter, "known limits: {error}"),
            Self::Validation(violations) => write!(
                formatter,
                "clean-room rebuild projection has {} validation violation(s)",
                violations.len()
            ),
        }
    }
}

impl Error for CleanRoomProjectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Manifest(error) | Self::KnownLimits(error) => Some(error),
            Self::Rehearsal(error) => Some(error),
            Self::Validation(_) => None,
        }
    }
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

/// Parsed benchmark and publication rehearsal methodology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRehearsalMethodology {
    /// Stable packet id declared in the rehearsal header.
    pub packet_id: String,
    /// Packet state such as `methodology_only`.
    pub packet_state: String,
    /// UTC capture timestamp declared by the rehearsal.
    pub captured_at: String,
    /// Exact-build identity artifact referenced by the methodology packet.
    pub methodology_exact_build_identity_ref: String,
    /// Rehearsal checklist referenced by the methodology packet.
    pub rehearsal_checklist_ref: String,
    /// Fixture register referenced by the methodology packet.
    pub fixture_register_ref: String,
    /// Reference-workspace dry run referenced by the methodology packet.
    pub reference_dry_run_ref: String,
    /// Known-limits packet referenced by the methodology packet.
    pub known_limits_packet_ref: String,
    /// Validator referenced by the methodology packet.
    pub validator_ref: String,
    /// Lane-level publication decisions from the methodology packet.
    pub lane_decisions: Vec<PublicationRehearsalLaneDecision>,
    /// Checklist coverage rows from the methodology packet.
    pub checklist_coverage: Vec<PublicationRehearsalChecklistCoverage>,
    /// Bundle and fixture bindings covered by the methodology packet.
    pub bundle_bindings: Vec<PublicationRehearsalBundleBinding>,
    /// Known-limit refs explicitly attached to the methodology packet.
    pub known_limit_refs: Vec<String>,
    /// Artifact refs that trigger methodology refresh.
    pub refresh_trigger_refs: Vec<String>,
    /// First consumer command published by the methodology packet.
    pub first_consumer_command: String,
}

impl PublicationRehearsalMethodology {
    fn from_markdown(markdown: &str) -> Result<Self, PublicationRehearsalParseError> {
        let packet_header_section = markdown_section(markdown, "Packet Header")?;
        let header_rows =
            parse_markdown_table("Packet Header", packet_header_section, &["Field", "Value"])?;
        let mut packet_header = BTreeMap::new();
        for row in header_rows {
            let field = required_table_value(&row, "Field", "Packet Header")?;
            let value = required_table_value(&row, "Value", "Packet Header")?;
            packet_header.insert(field, value);
        }

        let decision_section = markdown_section(markdown, "Rehearsal Decision")?;
        let lane_decisions = parse_markdown_table(
            "Rehearsal Decision",
            decision_section,
            &["Lane", "Result", "Reason"],
        )?
        .into_iter()
        .map(|row| {
            Ok(PublicationRehearsalLaneDecision {
                lane: required_table_value(&row, "Lane", "Rehearsal Decision")?,
                result: required_table_value(&row, "Result", "Rehearsal Decision")?,
                reason: required_table_value(&row, "Reason", "Rehearsal Decision")?,
            })
        })
        .collect::<Result<Vec<_>, PublicationRehearsalParseError>>()?;

        let checklist_section = markdown_section(markdown, "Checklist Coverage")?;
        let checklist_coverage = parse_markdown_table(
            "Checklist Coverage",
            checklist_section,
            &["Checklist group", "Dry-run state", "Evidence"],
        )?
        .into_iter()
        .map(|row| {
            Ok(PublicationRehearsalChecklistCoverage {
                checklist_group: required_table_value(
                    &row,
                    "Checklist group",
                    "Checklist Coverage",
                )?,
                dry_run_state: required_table_value(&row, "Dry-run state", "Checklist Coverage")?,
                evidence: required_table_value(&row, "Evidence", "Checklist Coverage")?,
            })
        })
        .collect::<Result<Vec<_>, PublicationRehearsalParseError>>()?;

        let bundle_section = markdown_section(markdown, "Bundle and Fixture Binding")?;
        let bundle_bindings = parse_markdown_table(
            "Bundle and Fixture Binding",
            bundle_section,
            &[
                "Bundle",
                "Fixture register row",
                "Corpus refs",
                "Publication result",
            ],
        )?
        .into_iter()
        .map(|row| {
            let corpus_refs =
                required_table_value(&row, "Corpus refs", "Bundle and Fixture Binding")?
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect();

            Ok(PublicationRehearsalBundleBinding {
                bundle_ref: required_table_value(&row, "Bundle", "Bundle and Fixture Binding")?,
                fixture_register_row_ref: required_table_value(
                    &row,
                    "Fixture register row",
                    "Bundle and Fixture Binding",
                )?,
                corpus_refs,
                publication_result: required_table_value(
                    &row,
                    "Publication result",
                    "Bundle and Fixture Binding",
                )?,
            })
        })
        .collect::<Result<Vec<_>, PublicationRehearsalParseError>>()?;

        let known_limits_section = markdown_section(markdown, "Known Limits and Exclusions")?;
        let known_limit_refs = extract_backtick_values(known_limits_section)
            .into_iter()
            .filter(|value| value.starts_with("known_limit:"))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let refresh_section = markdown_section(markdown, "Refresh Trigger")?;
        let refresh_trigger_refs = extract_backtick_values(refresh_section)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let first_consumer_section = markdown_section(markdown, "First Consumer")?;
        let first_consumer_command =
            first_code_block_line(first_consumer_section).ok_or_else(|| {
                PublicationRehearsalParseError::new(
                    "First Consumer",
                    "first consumer command block is missing",
                )
            })?;

        Ok(Self {
            packet_id: required_header_value(&packet_header, "Packet id")?,
            packet_state: required_header_value(&packet_header, "Packet state")?,
            captured_at: required_header_value(&packet_header, "Captured at")?,
            methodology_exact_build_identity_ref: required_header_value(
                &packet_header,
                "Exact build identity",
            )?,
            rehearsal_checklist_ref: required_header_value(&packet_header, "Rehearsal checklist")?,
            fixture_register_ref: required_header_value(&packet_header, "Fixture register")?,
            reference_dry_run_ref: required_header_value(&packet_header, "Reference dry run")?,
            known_limits_packet_ref: required_header_value(&packet_header, "Known-limits packet")?,
            validator_ref: required_header_value(&packet_header, "Validator")?,
            lane_decisions,
            checklist_coverage,
            bundle_bindings,
            known_limit_refs,
            refresh_trigger_refs,
            first_consumer_command,
        })
    }

    /// Validates that the methodology packet stays bounded to methodology-only claims.
    pub fn validate(
        &self,
        known_limits: &KnownLimitsAlphaPacket,
    ) -> Vec<PublicationDryRunViolation> {
        let mut violations = Vec::new();

        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("packet_state", &self.packet_state),
            (
                "methodology_exact_build_identity_ref",
                &self.methodology_exact_build_identity_ref,
            ),
            ("rehearsal_checklist_ref", &self.rehearsal_checklist_ref),
            ("fixture_register_ref", &self.fixture_register_ref),
            ("reference_dry_run_ref", &self.reference_dry_run_ref),
            ("known_limits_packet_ref", &self.known_limits_packet_ref),
            ("validator_ref", &self.validator_ref),
            ("first_consumer_command", &self.first_consumer_command),
        ] {
            if value.trim().is_empty() {
                push_violation(
                    &mut violations,
                    &format!("publication_rehearsal.{field}"),
                    &self.packet_id,
                    "publication rehearsal methodology field must be non-empty",
                );
            }
        }

        if self.packet_state != "methodology_only" {
            push_violation(
                &mut violations,
                "publication_rehearsal.packet_state",
                &self.packet_id,
                "publication rehearsal packet must remain methodology-only",
            );
        }
        if self.known_limits_packet_ref != CURRENT_ALPHA_KNOWN_LIMITS_PATH {
            push_violation(
                &mut violations,
                "publication_rehearsal.known_limits_packet_ref",
                &self.known_limits_packet_ref,
                "publication rehearsal must cite the current external alpha known-limits packet",
            );
        }
        if self.rehearsal_checklist_ref != "artifacts/bench/publication_rehearsal_checklist.yaml" {
            push_violation(
                &mut violations,
                "publication_rehearsal.rehearsal_checklist_ref",
                &self.rehearsal_checklist_ref,
                "publication rehearsal must cite the canonical rehearsal checklist",
            );
        }
        if self.validator_ref != "ci/check_reference_workspace_dry_run.py" {
            push_violation(
                &mut violations,
                "publication_rehearsal.validator_ref",
                &self.validator_ref,
                "publication rehearsal must cite the reference-workspace dry-run validator",
            );
        }

        for (lane, expected_result) in [
            ("benchmark", "keep_methodology_only"),
            ("public_proof", "keep_methodology_only"),
            ("docs_known_limits_support", "narrow_claim_before_publish"),
        ] {
            match self.lane_result(lane) {
                Some(result) if result == expected_result => {}
                Some(result) => push_violation(
                    &mut violations,
                    "publication_rehearsal.lane_decision",
                    lane,
                    &format!(
                        "publication rehearsal lane must remain {expected_result}, found {result}"
                    ),
                ),
                None => push_violation(
                    &mut violations,
                    "publication_rehearsal.lane_decision.missing",
                    lane,
                    "publication rehearsal lane decision is missing",
                ),
            }
        }

        if self.bundle_bindings.is_empty() {
            push_violation(
                &mut violations,
                "publication_rehearsal.bundle_bindings",
                &self.packet_id,
                "publication rehearsal must name bundle bindings",
            );
        }
        for binding in &self.bundle_bindings {
            if binding.publication_result != "keep_methodology_only" {
                push_violation(
                    &mut violations,
                    "publication_rehearsal.bundle_bindings.publication_result",
                    &binding.bundle_ref,
                    "bundle binding must remain methodology-only",
                );
            }
            if binding.corpus_refs.is_empty() {
                push_violation(
                    &mut violations,
                    "publication_rehearsal.bundle_bindings.corpus_refs",
                    &binding.bundle_ref,
                    "bundle binding must name corpus refs",
                );
            }
        }

        if !self
            .known_limit_refs
            .iter()
            .any(|reference| reference == PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID)
        {
            push_violation(
                &mut violations,
                "publication_rehearsal.known_limit_refs",
                &self.packet_id,
                "publication rehearsal must attach the methodology-only known limit",
            );
        }
        if !self
            .refresh_trigger_refs
            .iter()
            .any(|reference| reference == CURRENT_ALPHA_KNOWN_LIMITS_PATH)
        {
            push_violation(
                &mut violations,
                "publication_rehearsal.refresh_trigger_refs",
                &self.packet_id,
                "publication rehearsal must refresh when known limits change",
            );
        }

        violations.extend(known_limits.validate_methodology_only_limit());
        violations
    }

    /// Returns true when the packet does not admit benchmark or public-proof claims.
    pub fn is_methodology_only(&self) -> bool {
        self.packet_state == "methodology_only"
            && self.lane_result("benchmark") == Some("keep_methodology_only")
            && self.lane_result("public_proof") == Some("keep_methodology_only")
            && self
                .bundle_bindings
                .iter()
                .all(|binding| binding.publication_result == "keep_methodology_only")
            && self
                .known_limit_refs
                .iter()
                .any(|reference| reference == PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID)
    }

    /// Projects the methodology packet into a clean-room rebuild support record.
    pub fn clean_room_rebuild_projection(
        &self,
        manifest: &AlphaPublicationManifest,
        known_limits: &KnownLimitsAlphaPacket,
    ) -> CleanRoomRebuildProjection {
        let support_projection = manifest.support_projection();
        let known_limit = known_limits.methodology_only_limit();
        let mut source_refs = BTreeSet::from([
            CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH.to_owned(),
            CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_PATH.to_owned(),
            CURRENT_ALPHA_KNOWN_LIMITS_PATH.to_owned(),
            self.methodology_exact_build_identity_ref.clone(),
            self.rehearsal_checklist_ref.clone(),
            self.fixture_register_ref.clone(),
            self.reference_dry_run_ref.clone(),
            self.validator_ref.clone(),
        ]);
        for key in [
            "clean_room_rebuild_dry_run",
            "mirror_offline_publication_dry_run",
            "publication_validator",
            "cleanroom_rebuild_entrypoint",
        ] {
            if let Some(reference) = manifest.source_contract_refs.get(key) {
                source_refs.insert(reference.clone());
            }
        }

        let mut explicit_non_claims =
            BTreeSet::from(["actual_clean_room_rebuild_execution".to_owned()]);
        if let Some(limit) = known_limit {
            explicit_non_claims.extend(limit.explicit_exclusions.iter().cloned());
        }

        CleanRoomRebuildProjection {
            schema_version: CLEAN_ROOM_REBUILD_PROJECTION_SCHEMA_VERSION,
            record_kind: CLEAN_ROOM_REBUILD_PROJECTION_RECORD_KIND.to_owned(),
            projection_id: self.packet_id.replacen(
                "publication_rehearsal",
                "clean_room_rebuild_projection",
                1,
            ),
            projection_state: "alpha_methodology_only_projection".to_owned(),
            projection_only: true,
            actual_clean_room_rebuild_executed: false,
            rebuild_execution_state: "not_executed_methodology_projection".to_owned(),
            rehearsal_packet_id: self.packet_id.clone(),
            rehearsal_packet_state: self.packet_state.clone(),
            exact_build_identity_ref: support_projection.exact_build_identity_ref,
            methodology_exact_build_identity_ref: self.methodology_exact_build_identity_ref.clone(),
            methodology_only_known_limit_ref: PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID
                .to_owned(),
            known_limit_note_state: known_limit
                .map(|limit| limit.note_state.clone())
                .unwrap_or_default(),
            known_limit_summary: known_limit
                .map(|limit| limit.partner_summary.clone())
                .unwrap_or_default(),
            methodology_source_refs: source_refs.into_iter().collect(),
            projected_artifact_family_keys: support_projection.required_family_keys,
            publication_posture_classes: support_projection.posture_classes,
            vendor_unreachable_family_keys: support_projection.vendor_unreachable_family_keys,
            receipt_count: support_projection.receipt_count,
            blocking_blocker_count: support_projection.blocking_blocker_count,
            methodology_lane_results: self
                .lane_decisions
                .iter()
                .map(|decision| CleanRoomProjectionLaneResult {
                    lane: decision.lane.clone(),
                    result: decision.result.clone(),
                    projection_acceptance: if decision.result == "keep_methodology_only" {
                        "methodology_only".to_owned()
                    } else {
                        decision.result.clone()
                    },
                })
                .collect(),
            bundle_projection_results: self
                .bundle_bindings
                .iter()
                .map(|binding| CleanRoomProjectionBundleResult {
                    bundle_ref: binding.bundle_ref.clone(),
                    fixture_register_row_ref: binding.fixture_register_row_ref.clone(),
                    publication_result: binding.publication_result.clone(),
                })
                .collect(),
            explicit_non_claims: explicit_non_claims.into_iter().collect(),
        }
    }

    fn lane_result(&self, lane: &str) -> Option<&str> {
        self.lane_decisions
            .iter()
            .find(|decision| decision.lane == lane)
            .map(|decision| decision.result.as_str())
    }
}

/// Lane-level publication decision parsed from the rehearsal methodology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRehearsalLaneDecision {
    /// Publication lane such as `benchmark` or `public_proof`.
    pub lane: String,
    /// Decision result for the lane.
    pub result: String,
    /// Methodology reason for the decision.
    pub reason: String,
}

/// Checklist coverage row parsed from the rehearsal methodology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRehearsalChecklistCoverage {
    /// Checklist group id.
    pub checklist_group: String,
    /// Dry-run state declared for the checklist group.
    pub dry_run_state: String,
    /// Evidence summary declared for the checklist group.
    pub evidence: String,
}

/// Bundle and fixture binding parsed from the rehearsal methodology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRehearsalBundleBinding {
    /// Launch bundle ref covered by the rehearsal.
    pub bundle_ref: String,
    /// Fixture register row covered by the rehearsal.
    pub fixture_register_row_ref: String,
    /// Corpus refs covered by the rehearsal.
    pub corpus_refs: Vec<String>,
    /// Publication result for the bundle binding.
    pub publication_result: String,
}

/// Machine-readable subset of the external alpha known-limits packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitsAlphaPacket {
    /// Known-limits schema version.
    pub schema_version: u32,
    /// Stable known-limits packet id.
    pub packet_id: String,
    /// Known-limits packet revision.
    pub packet_revision: u32,
    /// Date at which the packet was captured.
    pub as_of: String,
    /// Source contracts cited by the known-limits packet.
    #[serde(default)]
    pub source_contract_refs: BTreeMap<String, String>,
    /// Known-limit notes carried by the packet.
    #[serde(default)]
    pub known_limits: Vec<KnownLimitNote>,
    /// Publication rules carried by the packet.
    #[serde(default)]
    pub publication_rules: Vec<KnownLimitPublicationRule>,
    /// Acceptance-state coverage carried by the packet.
    #[serde(default)]
    pub acceptance_state_coverage: Vec<KnownLimitAcceptanceState>,
}

impl KnownLimitsAlphaPacket {
    /// Returns the active methodology-only known limit when present.
    pub fn methodology_only_limit(&self) -> Option<&KnownLimitNote> {
        self.known_limits.iter().find(|limit| {
            limit.known_limit_id == PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID
        })
    }

    /// Validates that the packet keeps publication rehearsal claims narrowed.
    pub fn validate_methodology_only_limit(&self) -> Vec<PublicationDryRunViolation> {
        let mut violations = Vec::new();

        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "known_limits.schema_version",
                &self.packet_id,
                "known-limits schema_version must be 1",
            );
        }
        if self
            .source_contract_refs
            .get("publication_rehearsal")
            .map(String::as_str)
            != Some(CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_PATH)
        {
            push_violation(
                &mut violations,
                "known_limits.source_contract_refs.publication_rehearsal",
                &self.packet_id,
                "known-limits packet must cite the current publication rehearsal",
            );
        }
        if !self
            .publication_rules
            .iter()
            .any(|rule| rule.rule_id == "methodology_only_until_measured")
        {
            push_violation(
                &mut violations,
                "known_limits.publication_rules.methodology_only_until_measured",
                &self.packet_id,
                "known-limits packet must keep rehearsal reports methodology-only until measured",
            );
        }
        if !self
            .acceptance_state_coverage
            .iter()
            .any(|state| state.exercises_state == "methodology_only_publication_rehearsal")
        {
            push_violation(
                &mut violations,
                "known_limits.acceptance_state_coverage.methodology_only",
                &self.packet_id,
                "known-limits packet must cover the methodology-only publication rehearsal state",
            );
        }

        let Some(limit) = self.methodology_only_limit() else {
            push_violation(
                &mut violations,
                "known_limits.methodology_only_limit.missing",
                PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID,
                "methodology-only publication rehearsal known limit is missing",
            );
            return violations;
        };

        if !limit.is_active_methodology_only_publication_limit() {
            push_violation(
                &mut violations,
                "known_limits.methodology_only_limit.state",
                &limit.known_limit_id,
                "methodology-only known limit must remain active, major, mandatory, and competitor-parity narrowed",
            );
        }
        if !limit.excludes_public_benchmark_claims() {
            push_violation(
                &mut violations,
                "known_limits.methodology_only_limit.explicit_exclusions",
                &limit.known_limit_id,
                "methodology-only known limit must exclude public comparison and performance claims",
            );
        }
        let summary = limit.partner_summary.to_lowercase();
        if !(summary.contains("methodology-only") && summary.contains("does not publish")) {
            push_violation(
                &mut violations,
                "known_limits.methodology_only_limit.partner_summary",
                &limit.known_limit_id,
                "methodology-only known limit summary must not imply benchmark publication",
            );
        }
        for destination in [
            "docs_site",
            "release_packet",
            "evaluation_artifact",
            "public_proof_packet",
        ] {
            if !limit
                .mandatory_publication_destinations
                .iter()
                .any(|value| value == destination)
            {
                push_violation(
                    &mut violations,
                    "known_limits.methodology_only_limit.destinations",
                    &limit.known_limit_id,
                    "methodology-only known limit must publish to all required disclosure destinations",
                );
            }
        }
        match &limit.freshness {
            Some(freshness) if freshness.proof_class == "benchmark_publication_proof" => {}
            Some(_) => push_violation(
                &mut violations,
                "known_limits.methodology_only_limit.freshness",
                &limit.known_limit_id,
                "methodology-only known limit must use benchmark-publication proof freshness",
            ),
            None => push_violation(
                &mut violations,
                "known_limits.methodology_only_limit.freshness",
                &limit.known_limit_id,
                "methodology-only known limit must carry freshness metadata",
            ),
        }

        violations
    }
}

/// One known-limit note from the external alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitNote {
    /// Stable known-limit id.
    pub known_limit_id: String,
    /// Limitation class such as `competitor_parity_narrowed`.
    pub limitation_class: String,
    /// Severity class for the known limit.
    pub severity: String,
    /// Current note state such as `active`.
    pub note_state: String,
    /// Review rubric class such as `mandatory`.
    pub review_rubric_class: String,
    /// Partner-facing summary for the known limit.
    pub partner_summary: String,
    /// Explicit claim or artifact classes excluded by the known limit.
    #[serde(default)]
    pub explicit_exclusions: Vec<String>,
    /// Downgrade triggers attached to the known limit.
    #[serde(default)]
    pub downgrade_triggers: Vec<String>,
    /// Publication destinations where the known limit must appear.
    #[serde(default)]
    pub mandatory_publication_destinations: Vec<String>,
    /// Freshness metadata for the known limit.
    pub freshness: Option<KnownLimitFreshness>,
}

impl KnownLimitNote {
    /// Returns true when the limit is the active methodology-only publication rehearsal limit.
    pub fn is_active_methodology_only_publication_limit(&self) -> bool {
        self.known_limit_id == PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID
            && self.limitation_class == "competitor_parity_narrowed"
            && self.severity == "major"
            && self.note_state == "active"
            && self.review_rubric_class == "mandatory"
    }

    /// Returns true when the limit excludes public benchmark and comparison claims.
    pub fn excludes_public_benchmark_claims(&self) -> bool {
        [
            "public_head_to_head_comparison",
            "published_performance_claim",
            "certified_or_replacement_grade_public_wording",
        ]
        .iter()
        .all(|exclusion| {
            self.explicit_exclusions
                .iter()
                .any(|value| value == exclusion)
        })
    }
}

/// Freshness metadata for one known-limit note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitFreshness {
    /// UTC timestamp or date at which the note was captured.
    pub captured_at: String,
    /// Duration after which the note becomes stale.
    pub stale_after: String,
    /// Proof class backing the known limit.
    pub proof_class: String,
    /// Re-run trigger refs for the known limit.
    #[serde(default)]
    pub rerun_trigger_refs: Vec<String>,
}

/// Publication rule from the known-limits packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitPublicationRule {
    /// Stable publication rule id.
    pub rule_id: String,
    /// Human-readable rule summary.
    pub summary: String,
}

/// Acceptance-state coverage row from the known-limits packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitAcceptanceState {
    /// Stable acceptance case id.
    pub case_id: String,
    /// State exercised by the acceptance case.
    pub exercises_state: String,
    /// Fixture ref backing the acceptance case.
    pub fixture_ref: String,
    /// Expected validator result.
    pub expected_validator_result: String,
}

/// Projection record for clean-room rebuild readiness under alpha evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanRoomRebuildProjection {
    /// Projection schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable projection id derived from the rehearsal packet id.
    pub projection_id: String,
    /// Projection state, intentionally narrowed to methodology-only alpha evidence.
    pub projection_state: String,
    /// Whether this record is a projection rather than a rebuild result.
    pub projection_only: bool,
    /// Whether an actual clean-room rebuild was executed by this record.
    pub actual_clean_room_rebuild_executed: bool,
    /// Execution claim state for the clean-room rebuild path.
    pub rebuild_execution_state: String,
    /// Rehearsal packet id used to build the projection.
    pub rehearsal_packet_id: String,
    /// Rehearsal packet state used to build the projection.
    pub rehearsal_packet_state: String,
    /// Exact-build identity ref carried by the publication manifest.
    pub exact_build_identity_ref: String,
    /// Exact-build identity artifact referenced by the methodology packet.
    pub methodology_exact_build_identity_ref: String,
    /// Known-limit ref that constrains this projection.
    pub methodology_only_known_limit_ref: String,
    /// Note state for the methodology-only known limit.
    pub known_limit_note_state: String,
    /// Partner-facing known-limit summary.
    pub known_limit_summary: String,
    /// Source refs used to build the projection.
    pub methodology_source_refs: Vec<String>,
    /// Artifact family keys projected from the publication manifest.
    pub projected_artifact_family_keys: Vec<String>,
    /// Publication posture classes projected from the publication manifest.
    pub publication_posture_classes: Vec<String>,
    /// Artifact families that stay verifiable without vendor reachability.
    pub vendor_unreachable_family_keys: Vec<String>,
    /// Number of verification receipts projected from the publication manifest.
    pub receipt_count: usize,
    /// Number of broader-publication blockers projected from the publication manifest.
    pub blocking_blocker_count: usize,
    /// Lane results that constrain the projection.
    pub methodology_lane_results: Vec<CleanRoomProjectionLaneResult>,
    /// Bundle results that constrain the projection.
    pub bundle_projection_results: Vec<CleanRoomProjectionBundleResult>,
    /// Claims explicitly not made by this projection.
    pub explicit_non_claims: Vec<String>,
}

impl CleanRoomRebuildProjection {
    /// Validates that the projection does not claim actual clean-room execution.
    pub fn validate(&self) -> Vec<PublicationDryRunViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CLEAN_ROOM_REBUILD_PROJECTION_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "clean_room_projection.schema_version",
                &self.projection_id,
                "clean-room rebuild projection schema_version must be 1",
            );
        }
        if self.record_kind != CLEAN_ROOM_REBUILD_PROJECTION_RECORD_KIND {
            push_violation(
                &mut violations,
                "clean_room_projection.record_kind",
                &self.projection_id,
                "clean-room rebuild projection record_kind is not supported",
            );
        }
        if self.projection_state != "alpha_methodology_only_projection" {
            push_violation(
                &mut violations,
                "clean_room_projection.projection_state",
                &self.projection_id,
                "clean-room rebuild projection must remain alpha methodology-only",
            );
        }
        if !self.projection_only || self.actual_clean_room_rebuild_executed {
            push_violation(
                &mut violations,
                "clean_room_projection.execution_claim",
                &self.projection_id,
                "clean-room rebuild projection must not claim actual rebuild execution",
            );
        }
        if self.rebuild_execution_state != "not_executed_methodology_projection" {
            push_violation(
                &mut violations,
                "clean_room_projection.rebuild_execution_state",
                &self.projection_id,
                "clean-room rebuild execution state must remain projection-only",
            );
        }
        if self.rehearsal_packet_state != "methodology_only" {
            push_violation(
                &mut violations,
                "clean_room_projection.rehearsal_packet_state",
                &self.projection_id,
                "clean-room rebuild projection must consume a methodology-only rehearsal packet",
            );
        }
        if self.methodology_only_known_limit_ref
            != PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID
        {
            push_violation(
                &mut violations,
                "clean_room_projection.known_limit_ref",
                &self.projection_id,
                "clean-room rebuild projection must cite the methodology-only known limit",
            );
        }
        if self.known_limit_note_state != "active" {
            push_violation(
                &mut violations,
                "clean_room_projection.known_limit_note_state",
                &self.projection_id,
                "clean-room rebuild projection must cite an active known limit",
            );
        }
        for source_ref in [
            CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH,
            CURRENT_PUBLICATION_REHEARSAL_METHODOLOGY_PATH,
            CURRENT_ALPHA_KNOWN_LIMITS_PATH,
        ] {
            if !self
                .methodology_source_refs
                .iter()
                .any(|reference| reference == source_ref)
            {
                push_violation(
                    &mut violations,
                    "clean_room_projection.methodology_source_refs",
                    source_ref,
                    "clean-room rebuild projection is missing a required source ref",
                );
            }
        }

        let projected_families = self
            .projected_artifact_family_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for family in REQUIRED_FAMILIES {
            if !projected_families.contains(family) {
                push_violation(
                    &mut violations,
                    "clean_room_projection.projected_artifact_family_keys",
                    family,
                    "clean-room rebuild projection is missing a required artifact family",
                );
            }
        }

        let projected_postures = self
            .publication_posture_classes
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for posture in REQUIRED_POSTURES {
            if !projected_postures.contains(posture) {
                push_violation(
                    &mut violations,
                    "clean_room_projection.publication_posture_classes",
                    posture,
                    "clean-room rebuild projection is missing a required publication posture",
                );
            }
        }

        if self.receipt_count == 0 {
            push_violation(
                &mut violations,
                "clean_room_projection.receipt_count",
                &self.projection_id,
                "clean-room rebuild projection must carry publication receipt coverage",
            );
        }
        if self.blocking_blocker_count == 0 {
            push_violation(
                &mut violations,
                "clean_room_projection.blocking_blocker_count",
                &self.projection_id,
                "clean-room rebuild projection must carry broader-publication blockers",
            );
        }
        for lane in ["benchmark", "public_proof"] {
            let lane_result = self
                .methodology_lane_results
                .iter()
                .find(|result| result.lane == lane)
                .map(|result| result.result.as_str());
            if lane_result != Some("keep_methodology_only") {
                push_violation(
                    &mut violations,
                    "clean_room_projection.methodology_lane_results",
                    lane,
                    "clean-room rebuild projection cannot widen benchmark or public-proof lanes",
                );
            }
        }
        if self.bundle_projection_results.is_empty() {
            push_violation(
                &mut violations,
                "clean_room_projection.bundle_projection_results",
                &self.projection_id,
                "clean-room rebuild projection must carry bundle projection results",
            );
        }
        for bundle in &self.bundle_projection_results {
            if bundle.publication_result != "keep_methodology_only" {
                push_violation(
                    &mut violations,
                    "clean_room_projection.bundle_projection_results.publication_result",
                    &bundle.bundle_ref,
                    "clean-room rebuild projection bundle result must remain methodology-only",
                );
            }
        }
        for non_claim in [
            "actual_clean_room_rebuild_execution",
            "public_head_to_head_comparison",
            "published_performance_claim",
            "certified_or_replacement_grade_public_wording",
        ] {
            if !self
                .explicit_non_claims
                .iter()
                .any(|value| value == non_claim)
            {
                push_violation(
                    &mut violations,
                    "clean_room_projection.explicit_non_claims",
                    non_claim,
                    "clean-room rebuild projection is missing an explicit non-claim",
                );
            }
        }

        violations
    }

    /// Returns true when the projection stays within methodology-only known-limit bounds.
    pub fn respects_methodology_only_known_limit(&self) -> bool {
        self.projection_only
            && !self.actual_clean_room_rebuild_executed
            && self.rebuild_execution_state == "not_executed_methodology_projection"
            && self.methodology_only_known_limit_ref
                == PUBLICATION_REHEARSAL_METHODOLOGY_ONLY_KNOWN_LIMIT_ID
            && [
                "actual_clean_room_rebuild_execution",
                "public_head_to_head_comparison",
                "published_performance_claim",
                "certified_or_replacement_grade_public_wording",
            ]
            .iter()
            .all(|non_claim| {
                self.explicit_non_claims
                    .iter()
                    .any(|value| value == non_claim)
            })
    }
}

/// Lane result carried by the clean-room rebuild projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanRoomProjectionLaneResult {
    /// Publication lane such as `benchmark`.
    pub lane: String,
    /// Lane result copied from the methodology packet.
    pub result: String,
    /// Projection acceptance state derived from the lane result.
    pub projection_acceptance: String,
}

/// Bundle result carried by the clean-room rebuild projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanRoomProjectionBundleResult {
    /// Launch bundle ref covered by the projection.
    pub bundle_ref: String,
    /// Fixture register row covered by the projection.
    pub fixture_register_row_ref: String,
    /// Publication result copied from the methodology packet.
    pub publication_result: String,
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

fn markdown_section<'a>(
    markdown: &'a str,
    heading: &str,
) -> Result<&'a str, PublicationRehearsalParseError> {
    let marker = format!("## {heading}");
    let start = markdown.find(&marker).ok_or_else(|| {
        PublicationRehearsalParseError::new(heading, "required Markdown section is missing")
    })?;
    let after_heading = &markdown[start + marker.len()..];
    let after_heading = after_heading.strip_prefix('\n').unwrap_or(after_heading);
    let end = after_heading.find("\n## ").unwrap_or(after_heading.len());
    Ok(after_heading[..end].trim())
}

fn parse_markdown_table(
    section_name: &str,
    section: &str,
    expected_headers: &[&str],
) -> Result<Vec<BTreeMap<String, String>>, PublicationRehearsalParseError> {
    let mut headers: Option<Vec<String>> = None;
    let mut rows = Vec::new();

    for line in section.lines().map(str::trim) {
        if !(line.starts_with('|') && line.ends_with('|')) {
            continue;
        }

        let cells = split_markdown_table_row(line);
        if cells.iter().all(|cell| is_separator_cell(cell)) {
            continue;
        }

        if let Some(headers) = &headers {
            if cells.len() != headers.len() {
                return Err(PublicationRehearsalParseError::new(
                    section_name,
                    "Markdown table row width does not match header width",
                ));
            }
            rows.push(
                headers
                    .iter()
                    .cloned()
                    .zip(cells.into_iter())
                    .collect::<BTreeMap<_, _>>(),
            );
        } else {
            let expected_headers = expected_headers
                .iter()
                .map(|header| (*header).to_owned())
                .collect::<Vec<_>>();
            if cells != expected_headers {
                return Err(PublicationRehearsalParseError::new(
                    section_name,
                    "Markdown table headers do not match the expected methodology shape",
                ));
            }
            headers = Some(cells);
        }
    }

    if headers.is_none() {
        return Err(PublicationRehearsalParseError::new(
            section_name,
            "Markdown table is missing",
        ));
    }
    if rows.is_empty() {
        return Err(PublicationRehearsalParseError::new(
            section_name,
            "Markdown table has no data rows",
        ));
    }

    Ok(rows)
}

fn split_markdown_table_row(line: &str) -> Vec<String> {
    line.trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().replace('`', ""))
        .collect()
}

fn is_separator_cell(cell: &str) -> bool {
    let trimmed = cell.trim();
    trimmed.contains('-')
        && trimmed
            .chars()
            .all(|character| matches!(character, '-' | ':' | ' '))
}

fn required_table_value(
    row: &BTreeMap<String, String>,
    key: &str,
    section_name: &str,
) -> Result<String, PublicationRehearsalParseError> {
    row.get(key)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| {
            PublicationRehearsalParseError::new(section_name, "required table value is missing")
        })
}

fn required_header_value(
    header: &BTreeMap<String, String>,
    key: &str,
) -> Result<String, PublicationRehearsalParseError> {
    header
        .get(key)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| {
            PublicationRehearsalParseError::new("Packet Header", "required header value is missing")
        })
}

fn extract_backtick_values(text: &str) -> Vec<String> {
    text.split('`')
        .enumerate()
        .filter_map(|(index, value)| (index % 2 == 1).then(|| value.trim().to_owned()))
        .filter(|value| !value.is_empty())
        .collect()
}

fn first_code_block_line(section: &str) -> Option<String> {
    let mut in_code_block = false;
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block && !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }
    None
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
