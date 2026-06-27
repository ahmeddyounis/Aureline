//! Canonical seed builders for the M5 design-system contract matrix.
//!
//! These builders are the single producer of the checked-in matrix support export, the
//! published dashboard, the Markdown proof, the gallery demo fixtures, and the
//! missing-object / stale-proof / waiver drill fixtures. The headless emitter and the inline
//! tests both call them so the in-code matrix, the artifacts, and the fixtures never drift.
//! The canonical matrix is all-green; the drills mutate one mapping and let the coverage
//! derivation recompute the status, gate, effective claim, and named gaps.

use super::*;

/// Stable matrix id for the canonical (all-conformant) matrix.
pub const M5_DESIGN_SYSTEM_CONTRACT_MATRIX_ID: &str = "m5-design-system-contract:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

// Stable object ids for the shared (single-instance) governed objects.
const FOUNDATION_OBJECT_ID: &str = "design-system:foundation:tokens";
const LAYOUT_OBJECT_ID: &str = "design-system:layout:shell-reference";
const STATE_OBJECT_ID: &str = "design-system:state:canonical-states";
const FIXTURE_OBJECT_ID: &str = "design-system:fixture:component-gallery";
const PROOF_TOKEN_OBJECT_ID: &str = "design-system:proof:token-conformance";
const PROOF_SCREENSHOT_OBJECT_ID: &str = "design-system:proof:component-screenshot-diff";
const PROOF_APPEARANCE_OBJECT_ID: &str = "design-system:proof:appearance-session";

// Shared release / guidance refs.
const RELEASE_PACKET_REF: &str = "evidence:m5-design-system-release-packet";
const MIRROR_PACKET_REF: &str = "evidence:m5-design-system-mirror-offline-packet";
const COMPONENT_EXTENSION_GUIDANCE_REF: &str = "docs/sdk/extension-ui-component-contracts.md";
const FOUNDATION_EXTENSION_GUIDANCE_REF: &str = "docs/sdk/extension-ui-design-system.md";

/// The launch-critical surfaces this lane publishes component contracts and gallery demo
/// fixtures for. Each one becomes a component-contract inventory object, a gallery fixture,
/// and a claimed coverage surface.
const COMPONENT_SURFACES: [LaunchSurfaceClass; 4] = [
    LaunchSurfaceClass::ShellChrome,
    LaunchSurfaceClass::CommandPalette,
    LaunchSurfaceClass::TrustPrompt,
    LaunchSurfaceClass::NotificationEnvelope,
];

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The component-contract object id for a surface.
fn component_object_id(surface: LaunchSurfaceClass) -> String {
    format!("design-system:component:{}", surface.as_str())
}

/// The component-contract gallery fixture path for a surface. The filename uses the surface
/// token verbatim so it matches the fixture the emitter writes and the inline tests load.
fn component_gallery_ref(surface: LaunchSurfaceClass) -> String {
    format!(
        "{}component-contract-{}.json",
        M5_COMPONENT_GALLERY_DIR,
        surface.as_str()
    )
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_REF,
        M5_DESIGN_SYSTEM_DASHBOARD_SCHEMA_REF,
        M5_FOUNDATIONS_SCHEMA_REF,
        M5_COMPONENT_CONTRACT_SCHEMA_REF,
        M5_REFERENCE_LAYOUT_SCHEMA_REF,
        M5_DESIGN_SYSTEM_CONTRACT_DOC_REF,
        M5_DESIGN_SYSTEM_GOVERNANCE_REF,
        M5_DESIGN_SYSTEM_PROOF_REF,
    ])
}

fn conformance_review() -> M5DesignSystemConformanceReview {
    M5DesignSystemConformanceReview {
        every_object_named_with_owner_and_first_consumer: true,
        foundations_object_published: true,
        component_contracts_object_published: true,
        reference_layouts_object_published: true,
        state_semantic_families_object_published: true,
        demo_fixtures_object_published: true,
        proof_packets_object_published: true,
        every_object_binds_canonical_artifact_and_proof_lane: true,
        every_object_binds_release_packet: true,
        component_contracts_declare_anatomy_states_keyboard_a11y_tokens_extension: true,
        every_claimed_surface_maps_required_objects: true,
        unmapped_object_blocks_stable_promotion: true,
        stale_or_missing_proof_auto_narrows_before_stable: true,
        waivers_disclosed_with_scope_owner_and_expiry: true,
        exact_contract_gaps_named: true,
        dashboard_traffic_light_matches_rows: true,
        machine_readable_contract_shared_not_restated: true,
        support_export_carries_no_raw_boundary_material: true,
    }
}

fn consumer_projection() -> M5DesignSystemConsumerProjection {
    M5DesignSystemConsumerProjection {
        shell_consumes_contract: true,
        help_documents_contract: true,
        onboarding_reflects_contract: true,
        presentation_reflects_contract: true,
        extension_sdk_consumes_contract: true,
        release_center_consumes_contract: true,
        qa_gates_on_contract: true,
        support_export_consumes_contract: true,
        stable_claim_matrix_reads_contract: true,
    }
}

fn proof_freshness() -> M5ContractProofFreshnessPolicy {
    M5ContractProofFreshnessPolicy {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DesignSystemReleasePosture {
    M5DesignSystemReleasePosture {
        release_packet_ref: RELEASE_PACKET_REF.to_owned(),
        mirror_offline_packet_ref: MIRROR_PACKET_REF.to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_object: true,
    }
}

/// Builds one governed contract object with the shared release/proof refs filled in.
#[allow(clippy::too_many_arguments)]
fn object(
    object_id: &str,
    object_kind: M5ContractObjectKind,
    display_name: &str,
    owner_role: &str,
    first_consumer: M5DesignSystemConsumer,
    canonical_artifact_ref: &str,
    extension_guidance_ref: &str,
    proof_freshness: M5ContractProofFreshness,
) -> M5DesignSystemContractObject {
    M5DesignSystemContractObject {
        object_id: object_id.to_owned(),
        object_kind,
        display_name: display_name.to_owned(),
        owner_role: owner_role.to_owned(),
        first_consumer,
        canonical_artifact_ref: canonical_artifact_ref.to_owned(),
        schema_ref: object_kind.canonical_schema_ref().to_owned(),
        release_packet_ref: RELEASE_PACKET_REF.to_owned(),
        proof_lane_ref: M5_DESIGN_SYSTEM_PROOF_REF.to_owned(),
        extension_guidance_ref: extension_guidance_ref.to_owned(),
        proof_freshness,
        summary_message_id: format!("{}{}.summary", M5_CONTRACT_MESSAGE_ID_PREFIX, object_id),
    }
}

/// Builds the canonical governed-object inventory (all proof current).
fn build_inventory() -> Vec<M5DesignSystemContractObject> {
    let mut inventory = vec![
        object(
            FOUNDATION_OBJECT_ID,
            M5ContractObjectKind::Foundation,
            "Canonical token foundations",
            "Design system owner",
            M5DesignSystemConsumer::Shell,
            "fixtures/ui/m5-component-gallery/foundations.json",
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
        object(
            LAYOUT_OBJECT_ID,
            M5ContractObjectKind::ReferenceLayout,
            "Shell reference layout",
            "Design system owner",
            M5DesignSystemConsumer::Shell,
            "fixtures/ui/m5-component-gallery/reference-layout.json",
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
        object(
            STATE_OBJECT_ID,
            M5ContractObjectKind::StateSemanticFamily,
            "Canonical state-semantic families",
            "Design system owner",
            M5DesignSystemConsumer::Shell,
            "schemas/design-system/m5-design-system-contract-matrix.schema.json",
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
        object(
            FIXTURE_OBJECT_ID,
            M5ContractObjectKind::DemoFixture,
            "Component gallery demo fixtures",
            "Design QA owner",
            M5DesignSystemConsumer::Qa,
            M5_COMPONENT_GALLERY_DIR,
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
        object(
            PROOF_TOKEN_OBJECT_ID,
            M5ContractObjectKind::ProofPacket,
            "Token-conformance proof packet",
            "Design QA owner",
            M5DesignSystemConsumer::ReleaseCenter,
            "artifacts/release/m5-design-system-proof/support_export.json",
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
        object(
            PROOF_SCREENSHOT_OBJECT_ID,
            M5ContractObjectKind::ProofPacket,
            "Component screenshot-diff proof packet",
            "Design QA owner",
            M5DesignSystemConsumer::ReleaseCenter,
            "artifacts/release/m5-design-system-proof/support_export.json",
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
        object(
            PROOF_APPEARANCE_OBJECT_ID,
            M5ContractObjectKind::ProofPacket,
            "Appearance-session proof packet",
            "Design QA owner",
            M5DesignSystemConsumer::ReleaseCenter,
            "artifacts/release/m5-design-system-proof/support_export.json",
            FOUNDATION_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ),
    ];
    for surface in COMPONENT_SURFACES {
        inventory.push(object(
            &component_object_id(surface),
            M5ContractObjectKind::ComponentContract,
            &format!("{} component contract", component_display_name(surface)),
            "Component owner",
            M5DesignSystemConsumer::Shell,
            &component_gallery_ref(surface),
            COMPONENT_EXTENSION_GUIDANCE_REF,
            M5ContractProofFreshness::Current,
        ));
    }
    inventory
}

/// The objects every claimed surface must map: the shared foundation, layout, state family,
/// demo-fixture, and token-conformance proof, plus the surface's own component contract.
fn required_objects(surface: LaunchSurfaceClass) -> Vec<M5RequiredContractObject> {
    vec![
        M5RequiredContractObject {
            object_id: FOUNDATION_OBJECT_ID.to_owned(),
            object_kind: M5ContractObjectKind::Foundation,
        },
        M5RequiredContractObject {
            object_id: component_object_id(surface),
            object_kind: M5ContractObjectKind::ComponentContract,
        },
        M5RequiredContractObject {
            object_id: LAYOUT_OBJECT_ID.to_owned(),
            object_kind: M5ContractObjectKind::ReferenceLayout,
        },
        M5RequiredContractObject {
            object_id: STATE_OBJECT_ID.to_owned(),
            object_kind: M5ContractObjectKind::StateSemanticFamily,
        },
        M5RequiredContractObject {
            object_id: FIXTURE_OBJECT_ID.to_owned(),
            object_kind: M5ContractObjectKind::DemoFixture,
        },
        M5RequiredContractObject {
            object_id: PROOF_TOKEN_OBJECT_ID.to_owned(),
            object_kind: M5ContractObjectKind::ProofPacket,
        },
    ]
}

/// Builds one claimed-surface coverage row and reconciles its derived fields.
fn surface_row(
    surface: LaunchSurfaceClass,
    inventory: &[M5DesignSystemContractObject],
) -> M5SurfaceContractCoverage {
    let surface_id = format!("design-system-surface:{}", surface.as_str());
    let mut row = M5SurfaceContractCoverage {
        surface_id: surface_id.clone(),
        surface_class: surface,
        surface_label: format!("{} surface", component_display_name(surface)),
        owner_role: "Component owner".to_owned(),
        claimed_class: M5DesignSystemClaimClass::Stable,
        effective_class: M5DesignSystemClaimClass::Stable,
        coverage_status: M5CoverageStatus::Conformant,
        signal: M5CoverageSignal::Green,
        required_objects: required_objects(surface),
        waivers: Vec::new(),
        gate_decision: M5CoverageGateDecision::CertifiedPromote,
        gaps: Vec::new(),
        consumer_surfaces: vec![
            M5DesignSystemConsumer::Shell,
            M5DesignSystemConsumer::Help,
            M5DesignSystemConsumer::ReleaseCenter,
            M5DesignSystemConsumer::SupportExport,
            M5DesignSystemConsumer::StableClaimMatrix,
        ],
        source_contract_refs: source_contract_refs(),
        status_message_id: format!("{}{}.status", M5_CONTRACT_MESSAGE_ID_PREFIX, surface_id),
        gate_message_id: format!("{}{}.gate", M5_CONTRACT_MESSAGE_ID_PREFIX, surface_id),
    };
    row.recompute_derived(inventory);
    row
}

/// Builds the canonical claimed-surface coverage rows.
fn build_surfaces(inventory: &[M5DesignSystemContractObject]) -> Vec<M5SurfaceContractCoverage> {
    COMPONENT_SURFACES
        .iter()
        .map(|&surface| surface_row(surface, inventory))
        .collect()
}

/// Builds the packet-level release gate from the per-surface coverage gates.
fn aggregate_release_gate(surfaces: &[M5SurfaceContractCoverage]) -> M5DesignSystemReleaseGate {
    let collect = |predicate: &dyn Fn(&M5SurfaceContractCoverage) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = surfaces
            .iter()
            .filter(|s| predicate(s))
            .map(|s| s.surface_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = collect(&|s| s.is_blocked());
    M5DesignSystemReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_surface_ids: blocked,
        auto_narrowed_surface_ids: collect(&|s| s.is_auto_narrowed()),
        conformant_surface_ids: collect(&|s| s.is_conformant()),
        waived_surface_ids: collect(&|s| !s.waivers.is_empty()),
        gate_message_id: format!("{}release_gate", M5_CONTRACT_MESSAGE_ID_PREFIX),
    }
}

/// Assembles a matrix packet from an inventory and a set of (already reconciled) surfaces.
fn build_matrix(
    matrix_id: &str,
    inventory: Vec<M5DesignSystemContractObject>,
    surfaces: Vec<M5SurfaceContractCoverage>,
) -> M5DesignSystemContractMatrix {
    let release_gate = aggregate_release_gate(&surfaces);
    M5DesignSystemContractMatrix::new(M5DesignSystemContractMatrixInput {
        matrix_id: matrix_id.to_owned(),
        report_label: "M5 Design-System Contract Matrix".to_owned(),
        contract_objects: inventory,
        surfaces,
        vocabulary_set: M5DesignSystemContractVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        release_gate,
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Builds the canonical all-conformant design-system contract matrix.
///
/// This is the single producer of the checked-in support export and dashboard: every
/// governed object is published with current proof, and every claimed surface maps a full set
/// of contract objects, so the release gate certifies every surface for Stable promotion.
pub fn seeded_m5_design_system_contract_matrix() -> M5DesignSystemContractMatrix {
    let inventory = build_inventory();
    let surfaces = build_surfaces(&inventory);
    build_matrix(M5_DESIGN_SYSTEM_CONTRACT_MATRIX_ID, inventory, surfaces)
}

/// Contract matrix where one claimed surface requires a contract object that is not published
/// in the inventory (an unmapped object), so the surface is uncovered and blocked from Stable
/// promotion — and named, not hidden, in the release packet.
pub fn seeded_m5_design_system_contract_matrix_missing_object() -> M5DesignSystemContractMatrix {
    let inventory = build_inventory();
    let mut surfaces = build_surfaces(&inventory);
    // Shell chrome additionally requires a diff-viewer component contract that the inventory
    // does not publish: a claimed surface that lacks a mapped contract object.
    let shell = surface_index(&surfaces, LaunchSurfaceClass::ShellChrome);
    surfaces[shell]
        .required_objects
        .push(M5RequiredContractObject {
            object_id: "design-system:component:diff-viewer".to_owned(),
            object_kind: M5ContractObjectKind::ComponentContract,
        });
    surfaces[shell].recompute_derived(&inventory);
    build_matrix(
        "m5-design-system-contract:drill:missing-object",
        inventory,
        surfaces,
    )
}

/// Contract matrix where one claimed surface's component-contract proof has fallen out of its
/// freshness SLO, so the surface is retest-pending and auto-narrows to Beta before Stable
/// promotion. Stale proof narrows but never blocks.
pub fn seeded_m5_design_system_contract_matrix_stale_proof_retest_pending(
) -> M5DesignSystemContractMatrix {
    let mut inventory = build_inventory();
    let object_id = component_object_id(LaunchSurfaceClass::ShellChrome);
    if let Some(object) = inventory.iter_mut().find(|o| o.object_id == object_id) {
        object.proof_freshness = M5ContractProofFreshness::Stale;
    }
    let surfaces = build_surfaces(&inventory);
    build_matrix(
        "m5-design-system-contract:drill:stale-proof-retest-pending",
        inventory,
        surfaces,
    )
}

/// Contract matrix where a surface's unmapped-object gap is accepted under an active,
/// disclosed waiver, so the surface ships auto-narrowed to its waived claim while its true
/// status stays uncovered (red) and the gap is named as waived.
pub fn seeded_m5_design_system_contract_matrix_waived_narrowed() -> M5DesignSystemContractMatrix {
    let inventory = build_inventory();
    let mut surfaces = build_surfaces(&inventory);
    let shell = surface_index(&surfaces, LaunchSurfaceClass::ShellChrome);
    let shell_surface_id = surfaces[shell].surface_id.clone();
    surfaces[shell]
        .required_objects
        .push(M5RequiredContractObject {
            object_id: "design-system:component:diff-viewer".to_owned(),
            object_kind: M5ContractObjectKind::ComponentContract,
        });
    surfaces[shell].waivers.push(M5CoverageWaiver {
        waiver_id: "waiver:shell-chrome-diff-viewer".to_owned(),
        object_id: "design-system:component:diff-viewer".to_owned(),
        reason_message_id: format!(
            "{}{}.waiver.diff_viewer",
            M5_CONTRACT_MESSAGE_ID_PREFIX, shell_surface_id
        ),
        owner_role: "Component owner".to_owned(),
        expires_at: "2026-09-26T00:00:00Z".to_owned(),
        narrowed_to: M5DesignSystemClaimClass::Preview,
    });
    surfaces[shell].recompute_derived(&inventory);
    build_matrix(
        "m5-design-system-contract:drill:waived-narrowed",
        inventory,
        surfaces,
    )
}

/// Finds the index of the first coverage row for the given surface class.
fn surface_index(surfaces: &[M5SurfaceContractCoverage], surface: LaunchSurfaceClass) -> usize {
    surfaces
        .iter()
        .position(|s| s.surface_class == surface)
        .expect("surface class present")
}

// ---------------------------------------------------------------------------
// Canonical-artifact (gallery) seeds.
// ---------------------------------------------------------------------------

/// Human-readable component name for a surface class.
fn component_display_name(surface: LaunchSurfaceClass) -> &'static str {
    match surface {
        LaunchSurfaceClass::ShellChrome => "Shell chrome",
        LaunchSurfaceClass::StartCenter => "Start center",
        LaunchSurfaceClass::CommandPalette => "Command palette",
        LaunchSurfaceClass::SearchSurface => "Search surface",
        LaunchSurfaceClass::DialogSheet => "Dialog sheet",
        LaunchSurfaceClass::TrustPrompt => "Trust prompt",
        LaunchSurfaceClass::NotificationEnvelope => "Notification envelope",
        LaunchSurfaceClass::HelpAboutRow => "Help and About row",
        LaunchSurfaceClass::SettingsRoot => "Settings root",
        LaunchSurfaceClass::ActivityCenterRow => "Activity center row",
    }
}

/// Builds the canonical foundations gallery artifact.
pub fn seeded_m5_foundations_artifact() -> M5FoundationsArtifact {
    M5FoundationsArtifact {
        record_kind: M5_FOUNDATIONS_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION,
        foundations_id: FOUNDATION_OBJECT_ID.to_owned(),
        owner_role: "Design system owner".to_owned(),
        token_families: vec![
            M5TokenFamily {
                family_id: "color.surface".to_owned(),
                display_name: "Surface colors".to_owned(),
                semantic_token_refs: strings(&[
                    "color.surface.shell",
                    "color.surface.raised",
                    "color.surface.sunken",
                ]),
            },
            M5TokenFamily {
                family_id: "color.text".to_owned(),
                display_name: "Text colors".to_owned(),
                semantic_token_refs: strings(&[
                    "color.text.primary",
                    "color.text.secondary",
                    "color.text.inverse",
                ]),
            },
            M5TokenFamily {
                family_id: "space".to_owned(),
                display_name: "Spacing scale".to_owned(),
                semantic_token_refs: strings(&["space.100", "space.200", "space.400"]),
            },
        ],
        theme_classes: theme_class_tokens(),
        density_classes: density_class_tokens(),
        motion_postures: motion_posture_tokens(),
        proof_lane_ref: M5_DESIGN_SYSTEM_PROOF_REF.to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_CONTRACT_MESSAGE_ID_PREFIX, FOUNDATION_OBJECT_ID
        ),
    }
}

/// Builds the canonical reference-layout gallery artifact.
pub fn seeded_m5_reference_layout_artifact() -> M5ReferenceLayoutArtifact {
    M5ReferenceLayoutArtifact {
        record_kind: M5_REFERENCE_LAYOUT_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION,
        layout_id: LAYOUT_OBJECT_ID.to_owned(),
        display_name: "Shell reference layout".to_owned(),
        owner_role: "Design system owner".to_owned(),
        shell_slots: vec![
            M5ShellSlot {
                slot_id: "primary-rail".to_owned(),
                role: "navigation".to_owned(),
                placeholder_behavior: "Reserve rail width and show a quiet skeleton until \
                    navigation resolves."
                    .to_owned(),
            },
            M5ShellSlot {
                slot_id: "work-surface".to_owned(),
                role: "main".to_owned(),
                placeholder_behavior: "Show the empty-route guidance, never a blank canvas."
                    .to_owned(),
            },
            M5ShellSlot {
                slot_id: "status-strip".to_owned(),
                role: "contentinfo".to_owned(),
                placeholder_behavior: "Keep the strip present and announce route/state \
                    transitions."
                    .to_owned(),
            },
        ],
        placeholder_policy: M5PlaceholderPolicy {
            empty_slot_rule: "An empty slot names the useful next route; it never collapses \
                silently."
                .to_owned(),
            loading_slot_rule: "A loading slot reserves layout and discloses progress without \
                shifting neighbors."
                .to_owned(),
        },
        summary_message_id: format!(
            "{}{}.summary",
            M5_CONTRACT_MESSAGE_ID_PREFIX, LAYOUT_OBJECT_ID
        ),
    }
}

/// Builds one canonical component-contract gallery artifact for a surface.
fn component_contract_artifact(surface: LaunchSurfaceClass) -> M5ComponentContractArtifact {
    let component_id = component_object_id(surface);
    M5ComponentContractArtifact {
        record_kind: M5_COMPONENT_CONTRACT_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION,
        component_id: component_id.clone(),
        display_name: component_display_name(surface).to_owned(),
        surface_class: surface,
        owner_role: "Component owner".to_owned(),
        anatomy: vec![
            M5AnatomyPart {
                part_id: "root".to_owned(),
                role: "container".to_owned(),
            },
            M5AnatomyPart {
                part_id: "label".to_owned(),
                role: "label".to_owned(),
            },
            M5AnatomyPart {
                part_id: "action".to_owned(),
                role: "control".to_owned(),
            },
        ],
        states: vec![
            CanonicalStateClass::Empty,
            CanonicalStateClass::Loading,
            CanonicalStateClass::Pending,
            CanonicalStateClass::Blocked,
            CanonicalStateClass::Completed,
        ],
        keyboard_model: vec![
            M5KeyBinding {
                keys: "Tab".to_owned(),
                action: "Move focus to the next interactive part.".to_owned(),
            },
            M5KeyBinding {
                keys: "Enter".to_owned(),
                action: "Activate the primary action.".to_owned(),
            },
            M5KeyBinding {
                keys: "Escape".to_owned(),
                action: "Dismiss or return focus to the invoking object.".to_owned(),
            },
        ],
        accessibility: M5ComponentAccessibility {
            role: "group".to_owned(),
            screen_reader_label_rule: "The component announces its name, state, and the action \
                it offers."
                .to_owned(),
            focus_order_rule: "Focus follows visual order; focus returns to the invoker on \
                dismissal."
                .to_owned(),
        },
        token_dependencies: strings(&["color.surface.shell", "color.text.primary", "space.200"]),
        extension_guidance_ref: COMPONENT_EXTENSION_GUIDANCE_REF.to_owned(),
        summary_message_id: format!("{}{}.summary", M5_CONTRACT_MESSAGE_ID_PREFIX, component_id),
    }
}

/// Builds the canonical component-contract gallery (one artifact per launch-critical surface).
pub fn seeded_m5_component_contract_gallery() -> Vec<M5ComponentContractArtifact> {
    COMPONENT_SURFACES
        .iter()
        .map(|&surface| component_contract_artifact(surface))
        .collect()
}
