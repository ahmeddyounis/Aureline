//! Governed runbook object model for claimed incident and operator surfaces.
//!
//! Aureline markets runbooks as *governed executable guidance*, not rich-text
//! suggestions. This crate freezes the canonical object model that backs that
//! claim: every runbook declares where its authority comes from (a
//! [source class](m5_runbook_governance::RunbookSourceClass)), what class of
//! step is being run (a [step class](m5_runbook_governance::RunbookStepClass)),
//! what scope or approval each step requires
//! ([`RunbookApprovalScope`](m5_runbook_governance::RunbookApprovalScope)), what
//! evidence outputs are expected, and how console/browser pivots
//! ([control-plane boundary](m5_runbook_governance::ControlPlaneBoundaryClass))
//! or archived execution history stay attributable. Companions may follow or
//! request within declared scope but cannot mint hidden privileged mutate
//! channels.
//!
//! The [`m5_runbook_governance`] module publishes the contract matrix that names
//! every governed runbook object, its owner, its first consumer, and the proof
//! packet that keeps it current, plus a release gate that blocks Stable
//! promotion when a claimed runbook-backed surface lacks a mapped object or
//! current proof. Incident workspaces, operator dashboards, docs/help,
//! companions, and support bundles consume this one inventory rather than local
//! prose or screenshots.
//!
//! Runbook authority depends first on *where* a runbook came from. The
//! [`m5_runbook_sources`] module publishes the source register: every runbook
//! source declares its provenance class (repo-local, mirrored docs-pack,
//! managed-catalog, or browser-reference), its version, a signer/provenance
//! block, a freshness window, its owning scope, and its export rights, and the
//! register *derives* an effective authority posture — authoritative, mirrored,
//! managed, or reference-only — that every consuming surface renders identically.
//! Browser-only vendor docs stay reference-only unless another governed source
//! promotes them, and a stale source narrows back to reference-only, so a
//! reference cannot masquerade as a first-party executable runbook.
//!
//! Once a runbook's authority is established, every executable step is itself a
//! durable object. The [`m5_runbook_steps`] module publishes the executable step
//! library: each step declares a stable id, a step class, the
//! [target-selector scope](m5_runbook_steps::TargetSelectorScope) it reaches, the
//! approval it requires, whether it stays
//! [view-only, in-product executable, or handoff-only](m5_runbook_steps::StepExecutionMode),
//! the control-plane boundary it sits on, and the evidence it must produce. Each
//! step [binds](m5_runbook_steps::CommandEnvelopeBinding) the shared
//! command/action-envelope and approval systems instead of a runbook-local bypass,
//! so preview, approval, and audit behavior are
//! [derived mechanically](m5_runbook_steps::StepGovernanceProjection) from the
//! object and stay identical across the desktop UI, companion follow views, and
//! support exports. A step that would mint a hidden privileged mutate channel is
//! rejected.
//!
//! Once a runbook's steps are governed, every *execution* is itself a durable,
//! attributable object. The [`m5_runbook_executions`] module publishes the execution
//! history: each execution record carries one
//! [executed-step row](m5_runbook_governance::ExecutedStepResult) per step it ran —
//! the actor accountable for it, the target it acted on, its outcome, the deviation
//! lineage, any console/browser handoff, the evidence outputs, and the **preview-hash
//! and approval reuse** that gated any mutating step. A mutating row reuses the same
//! shared command/action-envelope preview and approval authority any other governed
//! mutation uses, while observe / verify / communicate rows record attributable
//! execution and evidence with no fake mutation semantics. The history is exposed
//! identically on operator history, support exports, and incident packets, so a
//! runbook execution is never a privileged exception path.
//!
//! Lineage outlives the live session. Each row's
//! [deviation note](m5_runbook_governance::DeviationNote) is a durable, inspectable
//! record — its reason class, affected steps, actor, time, and export-safe summary — so a
//! departure never disappears into generic completion copy. After closure, each
//! execution's [archival lineage](m5_runbook_executions::RunbookArchivalLineageProjection)
//! keeps the archived record joinable to the other Aureline evidence families —
//! incidents, rollouts, reviews, and support bundles — through stable ids, and exposes
//! that lineage from metadata alone, never by retaining raw payloads, so support and
//! audit exports can reconstruct a runbook's full lineage without screenshots or tribal
//! memory.
//!
//! Runbooks routinely cross out of Aureline's governed plane into provider consoles
//! and browser surfaces. The [`m5_runbook_handoffs`] module freezes that pivot as a
//! first-class, attributable transition rather than a hidden escape: every
//! [handoff packet](m5_runbook_governance::ControlPlaneHandoffPacket) names its
//! [destination class](m5_runbook_governance::HandoffDestinationClass), the
//! [reason](m5_runbook_governance::HandoffReasonClass) for the pivot, the object
//! identity it crosses to, a [return anchor](m5_runbook_governance::ReturnAnchor)
//! that keeps the initiating target and evidence identity intact, and any narrowed
//! far-side authority. Each destination carries a
//! [reference-plane state](m5_runbook_governance::ReferencePlaneState) so a browser
//! reference doc stays read-only and can never present itself as executable
//! in-product control, while a provider console is named as the true (external)
//! control plane that Aureline explicitly hands off to. The handoff register
//! projects every governed pivot — including the live ones embedded in execution
//! records — and a reference-plane catalog, exposed identically across the incident
//! workspace, operator history, support exports, and docs/help.
//!
//! Authority must hold all the way out to the smallest client. The
//! [`m5_runbook_companion`] module narrows the governed step model to the
//! **companion client scope**: a browser/mobile companion may
//! [follow, acknowledge, and comment](m5_runbook_companion::CompanionActionClass)
//! within a step's declared scope, may
//! [act in scope](m5_runbook_companion::CompanionScopeDisposition::ActInScope) on a
//! step it is permitted to run — reusing the *same* shared approval-authority and
//! action-envelope refs the desktop path uses, so a companion approval creates the
//! same durable audit/approval objects — but may never become a hidden privileged
//! mutate channel. Any step whose approval or mutation the companion may not grant
//! has its privileged mutate channel
//! [explicitly blocked](m5_runbook_companion::CompanionRunbookSurface::privileged_mutate_blocked_on_companion)
//! and [degrades to a clear desktop handoff](m5_runbook_companion::CompanionActionClass::HandoffToDesktop),
//! never a silent failure or a misleading claim of parity. The one companion
//! register projects this narrowing identically across the companion app, the desktop
//! handoff target, and support exports.
//!
//! Finally, every claim that rests on these lanes is itself qualified. The
//! [`m5_runbook_certification`] module is the capstone that binds the six lane proofs
//! — governance, sources, steps, executions, handoffs, and companion — to the product
//! rows that claim runbook-backed behavior. Each
//! [proof lane](m5_runbook_certification::RunbookProofLane) declares the
//! [certification facet](m5_runbook_certification::CertificationFacet) it covers
//! (source truth, step lineage, boundary honesty, or export proof), its schema, its
//! register, its release proof, and a freshness state; each
//! [claimed incident/operator row](m5_runbook_certification::IncidentOperatorRow) binds
//! the lanes it depends on, and the certification *derives* each row's gaps, gate, and
//! effective claim. A stale lane proof deterministically auto-narrows the rows that
//! bind it below Stable, and a missing or unmapped lane proof blocks them — so an aged
//! proof never leaves a claim standing as implied stable behavior. Help/About, the
//! shiproom, support exports, and the incident/operator surfaces all read this one
//! qualification rather than a private spreadsheet.
//!
//! The records this crate produces are inspectable, serde-serializable truth
//! packets that carry no credential bodies or raw provider/console payloads.

#![doc(html_root_url = "https://docs.rs/aureline-runbooks/0.0.0")]

pub mod m5_runbook_certification;
pub mod m5_runbook_companion;
pub mod m5_runbook_executions;
pub mod m5_runbook_governance;
pub mod m5_runbook_handoffs;
pub mod m5_runbook_sources;
pub mod m5_runbook_steps;
