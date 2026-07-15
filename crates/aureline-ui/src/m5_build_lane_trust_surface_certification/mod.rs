//! M05-1211 surface certification over the frozen M5 contributor-PR / protected-merge / release /
//! emergency-hotfix build-lane-trust matrix.
//!
//! Where the freeze matrix ([`crate::m5_build_lane_trust_matrix`]) defines the four governed build lanes, the
//! M05-1205..1209 implement lanes resolve each build-lane-descriptor, reproducibility-proof, verified-input,
//! sidecar-completeness, clean-room-rebuild, artifact-diff, remote-cache-integrity, cache-bypass,
//! exact-build-symbolication, and mirror/offline-parity registry, and the M05-1210 shared-consumer lane
//! ([`crate::m5_build_lane_trust_shared_consumers_one_registry_across_surfaces`]) aligns their grammar and
//! proves keyboard / screen-reader / high-zoom / high-contrast / localization / CLI-export parity and
//! per-lane auto-narrowing across the build-farm, cache-service, release-center, shiproom, provenance-service,
//! diagnostics, docs / help, CLI / export, and support-export consumers, this closing capstone *certifies*
//! that the shared build-lane-trust truth holds on every claimed M5 RC / stable / LTS / mirror-offline
//! publication-bearing profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a release engineer, reviewer, operator, or support engineer reads a
//! build-lane, cache-posture, publication-authority, clean-room-rebuild, reproducibility-proof, or
//! exact-build-supportability surface through (a live, first-party trusted exact-build supportable lane; a
//! reviewable reproducibility structure; a disclosed cache-discipline profile; an unverified clean-room-parity
//! profile; and an unverified exact-build-supportability profile), not on the build lane or implement lane.
//! Each [`BuildLaneTrustProfileCertificationRow`] certifies one profile across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! build-lane-trust-component-truth behavior — and either passes (green), auto-narrows its publication claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh
//! trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `TrustedExactBuildSupportableLane` / `ReviewableReproducibilitySurface` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party fully reproducible
//! release lane may certify a `TrustedExactBuildSupportableLane` claim — a reviewable, disclosed-cache-discipline,
//! unverified-clean-room-parity, or unverified-exact-build-supportability profile that keeps a trusted claim is
//! over-reaching and blocks. The always-on CLI/export axis must always stay certified so support and automation
//! can reconstruct the canonical build lane, cache posture, publication authority, exact build identity,
//! clean-room rebuild diff, reproducibility proof, sidecar convergence, support packet, and registry reference
//! from the same build-lane-trust truth the operator saw.
//!
//! The B144 hard invariants are enforced per row: no profile may let a PR cache publish release artifacts, treat
//! a remote-cache hit as reproducibility proof, let docs / schema / SBOM / symbol sidecars drift from the binary
//! build identity, overclaim clean-room parity when only partial artifact classes were rebuilt, or hide
//! non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green publication rows. A profile that
//! breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical build-lane-trust proof bundle
//! ([`BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF`]) — the frozen build-lane-trust matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer tokens,
//! endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-build-lane-trust-surface-certification.schema.json`](../../../../schemas/release/m5-build-lane-trust-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_build_lane_trust_surface_certification.md`](../../../../docs/release/m5_build_lane_trust_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_build_lane_trust_matrix as matrix;
use crate::m5_build_lane_trust_shared_consumers_one_registry_across_surfaces as shared_consumers;
use matrix::{M5BuildLaneDowngradeTrigger, M5BuildLaneFamily};

/// Schema version stamped on the M05-1211 certification packet.
pub const BUILD_LANE_TRUST_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BuildLaneTrustProfileCertificationPacket`].
pub const BUILD_LANE_TRUST_CERT_RECORD_KIND: &str =
    "m5_build_lane_trust_surface_certification_packet";

/// Stable record-kind tag carried by each [`BuildLaneTrustProfileCertificationRow`].
pub const BUILD_LANE_TRUST_CERT_ROW_RECORD_KIND: &str =
    "m5_build_lane_trust_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const BUILD_LANE_TRUST_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-build-lane-trust-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const BUILD_LANE_TRUST_CERT_DOC_REF: &str =
    "docs/release/m5_build_lane_trust_surface_certification.md";

/// Repo-relative path of the frozen build-lane-trust matrix schema the certified profiles render.
pub const BUILD_LANE_TRUST_CERT_MATRIX_REF: &str = matrix::M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF;

/// The one canonical build-lane-trust proof bundle every certified profile cites as its first-resolved
/// build-lane-trust truth. All five profiles point back to it rather than cloning per-profile evidence.
pub const BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_BUILD_LANE_TRUST_ARTIFACT_REF;

/// The M05-1210 shared-consumer support export the certification builds on. Recorded as a supporting evidence
/// ref on every row.
pub const BUILD_LANE_TRUST_CERT_CONSUMERS_BUNDLE_REF: &str =
    shared_consumers::M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const BUILD_LANE_TRUST_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-lane-trust-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BUILD_LANE_TRUST_CERT_CSV_REF: &str =
    "artifacts/release/m5-build-lane-trust-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BUILD_LANE_TRUST_CERT_REPORT_REF: &str =
    "artifacts/release/m5-build-lane-trust-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const BUILD_LANE_TRUST_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-build-lane-trust-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const BUILD_LANE_TRUST_CERT_PACKET_ID: &str =
    "m5-build-lane-trust-surface-certification:stable:0001";

/// The five claimed M5 publication-bearing operating profiles this capstone certifies. Keyed on the profile
/// a release engineer, reviewer, operator, or support engineer reads a build-lane, cache-posture,
/// publication-authority, clean-room-rebuild, reproducibility-proof, or exact-build-supportability surface
/// through, not on the reusable build lane it renders. Only a live, first-party fully reproducible release
/// lane profile may certify a trusted exact-build supportable lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneTrustCertifiedProfile {
    /// A live, first-party, fully-current release lane — a registry-bound, verified-or-re-materialized-input,
    /// clean-room-rebuilt, sidecar-converged, exact-build-identity release lane rendering the trusted
    /// exact-build supportable claim exactly right now.
    LiveExactBuildSupportableLane,
    /// A reviewable reproducibility structure: a self-sufficient, inspectable build-lane-trust projection (a
    /// build-lane descriptor / reproducibility proof / clean-room rebuild diff an operator can review), never
    /// itself an authoritative, live-publishing release lane.
    ReviewableReproducibilityStructure,
    /// A contributor / PR lane whose shared-remote-cache origin trust can only be partially disclosed; the
    /// claim narrows to a cache-discipline-disclosed projection that discloses the untrusted cache origin
    /// alongside its withheld publication authority, never a shared cache read shown as a verified, publishable
    /// posture or a PR cache publishing release artifacts.
    DisclosedCacheDisciplineProfile,
    /// A release lane whose clean-room rebuild covered only a partial set of artifact classes; the claim
    /// narrows to a clean-room-parity-unverified projection that keeps the last-known partial-rebuild posture
    /// explicit, never a partial rebuild shown as full clean-room parity when only some artifact classes were
    /// rebuilt.
    UnverifiedCleanRoomParityProfile,
    /// An emergency-hotfix lane whose docs / schema / SBOM / symbol sidecar has drifted from the binary build
    /// identity or aged out; the claim narrows to an exact-build-supportability-unverified projection that
    /// keeps the last-known sidecar-drift posture explicit, never a support packet shown as converged on one
    /// exact build identity or a drifted sidecar hidden behind a green publication row.
    UnverifiedExactBuildSupportabilityProfile,
}

impl M5BuildLaneTrustCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5BuildLaneTrustCertifiedProfile; 5] = [
        M5BuildLaneTrustCertifiedProfile::LiveExactBuildSupportableLane,
        M5BuildLaneTrustCertifiedProfile::ReviewableReproducibilityStructure,
        M5BuildLaneTrustCertifiedProfile::DisclosedCacheDisciplineProfile,
        M5BuildLaneTrustCertifiedProfile::UnverifiedCleanRoomParityProfile,
        M5BuildLaneTrustCertifiedProfile::UnverifiedExactBuildSupportabilityProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveExactBuildSupportableLane => "live_exact_build_supportable_lane",
            Self::ReviewableReproducibilityStructure => "reviewable_reproducibility_structure",
            Self::DisclosedCacheDisciplineProfile => "disclosed_cache_discipline_profile",
            Self::UnverifiedCleanRoomParityProfile => "unverified_clean_room_parity_profile",
            Self::UnverifiedExactBuildSupportabilityProfile => {
                "unverified_exact_build_supportability_profile"
            }
        }
    }

    /// True only for the live, first-party fully reproducible release lane profile. A trusted exact-build
    /// supportable lane may be certified on this profile alone; every other profile is at most a reviewable
    /// reproducibility structure or a narrowed projection.
    pub const fn is_live_exact_build_supportable_lane(self) -> bool {
        matches!(self, Self::LiveExactBuildSupportableLane)
    }
}

/// The claim ladder a certified build-lane-trust profile asserts and is certified down to. Minted locally
/// for this capstone (B144 has no separate accessibility lane): the strongest claim is a fully trusted
/// exact-build supportable lane; each weaker tier is a disclosed projection that keeps the last-known
/// cache-discipline, clean-room-parity, or exact-build-supportability posture explicit rather than overstating
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneTrustClaim {
    /// Trusted exact-build supportable lane: a fully current, registry-bound, verified-or-re-materialized-input,
    /// clean-room-rebuilt, sidecar-converged, exact-build-identity release lane — the strongest claim, a
    /// build-lane-trust surface Aureline can present as exactly reproducible and supportable right now.
    TrustedExactBuildSupportableLane,
    /// Reviewable reproducibility surface: a self-sufficient, inspectable read-only build-lane-trust projection
    /// (a static build-lane descriptor / reproducibility proof / clean-room rebuild diff an operator can
    /// inspect) that is not itself an authoritative, live-publishing lane.
    ReviewableReproducibilitySurface,
    /// Cache-discipline-disclosed projection: a contributor / PR lane's shared-remote-cache origin trust can
    /// only be partially disclosed; the lane stays a cache-discipline-disclosed projection that discloses the
    /// untrusted cache origin alongside its withheld publication authority, never a shared cache read shown as
    /// a verified, publishable posture or a PR cache publishing release artifacts.
    CacheDisciplineDisclosedProjection,
    /// Clean-room-parity-unverified projection: a release lane's clean-room rebuild covered only a partial set
    /// of artifact classes; the lane stays a clean-room-parity-unverified projection that keeps the last-known
    /// partial-rebuild posture explicit, never a partial rebuild shown as full clean-room parity.
    CleanRoomParityUnverifiedProjection,
    /// Exact-build-supportability-unverified projection: an emergency-hotfix lane's docs / schema / SBOM /
    /// symbol sidecar has drifted from the binary build identity or aged out; the lane stays an
    /// exact-build-supportability-unverified projection that keeps the last-known sidecar-drift posture
    /// explicit, never a support packet shown as converged on one exact build identity or a drifted sidecar
    /// hidden behind a green publication row.
    ExactBuildSupportabilityUnverifiedProjection,
}

impl M5BuildLaneTrustClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::TrustedExactBuildSupportableLane,
        Self::ReviewableReproducibilitySurface,
        Self::CacheDisciplineDisclosedProjection,
        Self::CleanRoomParityUnverifiedProjection,
        Self::ExactBuildSupportabilityUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedExactBuildSupportableLane => 4,
            Self::ReviewableReproducibilitySurface => 3,
            Self::CacheDisciplineDisclosedProjection => 2,
            Self::CleanRoomParityUnverifiedProjection => 1,
            Self::ExactBuildSupportabilityUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, reproducible exact-build supportable lane.
    pub const fn asserts_trusted_lane(self) -> bool {
        matches!(self, Self::TrustedExactBuildSupportableLane)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedExactBuildSupportableLane | Self::ReviewableReproducibilitySurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedExactBuildSupportableLane => "trusted_exact_build_supportable_lane",
            Self::ReviewableReproducibilitySurface => "reviewable_reproducibility_surface",
            Self::CacheDisciplineDisclosedProjection => "cache_discipline_disclosed_projection",
            Self::CleanRoomParityUnverifiedProjection => "clean_room_parity_unverified_projection",
            Self::ExactBuildSupportabilityUnverifiedProjection => {
                "exact_build_supportability_unverified_projection"
            }
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and build-lane-trust-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustCertificationAxis {
    /// Visual parity: canonical build lane, cache posture, publication authority, exact build identity,
    /// clean-room rebuild diff, reproducibility proof, sidecar convergence, support packet, and registry
    /// reference are shown on the primary surface without relying on a shell-chrome-only affordance or a
    /// mislabeled green publication row alone.
    Visual,
    /// Keyboard-reach parity: the same build-lane-trust truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled publication row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// build lane, cache posture, exact build identity, reproducibility proof, or registry reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the build lane, cache posture, or exact build identity.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// build-lane name, cache-posture class, clean-room-parity class, or exact-build-supportability class when
    /// a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: an untrusted or poisoned cache origin, an overclaimed clean-room parity on a
    /// partial rebuild, or an unproven exact build identity honestly downgrades a
    /// `TrustedExactBuildSupportableLane` / `ReviewableReproducibilitySurface` claim rather than reading as a
    /// fresh, fully reproducible publication lane.
    DegradedState,
    /// Build-lane-trust-component-truth parity: canonical build lane, cache posture, publication authority,
    /// exact build identity, clean-room rebuild diff, reproducibility proof, sidecar convergence, support
    /// packet, and registry reference stay explicit and never let a publication lane let a PR cache publish
    /// release artifacts, treat a remote-cache hit as reproducibility proof, drift a docs / schema / SBOM /
    /// symbol sidecar from the binary build identity, overclaim clean-room parity on a partial rebuild, or hide
    /// non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green publication rows.
    BuildLaneTrustComponentTruth,
}

impl BuildLaneTrustCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [BuildLaneTrustCertificationAxis; 9] = [
        BuildLaneTrustCertificationAxis::Visual,
        BuildLaneTrustCertificationAxis::Keyboard,
        BuildLaneTrustCertificationAxis::ScreenReader,
        BuildLaneTrustCertificationAxis::HighZoomReflow,
        BuildLaneTrustCertificationAxis::HighContrast,
        BuildLaneTrustCertificationAxis::Localization,
        BuildLaneTrustCertificationAxis::CliExport,
        BuildLaneTrustCertificationAxis::DegradedState,
        BuildLaneTrustCertificationAxis::BuildLaneTrustComponentTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrast => "high_contrast",
            Self::Localization => "localization",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::BuildLaneTrustComponentTruth => "build_lane_trust_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl BuildLaneTrustAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed
/// from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a trusted exact-build supportable lane, or the narrowing is inconsistent.
    Red,
}

impl BuildLaneTrustProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles block the
    /// release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B144 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustCertGuardrails {
    /// True if the profile lets a PR cache publish release artifacts. Must be false.
    pub pr_caches_publish_release_artifacts: bool,
    /// True if the profile treats a remote-cache hit as reproducibility proof. Must be false.
    pub treats_remote_cache_hits_as_reproducibility_proof: bool,
    /// True if the profile lets docs / schema / SBOM / symbol sidecars drift from the binary build identity.
    /// Must be false.
    pub lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity: bool,
    /// True if the profile overclaims clean-room parity when only partial artifact classes were rebuilt. Must
    /// be false.
    pub overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt: bool,
    /// True if the profile hides non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green
    /// publication rows. Must be false.
    pub hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows:
        bool,
}

impl BuildLaneTrustCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        pr_caches_publish_release_artifacts: false,
        treats_remote_cache_hits_as_reproducibility_proof: false,
        lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity: false,
        overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt: false,
        hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.pr_caches_publish_release_artifacts
            && !self.treats_remote_cache_hits_as_reproducibility_proof
            && !self.lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity
            && !self.overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt
            && !self.hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The build-lane / cache-posture / publication-authority / exact-build-identity / clean-room-parity /
    /// reproducibility-proof / sidecar-convergence / registry-reference fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl BuildLaneTrustCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: BuildLaneTrustCertificationAxis,
    /// The certification state of the axis.
    pub state: BuildLaneTrustAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5BuildLaneDowngradeTrigger>,
}

impl BuildLaneTrustAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is
    ///   exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            BuildLaneTrustAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            BuildLaneTrustAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            BuildLaneTrustAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the certified
/// claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: BuildLaneTrustCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5BuildLaneTrustClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5BuildLaneTrustClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 configuration-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustProfileCertificationRow {
    /// Record kind; must equal [`BUILD_LANE_TRUST_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BUILD_LANE_TRUST_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5BuildLaneTrustCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5BuildLaneTrustClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5BuildLaneTrustClaim,
    /// The frozen build lanes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5BuildLaneFamily>,
    /// One outcome per [`BuildLaneTrustCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<BuildLaneTrustAxisOutcome>,
    /// The B144 hard invariants; all must hold.
    pub guardrails: BuildLaneTrustCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<BuildLaneTrustClaimAutoNarrow>,
    /// The one canonical build-lane-trust proof bundle this profile cites. Must equal
    /// [`BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: BuildLaneTrustProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: BuildLaneTrustCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BuildLaneTrustProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: BuildLaneTrustCertificationAxis,
    ) -> Option<&BuildLaneTrustAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<BuildLaneTrustCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && BuildLaneTrustCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(BuildLaneTrustAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<BuildLaneTrustCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == BuildLaneTrustAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a trusted exact-build supportable lane, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> BuildLaneTrustProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return BuildLaneTrustProfileClaimStatus::Red;
        }

        // Every B144 hard invariant must hold.
        if !self.guardrails.all_held() {
            return BuildLaneTrustProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return BuildLaneTrustProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted exact-build supportable lane.
        if self.certified_claim.asserts_trusted_lane()
            && !self.profile.is_live_exact_build_supportable_lane()
        {
            return BuildLaneTrustProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(BuildLaneTrustCertificationAxis::CliExport) {
            Some(o) if o.state == BuildLaneTrustAxisCertificationState::Certified => {}
            _ => return BuildLaneTrustProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == BuildLaneTrustAxisCertificationState::UndisclosedDrift)
        {
            return BuildLaneTrustProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return BuildLaneTrustProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return BuildLaneTrustProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return BuildLaneTrustProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return BuildLaneTrustProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return BuildLaneTrustProfileClaimStatus::Red;
        }

        BuildLaneTrustProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BUILD_LANE_TRUST_CERT_ROW_RECORD_KIND
            && self.schema_version == BUILD_LANE_TRUST_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1211 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`BuildLaneTrustProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildLaneTrustProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<BuildLaneTrustProfileCertificationRow>,
}

/// Checked-in M05-1211 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<BuildLaneTrustProfileCertificationRow>,
    pub summary: BuildLaneTrustProfileCertificationSummary,
}

impl BuildLaneTrustProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: BuildLaneTrustProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BUILD_LANE_TRUST_CERT_SCHEMA_VERSION,
            record_kind: BUILD_LANE_TRUST_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: BuildLaneTrustProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5BuildLaneTrustCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Build lanes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5BuildLaneFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5BuildLaneTrustCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen build lane is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5BuildLaneFamily::ALL.iter().all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(BuildLaneTrustCertificationAxis::CliExport)
                .is_some_and(|o| o.state == BuildLaneTrustAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BuildLaneTrustProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BuildLaneTrustProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BuildLaneTrustProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BuildLaneTrustProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(BuildLaneTrustProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        BuildLaneTrustProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(BuildLaneTrustProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BuildLaneTrustCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUILD_LANE_TRUST_CERT_SCHEMA_VERSION {
            violations.push(BuildLaneTrustCertificationViolation::SchemaVersion {
                expected: BUILD_LANE_TRUST_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BUILD_LANE_TRUST_CERT_RECORD_KIND {
            violations.push(BuildLaneTrustCertificationViolation::RecordKind {
                expected: BUILD_LANE_TRUST_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BuildLaneTrustCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF {
            violations.push(BuildLaneTrustCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BuildLaneTrustCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(BuildLaneTrustCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    BuildLaneTrustCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(BuildLaneTrustCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    BuildLaneTrustCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B144 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(BuildLaneTrustCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted exact-build supportable lane.
            if row.certified_claim.asserts_trusted_lane()
                && !row.profile.is_live_exact_build_supportable_lane()
            {
                violations.push(
                    BuildLaneTrustCertificationViolation::NonLiveProfileClaimsTrustedLane {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(BuildLaneTrustCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    BuildLaneTrustCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    BuildLaneTrustCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    BuildLaneTrustCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == BuildLaneTrustProfileClaimStatus::Red {
                violations.push(BuildLaneTrustCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(BuildLaneTrustCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen build lane must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(BuildLaneTrustCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(BuildLaneTrustCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations
                .push(BuildLaneTrustCertificationViolation::RawBuildLaneTrustMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Build-Lane-Trust Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5BuildLaneTrustCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Invariants held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_build_lane_trust_surface_certification_export(
) -> Result<BuildLaneTrustProfileCertificationPacket, BuildLaneTrustCertificationArtifactError> {
    let packet: BuildLaneTrustProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-build-lane-trust-surface-certification/support_export.json"
        )))
        .map_err(BuildLaneTrustCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BuildLaneTrustCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum BuildLaneTrustCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BuildLaneTrustCertificationViolation>),
}

impl fmt::Display for BuildLaneTrustCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BuildLaneTrustCertificationArtifactError {}

/// Validation failure for M05-1211 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildLaneTrustCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLiveProfileClaimsTrustedLane { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawBuildLaneTrustMaterialInExport,
}

impl fmt::Display for BuildLaneTrustCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical build-lane-trust proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical build-lane-trust proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B144 hard invariant: letting a PR cache publish release artifacts; treating \
a remote-cache hit as reproducibility proof; letting docs / schema / SBOM / symbol sidecars drift from the \
binary build identity; overclaiming clean-room parity when only partial artifact classes were rebuilt; or \
hiding non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green publication rows"
                )
            }
            Self::NonLiveProfileClaimsTrustedLane { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted exact-build supportable lane on a non-live first-party profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted claim, a hard \
invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted exact-build supportable lane, \
or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 publication-bearing profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen build lane is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBuildLaneTrustMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for BuildLaneTrustCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&BuildLaneTrustAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != BuildLaneTrustAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the build-lane-trust
/// generics the spec forbids collapsing distinct build-lane, cache-posture, publication-authority,
/// exact-build-identity, clean-room-parity, reproducibility-proof, and exact-build-supportability truth into
/// (whole-label matches so a full sentence naming a concrete build lane, cache origin, or registry reference is
/// not flagged).
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "something went wrong"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "pending"
            | "loading"
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "build"
            | "build lane"
            | "lane"
            | "trust"
            | "cache"
            | "cache posture"
            | "remote cache"
            | "publication"
            | "publication authority"
            | "credential"
            | "credential boundary"
            | "hermetic"
            | "clean room"
            | "clean-room"
            | "clean room parity"
            | "clean-room parity"
            | "rebuild"
            | "reproducibility"
            | "reproducibility proof"
            | "reproducible"
            | "build identity"
            | "exact build"
            | "exact build identity"
            | "supportability"
            | "symbolication"
            | "sidecar"
            | "artifact"
            | "artifact convergence"
            | "attestation"
            | "provenance"
            | "mirror"
            | "release"
            | "hotfix"
            | "registry reference"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the build-lane-trust
/// matrix and M05-1210 heuristic so the reused [`M5BuildLaneDowngradeTrigger`] narrowings serialize
/// cleanly — the build-lane-trust grammar carries only typed class tokens and opaque refs, never raw
/// secret values or endpoints.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1211 certification packet. Certifies all five claimed M5
/// configuration-bearing profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_build_lane_trust_surface_certification_packet(
) -> BuildLaneTrustProfileCertificationPacket {
    BuildLaneTrustProfileCertificationPacket::new(BuildLaneTrustProfileCertificationPacketInput {
        packet_id: BUILD_LANE_TRUST_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-15T00:00:00Z".to_owned(),
        matrix_ref: BUILD_LANE_TRUST_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:build-lane-trust-surface-certification:{id}"),
        BUILD_LANE_TRUST_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> BuildLaneTrustCertExportParity {
    BuildLaneTrustCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: BuildLaneTrustCertificationAxis) -> &'static str {
    match axis {
        BuildLaneTrustCertificationAxis::Visual => {
            "canonical build lane, cache posture, publication authority, exact build identity, clean-room rebuild diff, reproducibility proof, sidecar convergence, support packet, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled green publication row alone"
        }
        BuildLaneTrustCertificationAxis::Keyboard => {
            "the same build-lane-trust role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        BuildLaneTrustCertificationAxis::ScreenReader => {
            "the same build-lane-trust truth is announced non-visually, never a shell-chrome-only / mislabeled-publication-row / unlabeled-control-only cue"
        }
        BuildLaneTrustCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the build lane, cache posture, exact build identity, reproducibility proof, or registry reference"
        }
        BuildLaneTrustCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the build lane, cache posture, or exact build identity"
        }
        BuildLaneTrustCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a build-lane name, cache-posture class, clean-room-parity class, or exact-build-supportability class"
        }
        BuildLaneTrustCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        BuildLaneTrustCertificationAxis::DegradedState => {
            "an untrusted or poisoned cache origin, an overclaimed clean-room parity on a partial rebuild, or an unproven exact build identity honestly downgrades the TrustedExactBuildSupportableLane/ReviewableReproducibilitySurface claim rather than reading as a fresh, fully reproducible publication lane"
        }
        BuildLaneTrustCertificationAxis::BuildLaneTrustComponentTruth => {
            "canonical build lane, cache posture, publication authority, exact build identity, clean-room rebuild diff, reproducibility proof, sidecar convergence, support packet, and registry reference stay explicit and never let a publication lane let a PR cache publish release artifacts, treat a remote-cache hit as reproducibility proof, drift a docs / schema / SBOM / symbol sidecar from the binary build identity, overclaim clean-room parity on a partial rebuild, or hide non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green publication rows"
        }
    }
}

fn seed_certified(axis: BuildLaneTrustCertificationAxis) -> BuildLaneTrustAxisOutcome {
    BuildLaneTrustAxisOutcome {
        axis,
        state: BuildLaneTrustAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: BuildLaneTrustCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5BuildLaneDowngradeTrigger,
) -> BuildLaneTrustAxisOutcome {
    BuildLaneTrustAxisOutcome {
        axis,
        state: BuildLaneTrustAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<BuildLaneTrustAxisOutcome> {
    BuildLaneTrustCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: BuildLaneTrustCertificationAxis,
    outcome: BuildLaneTrustAxisOutcome,
) -> Vec<BuildLaneTrustAxisOutcome> {
    BuildLaneTrustCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5BuildLaneTrustCertifiedProfile,
    claimed_claim: M5BuildLaneTrustClaim,
    certified_claim: M5BuildLaneTrustClaim,
    consumed_families: &[M5BuildLaneFamily],
    axis_outcomes: Vec<BuildLaneTrustAxisOutcome>,
    claim_auto_narrow: Option<BuildLaneTrustClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> BuildLaneTrustProfileCertificationRow {
    let mut row = BuildLaneTrustProfileCertificationRow {
        record_kind: BUILD_LANE_TRUST_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: BUILD_LANE_TRUST_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: BuildLaneTrustCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: BUILD_LANE_TRUST_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: BuildLaneTrustProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            BUILD_LANE_TRUST_CERT_MATRIX_REF.to_owned(),
            BUILD_LANE_TRUST_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-15T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: BuildLaneTrustCertificationAxis,
    from_claim: M5BuildLaneTrustClaim,
    to_claim: M5BuildLaneTrustClaim,
    label: &str,
) -> BuildLaneTrustClaimAutoNarrow {
    BuildLaneTrustClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<BuildLaneTrustProfileCertificationRow> {
    use BuildLaneTrustCertificationAxis as Ax;
    use M5BuildLaneDowngradeTrigger as Trig;
    use M5BuildLaneFamily::*;
    use M5BuildLaneTrustCertifiedProfile as P;
    use M5BuildLaneTrustClaim::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-exact-build-supportable-lane",
            P::LiveExactBuildSupportableLane,
            TrustedExactBuildSupportableLane,
            TrustedExactBuildSupportableLane,
            &[Release],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "exact_build_identity",
            ],
            &[
                "release lane: binaries, packages, docs packs, schemas, SBOMs, symbols, source maps, rollback metadata, and support packets converge on one exact build identity from verified or re-materialized inputs with a fresh clean-room rebuild diff and reproducibility proof, never a remote-cache hit dressed up as reproducibility proof",
                "the trusted exact-build supportable lane keeps stable operation IDs while the build lane, cache posture, publication authority, and exact build identity bind to the one build-lane-trust registry across release-center / shiproom / diagnostics / support",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered publication lane",
                "build-lane-trust-component-truth: a live, fully reproducible first-party release lane is the only profile that certifies a trusted exact-build supportable lane",
            ],
        ),
        seed_row(
            "cert:reviewable-reproducibility-structure",
            P::ReviewableReproducibilityStructure,
            ReviewableReproducibilitySurface,
            ReviewableReproducibilitySurface,
            &[ProtectedMerge],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "reproducibility_proof",
            ],
            &[
                "protected-merge lane: controlled credentials and verified caches only, with the clean-room rebuild diff, reproducibility proof, and sidecar convergence bound to the single build-lane-trust registry and inspectable before promotion rather than a per-surface description copied by hand, and build-identity continuity preserved across the merge",
                "the reviewable reproducibility structure keeps its build-lane, cache-posture, reproducibility-proof, and registry labels inspectable rather than a shell-chrome-only or mislabeled-publication-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable reproducibility structure",
                "build-lane-trust-component-truth: a reviewable reproducibility structure never certifies a live, fully reproducible publication claim and never lets a PR cache publish release artifacts",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-cache-discipline-profile",
            P::DisclosedCacheDisciplineProfile,
            ReviewableReproducibilitySurface,
            CacheDisciplineDisclosedProjection,
            &[ContributorPr],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the contributor / PR lane read a shared remote cache whose origin trust can only be partially disclosed for this profile so a fully verified cache posture cannot be certified",
                    "The contributor / PR lane read a shared remote cache whose origin trust can only be partially disclosed, so the ReviewableReproducibilitySurface claim narrows to a cache-discipline-disclosed projection and the lane discloses the untrusted cache origin alongside its withheld publication authority rather than presenting a shared cache read as a verified, publishable posture or letting a PR cache publish release artifacts",
                    Trig::UsedAnUntrustedCache,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableReproducibilitySurface,
                CacheDisciplineDisclosedProjection,
                "Cache discipline disclosed partial: the contributor / PR cache origin trust is only partially proven so it is disclosed alongside the withheld publication authority and no PR cache publishes release artifacts",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "contributor / PR lane: the cache posture names its shared, readable, non-publishing origin and marks the untrusted-cache read as disclosed-partial rather than publishing release artifacts from a PR cache when the origin trust is incomplete",
                "the contributor / PR surface keeps its cache posture, withheld publication authority, and untrusted-cache origin legible while the cache origin trust is disclosed as partial",
                "localization: ReviewableReproducibilitySurface narrows to a cache-discipline-disclosed projection (auto-narrowed)",
                "build-lane-trust-component-truth: a partially-trusted shared cache never publishes release artifacts — the withheld publication authority is preserved",
            ],
        ),
        seed_row(
            "cert:unverified-clean-room-parity-profile",
            P::UnverifiedCleanRoomParityProfile,
            ReviewableReproducibilitySurface,
            CleanRoomParityUnverifiedProjection,
            &[Release],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the clean-room rebuild covered only a partial set of artifact classes so a fully converged clean-room parity cannot be certified",
                    "The clean-room rebuild covered only a partial set of artifact classes, so the ReviewableReproducibilitySurface claim narrows to a clean-room-parity-unverified projection and the lane keeps the last-known partial-rebuild posture explicit rather than presenting a partial rebuild as full clean-room parity when only some artifact classes were rebuilt",
                    Trig::OverclaimedCleanRoomParityOnPartialRebuild,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableReproducibilitySurface,
                CleanRoomParityUnverifiedProjection,
                "Clean-room parity unverified: only a partial set of artifact classes was rebuilt so the last-known partial-rebuild posture stays explicit and no partial rebuild is presented as full clean-room parity",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "release lane: the clean-room rebuild keeps its per-artifact-class rebuild posture explicit and marks the parity as unverified rather than overclaiming clean-room parity when only partial artifact classes were rebuilt, and never collapses a partial rebuild into a full-parity claim",
                "the release surface keeps its clean-room rebuild diff and per-class coverage lineage legible while the clean-room parity is disclosed as unverified",
                "degraded-state: ReviewableReproducibilitySurface narrows to a clean-room-parity-unverified projection (auto-narrowed)",
                "build-lane-trust-component-truth: a clean-room rebuild never overclaims parity and never presents a partial artifact-class rebuild as full clean-room convergence",
            ],
        ),
        seed_row(
            "cert:unverified-exact-build-supportability-profile",
            P::UnverifiedExactBuildSupportabilityProfile,
            ReviewableReproducibilitySurface,
            ExactBuildSupportabilityUnverifiedProjection,
            &[EmergencyHotfix],
            seed_certified_except(
                Ax::BuildLaneTrustComponentTruth,
                seed_narrowed(
                    Ax::BuildLaneTrustComponentTruth,
                    "a docs / schema / SBOM / symbol sidecar has drifted from the binary build identity or aged out so exact-build symbolication and support convergence cannot be certified",
                    "A docs / schema / SBOM / symbol sidecar has drifted from the binary build identity or aged out, so the ReviewableReproducibilitySurface claim narrows to an exact-build-supportability-unverified projection and the lane keeps the last-known sidecar-drift posture explicit rather than presenting a support packet as converged on one exact build identity or letting a sidecar drift behind a green publication row",
                    Trig::DriftedASidecarFromTheBinaryBuildIdentity,
                ),
            ),
            Some(seed_narrow(
                Ax::BuildLaneTrustComponentTruth,
                ReviewableReproducibilitySurface,
                ExactBuildSupportabilityUnverifiedProjection,
                "Exact-build supportability unverified: a sidecar has drifted from the binary build identity so the last-known sidecar-drift posture stays explicit and no support packet is shown as converged on one exact build identity",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "emergency-hotfix lane: the support packet keeps its symbol / source-map / SBOM sidecar and exact build identity explicit and marks the supportability as unverified rather than letting a docs / schema / SBOM / symbol sidecar drift from the binary build identity behind a green publication row",
                "the emergency-hotfix surface keeps its support packet and exact build identity legible while the sidecar-convergence state is disclosed as unverified",
                "build-lane-trust-component-truth: ReviewableReproducibilitySurface narrows to an exact-build-supportability-unverified projection (auto-narrowed)",
                "build-lane-trust-component-truth: a support packet converges on one exact build identity and never lets a drifted sidecar read as a clean pass, and no supportability claim outpaces the verified build identity",
            ],
        ),
    ]
}
