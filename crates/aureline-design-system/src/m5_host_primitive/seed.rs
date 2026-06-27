//! Canonical seed builder for the M5 host-rendered primitive library.
//!
//! This builder is the single producer of the checked-in primitive fixtures (the library file and
//! one file per primitive) and the release-packet proof. The headless emitter and the inline tests
//! both call it so the in-code primitives, the schema fixtures, and the proof never drift.
//!
//! Each primitive is derived from the canonical
//! [component manifest](crate::m5_component_manifest::seeded_m5_component_manifest_package): its
//! component id, accessibility role, keyboard chords, foundation token references, and mandatory
//! states are copied straight from the manifest, so the primitive is wired to the shared contract
//! by construction rather than restating it. [`super::audit_primitive_manifest_alignment`] proves
//! the wiring holds, and the render plans, appearance binding, and consumer routing add the
//! host-rendered behavior the manifest only describes.

use super::*;

use crate::m5_component_manifest::{
    seeded_m5_component_manifest_package, M5ComponentManifest, M5ComponentManifestPackage,
};
use crate::CanonicalStateClass as State;
use crate::NonColorCueClass as Cue;

/// Stable id of the canonical host-primitive library.
pub const M5_HOST_PRIMITIVE_LIBRARY_ID: &str = "design-system:host-primitive-library:core";

/// Version of the canonical host-primitive library.
pub const M5_HOST_PRIMITIVE_LIBRARY_VERSION: &str = "1.0.0";

/// Mint timestamp pinned by the seed builder.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

const LIBRARY_OWNER_ROLE: &str = "Design system owner";

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_HOST_PRIMITIVE_SCHEMA_REF.to_owned(),
        M5_HOST_PRIMITIVE_DOC_REF.to_owned(),
        M5_HOST_PRIMITIVE_PROOF_REF.to_owned(),
    ]
}

fn primitive_id(kind: M5ComponentKind) -> String {
    format!("design-system:primitive:{}", kind.as_str())
}

fn summary_id(primitive_id: &str) -> String {
    format!(
        "{}{}.summary",
        M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX, primitive_id
    )
}

fn status_id(primitive_id: &str, state: State) -> String {
    format!(
        "{}{}.state.{}",
        M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX,
        primitive_id,
        state.as_str()
    )
}

fn badge_id(surface_class: &str) -> String {
    format!(
        "{}consumer.{}.partial_badge",
        M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX, surface_class
    )
}

/// The non-color cues a render plan carries for a controlled state. Every plan includes label text,
/// so meaning is never carried by color alone; the rest follow the canonical state semantics
/// (blocked names its constraint with a lock/shield glyph, completed shows a check marker, and so
/// on).
fn cues_for(state: State) -> Vec<Cue> {
    let mut cues = vec![Cue::LabelText];
    match state {
        State::Empty => cues.push(Cue::Icon),
        State::Loading => {
            cues.push(Cue::Icon);
            cues.push(Cue::ProgressIndicator);
        }
        State::Pending => {
            cues.push(Cue::Icon);
            cues.push(Cue::ProgressIndicator);
        }
        State::Degraded => {
            cues.push(Cue::Icon);
            cues.push(Cue::Border);
        }
        State::Blocked => {
            cues.push(Cue::Icon);
            cues.push(Cue::Border);
            cues.push(Cue::LockOrShieldGlyph);
        }
        State::Error => {
            cues.push(Cue::Icon);
            cues.push(Cue::Border);
        }
        State::Completed => {
            cues.push(Cue::Icon);
            cues.push(Cue::CheckOrSelectionMarker);
        }
    }
    cues
}

/// Whether a controlled state offers a focusable action. Loading and completed states are passive;
/// the rest expose a next route, recovery, or management action.
fn interactive_for(state: State) -> bool {
    !matches!(state, State::Loading | State::Completed)
}

/// Builds the render plans for a primitive: one per controlled state, covering the full canonical
/// set. Required anatomy parts render in every state; interactive states also render the manifest's
/// optional parts (the actions and decorations the state offers). Mandatory plans mirror the
/// manifest's mandatory states.
fn state_render_plans(
    manifest: &M5ComponentManifest,
    primitive_id: &str,
) -> Vec<M5StateRenderPlan> {
    let required_parts: Vec<String> = manifest
        .anatomy
        .iter()
        .filter(|p| p.required)
        .map(|p| p.part_id.clone())
        .collect();
    let optional_parts: Vec<String> = manifest
        .anatomy
        .iter()
        .filter(|p| !p.required)
        .map(|p| p.part_id.clone())
        .collect();
    let mandatory: BTreeSet<State> = manifest.states.mandatory.iter().copied().collect();

    State::required()
        .iter()
        .copied()
        .map(|state| {
            let interactive = interactive_for(state);
            let mut rendered_parts = required_parts.clone();
            if interactive {
                rendered_parts.extend(optional_parts.iter().cloned());
            }
            M5StateRenderPlan {
                state,
                mandatory: mandatory.contains(&state),
                rendered_parts,
                non_color_cues: cues_for(state),
                status_message_id: status_id(primitive_id, state),
                interactive,
            }
        })
        .collect()
}

/// The appearance binding every host primitive preserves: the full density and motion vocabularies,
/// all four theme classes (including both high-contrast variants), and the focus / keyboard /
/// contrast / motion guarantees.
fn appearance_binding() -> M5AppearanceBinding {
    M5AppearanceBinding {
        density_classes: vec![
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ],
        motion_postures: vec![
            AccessibilityPostureClass::MotionStandard,
            AccessibilityPostureClass::MotionReduced,
            AccessibilityPostureClass::MotionLowMotion,
            AccessibilityPostureClass::MotionPowerSaver,
        ],
        contrast_classes: vec![
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ],
        honors_focus_order: true,
        honors_keyboard_model: true,
        honors_high_contrast: true,
        honors_reduced_motion: true,
    }
}

/// Builds an inherited (full-parity) consumer that renders the host primitive verbatim.
fn inherited(
    surface_class: &str,
    display_name: &str,
    class: M5ConsumerClass,
) -> M5PrimitiveConsumer {
    M5PrimitiveConsumer {
        surface_class: surface_class.to_owned(),
        display_name: display_name.to_owned(),
        consumer_class: class,
        posture: M5ConformancePosture::InheritedHostRendered,
        partial_badge_message_id: None,
    }
}

/// Builds a reduced consumer that renders a partial posture behind an explicit partial badge.
fn reduced(surface_class: &str, display_name: &str, class: M5ConsumerClass) -> M5PrimitiveConsumer {
    M5PrimitiveConsumer {
        surface_class: surface_class.to_owned(),
        display_name: display_name.to_owned(),
        consumer_class: class,
        posture: M5ConformancePosture::ReducedWithPartialBadge,
        partial_badge_message_id: Some(badge_id(surface_class)),
    }
}

/// The M5 family surfaces that route through each primitive kind.
fn consumers_for(kind: M5ComponentKind) -> Vec<M5PrimitiveConsumer> {
    use M5ComponentKind as K;
    use M5ConsumerClass as C;
    match kind {
        K::PlaceholderCard => vec![
            inherited(
                "start_center.empty_workspace",
                "Start center empty workspace",
                C::FirstParty,
            ),
            inherited(
                "search_surface.no_results",
                "Search no-results placeholder",
                C::FirstParty,
            ),
            reduced(
                "extension_view.empty_state",
                "Extension view empty state",
                C::ExtensionContributed,
            ),
        ],
        K::StateBlock => vec![
            inherited(
                "activity_center.state_summary",
                "Activity-center state summary",
                C::FirstParty,
            ),
            inherited(
                "settings_root.managed_state",
                "Managed-setting state block",
                C::FirstParty,
            ),
            reduced(
                "provider_surface.degraded_state",
                "Provider-backed degraded state",
                C::ProviderBacked,
            ),
        ],
        K::ReviewSheet => vec![
            inherited(
                "trust_prompt.batch_review",
                "Trust-prompt batch review",
                C::FirstParty,
            ),
            inherited(
                "dialog_sheet.staged_decision",
                "Staged-decision dialog sheet",
                C::FirstParty,
            ),
            reduced(
                "extension_review.batch_apply",
                "Extension batch-apply review",
                C::ExtensionContributed,
            ),
        ],
        K::JobRow => vec![
            inherited(
                "activity_center.job_row",
                "Activity-center job row",
                C::FirstParty,
            ),
            inherited(
                "notification_envelope.durable_job",
                "Durable-job notification row",
                C::FirstParty,
            ),
            // A provider-backed job row still routes through the host primitive for full parity.
            inherited(
                "provider_surface.remote_job_row",
                "Provider-backed remote job row",
                C::ProviderBacked,
            ),
        ],
        K::BoundaryBar => vec![
            inherited(
                "embedded_boundary.origin_bar",
                "Embedded-surface origin bar",
                C::HostChrome,
            ),
            reduced(
                "extension_host.boundary_bar",
                "Extension-host boundary bar",
                C::ExtensionContributed,
            ),
            reduced(
                "provider_surface.remote_origin_bar",
                "Provider-backed remote origin bar",
                C::ProviderBacked,
            ),
        ],
        K::FormControl => vec![
            inherited(
                "settings_root.field_control",
                "Settings field control",
                C::FirstParty,
            ),
            inherited(
                "dialog_sheet.form_field",
                "Dialog-sheet form field",
                C::FirstParty,
            ),
            reduced(
                "extension_view.form_field",
                "Extension view form field",
                C::ExtensionContributed,
            ),
        ],
        K::DenseCollection => vec![
            inherited(
                "search_surface.result_list",
                "Search result list",
                C::FirstParty,
            ),
            inherited(
                "activity_center.dense_list",
                "Activity-center dense list",
                C::FirstParty,
            ),
            reduced(
                "provider_surface.remote_collection",
                "Provider-backed remote collection",
                C::ProviderBacked,
            ),
        ],
    }
}

fn build_primitive(manifest: &M5ComponentManifest) -> M5HostPrimitive {
    let kind = manifest.component_kind;
    let id = primitive_id(kind);
    M5HostPrimitive {
        component_kind: kind,
        primitive_id: id.clone(),
        component_id: manifest.component_id.clone(),
        display_name: format!("{} (host-rendered)", manifest.display_name),
        accessibility_role: manifest.accessibility.role.clone(),
        keyboard_chords: manifest.keyboard.iter().map(|b| b.keys.clone()).collect(),
        token_references: manifest.token_dependencies.clone(),
        state_render_plans: state_render_plans(manifest, &id),
        appearance: appearance_binding(),
        consumers: consumers_for(kind),
        summary_message_id: summary_id(&id),
    }
}

/// Builds the canonical host-primitive library (version 1.0.0) from the canonical component-manifest
/// package.
///
/// Publishes one host-rendered primitive per [`M5ComponentKind`]. Each primitive inherits its
/// manifest binding (component id, role, keyboard, tokens, mandatory states), declares a render plan
/// per controlled state, preserves the shared appearance vocabulary, and routes a set of M5 family
/// surfaces through the same implementation — first-party surfaces inherit it verbatim, while
/// embedded / extension consumers either inherit it or declare a reduced posture behind a partial
/// badge.
pub fn seeded_m5_host_primitive_library() -> M5HostPrimitiveLibrary {
    seeded_m5_host_primitive_library_from(&seeded_m5_component_manifest_package())
}

/// Builds the canonical host-primitive library from a specific component-manifest package. Exposed
/// for tests that want to drive the builder from a known manifest package.
pub(crate) fn seeded_m5_host_primitive_library_from(
    manifest_package: &M5ComponentManifestPackage,
) -> M5HostPrimitiveLibrary {
    let primitives = M5ComponentKind::ALL
        .iter()
        .map(|kind| {
            let manifest = manifest_package
                .manifest(*kind)
                .expect("component-manifest package publishes one manifest per kind");
            build_primitive(manifest)
        })
        .collect();

    M5HostPrimitiveLibrary {
        record_kind: M5_HOST_PRIMITIVE_LIBRARY_RECORD_KIND.to_owned(),
        schema_version: M5_HOST_PRIMITIVE_SCHEMA_VERSION,
        library_id: M5_HOST_PRIMITIVE_LIBRARY_ID.to_owned(),
        library_version: M5_HOST_PRIMITIVE_LIBRARY_VERSION.to_owned(),
        owner_role: LIBRARY_OWNER_ROLE.to_owned(),
        primitives,
        proof_lane_ref: M5_HOST_PRIMITIVE_PROOF_REF.to_owned(),
        release_packet_ref: M5_HOST_PRIMITIVE_RELEASE_PACKET_REF.to_owned(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_HOST_PRIMITIVE_MESSAGE_ID_PREFIX, M5_HOST_PRIMITIVE_LIBRARY_ID
        ),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}
