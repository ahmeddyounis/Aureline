//! Cross-surface safe-preview trust-class projection.
//!
//! This module is a thin shell consumer of the shared trust-class and
//! representation vocabularies in [`aureline_content_safety`] and
//! [`aureline_preview`]. It gives docs, preview, and package/install review
//! surfaces one inspectable packet so chrome, support export, and fixture
//! replay can answer the same questions before copy, export, active-open, or
//! mutation actions are available:
//!
//! - Which trust class was requested, and which one is effective now?
//! - Which representation is currently visible?
//! - Which copy/export representation would leave the product?
//! - Did a degraded active surface narrow to a truthful static or metadata
//!   mode before any active behavior remained reachable?

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use aureline_content_safety::{
    BodyPosture, RepresentationActionId, RepresentationClass, SurfaceFamily, TrustClass,
};
use aureline_preview::CurrentlyVisibleRepresentation;

/// Stable record-kind tag carried by [`TrustClassAlphaPacket`].
pub const TRUST_CLASS_ALPHA_PACKET_RECORD_KIND: &str = "trust_class_alpha_packet_record";

/// Schema version for the alpha trust-class shell projection.
pub const TRUST_CLASS_ALPHA_PACKET_SCHEMA_VERSION: u32 = 1;

/// Shell lane that consumes the shared trust-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClassLane {
    /// Docs/help content rendered through sanitized or raw inspection paths.
    Docs,
    /// Runtime or rich preview content, including active local or isolated remote content.
    Preview,
    /// Package or install review surfaces before apply.
    PackageInstall,
}

impl TrustClassLane {
    /// Returns the stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docs => "docs",
            Self::Preview => "preview",
            Self::PackageInstall => "package_install",
        }
    }
}

/// Effective fallback mode when a surface cannot keep its richer trust class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClassFallbackMode {
    /// No fallback is active.
    None,
    /// The surface narrows to raw or escaped source inspection.
    RawInspection,
    /// The surface narrows to a sanitized static snapshot.
    SanitizedStaticSnapshot,
    /// The surface carries only metadata and omits the body.
    MetadataOnlyEnvelope,
}

impl TrustClassFallbackMode {
    /// Returns the stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RawInspection => "raw_inspection",
            Self::SanitizedStaticSnapshot => "sanitized_static_snapshot",
            Self::MetadataOnlyEnvelope => "metadata_only_envelope",
        }
    }

    /// Returns the user-facing fallback label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "No fallback active",
            Self::RawInspection => "Raw inspection fallback",
            Self::SanitizedStaticSnapshot => "Sanitized static snapshot",
            Self::MetadataOnlyEnvelope => "Metadata-only envelope",
        }
    }

    fn is_active(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Family for an action shown from a trust-class projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClassActionFamily {
    /// Copy action.
    Copy,
    /// Export action.
    Export,
    /// Mutation action that can change workspace, package, or install state.
    Mutate,
    /// Active-content open or run action.
    ActiveOpen,
}

impl TrustClassActionFamily {
    /// Returns the stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Export => "export",
            Self::Mutate => "mutate",
            Self::ActiveOpen => "active_open",
        }
    }
}

/// Availability posture for an action row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClassActionPosture {
    /// Action is currently available under the effective trust class.
    Available,
    /// Action requires a preview or review packet before execution.
    RequiresPreview,
    /// Action is blocked under the current effective trust class.
    Blocked,
    /// Action is available only as the degraded fallback representation.
    DegradedFallback,
}

impl TrustClassActionPosture {
    /// Returns the stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RequiresPreview => "requires_preview",
            Self::Blocked => "blocked",
            Self::DegradedFallback => "degraded_fallback",
        }
    }
}

/// Input describing one shell surface that needs a trust-class projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustClassSurfaceInput {
    /// Stable surface id.
    pub surface_id: String,
    /// Product lane consuming the projection.
    pub lane: TrustClassLane,
    /// Shared surface family from the content-safety trust-class contract.
    pub surface_family: SurfaceFamily,
    /// Opaque subject ref for the rendered content.
    pub subject_ref: String,
    /// Opaque owner identity ref.
    pub owner_identity_ref: String,
    /// Opaque origin identity ref.
    pub origin_identity_ref: String,
    /// Origin-kind token from the trust-class schema.
    pub origin_kind: String,
    /// Trust class requested by the source surface.
    pub nominal_trust_class: TrustClass,
    /// Trust class currently effective after policy, origin, and health checks.
    pub effective_trust_class: TrustClass,
    /// On-screen representation visible before action.
    pub visible_representation: CurrentlyVisibleRepresentation,
    /// Fallback mode currently in effect.
    pub fallback_mode: TrustClassFallbackMode,
    /// Downgrade triggers observed for this surface.
    pub downgrade_trigger_observations: Vec<String>,
    /// Content-integrity cue tokens that must stay visible on the surface.
    pub content_integrity_cues: Vec<String>,
    /// Related packet, schema, fixture, or support refs.
    pub related_record_refs: Vec<String>,
}

/// Action row projected for copy, export, active-open, or mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustClassActionProjection {
    /// Stable action id.
    pub action_id: String,
    /// Action family enum.
    pub action_family: TrustClassActionFamily,
    /// Stable token for [`Self::action_family`].
    pub action_family_token: String,
    /// Availability posture enum.
    pub posture: TrustClassActionPosture,
    /// Stable token for [`Self::posture`].
    pub posture_token: String,
    /// Transfer representation class when this action moves a representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub representation_class: Option<String>,
    /// Body posture when this action moves a representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_posture: Option<String>,
    /// Label shown for the action.
    pub label: String,
    /// Fields that must be visible before the action can be invoked.
    pub required_pre_action_disclosures: Vec<String>,
    /// Reason shown when the action is blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
}

/// One surface projection in a trust-class packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustClassSurfaceProjection {
    /// Stable surface id.
    pub surface_id: String,
    /// Product lane enum.
    pub lane: TrustClassLane,
    /// Stable token for [`Self::lane`].
    pub lane_token: String,
    /// Shared surface family token.
    pub surface_family_token: String,
    /// Opaque subject ref for the rendered content.
    pub subject_ref: String,
    /// Opaque owner identity ref.
    pub owner_identity_ref: String,
    /// Opaque origin identity ref.
    pub origin_identity_ref: String,
    /// Origin-kind token from the trust-class schema.
    pub origin_kind: String,
    /// Requested trust-class token.
    pub nominal_trust_class_token: String,
    /// Effective trust-class token.
    pub effective_trust_class_token: String,
    /// On-screen representation token.
    pub visible_representation_token: String,
    /// On-screen representation label.
    pub visible_representation_label: String,
    /// Fallback mode enum.
    pub fallback_mode: TrustClassFallbackMode,
    /// Stable token for [`Self::fallback_mode`].
    pub fallback_mode_token: String,
    /// Fallback label.
    pub fallback_label: String,
    /// True when the effective posture narrowed from the nominal posture.
    pub is_degraded: bool,
    /// Owner/origin chrome fields that must remain visible.
    pub required_owner_origin_chrome: Vec<String>,
    /// Content-integrity cue tokens visible on the surface.
    pub content_integrity_cues: Vec<String>,
    /// Downgrade triggers observed for this surface.
    pub downgrade_trigger_observations: Vec<String>,
    /// Copy/export action rows.
    pub copy_export_actions: Vec<TrustClassActionProjection>,
    /// Active-open or mutation action rows.
    pub mutation_actions: Vec<TrustClassActionProjection>,
    /// Related packet, schema, fixture, or support refs.
    pub related_record_refs: Vec<String>,
}

impl TrustClassSurfaceProjection {
    /// Projects one shell surface from the shared trust-class vocabulary.
    pub fn project(input: TrustClassSurfaceInput) -> Self {
        let is_degraded = input.fallback_mode.is_active()
            || input.nominal_trust_class != input.effective_trust_class
            || !input.downgrade_trigger_observations.is_empty();
        let mut content_integrity_cues = input.content_integrity_cues.clone();
        push_unique(
            &mut content_integrity_cues,
            cue_for_trust_class(input.nominal_trust_class),
        );
        if is_degraded {
            push_unique(&mut content_integrity_cues, "degraded_fallback_visible");
        }

        let required_owner_origin_chrome = owner_origin_chrome_for(&input, is_degraded);
        let disclosures = pre_action_disclosures_for(&input, is_degraded);
        let copy_export_actions = copy_export_actions_for(&input, &disclosures, is_degraded);
        let mutation_actions = mutation_actions_for(&input, &disclosures, is_degraded);

        Self {
            surface_id: input.surface_id,
            lane: input.lane,
            lane_token: input.lane.as_str().to_owned(),
            surface_family_token: input.surface_family.as_str().to_owned(),
            subject_ref: input.subject_ref,
            owner_identity_ref: input.owner_identity_ref,
            origin_identity_ref: input.origin_identity_ref,
            origin_kind: input.origin_kind,
            nominal_trust_class_token: input.nominal_trust_class.as_str().to_owned(),
            effective_trust_class_token: input.effective_trust_class.as_str().to_owned(),
            visible_representation_token: input.visible_representation.as_str().to_owned(),
            visible_representation_label: input.visible_representation.label().to_owned(),
            fallback_mode: input.fallback_mode,
            fallback_mode_token: input.fallback_mode.as_str().to_owned(),
            fallback_label: input.fallback_mode.label().to_owned(),
            is_degraded,
            required_owner_origin_chrome,
            content_integrity_cues,
            downgrade_trigger_observations: input.downgrade_trigger_observations,
            copy_export_actions,
            mutation_actions,
            related_record_refs: input.related_record_refs,
        }
    }

    /// Returns every action projected for this surface.
    pub fn all_actions(&self) -> impl Iterator<Item = &TrustClassActionProjection> {
        self.copy_export_actions
            .iter()
            .chain(self.mutation_actions.iter())
    }
}

/// Inspectable shell packet that joins docs, preview, and install surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustClassAlphaPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Projected surfaces.
    pub surfaces: Vec<TrustClassSurfaceProjection>,
}

impl TrustClassAlphaPacket {
    /// Builds an inspectable packet from surface inputs.
    pub fn new(packet_id: impl Into<String>, inputs: Vec<TrustClassSurfaceInput>) -> Self {
        Self {
            record_kind: TRUST_CLASS_ALPHA_PACKET_RECORD_KIND.to_owned(),
            schema_version: TRUST_CLASS_ALPHA_PACKET_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            surfaces: inputs
                .into_iter()
                .map(TrustClassSurfaceProjection::project)
                .collect(),
        }
    }

    /// Returns true when docs, preview, and package/install lanes are present.
    pub fn covers_required_lanes(&self) -> bool {
        let lanes: BTreeSet<_> = self.surfaces.iter().map(|s| s.lane).collect();
        [
            TrustClassLane::Docs,
            TrustClassLane::Preview,
            TrustClassLane::PackageInstall,
        ]
        .into_iter()
        .all(|lane| lanes.contains(&lane))
    }

    /// Returns true when all four frozen trust classes appear in the packet.
    pub fn covers_all_trust_classes(&self) -> bool {
        let tokens: BTreeSet<_> = self
            .surfaces
            .iter()
            .flat_map(|surface| {
                [
                    surface.nominal_trust_class_token.as_str(),
                    surface.effective_trust_class_token.as_str(),
                ]
            })
            .collect();
        [
            TrustClass::RawText.as_str(),
            TrustClass::SanitizedRich.as_str(),
            TrustClass::TrustedLocalActive.as_str(),
            TrustClass::IsolatedRemoteActive.as_str(),
        ]
        .into_iter()
        .all(|token| tokens.contains(token))
    }

    /// Returns true when every action exposes trust class, visible
    /// representation, and representation label before invocation.
    pub fn all_actions_disclose_pre_action_state(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            surface.all_actions().all(|action| {
                required_pre_action_fields().iter().all(|field| {
                    action
                        .required_pre_action_disclosures
                        .iter()
                        .any(|f| f == field)
                })
            })
        })
    }

    /// Returns true when degraded surfaces narrow to static, raw, or
    /// metadata-only posture and do not leave active behavior available.
    pub fn degraded_surfaces_fallback_truthfully(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            if !surface.is_degraded {
                return true;
            }
            if !surface.fallback_mode.is_active() {
                return false;
            }
            let nominal_active = surface.nominal_trust_class_token
                == TrustClass::TrustedLocalActive.as_str()
                || surface.nominal_trust_class_token == TrustClass::IsolatedRemoteActive.as_str();
            let effective_active = surface.effective_trust_class_token
                == TrustClass::TrustedLocalActive.as_str()
                || surface.effective_trust_class_token == TrustClass::IsolatedRemoteActive.as_str();
            if nominal_active && effective_active {
                return false;
            }
            surface
                .mutation_actions
                .iter()
                .filter(|action| action.action_family == TrustClassActionFamily::ActiveOpen)
                .all(|action| action.posture == TrustClassActionPosture::Blocked)
        })
    }

    /// Returns true when workspace or package mutation actions require a
    /// preview/review packet or are blocked.
    pub fn mutation_actions_require_preview_or_block(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            surface
                .mutation_actions
                .iter()
                .filter(|action| action.action_family == TrustClassActionFamily::Mutate)
                .all(|action| {
                    matches!(
                        action.posture,
                        TrustClassActionPosture::RequiresPreview | TrustClassActionPosture::Blocked
                    )
                })
        })
    }

    /// Returns true when every copy/export action names representation and
    /// body posture.
    pub fn copy_export_actions_have_representation(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            surface.copy_export_actions.iter().all(|action| {
                action.representation_class.is_some() && action.body_posture.is_some()
            })
        })
    }

    /// Validates the alpha trust-class shell packet.
    pub fn validate(&self) -> Vec<TrustClassAlphaViolation> {
        let mut out = Vec::new();

        let lanes: BTreeSet<_> = self.surfaces.iter().map(|s| s.lane).collect();
        for lane in [
            TrustClassLane::Docs,
            TrustClassLane::Preview,
            TrustClassLane::PackageInstall,
        ] {
            if !lanes.contains(&lane) {
                out.push(TrustClassAlphaViolation::MissingLane {
                    lane: lane.as_str().to_owned(),
                });
            }
        }

        let trust_classes: BTreeSet<_> = self
            .surfaces
            .iter()
            .flat_map(|surface| {
                [
                    surface.nominal_trust_class_token.as_str(),
                    surface.effective_trust_class_token.as_str(),
                ]
            })
            .collect();
        for trust_class in [
            TrustClass::RawText.as_str(),
            TrustClass::SanitizedRich.as_str(),
            TrustClass::TrustedLocalActive.as_str(),
            TrustClass::IsolatedRemoteActive.as_str(),
        ] {
            if !trust_classes.contains(trust_class) {
                out.push(TrustClassAlphaViolation::MissingTrustClass {
                    trust_class: trust_class.to_owned(),
                });
            }
        }

        for surface in &self.surfaces {
            for action in surface.all_actions() {
                for required in required_pre_action_fields() {
                    if !action
                        .required_pre_action_disclosures
                        .iter()
                        .any(|field| field == required)
                    {
                        out.push(TrustClassAlphaViolation::MissingPreActionDisclosure {
                            surface_id: surface.surface_id.clone(),
                            action_id: action.action_id.clone(),
                            field: required.to_owned(),
                        });
                    }
                }
                if matches!(
                    action.action_family,
                    TrustClassActionFamily::Copy | TrustClassActionFamily::Export
                ) && (action.representation_class.is_none() || action.body_posture.is_none())
                {
                    out.push(TrustClassAlphaViolation::CopyExportMissingRepresentation {
                        surface_id: surface.surface_id.clone(),
                        action_id: action.action_id.clone(),
                    });
                }
            }

            if surface.is_degraded && !surface.fallback_mode.is_active() {
                out.push(TrustClassAlphaViolation::DegradedSurfaceWithoutFallback {
                    surface_id: surface.surface_id.clone(),
                });
            }

            let nominal_active = surface.nominal_trust_class_token
                == TrustClass::TrustedLocalActive.as_str()
                || surface.nominal_trust_class_token == TrustClass::IsolatedRemoteActive.as_str();
            let effective_active = surface.effective_trust_class_token
                == TrustClass::TrustedLocalActive.as_str()
                || surface.effective_trust_class_token == TrustClass::IsolatedRemoteActive.as_str();
            if surface.is_degraded && nominal_active && effective_active {
                out.push(TrustClassAlphaViolation::DegradedActiveSurfaceStillActive {
                    surface_id: surface.surface_id.clone(),
                    effective_trust_class: surface.effective_trust_class_token.clone(),
                });
            }

            for action in &surface.mutation_actions {
                if surface.is_degraded
                    && action.action_family == TrustClassActionFamily::ActiveOpen
                    && action.posture == TrustClassActionPosture::Available
                {
                    out.push(TrustClassAlphaViolation::DegradedActionStillAvailable {
                        surface_id: surface.surface_id.clone(),
                        action_id: action.action_id.clone(),
                    });
                }
                if action.action_family == TrustClassActionFamily::Mutate
                    && !matches!(
                        action.posture,
                        TrustClassActionPosture::RequiresPreview | TrustClassActionPosture::Blocked
                    )
                {
                    out.push(TrustClassAlphaViolation::MutationWithoutPreview {
                        surface_id: surface.surface_id.clone(),
                        action_id: action.action_id.clone(),
                        posture: action.posture_token.clone(),
                    });
                }
            }
        }

        out
    }

    /// Renders a deterministic support/export summary of the packet.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Safe-preview trust-class packet\n");
        out.push_str(&format!(
            "packet={} schema={}\n",
            self.packet_id, self.schema_version
        ));
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- {surface_id} lane={lane} surface={family} trust={nominal} effective={effective} visible={visible} fallback={fallback}\n",
                surface_id = surface.surface_id,
                lane = surface.lane_token,
                family = surface.surface_family_token,
                nominal = surface.nominal_trust_class_token,
                effective = surface.effective_trust_class_token,
                visible = surface.visible_representation_token,
                fallback = surface.fallback_mode_token,
            ));
            out.push_str(&format!(
                "  cues={}\n",
                surface.content_integrity_cues.join(",")
            ));
            out.push_str("  actions:\n");
            for action in surface.all_actions() {
                out.push_str(&format!(
                    "    - {} family={} posture={} representation={} body={} label={}\n",
                    action.action_id,
                    action.action_family_token,
                    action.posture_token,
                    action.representation_class.as_deref().unwrap_or("(none)"),
                    action.body_posture.as_deref().unwrap_or("(none)"),
                    action.label,
                ));
            }
        }
        out
    }
}

/// Builds an inspectable trust-class packet from surface inputs.
pub fn build_trust_class_alpha_packet(
    packet_id: impl Into<String>,
    inputs: Vec<TrustClassSurfaceInput>,
) -> TrustClassAlphaPacket {
    TrustClassAlphaPacket::new(packet_id, inputs)
}

/// Validation issue emitted by [`TrustClassAlphaPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TrustClassAlphaViolation {
    /// A required product lane was not present.
    MissingLane { lane: String },
    /// A frozen trust class was not exercised by the packet.
    MissingTrustClass { trust_class: String },
    /// An action omitted a pre-action disclosure field.
    MissingPreActionDisclosure {
        surface_id: String,
        action_id: String,
        field: String,
    },
    /// A copy/export action omitted representation or body posture.
    CopyExportMissingRepresentation {
        surface_id: String,
        action_id: String,
    },
    /// A degraded surface did not state its fallback mode.
    DegradedSurfaceWithoutFallback { surface_id: String },
    /// A degraded active surface kept an active effective trust class.
    DegradedActiveSurfaceStillActive {
        surface_id: String,
        effective_trust_class: String,
    },
    /// A degraded surface left an active-open action available.
    DegradedActionStillAvailable {
        surface_id: String,
        action_id: String,
    },
    /// A mutation action was available without preview/review.
    MutationWithoutPreview {
        surface_id: String,
        action_id: String,
        posture: String,
    },
}

impl TrustClassAlphaViolation {
    /// Returns the stable violation token.
    pub const fn token(&self) -> &'static str {
        match self {
            Self::MissingLane { .. } => "missing_lane",
            Self::MissingTrustClass { .. } => "missing_trust_class",
            Self::MissingPreActionDisclosure { .. } => "missing_pre_action_disclosure",
            Self::CopyExportMissingRepresentation { .. } => "copy_export_missing_representation",
            Self::DegradedSurfaceWithoutFallback { .. } => "degraded_surface_without_fallback",
            Self::DegradedActiveSurfaceStillActive { .. } => "degraded_active_surface_still_active",
            Self::DegradedActionStillAvailable { .. } => "degraded_action_still_available",
            Self::MutationWithoutPreview { .. } => "mutation_without_preview",
        }
    }
}

fn copy_export_actions_for(
    input: &TrustClassSurfaceInput,
    disclosures: &[String],
    is_degraded: bool,
) -> Vec<TrustClassActionProjection> {
    if input.fallback_mode == TrustClassFallbackMode::MetadataOnlyEnvelope {
        return vec![transfer_action(
            RepresentationActionId::ExportMetadataOnly,
            RepresentationClass::BlockedMetadataOnly,
            BodyPosture::MetadataOnlyEnvelope,
            "Export metadata only",
            TrustClassActionPosture::DegradedFallback,
            disclosures,
            None,
        )];
    }

    let posture = if is_degraded {
        TrustClassActionPosture::DegradedFallback
    } else {
        TrustClassActionPosture::Available
    };

    match input.effective_trust_class {
        TrustClass::RawText => vec![
            transfer_action(
                RepresentationActionId::CopyRaw,
                RepresentationClass::Raw,
                BodyPosture::ExactSourceBytes,
                "Copy raw source",
                posture,
                disclosures,
                None,
            ),
            transfer_action(
                RepresentationActionId::CopyEscaped,
                RepresentationClass::Escaped,
                BodyPosture::EscapedSourceText,
                "Copy escaped source",
                posture,
                disclosures,
                None,
            ),
            transfer_action(
                RepresentationActionId::ExportMetadataOnly,
                RepresentationClass::BlockedMetadataOnly,
                BodyPosture::MetadataOnlyEnvelope,
                "Export metadata only",
                posture,
                disclosures,
                None,
            ),
        ],
        TrustClass::SanitizedRich => vec![
            transfer_action(
                RepresentationActionId::CopyRendered,
                RepresentationClass::Rendered,
                BodyPosture::RenderedView,
                if is_degraded {
                    "Copy rendered static snapshot"
                } else {
                    "Copy rendered preview"
                },
                posture,
                disclosures,
                None,
            ),
            transfer_action(
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationClass::Sanitized,
                BodyPosture::SanitizedStaticSnapshot,
                "Export sanitized snapshot",
                posture,
                disclosures,
                None,
            ),
        ],
        TrustClass::TrustedLocalActive => vec![
            transfer_action(
                RepresentationActionId::CopyRendered,
                RepresentationClass::Rendered,
                BodyPosture::RenderedView,
                "Copy rendered active preview",
                posture,
                disclosures,
                None,
            ),
            transfer_action(
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationClass::Sanitized,
                BodyPosture::SanitizedStaticSnapshot,
                "Export sanitized snapshot",
                posture,
                disclosures,
                None,
            ),
        ],
        TrustClass::IsolatedRemoteActive => vec![
            transfer_action(
                RepresentationActionId::CopyRendered,
                RepresentationClass::Rendered,
                BodyPosture::RenderedView,
                "Copy rendered isolated preview",
                posture,
                disclosures,
                None,
            ),
            transfer_action(
                RepresentationActionId::ExportSanitizedSnapshot,
                RepresentationClass::Sanitized,
                BodyPosture::SanitizedStaticSnapshot,
                "Export sanitized snapshot",
                posture,
                disclosures,
                None,
            ),
            transfer_action(
                RepresentationActionId::ExportMetadataOnly,
                RepresentationClass::BlockedMetadataOnly,
                BodyPosture::MetadataOnlyEnvelope,
                "Export metadata only",
                posture,
                disclosures,
                None,
            ),
        ],
    }
}

fn mutation_actions_for(
    input: &TrustClassSurfaceInput,
    disclosures: &[String],
    is_degraded: bool,
) -> Vec<TrustClassActionProjection> {
    let mut actions = Vec::new();
    if input.lane == TrustClassLane::PackageInstall {
        actions.push(non_transfer_action(
            "apply_package_action",
            TrustClassActionFamily::Mutate,
            TrustClassActionPosture::RequiresPreview,
            "Apply package action after review",
            disclosures,
            None,
        ));
    }

    let nominal_active = matches!(
        input.nominal_trust_class,
        TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
    );
    if input.lane == TrustClassLane::Preview && nominal_active {
        if is_degraded {
            actions.push(non_transfer_action(
                "run_active_preview",
                TrustClassActionFamily::ActiveOpen,
                TrustClassActionPosture::Blocked,
                "Run active preview",
                disclosures,
                Some(format!(
                    "Active content narrowed to {}.",
                    input.fallback_mode.label()
                )),
            ));
        } else {
            actions.push(non_transfer_action(
                "run_active_preview",
                TrustClassActionFamily::ActiveOpen,
                TrustClassActionPosture::Available,
                "Run active preview in declared sandbox",
                disclosures,
                None,
            ));
        }
    }
    actions
}

fn transfer_action(
    action_id: RepresentationActionId,
    representation_class: RepresentationClass,
    body_posture: BodyPosture,
    label: &str,
    posture: TrustClassActionPosture,
    disclosures: &[String],
    blocked_reason: Option<String>,
) -> TrustClassActionProjection {
    let family = match action_id {
        RepresentationActionId::CopyRaw
        | RepresentationActionId::CopyRendered
        | RepresentationActionId::CopyEscaped => TrustClassActionFamily::Copy,
        RepresentationActionId::ExportSanitizedSnapshot
        | RepresentationActionId::ExportMetadataOnly => TrustClassActionFamily::Export,
    };
    TrustClassActionProjection {
        action_id: action_id.as_str().to_owned(),
        action_family: family,
        action_family_token: family.as_str().to_owned(),
        posture,
        posture_token: posture.as_str().to_owned(),
        representation_class: Some(representation_class.as_str().to_owned()),
        body_posture: Some(body_posture.as_str().to_owned()),
        label: label.to_owned(),
        required_pre_action_disclosures: disclosures.to_vec(),
        blocked_reason,
    }
}

fn non_transfer_action(
    action_id: &str,
    family: TrustClassActionFamily,
    posture: TrustClassActionPosture,
    label: &str,
    disclosures: &[String],
    blocked_reason: Option<String>,
) -> TrustClassActionProjection {
    TrustClassActionProjection {
        action_id: action_id.to_owned(),
        action_family: family,
        action_family_token: family.as_str().to_owned(),
        posture,
        posture_token: posture.as_str().to_owned(),
        representation_class: None,
        body_posture: None,
        label: label.to_owned(),
        required_pre_action_disclosures: disclosures.to_vec(),
        blocked_reason,
    }
}

fn owner_origin_chrome_for(input: &TrustClassSurfaceInput, is_degraded: bool) -> Vec<String> {
    let mut fields = vec![
        "trust_class_badge".to_owned(),
        "owner_identity".to_owned(),
        "origin_identity".to_owned(),
        "representation_label".to_owned(),
    ];
    match input.nominal_trust_class {
        TrustClass::RawText => {}
        TrustClass::SanitizedRich => {
            push_unique(&mut fields, "snapshot_age");
        }
        TrustClass::TrustedLocalActive => {
            push_unique(&mut fields, "capability_summary");
        }
        TrustClass::IsolatedRemoteActive => {
            push_unique(&mut fields, "publisher_or_host");
            push_unique(&mut fields, "connectivity_state");
        }
    }
    if input.lane == TrustClassLane::PackageInstall {
        push_unique(&mut fields, "publisher_or_host");
        push_unique(&mut fields, "suspicious_content_badge");
    }
    if is_degraded || input.fallback_mode.is_active() {
        push_unique(&mut fields, "snapshot_age");
    }
    fields
}

fn pre_action_disclosures_for(input: &TrustClassSurfaceInput, is_degraded: bool) -> Vec<String> {
    let mut fields = vec![
        "trust_class_badge".to_owned(),
        "currently_visible_representation".to_owned(),
        "representation_label".to_owned(),
        "owner_identity".to_owned(),
        "origin_identity".to_owned(),
        "fallback_mode".to_owned(),
    ];
    if input.lane == TrustClassLane::PackageInstall {
        push_unique(&mut fields, "publisher_or_host");
    }
    if matches!(
        input.nominal_trust_class,
        TrustClass::TrustedLocalActive | TrustClass::IsolatedRemoteActive
    ) {
        push_unique(&mut fields, "capability_summary");
    }
    if input.nominal_trust_class == TrustClass::IsolatedRemoteActive
        || input
            .downgrade_trigger_observations
            .iter()
            .any(|trigger| trigger == "connectivity_lost")
    {
        push_unique(&mut fields, "connectivity_state");
    }
    if is_degraded {
        push_unique(&mut fields, "snapshot_age");
    }
    fields
}

fn required_pre_action_fields() -> [&'static str; 3] {
    [
        "trust_class_badge",
        "currently_visible_representation",
        "representation_label",
    ]
}

fn cue_for_trust_class(trust_class: TrustClass) -> &'static str {
    match trust_class {
        TrustClass::RawText => "exact_bytes_visible",
        TrustClass::SanitizedRich => "sanitized_rich_snapshot",
        TrustClass::TrustedLocalActive => "trusted_local_capability_boundary",
        TrustClass::IsolatedRemoteActive => "isolated_origin_boundary",
    }
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_owned());
    }
}
