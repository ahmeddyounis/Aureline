//! Request-profile certification records for API collections, GraphQL/contract
//! freshness, request-origin truth, and persisted-operation continuity.
//!
//! This module owns the typed records that certify the API request lane across
//! every claimed request profile (desktop-local, CLI/headless, remote, container,
//! managed-workspace, browser-companion, and mirror/offline). Certification is
//! evidence-bound: each case binds a profile, a certified dimension (collections,
//! contract freshness, request-origin truth, persisted-operation continuity,
//! history retention, auth-source labeling), and a drill corpus (schema-stale,
//! origin-changed rerun, persisted-operation drift, persisted-operation
//! deprecation, mirror/offline snapshot, export/redaction) to an outcome that
//! either certifies, narrows, or blocks. Schema staleness and persisted-operation
//! drift never silently fall back to raw execution, history retention never
//! widens past its safe default, managed and browser-companion profiles never
//! inherit desktop-local trust, and a profile that overclaims validation
//! confidence or origin stability narrows automatically. The boundary schema is
//! [`/schemas/data/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.schema.json`](../../../schemas/data/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.json`](../../../artifacts/data/m5/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.json).
//!
//! Raw endpoint URLs, raw secrets, raw credential bodies, and raw request or
//! response payloads do not belong in these records. They carry stable IDs,
//! closed posture vocabularies, and reviewable summaries that UI, CLI, release,
//! service-health, support, and export surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for request-profile certification packets.
pub const REQUEST_PROFILE_CERT_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RequestProfileCertQualificationPacket`].
pub const REQUEST_PROFILE_CERT_QUALIFICATION_RECORD_KIND: &str =
    "certify_api_collections_graphql_freshness_request_origin_truth_and_persisted_operation_continuity_across_request_profiles";

/// Repo-relative path to the checked-in request-profile certification packet.
pub const REQUEST_PROFILE_CERT_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.json";

/// Embedded checked-in packet JSON.
pub const REQUEST_PROFILE_CERT_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.json"
));

/// Qualification label shown on promoted certification surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestProfileCertQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl RequestProfileCertQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Request profile being certified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestProfileClass {
    /// Local desktop send against a loopback or local target.
    DesktopLocal,
    /// CLI or headless send producing inspectable output.
    Cli,
    /// Remote send reached over the network (including SSH-forwarded).
    Remote,
    /// Container or compose service-name target.
    Container,
    /// Managed-workspace or cloud-hosted target.
    Managed,
    /// Browser-companion runtime send.
    BrowserCompanion,
    /// Collection reopened offline or from a mirror snapshot.
    MirrorOffline,
}

impl RequestProfileClass {
    /// Returns true when the profile must never inherit desktop-local trust or
    /// naming assumptions.
    pub const fn must_isolate_local_trust(self) -> bool {
        matches!(self, Self::Managed | Self::BrowserCompanion)
    }

    /// Returns true when the profile is not the live-online desktop-local profile.
    ///
    /// Used to prove certification does not rest on desktop-only fixtures.
    pub const fn is_non_desktop(self) -> bool {
        !matches!(self, Self::DesktopLocal)
    }
}

/// Certified dimension of the request lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// Text-first, versionable API collections and request lists.
    ApiCollections,
    /// GraphQL/contract schema freshness and source labeling.
    ContractFreshness,
    /// Request-origin truth and origin-drift review.
    RequestOriginTruth,
    /// Persisted-operation binding continuity (drift, deprecation, removal).
    PersistedOperationContinuity,
    /// Request-history retention posture and export-safe compare.
    HistoryRetention,
    /// Auth-scheme and secret-source labeling without raw secrets.
    AuthSourceLabeling,
}

/// Drill corpus exercised by a certification case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationCorpusClass {
    /// Stale or cached-beyond-window contract schema.
    SchemaStale,
    /// Rerun whose resolved origin changed since the last send.
    OriginChangedRerun,
    /// Persisted-operation id/hash no longer matches the operation text.
    PersistedOperationDrift,
    /// Persisted-operation contract version is deprecated or removed.
    PersistedOperationDeprecation,
    /// Collection reopened from a mirror or offline snapshot.
    MirrorOfflineSnapshot,
    /// Export or redaction posture check on collections and history.
    ExportRedaction,
}

impl CertificationCorpusClass {
    /// Returns true when a passing case for this corpus must block any unsafe
    /// raw-execution fallback because contract or operation risk changed.
    pub const fn must_block_unsafe_fallback(self) -> bool {
        matches!(
            self,
            Self::SchemaStale | Self::PersistedOperationDrift | Self::PersistedOperationDeprecation
        )
    }
}

/// Outcome of a certification case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationOutcome {
    /// Case is certified for its declared profile and dimension.
    Certified,
    /// Case narrows the claim instead of asserting full confidence.
    NarrowedClaim,
    /// Case blocks the send pending review (drift, deprecation, origin change).
    Blocked,
}

impl CertificationOutcome {
    /// Returns true when the case is fully certified.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// Trigger that narrows a claim or blocks a send through downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTrigger {
    /// Required proof is missing or stale.
    MissingProof,
    /// Contract schema is stale or unavailable.
    SchemaStale,
    /// Resolved request origin changed since the last send.
    OriginChanged,
    /// Persisted-operation id/hash drifted from the operation text.
    PersistedOperationDrift,
    /// Persisted-operation contract version is deprecated or removed.
    PersistedOperationDeprecation,
    /// Collection reopened offline and could not refresh its contract.
    MirrorOfflineUnavailable,
    /// Profile overclaims validation confidence beyond its proof.
    OverclaimedValidationConfidence,
    /// Profile overclaims origin stability.
    OverclaimedOriginStability,
}

/// Certification surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestProfileCertSurfaceKind {
    /// Profile certification scorecard listing per-profile claims.
    ProfileCertificationScorecard,
    /// M5 compatibility report consuming the certification result.
    CompatibilityReport,
    /// Downgrade automation that narrows overclaimed rows.
    DowngradeAutomation,
    /// Release center promotion surface.
    ReleaseCenter,
    /// Service-health and diagnostics surface.
    ServiceHealth,
    /// Support and export bundle surface.
    SupportExport,
    /// Help/About surface describing the certification.
    HelpAbout,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileCertProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileCertSurfaceGuardSet {
    /// Certified request profiles are visible.
    pub profiles_visible: bool,
    /// API-collection certification is visible.
    pub collections_visible: bool,
    /// Contract/GraphQL freshness certification is visible.
    pub contract_freshness_visible: bool,
    /// Request-origin truth certification is visible.
    pub request_origin_visible: bool,
    /// Persisted-operation continuity certification is visible.
    pub persisted_operation_visible: bool,
    /// History retention posture is visible.
    pub retention_posture_visible: bool,
    /// Drill-corpus coverage is visible.
    pub corpus_coverage_visible: bool,
    /// Managed and companion origins isolate desktop-local trust.
    pub origin_trust_isolated: bool,
    /// Drift, deprecation, and stale-schema cases block silent raw fallback.
    pub no_silent_raw_fallback: bool,
    /// Downgrade rules are visible.
    pub downgrade_rules_visible: bool,
    /// Upstream packet references are visible.
    pub upstream_refs_visible: bool,
}

impl RequestProfileCertSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.profiles_visible
            && self.collections_visible
            && self.contract_freshness_visible
            && self.request_origin_visible
            && self.persisted_operation_visible
            && self.retention_posture_visible
            && self.corpus_coverage_visible
            && self.origin_trust_isolated
            && self.no_silent_raw_fallback
            && self.downgrade_rules_visible
            && self.upstream_refs_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileCertSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: RequestProfileCertSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: RequestProfileCertQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: RequestProfileCertQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<RequestProfileCertProof>,
    /// Visible guard set.
    pub guards: RequestProfileCertSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One certified request profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileRow {
    /// Stable profile identifier.
    pub profile_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Profile class.
    pub profile_class: RequestProfileClass,
    /// Claimed label from upstream release planning.
    pub claim_label: RequestProfileCertQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: RequestProfileCertQualificationLabel,
    /// Dimensions certified for this profile.
    pub certified_dimensions: Vec<CertificationDimension>,
    /// True when the profile's certification rests only on live-online fixtures.
    ///
    /// This must be false: certification cannot rest on desktop-only live truth.
    pub live_online_only_fixtures: bool,
    /// True when the profile isolates desktop-local trust and naming.
    pub trust_isolated_from_desktop_local: bool,
    /// True when missing or narrowed proof narrows this profile automatically.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One certification case binding a profile, dimension, and drill corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileCertificationCaseRow {
    /// Stable case identifier.
    pub case_id: String,
    /// Profile this case certifies (references [`RequestProfileRow::profile_id`]).
    pub profile_ref: String,
    /// Certified dimension.
    pub dimension: CertificationDimension,
    /// Drill corpus exercised.
    pub corpus_class: CertificationCorpusClass,
    /// Certification outcome.
    pub outcome: CertificationOutcome,
    /// Claimed label from upstream release planning.
    pub claim_label: RequestProfileCertQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: RequestProfileCertQualificationLabel,
    /// True when the case blocks any unsafe raw-execution fallback.
    pub blocks_unsafe_fallback: bool,
    /// True when the case preserves the safe (metadata-only) retention default.
    pub preserves_safe_retention_default: bool,
    /// True when the case isolates desktop-local trust and naming.
    pub trust_isolated_from_desktop_local: bool,
    /// True when missing or narrowed proof narrows this case automatically.
    pub downgrade_if_missing: bool,
    /// Repo-relative evidence reference (fixture) for the case.
    pub evidence_ref: String,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One automatic downgrade rule threaded into the compatibility report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeRuleRow {
    /// Stable rule identifier.
    pub rule_id: String,
    /// Trigger that fires the downgrade.
    pub trigger: DowngradeTrigger,
    /// Reference to the profile, case, or surface narrowed by this rule.
    pub target_ref: String,
    /// Label the target narrows to when the trigger fires.
    pub narrows_to: RequestProfileCertQualificationLabel,
    /// True when the downgrade applies without manual intervention.
    pub automatic: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Reference to an upstream qualification packet integrated into this certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileCertUpstreamRefRow {
    /// Stable reference id.
    pub ref_id: String,
    /// Upstream record kind.
    pub upstream_record_kind: String,
    /// Repo-relative path to the upstream packet.
    pub upstream_packet_path: String,
    /// Repo-relative path to the upstream schema.
    pub upstream_schema_path: String,
    /// Whether integration has been verified.
    pub integration_verified: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Summary counts for a request-profile certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileCertQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of certified request profiles.
    pub profile_count: usize,
    /// Number of profiles displayed as stable.
    pub stable_profile_count: usize,
    /// Number of certification cases.
    pub case_count: usize,
    /// Number of certification cases that are fully certified.
    pub certified_case_count: usize,
    /// Number of downgrade rule rows.
    pub downgrade_rule_count: usize,
    /// Number of upstream packet reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical request-profile certification qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestProfileCertQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<RequestProfileCertSurfaceQualificationRow>,
    /// Certified request profiles.
    pub profiles: Vec<RequestProfileRow>,
    /// Certification cases.
    pub cases: Vec<ProfileCertificationCaseRow>,
    /// Automatic downgrade rules.
    pub downgrade_rules: Vec<DowngradeRuleRow>,
    /// Upstream packet reference rows.
    pub upstream_refs: Vec<RequestProfileCertUpstreamRefRow>,
    /// Summary counts.
    pub summary: RequestProfileCertQualificationSummary,
}

impl RequestProfileCertQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> RequestProfileCertQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        let stable_profile_count = self
            .profiles
            .iter()
            .filter(|profile| profile.displayed_label.is_stable())
            .count();
        let certified_case_count = self
            .cases
            .iter()
            .filter(|case| case.outcome.is_certified())
            .count();
        let integration_pass_count = self
            .upstream_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        RequestProfileCertQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            profile_count: self.profiles.len(),
            stable_profile_count,
            case_count: self.cases.len(),
            certified_case_count,
            downgrade_rule_count: self.downgrade_rules.len(),
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of profiles whose displayed label narrowed below a stable
    /// claim.
    pub fn narrowed_profile_ids(&self) -> Vec<String> {
        self.profiles
            .iter()
            .filter(|profile| {
                profile.claim_label.is_stable() && !profile.displayed_label.is_stable()
            })
            .map(|profile| profile.profile_id.clone())
            .collect()
    }

    /// Returns the ids of cases blocked pending review.
    pub fn blocked_case_ids(&self) -> Vec<String> {
        self.cases
            .iter()
            .filter(|case| case.outcome == CertificationOutcome::Blocked)
            .map(|case| case.case_id.clone())
            .collect()
    }

    /// Returns the ids of cases that block an unsafe raw-execution fallback.
    pub fn unsafe_fallback_blocking_case_ids(&self) -> Vec<String> {
        self.cases
            .iter()
            .filter(|case| case.blocks_unsafe_fallback)
            .map(|case| case.case_id.clone())
            .collect()
    }

    /// Returns the ids of cases that isolate desktop-local trust.
    pub fn trust_isolated_case_ids(&self) -> Vec<String> {
        self.cases
            .iter()
            .filter(|case| case.trust_isolated_from_desktop_local)
            .map(|case| case.case_id.clone())
            .collect()
    }

    /// Returns the ids of cases exercising a mirror/offline snapshot corpus.
    pub fn offline_corpus_case_ids(&self) -> Vec<String> {
        self.cases
            .iter()
            .filter(|case| case.corpus_class == CertificationCorpusClass::MirrorOfflineSnapshot)
            .map(|case| case.case_id.clone())
            .collect()
    }

    /// Returns the set of corpus classes covered by at least one case.
    pub fn covered_corpus_classes(&self) -> BTreeSet<CertificationCorpusClass> {
        self.cases.iter().map(|case| case.corpus_class).collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<RequestProfileCertQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != REQUEST_PROFILE_CERT_QUALIFICATION_SCHEMA_VERSION {
            violations.push(RequestProfileCertQualificationViolation::SchemaVersion {
                expected: REQUEST_PROFILE_CERT_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REQUEST_PROFILE_CERT_QUALIFICATION_RECORD_KIND {
            violations.push(RequestProfileCertQualificationViolation::RecordKind {
                expected: REQUEST_PROFILE_CERT_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            RequestProfileCertQualificationViolationKind::Surface,
        );
        let profile_ids: BTreeSet<String> = collect_ids(
            self.profiles.iter().map(|row| row.profile_id.as_str()),
            &mut violations,
            RequestProfileCertQualificationViolationKind::Profile,
        );
        collect_ids(
            self.cases.iter().map(|row| row.case_id.as_str()),
            &mut violations,
            RequestProfileCertQualificationViolationKind::Case,
        );
        collect_ids(
            self.downgrade_rules.iter().map(|row| row.rule_id.as_str()),
            &mut violations,
            RequestProfileCertQualificationViolationKind::DowngradeRule,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            RequestProfileCertQualificationViolationKind::UpstreamRef,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        RequestProfileCertQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        RequestProfileCertQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    RequestProfileCertQualificationViolation::NarrowedLacksDowngradeRule {
                        kind: RequestProfileCertQualificationViolationKind::Surface,
                        id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.rationale.is_empty() {
                violations.push(RequestProfileCertQualificationViolation::IncompleteRow {
                    kind: RequestProfileCertQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
        }

        for profile in &self.profiles {
            if profile.certified_dimensions.is_empty() || profile.rationale.is_empty() {
                violations.push(RequestProfileCertQualificationViolation::IncompleteRow {
                    kind: RequestProfileCertQualificationViolationKind::Profile,
                    id: profile.profile_id.clone(),
                });
            }
            if profile.live_online_only_fixtures {
                violations.push(
                    RequestProfileCertQualificationViolation::LiveOnlyFixtureCertification {
                        profile_id: profile.profile_id.clone(),
                    },
                );
            }
            if profile.profile_class.must_isolate_local_trust()
                && !profile.trust_isolated_from_desktop_local
            {
                violations.push(RequestProfileCertQualificationViolation::TrustNotIsolated {
                    kind: RequestProfileCertQualificationViolationKind::Profile,
                    id: profile.profile_id.clone(),
                });
            }
            if profile.claim_label.is_stable()
                && !profile.displayed_label.is_stable()
                && !profile.downgrade_if_missing
            {
                violations.push(
                    RequestProfileCertQualificationViolation::NarrowedLacksDowngradeRule {
                        kind: RequestProfileCertQualificationViolationKind::Profile,
                        id: profile.profile_id.clone(),
                    },
                );
            }
        }

        // Track, per profile, whether any case is not fully certified so an
        // overclaiming stable profile narrows automatically.
        let mut profile_has_open_case: BTreeSet<String> = BTreeSet::new();

        for case in &self.cases {
            if !profile_ids.contains(&case.profile_ref) {
                violations.push(
                    RequestProfileCertQualificationViolation::CaseUnknownProfile {
                        case_id: case.case_id.clone(),
                        profile_ref: case.profile_ref.clone(),
                    },
                );
            }
            if case.rationale.is_empty() || case.evidence_ref.is_empty() {
                violations.push(RequestProfileCertQualificationViolation::IncompleteRow {
                    kind: RequestProfileCertQualificationViolationKind::Case,
                    id: case.case_id.clone(),
                });
            }
            if case.corpus_class.must_block_unsafe_fallback() && !case.blocks_unsafe_fallback {
                violations.push(
                    RequestProfileCertQualificationViolation::UnsafeFallbackNotBlocked {
                        case_id: case.case_id.clone(),
                    },
                );
            }
            if case.dimension == CertificationDimension::HistoryRetention
                && !case.preserves_safe_retention_default
            {
                violations.push(
                    RequestProfileCertQualificationViolation::UnsafeRetentionDefault {
                        case_id: case.case_id.clone(),
                    },
                );
            }
            if case.claim_label.is_stable()
                && !case.displayed_label.is_stable()
                && !case.downgrade_if_missing
            {
                violations.push(
                    RequestProfileCertQualificationViolation::NarrowedLacksDowngradeRule {
                        kind: RequestProfileCertQualificationViolationKind::Case,
                        id: case.case_id.clone(),
                    },
                );
            }
            if !case.outcome.is_certified() {
                profile_has_open_case.insert(case.profile_ref.clone());
            }
        }

        // A stable profile must not carry an uncertified (narrowed or blocked)
        // case: overclaiming validation confidence narrows automatically.
        for profile in &self.profiles {
            if profile.displayed_label.is_stable()
                && profile_has_open_case.contains(&profile.profile_id)
            {
                violations.push(
                    RequestProfileCertQualificationViolation::StableProfileOverclaims {
                        profile_id: profile.profile_id.clone(),
                    },
                );
            }
        }

        // Every required drill corpus must be covered.
        let covered = self.covered_corpus_classes();
        for required in [
            CertificationCorpusClass::SchemaStale,
            CertificationCorpusClass::OriginChangedRerun,
            CertificationCorpusClass::PersistedOperationDrift,
            CertificationCorpusClass::PersistedOperationDeprecation,
            CertificationCorpusClass::MirrorOfflineSnapshot,
            CertificationCorpusClass::ExportRedaction,
        ] {
            if !covered.contains(&required) {
                violations.push(
                    RequestProfileCertQualificationViolation::MissingCorpusClass {
                        corpus_class: required,
                    },
                );
            }
        }

        // Certification cannot rest on desktop-only live fixtures: at least one
        // non-desktop profile must be certified.
        if !self
            .profiles
            .iter()
            .any(|profile| profile.profile_class.is_non_desktop())
        {
            violations.push(RequestProfileCertQualificationViolation::DesktopOnlyCertification);
        }

        for rule in &self.downgrade_rules {
            if rule.rationale.is_empty() || rule.target_ref.is_empty() {
                violations.push(RequestProfileCertQualificationViolation::IncompleteRow {
                    kind: RequestProfileCertQualificationViolationKind::DowngradeRule,
                    id: rule.rule_id.clone(),
                });
            }
            if !rule.automatic {
                violations.push(
                    RequestProfileCertQualificationViolation::ManualDowngradeRule {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
        }

        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(RequestProfileCertQualificationViolation::IncompleteRow {
                    kind: RequestProfileCertQualificationViolationKind::UpstreamRef,
                    id: row.ref_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(RequestProfileCertQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in request-profile certification qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_request_profile_certification_qualification(
) -> Result<RequestProfileCertQualificationPacket, serde_json::Error> {
    serde_json::from_str(REQUEST_PROFILE_CERT_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids and incomplete rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestProfileCertQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Profile rows.
    Profile,
    /// Certification case rows.
    Case,
    /// Downgrade rule rows.
    DowngradeRule,
    /// Upstream packet reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<RequestProfileCertQualificationViolation>,
    kind: RequestProfileCertQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(RequestProfileCertQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for request-profile certification qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestProfileCertQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: RequestProfileCertQualificationViolationKind,
        id: String,
    },
    /// A row is missing required reviewable content.
    IncompleteRow {
        kind: RequestProfileCertQualificationViolationKind,
        id: String,
    },
    /// Stable surface has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable surface is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// A narrowed stable claim lacks an explicit downgrade rule.
    NarrowedLacksDowngradeRule {
        kind: RequestProfileCertQualificationViolationKind,
        id: String,
    },
    /// A profile certifies only from live-online fixtures.
    LiveOnlyFixtureCertification { profile_id: String },
    /// A managed or companion row fails to isolate desktop-local trust.
    TrustNotIsolated {
        kind: RequestProfileCertQualificationViolationKind,
        id: String,
    },
    /// A case references an unknown profile.
    CaseUnknownProfile {
        case_id: String,
        profile_ref: String,
    },
    /// A drift/deprecation/stale case fails to block unsafe raw fallback.
    UnsafeFallbackNotBlocked { case_id: String },
    /// A history-retention case fails to preserve the safe retention default.
    UnsafeRetentionDefault { case_id: String },
    /// A stable profile carries an uncertified case instead of narrowing.
    StableProfileOverclaims { profile_id: String },
    /// A required drill corpus class is not covered.
    MissingCorpusClass {
        corpus_class: CertificationCorpusClass,
    },
    /// Certification rests only on desktop profiles.
    DesktopOnlyCertification,
    /// A downgrade rule is not automatic.
    ManualDowngradeRule { rule_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for RequestProfileCertQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::IncompleteRow { kind, id } => {
                write!(
                    f,
                    "{kind:?} {id} does not project reviewable truth everywhere"
                )
            }
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedLacksDowngradeRule { kind, id } => {
                write!(f, "{kind:?} {id} is narrowed without a downgrade rule")
            }
            Self::LiveOnlyFixtureCertification { profile_id } => {
                write!(f, "{profile_id} certifies only from live-online fixtures")
            }
            Self::TrustNotIsolated { kind, id } => {
                write!(f, "{kind:?} {id} inherits desktop-local trust")
            }
            Self::CaseUnknownProfile {
                case_id,
                profile_ref,
            } => write!(f, "{case_id} references unknown profile {profile_ref}"),
            Self::UnsafeFallbackNotBlocked { case_id } => {
                write!(f, "{case_id} does not block unsafe raw fallback")
            }
            Self::UnsafeRetentionDefault { case_id } => {
                write!(f, "{case_id} does not preserve the safe retention default")
            }
            Self::StableProfileOverclaims { profile_id } => {
                write!(
                    f,
                    "{profile_id} is stable while carrying an uncertified case"
                )
            }
            Self::MissingCorpusClass { corpus_class } => {
                write!(f, "drill corpus class {corpus_class:?} is not covered")
            }
            Self::DesktopOnlyCertification => {
                write!(f, "certification rests only on desktop profiles")
            }
            Self::ManualDowngradeRule { rule_id } => {
                write!(f, "{rule_id} downgrade rule is not automatic")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for RequestProfileCertQualificationViolation {}

#[cfg(test)]
mod tests;
