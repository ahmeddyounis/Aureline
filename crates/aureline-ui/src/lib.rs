//! Shared UI primitives and contracts.
//!
//! This crate is the cross-surface home for appearance primitives and the
//! semantic token registry used by first-party UI surfaces.
//!
//! [`m5_structured_input_and_staged_review::M5StructuredInputSetPacket`] freezes
//! the canonical structured-input, parameter-provenance, draft-state, and
//! staged-review truth shared by mutation-capable forms, wizards, and review
//! sheets across the provider, admin, request, package, import, settings, and
//! project lanes. Each
//! [`m5_structured_input_and_staged_review::FormSurfaceRecord`] binds field
//! provenance, validation state, draft/applied recovery, submit blockers, and a
//! staged-review (commit) sheet, then re-derives a
//! [`m5_structured_input_and_staged_review::SurfaceClaim`] so a form can never
//! submit from a source-hidden state, hide target scope, omitted defaults,
//! blocked prerequisites, or rollback consequences behind a generic Continue, or
//! discard a recoverable draft. Downstream settings, marketplace, request,
//! support, admin, import, and project surfaces ingest this packet rather than
//! minting per-feature form semantics.
//!
//! [`m5_field_control_rows::M5FieldControlRowSetPacket`] freezes the *per-row*
//! primitive those forms are built from: each
//! [`m5_field_control_rows::FieldControlRow`] carries a permanent label, a
//! required/optional marker, a source-of-value tag, an exact field-anchored
//! validation rule, and a restart/reconnect/trust/policy lifecycle implication
//! surfaced on the control itself, then re-derives a
//! [`m5_field_control_rows::RowClaim`] so a row that hides its label or source,
//! silently overrides a policy lock, defers a blocking validation to a banner, or
//! buries a lifecycle implication floors to an explicit blocked state. The shared
//! primitive is proven first across the provider/account-mapping, source-
//! registration, request-environment, package/install, and migration/import lanes.
//!
//! [`m5_form_validation_and_blocked_submit::M5FormValidationSetPacket`] freezes the
//! layer between them: how a form rolls field validity up into a *form-level
//! validation summary* without replacing the field anchors, how it explains
//! *cross-field dependencies* (provider/account mapping, environment selection,
//! package source/registry auth, import/export mode, derived field constraints),
//! and how it emits *machine-readable blocked-submit reasons* that desktop,
//! CLI/headless, support-export, and docs/help surfaces can all reuse. Each
//! [`m5_form_validation_and_blocked_submit::FormValidationRecord`] re-derives a
//! [`m5_form_validation_and_blocked_submit::FormClaim`] so a form can never submit
//! while a blocked prerequisite or cross-field invalidation is hidden, let its
//! form-level summary contradict or replace the field anchors, or ship a
//! blocked-submit reason that is not machine-readable or reusable.
//!
//! [`m5_draft_state_and_autosave::M5DraftStateSetPacket`] freezes what happens
//! *across an interruption* to those same forms: how edits autosave to a
//! [`m5_draft_state_and_autosave::AutosaveJournal`], how a surface keeps
//! draft-versus-applied state explicit, and how a recover-draft action restores
//! work after a crash, restart, reconnect, or missing-dependency condition. Each
//! [`m5_draft_state_and_autosave::DraftJournalRecord`] re-derives a
//! [`m5_draft_state_and_autosave::DraftClaim`] so an autosave indicator can never
//! claim a draft reached a remote/provider target when only local state was saved,
//! a local draft can never read as applied, an applied state must name its target,
//! and a recover-draft action can never imply a remote write or delete unrelated
//! workspace/profile state. Downstream settings, marketplace, request, support,
//! admin, import, and project surfaces ingest this packet rather than minting
//! per-feature draft/autosave/recovery semantics.
//!
//! [`m5_staged_review_sheets::M5StagedReviewSheetSetPacket`] freezes the *commit
//! sheet itself* as the first-class object every consequential M5 mutation flow
//! stops at before it changes remote/provider/admin/package/request/import state.
//! Each [`m5_staged_review_sheets::ReviewSheetRecord`] declares a target scope (a
//! [`m5_staged_review_sheets::ScopeKind`] from single-object to query-backed or
//! workspace-wide), disclosed omitted defaults, a reconciled
//! [`m5_staged_review_sheets::MemberCounts`] of included/excluded/blocked/hidden
//! objects, a disclosed side-effect summary with a rollback/export path, and a
//! scope-and-effect-specific commit action, then re-derives a
//! [`m5_staged_review_sheets::SheetClaim`] so a sheet that hides its scope, lets its
//! counts disagree, leaves collapsed members uncounted, hides the
//! included/excluded/blocked counts, hides omitted defaults or a side effect, buries
//! a blocked prerequisite or rollback consequence behind a generic Continue, or lets
//! an imported review read as a local apply floors to an explicit blocked state. The
//! one review model is reused across provider publish-later, admin/source-management,
//! request replay/mutation, package install/update/remove, and import/export/publish
//! flows.
//!
//! [`m5_parameter_source_and_precedence::M5ParameterSourceSetPacket`] freezes the
//! *parameter-source inspector* those same forms open to answer **why a current
//! value is present and which source actually wins** before a change is committed.
//! Each [`m5_parameter_source_and_precedence::ParameterFieldRecord`] binds a field's
//! per-layer [`m5_parameter_source_and_precedence::SourceCandidate`] values
//! (default, detected, imported, environment-resolved, user-override,
//! policy-provided), each carrying a personal/local vs workspace/shared vs
//! policy-owned [`m5_parameter_source_and_precedence::ValueScope`], to an
//! [`m5_parameter_source_and_precedence::EffectiveResolution`] that must be the
//! highest-precedence present candidate, a
//! [`m5_parameter_source_and_precedence::PolicyLock`] that pins and forbids a silent
//! override, and a [`m5_parameter_source_and_precedence::FallbackDisclosure`] that
//! explains a fall back to a default. It then re-derives a
//! [`m5_parameter_source_and_precedence::ParameterClaim`] so an inspector that hides
//! its effective source, collapses its distinct layers, mis-orders precedence, hides
//! or fails to enforce a lock, lets an imported review read as a user-set value,
//! hides a fallback reason or scope, or allows a submit from an ambiguous
//! source-hidden state floors to an explicit blocked-submit state — keeping imported
//! values, policy locks, detected values, and user overrides visually distinct
//! across the inspector panel, field popover, diagnostics, support export, CLI
//! inspect, and docs/help surfaces.
//!
//! [`m5_accessibility_and_continuity::M5AccessibilityContinuitySetPacket`] freezes the
//! *accessibility and interruption-safety contract* those same dense surfaces must hold
//! so the shared structured-input model stays fully usable under keyboard-only,
//! assistive-tech, reduced-motion, reconnect, and restart conditions. Each
//! [`m5_accessibility_and_continuity::SurfaceRecord`] binds a surface's keyboard
//! reachability (deterministic focus order, roving focus, batch-action parity, an
//! escapable focus trap), its assistive-tech reachability (permanent screen-reader
//! labels, inline validation links announced, blocked-submit reasons in a live region,
//! the step position announced), its reduced-motion behavior (bound to the shared
//! [`motion::ReducedMotionSubstitutionClass`] so state never depends on animation), and
//! its interruption-safe continuity (a recovery journal that preserves the current step,
//! blocked fields, and draft across reconnect, restart, missing dependency, and crash),
//! then re-derives a [`m5_accessibility_and_continuity::ContinuityClaim`] so a surface
//! that drops a keyboard path, a screen-reader label, an announced validation link or
//! blocked submit, a motion-independent state, a preserved step/blocked-field/draft, a
//! read-only imported review, a recovery path, or a continuity journal floors to an
//! explicit blocked-submit state with a keyboard recovery path. Extension and
//! provider-owned surfaces ingest this packet rather than re-inventing focus order,
//! screen-reader labelling, or recovery semantics, so they cannot quietly regress
//! accessibility or interruption behavior.
//!
//! [`m5_form_family_certification::M5FormFamilyCertificationSetPacket`] is the *promotion
//! model* that certifies the seven preceding component lanes as a whole: it freezes one
//! [`m5_form_family_certification::FamilyRecord`] per claimed M5 mutation-capable form
//! family (provider, admin, request, package, settings, import, and project lanes), binds an
//! [`m5_form_family_certification::EvidenceCell`] per required `(dimension, lane)` proof pair
//! — field/form validation, parameter provenance, draft-versus-applied truth, interruption
//! recovery, and staged-review-before-commit, each pointing at the upstream lane's support
//! export — and re-derives a [`m5_form_family_certification::FamilyDecision`] that floors the
//! family's claimed [`m5_form_family_certification::QualificationTier`] when any of that
//! proof is stale, partial, missing, or failing, or when the certification-freshness window
//! has elapsed or a consumer surface renders a wider tier than the evidence supports. The
//! About, help, service-health, compatibility, release, and support surfaces ingest this
//! packet rather than re-deciding which form families are certified, so a claimed family can
//! never read wider than its current structured-input, provenance, draft-recovery, and
//! staged-review evidence backs.
//!
//! [`m5_annotation_rows::AnnotationRow`] freezes the reusable annotation row for
//! build, test, security, and provider findings. It keeps source provider/scanner
//! provenance, typed file/symbol/manifest anchors, severity, confidence,
//! freshness, stale/superseded/partial handoff state, suppression, remediation,
//! and open-details action as separate export-safe fields so review panes,
//! project-health centers, companion clients, support exports, and release proof
//! render the same anchor truth instead of silently retargeting stale findings.
//!
//! [`m5_dependency_rows::DependencyRow`] freezes the reusable dependency row for
//! package-manager, review, project-health, framework-pack health, companion,
//! support, and release surfaces. It keeps package name and ecosystem, direct
//! versus transitive relation, current-to-target version delta, manifest scope,
//! lockfile impact, advisory counts, and changelog/license actions as separate
//! export-safe fields, and preserves limited, blocked, and policy-constrained
//! update states as visible rows instead of reducing them to disabled buttons.
//!
//! [`m5_pipeline_run_rows::PipelineRunRow`] freezes the reusable pipeline-run
//! row for review, pipeline, project-health, companion, support, and release
//! surfaces. It preserves provider/run identity, trigger, branch/change
//! relation, artifact counts, freshness, provider handoff, and rerun/cancel
//! authority so reduced-capability consumers show limited-action notes instead
//! of hiding controls.
//!
//! [`m5_manifest_diff_cards::ManifestDiffCard`] freezes the reusable manifest
//! diff card for package, review, project-health, companion, support, and
//! release surfaces. It preserves scripts/hooks preview, peer/runtime
//! constraints, checkpoint/rollback state, and apply boundary so package changes
//! cannot flatten into a generic update card.
//!
//! [`m5_security_finding_cards::SecurityFindingCard`] freezes the reusable
//! security-finding card for package, secret, policy, and code-analysis
//! findings. It keeps finding class, affected scope, severity, confidence,
//! freshness, fix availability, controlled suppression labels, remediation
//! path, local validation, docs/help path, and audit actions as separate
//! export-safe fields for review, package, health, companion, support, and
//! release surfaces.
//!
//! [`m5_decision_feedback_component_matrix::M5DecisionFeedbackComponentMatrixPacket`]
//! freezes the reusable badge-chip-pill, popover, dialog-sheet,
//! banner-inline-notice, toast, empty-state, loading-state, and
//! consequence-block decision/feedback primitives into one export-safe matrix.
//! It binds every primitive to one shared state taxonomy (info, success,
//! warning, blocked, pending, degraded, acknowledged, dismissed) and to the
//! family-specific badge, popover, dialog, notice, toast, empty-state,
//! loading, and consequence vocabularies, so badge meaning never depends on
//! color alone, popovers never carry the only critical instruction, high-risk
//! dialogs never use generic Yes/No copy, toasts never become the only durable
//! truth, useful panes are never blanked during loading, and full-screen
//! spinners never replace partial capability across the shell, entry, trust,
//! review, repair, and notification surfaces.
//!
//! [`m5_visual_foundation_matrix::M5VisualFoundationMatrixPacket`] freezes
//! Aureline's concrete visual foundation — color system, semantic theme
//! tokens, syntax / diff / chart token families, typography, and
//! spacing / sizing / radii / elevation geometry with minimum hit-target
//! rules — into one export-safe matrix. It binds every governed family to one
//! shared semantic-role taxonomy (brand, interactive, neutral, status, syntax,
//! diff, chart) and to the family-specific color, theme-token, syntax, diff,
//! chart, typography, geometry, and hit-target vocabularies, and back to the
//! already-landed design-system foundations and publication packets, so status
//! and trust meaning never collapse into color-only cues, syntax and diff
//! palettes never collide with diagnostics, chart meaning never depends on
//! color alone, hit targets never shrink below supported minima, and no
//! surface forks local geometry away from the shared foundation across the
//! shell, editor, review, data, and docs surfaces.
//!
//! [`m5_motion_layer_iconography_matrix::M5MotionLayerIconographyMatrixPacket`]
//! freezes Aureline's motion, overlay, layering, symbol, and illustration
//! grammar — motion-token and reduced-motion families, opacity / scrim classes,
//! z-order layer tiers, portal ownership, iconography, and illustration
//! boundaries — into one export-safe matrix. It binds every governed family to
//! one shared interaction-role taxonomy (motion, overlay, layer, portal, icon,
//! illustration, attention) and to the family-specific motion, reduced-motion,
//! scrim, layer, portal, icon, and illustration vocabularies, and back to the
//! already-landed design-system foundations and publication packets, so motion
//! never delays input on protected paths, scrims never erase workspace
//! orientation or contrast, overlays and portals never bypass the shared z-order
//! model, uncommon or destructive icons never ship unlabeled, and illustrations
//! never impersonate trust, severity, or operational truth across the desktop,
//! dialog, onboarding, notification, and embedded surfaces.
//!
//! [`m5_shell_metric_density_matrix::M5ShellMetricDensityMatrixPacket`]
//! freezes Aureline's concrete shell geometry and density behavior — shell-zone
//! metrics, minimum sizes and hit targets, density modes, responsive window
//! classes, and adaptive-collapse priorities — into one export-safe matrix. It
//! binds every governed family to one shared shell-geometry-role taxonomy
//! (zone, metric, hit_target, density, responsive, collapse,
//! workspace_dominance) and to the family-specific shell-metric, minimum-size,
//! density, responsive, and collapse vocabularies, and back to the
//! already-landed shell-zone and reusable-shell-primitive packets, so the main
//! workspace stays dominant, zones honor declared minimum and recommended sizes,
//! density changes presentation rather than information architecture, responsive
//! collapse preserves task identity and recovery-critical state, hit targets
//! never shrink below supported minimums, and extension or embedded surfaces
//! never invent private widths that fracture the shell across the desktop,
//! editor, review, notebook, and data surfaces.
//!
//! [`m5_install_topology_matrix::M5InstallTopologyMatrixPacket`] freezes
//! Aureline's concrete delivery-topology behavior — per-user managed install,
//! per-machine managed install, side-by-side stable-plus-preview, portable mode,
//! and offline / air-gap bundles — into one export-safe matrix. It binds every
//! governed family to one shared install-topology-role taxonomy (install_mode,
//! updater_owner, binary_root, writable_state_roots, policy_roots,
//! rollback_target, rollout_ring) and to the family-specific per-user,
//! per-machine, side-by-side, portable, and offline / air-gap vocabularies, and
//! back to the already-landed coexistence / fleet-rollout and native-desktop
//! packets, so binary placement and updater ownership stay inspectable, portable
//! mode never spills machine-global durable state, stable and preview channels
//! never corrupt one another, rollback targets the full artifact graph, and
//! rollout rings keep promotion and rollback evidence across the About, update,
//! diagnostics, admin, docs, and support surfaces.
//!
//! [`m5_window_restore_matrix::M5WindowRestoreMatrixPacket`] freezes Aureline's
//! concrete multi-window ownership and restore-orchestration behavior — shared
//! workspace authority backing multiple windows, window-local pane topology,
//! skeleton-first / hydrate-second restore, no-rerun session hydration, and
//! display-topology recovery — into one export-safe matrix. It binds every
//! governed family to one shared window-restore-role taxonomy (workspace_authority,
//! window_topology, pane_role, layout_skeleton, session_hydration, restore_fidelity,
//! display_affinity) and to the family-specific shared-authority, window-local,
//! skeleton-restore, no-rerun, and display-recovery vocabularies, and back to the
//! already-landed multi-window-parity and monitor-geometry-remap packets, so
//! workspace authority and window topology stay separately inspectable, selections
//! and focus stay window-local, restore rebuilds layout skeletons before hydrating
//! heavy dependencies, session-scoped tools never silently rerun or reacquire
//! broader authority, and display-topology changes keep every window and dialog
//! reachable across the shell, recovery, diagnostics, admin, docs, and support
//! surfaces.
//!
//! [`m5_workspace_authority_and_window_topology_registries::M5WorkspaceAuthorityWindowTopologyRegistriesPacket`]
//! is the first implement lane over that frozen matrix: it turns the shared
//! workspace-authority grammar and the window-local-topology grammar into registry
//! resolvers, so every claimed workspace resolves to one stable
//! workspace-authority object — the authority scope, the windows it backs, the
//! stable versioned pane-tree IDs, the shared dirty-buffer / save / checkpoint
//! state, the authoritative workspace state root, and the distinct profile-defaults
//! reference — that the shell, recovery, diagnostics, admin, and support / export
//! surfaces inspect without manual reconstruction, multiple windows share one
//! authority while selection and focus stay window-local without dirty-state drift,
//! window topology never absorbs shared authority into private window state, and a
//! workspace that cannot explain which state is shared and which is window-local
//! degrades honestly.
//!
//! [`m5_skeleton_first_restore_and_session_hydration_registries::M5SkeletonFirstRestoreSessionHydrationRegistriesPacket`]
//! is the skeleton-first / hydrate-second implement lane over that frozen matrix:
//! it turns the skeleton-first-restore grammar and the no-rerun-session-hydration
//! grammar into registry resolvers, so every claimed restore rebuilds one stable
//! restore-skeleton object — the restore-fidelity class, the window shell, the
//! stable versioned pane-tree structure, the preserved pane roles and placeholder
//! set, the layout-skeleton root, and the distinct deferred-hydration plan — before
//! any heavy dependency hydrates, restore is progressively truthful instead of
//! all-or-nothing, a missing dependency produces a pane-role-preserving placeholder
//! instead of a silent layout collapse, session-scoped tools never silently rerun or
//! reacquire broader authority, and support / export can explain which panes came
//! back live, as placeholders, context-only, or evidence-only.
//!
//! [`m5_install_topology_and_state_root_registries::M5InstallTopologyStateRootRegistriesPacket`]
//! is the first implement lane over that frozen matrix: it turns the per-user
//! managed, per-machine managed, and side-by-side install-topology grammar and
//! the portable-mode / offline-air-gap state-root-boundary grammar into registry
//! resolvers, so every claimed delivery profile resolves to one stable
//! install-topology object — install mode, channel, updater owner, binary root,
//! writable state roots, policy roots, and rollback target — that About, update,
//! diagnostics, admin, and support / export surfaces inspect without manual
//! reconstruction, managed-versus-user scopes and side-by-side channels resolve to
//! explicitly isolated state namespaces, and a profile that cannot explain its
//! shared-versus-isolated state degrades honestly.
//!
//! [`m5_portable_mode_state_containment_and_diagnostics::M5PortableModeStateContainmentAndDiagnosticsPacket`]
//! is the portable-mode runtime-enforcement lane over that frozen matrix: it
//! makes *portable* a contract by resolving every claimed portable profile to a
//! colocated or explicitly named sibling-state layout with a complete durable-root
//! inventory of settings, secrets, services, and shell hooks, proving hidden
//! machine-global mutation is absent or explicitly blocked, keeping portable state
//! distinguishable from ordinary installed state, and publishing discoverable
//! portable-mode diagnostics — executable root, state roots, log / crash
//! locations, update posture, and unsupported shell-integration paths — with
//! documented retained-versus-replaced update continuity, so a portable profile
//! that spills durable state into a hidden machine-global path, cannot name its
//! roots, or leaves its diagnostics implicit degrades honestly across the About,
//! update, diagnostics, admin, docs, and support surfaces.
//!
//! [`m5_managed_deployment_operations_and_policy_bootstrap_injection::M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket`]
//! is the managed-deployment execution lane over that frozen matrix: it makes
//! *managed deployment* a contract by resolving every claimed managed profile's
//! silent install, silent uninstall, repair-or-verify, channel-pinning, and
//! update-deferral operation to one inspectable object — the operation, the
//! operation-target / receipt / failure-diagnostics roots, the copyable install-ID
//! / timestamp / failure-summary / repair-verify receipt, and the explicit
//! admin-versus-user ownership — and every policy-bundle / bootstrap injection to
//! one published record — policy-bundle source, bootstrap target, applied
//! settings, admin owner, and deferral window — so a managed installer presented
//! as user-controlled, a failure that strands the user in an ambiguous ownership
//! state, or a bootstrap-policy / channel-pinning / repair-verify semantics drift
//! degrades honestly across the installer, update, diagnostics, admin, docs, and
//! support surfaces.
//!
//! [`m5_channel_isolation_precedence_review_and_rollback_targets::M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket`]
//! is the side-by-side coexistence-execution lane over that frozen matrix: it makes
//! *side-by-side channel isolation* a contract by resolving every claimed
//! side-by-side profile's stable, preview, beta, and LTS channel to one inspectable
//! object — the channel, the channel / state-namespace / secrets-namespace roots,
//! the isolated channel-root / state-namespace / secrets-namespace / services-namespace
//! inventory, and the explicit isolated-versus-governed-handoff containment — and
//! every file-association / protocol-handler / deep-link / default-open precedence
//! rule to one published record — owner channel, precedence rank, conflict
//! resolution, full rollback artifact graph, and inspectable-before-and-after — so a
//! preview or beta channel that reused the stable state namespace without a governed
//! handoff, a handler ownership that became a last-writer-wins accident, or a
//! rollback target narrowed below the full artifact graph degrades honestly across
//! the installer, update, diagnostics, admin, docs, and support surfaces.
//!
//! [`m5_install_topology_shared_consumers_one_registry_across_surfaces::M5InstallTopologySharedConsumersPacket`]
//! is the consumer-adoption capstone over that frozen matrix: it binds each of the
//! five install-topology families to the concrete installer / package-manager,
//! About / shell, update-center / updater, diagnostics, admin, docs / help, CLI /
//! export, support-export, and product / fleet-rollout consumers that render it, and
//! proves — by fixtures, not screenshots — that the same delivery profile presents
//! the same install-topology-role, family, registry-reference, channel,
//! surface-context, and ownership-identity grammar wherever it appears, so a family
//! is adopted by two or more consumers, install mode / channel / updater owner /
//! state roots / rollback target / rollout ring never drift between surfaces, and a
//! surface that reuses a stable state namespace, spills machine-global durable
//! state, hides updater ownership, narrows rollback below the full artifact graph, or
//! outpaces ring evidence degrades honestly rather than silently.
//!
//! [`m5_install_topology_accessibility_parity_and_narrowing_when_install_topology_state_root_repair_verify_or_rollout_evidence_is_stale::InstallTopologyAccessibilityPacket`]
//! is the accessibility-localization-support-export parity and auto-narrowing
//! capstone over that frozen install-topology matrix. It certifies, per
//! delivery-topology family, that install-mode / updater-owner / state-root /
//! repair-verify / rollout-ring / rollback claims stay keyboard-reachable,
//! screen-reader-announced, high-zoom-legible, high-contrast-safe,
//! localization-safe, and CLI/export-safe, and auto-narrows a side-by-side family
//! whose state-boundary proof is only partially disclosed, a portable / offline
//! family whose repair/verify coverage is unconfirmed, or a family whose
//! rollout-ring evidence has aged out or is policy-blocked to a
//! state-boundary-disclosed / repair-verify-unverified / rollout-evidence-unverified
//! projection, so no claimed delivery profile stays green after its B140 evidence
//! ages out or becomes policy-blocked.
//!
//! [`m5_install_topology_surface_certification::InstallTopologyProfileCertificationPacket`]
//! is the closing B140 surface-certification capstone over that frozen
//! install-topology matrix. Keyed on the claimed delivery profile — a live
//! trusted delivery surface, a reviewable delivery structure, a disclosed
//! state-boundary profile, an unverified repair/verify profile, and an unverified
//! rollout-evidence profile — it certifies each profile across nine truth axes
//! (visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, CLI/export, degraded-state, and install-topology-component-truth)
//! and either delivers its claim (green) or auto-narrows to a state-boundary-disclosed
//! / repair-verify-unverified / rollout-evidence-unverified projection (yellow).
//! Only a live first-party profile may certify a trusted delivery surface,
//! CLI/export parity must always certify, every B140 hard invariant must hold, and
//! every row cites the one canonical install-topology proof bundle so release,
//! docs, and support consume a single delivery-topology certification source.
//!
//! [`m5_pipeline_dependency_finding_components::M5PipelineDependencyFindingComponentProof`]
//! validates the first-consumer proof across the five component families and
//! checks that review panes, package centers, project-health centers, companion
//! clients, support export, and release proof preserve controlled labels and
//! narrow action authority explicitly.
//!
//! [`m5_platform_fit_matrix::M5PlatformFitMatrixPacket`] freezes Aureline's
//! desktop platform-fit conventions — platform conventions, shortcut notation,
//! file / path / reveal / save terminology, live theme / contrast / accent /
//! text-scale response, credential-store wording, and IME / dead-key / AltGr /
//! dictation / emoji / layout-switch behavior — into one export-safe matrix. It
//! binds every governed family to one shared platform-fit-role taxonomy
//! (shortcut, window_menu, path_terminology, appearance, credential_wording,
//! input_fidelity, command_stability) and to the family-specific
//! platform-convention, shortcut-notation, file-path-reveal, theme-contrast,
//! credential-wording, and input-method vocabularies, and back to the
//! already-landed native-desktop matrix, so command IDs stay stable while
//! platform labels and shortcut notation adapt, primary actions are never hidden
//! in OS chrome alone, file / path / reveal / save terminology matches the host,
//! theme / contrast changes apply live or explain their fallback, credential-store
//! wording stays truthful and non-leaky, and input methods never corrupt text or
//! trust fidelity across macOS, Windows, and Linux.
//!
//! [`m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale::PlatformFitAccessibilityPacket`]
//! is the accessibility-localization-support-export parity and auto-narrowing
//! capstone over that frozen platform-fit matrix. It certifies, per platform-fit
//! family, that platform-convention / shortcut / path / appearance /
//! credential-wording / input-method claims stay keyboard-reachable,
//! screen-reader-announced, high-zoom-legible, high-contrast-safe,
//! localization-safe, and CLI/export-safe, exports the active platform profile and
//! its shortcut / path / appearance / credential / input state into diagnostics and
//! support bundles without a raw payload, and auto-narrows the affected claim to a
//! path-terminology-disclosed / appearance-response-unverified /
//! credential-wording-unverified / input-fidelity-unverified projection whenever the
//! platform-fit, input-method, or screenshot/help parity evidence is stale, missing,
//! or failing.

#![doc(html_root_url = "https://docs.rs/aureline-ui/0.0.0")]

pub mod components;
pub mod density;
pub mod m5_accessibility_and_continuity;
pub mod m5_annotation_rows;
pub mod m5_badge_chip_pill_and_popover_expansion_and_anchored_focus_return;
pub mod m5_banner_inline_notice_and_empty_state_scoped_cause_and_next_action;
pub mod m5_button_and_icon_button_state_and_command_attribution;
pub mod m5_channel_isolation_precedence_review_and_rollback_targets;
pub mod m5_color_system_and_semantic_theme_token_registries;
pub mod m5_combobox_and_checkbox_radio_switch_value_source_and_toggle_semantics;
pub mod m5_core_action_input_accessibility_parity_and_narrowing_when_control_truth_is_stale;
pub mod m5_core_action_input_component_matrix;
pub mod m5_core_action_input_component_surface_certification;
pub mod m5_core_action_input_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_decision_feedback_accessibility_parity_and_narrowing_when_decision_feedback_truth_is_stale;
pub mod m5_decision_feedback_component_matrix;
pub mod m5_decision_feedback_component_surface_certification;
pub mod m5_decision_feedback_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_density_mode_registries;
pub mod m5_dependency_rows;
pub mod m5_dialog_sheet_and_consequence_block_rationale_scope_and_rollback_continuity;
pub mod m5_draft_state_and_autosave;
pub mod m5_field_control_rows;
pub mod m5_file_path_reveal_and_native_window_menu_registries;
pub mod m5_form_family_certification;
pub mod m5_form_validation_and_blocked_submit;
pub mod m5_iconography_and_illustration_registries;
pub mod m5_input_method_and_credential_store_wording_registries;
pub mod m5_install_topology_accessibility_parity_and_narrowing_when_install_topology_state_root_repair_verify_or_rollout_evidence_is_stale;
pub mod m5_install_topology_and_state_root_registries;
pub mod m5_install_topology_matrix;
pub mod m5_install_topology_shared_consumers_one_registry_across_surfaces;
pub mod m5_install_topology_surface_certification;
pub mod m5_layer_order_and_portal_registries;
pub mod m5_managed_deployment_operations_and_policy_bootstrap_injection;
pub mod m5_manifest_diff_cards;
pub mod m5_monitor_geometry_remap_and_restore_bounds;
pub mod m5_motion_layer_iconography_accessibility_parity_and_narrowing_when_motion_layer_or_icon_truth_is_stale;
pub mod m5_motion_layer_iconography_matrix;
pub mod m5_motion_layer_iconography_shared_consumers_one_grammar_across_surfaces;
pub mod m5_motion_layer_iconography_surface_certification;
pub mod m5_motion_token_and_reduced_motion_registries;
pub mod m5_opacity_scrim_and_overlay_depth_registries;
pub mod m5_parameter_source_and_precedence;
pub mod m5_pipeline_dependency_finding_components;
pub mod m5_pipeline_run_rows;
pub mod m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale;
pub mod m5_platform_fit_matrix;
pub mod m5_platform_fit_shared_consumers_one_convention_across_surfaces;
pub mod m5_platform_fit_surface_certification;
pub mod m5_portable_mode_state_containment_and_diagnostics;
pub mod m5_responsive_geometry_and_collapse_priority_registries;
pub mod m5_security_finding_cards;
pub mod m5_shell_metric_and_minimum_size_registries;
pub mod m5_shell_metric_density_accessibility_parity_and_narrowing_when_shell_metric_density_or_adaptive_geometry_truth_is_stale;
pub mod m5_shell_metric_density_matrix;
pub mod m5_shell_metric_density_shared_consumers_one_geometry_across_surfaces;
pub mod m5_shell_metric_density_surface_certification;
pub mod m5_shortcut_notation_and_command_label_registries;
pub mod m5_skeleton_first_restore_and_session_hydration_registries;
pub mod m5_spacing_sizing_radii_elevation_and_hit_target_registries;
pub mod m5_split_button_and_segmented_control_safe_default_and_selected_mode;
pub mod m5_staged_review_sheets;
pub mod m5_structured_input_and_staged_review;
pub mod m5_syntax_diff_and_chart_token_registries;
pub mod m5_system_appearance_live_apply_and_source_provenance_registries;
pub mod m5_text_field_and_search_field_labels_validation_and_privacy;
pub mod m5_toast_and_loading_state_acknowledgement_and_loading_fidelity;
pub mod m5_typography_scale_font_stack_and_overflow_registries;
pub mod m5_visual_foundation_matrix;
pub mod m5_visual_foundations_accessibility_parity_and_narrowing_when_visual_foundation_truth_is_stale;
pub mod m5_visual_foundations_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_visual_foundations_surface_certification;
pub mod m5_window_restore_matrix;
pub mod m5_workspace_authority_and_window_topology_registries;
pub mod motion;
pub mod themes;
pub mod tokens;
