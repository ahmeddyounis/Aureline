//! Canonical seed builder for the M5 design-system component-manifest package.
//!
//! This builder is the single producer of the checked-in manifest fixtures (the package file and
//! one file per manifest) and the release-packet proof. The headless emitter and the inline tests
//! both call it so the in-code manifests, the schema fixtures, and the proof never drift. The
//! mandatory / optional states are drawn from the same [`CanonicalStateClass`] vocabulary the rest
//! of the design system uses, and the token dependencies name entries the foundation package
//! publishes, so the manifests read from one shared source rather than feature-local wiring.

use super::*;

use crate::CanonicalStateClass as State;

/// Stable id of the canonical component-manifest package.
pub const M5_COMPONENT_MANIFEST_PACKAGE_ID: &str = "design-system:component-manifest-package:core";

/// Version of the canonical component-manifest package.
pub const M5_COMPONENT_MANIFEST_PACKAGE_VERSION: &str = "1.0.0";

/// Mint timestamp pinned by the seed builder.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

const PACKAGE_OWNER_ROLE: &str = "Design system owner";
const COMPONENT_OWNER_ROLE: &str = "Component owner";

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_COMPONENT_MANIFEST_SCHEMA_REF.to_owned(),
        M5_COMPONENT_MANIFEST_DOC_REF.to_owned(),
        M5_COMPONENT_MANIFEST_PROOF_REF.to_owned(),
    ]
}

fn lifecycle(state: M5LifecycleState) -> M5ComponentLifecycle {
    M5ComponentLifecycle {
        owner_role: COMPONENT_OWNER_ROLE.to_owned(),
        lifecycle_state: state,
        manifest_version: 1,
        introduced_in_package_version: M5_COMPONENT_MANIFEST_PACKAGE_VERSION.to_owned(),
    }
}

fn extension_guidance(rules: &[&str]) -> M5ExtensionGuidance {
    M5ExtensionGuidance {
        guidance_ref: M5_COMPONENT_EXTENSION_GUIDANCE_REF.to_owned(),
        consumption_rules: rules.iter().map(|r| (*r).to_owned()).collect(),
    }
}

fn summary_id(component_id: &str) -> String {
    format!("{}{}.summary", M5_COMPONENT_MESSAGE_ID_PREFIX, component_id)
}

fn component_id(kind: M5ComponentKind) -> String {
    format!("design-system:component:{}", kind.as_str())
}

fn labels(id: &str, specs: &[(&str, &str)]) -> Vec<M5ComponentLabel> {
    specs
        .iter()
        .map(|(label_id, purpose)| M5ComponentLabel::new(id, label_id, purpose))
        .collect()
}

fn commands(id: &str, specs: &[(&str, &str)]) -> Vec<M5ComponentCommand> {
    specs
        .iter()
        .map(|(command_id, keys)| M5ComponentCommand::new(id, command_id, keys))
        .collect()
}

fn tokens(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|t| (*t).to_owned()).collect()
}

// --- Placeholder card -------------------------------------------------------

fn placeholder_card() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::PlaceholderCard);
    M5ComponentManifest {
        component_kind: M5ComponentKind::PlaceholderCard,
        component_id: id.clone(),
        display_name: "Placeholder card".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Stable),
        anatomy: vec![
            M5AnatomyPart::required("root", "container"),
            M5AnatomyPart::required("title", "heading"),
            M5AnatomyPart::required("description", "supporting text"),
            M5AnatomyPart::optional("illustration", "decorative graphic"),
            M5AnatomyPart::optional("primary_action", "control"),
        ],
        states: M5ComponentStates::new(
            &[State::Empty, State::Loading],
            &[
                State::Pending,
                State::Degraded,
                State::Blocked,
                State::Error,
                State::Completed,
            ],
        ),
        labels: labels(
            &id,
            &[
                ("title", "Names what is absent and why the surface is ready"),
                ("description", "Explains the next useful route the user can take"),
                ("primary_action", "Names the recommended first action"),
            ],
        ),
        commands: commands(
            &id,
            &[("invoke_primary", "Enter"), ("dismiss", "Escape")],
        ),
        keyboard: vec![
            M5KeyBinding::new("Tab", "Move focus to the next interactive part."),
            M5KeyBinding::new("Enter", "Invoke the primary action when one is offered."),
            M5KeyBinding::new("Escape", "Dismiss the card and return focus to the invoker."),
        ],
        accessibility: M5AccessibilityContract {
            role: "group".to_owned(),
            screen_reader_label_rule:
                "Announces the title, then the description, then the primary action when present."
                    .to_owned(),
            focus_order_rule:
                "The illustration is decorative and skipped; focus reaches the primary action last."
                    .to_owned(),
            notes: vec![
                "The illustration carries no semantic meaning and is hidden from assistive tech."
                    .to_owned(),
                "An empty placeholder still exposes a reachable next action.".to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.raised",
            "color.text.primary",
            "color.text.secondary",
            "typography.heading",
            "typography.body",
            "space.4",
        ]),
        extension_guidance: extension_guidance(&[
            "Reuse the published title / description / primary-action label slots; do not invent parallel ones.",
            "Render the empty and loading states; both are mandatory for this component.",
            "Resolve surfaces from the named foundation token references rather than literal values.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

// --- State block ------------------------------------------------------------

fn state_block() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::StateBlock);
    M5ComponentManifest {
        component_kind: M5ComponentKind::StateBlock,
        component_id: id.clone(),
        display_name: "State block".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Stable),
        anatomy: vec![
            M5AnatomyPart::required("root", "container"),
            M5AnatomyPart::required("status_icon", "state indicator"),
            M5AnatomyPart::required("title", "heading"),
            M5AnatomyPart::required("detail", "supporting text"),
            M5AnatomyPart::optional("recovery_action", "control"),
        ],
        states: M5ComponentStates::new(
            &[
                State::Pending,
                State::Degraded,
                State::Blocked,
                State::Error,
                State::Completed,
            ],
            &[State::Empty, State::Loading],
        ),
        labels: labels(
            &id,
            &[
                ("title", "Names the current controlled state"),
                ("detail", "Explains what still works or what is required"),
                ("recovery_action", "Names the retry, repair, or unblock action"),
            ],
        ),
        commands: commands(
            &id,
            &[("recover", "Enter"), ("show_diagnostics", "Ctrl+D")],
        ),
        keyboard: vec![
            M5KeyBinding::new("Tab", "Move focus to the recovery action when present."),
            M5KeyBinding::new("Enter", "Invoke the recovery action."),
            M5KeyBinding::new("Ctrl+D", "Open diagnostics for the failing or degraded state."),
        ],
        accessibility: M5AccessibilityContract {
            role: "status".to_owned(),
            screen_reader_label_rule:
                "Announces the state, then the title, then the detail; error and blocked states use an assertive live region."
                    .to_owned(),
            focus_order_rule:
                "Focus reaches the recovery action after the detail text is announced.".to_owned(),
            notes: vec![
                "Blocked states never present a spinner; they name the policy, trust, or capability that blocks them."
                    .to_owned(),
                "The status icon's meaning is mirrored in text, never carried by color alone."
                    .to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.raised",
            "color.text.primary",
            "color.text.secondary",
            "color.state.success",
            "icon.size.md",
            "typography.heading",
            "typography.body",
            "space.4",
        ]),
        extension_guidance: extension_guidance(&[
            "Map every controlled state your surface can enter onto this block; the pending, degraded, blocked, error, and completed states are mandatory.",
            "Never render a spinner in the blocked state; name what blocks the action instead.",
            "Mirror the status icon in the title text so meaning is not carried by color alone.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

// --- Review sheet -----------------------------------------------------------

fn review_sheet() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::ReviewSheet);
    M5ComponentManifest {
        component_kind: M5ComponentKind::ReviewSheet,
        component_id: id.clone(),
        display_name: "Review sheet".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Preview),
        anatomy: vec![
            M5AnatomyPart::required("root", "dialog container"),
            M5AnatomyPart::required("header", "heading"),
            M5AnatomyPart::required("summary", "decision summary"),
            M5AnatomyPart::required("body", "scrollable detail"),
            M5AnatomyPart::required("action_bar", "control group"),
        ],
        states: M5ComponentStates::new(
            &[State::Pending, State::Blocked, State::Completed],
            &[State::Empty, State::Loading, State::Degraded, State::Error],
        ),
        labels: labels(
            &id,
            &[
                ("header", "Names the decision under review"),
                ("summary", "Summarizes what will change on approval"),
                ("approve", "Names the approve action"),
                ("reject", "Names the reject action"),
            ],
        ),
        commands: commands(
            &id,
            &[
                ("approve", "Ctrl+Enter"),
                ("reject", "Ctrl+Backspace"),
                ("dismiss", "Escape"),
            ],
        ),
        keyboard: vec![
            M5KeyBinding::new("Tab", "Cycle focus within the sheet; focus is trapped while open."),
            M5KeyBinding::new("Ctrl+Enter", "Approve the staged decision."),
            M5KeyBinding::new("Ctrl+Backspace", "Reject the staged decision."),
            M5KeyBinding::new("Escape", "Dismiss without deciding and return focus to the invoker."),
        ],
        accessibility: M5AccessibilityContract {
            role: "dialog".to_owned(),
            screen_reader_label_rule:
                "Announces the header and summary on open; the action bar names the consequence of each control."
                    .to_owned(),
            focus_order_rule:
                "Focus moves to the summary on open, is trapped within the sheet, and returns to the invoker on close."
                    .to_owned(),
            notes: vec![
                "The sheet is modal; background content is inert while it is open.".to_owned(),
                "Approve and reject are distinct commands and never share a single ambiguous control."
                    .to_owned(),
                "The blocked state explains why the decision cannot be made and offers no approve action."
                    .to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.shell",
            "color.surface.raised",
            "color.text.primary",
            "color.text.secondary",
            "typography.heading",
            "typography.body",
            "space.4",
            "space.6",
        ]),
        extension_guidance: extension_guidance(&[
            "Stage decisions in the pending state and record the outcome in the completed state; both are mandatory.",
            "Keep approve and reject as separate commands; do not collapse them into one toggle.",
            "Trap focus while the sheet is open and restore it to the invoker on close.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

// --- Job row ----------------------------------------------------------------

fn job_row() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::JobRow);
    M5ComponentManifest {
        component_kind: M5ComponentKind::JobRow,
        component_id: id.clone(),
        display_name: "Job row".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Stable),
        anatomy: vec![
            M5AnatomyPart::required("root", "row container"),
            M5AnatomyPart::required("status_icon", "state indicator"),
            M5AnatomyPart::required("title", "primary label"),
            M5AnatomyPart::required("meta", "secondary label"),
            M5AnatomyPart::optional("progress", "progress indicator"),
            M5AnatomyPart::optional("row_action", "control"),
        ],
        states: M5ComponentStates::new(
            &[State::Pending, State::Error, State::Completed],
            &[State::Empty, State::Loading, State::Degraded, State::Blocked],
        ),
        labels: labels(
            &id,
            &[
                ("title", "Names the job"),
                ("meta", "Names the job's source, time, or owner"),
                ("status", "Names the current job state"),
            ],
        ),
        commands: commands(
            &id,
            &[("open", "Enter"), ("cancel", "Ctrl+."), ("retry", "Ctrl+R")],
        ),
        keyboard: vec![
            M5KeyBinding::new("Up / Down", "Move row selection within the collection."),
            M5KeyBinding::new("Enter", "Open the job's detail surface."),
            M5KeyBinding::new("Ctrl+.", "Cancel a running job."),
            M5KeyBinding::new("Ctrl+R", "Retry a failed job."),
        ],
        accessibility: M5AccessibilityContract {
            role: "row".to_owned(),
            screen_reader_label_rule:
                "Announces the title, then the state, then the meta; progress is announced as a percentage when present."
                    .to_owned(),
            focus_order_rule:
                "The row is one focus stop; row actions are reached via the roving collection model, not separate tab stops."
                    .to_owned(),
            notes: vec![
                "The status icon's meaning is also present in text for assistive tech.".to_owned(),
                "Progress updates are throttled so they do not flood the announcement channel."
                    .to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.shell",
            "color.text.primary",
            "color.text.secondary",
            "color.state.success",
            "icon.size.sm",
            "typography.body",
            "space.2",
        ]),
        extension_guidance: extension_guidance(&[
            "Render the pending, error, and completed states; they are mandatory for a job row.",
            "Expose row actions through the collection's roving model, not as independent tab stops.",
            "Throttle progress announcements so dense lists do not flood assistive tech.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

// --- Boundary bar -----------------------------------------------------------

fn boundary_bar() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::BoundaryBar);
    M5ComponentManifest {
        component_kind: M5ComponentKind::BoundaryBar,
        component_id: id.clone(),
        display_name: "Boundary bar".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Preview),
        anatomy: vec![
            M5AnatomyPart::required("root", "banner container"),
            M5AnatomyPart::required("boundary_icon", "trust indicator"),
            M5AnatomyPart::required("route_label", "origin label"),
            M5AnatomyPart::required("capability_summary", "capability label"),
            M5AnatomyPart::optional("manage_action", "control"),
        ],
        states: M5ComponentStates::new(
            &[State::Degraded, State::Blocked],
            &[
                State::Empty,
                State::Loading,
                State::Pending,
                State::Error,
                State::Completed,
            ],
        ),
        labels: labels(
            &id,
            &[
                ("route_label", "Names the route or origin of the embedded surface"),
                ("capability_summary", "Names which capabilities are available across the boundary"),
                ("manage_action", "Names the action that manages the boundary"),
            ],
        ),
        commands: commands(
            &id,
            &[("manage", "Enter"), ("learn_more", "Ctrl+I")],
        ),
        keyboard: vec![
            M5KeyBinding::new("Tab", "Move focus to the manage action when present."),
            M5KeyBinding::new("Enter", "Open the boundary management surface."),
            M5KeyBinding::new("Ctrl+I", "Explain what the boundary restricts and why."),
        ],
        accessibility: M5AccessibilityContract {
            role: "region".to_owned(),
            screen_reader_label_rule:
                "Announces the route, then the capability summary, then any degraded or blocked reason."
                    .to_owned(),
            focus_order_rule:
                "The bar precedes the surface it guards in focus order; the manage action is its last stop."
                    .to_owned(),
            notes: vec![
                "The trust indicator never relies on color alone to convey the boundary state."
                    .to_owned(),
                "A blocked boundary names the policy or capability gap rather than failing silently."
                    .to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.shell",
            "color.text.primary",
            "color.text.secondary",
            "icon.metaphor.lock",
            "icon.size.sm",
            "typography.body",
            "space.2",
        ]),
        extension_guidance: extension_guidance(&[
            "Render the degraded and blocked states; they are mandatory for a boundary bar.",
            "Name the route and capability gap in text; never convey trust state by color alone.",
            "Place the bar before the surface it guards in both visual and focus order.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

// --- Form control -----------------------------------------------------------

fn form_control() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::FormControl);
    M5ComponentManifest {
        component_kind: M5ComponentKind::FormControl,
        component_id: id.clone(),
        display_name: "Form control".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Stable),
        anatomy: vec![
            M5AnatomyPart::required("root", "field container"),
            M5AnatomyPart::required("label", "field label"),
            M5AnatomyPart::required("input", "control"),
            M5AnatomyPart::optional("hint", "supporting text"),
            M5AnatomyPart::optional("error_text", "validation message"),
        ],
        states: M5ComponentStates::new(
            &[State::Empty, State::Error, State::Completed],
            &[State::Loading, State::Pending, State::Degraded, State::Blocked],
        ),
        labels: labels(
            &id,
            &[
                ("label", "Names the field"),
                ("hint", "Explains the expected input"),
                ("error_text", "States how to correct an invalid value"),
            ],
        ),
        commands: commands(
            &id,
            &[("submit", "Enter"), ("reset", "Ctrl+Z")],
        ),
        keyboard: vec![
            M5KeyBinding::new("Tab", "Move focus to the input, then onward out of the field."),
            M5KeyBinding::new("Enter", "Submit the field's owning form when valid."),
            M5KeyBinding::new("Ctrl+Z", "Reset the field to its last committed value."),
        ],
        accessibility: M5AccessibilityContract {
            role: "group".to_owned(),
            screen_reader_label_rule:
                "The input is programmatically labelled by the field label, described by the hint, and linked to the error text when invalid."
                    .to_owned(),
            focus_order_rule:
                "Focus reaches the label-associated input directly; the hint and error text are announced as descriptions, not separate stops."
                    .to_owned(),
            notes: vec![
                "An invalid field marks the input invalid and links the error text as its description."
                    .to_owned(),
                "The error state names how to fix the value, never just that it is wrong.".to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.raised",
            "color.text.primary",
            "color.text.secondary",
            "typography.body",
            "space.2",
            "space.4",
        ]),
        extension_guidance: extension_guidance(&[
            "Render the empty, error, and completed states; they are mandatory for a form control.",
            "Associate the label, hint, and error text programmatically with the input.",
            "Name the correction in the error state; do not report only that the value is invalid.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

// --- Dense collection -------------------------------------------------------

fn dense_collection() -> M5ComponentManifest {
    let id = component_id(M5ComponentKind::DenseCollection);
    M5ComponentManifest {
        component_kind: M5ComponentKind::DenseCollection,
        component_id: id.clone(),
        display_name: "Dense collection".to_owned(),
        lifecycle: lifecycle(M5LifecycleState::Experimental),
        anatomy: vec![
            M5AnatomyPart::required("root", "collection container"),
            M5AnatomyPart::required("viewport", "virtualized scroll region"),
            M5AnatomyPart::required("item", "collection item"),
            M5AnatomyPart::optional("header", "column or section header"),
            M5AnatomyPart::optional("empty_slot", "empty-state surface"),
        ],
        states: M5ComponentStates::new(
            &[State::Empty, State::Loading, State::Completed],
            &[State::Pending, State::Degraded, State::Blocked, State::Error],
        ),
        labels: labels(
            &id,
            &[
                ("collection", "Names the collection and its item count"),
                ("item", "Names an individual item"),
                ("empty_slot", "Explains why the collection is empty"),
            ],
        ),
        commands: commands(
            &id,
            &[
                ("activate", "Enter"),
                ("select", "Space"),
                ("expand", "Right"),
                ("collapse", "Left"),
            ],
        ),
        keyboard: vec![
            M5KeyBinding::new("Up / Down", "Move the focused item within the collection."),
            M5KeyBinding::new("Home / End", "Jump to the first or last item."),
            M5KeyBinding::new("Enter", "Activate the focused item."),
            M5KeyBinding::new("Space", "Toggle selection of the focused item."),
            M5KeyBinding::new("Right / Left", "Expand or collapse a tree item."),
        ],
        accessibility: M5AccessibilityContract {
            role: "grid".to_owned(),
            screen_reader_label_rule:
                "Announces the collection name and total count; each item announces its position and selection state."
                    .to_owned(),
            focus_order_rule:
                "The collection is a single tab stop with a roving focus model; arrow keys move within it without leaving the page tab order."
                    .to_owned(),
            notes: vec![
                "Virtualized items expose their true position and total count so assistive tech is not misled by the rendered window."
                    .to_owned(),
                "The empty state remains keyboard-reachable and names why the collection is empty."
                    .to_owned(),
            ],
        },
        token_dependencies: tokens(&[
            "color.surface.shell",
            "color.text.primary",
            "color.text.secondary",
            "typography.body",
            "typography.code",
            "space.2",
        ]),
        extension_guidance: extension_guidance(&[
            "Render the empty, loading, and completed states; they are mandatory for a dense collection.",
            "Use a single roving tab stop with arrow-key navigation rather than one tab stop per item.",
            "Expose true item position and total count even when the collection is virtualized.",
        ]),
        summary_message_id: summary_id(&id),
    }
}

/// Builds the canonical component-manifest package (version 1.0.0).
///
/// Publishes one manifest per [`M5ComponentKind`] for the launch-critical M5 component families.
/// Each manifest declares its anatomy, its mandatory and optional controlled states (drawn from
/// [`CanonicalStateClass`]), its labels, commands, keyboard model, accessibility contract, the
/// foundation token references it renders from, and the extension-author consumption rules.
pub fn seeded_m5_component_manifest_package() -> M5ComponentManifestPackage {
    M5ComponentManifestPackage {
        record_kind: M5_COMPONENT_MANIFEST_PACKAGE_RECORD_KIND.to_owned(),
        schema_version: M5_COMPONENT_MANIFEST_SCHEMA_VERSION,
        package_id: M5_COMPONENT_MANIFEST_PACKAGE_ID.to_owned(),
        package_version: M5_COMPONENT_MANIFEST_PACKAGE_VERSION.to_owned(),
        owner_role: PACKAGE_OWNER_ROLE.to_owned(),
        manifests: vec![
            placeholder_card(),
            state_block(),
            review_sheet(),
            job_row(),
            boundary_bar(),
            form_control(),
            dense_collection(),
        ],
        proof_lane_ref: M5_COMPONENT_MANIFEST_PROOF_REF.to_owned(),
        release_packet_ref: M5_COMPONENT_MANIFEST_RELEASE_PACKET_REF.to_owned(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_COMPONENT_MESSAGE_ID_PREFIX, M5_COMPONENT_MANIFEST_PACKAGE_ID
        ),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}
